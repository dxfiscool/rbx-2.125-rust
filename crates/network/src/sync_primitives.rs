//! `RakNet::SignaledEvent` + `RakNet::SimpleMutex` threading primitives.
//!
//! Decompiled from the ctor/dtors (IDA 0xa79900/0xa79914/0xa7a0b4/0xa7a0c4)
//! and the method set (IDA 0xa79924/0xa79954/0xa7997c/0xa7999c/0xa7a0d4/
//! 0xa7a0e0). The OS event/mutex handles stay engine-side: the event is a
//! manually-reset flag over `Mutex` + `Condvar` (a set signal is never
//! lost, at the cost of benign extra wakeups), and the simple mutex is an
//! atomic spinlock.

#![allow(dead_code)]

use std::sync::{Condvar, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

/// `RakNet::SignaledEvent` as a manually-reset flag.
#[derive(Debug, Default)]
pub struct SignaledEvent {
    /// Set by [`set`](Self::set_event), observed by [`wait`](Self::wait_on_event).
    signaled: Mutex<bool>,
    /// Notified on every [`set`](Self::set_event).
    changed: Condvar,
}

impl SignaledEvent {
    /// Ctor (IDA 0xa79900); the handle init runs inline.
    pub fn new() -> Self {
        Self::default()
    }

    /// `InitEvent` (IDA 0xa79924): handles are created inline; no-op.
    pub fn init_event(&self) {}

    /// `CloseEvent` (IDA 0xa79954): nothing owned; no-op.
    pub fn close_event(&self) {}

    /// `SetEvent` (IDA 0xa7997c): latch the flag and wake all waiters.
    pub fn set_event(&self) {
        *self.signaled.lock().expect("signaled") = true;
        self.changed.notify_all();
    }

    /// `WaitOnEvent` (IDA 0xa7999c): block up to `timeout_ms` for the
    /// flag. Negative timeouts wait without a deadline. The flag stays
    /// latched (manual reset).
    pub fn wait_on_event(&self, timeout_ms: i32) -> bool {
        let guard = self.signaled.lock().expect("signaled");
        if timeout_ms < 0 {
            self.changed
                .wait_while(guard, |signaled| !*signaled)
                .map(|guard| *guard)
                .unwrap_or(false)
        } else {
            self.changed
                .wait_timeout_while(
                    guard,
                    Duration::from_millis(timeout_ms as u64),
                    |signaled| !*signaled,
                )
                .map(|(guard, _)| *guard)
                .unwrap_or(false)
        }
    }

    /// Clear a latched signal (companion to the manual-reset choice).
    pub fn reset(&self) {
        *self.signaled.lock().expect("signaled") = false;
    }
}

/// `RakNet::SimpleMutex` as an atomic spinlock.
#[derive(Debug, Default)]
pub struct SimpleMutex {
    /// Locked while `true`.
    locked: AtomicBool,
}

impl SimpleMutex {
    /// Ctor (IDA 0xa7a0b4).
    pub fn new() -> Self {
        Self::default()
    }

    /// `Lock` (IDA 0xa7a0d4): spin until the flag is ours.
    pub fn lock(&self) {
        while self
            .locked
            .compare_exchange_weak(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            std::hint::spin_loop();
        }
    }

    /// `Unlock` (IDA 0xa7a0e0): release the flag.
    pub fn unlock(&self) {
        self.locked.store(false, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_latches_and_resets() {
        // IDA 0xa79900/0xa79924/0xa7997c/0xa7999c/0xa79954: set, wait, close.
        let event = SignaledEvent::new();
        event.init_event();
        assert!(!event.wait_on_event(0));
        event.set_event();
        assert!(event.wait_on_event(0));
        // Manual reset: still latched until cleared.
        assert!(event.wait_on_event(0));
        event.reset();
        assert!(!event.wait_on_event(0));
        event.close_event();
    }

    #[test]
    fn mutex_excludes() {
        // IDA 0xa7a0b4/0xa7a0d4/0xa7a0e0: lock, unlock, relock.
        let mutex = SimpleMutex::new();
        mutex.lock();
        assert!(mutex.locked.load(Ordering::SeqCst));
        mutex.unlock();
        assert!(!mutex.locked.load(Ordering::SeqCst));
        mutex.lock();
        mutex.unlock();
    }
}
