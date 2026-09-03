use parking_lot::Mutex;
use std::sync::{Arc, Weak};
/// Rust replacement for `rbx::signals::signal` / `boost::signals2`.
/// Keep `connect`/`disconnect` API. Interior: Mutex<Vec<Weak<dyn Fn>>>.
#[allow(clippy::type_complexity)]
pub struct Signal<T> {
    slots: Mutex<Vec<Weak<dyn Fn(T) + Send + Sync>>>,
}
impl<T> Default for Signal<T> {
    fn default() -> Self { Self { slots: Mutex::new(Vec::new()) } }
}
impl<T> Signal<T> {
    pub fn new() -> Self { Self::default() }
    pub fn connect<F>(&self, f: Arc<F>) where F: Fn(T) + Send + Sync + 'static {
        let erased: Arc<dyn Fn(T) + Send + Sync> = f;
        self.slots.lock().push(Arc::downgrade(&erased));
    }
    pub fn disconnect_all(&self) { self.slots.lock().clear(); }
    pub fn fire(&self, val: T) where T: Clone {
        let live: Vec<Arc<dyn Fn(T) + Send + Sync>> =
            self.slots.lock().iter().filter_map(Weak::upgrade).collect();
        for slot in live {
            slot(val.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicI32, Ordering};

    #[test]
    fn fire_reaches_live_slots_and_prunes_dead() {
        let s = Signal::new();
        let a = Arc::new(AtomicI32::new(0));
        let b = Arc::new(AtomicI32::new(0));
        let ca = Arc::clone(&a);
        let cb = Arc::clone(&b);
        let ha = Arc::new(move |v: i32| { ca.fetch_add(v, Ordering::SeqCst); });
        let hb = Arc::new(move |v: i32| { cb.fetch_add(v, Ordering::SeqCst); });
        s.connect(ha.clone());
        s.connect(hb.clone());
        s.fire(5);
        assert_eq!(a.load(Ordering::SeqCst), 5);
        assert_eq!(b.load(Ordering::SeqCst), 5);
        drop(ha);
        drop(a);
        s.fire(7);
        assert_eq!(b.load(Ordering::SeqCst), 12);
    }

    #[test]
    fn disconnect_all_silences() {
        let s = Signal::new();
        let a = Arc::new(AtomicI32::new(0));
        let ca = Arc::clone(&a);
        s.connect(Arc::new(move |v: i32| { ca.fetch_add(v, Ordering::SeqCst); }));
        s.disconnect_all();
        s.fire(3);
        assert_eq!(a.load(Ordering::SeqCst), 0);
    }
}
