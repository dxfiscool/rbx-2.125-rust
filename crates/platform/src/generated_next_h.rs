//! platform — generated_next_h — 150 stubs EA-sorted asc global gap filler
//! Source: ida/export.json (85545 funcs) global gap filler next 150 after 0x43f73 not yet in crates/platform/src
//! Batch: 150 stubs | range 0x43f74..0x539fc | rbx_core::SharedPtr not boost
//! Filter: iOS|ViewController|RobloxView|Platform 1276 total, 1276/1276 done, 0 remaining — global gap filler

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};
use core::ffi::c_void;
use parking_lot::{Condvar, Mutex, Once};
use std::collections::{BTreeMap, VecDeque};
use std::sync::LazyLock;

// ---- 0x43f74..0x45eb0 shared carriers ----

/// was: `boost::recursive_mutex` — pthread recursive mutex (AGENTS.md section 4:
/// boost::thread/mutex -> std::thread). Owner tracking gives the recursive
/// acquire/release semantics the `unique_lock::lock` path relies on.
// IDA 0x442bc: ctor runs pthread_mutexattr_settype(RECURSIVE) + init (disasm).
pub struct BoostRecursiveMutex {
    state: Mutex<RecursiveMutexState>,
    cvar: Condvar,
}
struct RecursiveMutexState {
    owner: Option<std::thread::ThreadId>,
    depth: u32,
}
impl BoostRecursiveMutex {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(RecursiveMutexState { owner: None, depth: 0 }),
            cvar: Condvar::new(),
        }
    }
    pub fn lock(&self) {
        let current = std::thread::current().id();
        let mut state = self.state.lock();
        while let Some(owner) = state.owner {
            if owner == current {
                break;
            }
            self.cvar.wait(&mut state);
        }
        state.owner = Some(current);
        state.depth += 1;
    }
    pub fn unlock(&self) {
        let mut state = self.state.lock();
        if state.depth > 0 {
            state.depth -= 1;
        }
        if state.depth == 0 {
            state.owner = None;
            self.cvar.notify_one();
        }
    }
}
impl Default for BoostRecursiveMutex {
    fn default() -> Self {
        Self::new()
    }
}

/// was: `std::deque<boost::function<void ()(void)> *>` — the deque stores
/// unowned function pointers, so elements stay opaque `*mut c_void` and only
/// the node map itself is owned (VecDeque drop frees it, never the pointees).
// IDA 0x44564/0x44590/0x44700: node walk + delete, map alloc, per-slot buffers.
#[derive(Clone, Default)]
pub struct FunctionPtrDeque {
    inner: VecDeque<*mut c_void>,
}
impl FunctionPtrDeque {
    pub fn with_nodes(node_count: usize) -> Self {
        Self {
            inner: (0..node_count).map(|_| core::ptr::null_mut()).collect(),
        }
    }
    pub fn create_nodes(&mut self, first: usize, last: usize) {
        let last = last.min(self.inner.len());
        let first = first.min(last);
        for i in first..last {
            if let Some(slot) = self.inner.get_mut(i) {
                *slot = core::ptr::null_mut();
            }
        }
    }
}

/// was: `boost::function<void ()(bool, void *, RBX::UIEvent)>` — Box<dyn Fn> is
/// the boost::function mapping (AGENTS.md section 4). `RBX::UIEvent` has no host
/// definition in this crate, so the view/event words stay opaque pointers.
// IDA 0x45dc8: operator() throws bad_function_call on empty, else dispatches
// through the functor vtable (decompile).
pub type UiEventCallback = Box<dyn Fn(bool, *mut c_void, *const c_void) + Send + Sync + 'static>;

/// was: `rbx::signals::signal<void ()(bool, void *, RBX::UIEvent)>::slot` —
/// intrusive slot node. Offset +0xC holds the signal back-pointer: connected
/// holds exactly while the signal link is set (IDA 0x45d5c: `LDR R0,[R0,#0xC];
/// return R0 != 0`; 0x45c4c clears it before `remove`). `SharedPtr` is
/// `rbx_core::SharedPtr` (`Arc`), never `boost::intrusive_ptr`.
pub struct UiEventSlot {
    callback: Mutex<Option<UiEventCallback>>,
    signal: Mutex<Option<SharedPtr<UiEventSignal>>>,
    next: Mutex<Option<SharedPtr<UiEventSlot>>>,
}

/// was: `rbx::signals::signal<void ()(bool, void *, RBX::UIEvent)>` — owns the
/// intrusive slot-list head; every mutation runs under the class-wide static
/// mutex from `safe_static_do_get_mutex` (IDA 0x45554, decompile).
pub struct UiEventSignal {
    head: Mutex<Option<SharedPtr<UiEventSlot>>>,
}

/// was: `rbx::signals::connection` — handle returned by `connect`; the weak ref
/// the original adds (`intrusive_ptr_add_weak_ref`, IDA 0x4546c) is automatic
/// for `Weak`, so only the strong slot is retained here.
pub struct UiEventConnection {
    slot: SharedPtr<UiEventSlot>,
}

/// Slot-class static mutex (`slot::mutex()` once_init in IDA 0x45c4c; the
/// `safe_static_*` instantiations sort later in this file).
fn uievent_slot_mutex() -> &'static Mutex<()> {
    static VALUE: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
    &VALUE
}

// 0x43f74 — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueERKS5_
// type: int(void)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_insert_unique(std::pair<unsigned int const,RBX::FunctionMarshaller *> const&)")]
pub fn rb_tree_insert_unique_marshaller_43f74(
    map: &mut BTreeMap<u32, usize>,
    key: u32,
    marshaller: usize,
) -> bool {
// IDA 0x43f74: _M_insert_unique walks with less<unsigned> and inserts only when
// the key is absent (decompile). BTreeMap is the std::map narrowing; values are
// FunctionMarshaller* addresses held as usize.
    map.insert(key, marshaller).is_none()
}

// 0x43fdc — __ZN5boost11unique_lockINS_15recursive_mutexEE4lockEv
// type: int __fastcall(_DWORD)
#[doc(alias = "boost::unique_lock<boost::recursive_mutex>::lock(void)")]
pub fn lock_recursive_mutex_43fdc(mutex: &BoostRecursiveMutex) {
// IDA 0x43fdc: unique_lock<recursive_mutex>::lock — pthread-level acquire with
// owner tracking (disasm).
    mutex.lock();
}

// 0x441a8 — __ZN3RBX18FunctionMarshaller27safe_static_init_staticDataEv
// type: _DWORD __fastcall(RBX::FunctionMarshaller *__hidden this)
#[doc(alias = "RBX::FunctionMarshaller::safe_static_init_staticData(void)")]
pub fn init_function_marshaller_static_data_441a8() {
// IDA 0x441a8: safe_static_init_staticData tail-branches (B.W) into
// safe_static_do_get_staticData at 0x441ac (disasm) — init is the getter.
    function_marshaller_static_data_441ac();
}

// 0x441ac — __ZN3RBX18FunctionMarshaller29safe_static_do_get_staticDataEv
// type: void *__fastcall(RBX::FunctionMarshaller *this)
#[doc(alias = "RBX::FunctionMarshaller::safe_static_do_get_staticData(void)")]
pub fn function_marshaller_static_data_441ac() -> &'static Mutex<BTreeMap<u32, usize>> {
// IDA 0x441ac: function-local static behind the __ZGV...value_ptr guard
// (disasm). The guarded registry has the same unsigned->FunctionMarshaller*
// shape written by _M_insert_unique at 0x43f74 [INFERENCE].
    static STATIC_DATA: LazyLock<Mutex<BTreeMap<u32, usize>>> =
        LazyLock::new(|| Mutex::new(BTreeMap::new()));
    &STATIC_DATA
}

// 0x442bc — __ZN5boost15recursive_mutexC2Ev
// type: _DWORD __fastcall(boost::recursive_mutex *__hidden this)
#[doc(alias = "boost::recursive_mutex::recursive_mutex(void)")]
pub fn new_recursive_mutex_442bc() -> BoostRecursiveMutex {
// IDA 0x442bc: recursive_mutex ctor — pthread_mutexattr_settype(RECURSIVE) +
// init (disasm); the Rust narrowing starts unlocked and unowned.
    BoostRecursiveMutex::new()
}

// 0x44564 — __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EED2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::~_Deque_base()")]
pub fn drop_function_ptr_deque_44564(deque: FunctionPtrDeque) {
// IDA 0x44564: _Deque_base dtor walks _M_start.._M_finish releasing node buffers
// via operator delete (disasm); elements are unowned function* and are not freed.
    drop(deque);
}

// 0x44590 — __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE17_M_initialize_mapEm
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::_M_initialize_map(unsigned long)")]
pub fn initialize_function_ptr_deque_map_44590(node_count: usize) -> FunctionPtrDeque {
// IDA 0x44590: _M_initialize_map allocates the node map and creates the node
// buffers (disasm).
    FunctionPtrDeque::with_nodes(node_count)
}

// 0x446e8 — __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE15_M_allocate_mapEm
// type: int(void)
#[doc(alias = "std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::_M_allocate_map(unsigned long)")]
pub fn allocate_function_ptr_deque_map_446e8(node_count: usize) -> usize {
// IDA 0x446e8: _M_allocate_map throws bad_alloc when count >= 0x40000000, else
// returns node_count<<2 bytes from operator new (disasm).
    if node_count >= 0x4000_0000 {
        panic!("std::__throw_bad_alloc");
    }
    node_count << 2
}

// 0x44700 — __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE15_M_create_nodesEPPS4_S8_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::_M_create_nodes(boost::function<void ()(void)> ***,boost::function<void ()(void)> ***)")]
pub fn create_function_ptr_deque_nodes_44700(
    deque: &mut FunctionPtrDeque,
    first: usize,
    last: usize,
) {
// IDA 0x44700: _M_create_nodes news a buffer per map slot in [first, last)
// (disasm); buffers start null, matching with_nodes state.
    deque.create_nodes(first, last);
}

// 0x447f4 — __ZNSt5dequeIPN5boost8functionIFvvEEESaIS4_EEC2ERKS6_
// type: int __fastcall(int)
#[doc(alias = "std::deque<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::deque(std::deque<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>> const&)")]
pub fn copy_function_ptr_deque_447f4(source: &FunctionPtrDeque) -> FunctionPtrDeque {
// IDA 0x447f4: deque copy ctor sizes the new map from the source range and copies
// the (unowned) function* elements (disasm). Clone copies the pointers only.
    source.clone()
}

// 0x44888 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorIPN5boost8functionIFvvEEERKS8_PS9_ES3_IS8_RS8_PS8_EEET0_T_SH_SG_
#[doc(alias = "std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> *&,boost::function<void ()(void)> **> std::__copy<false,std::random_access_iterator_tag>::copy<std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> * const&,boost::function<void ()(void)> * const*>,std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> *&,boost::function<void ()(void)> **>>(std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> * const&,boost::function<void ()(void)> * const*>,std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> * const&,boost::function<void ()(void)> * const*>,std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> *&,boost::function<void ()(void)> **>)")]
pub fn copy_function_ptr_range_44888(source: &[*mut c_void]) -> Vec<*mut c_void> {
// IDA 0x44888: __copy over _Deque_iterator<function*> ranges — elementwise copy
// across node boundaries (disasm); contiguous narrowing below.
    source.to_vec()
}

// 0x44924 — __GLOBAL__I_a_14
#[doc(alias = "global constructor keyed to_a_14")]
pub fn init_global_a14_44924() {
// IDA 0x44924: global ctor keyed to _a_14 — boost::system generic/system category
// slots + std::ios_base::Init + atexit fini registration (disasm). The Rust
// runtime pre-initializes iostream state, so only once-semantics remain.
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {});
}

// 0x4546c — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::connect<boost::function<void ()(bool,void *,RBX::UIEvent)>>(boost::function<void ()(bool,void *,RBX::UIEvent)> const&)")]
pub fn connect_uievent_signal_4546c(
    signal: &SharedPtr<UiEventSignal>,
    callback: UiEventCallback,
) -> UiEventConnection {
// IDA 0x4546c: signal::connect news a 32-byte callable_slot, runs the callable
// ctor (vtable tags + signal link + assign_to_own of the functor), inserts it,
// and weak-refs the returned connection (decompile).
    let slot = new_uievent_callable_459a4(signal, callback);
    insert_uievent_slot_45554(signal, SharedPtr::clone(&slot));
    UiEventConnection { slot }
}

// 0x45554 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE6insertEPNS6_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::insert(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot *)")]
pub fn insert_uievent_slot_45554(signal: &UiEventSignal, slot: SharedPtr<UiEventSlot>) {
// IDA 0x45554: ReleaseAssert(item) per signal.h:290, call_once static-mutex init,
// lock_guard, then head-insert on the intrusive list with the signal.h:310
// next==head linkage check (decompile).
    debug_assert!(SharedPtr::strong_count(&slot) > 0, "item");
    let _guard = uievent_signal_mutex_458ac().lock();
    let mut head = signal.head.lock();
    *slot.next.lock() = head.take();
    // signal.h:310 next==head holds by construction: item->next is the old head.
    *head = Some(slot);
}

// 0x45764 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotEEaSEPS9_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot>::operator=(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot*)")]
pub fn retain_uievent_slot_45764(slot: SharedPtr<UiEventSlot>) -> SharedPtr<UiEventSlot> {
// IDA 0x45764: intrusive_ptr::operator=(slot*) — add_ref(new), swap, release(old).
    // Arc move folds addref+release — return the retained slot.
    slot
}

// 0x45808 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotEEaSERKSA_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot> const&)")]
pub fn clone_uievent_slot_45808(slot: &SharedPtr<UiEventSlot>) -> SharedPtr<UiEventSlot> {
// IDA 0x45808: intrusive_ptr::operator=(const&) — add_ref plus assign.
    SharedPtr::clone(slot)
}

// 0x458ac — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::safe_static_do_get_mutex(void)")]
pub fn uievent_signal_mutex_458ac() -> &'static Mutex<()> {
// IDA 0x458ac: safe_static_do_get_mutex — guard-checked init of the class-wide
// signal mutex value (disasm: __ZGV...value_ptr guard).
    static VALUE: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
    &VALUE
}

// 0x459a4 — __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_EC2IPS7_EERKSB_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>*>(boost::function<void ()(bool,void *,RBX::UIEvent)> const&,rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>*)")]
pub fn new_uievent_callable_459a4(
    signal: &SharedPtr<UiEventSignal>,
    callback: UiEventCallback,
) -> SharedPtr<UiEventSlot> {
// IDA 0x459a4: callable ctor — next=0, signal link=a3, vtable tags, functor-empty
// marker, then function3::assign_to_own copies the functor in (decompile).
    SharedPtr::new(UiEventSlot {
        callback: Mutex::new(Some(callback)),
        signal: Mutex::new(Some(SharedPtr::clone(signal))),
        next: Mutex::new(None),
    })
}

// 0x45aa0 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE13callable_slotIN5boost8functionIS5_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::callable_slot<boost::function<void ()(bool,void *,RBX::UIEvent)>>::~callable_slot()")]
pub fn drop_uievent_callable_slot_45aa0(slot: &SharedPtr<UiEventSlot>) {
// IDA 0x45aa0: callable_slot D1 — vtable reset + function::clear + member release.
    slot.callback.lock().take();
}

// 0x45b74 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE13callable_slotIN5boost8functionIS5_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::callable_slot<boost::function<void ()(bool,void *,RBX::UIEvent)>>::~callable_slot()")]
pub fn delete_uievent_callable_slot_45b74(slot: SharedPtr<UiEventSlot>) {
// IDA 0x45b74: callable_slot D0 — D1 above plus operator delete; the Arc drop
// below is the delete.
    slot.callback.lock().take();
    drop(slot);
}

// 0x45c4c — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::disconnect(void)")]
pub fn disconnect_uievent_slot_45c4c(slot: &SharedPtr<UiEventSlot>) {
// IDA 0x45c4c: if (slot->signal) { call_once slot-mutex init; lock; if still set
// { slot->signal = 0; signal->remove(slot); } unlock; } (decompile).
    if uievent_slot_connected_45d5c(slot) {
        let _guard = uievent_slot_mutex().lock();
        let signal = slot.signal.lock().take();
        if let Some(signal) = signal {
            remove_uievent_slot_45eb0(&signal, slot);
        }
    }
}

// 0x45d5c — __ZNK3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot9connectedEv
// type: bool __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::connected(void)const")]
pub fn uievent_slot_connected_45d5c(slot: &SharedPtr<UiEventSlot>) -> bool {
// IDA 0x45d5c: LDR R0,[R0,#0xC]; return R0 != 0 (disasm) — the +0xC word is the
// signal back-pointer, so connected holds exactly while the signal link is set.
    slot.signal.lock().is_some()
}

// 0x45d68 — __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_E4callEbS3_S5_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::call(bool,void *,RBX::UIEvent)")]
pub fn call_uievent_callable_45d68(
    slot: &SharedPtr<UiEventSlot>,
    confirmed: bool,
    view: *mut c_void,
    event: *const c_void,
) {
// IDA 0x45d68: callable::call forwards (bool,void*,UIEvent) to the embedded
// function3 at this+0x10 (disasm: ADDS R0,#0x10; BLX function3::op).
    let callback = slot.callback.lock();
    invoke_uievent_function_45dc8(callback.as_ref(), confirmed, view, event);
}

// 0x45d98 — __ZThn4_N3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_E4callEbS3_S5_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::call(bool,void *,RBX::UIEvent)")]
pub fn call_uievent_callable_thunk_45d98(
    slot: &SharedPtr<UiEventSlot>,
    confirmed: bool,
    view: *mut c_void,
    event: *const c_void,
) {
// IDA 0x45d98: non-virtual thunk — this+0xC steps past the second vtable where
// 0x45d68 uses +0x10 (disasm), then the identical forward. Same slot carrier.
    call_uievent_callable_45d68(slot, confirmed, view, event);
}

// 0x45dc8 — __ZNK5boost9function3IvbPvN3RBX7UIEventEEclEbS1_S3_
#[doc(alias = "boost::function3<void,bool,void *,RBX::UIEvent>::operator()(bool,void *,RBX::UIEvent)const")]
pub fn invoke_uievent_function_45dc8(
    func: Option<&UiEventCallback>,
    confirmed: bool,
    view: *mut c_void,
    event: *const c_void,
) {
// IDA 0x45dc8: function3::operator() throws bad_function_call on empty, else
// dispatches via the functor vtable (*(vtable & ~1) + 4) (decompile).
    match func {
        Some(callback) => callback(confirmed, view, event),
        None => panic!("boost::bad_function_call"),
    }
}

// 0x45eb0 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE6removeEPNS6_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::remove(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot *)")]
pub fn remove_uievent_slot_45eb0(signal: &UiEventSignal, slot: &SharedPtr<UiEventSlot>) {
// IDA 0x45eb0: ReleaseAssert(!intrusive_ptr_expired(item)) (signal.h:261),
// optional FLog::SignalPrints trace, unlink-by-identity (head or predecessor
// splice via intrusive_ptr assign — item->next is preserved, not cleared),
// then the signal.h:284 expired re-check (decompile).
    debug_assert!(SharedPtr::strong_count(slot) > 0, "!intrusive_ptr_expired(item)");
    let mut head = signal.head.lock();
    let head_is_item = head
        .as_ref()
        .map(|current| SharedPtr::ptr_eq(current, slot))
        .unwrap_or(false);
    if head_is_item {
        let next = head
            .as_ref()
            .and_then(|current| current.next.lock().clone());
        *head = next;
        return;
    }
    let mut predecessor = head.clone();
    while let Some(node) = predecessor {
        let next = node.next.lock().clone();
        match next {
            Some(following) if SharedPtr::ptr_eq(&following, slot) => {
                *node.next.lock() = following.next.lock().clone();
                break;
            }
            other => predecessor = other,
        }
    }
}

/// was: `rbx::signals::signal<void ()(RBX::DataModel *)>::slot` — intrusive
/// slot node. Same layout contract as `UiEventSlot` (+0x8 signal back-pointer
/// released in D1, IDA 0x46240 pattern); the payload is a raw DataModel word.
pub struct DataModelSlot {
    callback: Mutex<Option<DataModelCallback>>,
    signal: Mutex<Option<SharedPtr<DataModelSignal>>>,
    next: Mutex<Option<SharedPtr<DataModelSlot>>>,
}

/// was: `rbx::signals::signal<void ()(RBX::DataModel *)>` — owns the intrusive
/// slot-list head; mutations run under the class-wide static mutex.
pub struct DataModelSignal {
    head: Mutex<Option<SharedPtr<DataModelSlot>>>,
}

/// was: `rbx::signals::connection` for the DataModel signal — the weak ref the
/// original adds is automatic for `Weak`, so only the strong slot is retained.
pub struct DataModelConnection {
    slot: SharedPtr<DataModelSlot>,
}

/// was: `boost::function<void ()(RBX::DataModel *)>` — Box<dyn Fn> is the
/// boost::function mapping (AGENTS.md section 4). RBX::DataModel stays opaque.
// IDA 0x49e7c: connect news a 32-byte callable_slot and inserts it (decompile).
pub type DataModelCallback = Box<dyn Fn(*const c_void) + Send + Sync + 'static>;

/// was: `rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot`
/// — intrusive slot node; +0x8 signal back-pointer, functor at +0x10
/// (IDA 0x4a148: ADDS R0,#0x10 before function1::op; 0x4a640 D1 clears +0x10).
pub struct TextBoxSlot {
    callback: Mutex<Option<TextBoxCallback>>,
    signal: Mutex<Option<SharedPtr<TextBoxSignal>>>,
    next: Mutex<Option<SharedPtr<TextBoxSlot>>>,
}

/// was: `rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>` —
/// owns the intrusive slot-list head under the class-wide static mutex
/// (IDA 0x4a28c: ReleaseAssert(item) then head-insert, decompile).
pub struct TextBoxSignal {
    head: Mutex<Option<SharedPtr<TextBoxSlot>>>,
}

/// was: `rbx::signals::connection` for the TextBox signal.
pub struct TextBoxConnection {
    slot: SharedPtr<TextBoxSlot>,
}
/// was: `RBX::TextBox` — opaque payload; only the shared ownership moves in
/// the `shared_ptr<TextBox>::operator=` instantiations below (IDA 0x4d238
/// pilfers the src pair, 0x4d2dc addrefs it), so the pointee stays empty.
pub struct TextBoxObject {
    _opaque: [u8; 0],
}

/// was: `boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>` — Box<dyn
/// Fn> per AGENTS.md section 4; the shared_ptr<TextBox> arg stays opaque.
pub type TextBoxCallback = Box<dyn Fn(*const c_void) + Send + Sync + 'static>;

/// was: `rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor
/// const *)>::slot` — intrusive slot node; functor at +0x10 (IDA 0x4a148).
pub struct PropDescSlot {
    callback: Mutex<Option<PropDescCallback>>,
    signal: Mutex<Option<SharedPtr<PropDescSignal>>>,
    next: Mutex<Option<SharedPtr<PropDescSlot>>>,
}

/// was: `rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor
/// const *)>` — owns the intrusive slot-list head.
pub struct PropDescSignal {
    head: Mutex<Option<SharedPtr<PropDescSlot>>>,
}

/// was: `boost::function<void ()(RBX::Reflection::PropertyDescriptor const *)>`.
// IDA 0x4a158: operator() throws bad_function_call on empty (decompile).
pub type PropDescCallback = Box<dyn Fn(*const c_void) + Send + Sync + 'static>;

/// Signal-class static mutex for the TextBox signal (`safe_static_*`
/// instantiations sort later in this file; the do_get wraps this).
fn textbox_signal_mutex() -> &'static Mutex<()> {
    static VALUE: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
    &VALUE
}

/// Slot-class static mutex for the TextBox signal slot (`slot::mutex()`
/// once_init in IDA 0x4a7ec; the `safe_static_*` instantiation sorts later).
fn textbox_slot_mutex() -> &'static Mutex<()> {
    static VALUE: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
    &VALUE
}

/// Signal-class static mutex for the DataModel signal.
fn datamodel_signal_mutex() -> &'static Mutex<()> {
    static VALUE: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
    &VALUE
}

/// Callable-slot constructor shared by `connect` (IDA 0x49e7c) and the
/// `callable` ctor instantiation that sorts later (0x4b5b8).
fn new_datamodel_slot(
    signal: &SharedPtr<DataModelSignal>,
    callback: DataModelCallback,
) -> SharedPtr<DataModelSlot> {
    SharedPtr::new(DataModelSlot {
        callback: Mutex::new(Some(callback)),
        signal: Mutex::new(Some(SharedPtr::clone(signal))),
        next: Mutex::new(None),
    })
}

/// Head-insert shared by `connect` (IDA 0x49e7c) and the `insert`
/// instantiation that sorts later (0x4b164, same signal.h:310 pattern as
/// IDA 0x45554).
fn push_datamodel_slot(signal: &DataModelSignal, slot: SharedPtr<DataModelSlot>) {
    debug_assert!(SharedPtr::strong_count(&slot) > 0, "item");
    let _guard = datamodel_signal_mutex().lock();
    let mut head = signal.head.lock();
    *slot.next.lock() = head.take();
    *head = Some(slot);
}

/// Unlink-by-identity shared by `disconnect` (IDA 0x4a7ec) and the `remove`
/// instantiation that sorts later (0x4aaf4, same pattern as IDA 0x45eb0).
fn unlink_textbox_slot(signal: &TextBoxSignal, slot: &SharedPtr<TextBoxSlot>) {
    debug_assert!(SharedPtr::strong_count(slot) > 0, "!intrusive_ptr_expired(item)");
    let mut head = signal.head.lock();
    let head_is_item = head
        .as_ref()
        .map(|current| SharedPtr::ptr_eq(current, slot))
        .unwrap_or(false);
    if head_is_item {
        let next = head
            .as_ref()
            .and_then(|current| current.next.lock().clone());
        *head = next;
        return;
    }
    let mut predecessor = head.clone();
    while let Some(node) = predecessor {
        let next = node.next.lock().clone();
        match next {
            Some(following) if SharedPtr::ptr_eq(&following, slot) => {
                *node.next.lock() = following.next.lock().clone();
                break;
            }
            other => predecessor = other,
        }
    }
}
/// Slot-class static mutex for the DataModel signal slot (`slot::mutex()`
/// once_init in IDA 0x4b860; the `safe_static_*` instantiations sort later at
/// 0x4bb40/0x4bb44 and forward here).
fn datamodel_slot_mutex() -> &'static Mutex<()> {
    static VALUE: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
    &VALUE
}

/// Unlink-by-identity shared by `disconnect` (IDA 0x4b860) and the `remove`
/// instantiation that sorts with 0x4ba50 (same ReleaseAssert + head-or-splice
/// pattern as IDA 0x45eb0/0x4aaf4).
fn unlink_datamodel_slot(signal: &DataModelSignal, slot: &SharedPtr<DataModelSlot>) {
    debug_assert!(SharedPtr::strong_count(slot) > 0, "!intrusive_ptr_expired(item)");
    let mut head = signal.head.lock();
    let head_is_item = head
        .as_ref()
        .map(|current| SharedPtr::ptr_eq(current, slot))
        .unwrap_or(false);
    if head_is_item {
        let next = head
            .as_ref()
            .and_then(|current| current.next.lock().clone());
        *head = next;
        return;
    }
    let mut predecessor = head.clone();
    while let Some(node) = predecessor {
        let next = node.next.lock().clone();
        match next {
            Some(following) if SharedPtr::ptr_eq(&following, slot) => {
                *node.next.lock() = following.next.lock().clone();
                break;
            }
            other => predecessor = other,
        }
    }
}

/// `function1<void,RBX::DataModel *>::operator()` body shared by
/// `callable::call` (IDA 0x4b97c: `ADDS R0,#0x10` then branch into the
/// operator), its thunk (0x4b984: `ADDS R0,#0xC`, same arg passthrough), and
/// the `operator()` instantiation at 0x4b98c — same empty-throws shape as
/// IDA 0x45dc8/0x4a158.
fn invoke_datamodel_function(func: Option<&DataModelCallback>, model: *const c_void) {
    match func {
        Some(callback) => callback(model),
        None => panic!("boost::bad_function_call"),
    }
}

/// `function1<void,shared_ptr<TextBox>>::operator()` body shared by
/// `callable::call` (IDA 0x4a908: shared_count copy of the TextBox arg, then
/// operator(), then release), its thunk (0x4a9dc: `SUBS R0,#4` then branch
/// into call), and the `operator()` instantiation at 0x4a9e4 — same
/// empty-throws shape as IDA 0x45dc8.
fn invoke_textbox_function(func: Option<&TextBoxCallback>, textbox: *const c_void) {
    match func {
        Some(callback) => callback(textbox),
        None => panic!("boost::bad_function_call"),
    }
}

// 0x45fa0 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::safe_static_init_mutex(void)")]
pub fn init_uievent_slot_mutex_45fa0() -> &'static Mutex<()> {
// IDA 0x45fa0: thunk tail-branch (B.W) into safe_static_do_get_mutex (disasm).
    uievent_slot_mutex_45fa4()
}

// 0x45fa4 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::safe_static_do_get_mutex(void)")]
pub fn uievent_slot_mutex_45fa4() -> &'static Mutex<()> {
// IDA 0x45fa4: __cxa_guard_acquire on the function-local value guard, then
// boost::mutex::mutex at the value with __cxa_atexit dtor (decompile).
// LazyLock folds the guard plus the atexit destroy.
    uievent_slot_mutex()
}

// 0x46094 — __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::~callable()")]
pub fn drop_uievent_callable_46094(slot: &SharedPtr<UiEventSlot>) {
// IDA 0x46094: vtable reset, function3::clear on the embedded functor, vtable
// reset to slot base, then intrusive_ptr_release of the +0x8 signal link
// (decompile). Arc take is the release; vtable resets are drop glue.
    clear_uievent_function_46464(slot);
    slot.signal.lock().take();
}

// 0x46168 — __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::~callable()")]
pub fn delete_uievent_callable_46168(slot: SharedPtr<UiEventSlot>) {
// IDA 0x46168: D1 above plus operator delete (decompile); the Arc drop below
// is the delete.
    drop_uievent_callable_46094(&slot);
    drop(slot);
}

// 0x46240 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::~slot()")]
pub fn drop_uievent_slot_46240(slot: &SharedPtr<UiEventSlot>) {
// IDA 0x46240: vtable reset to slot base, then intrusive_ptr_release of the
// +0x8 signal link (decompile). Plain slots never hold a functor, so the
// callback take is a no-op for them and covers the callable path.
    slot.callback.lock().take();
    slot.signal.lock().take();
}

// 0x462ec — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::~slot()")]
pub fn delete_uievent_slot_462ec(slot: SharedPtr<UiEventSlot>) {
// IDA 0x462ec: D1 above plus operator delete (decompile).
    drop_uievent_slot_46240(&slot);
    drop(slot);
}

// 0x4639c — __ZN5boost9function3IvbPvN3RBX7UIEventEE13assign_to_ownERKS4_
// type: int(void)
#[doc(alias = "boost::function3<void,bool,void *,RBX::UIEvent>::assign_to_own(boost::function3<void,bool,void *,RBX::UIEvent> const&)")]
pub fn assign_uievent_function_4639c(slot: &SharedPtr<UiEventSlot>, callback: UiEventCallback) {
// IDA 0x4639c: if the src vtable word is set, store it; small-object flag
// (bit0) copies the inline words, else the manager-clone op runs through
// (vtable & ~1) with (src+4, dst+4, 0) (decompile). Box<dyn Fn> is always the
// indirect path, moved into the empty own storage here at construction
// (IDA 0x459a4).
    *slot.callback.lock() = Some(callback);
}

// 0x46464 — __ZN5boost9function3IvbPvN3RBX7UIEventEE5clearEv
// type: int(void)
#[doc(alias = "boost::function3<void,bool,void *,RBX::UIEvent>::clear(void)")]
pub fn clear_uievent_function_46464(slot: &SharedPtr<UiEventSlot>) {
// IDA 0x46464: if the vtable word is set and heap-managed (bit0 clear), run
// the manager destroy op (op 2) through the vtable, then store 0 (decompile).
// take() drops the functor (destroy) and leaves empty storage.
    slot.callback.lock().take();
}

// 0x46490 — __GLOBAL__I_a_15
#[doc(alias = "global constructor keyed to_a_15")]
pub fn init_global_a15_46490() {
// IDA 0x46490: global ctor keyed to _a_15 — boost::system generic/system
// category slots plus once init (disasm; decompile failed). Same once-only
// shape as 0x44924; the runtime owns iostream/category state.
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {});
}

// 0x46f64 — __GLOBAL__I_a_16
#[doc(alias = "global constructor keyed to_a_16")]
pub fn init_global_a16_46f64() {
// IDA 0x46f64: global ctor keyed to _a_16 — same generic/system category
// slots plus once init as 0x46490 (disasm; decompile failed).
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {});
}

// 0x47424 — __GLOBAL__I_a_17
#[doc(alias = "global constructor keyed to_a_17")]
pub fn init_global_a17_47424() {
// IDA 0x47424: global ctor keyed to _a_17 — same generic/system category
// slots plus once init as 0x46490 (disasm; decompile failed).
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {});
}

// 0x47c04 — ___copy_helper_block__8
#[doc(alias = "___copy_helper_block__8")]
pub unsafe fn copy_block_capture_47c04(dst: *mut c_void, src: *const c_void) {
// IDA 0x47c04: _Block_object_assign(dst+20, src+20, 3) (decompile+disasm).
// Flag 3 is BLOCK_FIELD_IS_OBJECT: the runtime retains the captured object;
// the host owns the retain, so only the pointer word moves here.
    unsafe {
        *(dst as *mut *const c_void).byte_add(20) =
            *(src as *const *const c_void).byte_add(20);
    }
}

// 0x47c10 — ___destroy_helper_block__8
#[doc(alias = "___destroy_helper_block__8")]
pub unsafe fn destroy_block_capture_47c10(block: *mut c_void) {
// IDA 0x47c10: _Block_object_dispose(block+20, 3) (decompile+disasm) — the
// runtime releases the captured object; the word is cleared below.
    unsafe {
        (block as *mut *const c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x49e7c — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::DataModel *)>::connect<boost::function<void ()(RBX::DataModel *)>>(boost::function<void ()(RBX::DataModel *)> const&)")]
pub fn connect_datamodel_signal_49e7c(
    signal: &SharedPtr<DataModelSignal>,
    callback: DataModelCallback,
) -> DataModelConnection {
// IDA 0x49e7c: operator new(32) a callable_slot, run the callable ctor
// (vtable tags + signal link + assign_to_own), insert it, and weak-ref the
// returned connection (decompile) — same shape as IDA 0x4546c.
    let slot = new_datamodel_slot(signal, callback);
    push_datamodel_slot(signal, SharedPtr::clone(&slot));
    DataModelConnection { slot }
}

// 0x49f64 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)> const&)")]
pub fn connect_textbox_signal_49f64(
    signal: &SharedPtr<TextBoxSignal>,
    callback: TextBoxCallback,
) -> TextBoxConnection {
// IDA 0x49f64: operator new(32) a callable_slot, callable ctor, insert, and
// weak-ref the returned connection (decompile) — same shape as IDA 0x4546c.
    let slot = new_textbox_callable_4a544(signal, callback);
    insert_textbox_slot_4a28c(signal, SharedPtr::clone(&slot));
    TextBoxConnection { slot }
}

// 0x4a04c — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost8functionIS8_EELi1ES8_EC2IPS9_EERKSD_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>*>(boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)> const&,rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>*)")]
pub fn new_propdesc_callable_4a04c(
    signal: &SharedPtr<PropDescSignal>,
    callback: PropDescCallback,
) -> SharedPtr<PropDescSlot> {
// IDA 0x4a04c: next=0, signal link=a3, vtable tags, functor-empty marker, then
// function1::assign_to_own copies the functor in (decompile) — same shape as
// IDA 0x459a4; the functor moves in here.
    SharedPtr::new(PropDescSlot {
        callback: Mutex::new(Some(callback)),
        signal: Mutex::new(Some(SharedPtr::clone(signal))),
        next: Mutex::new(None),
    })
}

// 0x4a148 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost8functionIS8_EELi1ES8_E4callES7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn call_propdesc_callable_4a148(slot: &SharedPtr<PropDescSlot>, desc: *const c_void) {
// IDA 0x4a148: ADDS R0,#0x10 then B.W into function1::operator() (disasm) —
// forwards to the embedded functor at this+0x10.
    let callback = slot.callback.lock();
    invoke_propdesc_function_4a158(callback.as_ref(), desc);
}

// 0x4a150 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost8functionIS8_EELi1ES8_E4callES7_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn call_propdesc_callable_thunk_4a150(slot: &SharedPtr<PropDescSlot>, desc: *const c_void) {
// IDA 0x4a150: ADDS R0,#0xC then the same function1::operator() shim (disasm)
// — this+0xC steps past the second vtable where 0x4a148 uses +0x10, then the
// identical forward. Same slot carrier.
    call_propdesc_callable_4a148(slot, desc);
}

// 0x4a158 — __ZNK5boost9function1IvPKN3RBX10Reflection18PropertyDescriptorEEclES5_
// type: int(void)
#[doc(alias = "boost::function1<void,RBX::Reflection::PropertyDescriptor const*>::operator()(RBX::Reflection::PropertyDescriptor const*)const")]
pub fn invoke_propdesc_function_4a158(func: Option<&PropDescCallback>, desc: *const c_void) {
// IDA 0x4a158: empty functor throws (runtime_error for bad_function_call),
// else dispatches via the functor vtable (decompile) — same shape as
// IDA 0x45dc8.
    match func {
        Some(callback) => callback(desc),
        None => panic!("boost::bad_function_call"),
    }
}

// 0x4a28c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE6insertEPNS8_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::insert(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot *)")]
pub fn insert_textbox_slot_4a28c(signal: &TextBoxSignal, slot: SharedPtr<TextBoxSlot>) {
// IDA 0x4a28c: ReleaseAssert(item), call_once static-mutex init, lock_guard,
// then head-insert on the intrusive list (decompile) — same shape as
// IDA 0x45554.
    debug_assert!(SharedPtr::strong_count(&slot) > 0, "item");
    let _guard = textbox_signal_mutex().lock();
    let mut head = signal.head.lock();
    *slot.next.lock() = head.take();
    *head = Some(slot);
}

// 0x4a49c — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX7TextBoxEEEEE4slotEEaSEPSA_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot*)")]
pub fn retain_textbox_slot_4a49c(slot: SharedPtr<TextBoxSlot>) -> SharedPtr<TextBoxSlot> {
// IDA 0x4a49c: add_ref(new), swap, release(old) (decompile) — Arc move folds
// addref+release, so return the retained slot.
    slot
}

// 0x4a540 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::safe_static_init_mutex(void)")]
pub fn init_textbox_signal_mutex_4a540() {
// IDA 0x4a540: thunk tail-branch (B.W) into safe_static_do_get_mutex
// (disasm). The do_get instantiation sorts later; force-init the shared
// class-wide signal mutex here so both spellings alias one static.
    let _ = textbox_signal_mutex();
}

// 0x4a544 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_EC2IPS9_EERKSC_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>*>(boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)> const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>*)")]
pub fn new_textbox_callable_4a544(
    signal: &SharedPtr<TextBoxSignal>,
    callback: TextBoxCallback,
) -> SharedPtr<TextBoxSlot> {
// IDA 0x4a544: next=0, signal link=a3, vtable tags, functor-empty marker, then
// function1::assign_to_own copies the functor in (decompile) — same shape as
// IDA 0x459a4/0x4a04c; the functor moves in here.
    SharedPtr::new(TextBoxSlot {
        callback: Mutex::new(Some(callback)),
        signal: Mutex::new(Some(SharedPtr::clone(signal))),
        next: Mutex::new(None),
    })
}

// 0x4a640 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE13callable_slotINS2_8functionIS7_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>>::~callable_slot()")]
pub fn drop_textbox_callable_slot_4a640(slot: &SharedPtr<TextBoxSlot>) {
// IDA 0x4a640: vtable reset, function1::clear on the embedded functor at
// +0x10, vtable reset to slot base, then intrusive_ptr_release of the +0x8
// signal link (decompile) — same shape as IDA 0x46094.
    slot.callback.lock().take();
    slot.signal.lock().take();
}

// 0x4a714 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE13callable_slotINS2_8functionIS7_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>>::~callable_slot()")]
pub fn delete_textbox_callable_slot_4a714(slot: SharedPtr<TextBoxSlot>) {
// IDA 0x4a714: D1 above plus operator delete (decompile); the Arc drop below
// is the delete.
    drop_textbox_callable_slot_4a640(&slot);
    drop(slot);
}

// 0x4a7ec — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::disconnect(void)")]
pub fn disconnect_textbox_slot_4a7ec(slot: &SharedPtr<TextBoxSlot>) {
// IDA 0x4a7ec: if (slot->signal) { call_once slot-mutex init; lock; if still
// set { slot->signal = 0; signal->remove(slot); } unlock; } (decompile) —
// same shape as IDA 0x45c4c. The `remove` instantiation sorts later (0x4aaf4);
// unlink below is the shared splice it wraps.
    if slot.signal.lock().is_some() {
        let _guard = textbox_slot_mutex().lock();
        let signal = slot.signal.lock().take();
        if let Some(signal) = signal {
            unlink_textbox_slot(&signal, slot);
        }
    }
}

// 0x4a8fc — __ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::connected(void)const")]
pub fn textbox_slot_connected_4a8fc(slot: &SharedPtr<TextBoxSlot>) -> bool {
// IDA 0x4a8fc: LDR R0,[R0,#0xC]; CMP R0,#0; IT NE; MOVNE R0,#1 (disasm) —
// the +0xC word is the signal back-pointer, so connected holds exactly while
// the signal link is set (decompile returns *(_DWORD *)(a1+12) != 0).
    slot.signal.lock().is_some()
}

// 0x4a908 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::call(rbx_core::SharedPtr<RBX::TextBox>)")]
pub fn call_textbox_callable_4a908(slot: &SharedPtr<TextBoxSlot>, textbox: *const c_void) {
// IDA 0x4a908: shared_count copy of the shared_ptr<TextBox> arg (addref),
// then function1::operator() on the embedded functor at this+0x10, then
// release (decompile). The copy keeps the TextBox alive across dispatch;
// with the opaque payload the borrow covers the call below.
    let callback = slot.callback.lock();
    invoke_textbox_function(callback.as_ref(), textbox);
}

// 0x4a9dc — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::call(rbx_core::SharedPtr<RBX::TextBox>)")]
pub fn call_textbox_callable_thunk_4a9dc(slot: &SharedPtr<TextBoxSlot>, textbox: *const c_void) {
// IDA 0x4a9dc: SUBS R0,#4 then B.W into callable::call at 0x4a908 (disasm) —
// this-4 steps past the second vtable where 0x4a908 uses the primary base,
// then the identical forward. Same slot carrier.
    call_textbox_callable_4a908(slot, textbox);
}

// 0x4a9e4 — __ZNK5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEEclES4_
// type: int(void)
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>>::operator()(rbx_core::SharedPtr<RBX::TextBox>)const")]
pub fn invoke_textbox_function_4a9e4(func: Option<&TextBoxCallback>, textbox: *const c_void) {
// IDA 0x4a9e4: empty functor throws bad_function_call, else dispatches via
// the functor vtable (decompile) — same shape as IDA 0x45dc8/0x4a158.
    invoke_textbox_function(func, textbox);
}

// 0x4aaf4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE6removeEPNS8_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::remove(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot *)")]
pub fn remove_textbox_slot_4aaf4(signal: &TextBoxSignal, slot: &SharedPtr<TextBoxSlot>) {
// IDA 0x4aaf4: ReleaseAssert(!intrusive_ptr_expired(item)) (signal.h:261),
// optional SignalPrints trace, unlink-by-identity with item->next preserved,
// then the signal.h:284 expired re-check (decompile) — same shape as
// IDA 0x45eb0. The shared splice below is unlink_textbox_slot.
    unlink_textbox_slot(signal, slot);
}

// 0x4abe4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::safe_static_init_mutex(void)")]
pub fn init_textbox_slot_mutex_4abe4() -> &'static Mutex<()> {
// IDA 0x4abe4: thunk tail-branch (B.W) into safe_static_do_get_mutex at
// 0x4abe8 (disasm).
    textbox_slot_mutex_4abe8()
}

// 0x4abe8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::safe_static_do_get_mutex(void)")]
pub fn textbox_slot_mutex_4abe8() -> &'static Mutex<()> {
// IDA 0x4abe8: __cxa_guard_acquire on the function-local value guard, then
// boost::mutex::mutex at the value with __cxa_atexit dtor (decompile).
// LazyLock folds the guard plus the atexit destroy.
    textbox_slot_mutex()
}

// 0x4acd8 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::~callable()")]
pub fn drop_textbox_callable_4acd8(slot: &SharedPtr<TextBoxSlot>) {
// IDA 0x4acd8: vtable reset, function1::clear on the embedded functor at
// +0x10, vtable reset to slot base, then intrusive_ptr_release of the +0x8
// signal link (decompile) — same shape as IDA 0x46094/0x4a640.
    slot.callback.lock().take();
    slot.signal.lock().take();
}

// 0x4adac — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::~callable()")]
pub fn delete_textbox_callable_4adac(slot: SharedPtr<TextBoxSlot>) {
// IDA 0x4adac: D1 above plus operator delete (decompile); the Arc drop below
// is the delete.
    drop_textbox_callable_4acd8(&slot);
    drop(slot);
}

// 0x4ae84 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotD1Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::~slot()")]
pub fn drop_textbox_slot_4ae84(slot: &SharedPtr<TextBoxSlot>) {
// IDA 0x4ae84: vtable reset to slot base, then intrusive_ptr_release of the
// +0x8 signal link (decompile). Plain slots never hold a functor, so the
// callback take is a no-op for them and covers the callable path.
    slot.callback.lock().take();
    slot.signal.lock().take();
}

// 0x4af30 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::~slot()")]
pub fn delete_textbox_slot_4af30(slot: SharedPtr<TextBoxSlot>) {
// IDA 0x4af30: D1 above plus operator delete (decompile).
    drop_textbox_slot_4ae84(&slot);
    drop(slot);
}

// 0x4afe0 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEE13assign_to_ownERKS5_
// type: int(void)
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>>::assign_to_own(boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>> const&)")]
pub fn assign_textbox_function_4afe0(slot: &SharedPtr<TextBoxSlot>, callback: TextBoxCallback) {
// IDA 0x4afe0: if the src vtable word is set, store it; small-object flag
// (bit0) copies the inline words, else the manager-clone op runs through
// (vtable & ~1) with (src+4, dst+4, 0) (decompile). Box<dyn Fn> is always the
// indirect path, moved into the empty own storage here (same narrowing as
// IDA 0x4639c).
    *slot.callback.lock() = Some(callback);
}

// 0x4b164 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE6insertEPNS6_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::insert(rbx::signals::signal<void ()(RBX::DataModel *)>::slot *)")]
pub fn insert_datamodel_slot_4b164(signal: &DataModelSignal, slot: SharedPtr<DataModelSlot>) {
// IDA 0x4b164: ReleaseAssert(item), call_once static-mutex init, lock_guard,
// then head-insert on the intrusive list (decompile) — same shape as
// IDA 0x45554/0x4a28c. The shared head-insert below is push_datamodel_slot.
    push_datamodel_slot(signal, slot);
}

// 0x4b374 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotEEaSEPS9_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::DataModel *)>::slot*)")]
pub fn retain_datamodel_slot_4b374(slot: SharedPtr<DataModelSlot>) -> SharedPtr<DataModelSlot> {
// IDA 0x4b374: intrusive_ptr::operator=(slot*) — add_ref(new), swap,
// release(old) (decompile). Arc move folds addref+release — return the
// retained slot.
    slot
}

// 0x4b418 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotEEaSERKSA_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot> const&)")]
pub fn clone_datamodel_slot_4b418(slot: &SharedPtr<DataModelSlot>) -> SharedPtr<DataModelSlot> {
// IDA 0x4b418: intrusive_ptr::operator=(const&) — add_ref plus assign
// (decompile).
    SharedPtr::clone(slot)
}

// 0x4b4bc — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::safe_static_init_mutex(void)")]
pub fn init_datamodel_signal_mutex_4b4bc() -> &'static Mutex<()> {
// IDA 0x4b4bc: thunk tail-branch (B.W) into safe_static_do_get_mutex at
// 0x4b4c0 (disasm).
    datamodel_signal_mutex_4b4c0()
}

// 0x4b4c0 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::safe_static_do_get_mutex(void)")]
pub fn datamodel_signal_mutex_4b4c0() -> &'static Mutex<()> {
// IDA 0x4b4c0: __cxa_guard_acquire on the function-local value guard, then
// operator new + boost::mutex::mutex at the value with guard release
// (decompile). LazyLock folds the guard plus lifetime management.
    datamodel_signal_mutex()
}

// 0x4b5b8 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_EC2IPS7_EERKSB_T_
// type: _DWORD *__fastcall(_DWORD *, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::callable<rbx::signals::signal<void ()(RBX::DataModel *)>*>(boost::function<void ()(RBX::DataModel *)> const&,rbx::signals::signal<void ()(RBX::DataModel *)>*)")]
pub fn new_datamodel_callable_4b5b8(
    signal: &SharedPtr<DataModelSignal>,
    callback: DataModelCallback,
) -> SharedPtr<DataModelSlot> {
// IDA 0x4b5b8: next=0, signal link=a3, vtable tags, functor-empty marker, then
// function1::assign_to_own copies the functor in (decompile: a1[2]=0,
// a1[3]=a3, vtables, a1[4]=0, assign_to_own) — same shape as IDA 0x459a4.
// The shared constructor below is new_datamodel_slot.
    new_datamodel_slot(signal, callback)
}

// 0x4b6b4 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE13callable_slotIN5boost8functionIS5_EEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::callable_slot<boost::function<void ()(RBX::DataModel *)>>::~callable_slot()")]
pub fn drop_datamodel_callable_slot_4b6b4(slot: &SharedPtr<DataModelSlot>) {
// IDA 0x4b6b4: vtable reset, function1::clear on the embedded functor at
// +0x10, vtable reset to slot base, then intrusive_ptr_release of the +0x8
// signal link (decompile) — same shape as IDA 0x46094/0x4a640.
    slot.callback.lock().take();
    slot.signal.lock().take();
}

// 0x4b788 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE13callable_slotIN5boost8functionIS5_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::callable_slot<boost::function<void ()(RBX::DataModel *)>>::~callable_slot()")]
pub fn delete_datamodel_callable_slot_4b788(slot: SharedPtr<DataModelSlot>) {
// IDA 0x4b788: D1 above plus operator delete (decompile); the Arc drop below
// is the delete.
    drop_datamodel_callable_slot_4b6b4(&slot);
    drop(slot);
}

// 0x4b860 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot10disconnectEv
// type: void __fastcall(int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::disconnect(void)")]
pub fn disconnect_datamodel_slot_4b860(slot: &SharedPtr<DataModelSlot>) {
// IDA 0x4b860: if (slot->signal) { call_once slot-mutex init; lock; if still
// set { slot->signal = 0; signal->remove(slot); } unlock; } (decompile) —
// same shape as IDA 0x45c4c/0x4a7ec. The `remove` instantiation sorts later
// (0x4ba50); unlink below is the shared splice it wraps.
    if slot.signal.lock().is_some() {
        let _guard = datamodel_slot_mutex().lock();
        let signal = slot.signal.lock().take();
        if let Some(signal) = signal {
            unlink_datamodel_slot(&signal, slot);
        }
    }
}

// 0x4b970 — __ZNK3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot9connectedEv
// type: bool __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::connected(void)const")]
pub fn datamodel_slot_connected_4b970(slot: &SharedPtr<DataModelSlot>) -> bool {
// IDA 0x4b970: LDR R0,[R0,#0xC]; CMP R0,#0; IT NE; MOVNE R0,#1 (disasm) —
// the +0xC word is the signal back-pointer (decompile returns
// *(_DWORD *)(a1+12) != 0), same shape as IDA 0x45d5c/0x4a8fc.
    slot.signal.lock().is_some()
}

// 0x4b97c — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::call(RBX::DataModel *)")]
pub fn call_datamodel_callable_4b97c(slot: &SharedPtr<DataModelSlot>, model: *const c_void) {
// IDA 0x4b97c: ADDS R0,#0x10 then B.W into function1::operator() (disasm) —
// forwards to the embedded functor at this+0x10 (decompile returns
// operator()(a1+16)).
    let callback = slot.callback.lock();
    invoke_datamodel_function(callback.as_ref(), model);
}

// 0x4b984 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::call(RBX::DataModel *)")]
pub fn call_datamodel_callable_thunk_4b984(slot: &SharedPtr<DataModelSlot>, model: *const c_void) {
// IDA 0x4b984: ADDS R0,#0xC then B.W into function1::operator() (disasm;
// decompile returns operator()(a1+12)) — this+0xC steps past the second
// vtable where 0x4b97c uses +0x10, then the identical forward with the same
// DataModel* arg passthrough. Same slot carrier.
    call_datamodel_callable_4b97c(slot, model);
}

// 0x4b98c — __ZNK5boost9function1IvPN3RBX9DataModelEEclES3_
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "boost::function1<void,RBX::DataModel *>::operator()(RBX::DataModel *)const")]
pub fn invoke_datamodel_function_4b98c(func: Option<&DataModelCallback>, model: *const c_void) {
// IDA 0x4b98c: empty functor throws bad_function_call, else dispatches via
// the functor vtable (decompile) — same shape as IDA 0x45dc8.
    invoke_datamodel_function(func, model);
}

// 0x4ba50 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE6removeEPNS6_4slotE
// type: int __fastcall(char **, char *, int, const void *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::remove(rbx::signals::signal<void ()(RBX::DataModel *)>::slot *)")]
pub fn remove_datamodel_slot_4ba50(signal: &DataModelSignal, slot: &SharedPtr<DataModelSlot>) {
// IDA 0x4ba50: ReleaseAssert(!intrusive_ptr_expired(item)) (signal.h:261),
// optional SignalPrints trace, unlink-by-identity with item->next preserved,
// then the signal.h:284 expired re-check (decompile) — same shape as
// IDA 0x45eb0/0x4aaf4. The shared splice below is unlink_datamodel_slot.
    unlink_datamodel_slot(signal, slot);
}

// 0x4bb40 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot22safe_static_init_mutexEv
// type: int()
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::safe_static_init_mutex(void)")]
pub fn init_datamodel_slot_mutex_4bb40() -> &'static Mutex<()> {
// IDA 0x4bb40: thunk tail-branch (B.W) into safe_static_do_get_mutex
// (decompile returns do_get_mutex()).
    datamodel_slot_mutex_4bb44()
}

// 0x4bb44 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot24safe_static_do_get_mutexEv
// type: void *()
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::safe_static_do_get_mutex(void)")]
pub fn datamodel_slot_mutex_4bb44() -> &'static Mutex<()> {
// IDA 0x4bb44: __cxa_guard_acquire on the function-local value guard, then
// boost::mutex::mutex at the value with __cxa_atexit dtor (decompile).
// LazyLock folds the guard plus the atexit destroy — same shape as
// IDA 0x45fa4.
    datamodel_slot_mutex()
}

// 0x4bc34 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::~callable()")]
pub fn drop_datamodel_callable_4bc34(slot: &SharedPtr<DataModelSlot>) {
// IDA 0x4bc34: vtable reset, function1::clear on the embedded functor at
// +0x10 (decompile clears a1+16), vtable reset to slot base, then
// intrusive_ptr_release of the +0x8 signal link (decompile) — same shape as
// IDA 0x46094. Arc take is the release; vtable resets are drop glue.
    slot.callback.lock().take();
    slot.signal.lock().take();
}

// 0x4bd08 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::~callable()")]
pub fn delete_datamodel_callable_4bd08(slot: SharedPtr<DataModelSlot>) {
// IDA 0x4bd08: D1 above (decompile clears the functor word, then releases
// the +0x8 signal link) plus operator delete (decompile); the Arc drop below
// is the delete — same shape as IDA 0x46168.
    drop_datamodel_callable_4bc34(&slot);
    drop(slot);
}

// 0x4bde0 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotD1Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::~slot()")]
pub fn drop_datamodel_slot_4bde0(slot: &SharedPtr<DataModelSlot>) {
// IDA 0x4bde0: vtable reset to slot base, then intrusive_ptr_release of the
// +0x8 signal link when set (decompile) — same shape as IDA 0x46240. Plain
// slots never hold a functor, so the callback take is a no-op for them and
// covers the callable path.
    slot.callback.lock().take();
    slot.signal.lock().take();
}

// 0x4be8c — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotD0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::~slot()")]
pub fn delete_datamodel_slot_4be8c(slot: SharedPtr<DataModelSlot>) {
// IDA 0x4be8c: D1 above plus operator delete (decompile) — same shape as
// IDA 0x462ec.
    drop_datamodel_slot_4bde0(&slot);
    drop(slot);
}

// 0x4bf3c — __ZN5boost9function1IvPN3RBX9DataModelEE13assign_to_ownERKS4_
// type: int __fastcall(int result, int *)
#[doc(alias = "boost::function1<void,RBX::DataModel *>::assign_to_own(boost::function1<void,RBX::DataModel *> const&)")]
pub fn assign_datamodel_function_4bf3c(slot: &SharedPtr<DataModelSlot>, callback: DataModelCallback) {
// IDA 0x4bf3c: if the src vtable word is set, store it; small-object flag
// (bit0) copies the inline words, else the manager-clone op runs through
// (vtable & ~1) with (src+4, dst+4, 0) (decompile). Box<dyn Fn> is always the
// indirect path, moved into the empty own storage — same shape as
// IDA 0x4639c.
    *slot.callback.lock() = Some(callback);
}

// 0x4bfdc — __ZN5boost9function1IvPKN3RBX10Reflection18PropertyDescriptorEE5clearEv
// type: int __fastcall(int *)
#[doc(alias = "boost::function1<void,RBX::Reflection::PropertyDescriptor const*>::clear(void)")]
pub fn clear_propdesc_function_4bfdc(slot: &SharedPtr<PropDescSlot>) {
// IDA 0x4bfdc: if the vtable word is set and heap-managed (bit0 clear), run
// the manager destroy op (op 2) through the vtable, then store 0 (decompile).
// take() drops the functor (destroy) and leaves empty storage — same shape
// as IDA 0x46464.
    slot.callback.lock().take();
}

// 0x4c008 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEE5clearEv
// type: int __fastcall(int *)
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>>::clear(void)")]
pub fn clear_textbox_function_4c008(slot: &SharedPtr<TextBoxSlot>) {
// IDA 0x4c008: same vtable-set/heap-managed destroy-op-2-then-store-0 shape
// as 0x4bfdc (decompile) for the shared_ptr<TextBox> instantiation.
// take() drops the functor (destroy) and leaves empty storage.
    slot.callback.lock().take();
}

// 0x4c034 — __GLOBAL__I_a_18
#[doc(alias = "global constructor keyed to_a_18")]
pub fn init_global_a18_4c034() {
// IDA 0x4c034: global ctor keyed to _a_18 — boost::system generic(2x) +
// system category slots, ios_base::Init + __cxa_atexit, bad_alloc /
// bad_exception static exception objects, singleton_pool storage +
// create_object guards (XmlAttribute, XmlElement, FWInstance,
// OnDemandInstance), then FactoryProduct creators UserInputService and
// Camera (disasm; decompile failed) — same once-only shape as 0x46490; the
// runtime owns iostream/category state.
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {});
}

// 0x4c498 — __GLOBAL__I_a_19
#[doc(alias = "global constructor keyed to_a_19")]
pub fn init_global_a19_4c498() {
// IDA 0x4c498: global ctor keyed to _a_19 — same generic/system category
// slots, ios Init, exception statics, singleton pools, and FactoryProduct
// creators as 0x4c034 but Camera before UserInputService (disasm; decompile
// failed). Same once-only shape as 0x46490.
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {});
}

// 0x4ce30 — ___copy_helper_block__9
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__9")]
pub unsafe fn copy_block_capture_4ce30(dst: *mut c_void, src: *const c_void) {
// IDA 0x4ce30: _Block_object_assign(dst+20, src+20, 3) (decompile). Flag 3
// is BLOCK_FIELD_IS_OBJECT: the runtime retains the captured object; the
// host owns the retain, so only the pointer word moves here — same shape as
// IDA 0x47c04.
    unsafe {
        *(dst as *mut *const c_void).byte_add(20) =
            *(src as *const *const c_void).byte_add(20);
    }
}

// 0x4ce3c — ___destroy_helper_block__9
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__9")]
pub unsafe fn destroy_block_capture_4ce3c(block: *mut c_void) {
// IDA 0x4ce3c: _Block_object_dispose(block+20, 3) (decompile) — the runtime
// releases the captured object; the word is cleared below — same shape as
// IDA 0x47c10.
    unsafe {
        (block as *mut *const c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x4d090 — ___copy_helper_block_82
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_82")]
pub unsafe fn copy_block_capture_4d090(dst: *mut c_void, src: *const c_void) {
// IDA 0x4d090: _Block_object_assign(dst+20, src+20, 3) (decompile) — same
// single-capture shape as IDA 0x47c04/0x4ce30.
    unsafe {
        *(dst as *mut *const c_void).byte_add(20) =
            *(src as *const *const c_void).byte_add(20);
    }
}

// 0x4d09c — ___destroy_helper_block_83
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_83")]
pub unsafe fn destroy_block_capture_4d09c(block: *mut c_void) {
// IDA 0x4d09c: _Block_object_dispose(block+20, 3) (decompile) — same shape
// as IDA 0x47c10/0x4ce3c.
    unsafe {
        (block as *mut *const c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x4d170 — ___copy_helper_block_87
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_87")]
pub unsafe fn copy_block_capture_4d170(dst: *mut c_void, src: *const c_void) {
// IDA 0x4d170: _Block_object_assign(dst+20, src+20, 3) (decompile) — same
// single-capture shape as IDA 0x47c04/0x4ce30.
    unsafe {
        *(dst as *mut *const c_void).byte_add(20) =
            *(src as *const *const c_void).byte_add(20);
    }
}

// 0x4d17c — ___destroy_helper_block_88
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_88")]
pub unsafe fn destroy_block_capture_4d17c(block: *mut c_void) {
// IDA 0x4d17c: _Block_object_dispose(block+20, 3) (decompile) — same shape
// as IDA 0x47c10/0x4ce3c.
    unsafe {
        (block as *mut *const c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x4d238 — __ZN5boost10shared_ptrIN3RBX7TextBoxEEaSEOS3_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(_DWORD *, __int64 *)
#[doc(alias = "rbx_core::SharedPtr<RBX::TextBox>::operator=(rbx_core::SharedPtr<RBX::TextBox>&&)")]
pub fn assign_textbox_shared_move_4d238(
    dst: &mut Option<SharedPtr<TextBoxObject>>,
    src: &mut Option<SharedPtr<TextBoxObject>>,
) {
// IDA 0x4d238: move-assign — pilfers the src pair (stores 0/0 into src),
// stores the pilfered words into dst, then releases the old dst count
// (decompile). take() moves the Arc out (leaving src empty) and drops the
// replaced dst value (release).
    *dst = src.take();
}

// 0x4d2dc — __ZN5boost10shared_ptrIN3RBX7TextBoxEEaSERKS3_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::TextBox>::operator=(rbx_core::SharedPtr<RBX::TextBox> const&)")]
pub fn assign_textbox_shared_copy_4d2dc(
    dst: &mut Option<SharedPtr<TextBoxObject>>,
    src: &Option<SharedPtr<TextBoxObject>>,
) {
// IDA 0x4d2dc: copy-assign — shared_count copy (addref) of the src pair,
// swap into the dst pair, then release the old dst count (decompile). clone
// is the addref; the replaced dst drops (release).
    *dst = src.clone();
}

// 0x4d398 — __GLOBAL__I_a_20
#[doc(alias = "global constructor keyed to_a_20")]
pub fn init_global_a20_4d398() {
// IDA 0x4d398: global ctor keyed to _a_20 — same generic/system category
// slots, ios Init, exception statics, singleton pools, and FactoryProduct
// creators (UserInputService then Camera) as 0x4c034 (disasm; decompile
// failed). Same once-only shape as 0x46490.
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {});
}

// 0x4d6d4 — __GLOBAL__I_a_21
// type: int()
#[doc(alias = "global constructor keyed to_a_21")]
pub fn init_global_a21_4d6d4() {
// IDA 0x4d6d4: global ctor keyed to _a_21 — ios_base::Init on __ioinit plus
// __cxa_atexit of the dtor (disasm; decompile failed). Same once-only shape
// as 0x46490; the runtime owns iostream state.
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {});
}

// 0x4dfd8 — ___copy_helper_block__10
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__10")]
pub unsafe fn copy_block_capture_4dfd8(dst: *mut c_void, src: *const c_void) {
// IDA 0x4dfd8: _Block_object_assign(dst+20, src+20, 3) (decompile) — same
// single-capture shape as IDA 0x47c04/0x4ce30.
    unsafe {
        *(dst as *mut *const c_void).byte_add(20) =
            *(src as *const *const c_void).byte_add(20);
    }
}

// 0x4dfe4 — ___destroy_helper_block__10
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__10")]
pub unsafe fn destroy_block_capture_4dfe4(block: *mut c_void) {
// IDA 0x4dfe4: _Block_object_dispose(block+20, 3) (decompile) — same shape
// as IDA 0x47c10/0x4ce3c.
    unsafe {
        (block as *mut *const c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x4e01c — ___copy_helper_block_94
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_94")]
pub unsafe fn copy_block_capture_4e01c(dst: *mut c_void, src: *const c_void) {
// IDA 0x4e01c: _Block_object_assign(dst+20, src+20, 3) (decompile) — same
// single-capture shape as IDA 0x47c04/0x4ce30.
    unsafe {
        *(dst as *mut *const c_void).byte_add(20) =
            *(src as *const *const c_void).byte_add(20);
    }
}

// 0x4e028 — ___destroy_helper_block_95
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_95")]
pub unsafe fn destroy_block_capture_4e028(block: *mut c_void) {
// IDA 0x4e028: _Block_object_dispose(block+20, 3) (decompile) — same shape
// as IDA 0x47c10/0x4ce3c.
    unsafe {
        (block as *mut *const c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x4e030 — ___copy_helper_block_100
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_100")]
pub unsafe fn copy_block_captures_4e030(dst: *mut c_void, src: *const c_void) {
// IDA 0x4e030: two captures — _Block_object_assign(dst+20, src+20, 3) then
// _Block_object_assign(dst+24, src+24, 3) (decompile). Flag 3 is
// BLOCK_FIELD_IS_OBJECT; the host owns both retains, so only the two
// pointer words move here.
    unsafe {
        *(dst as *mut *const c_void).byte_add(20) =
            *(src as *const *const c_void).byte_add(20);
        *(dst as *mut *const c_void).byte_add(24) =
            *(src as *const *const c_void).byte_add(24);
    }
}

// 0x4e054 — ___destroy_helper_block_101
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_101")]
pub unsafe fn destroy_block_captures_4e054(block: *mut c_void) {
// IDA 0x4e054: two disposes — _Block_object_dispose(block+20, 3) then
// _Block_object_dispose(block+24, 3) (decompile); the runtime releases both
// captured objects and both words are cleared below.
    unsafe {
        (block as *mut *const c_void)
            .byte_add(20)
            .write(core::ptr::null());
        (block as *mut *const c_void)
            .byte_add(24)
            .write(core::ptr::null());
    }
}

// 0x4e4c8 — ___copy_helper_block_133
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_133")]
pub unsafe fn copy_block_capture_4e4c8(dst: *mut c_void, src: *const c_void) {
// IDA 0x4e4c8: _Block_object_assign(dst+20, src+20, 3) (decompile) — same
// single-capture shape as IDA 0x47c04/0x4ce30.
    unsafe {
        *(dst as *mut *const c_void).byte_add(20) =
            *(src as *const *const c_void).byte_add(20);
    }
}

// 0x4e4d4 — ___destroy_helper_block_134
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_134")]
pub unsafe fn destroy_block_capture_4e4d4(block: *mut c_void) {
// IDA 0x4e4d4: _Block_object_dispose(block+20, 3) (decompile) — same shape
// as IDA 0x47c10/0x4ce3c.
    unsafe {
        (block as *mut *const c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x4e6dc — ___copy_helper_block_148
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_148")]
pub unsafe fn copy_block_capture_4e6dc(dst: *mut c_void, src: *const c_void) {
// IDA 0x4e6dc: _Block_object_assign(dst+20, src+20, 3) (decompile) — same
// single-capture shape as IDA 0x47c04/0x4ce30.
    unsafe {
        *(dst as *mut *const c_void).byte_add(20) =
            *(src as *const *const c_void).byte_add(20);
    }
}

// 0x4e6e8 — ___destroy_helper_block_149
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_149")]
pub unsafe fn destroy_block_capture_4e6e8(block: *mut c_void) {
// IDA 0x4e6e8: _Block_object_dispose(block+20, 3) (decompile) — same shape
// as IDA 0x47c10/0x4ce3c.
    unsafe {
        (block as *mut *const c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x4e6f0 — ___copy_helper_block_153
// type: int __fastcall(int, int)
#[doc(alias = "___copy_helper_block_153")]
pub unsafe fn copy_block_capture_string_4e6f0(dst: *mut c_void, src: *const c_void) {
// IDA 0x4e6f0: _Block_object_assign(dst+20, src+20, 3) then
// std::string::string(dst+24, src+24) (decompile): the object word moves
// and the string copies.
    unsafe {
        *(dst as *mut *const c_void).byte_add(20) =
            *(src as *const *const c_void).byte_add(20);
        let src_string = &*((src as *const u8).byte_add(24) as *const String);
        ((dst as *mut u8).byte_add(24) as *mut String).write(src_string.clone());
    }
}

// 0x4e714 — ___destroy_helper_block_154
// type: int __fastcall(int)
#[doc(alias = "___destroy_helper_block_154")]
pub unsafe fn destroy_block_capture_string_4e714(block: *mut c_void) {
// IDA 0x4e714: _Block_object_dispose(block+20, 3) then
// std::string::~string(block+24) (decompile); the runtime releases the
// captured object and the string storage drops here.
    unsafe {
        (block as *mut *const c_void)
            .byte_add(20)
            .write(core::ptr::null());
        core::ptr::drop_in_place((block as *mut String).byte_add(24));
    }
}

// 0x4e854 — ___copy_helper_block_174
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_174")]
pub unsafe fn copy_block_capture_4e854(dst: *mut c_void, src: *const c_void) {
// IDA 0x4e854: _Block_object_assign(dst+20, src+20, 3) (decompile) — same
// single-capture shape as IDA 0x47c04/0x4ce30.
    unsafe {
        *(dst as *mut *const c_void).byte_add(20) =
            *(src as *const *const c_void).byte_add(20);
    }
}

// 0x4e860 — ___destroy_helper_block_175
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_175")]
pub unsafe fn destroy_block_capture_4e860(block: *mut c_void) {
// IDA 0x4e860: _Block_object_dispose(block+20, 3) (decompile) — same shape
// as IDA 0x47c10/0x4ce3c.
    unsafe {
        (block as *mut *const c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x4e98c — ___copy_helper_block_179
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_179")]
pub unsafe fn copy_block_capture_4e98c(dst: *mut c_void, src: *const c_void) {
// IDA 0x4e98c: _Block_object_assign(dst+20, src+20, 3) (decompile) — same
// single-capture shape as IDA 0x47c04/0x4ce30.
    unsafe {
        *(dst as *mut *const c_void).byte_add(20) =
            *(src as *const *const c_void).byte_add(20);
    }
}

// 0x4e998 — ___destroy_helper_block_180
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_180")]
pub unsafe fn destroy_block_capture_4e998(block: *mut c_void) {
// IDA 0x4e998: _Block_object_dispose(block+20, 3) (decompile) — same shape
// as IDA 0x47c10/0x4ce3c.
    unsafe {
        (block as *mut *const c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x4edcc — ___copy_helper_block_203
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_203")]
pub unsafe fn copy_block_captures_4edcc(dst: *mut c_void, src: *const c_void) {
// IDA 0x4edcc: two captures — _Block_object_assign(dst+20, src+20, 3) then
// the +24 shim assign (decompile). Both flags are BLOCK_FIELD_IS_OBJECT;
// the host owns both retains, so only the two pointer words move here.
    unsafe {
        *(dst as *mut *const c_void).byte_add(20) =
            *(src as *const *const c_void).byte_add(20);
        *(dst as *mut *const c_void).byte_add(24) =
            *(src as *const *const c_void).byte_add(24);
    }
}

// 0x4edf0 — ___destroy_helper_block_204
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_204")]
pub unsafe fn destroy_block_captures_4edf0(block: *mut c_void) {
// IDA 0x4edf0: two disposes — _Block_object_dispose(block+20, 3) then
// _Block_object_dispose(block+24, 3) (decompile); the runtime releases both
// captured objects and both words are cleared below.
    unsafe {
        (block as *mut *const c_void)
            .byte_add(20)
            .write(core::ptr::null());
        (block as *mut *const c_void)
            .byte_add(24)
            .write(core::ptr::null());
    }
}

// 0x4ee0c — __ZN3rbx7signals6signalIFvSsEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(std::string)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string)>::slot> &)")]
/// was: `rbx::signals::signal<void ()(std::string)>::slot` — intrusive slot
/// node; same linkage contract as `UiEventSlot` (signal link + next link).
pub struct StringSlot {
    callback: Mutex<Option<StringCallback>>,
    signal: Mutex<Option<SharedPtr<StringSignal>>>,
    next: Mutex<Option<SharedPtr<StringSlot>>>,
}
/// was: `rbx::signals::signal<void ()(std::string)>` — owns the intrusive
/// slot-list head under the class-wide static mutex.
pub struct StringSignal {
    head: Mutex<Option<SharedPtr<StringSignal>>>,
}
/// was: `boost::function<void ()(std::string)>` — Box<dyn Fn> per AGENTS.md
/// section 4; the string arg moves as an owned `String`.
pub type StringCallback = Box<dyn Fn(String) + Send + Sync + 'static>;
/// Signal-class static mutex for the string signal (`safe_static_*`
/// instantiations sort later in this file; the do_get wraps this).
fn string_signal_mutex() -> &'static Mutex<()> {
    static VALUE: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
    &VALUE
}
pub fn next_string_slot_4ee0c(slot: &SharedPtr<StringSlot>) -> Option<SharedPtr<StringSlot>> {
// IDA 0x4ee0c: add_ref the in-param (0x4ee66), call_once static-mutex init
// (0x4ee86), lock (0x4eea0), intrusive_ptr operator= advance (0x4eeb4),
// conditional unlock (0x4eed0..0x4eed8), release the temp (0x4eee6); a
// non-null advance returns 1, null returns 0 (0x4eef0..0x4eefc) — the
// null-ness survives here as the `Option`.
    let _guard = string_signal_mutex().lock();
    slot.next.lock().clone()
}

// 0x4ef74 — __GLOBAL__I_a_22
#[doc(alias = "global constructor keyed to_a_22")]
pub fn init_global_a22_4ef74() {
// IDA 0x4ef74: global ctor keyed to _a_22 — ios_base::Init on __ioinit plus
// __cxa_atexit of the dtor (disasm: InitC1 + atexit). Same once-only shape
// as 0x4d6d4; the runtime owns iostream state.
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {});
}

// 0x4f7bc — __GLOBAL__I_a_23
#[doc(alias = "global constructor keyed to_a_23")]
pub fn stub_4f7bc() -> ! {
    todo!("0x4f7bc global constructor keyed to_a_23")
}

// 0x4fd40 — ___copy_helper_block__11
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__11")]
pub fn stub_4fd40() -> ! {
    todo!("0x4fd40 ___copy_helper_block__11")
}

// 0x4fd4c — ___destroy_helper_block__11
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__11")]
pub fn stub_4fd4c() -> ! {
    todo!("0x4fd4c ___destroy_helper_block__11")
}

// 0x509a8 — ___copy_helper_block_77
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_77")]
pub fn stub_509a8() -> ! {
    todo!("0x509a8 ___copy_helper_block_77")
}

// 0x509b4 — ___destroy_helper_block_78
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_78")]
pub fn stub_509b4() -> ! {
    todo!("0x509b4 ___destroy_helper_block_78")
}

// 0x50c6c — ___copy_helper_block_81
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_81")]
pub fn stub_50c6c() -> ! {
    todo!("0x50c6c ___copy_helper_block_81")
}

// 0x50c78 — ___destroy_helper_block_82
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_82")]
pub fn stub_50c78() -> ! {
    todo!("0x50c78 ___destroy_helper_block_82")
}

// 0x50c84 — ___copy_helper_block_89
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_89")]
pub fn stub_50c84() -> ! {
    todo!("0x50c84 ___copy_helper_block_89")
}

// 0x50c90 — ___destroy_helper_block_90
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_90")]
pub fn stub_50c90() -> ! {
    todo!("0x50c90 ___destroy_helper_block_90")
}

// 0x50c98 — __GLOBAL__I_a_24
#[doc(alias = "global constructor keyed to_a_24")]
pub fn stub_50c98() -> ! {
    todo!("0x50c98 global constructor keyed to_a_24")
}

// 0x515dc — ___copy_helper_block__12
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__12")]
pub fn stub_515dc() -> ! {
    todo!("0x515dc ___copy_helper_block__12")
}

// 0x515e8 — ___destroy_helper_block__12
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__12")]
pub fn stub_515e8() -> ! {
    todo!("0x515e8 ___destroy_helper_block__12")
}

// 0x51794 — ___copy_helper_block_96
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_96")]
pub fn stub_51794() -> ! {
    todo!("0x51794 ___copy_helper_block_96")
}

// 0x517a0 — ___destroy_helper_block_97
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_97")]
pub fn stub_517a0() -> ! {
    todo!("0x517a0 ___destroy_helper_block_97")
}

// 0x517d8 — ___copy_helper_block_102
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_102")]
pub fn stub_517d8() -> ! {
    todo!("0x517d8 ___copy_helper_block_102")
}

// 0x517e4 — ___destroy_helper_block_103
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_103")]
pub fn stub_517e4() -> ! {
    todo!("0x517e4 ___destroy_helper_block_103")
}

// 0x517f0 — __GLOBAL__I_a_25
#[doc(alias = "global constructor keyed to_a_25")]
pub fn stub_517f0() -> ! {
    todo!("0x517f0 global constructor keyed to_a_25")
}

// 0x51bb0 — __GLOBAL__I_a_26
#[doc(alias = "global constructor keyed to_a_26")]
pub fn stub_51bb0() -> ! {
    todo!("0x51bb0 global constructor keyed to_a_26")
}

// 0x51e54 — ___copy_helper_block__13
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__13")]
pub fn stub_51e54() -> ! {
    todo!("0x51e54 ___copy_helper_block__13")
}

// 0x51e60 — ___destroy_helper_block__13
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__13")]
pub fn stub_51e60() -> ! {
    todo!("0x51e60 ___destroy_helper_block__13")
}

// 0x51fe0 — __GLOBAL__I_a_27
#[doc(alias = "global constructor keyed to_a_27")]
pub fn stub_51fe0() -> ! {
    todo!("0x51fe0 global constructor keyed to_a_27")
}

// 0x52ed4 — ___copy_helper_block__14
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__14")]
pub fn stub_52ed4() -> ! {
    todo!("0x52ed4 ___copy_helper_block__14")
}

// 0x52ef8 — ___destroy_helper_block__14
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__14")]
pub fn stub_52ef8() -> ! {
    todo!("0x52ef8 ___destroy_helper_block__14")
}

// 0x52f44 — ___copy_helper_block_76
// type: void __fastcall(int, const void **)
#[doc(alias = "___copy_helper_block_76")]
pub fn stub_52f44() -> ! {
    todo!("0x52f44 ___copy_helper_block_76")
}

// 0x52f74 — ___destroy_helper_block_77
// type: void __fastcall(const void **)
#[doc(alias = "___destroy_helper_block_77")]
pub fn stub_52f74() -> ! {
    todo!("0x52f74 ___destroy_helper_block_77")
}

// 0x535ac — ___copy_helper_block_84
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_84")]
pub fn stub_535ac() -> ! {
    todo!("0x535ac ___copy_helper_block_84")
}

// 0x535d0 — ___destroy_helper_block_85
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_85")]
pub fn stub_535d0() -> ! {
    todo!("0x535d0 ___destroy_helper_block_85")
}

// 0x53634 — ___copy_helper_block_88
// type: void __fastcall(int, const void **)
#[doc(alias = "___copy_helper_block_88")]
pub fn stub_53634() -> ! {
    todo!("0x53634 ___copy_helper_block_88")
}

// 0x53664 — ___destroy_helper_block_89
// type: void __fastcall(const void **)
#[doc(alias = "___destroy_helper_block_89")]
pub fn stub_53664() -> ! {
    todo!("0x53664 ___destroy_helper_block_89")
}

// 0x539f0 — ___copy_helper_block_97
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_97")]
pub fn stub_539f0() -> ! {
    todo!("0x539f0 ___copy_helper_block_97")
}

// 0x539fc — ___destroy_helper_block_98
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_98")]
pub fn stub_539fc() -> ! {
    todo!("0x539fc ___destroy_helper_block_98")
}
