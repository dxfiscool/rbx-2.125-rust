use std::sync::{Arc, Weak, Mutex};
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
    pub fn connect<F>(&self, _f: Arc<F>) where F: Fn(T) + Send + Sync + 'static { todo!("Signal::connect") }
    pub fn disconnect_all(&self) { self.slots.lock().unwrap().clear(); }
    pub fn fire(&self, _val: T) { todo!("Signal::fire") }
}
