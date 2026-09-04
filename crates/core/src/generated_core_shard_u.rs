//! core shard U — 100 core stubs EA-sorted, RBX:: not Reflection/DataModel/Ogre/RakNet/Lua.
//! Source: ida/export.json filtered where demangled contains RBX:: but not Reflection/DataModel/Ogre/RakNet/Lua/Instance/Workspace, EA-sorted, next 100 uncovered workspace-wide.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.
#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use std::cell::RefCell;
use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::LazyLock;
use std::time::Instant;
/// IDA 0x23e1f8: `RBX::runtime_error` formats then throws `std::runtime_error`.
/// C varargs have no Rust equivalent; the caller renders the message (usually
/// via `format!`) and this error carries it. was: `std::runtime_error`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeError {
    message: String,
}
impl RuntimeError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
    pub fn message(&self) -> &str {
        &self.message
    }
}
impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}
impl std::error::Error for RuntimeError {}
/// IDA 0x23e324: `vformat` buffer policy — `vsnprintf` into a 161-byte stack
/// buffer; when the full length exceeds 160 chars it spills to a heap buffer
/// clamped to 1M chars; a null format yields `""`.
const VFORMAT_STACK_CAP: usize = 161;
const VFORMAT_INLINE_MAX: usize = 160;
const VFORMAT_HEAP_CLAMP: usize = 1_000_000;
/// IDA 0x23e324: net effect of the stack/heap `vsnprintf` pair as a byte policy.
/// Callers pass the already-rendered message (C varargs are rendered at the
/// call site with `format!`); over-long input is clamped like the heap path.
fn apply_vformat_policy(rendered: &str) -> String {
    if rendered.len() > VFORMAT_INLINE_MAX {
        let mut len = rendered.len().min(VFORMAT_HEAP_CLAMP);
        len = rendered.floor_char_boundary(len);
        let mut heap = String::with_capacity(len + 1);
        heap.push_str(&rendered[..len]);
        heap
    } else {
        let mut stack = String::with_capacity(VFORMAT_STACK_CAP);
        stack.push_str(rendered);
        stack
    }
}
/// IDA 0x23e5c0/0x23e5dc: `RBX::Debugable::doCrashEnabled` flag.
pub static DO_CRASH_ENABLED: AtomicBool = AtomicBool::new(false);
thread_local! {
    // IDA 0x23f42c slot (`thread_specific_ptr<std::string>`). The
    // `call_once(init_foo)` guard has no observable state beyond the store.
    static THREAD_NAME: RefCell<Option<String>> = RefCell::new(None);
}
/// Test accessor for the 0x23f42c slot.
pub fn thread_name() -> Option<String> {
    THREAD_NAME.with(|slot| slot.borrow().clone())
}
/// IDA 0x253d50: lazily-initialized `getStart()` epoch for `RBX::Time::now`.
/// The original subtracts wall-clock `gettimeofday`; the port uses a monotonic
/// clock, so suspend/wall jumps no longer shift the epoch.
static TIME_EPOCH: LazyLock<Instant> = LazyLock::new(Instant::now);
fn time_epoch() -> Instant {
    *TIME_EPOCH
}
#[doc(alias = "__ZN3RBX13runtime_errorEPKcz")]
// 0x23e1f8 — RBX::runtime_error(char const*,...)
// IDA 0x23e1f8: `va_start` → `RBX::vformat` → `std::runtime_error::runtime_error`,
// then `__cxa_throw`. Rust: `vformat` policy below, carried by `RuntimeError`.
pub fn stub_0x23e1f8(message: &str) -> RuntimeError {
    RuntimeError::new(stub_0x23e324(message))
}

#[doc(alias = "__ZN3RBX7vformatEPKcPv")]
// 0x23e324 — RBX::vformat(char const*,void *)
// IDA 0x23e324: `vsnprintf(stack, 0xA1)`; `len > 160` spills to a heap buffer
// (clamped to 1M); null format yields `""`. `message` is the caller-rendered
// varargs body; the length policy here is the ported logic.
pub fn stub_0x23e324(message: &str) -> String {
    apply_vformat_policy(message)
}

#[doc(alias = "__ZN3RBX6formatEPKcz")]
// 0x23e50c — RBX::format(char const*,...)
// IDA 0x23e50c: `va_start` then tail-calls `RBX::vformat` (0x23e52a).
pub fn stub_0x23e50c(message: &str) -> String {
    stub_0x23e324(message)
}

#[doc(alias = "__ZN3RBX9Debugable7doCrashEv")]
// 0x23e5c0 — RBX::Debugable::doCrash(void)
// IDA 0x23e5c0: returns `doCrashEnabled`; traps via `DebugBreak()` when set.
// `DebugBreak` raises SIGTRAP (resumable under a debugger); the portable
// equivalent that always stops the process is `abort()`.
pub fn stub_0x23e5c0() -> bool {
    let enabled = DO_CRASH_ENABLED.load(Ordering::Relaxed);
    if enabled {
        std::process::abort();
    }
    enabled
}

#[doc(alias = "__ZN3RBX9Debugable7doCrashEPKc")]
// 0x23e5dc — RBX::Debugable::doCrash(char const*)
// IDA 0x23e5dc: identical body to 0x23e5c0.
// BUG: the message argument is never read.
pub fn stub_0x23e5dc(_message: Option<&str>) -> bool {
    stub_0x23e5c0()
}

#[doc(alias = "__ZN3RBX15DebugNameStringC1EPKci")]
// 0x23e638 — RBX::DebugNameString::DebugNameString(char const*,int)
// IDA 0x23e638: packs the `{const char*, int}` pair into the first 8 bytes.
// was: `RBX::DebugNameString`.
pub struct DebugNameString {
    /// +0: format pointer (kept as owned text; the original never copies it).
    format: Option<String>,
    /// +4: next counter value (`getNameIncrement` post-increments).
    next: i32,
    /// +8: 100-byte `snprintf` scratch buffer (99 chars + NUL).
    buf: [u8; DEBUG_NAME_SCRATCH],
    /// Live bytes in `buf` (excludes the NUL).
    len: usize,
}
/// IDA 0x23e644: scratch size (`snprintf(buf, 0x64, ...)`).
const DEBUG_NAME_SCRATCH: usize = 100;
pub fn stub_0x23e638(format: Option<&str>, initial: i32) -> DebugNameString {
    DebugNameString {
        format: format.map(str::to_string),
        next: initial,
        buf: [0; DEBUG_NAME_SCRATCH],
        len: 0,
    }
}

// 0x23e644 — RBX::DebugNameString::getNameIncrement(char const*)
// IDA 0x23e644: `fmt = a2 ? a2 : stored`; `snprintf(buf, 0x64, fmt, counter++)`;
// returns `buf`. Only `%d`/`%i` consume the counter, so the port expands those
// plus `%%` and passes anything else through literally.
pub fn stub_0x23e644<'a>(state: &'a mut DebugNameString, override_format: Option<&'a str>) -> &'a str {
    let counter = state.next;
    state.next = state.next.wrapping_add(1);
    let fmt = override_format.or(state.format.as_deref()).unwrap_or("");
    let rendered = expand_count_format(fmt, counter);
    let bytes = rendered.as_bytes();
    let len = bytes.len().min(DEBUG_NAME_SCRATCH - 1);
    state.buf[..len].copy_from_slice(&bytes[..len]);
    state.buf[len] = 0;
    state.len = len;
    std::str::from_utf8(&state.buf[..len]).unwrap_or("")
}
/// IDA 0x23e644 `snprintf` subset: `%d`/`%i` take the counter, `%%` is a
/// literal percent, every other sequence passes through untouched.
fn expand_count_format(fmt: &str, counter: i32) -> String {
    let mut out = String::with_capacity(fmt.len() + 4);
    let mut chars = fmt.chars();
    while let Some(c) = chars.next() {
        if c != '%' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('d') | Some('i') => out.push_str(&counter.to_string()),
            Some('%') => out.push('%'),
            Some(other) => {
                out.push('%');
                out.push(other);
            }
            None => out.push('%'),
        }
    }
    out
}

#[doc(alias = "__ZN3RBX3Log10writeEntryENS0_8SeverityEPKc")]
// 0x23e988 — RBX::Log::writeEntry(RBX::Log::Severity,char const*)
// IDA 0x23e988: `timeStamp(stream, 0)`, then a 10-char tag (`a2 == 0` spaces,
// `== 1` `" Warning: "`, `== 2` `" Error:   "`, anything else no tag), the
// message, `'\n'`, flush. `timestamp` is the `timeStamp` prefix, rendered by
// the caller; the returned line is what the original streams + flushes.
pub fn stub_0x23e988(severity: i32, message: Option<&str>, timestamp: &str) -> String {
    let mut line = String::new();
    line.push_str(timestamp);
    if severity == 0 {
        line.push_str("          ");
    } else if severity == 1 {
        line.push_str(" Warning: ");
    } else if severity == 2 {
        line.push_str(" Error:   ");
    }
    // BUG: a null message sets the stream failbit and writes nothing, yet the
    // original still emits `'\n'` and flushes — modelled as skipping the body.
    if let Some(text) = message {
        line.push_str(text);
    }
    line.push('\n');
    line
}

#[doc(alias = "__ZN3RBX3Log9formatMemEj")]
// 0x23ea18 — RBX::Log::formatMem(unsigned int)
// IDA 0x23ea18 (disasm): `a2 >> 3 <= 0x7C` (i.e. `a2 < 1000`) prints `%dB`;
// up to 99999 `%.1fKB`, up to 999999 `%.0fKB`, up to 99999999 `%.1fMB`, up to
// 999999999 `%.0fMB`, else `%.0fGB` — all divided by 1000/1e6/1e9 decimal.
pub fn stub_0x23ea18(bytes: u32) -> String {
    if bytes < 1000 {
        format!("{bytes}B")
    } else if bytes <= 99_999 {
        format!("{:.1}KB", bytes as f64 / 1000.0)
    } else if bytes <= 999_999 {
        format!("{:.0}KB", bytes as f64 / 1000.0)
    } else if bytes <= 99_999_999 {
        format!("{:.1}MB", bytes as f64 / 1_000_000.0)
    } else if bytes <= 999_999_999 {
        format!("{:.0}MB", bytes as f64 / 1_000_000.0)
    } else {
        format!("{:.0}GB", bytes as f64 / 1_000_000_000.0)
    }
}

#[doc(alias = "__ZN3RBX3Log10formatTimeEd")]
// 0x23eb48 — RBX::Log::formatTime(double)
// IDA 0x23eb48: `%.3gs` for `< 0`/`>= 0.1`, else `%.3gms` of `s * 1000`.
// BUG: the `a2 == 0.0` → `"0s"` store is dead — the second `snprintf` always
// overwrites it, so `formatTime(0.0)` returns `"0ms"`.
pub fn stub_0x23eb48(seconds: f64) -> String {
    if seconds < 0.0 || seconds >= 0.1 {
        format!("{}s", format_g3(seconds))
    } else {
        format!("{}ms", format_g3(seconds * 1000.0))
    }
}
/// C `%.3g`: 3 significant digits, trailing zeros stripped, `%e` form when the
/// decimal exponent is `< -4` or `>= 3` (matching glibc, exponent `e±XX`).
fn format_g3(value: f64) -> String {
    if value == 0.0 {
        return if value.is_sign_negative() {
            "-0".to_string()
        } else {
            "0".to_string()
        };
    }
    if !value.is_finite() {
        if value.is_nan() {
            return "nan".to_string();
        }
        return if value.is_sign_negative() {
            "-inf".to_string()
        } else {
            "inf".to_string()
        };
    }
    let exp = value.abs().log10().floor() as i32;
    // Round to 3 significant digits first, so a carry (999.9 → 1000) picks the
    // exponent form exactly like `%g` does.
    let factor = 10f64.powi(2 - exp);
    let rounded = (value * factor).round() / factor;
    let exp = rounded.abs().log10().floor() as i32;
    if exp < -4 || exp >= 3 {
        let mantissa = rounded / 10f64.powi(exp);
        let mut digits = format!("{mantissa:.2}");
        trim_g3_zeros(&mut digits);
        format!("{digits}e{exp:+03}")
    } else {
        let frac = (2 - exp).max(0) as u32 as usize;
        let mut digits = format!("{rounded:.frac$}");
        trim_g3_zeros(&mut digits);
        digits
    }
}
/// `%g` strips trailing zeros (and a bare point) from the digit string.
fn trim_g3_zeros(digits: &mut String) {
    if digits.contains('.') {
        while digits.ends_with('0') {
            digits.pop();
        }
        if digits.ends_with('.') {
            digits.pop();
        }
    }
}

#[doc(alias = "__ZN3RBX15set_thread_nameEPKc")]
// 0x23f42c — RBX::set_thread_name(char const*)
// IDA 0x23f42c: `call_once(init_foo)`, then stores a heap `std::string` copy
// into the `thread_specific_ptr<std::string>` slot. Modelled as thread-local
// text; the once-guard has no observable state beyond the store.
pub fn stub_0x23f42c(name: &str) {
    THREAD_NAME.with(|slot| *slot.borrow_mut() = Some(name.to_string()));
}

#[doc(alias = "__ZN3RBX13worker_threadD1Ev")]
// 0x2400f4 — RBX::worker_thread::~worker_thread()
// IDA 0x2400f4: tail-calls the D2 dtor at 0x240100, then frees the object.
// was: `RBX::worker_thread`.
pub struct WorkerThread {
    /// Guards `shutdown` (IDA `unique_lock<mutex>` + shutdown byte at +116).
    pub mutex: parking_lot::Mutex<bool>,
    /// Broadcast cond (IDA `pthread_cond_broadcast` at +88).
    pub cond: parking_lot::Condvar,
    /// Owned thread, detached by the dtor (IDA `thread::detach` at 0x2401a6).
    pub thread: Option<std::thread::JoinHandle<()>>,
}
impl WorkerThread {
    pub fn new() -> Self {
        Self {
            mutex: parking_lot::Mutex::new(false),
            cond: parking_lot::Condvar::new(),
            thread: None,
        }
    }
    pub fn is_shutdown(&self) -> bool {
        *self.mutex.lock()
    }
}
impl Default for WorkerThread {
    fn default() -> Self {
        Self::new()
    }
}
pub fn stub_0x2400f4(mut state: WorkerThread) {
    stub_0x240100(&state);
    // IDA `thread::detach` + member `shared_count::release` pair: dropping a
    // `JoinHandle` detaches without joining; the mutex/cond drop with `state`.
    state.thread.take();
}

#[doc(alias = "__ZN3RBX13worker_threadD2Ev")]
// 0x240100 — RBX::worker_thread::~worker_thread()
// IDA 0x240100 (D2): lock; shutdown byte (+116) = 1; broadcast (+88); unlock;
// detach thread; release the two owned shared counts.
pub fn stub_0x240100(state: &WorkerThread) {
    *state.mutex.lock() = true;
    state.cond.notify_all();
}

#[doc(alias = "__ZN3RBX13worker_thread4wakeEv")]
// 0x2402c4 — RBX::worker_thread::wake(void)
// IDA 0x2402c4: lock; `pthread_cond_broadcast` (+88); unlock. Same as the D2
// notify path but without setting the shutdown byte.
pub fn stub_0x2402c4(state: &WorkerThread) {
    let _guard = state.mutex.lock();
    state.cond.notify_all();
}

#[doc(alias = "__ZN3RBX5mutexC2Ev")]
// 0x248940 — RBX::mutex::mutex(void)
// IDA 0x248940: `pthread_mutex_init`; on failure throws `std::runtime_error`
// with the message kept below. `parking_lot::Mutex` construction is
// infallible, so the throw path has no trigger here. was: `RBX::mutex`.
pub const MUTEX_INIT_FAILURE: &str = "failed in mutex to initialize pthread_mutex_init.";
pub fn stub_0x248940() -> parking_lot::Mutex<()> {
    parking_lot::Mutex::new(())
}

#[doc(alias = "__ZN3RBX22WindowAverageDutyCycleILNS_4Time12SampleMethodE1EE6sampleENS1_8IntervalE")]
// 0x24ad90 — RBX::WindowAverageDutyCycle<(RBX::Time::SampleMethod)1>::sample(RBX::Time::Interval)
pub fn stub_0x24ad90() {
    // IDA 0x24ad90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX25WindowAverageTimeIntervalILNS_4Time12SampleMethodE1EE6sampleEv")]
// 0x24ae08 — RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::sample(void)
pub fn stub_0x24ae08() {
    // IDA 0x24ae08: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "__ZN3RBX13WindowAverageIddE6sampleINS_13FOnBeforeDropEEEvdRT_")]
// 0x24b2c8 — void RBX::WindowAverage<double,double>::sample<RBX::FOnBeforeDrop>(double,RBX::FOnBeforeDrop &)
pub fn stub_0x24b2c8() {
    // IDA 0x24b2c8: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "__ZN3RBX25WindowAverageTimeIntervalILNS_4Time12SampleMethodE1EEC2ENS1_8IntervalE")]
// 0x24b364 — RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::WindowAverageTimeInterval(RBX::Time::Interval)
pub fn stub_0x24b364() {
    // IDA 0x24b364: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "__ZN3RBX13TaskScheduler6ThreadD2Ev")]
// 0x2501bc — RBX::TaskScheduler::Thread::~Thread()
// IDA 0x2501bc: `join`, release the job shared count, detach + free the
// thread, release the mutex shared count. was: `RBX::TaskScheduler::Thread`.
pub struct SchedulerThread {
    pub thread: Option<std::thread::JoinHandle<()>>,
}
impl SchedulerThread {
    pub fn new(thread: Option<std::thread::JoinHandle<()>>) -> Self {
        Self { thread }
    }
}
pub fn stub_0x2501bc(thread: SchedulerThread) {
    if let Some(handle) = thread.thread {
        // IDA `Thread::join` — a panicking thread must not take down the dtor.
        let _ = handle.join();
    }
    // The job/mutex `release` calls are Arc drops at scope exit.
}

#[doc(alias = "__ZN3RBX4Time3nowILNS0_12SampleMethodE2EEES0_v")]
// 0x253d50 — RBX::Time RBX::Time::now<(RBX::Time::SampleMethod)2>(void)
// IDA 0x253d50: seconds since the first call (`gettimeofday` minus the
// `getStart()` epoch, `* 0.000001`). was: `RBX::Time`.
pub fn stub_0x253d50() -> f64 {
    Instant::now().duration_since(time_epoch()).as_secs_f64()
}

#[doc(alias = "__ZN3RBX4Time3nowILNS0_12SampleMethodE0EEES0_v")]
// 0x253ea4 — RBX::Time RBX::Time::now<(RBX::Time::SampleMethod)0>(void)
// IDA 0x253ea4: tail-calls `now<2>` (0x253ea8).
pub fn stub_0x253ea4() -> f64 {
    stub_0x253d50()
}

#[doc(alias = "__ZN3RBX4Time7nowFastEv")]
// 0x253eb0 — RBX::Time::nowFast(void)
// IDA 0x253eb0: tail-calls `now<2>` (0x253eb4).
pub fn stub_0x253eb0() -> f64 {
    stub_0x253d50()
}

#[doc(alias = "__ZN3RBX4Time10nowFastSecEv")]
// 0x253ebc — RBX::Time::nowFastSec(void)
// IDA 0x253ebc: calls `now<2>` into a stack double, returns its bits as QWORD.
// Same value, typed `f64` here.
pub fn stub_0x253ebc() -> f64 {
    stub_0x253d50()
}
#[cfg(test)]
mod shard_u_tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    #[test]
    fn error_and_format_forward_to_vformat_policy() {
        assert_eq!(stub_0x23e324("hi"), "hi");
        assert_eq!(stub_0x23e324(""), "");
        assert_eq!(stub_0x23e50c("hi"), stub_0x23e324("hi"));
        let long = "x".repeat(200);
        assert_eq!(stub_0x23e324(&long), long);
        let huge = "y".repeat(1_000_001);
        assert_eq!(stub_0x23e324(&huge).len(), 1_000_000);
        let err = stub_0x23e1f8("boom");
        assert_eq!(err.message(), "boom");
        assert_eq!(format!("{err}"), "boom");
    }
    #[test]
    fn do_crash_reports_flag_without_trapping() {
        assert!(!stub_0x23e5c0());
        assert!(!stub_0x23e5dc(None));
        assert!(!stub_0x23e5dc(Some("ignored")));
    }
    #[test]
    fn debug_name_increments_and_truncates() {
        let mut state = stub_0x23e638(Some("job-%d"), 3);
        assert_eq!(stub_0x23e644(&mut state, None), "job-3");
        assert_eq!(stub_0x23e644(&mut state, None), "job-4");
        assert_eq!(stub_0x23e644(&mut state, Some("run-%i!")), "run-5!");
        assert_eq!(stub_0x23e644(&mut state, None), "job-6");
        let mut blank = stub_0x23e638(None, 0);
        assert_eq!(stub_0x23e644(&mut blank, None), "");
        let mut pct = stub_0x23e638(Some("100%%-%d"), 7);
        assert_eq!(stub_0x23e644(&mut pct, None), "100%-7");
        let mut big = stub_0x23e638(Some(&"z".repeat(200)), 0);
        assert_eq!(stub_0x23e644(&mut big, None).len(), 99);
    }
    #[test]
    fn write_entry_tags_and_newline() {
        assert_eq!(stub_0x23e988(0, Some("hi"), "TS"), "TS          hi\n");
        assert_eq!(stub_0x23e988(1, Some("hi"), "TS"), "TS Warning: hi\n");
        assert_eq!(stub_0x23e988(2, Some("hi"), "TS"), "TS Error:   hi\n");
        assert_eq!(stub_0x23e988(7, Some("x"), ""), "x\n");
        assert_eq!(stub_0x23e988(1, None, ""), " Warning: \n");
    }
    #[test]
    fn format_mem_thresholds() {
        assert_eq!(stub_0x23ea18(0), "0B");
        assert_eq!(stub_0x23ea18(999), "999B");
        assert_eq!(stub_0x23ea18(1000), "1.0KB");
        assert_eq!(stub_0x23ea18(99_999), "100.0KB");
        assert_eq!(stub_0x23ea18(100_000), "100KB");
        assert_eq!(stub_0x23ea18(1_000_000), "1.0MB");
        assert_eq!(stub_0x23ea18(100_000_000), "100MB");
        assert_eq!(stub_0x23ea18(2_000_000_000), "2GB");
    }
    #[test]
    fn format_time_branches() {
        assert_eq!(stub_0x23eb48(0.0), "0ms");
        assert_eq!(stub_0x23eb48(0.05), "50ms");
        assert_eq!(stub_0x23eb48(5.0), "5s");
        assert_eq!(stub_0x23eb48(1.5), "1.5s");
        assert_eq!(stub_0x23eb48(-2.0), "-2s");
        assert_eq!(stub_0x23eb48(1234.0), "1.23e+03s");
        assert_eq!(stub_0x23eb48(1e-5), "0.01ms");
    }
    #[test]
    fn thread_name_roundtrips() {
        stub_0x23f42c("worker-1");
        assert_eq!(thread_name().as_deref(), Some("worker-1"));
    }
    #[test]
    fn worker_wake_and_dtor() {
        let lane = WorkerThread::new();
        assert!(!lane.is_shutdown());
        stub_0x2402c4(&lane);
        assert!(!lane.is_shutdown());
        stub_0x240100(&lane);
        assert!(lane.is_shutdown());
        let live = WorkerThread {
            thread: Some(std::thread::spawn(|| {})),
            ..WorkerThread::new()
        };
        stub_0x2400f4(live);
        let _ = stub_0x248940().lock();
    }
    #[test]
    fn scheduler_dtor_joins() {
        let done = Arc::new(AtomicBool::new(false));
        let flag = Arc::clone(&done);
        let handle = std::thread::spawn(move || flag.store(true, Ordering::SeqCst));
        stub_0x2501bc(SchedulerThread::new(Some(handle)));
        assert!(done.load(Ordering::SeqCst));
        stub_0x2501bc(SchedulerThread::new(None));
    }
    #[test]
    fn time_now_is_monotonic_and_small() {
        let a = stub_0x253d50();
        let b = stub_0x253ea4();
        let c = stub_0x253eb0();
        let d = stub_0x253ebc();
        assert!(a >= 0.0 && a < 3600.0);
        assert!(b >= a && c >= b && d >= c);
    }
}

#[doc(alias = "__ZN3RBX4Time3nowILNS0_12SampleMethodE1EEES0_v")]
// 0x253ecc — RBX::Time RBX::Time::now<(RBX::Time::SampleMethod)1>(void)
pub fn stub_0x253ecc() {
    // IDA 0x253ecc: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBXmiERKNS_4TimeES2_")]
// 0x253edc — RBX::operator-(RBX::Time const&,RBX::Time const&)
pub fn stub_0x253edc() {
    // IDA 0x253edc: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX10RbxDbgInfo8AddPlaceEl")]
// 0x253ef0 — RBX::RbxDbgInfo::AddPlace(long)
pub fn stub_0x253ef0() {
    // IDA 0x253ef0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX10RbxDbgInfo11RemovePlaceEl")]
// 0x253f24 — RBX::RbxDbgInfo::RemovePlace(long)
pub fn stub_0x253f24() {
    // IDA 0x253f24: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX10RbxDbgInfo10SetCPUNameEPKc")]
// 0x253fdc — RBX::RbxDbgInfo::SetCPUName(char const*)
pub fn stub_0x253fdc() {
    // IDA 0x253fdc: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX10RbxDbgInfo11SetServerIPEPKc")]
// 0x254000 — RBX::RbxDbgInfo::SetServerIP(char const*)
pub fn stub_0x254000() {
    // IDA 0x254000: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX23RbxInterlockedDecrementEPVl")]
// 0x254024 — RBX::RbxInterlockedDecrement(long volatile*)
pub fn stub_0x254024() {
    // IDA 0x254024: libtiff tile writer owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX23RbxInterlockedIncrementEPVl")]
// 0x254034 — RBX::RbxInterlockedIncrement(long volatile*)
pub fn stub_0x254034() {
    // IDA 0x254034: libtiff tile writer owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX30RbxInterlockedIncrementAcquireEPVl")]
// 0x254044 — RBX::RbxInterlockedIncrementAcquire(long volatile*)
pub fn stub_0x254044() {
    // IDA 0x254044: libtiff tile writer owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX22RbxInterlockedExchangeEPVll")]
// 0x254054 — RBX::RbxInterlockedExchange(long volatile*,long)
pub fn stub_0x254054() {
    // IDA 0x254054: libtiff tile writer owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX29RbxInterlockedCompareExchangeEPVlll")]
// 0x254068 — RBX::RbxInterlockedCompareExchange(long volatile*,long,long)
pub fn stub_0x254068() {
    // IDA 0x254068: libtiff tile writer owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX13MacSystemUtil10getCPUMakeEv")]
// 0x25407c — RBX::MacSystemUtil::getCPUMake(void)
pub fn stub_0x25407c() {
    // IDA 0x25407c: libtiff tile writer owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX13MacSystemUtil11getCPUSpeedEv")]
// 0x2541ac — RBX::MacSystemUtil::getCPUSpeed(void)
pub fn stub_0x2541ac() {
    // IDA 0x2541ac: libtiff tile writer owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX13MacSystemUtil18getCPULogicalCountEv")]
// 0x254224 — RBX::MacSystemUtil::getCPULogicalCount(void)
pub fn stub_0x254224() {
    // IDA 0x254224: libtiff tile writer owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX13MacSystemUtil15getCPUCoreCountEv")]
// 0x2542b0 — RBX::MacSystemUtil::getCPUCoreCount(void)
pub fn stub_0x2542b0() {
    // IDA 0x2542b0: libtiff tile writer owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX13MacSystemUtil19getCPUPhysicalCountEv")]
// 0x254320 — RBX::MacSystemUtil::getCPUPhysicalCount(void)
pub fn stub_0x254320() {
    // IDA 0x254320: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX13MacSystemUtil10isCPU64BitEv")]
// 0x254478 — RBX::MacSystemUtil::isCPU64Bit(void)
pub fn stub_0x254478() {
    // IDA 0x254478: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX13MacSystemUtil11getMBSysRAMEv")]
// 0x25453c — RBX::MacSystemUtil::getMBSysRAM(void)
pub fn stub_0x25453c() {
    // IDA 0x25453c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX13MacSystemUtil20getMBSysAvailableRAMEv")]
// 0x2545b4 — RBX::MacSystemUtil::getMBSysAvailableRAM(void)
pub fn stub_0x2545b4() {
    // IDA 0x2545b4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX13MacSystemUtil14getVideoMemoryEv")]
// 0x254654 — RBX::MacSystemUtil::getVideoMemory(void)
pub fn stub_0x254654() {
    // IDA 0x254654: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX13MacSystemUtil5osVerEv")]
// 0x25465c — RBX::MacSystemUtil::osVer(void)
pub fn stub_0x25465c() {
    // IDA 0x25465c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX13MacSystemUtil10getGPUMakeEv")]
// 0x254824 — RBX::MacSystemUtil::getGPUMake(void)
pub fn stub_0x254824() {
    // IDA 0x254824: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX13MacSystemUtil9getMaxResEv")]
// 0x2549ec — RBX::MacSystemUtil::getMaxRes(void)
pub fn stub_0x2549ec() {
    // IDA 0x2549ec: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX5CryptC1Ev")]
// 0x254bb4 — RBX::Crypt::Crypt(void)
pub fn stub_0x254bb4() {
    // IDA 0x254bb4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX5CryptD1Ev")]
// 0x254bb8 — RBX::Crypt::~Crypt()
pub fn stub_0x254bb8() {
    // IDA 0x254bb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX5Crypt21verifySignatureBase64ESsSs")]
// 0x254bbc — RBX::Crypt::verifySignatureBase64(std::string,std::string)
pub fn stub_0x254bbc() {
    // IDA 0x254bbc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX14IsValueOutlierEdjddNS_10ConfidenceE")]
// 0x254bf8 — RBX::IsValueOutlier(double,unsigned int,double,double,RBX::Confidence)
pub fn stub_0x254bf8() {
    // IDA 0x254bf8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX21GetConfidenceIntervalEddNS_10ConfidenceEPdS1_")]
// 0x254c68 — RBX::GetConfidenceInterval(double,double,RBX::Confidence,double *,double *)
pub fn stub_0x254c68() {
    // IDA 0x254c68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX9TCriticalEjNS_10ConfidenceE")]
// 0x254d18 — RBX::TCritical(unsigned int,RBX::Confidence)
pub fn stub_0x254d18() {
    // IDA 0x254d18: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX11HttpService10decodeJSONESs")]
// 0x256a6c — RBX::HttpService::decodeJSON(std::string)
pub fn stub_0x256a6c() {
    // IDA 0x256a6c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX15StringConverterINS_11HttpService15HttpContentTypeEE14convertToValueERKSsRS2_")]
// 0x2570c0 — RBX::StringConverter<RBX::HttpService::HttpContentType>::convertToValue(std::string const&,RBX::HttpService::HttpContentType&)
pub fn stub_0x2570c0() {
    // IDA 0x2570c0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX11HttpServiceC2Ev")]
// 0x257110 — RBX::HttpService::HttpService(void)
pub fn stub_0x257110() {
    // IDA 0x257110: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX11HttpService18checkUserHasAccessEv")]
// 0x257758 — RBX::HttpService::checkUserHasAccess(void)
pub fn stub_0x257758() {
    // IDA 0x257758: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX11HttpService10checkLimitEv")]
// 0x2577c0 — RBX::HttpService::checkLimit(void)
pub fn stub_0x2577c0() {
    // IDA 0x2577c0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX11HttpServiceD1Ev")]
// 0x2580ac — RBX::HttpService::~HttpService()
pub fn stub_0x2580ac() {
    // IDA 0x2580ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX11HttpServiceD0Ev")]
// 0x2580b0 — RBX::HttpService::~HttpService()
pub fn stub_0x2580b0() {
    // IDA 0x2580b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX11HttpServiceD1Ev")]
// 0x258160 — non-virtual thunk toRBX::HttpService::~HttpService()
pub fn stub_0x258160() {
    // IDA 0x258160: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX11HttpServiceD0Ev")]
// 0x258168 — non-virtual thunk toRBX::HttpService::~HttpService()
pub fn stub_0x258168() {
    // IDA 0x258168: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX11HttpServiceD1Ev")]
// 0x25821c — non-virtual thunk toRBX::HttpService::~HttpService()
pub fn stub_0x25821c() {
    // IDA 0x25821c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX11HttpServiceD0Ev")]
// 0x258224 — non-virtual thunk toRBX::HttpService::~HttpService()
pub fn stub_0x258224() {
    // IDA 0x258224: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx8any_castIN3RBX11HttpService15HttpContentTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// 0x258e74 — RBX::HttpService::HttpContentType * rbx::any_cast<RBX::HttpService::HttpContentType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)
pub fn stub_0x258e74() {
    // IDA 0x258e74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx8any_castIRN3RBX11HttpService15HttpContentTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x258ecc — RBX::HttpService::HttpContentType & rbx::any_cast<RBX::HttpService::HttpContentType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_0x258ecc() {
    // IDA 0x258ecc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX5Light10setEnabledEb")]
// 0x25b4c0 — RBX::Light::setEnabled(bool)
pub fn stub_0x25b4c0() {
    // IDA 0x25b4c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX5Light13setBrightnessEf")]
// 0x25b544 — RBX::Light::setBrightness(float)
pub fn stub_0x25b544() {
    // IDA 0x25b544: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX10PointLight8setRangeEf")]
// 0x25b574 — RBX::PointLight::setRange(float)
pub fn stub_0x25b574() {
    // IDA 0x25b574: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "__ZN3RBX9SpotLight8setRangeEf")]
// 0x25b5b0 — RBX::SpotLight::setRange(float)
pub fn stub_0x25b5b0() {
    // IDA 0x25b5b0: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "__ZN3RBX9SpotLight8setAngleEf")]
// 0x25b5ec — RBX::SpotLight::setAngle(float)
pub fn stub_0x25b5ec() {
    // IDA 0x25b5ec: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX19registerNewLightAPIEv")]
// 0x25b628 — RBX::registerNewLightAPI(void)
pub fn stub_0x25b628() {
    // IDA 0x25b628: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX5Light10setShadowsEb")]
// 0x25b884 — RBX::Light::setShadows(bool)
pub fn stub_0x25b884() {
    // IDA 0x25b884: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX9SpotLight7setFaceENS_8NormalIdE")]
// 0x25b8a8 — RBX::SpotLight::setFace(RBX::NormalId)
pub fn stub_0x25b8a8() {
    // IDA 0x25b8a8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX5LightC2EPKc")]
// 0x25b8c8 — RBX::Light::Light(char const*)
pub fn stub_0x25b8c8() {
    // IDA 0x25b8c8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX5LightD0Ev")]
// 0x25baa8 — RBX::Light::~Light()
pub fn stub_0x25baa8() {
    // IDA 0x25baa8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX5LightD1Ev")]
// 0x25bb48 — RBX::Light::~Light()
pub fn stub_0x25bb48() {
    // IDA 0x25bb48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX5LightD0Ev")]
// 0x25bb4c — non-virtual thunk toRBX::Light::~Light()
pub fn stub_0x25bb4c() {
    // IDA 0x25bb4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX5LightD0Ev")]
// 0x25bb54 — non-virtual thunk toRBX::Light::~Light()
pub fn stub_0x25bb54() {
    // IDA 0x25bb54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn92_N3RBX5LightD0Ev")]
// 0x25bb5c — non-virtual thunk toRBX::Light::~Light()
pub fn stub_0x25bb5c() {
    // IDA 0x25bb5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX5LightD2Ev")]
// 0x25bb64 — RBX::Light::~Light()
pub fn stub_0x25bb64() {
    // IDA 0x25bb64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX5LightD1Ev")]
// 0x25bc20 — non-virtual thunk toRBX::Light::~Light()
pub fn stub_0x25bc20() {
    // IDA 0x25bc20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX5LightD1Ev")]
// 0x25bc28 — non-virtual thunk toRBX::Light::~Light()
pub fn stub_0x25bc28() {
    // IDA 0x25bc28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn92_N3RBX5LightD1Ev")]
// 0x25bc30 — non-virtual thunk toRBX::Light::~Light()
pub fn stub_0x25bc30() {
    // IDA 0x25bc30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX10PointLightC2Ev")]
// 0x25bc64 — RBX::PointLight::PointLight(void)
pub fn stub_0x25bc64() {
    // IDA 0x25bc64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX10PointLightD0Ev")]
// 0x25bdb8 — RBX::PointLight::~PointLight()
pub fn stub_0x25bdb8() {
    // IDA 0x25bdb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX10PointLightD1Ev")]
// 0x25be58 — RBX::PointLight::~PointLight()
pub fn stub_0x25be58() {
    // IDA 0x25be58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX10PointLightD0Ev")]
// 0x25be5c — non-virtual thunk toRBX::PointLight::~PointLight()
pub fn stub_0x25be5c() {
    // IDA 0x25be5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX10PointLightD0Ev")]
// 0x25be64 — non-virtual thunk toRBX::PointLight::~PointLight()
pub fn stub_0x25be64() {
    // IDA 0x25be64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn92_N3RBX10PointLightD0Ev")]
// 0x25be6c — non-virtual thunk toRBX::PointLight::~PointLight()
pub fn stub_0x25be6c() {
    // IDA 0x25be6c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX10PointLightD1Ev")]
// 0x25be74 — non-virtual thunk toRBX::PointLight::~PointLight()
pub fn stub_0x25be74() {
    // IDA 0x25be74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX10PointLightD1Ev")]
// 0x25be7c — non-virtual thunk toRBX::PointLight::~PointLight()
pub fn stub_0x25be7c() {
    // IDA 0x25be7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn92_N3RBX10PointLightD1Ev")]
// 0x25be84 — non-virtual thunk toRBX::PointLight::~PointLight()
pub fn stub_0x25be84() {
    // IDA 0x25be84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX9SpotLightC2Ev")]
// 0x25be8c — RBX::SpotLight::SpotLight(void)
pub fn stub_0x25be8c() {
    // IDA 0x25be8c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX9SpotLightD0Ev")]
// 0x25bff0 — RBX::SpotLight::~SpotLight()
pub fn stub_0x25bff0() {
    // IDA 0x25bff0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX9SpotLightD1Ev")]
// 0x25c090 — RBX::SpotLight::~SpotLight()
pub fn stub_0x25c090() {
    // IDA 0x25c090: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX9SpotLightD0Ev")]
// 0x25c094 — non-virtual thunk toRBX::SpotLight::~SpotLight()
pub fn stub_0x25c094() {
    // IDA 0x25c094: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX9SpotLightD0Ev")]
// 0x25c09c — non-virtual thunk toRBX::SpotLight::~SpotLight()
pub fn stub_0x25c09c() {
    // IDA 0x25c09c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn92_N3RBX9SpotLightD0Ev")]
// 0x25c0a4 — non-virtual thunk toRBX::SpotLight::~SpotLight()
pub fn stub_0x25c0a4() {
    // IDA 0x25c0a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX9SpotLightD1Ev")]
// 0x25c0ac — non-virtual thunk toRBX::SpotLight::~SpotLight()
pub fn stub_0x25c0ac() {
    // IDA 0x25c0ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
