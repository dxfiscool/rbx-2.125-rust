//! RBX::TaskScheduler — job scheduler + worker thread pool.
//! was: `RBX::TaskScheduler` + `boost::thread` pool + `boost::function0<void>`
//!      → `std::thread` + `parking_lot::Mutex/Condvar` + `Box<dyn Fn()>`.

use parking_lot::{Condvar, Mutex};
use std::sync::{
    Arc, OnceLock,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};
use std::time::{Duration, Instant};

use crate::SharedPtr;

/// was: `RBX::RunningAverage<int, double>` — periodic sampler target.
// IDA 0x246330/0x246338/0x246342: three averages sampled each tick
// (offsets +152, +112, +192 in the scheduler).
#[doc(alias = "RBX::RunningAverage<int,double>")]
#[derive(Debug, Default)]
pub struct RunningAverage {
    sum: f64,
    count: u64,
}

impl RunningAverage {
    pub fn new() -> Self {
        Self::default()
    }

    /// IDA 0x246308: `sample(field, current_value)` once per tick.
    #[doc(alias = "RBX::RunningAverage<int,double>::sample")]
    pub fn sample(&mut self, value: i64) {
        self.sum += value as f64;
        self.count += 1;
    }

    pub fn average(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            self.sum / self.count as f64
        }
    }
}

/// was: `RBX::TaskScheduler::Job` — one schedulable unit of work.
#[doc(alias = "RBX::TaskScheduler::Job")]
pub struct Job {
    name: &'static str,
    step: Mutex<Option<Box<dyn FnMut() + Send>>>,
}

impl Job {
    pub fn new(name: &'static str, step: impl FnMut() + Send + 'static) -> Self {
        Self {
            name,
            step: Mutex::new(Some(Box::new(step))),
        }
    }

    pub fn debug_name(&self) -> &'static str {
        self.name
    }

    /// IDA 0x3f090: body is `return 1`.
    // 0x3f090 — __ZNK3RBX13TaskScheduler3Job26getDesiredConcurrencyCountEv
    #[doc(alias = "RBX::TaskScheduler::Job::getDesiredConcurrencyCount")]
    pub fn get_desired_concurrency_count(&self) -> i32 {
        1
    }
}

/// was: `RBX::TaskScheduler` — singleton job scheduler.
// IDA 0x245c94: atomic word + `RBX::mutex` at +12, job lists, several
// `RunningAverage` fields, timestamp at +276, `CEvent` at +404, sampler
// thread handle at +524/528.
#[doc(alias = "RBX::TaskScheduler")]
pub struct TaskScheduler {
    jobs: Mutex<Vec<SharedPtr<Job>>>,
    running_job_count: AtomicUsize,
    duty_window_secs: Mutex<f64>,
    duty_busy_secs: Mutex<f64>,
    duty_t0: Mutex<Instant>,
    avg_a: Mutex<RunningAverage>,
    avg_b: Mutex<RunningAverage>,
    avg_c: Mutex<RunningAverage>,
    shutdown: AtomicBool,
    sampler_wake: Condvar,
    sampler_mu: Mutex<bool>,
}

impl TaskScheduler {
    /// IDA 0x245c94: zero lists/averages, stamp `now`, create the
    /// `CEvent(1)`, spawn the sampler thread and detach it.
    // 0x245c94 — __ZN3RBX13TaskSchedulerC2Ev
    #[doc(alias = "RBX::TaskScheduler::TaskScheduler")]
    pub fn new() -> Arc<Self> {
        let this = Arc::new(Self {
            jobs: Mutex::new(Vec::new()),
            running_job_count: AtomicUsize::new(0),
            duty_window_secs: Mutex::new(0.0),
            duty_busy_secs: Mutex::new(0.0),
            duty_t0: Mutex::new(Instant::now()),
            avg_a: Mutex::new(RunningAverage::new()),
            avg_b: Mutex::new(RunningAverage::new()),
            avg_c: Mutex::new(RunningAverage::new()),
            shutdown: AtomicBool::new(false),
            sampler_wake: Condvar::new(),
            sampler_mu: Mutex::new(false),
        });
        // IDA 0x245f88: `RBX::thread_wrapper(..., "Roblox sampleRunningJobCount")`
        // + `boost::thread::thread<...>` then `detach` — a detached std thread
        // is exactly that: the handle is dropped, the thread keeps running.
        let sampler = Arc::clone(&this);
        let _detached: std::thread::JoinHandle<()> = std::thread::Builder::new()
            .name("Roblox sampleRunningJobCount".to_owned())
            .spawn(move || sampler.sampler_loop())
            .expect("TaskScheduler sampler thread");
        this
    }

    /// IDA 0x245b68: `__cxa_guard` once-construct, `__cxa_atexit(dtor)`,
    /// `setThreadCount(default)`, publish to the global slot.
    // 0x245b68 — __ZN3RBX13TaskScheduler11static_initEv
    #[doc(alias = "RBX::TaskScheduler::static_init")]
    fn static_init() -> Arc<TaskScheduler> {
        static CELL: OnceLock<Arc<TaskScheduler>> = OnceLock::new();
        Arc::clone(CELL.get_or_init(TaskScheduler::new))
    }

    /// IDA 0x245c70: `boost::call_once(static_init)` then load the global.
    // 0x245c70 — __ZN3RBX13TaskScheduler9singletonEv
    #[doc(alias = "RBX::TaskScheduler::singleton")]
    pub fn singleton() -> Arc<TaskScheduler> {
        Self::static_init()
    }

    /// IDA 0x245a08: elapsed = now − t0(+276); if elapsed > 2·window,
    /// window = elapsed; ratio = busy/window (1.0/0.0 when window == 0.0
    /// according as busy > 0.0); return ratio / thread_count, 0.0 when
    /// the running/max words(+98/+97) are equal.
    // 0x245a08 — __ZNK3RBX13TaskScheduler30getSchedulerDutyCyclePerThreadEv
    #[doc(alias = "RBX::TaskScheduler::getSchedulerDutyCyclePerThread")]
    pub fn duty_cycle_per_thread(&self) -> f64 {
        let running = self.running_job_count.load(Ordering::Relaxed);
        let total = self.jobs.lock().len();
        if running == total {
            // IDA 0x245a1a/0x245a5e: equal words → 0.0.
            return 0.0;
        }
        let elapsed = self.duty_t0.lock().elapsed().as_secs_f64();
        let mut window = *self.duty_window_secs.lock();
        let busy = *self.duty_busy_secs.lock();
        if elapsed > window + window {
            // IDA 0x245a48/0x245a4a.
            window = elapsed;
        }
        let ratio = if window == 0.0 {
            // IDA 0x245a56: dbl pick by `busy > 0.0` — 1.0 vs 0.0.
            if busy > 0.0 { 1.0 } else { 0.0 }
        } else {
            // IDA 0x245a58.
            busy / window
        };
        let threads = total.saturating_sub(running).max(1) as f64;
        // IDA 0x245a8a: ratio / ((w98 − w97) >> 3) — per-thread share.
        ratio / threads
    }

    /// IDA 0x246308: loop `CEvent::Wait(71)` until signaled, sampling the
    /// three running averages each tick; returns when the event fires.
    // 0x246308 — __ZN3RBX13TaskScheduler21sampleRunningJobCountEv
    #[doc(alias = "RBX::TaskScheduler::sampleRunningJobCount")]
    fn sampler_loop(&self) {
        // IDA 0x24631e literal 71 is the `CEvent::Wait` timeout; [INFERENCE]
        // units are milliseconds (a 71 s sample period would never converge).
        const SAMPLE_WAIT: Duration = Duration::from_millis(71);
        let mut guard = self.sampler_mu.lock();
        while !self.shutdown.load(Ordering::Relaxed) {
            self.sampler_wake.wait_for(&mut guard, SAMPLE_WAIT);
            if self.shutdown.load(Ordering::Relaxed) {
                break;
            }
            // IDA 0x246330/0x246338/0x246342.
            let n = self.running() as i64;
            self.avg_a.lock().sample(n);
            self.avg_b.lock().sample(n);
            self.avg_c.lock().sample(n);
        }
    }

    fn running(&self) -> usize {
        self.running_job_count.load(Ordering::Relaxed)
    }

    /// IDA 0x39c6c: copy the shared job + copy the completion fn, then
    /// `remove(job, blocking = 1, fn)`.
    // 0x39c6c — __ZN3RBX13TaskScheduler14removeBlockingEN5boost10shared_ptrINS0_3JobEEENS1_8functionIFvvEEE
    #[doc(alias = "RBX::TaskScheduler::removeBlocking")]
    pub fn remove_blocking(&self, job: SharedPtr<Job>, on_removed: impl FnOnce() + Send + 'static) {
        let job = SharedPtr::clone(&job);
        self.remove(job, true, on_removed);
    }

    /// was: `RBX::TaskScheduler::remove(job, blocking, on_removed)`.
    /// Blocking join is [INFERENCE]: the original blocks the caller on the
    /// job's removal event; here removal is synchronous under the lock, so
    /// `blocking` only orders the callback after the erase.
    #[doc(alias = "RBX::TaskScheduler::remove")]
    pub fn remove(
        &self,
        job: SharedPtr<Job>,
        _blocking: bool,
        on_removed: impl FnOnce() + Send + 'static,
    ) {
        {
            let mut jobs = self.jobs.lock();
            if let Some(pos) = jobs.iter().position(|j| Arc::ptr_eq(j, &job)) {
                jobs.remove(pos);
            }
        }
        on_removed();
    }

    /// was: `RBX::TaskScheduler::add(job)`.
    #[doc(alias = "RBX::TaskScheduler::add")]
    pub fn add(&self, job: SharedPtr<Job>) {
        self.jobs.lock().push(job);
    }

    /// was: `RBX::TaskScheduler::reschedule(job)` — re-arm at the back.
    #[doc(alias = "RBX::TaskScheduler::reschedule")]
    pub fn reschedule(&self, job: SharedPtr<Job>) {
        let mut jobs = self.jobs.lock();
        if let Some(pos) = jobs.iter().position(|j| Arc::ptr_eq(j, &job)) {
            jobs.remove(pos);
        }
        jobs.push(job);
    }

    pub fn job_count(&self) -> usize {
        self.jobs.lock().len()
    }

    pub fn shutdown(&self) {
        self.shutdown.store(true, Ordering::Relaxed);
        self.sampler_wake.notify_all();
    }
}

impl Drop for TaskScheduler {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::Relaxed);
        self.sampler_wake.notify_all();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn desired_concurrency_is_one() {
        // IDA 0x3f090: `return 1`.
        let job = Job::new("test", || {});
        assert_eq!(job.get_desired_concurrency_count(), 1);
    }

    #[test]
    fn add_remove_blocking_runs_callback() {
        let sched = TaskScheduler::new();
        let job: SharedPtr<Job> = Arc::new(Job::new("j", || {}));
        sched.add(Arc::clone(&job));
        assert_eq!(sched.job_count(), 1);
        let ran = Arc::new(AtomicBool::new(false));
        let flag = Arc::clone(&ran);
        sched.remove_blocking(job, move || flag.store(true, Ordering::SeqCst));
        assert_eq!(sched.job_count(), 0);
        assert!(ran.load(Ordering::SeqCst));
    }

    #[test]
    fn duty_cycle_zero_when_idle() {
        let sched = TaskScheduler::new();
        assert_eq!(sched.duty_cycle_per_thread(), 0.0);
        sched.shutdown();
    }

    #[test]
    fn singleton_is_stable() {
        assert!(Arc::ptr_eq(&TaskScheduler::singleton(), &TaskScheduler::singleton()));
    }
}
