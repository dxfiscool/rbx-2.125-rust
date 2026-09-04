// Auto-generated skeletons for rbx-script — filler EA-sorted after 0x23f50c (next 150) [filler EA-sorted ascending earliest gap]
// Filter: Lua|Script|Yield|CodeGen (4818 filtered, all already stubbed globally) — filler EA-sorted ascending
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +150 stubs | range 0x23f8f0..0x248b90 | existing 7991 -> 8141 total (filler after 0x23f50c, EA-sorted ascending)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// ---- RBX::worker_thread + boost::thread/bind support cluster (IDA 0x23f8f0..0x241df4) ----
// Ground truth per stub: `decompile(ea)` + `disasm(ea)` via IDA MCP.
// Boost mapping (AGENTS.md section 4): boost::shared_ptr -> rbx_core::SharedPtr
// (Arc); boost::mutex -> parking_lot::Mutex; boost::thread/thread_data ->
// std::thread; boost::bind/function/_bi::bind_t/list2 -> boxed closures;
// boost exceptions -> thiserror errors + panic_any (host for __cxa_throw).
// Unmodeled throughout: pthread interrupt/checker state, thread cancellation,
// and C++ RTTI/vtable offices (thunks adjust `this`, which has no host).

/// was: `RBX::worker_thread::work_result` — int status from the work closure.
/// 1 means more work is ready, so the loop skips the condvar wait (IDA 0x240024).
pub type WorkResult = i32;
/// was: `RBX::worker_thread::work_result` == 1 (IDA 0x240024).
pub const WORK_HAS_MORE: WorkResult = 1;
/// was: `boost::function0<RBX::worker_thread::work_result>` -> boxed closure.
pub type WorkFn = Box<dyn Fn() -> WorkResult + Send + Sync>;
/// was: `boost::function0<void>` -> boxed closure.
pub type VoidFn = Box<dyn Fn() + Send + Sync>;
/// was: `void (*)(boost::function0<void> const&,std::string)` entry (IDA 0x2404f4).
pub type ThreadEntryFn = fn(Option<&VoidFn>, &str);

/// was: `boost::bad_function_call` — built and thrown when an empty function
/// object is invoked (IDA 0x23f98a..0x23fa04, 0x240014..0x2400de).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("boost::bad_function_call: empty function object invoked")]
pub struct BadFunctionCall;

/// was: `boost::condition_error` (a std::runtime_error) — thrown when the
/// condvar wait fails (IDA 0x240b3a..0x240b8c).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("boost::condition_error: {0}")]
pub struct ConditionError(pub String);

/// was: `boost::exception_detail::clone_impl<error_info_injector<condition_error>>`
/// — the thrown wrapper around the condition error (IDA 0x240c80..0x241030).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionErrorClone {
    pub error: ConditionError,
}

impl ConditionErrorClone {
    /// was: `clone() const` — the virtual clone the 0x241430 thunk forwards to.
    pub fn clone_box(&self) -> ConditionErrorClone {
        self.clone()
    }
}

/// was: `boost::throw_exception<boost::bad_function_call>` — host panics with
/// the value (panic_any is the host for __cxa_throw unwinding).
pub fn throw_bad_function_call() -> ! {
    std::panic::panic_any(BadFunctionCall)
}

/// Host for the 0x240b8c throw site: builds the wrapper and throws it.
pub fn throw_condition_error(message: impl Into<String>) -> ! {
    stub_0x240c80(ConditionError(message.into()))
}

/// Host model of `RBX::boost_detail::once_init_foo`/`init_foo` plus the
/// `boost::thread_specific_ptr<std::string>` slot written by `thread_function`
/// (IDA 0x23f924..0x23f980): one-shot init flag and the calling thread's name.
static THREAD_INIT_ONCE: std::sync::Once = std::sync::Once::new();
static THREAD_INIT_DONE: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
thread_local! {
    static THREAD_NAME: std::cell::RefCell<Option<String>> =
        std::cell::RefCell::new(None);
}

/// was: `RBX::boost_detail::init_foo` — the call_once target (IDA 0x23f924).
fn boost_detail_init_foo() {
    THREAD_INIT_DONE.store(true, std::sync::atomic::Ordering::Release);
}

/// Reads the calling thread's name slot (for tests).
pub fn thread_name() -> Option<String> {
    THREAD_NAME.with(|slot| slot.borrow().clone())
}

/// was: `boost::thread_specific_ptr<std::string>` — the target keeps one
/// process-wide key; the per-thread value lives in THREAD_NAME above.
#[derive(Debug, Default)]
pub struct ThreadNameSlot;

/// Host stand-in for the out-of-range D2: dropping the key handle releases it.
/// Per-thread values drain at thread exit (unmodeled).
pub fn thread_specific_ptr_string_d2(_slot: ThreadNameSlot) {}

/// was: `RBX::worker_thread::data` — mutex at +0, condvar at +44, stop flag at
/// +116 (IDA 0x23ffe4, 0x24015a..0x24018a). Boost mapping: mutex+cond become
/// the parking_lot pair; the stop byte becomes an AtomicBool.
#[derive(Debug, Default)]
pub struct WorkerThreadData {
    /// Stop flag at +116, set by D2 (IDA 0x240166), polled by threadProc (IDA 0x23ffe4).
    pub stop: std::sync::atomic::AtomicBool,
    /// Mutex at +0 (IDA 0x23fa7e, 0x24015a).
    pub mutex: parking_lot::Mutex<()>,
    /// Condvar at +44 (IDA 0x23fa8a, 0x240174..0x24018a).
    pub wake: parking_lot::Condvar,
}

/// was: `RBX::worker_thread` — SharedPtr<data> plus the boost::thread at +8
/// (IDA 0x23fa1c..0x23fd1c). Boost mapping: the thread is a std::thread.
pub struct WorkerThread {
    pub data: SharedPtr<WorkerThreadData>,
    pub thread: parking_lot::Mutex<Option<std::thread::JoinHandle<()>>>,
}

impl Drop for WorkerThread {
    // Host for D2 (IDA 0x240100): dropping the value runs the same teardown.
    fn drop(&mut self) {
        self.data
            .stop
            .store(true, std::sync::atomic::Ordering::SeqCst);
        {
            let _guard = self.data.mutex.lock();
            self.data.wake.notify_all();
        }
        std::mem::drop(self.thread.lock().take());
    }
}

/// Host model of the `function_buffer` holding this batch's
/// `bind_t<void,void(*)(SharedPtr<data>,const function0<work_result>&),list2<...>>`
/// (IDA 0x241798/0x241bbc): the bound data handle plus the bound work target.
/// Both are Arcs, so manager clone/move/destroy are retain/move/release.
#[derive(Clone, Default)]
pub struct ThreadProcSlot {
    pub data: Option<SharedPtr<WorkerThreadData>>,
    pub work: Option<SharedPtr<WorkFn>>,
}

/// was: `boost::detail::function::functor_manager_operation_type` for this
/// bind_t (IDA 0x241c20 switch): 0 clone, 1 move, 2 destroy, 3 check type,
/// 4 get typeinfo, anything else publishes the typeinfo (IDA 0x241c1a).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum FunctorOp {
    Clone = 0,
    Move = 1,
    Destroy = 2,
    CheckType = 3,
    GetTypeInfo = 4,
}

/// was: the `function_buffer&` out-param of the manager ops — the host returns
/// the published value instead (typeinfo name or match bit).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctorResult {
    Done,
    TypeName(&'static str),
    TypeMatches(bool),
}

/// Type name published by the manager ops (IDA 0x241d32 strcmp literal).
pub const BIND_T_TYPE_NAME: &str = "N5boost3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS4_11work_resultEEEENS0_5list2INS0_5valueIS6_EENSF_IS9_EEEEEE";

/// Shared bind body behind 0x2407fc/0x241444: invoking runs
/// threadProc(data, work) (IDA 0x241b56: list2::operator() calls threadProc).
pub fn bind_thread_proc(data: SharedPtr<WorkerThreadData>, work: WorkFn) -> VoidFn {
    Box::new(move || stub_0x241aac(&data, Some(&work)))
}

// 0x23f8f0 — __ZN3RBXL15thread_functionERKN5boost9function0IvEESs
// type: void __fastcall(int, int *, int, int)
// was: void __fastcall(int, int *, int, int)
#[doc(alias = "RBX::thread_function(boost::function0<void> const&,std::string)")]
// IDA 0x23f8f0: call_once(init_foo) (0x23f924); copy the name into a fresh
// string and reset the thread-specific slot (0x23f946..0x23f980); empty fn ->
// bad_function_call + throw (0x23f98a..0x23fa04), else invoke it (0x23f99a).
pub fn stub_0x23f8f0(func: Option<&VoidFn>, name: &str) {
    THREAD_INIT_ONCE.call_once(boost_detail_init_foo);
    THREAD_NAME.with(|slot| *slot.borrow_mut() = Some(name.to_owned()));
    match func {
        Some(f) => f(),
        None => throw_bad_function_call(),
    }
}

// 0x23fa10 — __ZN3RBX13worker_threadC1ERKN5boost9function0INS0_11work_resultEEEPKc
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
// was: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::worker_thread::worker_thread(boost::function0<RBX::worker_thread::work_result> const&,char const*)")]
// IDA 0x23fa10: tail-calls C2 (disasm 0x23fa10..0x23fa18: frame around BL to C2).
pub fn stub_0x23fa10(work: WorkFn, name: &str) -> WorkerThread {
    stub_0x23fa1c(work, name)
}

// 0x23fa1c — __ZN3RBX13worker_threadC2ERKN5boost9function0INS0_11work_resultEEEPKc
// type: int __fastcall(int, int *, boost::detail::sp_counted_base *, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, boost::detail::sp_counted_base *, char, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, char, int, boost::detail::sp_counted_base *, int, int, int, int, int, pthread_mutex_t *, int, int, int, int, int, int)
// was: int __fastcall(int, int *, boost::detail::sp_counted_base *, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, boost::detail::sp_counted_base *, char, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, char, int, boost::detail::sp_counted_base *, int, int, int, int, int, pthread_mutex_t *, int, int, int, int, int, int)
#[doc(alias = "RBX::worker_thread::worker_thread(boost::function0<RBX::worker_thread::work_result> const&,char const*) [0x23fa1c]")]
// IDA 0x23fa1c: alloc data 0x78 (mutex at +0, condvar at +44, stop = 0 at +116)
// (0x23fa78..0x23fa9c); wrap in SharedPtr (0x23faae); bind threadProc + work
// (0x23fb5c); spawn via thread_wrapper + boost::thread (0x23fc44..0x23fc52);
// release temporaries (0x23fc58..0x23fcfa); return *this (0x23fd1c).
pub fn stub_0x23fa1c(work: WorkFn, name: &str) -> WorkerThread {
    let data: SharedPtr<WorkerThreadData> = SharedPtr::new(WorkerThreadData::default());
    let worker = WorkerThread {
        data: data.clone(),
        thread: parking_lot::Mutex::new(None),
    };
    let bound: VoidFn = bind_thread_proc(worker.data.clone(), work);
    let thread_name = name.to_owned();
    let handle = std::thread::Builder::new()
        .name(thread_name.clone())
        .spawn(move || {
            stub_0x23f8f0(Some(&bound), &thread_name);
        })
        .expect("RBX::worker_thread: thread spawn failed");
    *worker.thread.lock() = Some(handle);
    worker
}

// 0x23ffb0 — __ZN3RBX13worker_thread10threadProcEN5boost10shared_ptrINS0_4dataEEERKNS1_9function0INS0_11work_resultEEE
// type: void __fastcall(boost::mutex **, _DWORD *)
// was: void __fastcall(boost::mutex **, _DWORD *)
#[doc(alias = "RBX::worker_thread::threadProc(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&)")]
// IDA 0x23ffb0: while (!stop) (0x23ffe4): empty fn -> throw bad_function_call
// (0x240014..0x2400de); if work() != 1 (0x240024) lock + condvar wait
// (0x240028..0x240044), then re-poll stop.
pub fn stub_0x23ffb0(data: &SharedPtr<WorkerThreadData>, work: Option<&WorkFn>) {
    loop {
        if data.stop.load(std::sync::atomic::Ordering::Acquire) {
            break;
        }
        let Some(work) = work else {
            throw_bad_function_call()
        };
        if work() != WORK_HAS_MORE {
            let mut guard = data.mutex.lock();
            data.wake.wait(&mut guard);
        }
    }
}

// 0x2400f4 — __ZN3RBX13worker_threadD1Ev
// type: void __fastcall(RBX::worker_thread *__hidden this)
// was: void __fastcall(RBX::worker_thread *__hidden this)
#[doc(alias = "RBX::worker_thread::~worker_thread()")]
// IDA 0x2400f4: tail-calls D2 (disasm 0x2400f4..0x2400fa: BL to D2).
pub fn stub_0x2400f4(worker: WorkerThread) {
    stub_0x240100(worker);
}

// 0x240100 — __ZN3RBX13worker_threadD2Ev
// type: void __fastcall(boost::mutex **this)
// was: void __fastcall(boost::mutex **this)
#[doc(alias = "RBX::worker_thread::~worker_thread() [0x240100]")]
// IDA 0x240100: lock (0x24015a), stop = 1 (0x240166), cond broadcast
// (0x240174..0x24018a), unlock (0x24018a..0x240192), thread detach (0x2401a6),
// release both shared counts (0x2401ac..0x2401c2). Host: Drop runs the same
// teardown (dropping the JoinHandle detaches; Arcs release).
pub fn stub_0x240100(worker: WorkerThread) {
    std::mem::drop(worker);
}

// 0x2402c4 — __ZN3RBX13worker_thread4wakeEv
// type: void __fastcall(boost::mutex **this)
// was: void __fastcall(boost::mutex **this)
#[doc(alias = "RBX::worker_thread::wake(void)")]
// IDA 0x2402c4: lock (0x2402ee), cond broadcast (0x24032c..0x240338), unlock
// (0x240342..0x24034a).
pub fn stub_0x2402c4(worker: &WorkerThread) {
    let _guard = worker.data.mutex.lock();
    worker.data.wake.notify_all();
}

// 0x2403cc — __ZN5boost19thread_specific_ptrISsED1Ev
#[doc(alias = "boost::thread_specific_ptr<std::string>::~thread_specific_ptr()")]
// IDA 0x2403cc: tail-calls D2 (disasm 0x2403cc..0x2403d4: BLX to D2).
pub fn stub_0x2403cc(slot: ThreadNameSlot) {
    thread_specific_ptr_string_d2(slot);
}

// 0x2403d8 — __ZN5boost19thread_specific_ptrISsE5resetEPSs
// type: void __fastcall(int *, const void *)
// was: void __fastcall(int *, const void *)
#[doc(alias = "boost::thread_specific_ptr<std::string>::reset(std::string *)")]
// IDA 0x2403d8: if the slot value != new (0x24042a): retain the new count
// (0x240462..0x24047c), set_tss_data (0x24048c), release the old (0x240492..0x24049a).
// Host: replace the thread-local name only when it differs.
pub fn stub_0x2403d8(_slot: &ThreadNameSlot, name: Option<String>) {
    THREAD_NAME.with(|slot| {
        let differs = *slot.borrow() != name;
        if differs {
            *slot.borrow_mut() = name;
        }
    });
}

// 0x2404f4 — __ZN5boost4bindIvRKNS_9function0IvEESsS2_SsEENS_3_bi6bind_tIT_PFS7_T0_T1_ENS5_9list_av_2IT2_T3_E4typeEEESB_SD_SE_
// type: void __fastcall(double *, int, int *, const std::string *)
// was: void __fastcall(double *, int, int *, const std::string *)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list_av_2<boost::function0<void>,std::string>::type> boost::bind<void,boost::function0<void> const&,std::string,boost::function0<void>,std::string>(void (*)(boost::function0<void> const&,std::string),boost::function0<void>,std::string)")]
// IDA 0x2404f4: copy the function object (0x24052c..0x240574), copy the string
// (0x24057e), build list2 (0x24058c), pack the bind_t target + captures
// (0x240594..0x2405dc), release temporaries (0x2405ee..0x24068c).
// Host: invoking the result calls `entry` with the stored captures.
pub fn stub_0x2404f4(entry: ThreadEntryFn, func: VoidFn, arg: String) -> VoidFn {
    Box::new(move || entry(Some(&func), &arg))
}

// 0x2407fc — __ZN5boost4bindIvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS3_11work_resultEEES5_S8_EENS_3_bi6bind_tIT_PFSD_T0_T1_ENSB_9list_av_2IT2_T3_E4typeEEESH_SJ_SK_
// type: void __fastcall(boost::detail::sp_counted_base *, int, int *, int, int, int, int, boost::detail::sp_counted_base *, char, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int)
// was: void __fastcall(boost::detail::sp_counted_base *, int, int, int, int, int, int, boost::detail::sp_counted_base *, char, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>>::type> boost::bind<void,rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&,rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>>(void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>)")]
// IDA 0x2407fc: retain the data count (0x240880..0x24089a), copy the work fn
// (0x2408a2..0x2408d4), pack list2 + bind_t (0x2408dc..0x2409xx); invoking the
// result runs threadProc(data, work) (IDA 0x241aac).
pub fn stub_0x2407fc(data: SharedPtr<WorkerThreadData>, work: WorkFn) -> VoidFn {
    bind_thread_proc(data, work)
}

// 0x240a54 — __ZN5boost22condition_variable_any4waitINS_11unique_lockINS_5mutexEEEEEvRT_
// type: void __fastcall(int, int)
// was: void __fastcall(int, int)
#[doc(alias = "void boost::condition_variable_any::wait<boost::unique_lock<boost::mutex>>(boost::unique_lock<boost::mutex> &)")]
// IDA 0x240a54: build the interruption checker (0x240ab2); unlock (0x240ac0);
// pthread_cond_wait (0x240ad6); clear the interrupt state (0x240af2..0x240b1c);
// relock + interruption_point (0x240b20..0x240b34); on failure build a
// system_error and throw condition_error (0x240b3a..0x240b8c).
// BUG(host): pthread interruption/checker state is not modeled and the
// parking_lot wait has no error path, so the throw is unreachable here.
pub fn stub_0x240a54(data: &SharedPtr<WorkerThreadData>) {
    let mut guard = data.mutex.lock();
    data.wake.wait(&mut guard);
}

// 0x240c80 — __ZN5boost15throw_exceptionINS_15condition_errorEEEvRKT_
// type: void __fastcall __noreturn(_QWORD *)
// was: void __fastcall __noreturn(_QWORD *)
#[doc(alias = "void boost::throw_exception<boost::condition_error>(boost::condition_error const&)")]
// IDA 0x240c80: allocate the exception (0x240cba), copy the message strings
// (0x240cf2..0x240f7c), wire the vtables (0x240f40..0x241002), copy the boost
// exception data (0x24100e), __cxa_throw the clone_impl wrapper (0x241030).
// Boost mapping: __cxa_throw becomes panic_any with the wrapper value.
pub fn stub_0x240c80(err: ConditionError) -> ! {
    std::panic::panic_any(ConditionErrorClone { error: err })
}

// 0x241040 — __ZN5boost15condition_errorD1Ev
// type: void __fastcall(std::runtime_error *this)
// was: void __fastcall(std::runtime_error *this)
#[doc(alias = "boost::condition_error::~condition_error()")]
// IDA 0x241040: reset the vtable to runtime_error (0x24105e), release the
// message rep unless shared-static (0x241062..0x24109a), run ~runtime_error
// (0x241070). Host: dropping the value frees the message the same way.
pub fn stub_0x241040(err: ConditionError) {
    std::mem::drop(err);
}

// 0x2410a0 — __ZN5boost15condition_errorD0Ev
// type: void __fastcall(std::runtime_error *this)
// was: void __fastcall(std::runtime_error *this)
#[doc(alias = "boost::condition_error::~condition_error() [0x2410a0]")]
// IDA 0x2410a0: D1 body (0x2410be..0x2410d0) then operator delete (0x2410d6).
// Host: dropping the Box frees the value and the allocation.
pub fn stub_0x2410a0(err: Box<ConditionError>) {
    stub_0x241040(*err);
}

// 0x241108 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_15condition_errorEEEED1Ev
// type: std::runtime_error *__fastcall(std::runtime_error *)
// was: std::runtime_error *__fastcall(std::runtime_error *)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>>::~clone_impl()")]
// IDA 0x241108: release the error_info chain (0x241144..0x241178), reset to
// runtime_error (0x241194), release the message rep (0x241198..0x2411ee), run
// the base dtor (0x2411a8). Host: dropping the wrapper frees both.
pub fn stub_0x241108(wrap: ConditionErrorClone) {
    std::mem::drop(wrap);
}

// 0x241214 — __ZThn20_N5boost16exception_detail19error_info_injectorINS_15condition_errorEED1Ev
// type: void __fastcall(_DWORD *)
// was: void __fastcall(_DWORD *)
#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::condition_error>::~error_info_injector()")]
// IDA 0x241214: this -= 20 (0x241274) then the injector D1 (0x241274..0x2412bc).
// Host: single inheritance, no adjustment; forwards to the D1 body.
pub fn stub_0x241214(wrap: ConditionErrorClone) {
    stub_0x241108(wrap);
}

// 0x241324 — __ZThn20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_15condition_errorEEEED1Ev
// type: void __fastcall(_DWORD *)
// was: void __fastcall(_DWORD *)
#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>>::~clone_impl()")]
// IDA 0x241324: this -= 20 (0x241382) then the clone_impl D1 (0x241382..0x2413c8).
// Host: single inheritance, no adjustment; forwards to the D1 body.
pub fn stub_0x241324(wrap: ConditionErrorClone) {
    stub_0x241108(wrap);
}

// 0x241430 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_15condition_errorEEEE5cloneEv
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>>::clone(void)const")]
// IDA 0x241430: load vtable, adjust this by [vtable,#-12] (0x241432..0x24143a),
// tail-call clone() (0x24143c). Host: no vtable offices; forwards to clone_box.
// (Decompilation fails for this thunk; disasm only.)
pub fn stub_0x241430(wrap: &ConditionErrorClone) -> ConditionErrorClone {
    wrap.clone_box()
}

// 0x241444 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS0_INS7_11work_resultEEEENS3_5list2INS3_5valueIS9_EENSH_ISB_EEEEEEEEvT_
// type: void __fastcall(int, int, int, int, char, int, boost::detail::sp_counted_base *, int, int, int, int, char, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int)
// was: void __fastcall(int, int, int, int, char, int, boost::detail::sp_counted_base *, int, int, int, int, char, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>)")]
// IDA 0x241444: copy the bind_t captures (retain the shared data, clone the
// work fn) then basic_vtable::assign_to installs the stored vtable. Host:
// returns the installed binding as the function value.
pub fn stub_0x241444(data: SharedPtr<WorkerThreadData>, work: WorkFn) -> VoidFn {
    bind_thread_proc(data, work)
}

// 0x241798 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS7_11work_resultEEEENS3_5list2INS3_5valueIS9_EENSI_ISC_EEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// IDA 0x241798: op == 4 publishes the bind_t typeinfo inline (0x24179c..0x2417b2);
// every other op delegates to manager<mpl::bool_<false>> (0x2417b4).
// (Decompilation fails for this dispatch stub; disasm only.)
pub fn stub_0x241798(
    src: &mut ThreadProcSlot,
    dst: &mut ThreadProcSlot,
    op: i32,
) -> FunctorResult {
    if op == FunctorOp::GetTypeInfo as i32 {
        FunctorResult::TypeName(BIND_T_TYPE_NAME)
    } else {
        stub_0x241bbc(src, dst, op)
    }
}

// 0x2417bc — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS7_11work_resultEEEENS3_5list2INS3_5valueIS9_EENSI_ISC_EEEEEEvE6invokeERNS1_15function_bufferE
// type: int __fastcall(_DWORD *)
// was: int __fastcall(_DWORD *)
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// IDA 0x2417bc: list2::operator()<threadProc,list0>(buf[0] + 4, buf[0], tmp)
// (0x2417ce). Host: run threadProc with the slot's captures; missing captures
// throw bad_function_call instead of faulting (see 0x23ffb0).
pub fn stub_0x2417bc(slot: &ThreadProcSlot) {
    match (slot.data.clone(), slot.work.clone()) {
        (Some(data), Some(work)) => {
            let work_fn: &WorkFn = &work;
            stub_0x241aac(&data, Some(work_fn));
        }
        _ => throw_bad_function_call(),
    }
}

// 0x2417d0 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS9_11work_resultEEEENS5_5list2INS5_5valueISB_EENSK_ISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, double *, _DWORD *, int, boost::detail::sp_counted_base *, int, int, int, int, void *, int, int, int, int)
// was: int __fastcall(int, double *, _DWORD *, int, boost::detail::sp_counted_base *, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// IDA 0x2417d0: copy the data handle (retain, 0x241826..0x24186e), copy the
// work fn (0x241878..0x2418a6), heap-alloc the 28-byte bind copy
// (0x2418ba..0x241978), install it into the buffer (0x24194a/0x241958),
// release the temp (0x2419a6..0x2419ae), return 1 (0x2419ce).
// Host: move the captures into the slot; the small buffer always fits.
pub fn stub_0x2417d0(
    dst: &mut ThreadProcSlot,
    data: SharedPtr<WorkerThreadData>,
    work: WorkFn,
) -> bool {
    *dst = ThreadProcSlot {
        data: Some(data),
        work: Some(SharedPtr::new(work)),
    };
    true
}

// 0x241aac — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX13worker_thread4dataEEEEENS2_INS_9function0INS5_11work_resultEEEEEEclIPFvS7_RKSB_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, void (__fastcall **)(int *, int))
// was: void __fastcall(int *, void (__fastcall **)(int *, int))
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::operator()<void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list0>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&) &,boost::_bi::list0 &,int)")]
// IDA 0x241aac: retain the data (0x241b00..0x241b48), call threadProc with the
// stored captures (0x241b56), release (0x241b5a..0x241b62). Host: the Arc
// clone scopes the retain across the call.
pub fn stub_0x241aac(data: &SharedPtr<WorkerThreadData>, work: Option<&WorkFn>) {
    let retained: SharedPtr<WorkerThreadData> = data.clone();
    stub_0x23ffb0(&retained, work);
}

// 0x241bbc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS7_11work_resultEEEENS3_5list2INS3_5valueIS9_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
// was: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// IDA 0x241bbc: clone (retain + copy, 0x241c30..0x241c62), move (0x241cc8..0x241ccc),
// destroy (0x241cd2..0x241d14), check type via strcmp (0x241d32..0x241d3c),
// default publishes the bind_t typeinfo (0x241c1a..0x241c1e). Host: Arc
// clone/move/take/drop are the retain/move/release; the slot only ever holds
// this bind_t, so the check always matches.
pub fn stub_0x241bbc(
    src: &mut ThreadProcSlot,
    dst: &mut ThreadProcSlot,
    op: i32,
) -> FunctorResult {
    if op == FunctorOp::Clone as i32 {
        *dst = src.clone();
        FunctorResult::Done
    } else if op == FunctorOp::Move as i32 {
        *dst = std::mem::take(src);
        FunctorResult::Done
    } else if op == FunctorOp::Destroy as i32 {
        *dst = ThreadProcSlot::default();
        FunctorResult::Done
    } else if op == FunctorOp::CheckType as i32 {
        FunctorResult::TypeMatches(true)
    } else {
        FunctorResult::TypeName(BIND_T_TYPE_NAME)
    }
}

// 0x241df4 — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX13worker_thread4dataEEEEENS2_INS_9function0INS5_11work_resultEEEEEEC2ES8_SC_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::list2(boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>)")]
// IDA 0x241df4: copy the shared data (retain, 0x241e48..0x241e90), copy the
// work fn (0x241e98..0x241eca), storage2 init (0x241ed6), release the temps
// (0x241edc..0x241f08). Host: assemble the slot; Arcs own the retains.
pub fn stub_0x241df4(data: SharedPtr<WorkerThreadData>, work: WorkFn) -> ThreadProcSlot {
    ThreadProcSlot {
        data: Some(data),
        work: Some(SharedPtr::new(work)),
    }
}

// 0x241f98 — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX13worker_thread4dataEEEEENS2_INS_9function0INS5_11work_resultEEEEEEC2ES8_SC_
// type: int __fastcall(int, int *, int *, int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: int __fastcall(int, int *, int *, int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>)")]
pub fn stub_0x241f98() -> ! {
    todo!("0x241f98 __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX13worker_thread4dataEEEEENS2_INS_9function0INS5_11work_resultEEEEEEC2ES8_SC_")
}

// 0x242144 — __ZN5boost3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS4_11work_resultEEEENS0_5list2INS0_5valueIS6_EENSF_IS9_EEEEEC2ESD_RKSI_
// type: int __fastcall(int, int, int)
// was: int __fastcall(int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>::bind_t(void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>> const&)")]
pub fn stub_0x242144() -> ! {
    todo!("0x242144 __ZN5boost3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS4_11work_resultEEEENS0_5list2INS0_5valueIS6_EENSF_IS9_EEEEEC2ESD_RKSI_")
}

// 0x242284 — __ZN5boost6detail20sp_pointer_constructIN3RBX13worker_thread4dataES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, boost::detail::sp_counted_base **, int, void *, int)
// was: void __fastcall(int, int, boost::detail::sp_counted_base **, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::worker_thread::data,RBX::worker_thread::data>(rbx_core::SharedPtr<RBX::worker_thread::data> *,RBX::worker_thread::data *,boost::detail::shared_count &)")]
pub fn stub_0x242284() -> ! {
    todo!("0x242284 __ZN5boost6detail20sp_pointer_constructIN3RBX13worker_thread4dataES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

// 0x2423c8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEED1Ev
// type: void()
// was: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::~sp_counted_impl_p()")]
pub fn stub_0x2423c8() -> ! {
    todo!("0x2423c8 __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEED1Ev")
}

// 0x2423cc — __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEED0Ev
// type: void __fastcall(void *)
// was: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::~sp_counted_impl_p() [0x2423cc]")]
pub fn stub_0x2423cc() -> ! {
    todo!("0x2423cc __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEED0Ev")
}

// 0x2423d8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEE7disposeEv
// type: void __fastcall(int)
// was: void __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::dispose(void)")]
pub fn stub_0x2423d8() -> ! {
    todo!("0x2423d8 __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEE7disposeEv")
}

// 0x2424bc — __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEE11get_deleterERKSt9type_info
// type: int()
// was: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::get_deleter(std::type_info const&)")]
pub fn stub_0x2424bc() -> ! {
    todo!("0x2424bc __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEE11get_deleterERKSt9type_info")
}

// 0x2424c0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEE19get_untyped_deleterEv
// type: int()
// was: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::get_untyped_deleter(void)")]
pub fn stub_0x2424c0() -> ! {
    todo!("0x2424c0 __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEE19get_untyped_deleterEv")
}

// 0x2424c4 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvRKS1_SsENS3_5list2INS3_5valueIS1_EENSA_ISsEEEEEEEEvT_
// type: void __fastcall(_DWORD *, double *)
// was: void __fastcall(_DWORD *, double *)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>)")]
pub fn stub_0x2424c4() -> ! {
    todo!("0x2424c4 __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvRKS1_SsENS3_5list2INS3_5valueIS1_EENSA_ISsEEEEEEEEvT_")
}

// 0x242818 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKNS_9function0IvEESsENS3_5list2INS3_5valueIS6_EENSC_ISsEEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
// was: _UNKNOWN **__fastcall(int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x242818() -> ! {
    todo!("0x242818 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKNS_9function0IvEESsENS3_5list2INS3_5valueIS6_EENSC_ISsEEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE")
}

// 0x24283c — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvRKNS_9function0IvEESsENS3_5list2INS3_5valueIS6_EENSC_ISsEEEEEEvE6invokeERNS1_15function_bufferE
// type: void __fastcall(void (__fastcall ***)(_DWORD, int *))
// was: void __fastcall(void (__fastcall ***)(_DWORD, int *))
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_0x24283c() -> ! {
    todo!("0x24283c __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvRKNS_9function0IvEESsENS3_5list2INS3_5valueIS6_EENSC_ISsEEEEEEvE6invokeERNS1_15function_bufferE")
}

// 0x242958 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKNS_9function0IvEESsENS5_5list2INS5_5valueIS8_EENSE_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, double *, void **)
// was: int __fastcall(int, double *, void **)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x242958() -> ! {
    todo!("0x242958 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKNS_9function0IvEESsENS5_5list2INS5_5valueIS8_EENSE_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

// 0x242be8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKNS_9function0IvEESsENS3_5list2INS3_5valueIS6_EENSC_ISsEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, int, int)
// was: void __fastcall(int *, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x242be8() -> ! {
    todo!("0x242be8 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKNS_9function0IvEESsENS3_5list2INS3_5valueIS6_EENSC_ISsEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

// 0x242e08 — __ZN5boost3_bi5list2INS0_5valueINS_9function0IvEEEENS2_ISsEEEC2ES5_S6_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *, const std::string *)
// was: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *, const std::string *)
#[doc(alias = "boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>::list2(boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>)")]
pub fn stub_0x242e08() -> ! {
    todo!("0x242e08 __ZN5boost3_bi5list2INS0_5valueINS_9function0IvEEEENS2_ISsEEEC2ES5_S6_")
}

// 0x242fc0 — __ZN5boost3_bi8storage2INS0_5valueINS_9function0IvEEEENS2_ISsEEEC2ES5_S6_
// type: _DWORD *__fastcall(_DWORD *, int *, const std::string *)
// was: _DWORD *__fastcall(_DWORD *, int *, const std::string *)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>)")]
pub fn stub_0x242fc0() -> ! {
    todo!("0x242fc0 __ZN5boost3_bi8storage2INS0_5valueINS_9function0IvEEEENS2_ISsEEEC2ES5_S6_")
}

// 0x24316c — __ZN5boost19thread_specific_ptrISsED2Ev
// type: boost::_anonymous_namespace_ *__fastcall(boost::_anonymous_namespace_ *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: boost::_anonymous_namespace_ *__fastcall(boost::_anonymous_namespace_ *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::thread_specific_ptr<std::string>::~thread_specific_ptr() [0x24316c]")]
pub fn stub_0x24316c() -> ! {
    todo!("0x24316c __ZN5boost19thread_specific_ptrISsED2Ev")
}

// 0x243260 — __ZN5boost19thread_specific_ptrISsE11delete_dataD1Ev
// type: void()
// was: void()
#[doc(alias = "boost::thread_specific_ptr<std::string>::delete_data::~delete_data()")]
pub fn stub_0x243260() -> ! {
    todo!("0x243260 __ZN5boost19thread_specific_ptrISsE11delete_dataD1Ev")
}

// 0x243264 — __ZN5boost19thread_specific_ptrISsE11delete_dataD0Ev
// type: void __fastcall(void *)
// was: void __fastcall(void *)
#[doc(alias = "boost::thread_specific_ptr<std::string>::delete_data::~delete_data() [0x243264]")]
pub fn stub_0x243264() -> ! {
    todo!("0x243264 __ZN5boost19thread_specific_ptrISsE11delete_dataD0Ev")
}

// 0x243270 — __ZN5boost19thread_specific_ptrISsE11delete_dataclEPv
// type: void __fastcall(int, int *)
// was: void __fastcall(int, int *)
#[doc(alias = "boost::thread_specific_ptr<std::string>::delete_data::operator()(void *)")]
pub fn stub_0x243270() -> ! {
    todo!("0x243270 __ZN5boost19thread_specific_ptrISsE11delete_dataclEPv")
}

// 0x2432c4 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEED1Ev
// type: void()
// was: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::~sp_counted_impl_pd()")]
pub fn stub_0x2432c4() -> ! {
    todo!("0x2432c4 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEED1Ev")
}

// 0x2432c8 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEED0Ev
// type: void __fastcall(void *)
// was: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::~sp_counted_impl_pd() [0x2432c8]")]
pub fn stub_0x2432c8() -> ! {
    todo!("0x2432c8 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEED0Ev")
}

// 0x2432d4 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEE7disposeEv
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::dispose(void)")]
pub fn stub_0x2432d4() -> ! {
    todo!("0x2432d4 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEE7disposeEv")
}

// 0x2432e8 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
// was: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::get_deleter(std::type_info const&)")]
pub fn stub_0x2432e8() -> ! {
    todo!("0x2432e8 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEE11get_deleterERKSt9type_info")
}

// 0x243300 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEE19get_untyped_deleterEv
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::get_untyped_deleter(void)")]
pub fn stub_0x243300() -> ! {
    todo!("0x243300 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEE19get_untyped_deleterEv")
}

// 0x243304 — __ZN5boost22condition_variable_anyC2Ev
// type: boost::condition_variable_any *__fastcall(boost::condition_variable_any *this)
// was: boost::condition_variable_any *__fastcall(boost::condition_variable_any *this)
#[doc(alias = "boost::condition_variable_any::condition_variable_any(void)")]
pub fn stub_0x243304() -> ! {
    todo!("0x243304 __ZN5boost22condition_variable_anyC2Ev")
}

// 0x2434dc — __GLOBAL__I_a_44
#[doc(alias = "global constructor keyed to_a_44")]
pub fn stub_0x2434dc() -> ! {
    todo!("0x2434dc __GLOBAL__I_a_44")
}

// 0x2435a4 — __ZN3RBX6CEvent4WaitEv
// type: int __fastcall(RBX::CEvent *this, int, int)
// was: int __fastcall(RBX::CEvent *this, int, int)
#[doc(alias = "RBX::CEvent::Wait(void)")]
pub fn stub_0x2435a4() -> ! {
    todo!("0x2435a4 __ZN3RBX6CEvent4WaitEv")
}

// 0x2435b4 — __ZN3RBX6CEvent19WaitForSingleObjectERS0_i
// type: int __fastcall(RBX::CEvent *this, int, int)
// was: int __fastcall(RBX::CEvent *this, int, int)
#[doc(alias = "RBX::CEvent::WaitForSingleObject(RBX::CEvent&,int)")]
pub fn stub_0x2435b4() -> ! {
    todo!("0x2435b4 __ZN3RBX6CEvent19WaitForSingleObjectERS0_i")
}

// 0x24381c — __ZN3RBX6CEvent4WaitEi
// type: bool __fastcall(RBX::CEvent *this, int, int)
// was: bool __fastcall(RBX::CEvent *this, int, int)
#[doc(alias = "RBX::CEvent::Wait(int)")]
pub fn stub_0x24381c() -> ! {
    todo!("0x24381c __ZN3RBX6CEvent4WaitEi")
}

// 0x243830 — __ZN3RBX6CEventD1Ev
// type: void __fastcall(RBX::CEvent *__hidden this)
// was: void __fastcall(RBX::CEvent *__hidden this)
#[doc(alias = "RBX::CEvent::~CEvent()")]
pub fn stub_0x243830() -> ! {
    todo!("0x243830 __ZN3RBX6CEventD1Ev")
}

// 0x24383c — __ZN3RBX6CEventD2Ev
// type: void __fastcall(RBX::CEvent *__hidden this)
// was: void __fastcall(RBX::CEvent *__hidden this)
#[doc(alias = "RBX::CEvent::~CEvent() [0x24383c]")]
pub fn stub_0x24383c() -> ! {
    todo!("0x24383c __ZN3RBX6CEventD2Ev")
}

// 0x243944 — __ZN3RBX6CEventC1Eb
// type: RBX::CEvent *__fastcall(RBX::CEvent *this, bool)
// was: RBX::CEvent *__fastcall(RBX::CEvent *this, bool)
#[doc(alias = "RBX::CEvent::CEvent(bool)")]
pub fn stub_0x243944() -> ! {
    todo!("0x243944 __ZN3RBX6CEventC1Eb")
}

// 0x243a30 — __ZN3RBX6CEvent3SetEv
// type: void __fastcall(RBX::CEvent *this)
// was: void __fastcall(RBX::CEvent *this)
#[doc(alias = "RBX::CEvent::Set(void)")]
pub fn stub_0x243a30() -> ! {
    todo!("0x243a30 __ZN3RBX6CEvent3SetEv")
}

// 0x243b84 — __ZN5boost18condition_variable13do_wait_untilERNS_11unique_lockINS_5mutexEEERK8timespec
// type: int __fastcall(int, int, const timespec *)
// was: int __fastcall(int, int, const timespec *)
#[doc(alias = "boost::condition_variable::do_wait_until(boost::unique_lock<boost::mutex> &,timespec const&)")]
pub fn stub_0x243b84() -> ! {
    todo!("0x243b84 __ZN5boost18condition_variable13do_wait_untilERNS_11unique_lockINS_5mutexEEERK8timespec")
}

// 0x243dd0 — __GLOBAL__I_a_45
#[doc(alias = "global constructor keyed to_a_45")]
pub fn stub_0x243dd0() -> ! {
    todo!("0x243dd0 __GLOBAL__I_a_45")
}

// 0x243e98 — __ZN3RBX6Limits9CountableC2Ev
// type: RBX::Limits::Countable *__fastcall(RBX::Limits::Countable *this, int, int, int)
// was: RBX::Limits::Countable *__fastcall(RBX::Limits::Countable *this, int, int, int)
#[doc(alias = "RBX::Limits::Countable::Countable(void)")]
pub fn stub_0x243e98() -> ! {
    todo!("0x243e98 __ZN3RBX6Limits9CountableC2Ev")
}

// 0x244088 — __ZN3RBX6Limits7Counter3addEPNS0_9CountableE
// type: void __fastcall(int32_t *, volatile int *)
// was: void __fastcall(int32_t *, volatile int *)
#[doc(alias = "RBX::Limits::Counter::add(RBX::Limits::Countable *)")]
pub fn stub_0x244088() -> ! {
    todo!("0x244088 __ZN3RBX6Limits7Counter3addEPNS0_9CountableE")
}

// 0x244200 — __ZN3RBX6Limits9CountableD2Ev
// type: void __fastcall(int32_t **this, volatile int *)
// was: void __fastcall(int32_t **this, volatile int *)
#[doc(alias = "RBX::Limits::Countable::~Countable()")]
pub fn stub_0x244200() -> ! {
    todo!("0x244200 __ZN3RBX6Limits9CountableD2Ev")
}

// 0x2442c4 — __ZN3RBX6Limits7Counter15getCurrentCountEv
// type: _DWORD __fastcall(RBX::Limits::Counter *__hidden this)
// was: _DWORD __fastcall(RBX::Limits::Counter *__hidden this)
#[doc(alias = "RBX::Limits::Counter::getCurrentCount(void)")]
pub fn stub_0x2442c4() -> ! {
    todo!("0x2442c4 __ZN3RBX6Limits7Counter15getCurrentCountEv")
}

// 0x244358 — __ZN3RBX6Limits7Counter6canAddEi
// type: bool __fastcall(RBX::Limits::Counter *this, int)
// was: bool __fastcall(RBX::Limits::Counter *this, int)
#[doc(alias = "RBX::Limits::Counter::canAdd(int)")]
pub fn stub_0x244358() -> ! {
    todo!("0x244358 __ZN3RBX6Limits7Counter6canAddEi")
}

// 0x244384 — __ZN3RBX6Limits7Counter9ActivatorC1EN5boost10shared_ptrIS1_EE
#[doc(alias = "RBX::Limits::Counter::Activator::Activator(rbx_core::SharedPtr<RBX::Limits::Counter>)")]
pub fn stub_0x244384() -> ! {
    todo!("0x244384 __ZN3RBX6Limits7Counter9ActivatorC1EN5boost10shared_ptrIS1_EE")
}

// 0x244390 — __ZN3RBX6Limits7Counter9ActivatorC2EN5boost10shared_ptrIS1_EE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, void *, int, int, int, int)
// was: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, void *, int, int, int, int)
#[doc(alias = "RBX::Limits::Counter::Activator::Activator(rbx_core::SharedPtr<RBX::Limits::Counter>) [0x244390]")]
pub fn stub_0x244390() -> ! {
    todo!("0x244390 __ZN3RBX6Limits7Counter9ActivatorC2EN5boost10shared_ptrIS1_EE")
}

// 0x2445fc — __ZN3RBX6Limits7Counter9ActivatorD1Ev
// type: void __fastcall(RBX::Limits::Counter::Activator *__hidden this)
// was: void __fastcall(RBX::Limits::Counter::Activator *__hidden this)
#[doc(alias = "RBX::Limits::Counter::Activator::~Activator()")]
pub fn stub_0x2445fc() -> ! {
    todo!("0x2445fc __ZN3RBX6Limits7Counter9ActivatorD1Ev")
}

// 0x244608 — __ZN3RBX6Limits7Counter9ActivatorD2Ev
// type: void __fastcall(RBX::Limits::Counter::Activator *this, int, int, int)
// was: void __fastcall(RBX::Limits::Counter::Activator *this, int, int, int)
#[doc(alias = "RBX::Limits::Counter::Activator::~Activator() [0x244608]")]
pub fn stub_0x244608() -> ! {
    todo!("0x244608 __ZN3RBX6Limits7Counter9ActivatorD2Ev")
}

// 0x24480c — __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE5resetEPS5_
// type: void __fastcall(int *, const void *)
// was: void __fastcall(int *, const void *)
#[doc(alias = "boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::reset(rbx_core::SharedPtr<RBX::Limits::Counter>*)")]
pub fn stub_0x24480c() -> ! {
    todo!("0x24480c __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE5resetEPS5_")
}

// 0x244928 — __ZN3RBX6Limits7Counter24safe_static_init_currentEv
// type: int __fastcall(RBX::Limits::Counter *this)
// was: int __fastcall(RBX::Limits::Counter *this)
#[doc(alias = "RBX::Limits::Counter::safe_static_init_current(void)")]
pub fn stub_0x244928() -> ! {
    todo!("0x244928 __ZN3RBX6Limits7Counter24safe_static_init_currentEv")
}

// 0x244934 — __ZN3RBX6Limits7Counter26safe_static_do_get_currentEv
// type: int *__fastcall(RBX::Limits::Counter *this)
// was: int *__fastcall(RBX::Limits::Counter *this)
#[doc(alias = "RBX::Limits::Counter::safe_static_do_get_current(void)")]
pub fn stub_0x244934() -> ! {
    todo!("0x244934 __ZN3RBX6Limits7Counter26safe_static_do_get_currentEv")
}

// 0x244ab8 — __ZN3rbx26thread_specific_shared_ptrIN3RBX6Limits7CounterEED1Ev
#[doc(alias = "rbx::thread_specific_shared_ptr<RBX::Limits::Counter>::~thread_specific_shared_ptr()")]
pub fn stub_0x244ab8() -> ! {
    todo!("0x244ab8 __ZN3rbx26thread_specific_shared_ptrIN3RBX6Limits7CounterEED1Ev")
}

// 0x244ac8 — __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEED2Ev
// type: boost::_anonymous_namespace_ *__fastcall(boost::_anonymous_namespace_ *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: boost::_anonymous_namespace_ *__fastcall(boost::_anonymous_namespace_ *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::~thread_specific_ptr()")]
pub fn stub_0x244ac8() -> ! {
    todo!("0x244ac8 __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEED2Ev")
}

// 0x244bbc — __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataD1Ev
// type: void()
// was: void()
#[doc(alias = "boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data::~delete_data()")]
pub fn stub_0x244bbc() -> ! {
    todo!("0x244bbc __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataD1Ev")
}

// 0x244bc0 — __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataD0Ev
// type: void __fastcall(void *)
// was: void __fastcall(void *)
#[doc(alias = "boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data::~delete_data() [0x244bc0]")]
pub fn stub_0x244bc0() -> ! {
    todo!("0x244bc0 __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataD0Ev")
}

// 0x244bcc — __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataclEPv
// type: void __fastcall(int, _DWORD *)
// was: void __fastcall(int, _DWORD *)
#[doc(alias = "boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data::operator()(void *)")]
pub fn stub_0x244bcc() -> ! {
    todo!("0x244bcc __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataclEPv")
}

// 0x244c74 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEED1Ev
// type: void()
// was: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::~sp_counted_impl_pd()")]
pub fn stub_0x244c74() -> ! {
    todo!("0x244c74 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEED1Ev")
}

// 0x244c78 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEED0Ev
// type: void __fastcall(void *)
// was: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::~sp_counted_impl_pd() [0x244c78]")]
pub fn stub_0x244c78() -> ! {
    todo!("0x244c78 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEED0Ev")
}

// 0x244c84 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEE7disposeEv
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::dispose(void)")]
pub fn stub_0x244c84() -> ! {
    todo!("0x244c84 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEE7disposeEv")
}

// 0x244c98 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
// was: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::get_deleter(std::type_info const&)")]
pub fn stub_0x244c98() -> ! {
    todo!("0x244c98 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEE11get_deleterERKSt9type_info")
}

// 0x244cb0 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEE19get_untyped_deleterEv
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::get_untyped_deleter(void)")]
pub fn stub_0x244cb0() -> ! {
    todo!("0x244cb0 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEE19get_untyped_deleterEv")
}

// 0x244cb4 — __GLOBAL__I_a_46
#[doc(alias = "global constructor keyed to_a_46")]
pub fn stub_0x244cb4() -> ! {
    todo!("0x244cb4 __GLOBAL__I_a_46")
}

// 0x244d7c — __ZN3RBX16roblox_allocator6mallocEm
// type: void *__fastcall(size_t this, unsigned int)
// was: void *__fastcall(size_t this, unsigned int)
#[doc(alias = "RBX::roblox_allocator::malloc(unsigned long)")]
pub fn stub_0x244d7c() -> ! {
    todo!("0x244d7c __ZN3RBX16roblox_allocator6mallocEm")
}

// 0x244dac — __ZN3RBX16roblox_allocator4freeEPc
// type: void __fastcall(RBX::roblox_allocator *this, char *)
// was: void __fastcall(RBX::roblox_allocator *this, char *)
#[doc(alias = "RBX::roblox_allocator::free(char *)")]
pub fn stub_0x244dac() -> ! {
    todo!("0x244dac __ZN3RBX16roblox_allocator4freeEPc")
}

// 0x244db8 — __ZNSt6vectorIPmSaIS0_EED1Ev
// type: void **__fastcall(void **)
// was: void **__fastcall(void **)
#[doc(alias = "std::vector<unsigned long *,std::allocator<unsigned long *>>::~vector()")]
pub fn stub_0x244db8() -> ! {
    todo!("0x244db8 __ZNSt6vectorIPmSaIS0_EED1Ev")
}

// 0x244dcc — __ZNSt6vectorIPFbvESaIS1_EED1Ev
// type: void **__fastcall(void **)
// was: void **__fastcall(void **)
#[doc(alias = "std::vector<bool (*)(void),std::allocator<bool (*)(void)>>::~vector()")]
pub fn stub_0x244dcc() -> ! {
    todo!("0x244dcc __ZNSt6vectorIPFbvESaIS1_EED1Ev")
}

// 0x244de0 — __GLOBAL__I_a_47
// type: int()
// was: int()
#[doc(alias = "global constructor keyed to_a_47")]
pub fn stub_0x244de0() -> ! {
    todo!("0x244de0 __GLOBAL__I_a_47")
}

// 0x244e94 — __ZNK3rbx7signals10connection10disconnectEv
// type: void __fastcall(int32_t **this)
// was: void __fastcall(int32_t **this)
#[doc(alias = "rbx::signals::connection::disconnect(void)const")]
pub fn stub_0x244e94() -> ! {
    todo!("0x244e94 __ZNK3rbx7signals10connection10disconnectEv")
}

// 0x244fd4 — __ZNK3rbx7signals10connection9connectedEv
// type: int __fastcall(rbx::signals::connection *this)
// was: int __fastcall(rbx::signals::connection *this)
#[doc(alias = "rbx::signals::connection::connected(void)const")]
pub fn stub_0x244fd4() -> ! {
    todo!("0x244fd4 __ZNK3rbx7signals10connection9connectedEv")
}

// 0x245118 — __ZNK3rbx7signals10connectioneqERKS1_
// type: bool __fastcall(int32_t, int32_t **, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
// was: bool __fastcall(int32_t, int32_t **, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "rbx::signals::connection::operator==(rbx::signals::connection const&)const")]
pub fn stub_0x245118() -> ! {
    todo!("0x245118 __ZNK3rbx7signals10connectioneqERKS1_")
}

// 0x2452d0 — __ZNK3rbx7signals10connectionneERKS1_
// type: bool __fastcall(int32_t, int32_t **, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
// was: bool __fastcall(int32_t, int32_t **, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "rbx::signals::connection::operator!=(rbx::signals::connection const&)const")]
pub fn stub_0x2452d0() -> ! {
    todo!("0x2452d0 __ZNK3rbx7signals10connectionneERKS1_")
}

// 0x245488 — __ZN3rbx7signals10connectionaSERKS1_
// type: int *__fastcall(int *, int *)
// was: int *__fastcall(int *, int *)
#[doc(alias = "rbx::signals::connection::operator=(rbx::signals::connection const&)")]
pub fn stub_0x245488() -> ! {
    todo!("0x245488 __ZN3rbx7signals10connectionaSERKS1_")
}

// 0x24551c — __ZN5boost8functionIFvRSt9exceptionEED1Ev
// type: int *__fastcall(int *)
// was: int *__fastcall(int *)
#[doc(alias = "boost::function<void ()(std::exception &)>::~function()")]
pub fn stub_0x24551c() -> ! {
    todo!("0x24551c __ZN5boost8functionIFvRSt9exceptionEED1Ev")
}

// 0x245544 — __ZN4Init14initStaticDataEv
// type: void __fastcall(Init *this)
// was: void __fastcall(Init *this)
#[doc(alias = "Init::initStaticData(void)")]
pub fn stub_0x245544() -> ! {
    todo!("0x245544 __ZN4Init14initStaticDataEv")
}

// 0x245548 — __GLOBAL__I_a_48
#[doc(alias = "global constructor keyed to_a_48")]
pub fn stub_0x245548() -> ! {
    todo!("0x245548 __GLOBAL__I_a_48")
}

// 0x2456a0 — __ZN3RBX5Tasks12SequenceBase11isInhibitedEPNS_13TaskScheduler3JobE
// type: bool __fastcall(RBX::Tasks::SequenceBase *this, RBX::TaskScheduler::Job *)
// was: bool __fastcall(RBX::Tasks::SequenceBase *this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::Tasks::SequenceBase::isInhibited(RBX::TaskScheduler::Job *)")]
pub fn stub_0x2456a0() -> ! {
    todo!("0x2456a0 __ZN3RBX5Tasks12SequenceBase11isInhibitedEPNS_13TaskScheduler3JobE")
}

// 0x2456d8 — __ZN3RBX5Tasks12SequenceBase7advanceEv
// type: int __fastcall(RBX::Tasks::SequenceBase *this)
// was: int __fastcall(RBX::Tasks::SequenceBase *this)
#[doc(alias = "RBX::Tasks::SequenceBase::advance(void)")]
pub fn stub_0x2456d8() -> ! {
    todo!("0x2456d8 __ZN3RBX5Tasks12SequenceBase7advanceEv")
}

// 0x245708 — __ZN3RBX5Tasks12SequenceBase7onAddedEPNS_13TaskScheduler3JobE
// type: void __fastcall(RBX::Tasks::SequenceBase *this, RBX::TaskScheduler::Job *)
// was: void __fastcall(RBX::Tasks::SequenceBase *this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::Tasks::SequenceBase::onAdded(RBX::TaskScheduler::Job *)")]
pub fn stub_0x245708() -> ! {
    todo!("0x245708 __ZN3RBX5Tasks12SequenceBase7onAddedEPNS_13TaskScheduler3JobE")
}

// 0x2457f0 — __ZN3RBX5Tasks12SequenceBase9onRemovedEPNS_13TaskScheduler3JobE
// type: int __fastcall(RBX::Tasks::SequenceBase *this, RBX::TaskScheduler::Job *)
// was: int __fastcall(RBX::Tasks::SequenceBase *this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::Tasks::SequenceBase::onRemoved(RBX::TaskScheduler::Job *)")]
pub fn stub_0x2457f0() -> ! {
    todo!("0x2457f0 __ZN3RBX5Tasks12SequenceBase9onRemovedEPNS_13TaskScheduler3JobE")
}

// 0x245848 — __ZNSt6vectorIPN3RBX13TaskScheduler3JobESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: void *__fastcall(int, char *__src, _DWORD *)
// was: void *__fastcall(int, char *__src, _DWORD *)
#[doc(alias = "std::vector<RBX::TaskScheduler::Job *,std::allocator<RBX::TaskScheduler::Job *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::Job **,std::vector<RBX::TaskScheduler::Job *,std::allocator<RBX::TaskScheduler::Job *>>>,RBX::TaskScheduler::Job * const&)")]
pub fn stub_0x245848() -> ! {
    todo!("0x245848 __ZNSt6vectorIPN3RBX13TaskScheduler3JobESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")
}

// 0x245940 — __GLOBAL__I_a_49
#[doc(alias = "global constructor keyed to_a_49")]
pub fn stub_0x245940() -> ! {
    todo!("0x245940 __GLOBAL__I_a_49")
}

// 0x245a08 — __ZNK3RBX13TaskScheduler30getSchedulerDutyCyclePerThreadEv
// type: __int64 __fastcall(RBX::TaskScheduler *this)
// was: __int64 __fastcall(RBX::TaskScheduler *this)
#[doc(alias = "RBX::TaskScheduler::getSchedulerDutyCyclePerThread(void)const")]
pub fn stub_0x245a08() -> ! {
    todo!("0x245a08 __ZNK3RBX13TaskScheduler30getSchedulerDutyCyclePerThreadEv")
}

// 0x245ab0 — __ZN3RBX16ExclusiveArbiter12areExclusiveEPNS_13TaskScheduler3JobES3_
// type: int __fastcall(RBX::ExclusiveArbiter *this, RBX::TaskScheduler::Job *, RBX::TaskScheduler::Job *)
// was: int __fastcall(RBX::ExclusiveArbiter *this, RBX::TaskScheduler::Job *, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::ExclusiveArbiter::areExclusive(RBX::TaskScheduler::Job *,RBX::TaskScheduler::Job *)")]
pub fn stub_0x245ab0() -> ! {
    todo!("0x245ab0 __ZN3RBX16ExclusiveArbiter12areExclusiveEPNS_13TaskScheduler3JobES3_")
}

// 0x245b68 — __ZN3RBX13TaskScheduler11static_initEv
// type: void __fastcall(RBX::TaskScheduler *this, int, int, int)
// was: void __fastcall(RBX::TaskScheduler *this, int, int, int)
#[doc(alias = "RBX::TaskScheduler::static_init(void)")]
pub fn stub_0x245b68() -> ! {
    todo!("0x245b68 __ZN3RBX13TaskScheduler11static_initEv")
}

// 0x245c64 — __ZN3RBX13TaskSchedulerD1Ev
// type: void __fastcall(RBX::TaskScheduler *__hidden this)
// was: void __fastcall(RBX::TaskScheduler *__hidden this)
#[doc(alias = "RBX::TaskScheduler::~TaskScheduler()")]
pub fn stub_0x245c64() -> ! {
    todo!("0x245c64 __ZN3RBX13TaskSchedulerD1Ev")
}

// 0x245c70 — __ZN3RBX13TaskScheduler9singletonEv
// type: _DWORD __fastcall(RBX::TaskScheduler *__hidden this)
// was: _DWORD __fastcall(RBX::TaskScheduler *__hidden this)
#[doc(alias = "RBX::TaskScheduler::singleton(void)")]
pub fn stub_0x245c70() -> ! {
    todo!("0x245c70 __ZN3RBX13TaskScheduler9singletonEv")
}

// 0x245c94 — __ZN3RBX13TaskSchedulerC2Ev
// type: int __fastcall(RBX::TaskScheduler *this, int, int)
// was: int __fastcall(RBX::TaskScheduler *this, int, int)
#[doc(alias = "RBX::TaskScheduler::TaskScheduler(void)")]
pub fn stub_0x245c94() -> ! {
    todo!("0x245c94 __ZN3RBX13TaskSchedulerC2Ev")
}

// 0x246308 — __ZN3RBX13TaskScheduler21sampleRunningJobCountEv
// type: bool __fastcall(RBX::TaskScheduler *this, int, int)
// was: bool __fastcall(RBX::TaskScheduler *this, int, int)
#[doc(alias = "RBX::TaskScheduler::sampleRunningJobCount(void)")]
pub fn stub_0x246308() -> ! {
    todo!("0x246308 __ZN3RBX13TaskScheduler21sampleRunningJobCountEv")
}

// 0x246358 — __ZN3RBX13TaskSchedulerD2Ev
// type: void __fastcall(RBX::TaskScheduler *this, int, int, const void *)
// was: void __fastcall(RBX::TaskScheduler *this, int, int, const void *)
#[doc(alias = "RBX::TaskScheduler::~TaskScheduler() [0x246358]")]
pub fn stub_0x246358() -> ! {
    todo!("0x246358 __ZN3RBX13TaskSchedulerD2Ev")
}

// 0x2467d0 — __ZN3RBX13TaskScheduler6removeEN5boost10shared_ptrINS0_3JobEEEbNS1_8functionIFvvEEE
// type: void __fastcall(int, int *, unsigned __int8, int)
// was: void __fastcall(int, int *, unsigned __int8, int)
#[doc(alias = "RBX::TaskScheduler::remove(rbx_core::SharedPtr<RBX::TaskScheduler::Job>,bool,boost::function<void ()(void)>)")]
pub fn stub_0x2467d0() -> ! {
    todo!("0x2467d0 __ZN3RBX13TaskScheduler6removeEN5boost10shared_ptrINS0_3JobEEEbNS1_8functionIFvvEEE")
}

// 0x246a48 — __ZN3RBX13TaskScheduler6removeERKN5boost10shared_ptrINS0_3JobEEENS2_INS_6CEventEEE
// type: void __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int)
// was: void __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "RBX::TaskScheduler::remove(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&,rbx_core::SharedPtr<RBX::CEvent>)")]
pub fn stub_0x246a48() -> ! {
    todo!("0x246a48 __ZN3RBX13TaskScheduler6removeERKN5boost10shared_ptrINS0_3JobEEENS2_INS_6CEventEEE")
}

// 0x246da8 — __ZN3RBX13TaskScheduler10rescheduleEN5boost10shared_ptrINS0_3JobEEE
// type: void __fastcall(int, RBX::TaskScheduler::Job **)
// was: void __fastcall(int, RBX::TaskScheduler::Job **)
#[doc(alias = "RBX::TaskScheduler::reschedule(rbx_core::SharedPtr<RBX::TaskScheduler::Job>)")]
pub fn stub_0x246da8() -> ! {
    todo!("0x246da8 __ZN3RBX13TaskScheduler10rescheduleEN5boost10shared_ptrINS0_3JobEEE")
}

// 0x246e98 — __ZN3RBX13TaskScheduler11scheduleJobERNS0_3JobE
// type: int __fastcall(RBX::TaskScheduler *this, RBX::TaskScheduler::Job *)
// was: int __fastcall(RBX::TaskScheduler *this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::TaskScheduler::scheduleJob(RBX::TaskScheduler::Job &)")]
pub fn stub_0x246e98() -> ! {
    todo!("0x246e98 __ZN3RBX13TaskScheduler11scheduleJobERNS0_3JobE")
}

// 0x246f90 — __ZN3RBX13TaskScheduler3addEN5boost10shared_ptrINS0_3JobEEE
// type: void __fastcall(int, int, int, int, int, pthread_mutex_t *, int, int, int, int)
// was: void __fastcall(int, int, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "RBX::TaskScheduler::add(rbx_core::SharedPtr<RBX::TaskScheduler::Job>)")]
pub fn stub_0x246f90() -> ! {
    todo!("0x246f90 __ZN3RBX13TaskScheduler3addEN5boost10shared_ptrINS0_3JobEEE")
}

// 0x24710c — __ZN3RBX13TaskScheduler20incrementThreadCountEv
// type: int __fastcall(int32_t *this, volatile int *)
// was: int __fastcall(int32_t *this, volatile int *)
#[doc(alias = "RBX::TaskScheduler::incrementThreadCount(void)")]
pub fn stub_0x24710c() -> ! {
    todo!("0x24710c __ZN3RBX13TaskScheduler20incrementThreadCountEv")
}

// 0x24711c — __ZN3RBX13TaskScheduler20decrementThreadCountEv
// type: int __fastcall(int32_t *this, volatile int *)
// was: int __fastcall(int32_t *this, volatile int *)
#[doc(alias = "RBX::TaskScheduler::decrementThreadCount(void)")]
pub fn stub_0x24711c() -> ! {
    todo!("0x24711c __ZN3RBX13TaskScheduler20decrementThreadCountEv")
}

// 0x247130 — __ZNK3RBX13TaskScheduler20getShortestSleepTimeEv
// type: int __fastcall(RBX::TaskScheduler *this, int)
// was: int __fastcall(RBX::TaskScheduler *this, int)
#[doc(alias = "RBX::TaskScheduler::getShortestSleepTime(void)const")]
pub fn stub_0x247130() -> ! {
    todo!("0x247130 __ZNK3RBX13TaskScheduler20getShortestSleepTimeEv")
}

// 0x247154 — __ZN3RBX13TaskScheduler16wakeSleepingJobsEv
// type: int __fastcall(RBX::TaskScheduler *this)
// was: int __fastcall(RBX::TaskScheduler *this)
#[doc(alias = "RBX::TaskScheduler::wakeSleepingJobs(void)")]
pub fn stub_0x247154() -> ! {
    todo!("0x247154 __ZN3RBX13TaskScheduler16wakeSleepingJobsEv")
}

// 0x247220 — __ZN3RBX13TaskScheduler12findJobToRunEN5boost10shared_ptrINS0_6ThreadEEE
// type: void __fastcall(RBX::TaskScheduler::Job **, int, int *, int, int, int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: void __fastcall(RBX::TaskScheduler::Job **, int, int *, int, int, int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::TaskScheduler::findJobToRun(rbx_core::SharedPtr<RBX::TaskScheduler::Thread>)")]
pub fn stub_0x247220() -> ! {
    todo!("0x247220 __ZN3RBX13TaskScheduler12findJobToRunEN5boost10shared_ptrINS0_6ThreadEEE")
}

// 0x247bd8 — __ZN3rbx25thread_specific_referenceIN3RBX13TaskScheduler3JobEED1Ev
#[doc(alias = "rbx::thread_specific_reference<RBX::TaskScheduler::Job>::~thread_specific_reference()")]
pub fn stub_0x247bd8() -> ! {
    todo!("0x247bd8 __ZN3rbx25thread_specific_referenceIN3RBX13TaskScheduler3JobEED1Ev")
}

// 0x247be8 — __ZNK3RBX13TaskScheduler3Job12getDebugNameEv
// type: void __fastcall(RBX::TaskScheduler::Job *this, int)
// was: void __fastcall(RBX::TaskScheduler::Job *this, int)
#[doc(alias = "RBX::TaskScheduler::Job::getDebugName(void)const")]
pub fn stub_0x247be8() -> ! {
    todo!("0x247be8 __ZNK3RBX13TaskScheduler3Job12getDebugNameEv")
}

// 0x247db0 — __ZN3RBX14RunningAverageIidE6sampleEi
// type: _DWORD *__fastcall(int, int)
// was: _DWORD *__fastcall(int, int)
#[doc(alias = "RBX::RunningAverage<int,double>::sample(int)")]
pub fn stub_0x247db0() -> ! {
    todo!("0x247db0 __ZN3RBX14RunningAverageIidE6sampleEi")
}

// 0x247e74 — __ZN3RBX16ExclusiveArbiter11arbiterNameEv
// type: int __fastcall(RBX::ExclusiveArbiter *this)
// was: int __fastcall(RBX::ExclusiveArbiter *this)
#[doc(alias = "RBX::ExclusiveArbiter::arbiterName(void)")]
pub fn stub_0x247e74() -> ! {
    todo!("0x247e74 __ZN3RBX16ExclusiveArbiter11arbiterNameEv")
}

// 0x247e90 — __ZN3RBX16ExclusiveArbiter11isThrottledEv
// type: int __fastcall(RBX::ExclusiveArbiter *this)
// was: int __fastcall(RBX::ExclusiveArbiter *this)
#[doc(alias = "RBX::ExclusiveArbiter::isThrottled(void)")]
pub fn stub_0x247e90() -> ! {
    todo!("0x247e90 __ZN3RBX16ExclusiveArbiter11isThrottledEv")
}

// 0x247e94 — __ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEC2IS3_EERKNS_8weak_ptrIT_EE
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
// was: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Job>::shared_ptr<RBX::TaskScheduler::Job>(rbx_core::WeakPtr<RBX::TaskScheduler::Job> const&)")]
pub fn stub_0x247e94() -> ! {
    todo!("0x247e94 __ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEC2IS3_EERKNS_8weak_ptrIT_EE")
}

// 0x247fac — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_ESD_
// type: _Rb_tree_node_base *__fastcall(_DWORD *, _Rb_tree_node_base *, _Rb_tree_node_base *)
// was: _Rb_tree_node_base *__fastcall(_DWORD *, _Rb_tree_node_base *, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::erase(std::_Rb_tree_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::_Rb_tree_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>)")]
pub fn stub_0x247fac() -> ! {
    todo!("0x247fac __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_ESD_")
}

// 0x248020 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: void __fastcall(int, _DWORD *)
// was: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_erase(std::_Rb_tree_node<rbx_core::SharedPtr<RBX::TaskScheduler::Job>> *)")]
pub fn stub_0x248020() -> ! {
    todo!("0x248020 __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}

// 0x248050 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, _DWORD *, unsigned int M_parent, int)
// was: int __fastcall(int, _DWORD *, unsigned int M_parent, int)
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_insert_unique(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&)")]
pub fn stub_0x248050() -> ! {
    todo!("0x248050 __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE16_M_insert_uniqueERKS5_")
}

// 0x248104 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE14_M_create_nodeERKS5_
// type: int __fastcall(int, int *, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, void *, int)
// was: int __fastcall(int, int *, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_create_node(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&)")]
pub fn stub_0x248104() -> ! {
    todo!("0x248104 __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE14_M_create_nodeERKS5_")
}

// 0x248224 — __ZN5boost6detail12shared_countC2IN3RBX6CEventEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
// was: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::CEvent>(RBX::CEvent *)")]
pub fn stub_0x248224() -> ! {
    todo!("0x248224 __ZN5boost6detail12shared_countC2IN3RBX6CEventEEEPT_")
}

// 0x24831c — __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEED1Ev
// type: void()
// was: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CEvent>::~sp_counted_impl_p()")]
pub fn stub_0x24831c() -> ! {
    todo!("0x24831c __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEED1Ev")
}

// 0x248320 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEED0Ev
// type: void __fastcall(void *)
// was: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CEvent>::~sp_counted_impl_p() [0x248320]")]
pub fn stub_0x248320() -> ! {
    todo!("0x248320 __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEED0Ev")
}

// 0x24832c — __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEE7disposeEv
// type: void __fastcall(int)
// was: void __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CEvent>::dispose(void)")]
pub fn stub_0x24832c() -> ! {
    todo!("0x24832c __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEE7disposeEv")
}

// 0x24834c — __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEE11get_deleterERKSt9type_info
// type: int()
// was: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CEvent>::get_deleter(std::type_info const&)")]
pub fn stub_0x24834c() -> ! {
    todo!("0x24834c __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEE11get_deleterERKSt9type_info")
}

// 0x248350 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEE19get_untyped_deleterEv
// type: int()
// was: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CEvent>::get_untyped_deleter(void)")]
pub fn stub_0x248350() -> ! {
    todo!("0x248350 __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEE19get_untyped_deleterEv")
}

// 0x248358 — __ZN5boost6detail11thread_dataINS_9function0IvEEED1Ev
// type: int __fastcall(boost::detail::thread_data_base *)
// was: int __fastcall(boost::detail::thread_data_base *)
#[doc(alias = "boost::detail::thread_data<boost::function0<void>>::~thread_data()")]
pub fn stub_0x248358() -> ! {
    todo!("0x248358 __ZN5boost6detail11thread_dataINS_9function0IvEEED1Ev")
}

// 0x248448 — __ZN5boost18condition_variableC2Ev
// type: boost::condition_variable *__fastcall(boost::condition_variable *this)
// was: boost::condition_variable *__fastcall(boost::condition_variable *this)
#[doc(alias = "boost::condition_variable::condition_variable(void)")]
pub fn stub_0x248448() -> ! {
    todo!("0x248448 __ZN5boost18condition_variableC2Ev")
}

// 0x248620 — __ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_9function0IvEEEEEEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int)
// was: void __fastcall(_DWORD *, int, int)
#[doc(alias = "void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::function0<void>>>(rbx_core::SharedPtr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::function0<void>> *)const")]
pub fn stub_0x248620() -> ! {
    todo!("0x248620 __ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_9function0IvEEEEEEvPKNS_10shared_ptrIT_EEPT0_")
}

// 0x248778 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_9function0IvEEEEE19get_untyped_deleterEv
// type: int()
// was: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::function0<void>>>::get_untyped_deleter(void)")]
pub fn stub_0x248778() -> ! {
    todo!("0x248778 __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_9function0IvEEEEE19get_untyped_deleterEv")
}

// 0x24877c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskSchedulerEEENS3_5list1INS3_5valueIPS8_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
// was: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler>,boost::_bi::list1<boost::_bi::value<RBX::TaskScheduler*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x24877c() -> ! {
    todo!("0x24877c __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskSchedulerEEENS3_5list1INS3_5valueIPS8_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE")
}

// 0x2487dc — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskSchedulerEEENS3_5list1INS3_5valueIPS8_EEEEEEvE6invokeERNS1_15function_bufferE
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler>,boost::_bi::list1<boost::_bi::value<RBX::TaskScheduler*>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_0x2487dc() -> ! {
    todo!("0x2487dc __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskSchedulerEEENS3_5list1INS3_5valueIPS8_EEEEEEvE6invokeERNS1_15function_bufferE")
}

// 0x2487f8 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEEC1ERKS5_
// type: int __fastcall(int, int)
// was: int __fastcall(int, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>> const&)")]
pub fn stub_0x2487f8() -> ! {
    todo!("0x2487f8 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEEC1ERKS5_")
}

// 0x248938 — __ZN5boost9function0IvE5dummy7nonnullEv
// type: void()
// was: void()
#[doc(alias = "boost::function0<void>::dummy::nonnull(void)")]
pub fn stub_0x248938() -> ! {
    todo!("0x248938 __ZN5boost9function0IvE5dummy7nonnullEv")
}

// 0x248940 — __ZN3RBX5mutexC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *this)
// was: pthread_mutex_t *__fastcall(pthread_mutex_t *this)
#[doc(alias = "RBX::mutex::mutex(void)")]
pub fn stub_0x248940() -> ! {
    todo!("0x248940 __ZN3RBX5mutexC2Ev")
}

// 0x248a8c — __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEED2Ev
// type: boost::_anonymous_namespace_ *__fastcall(boost::_anonymous_namespace_ *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: boost::_anonymous_namespace_ *__fastcall(boost::_anonymous_namespace_ *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::~thread_specific_ptr()")]
pub fn stub_0x248a8c() -> ! {
    todo!("0x248a8c __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEED2Ev")
}

// 0x248b80 — __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataD1Ev
// type: void()
// was: void()
#[doc(alias = "boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data::~delete_data()")]
pub fn stub_0x248b80() -> ! {
    todo!("0x248b80 __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataD1Ev")
}

// 0x248b84 — __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataD0Ev
// type: void __fastcall(void *)
// was: void __fastcall(void *)
#[doc(alias = "boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data::~delete_data() [0x248b84]")]
pub fn stub_0x248b84() -> ! {
    todo!("0x248b84 __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataD0Ev")
}

// 0x248b90 — __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataclEPv
// type: void __fastcall(int, void *)
// was: void __fastcall(int, void *)
#[doc(alias = "boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data::operator()(void *)")]
pub fn stub_0x248b90() -> ! {
    todo!("0x248b90 __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataclEPv")
}
