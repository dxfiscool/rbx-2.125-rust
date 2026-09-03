//! RBX::worker_thread — dedicated wakeable worker.
//! Grounded in IDA: 0x23fa10 (C1), 0x23fa1c (C2), 0x23ffb0 (threadProc),
//! 0x2400f4/0x240100 (D1/D2), 0x2402c4 (wake).
//! was: boost::thread + boost::mutex + boost::condition_variable_any
//!      + boost::function0<work_result> → std::thread + Mutex/Condvar + Box<dyn Fn>.

use parking_lot::{Condvar, Mutex};
use std::sync::Arc;

/// was: `RBX::worker_thread::work_result`.
// IDA 0x23ffb0/0x240024: result `!= 1` parks on the condvar, `== 1` loops immediately.
pub type WorkResult = i32;
/// Re-run without parking (the `== 1` path at IDA 0x240024).
pub const WORK_AGAIN: WorkResult = 1;

struct Shared {
    state: Mutex<State>,
    wake: Condvar,
}

struct State {
    exit: bool, // IDA worker data+116 (0x74)
}

impl Shared {
    fn exited(&self) -> bool {
        self.state.lock().exit
    }
}

// IDA 0x23ffb0 RBX::worker_thread::threadProc(data, work).
// Plain wait, no predicate — a spurious wakeup re-runs work, exactly like the original.
fn thread_proc(shared: Arc<Shared>, work: Box<dyn Fn() -> WorkResult + Send>) {
    while !shared.exited() {
        if work() != WORK_AGAIN {
            let mut guard = shared.state.lock();
            shared.wake.wait(&mut guard);
        }
    }
}

/// was: `RBX::worker_thread` (IDA data = 0x78 bytes: mutex+0, condvar-any+44, exit+116).
#[doc(alias = "RBX::worker_thread")]
pub struct WorkerThread {
    shared: Arc<Shared>,
    // No JoinHandle stored: IDA 0x240100/0x2401a6 detaches the boost::thread.
    // The spawned thread holds its own Arc<Shared>, so it outlives Drop until
    // it observes exit.
    // BUG (preserved from original at 0x2401a6): Drop never joins — a blocked
    // work fn keeps its thread alive past the owner.
}

impl WorkerThread {
    /// IDA 0x23fa1c RBX::worker_thread::worker_thread(fn, name).
    /// was: `boost::function0<work_result> const&` + `char const*` thread name
    ///      (via RBX::thread_wrapper) → closure + std thread builder name.
    #[doc(alias = "RBX::worker_thread::worker_thread(boost::function0<RBX::worker_thread::work_result> const&,char const*)")]
    pub fn new<F>(work: F, name: &str) -> Self
    where
        F: Fn() -> WorkResult + Send + 'static,
    {
        let shared = Arc::new(Shared {
            state: Mutex::new(State { exit: false }),
            wake: Condvar::new(),
        });
        let thread_shared = Arc::clone(&shared);
        let work: Box<dyn Fn() -> WorkResult + Send> = Box::new(work);
        std::thread::Builder::new()
            .name(name.to_string())
            .spawn(move || thread_proc(thread_shared, work))
            .expect("RBX::worker_thread spawn");
        Self { shared }
    }

    /// IDA 0x2402c4 RBX::worker_thread::wake — lock + broadcast the condvar.
    #[doc(alias = "RBX::worker_thread::wake(void)")]
    pub fn wake(&self) {
        let _guard = self.shared.state.lock();
        self.shared.wake.notify_all();
    }
}

impl Drop for WorkerThread {
    /// IDA 0x240100: set exit (data+116), broadcast, detach (no join).
    fn drop(&mut self) {
        self.shared.state.lock().exit = true;
        self.shared.wake.notify_all();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::{Duration, Instant};

    fn poll_until(d: Duration, mut f: impl FnMut() -> bool) -> bool {
        let t = Instant::now();
        while t.elapsed() < d {
            if f() {
                return true;
            }
            std::thread::sleep(Duration::from_millis(1));
        }
        f()
    }

    #[test]
    fn parks_until_wake() {
        let calls = Arc::new(AtomicUsize::new(0));
        let c = Arc::clone(&calls);
        let w = WorkerThread::new(
            move || {
                c.fetch_add(1, Ordering::SeqCst);
                0
            },
            "park",
        );
        assert!(poll_until(Duration::from_secs(2), || calls.load(Ordering::SeqCst) == 1));
        std::thread::sleep(Duration::from_millis(50));
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        w.wake();
        assert!(poll_until(Duration::from_secs(2), || calls.load(Ordering::SeqCst) >= 2));
    }

    #[test]
    fn again_spins_without_wake() {
        let calls = Arc::new(AtomicUsize::new(0));
        let c = Arc::clone(&calls);
        let _w = WorkerThread::new(
            move || {
                c.fetch_add(1, Ordering::SeqCst);
                WORK_AGAIN
            },
            "spin",
        );
        assert!(poll_until(Duration::from_secs(2), || calls.load(Ordering::SeqCst) > 100));
    }

    #[test]
    fn drop_stops_thread() {
        let calls = Arc::new(AtomicUsize::new(0));
        let c = Arc::clone(&calls);
        let w = WorkerThread::new(
            move || {
                c.fetch_add(1, Ordering::SeqCst);
                0
            },
            "exit",
        );
        assert!(poll_until(Duration::from_secs(2), || calls.load(Ordering::SeqCst) >= 1));
        let weak = Arc::downgrade(&w.shared);
        drop(w);
        assert!(poll_until(Duration::from_secs(2), || weak.upgrade().is_none()));
    }
}
