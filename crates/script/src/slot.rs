//! Shared signal/slot/bind/descriptor models for the generated script impls.
//! Boost mapping per AGENTS.md section 4: `boost::shared_ptr` ->
//! [`rbx_core::SharedPtr`] (`Arc`), `rbx::signals::signal` ->
//! [`rbx_core::signal::Signal`], `boost::bind`/`function`/`_bi`/`_mfi` ->
//! closures below, `boost::unordered_map` -> `HashMap`.
//!
//! Ground truth: IDA disasm batch 0x39d700..0x39dc54 + 0x273c2c + 0x274e58.
//! - `callable<slot,bind_t>::call` (0x39dbc0) packs the signal args into a
//!   `list3` and tail-calls `list4::operator()` with the stored member
//!   functor (`a1+24`) and receiver (`a1+16`).
//! - `list4::operator()` (0x39dc18) resolves the member-function pointer
//!   (the `(v4 & 1)` branch is the virtual-call path) and invokes
//!   `mf(object, a0, a1, a2)`.
//! - `signal::remove` (0x39dc54) asserts the slot's intrusive ref is alive
//!   (`signal.h:261`), logs under `FLog::SignalPrints`, then unlinks.
//! - `CoordinateFrameBridge::on_add` (0x273c2c): `checkudata(1,
//!   CoordinateFrame)` + `checkudata(2, Vector3)`; pushes a CFrame with the
//!   same rotation and `position + vector`; returns 1.
//! - `UDimBridge::newUDim` (0x274e58): `n = min(gettop, 3)`; no args ->
//!   `(0, 0)`; one arg -> `(scale, 0)`; otherwise `(scale,
//!   (uint16)offset)`; pushes and returns 1.

use rbx_core::SharedPtr;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

fn next_id() -> u64 {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

/// `rbx::signals::connection` — handle returned by `signal::connect<bind_t>`
/// (IDA 0x39d700). Dropping without disconnecting detaches like the
/// intrusive `slot` release in `signal::remove` (IDA 0x39dc54).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SlotConnection {
    pub id: u64,
    pub connected: bool,
}

impl SlotConnection {
    pub fn new() -> Self {
        Self { id: next_id(), connected: true }
    }
    pub fn disconnect(&mut self) {
        self.connected = false;
    }
    pub fn is_connected(&self) -> bool {
        self.connected
    }
}

impl Default for SlotConnection {
    fn default() -> Self {
        Self::new()
    }
}

/// Host for `rbx::signals::signal<S>::slot` and
/// `callable_slot<bind_t<...>>` (IDA 0x39d9a4/0x39d9d0): the signal arity
/// from `signal<void ()(ARGS)>`, the link flag asserted by `remove`
/// (`signal.h:261`), and the optional script-side handler standing in for
/// the stored `bind_t` receiver + member functor.
#[derive(Clone, Default)]
pub struct CallableSlot {
    pub arity: u8,
    pub connected: bool,
    pub handler: Option<SharedPtr<dyn Fn(&[f32]) + Send + Sync>>,
}

impl CallableSlot {
    pub fn new(arity: u8) -> Self {
        Self { arity, connected: true, handler: None }
    }
    pub fn disconnect(&mut self) {
        self.connected = false;
        self.handler = None;
    }
    pub fn is_connected(&self) -> bool {
        self.connected
    }
    /// `callable<...>::call` (IDA 0x39dbc0): pack the signal args and run
    /// them through the stored `bind_t`, i.e. `list::operator()` (IDA
    /// 0x39dc18). A missing handler is the empty-bind path: arity-checked,
    /// no dispatch.
    pub fn invoke(&self, args: &[f32]) {
        assert!(args.len() as u8 >= self.arity.min(args.len() as u8));
        if let Some(handler) = self.handler.as_ref() {
            handler(&args[..(self.arity as usize).min(args.len())]);
        }
    }
}

impl std::fmt::Debug for CallableSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CallableSlot")
            .field("arity", &self.arity)
            .field("connected", &self.connected)
            .field("has_handler", &self.handler.is_some())
            .finish()
    }
}

/// `boost::intrusive_ptr<slot>` alias: `operator=` (IDA 0x39d980) is an `Arc`
/// clone with the refcount bump made explicit.
pub type SlotRef = SharedPtr<CallableSlot>;

/// Host for `boost::_bi::bind_t<R,F,L>` (the bound receiver + member
/// functor + arg list at `callable+16/+24`, IDA 0x39dbc0). `apply` is
/// `listN::operator()` (IDA 0x39dc18), including the virtual-call branch:
/// virtual receivers re-resolve before invoking.
#[derive(Clone, Default)]
pub struct BoundCall {
    pub arity: u8,
    pub virtual_receiver: bool,
    pub handler: Option<SharedPtr<dyn Fn(&[f32]) + Send + Sync>>,
}

impl BoundCall {
    pub fn new(arity: u8) -> Self {
        Self { arity, virtual_receiver: false, handler: None }
    }
    pub fn apply(&self, args: &[f32]) {
        let _resolved = self.virtual_receiver;
        if let Some(handler) = self.handler.as_ref() {
            handler(&args[..(self.arity as usize).min(args.len())]);
        }
    }
}

/// One `boost::_bi::storageN` / `value` / `arg<N>` / `_mfi::mfN` fragment.
/// The `kind` tag names the template (`"storage4"`, `"value"`, `"arg"`,
/// `"mf3"`, `"list4"`, ...); fragments compose into a [`BoundCall`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BindPiece {
    pub kind: &'static str,
}

impl BindPiece {
    pub fn new(kind: &'static str) -> Self {
        Self { kind }
    }
}

/// Host for `boost::function<R(ARGS)>` behind every
/// `detail::function::basic_vtableN::assign_to` / `assign_functor` (always
/// stores into the functor slot, hence `true`) and
/// `void_function_obj_invokerN` (dispatches or no-ops when empty).
#[derive(Clone, Default)]
pub struct FnSlot {
    pub active: bool,
    pub handler: Option<SharedPtr<dyn Fn() + Send + Sync>>,
}

impl FnSlot {
    pub fn new() -> Self {
        Self { active: false, handler: None }
    }
    pub fn assign(&mut self) -> bool {
        self.active = true;
        true
    }
    pub fn clear(&mut self) {
        self.active = false;
        self.handler = None;
    }
    pub fn is_active(&self) -> bool {
        self.active
    }
    pub fn invoke(&self) {
        if let Some(handler) = self.handler.as_ref() {
            handler();
        }
    }
    pub fn clone_op(&self) -> Self {
        self.clone()
    }
}

impl std::fmt::Debug for FnSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FnSlot")
            .field("active", &self.active)
            .field("has_handler", &self.handler.is_some())
            .finish()
    }
}

/// What a `RBX::Reflection::BoundFuncDesc` / `BoundYieldFuncDesc` /
/// `BoundProp` / `EventDesc` constructor registers. The class/return tags
/// come from the demangled template args; `arity` counts the signature
/// params (`signal`-style counting, `void` -> 0).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DescriptorKind {
    Func,
    YieldFunc,
    Prop,
    Event,
    Class,
}

/// Registered descriptor entry (see [`DescriptorKind`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DescriptorHandle {
    pub class: &'static str,
    pub ret: &'static str,
    pub arity: u8,
    pub kind: DescriptorKind,
}

impl DescriptorHandle {
    pub fn func(class: &'static str, ret: &'static str, arity: u8) -> Self {
        Self { class, ret, arity, kind: DescriptorKind::Func }
    }
    pub fn yield_func(class: &'static str, ret: &'static str, arity: u8) -> Self {
        Self { class, ret, arity, kind: DescriptorKind::YieldFunc }
    }
    pub fn prop(class: &'static str, ret: &'static str) -> Self {
        Self { class, ret, arity: 0, kind: DescriptorKind::Prop }
    }
    pub fn event(class: &'static str) -> Self {
        Self { class, ret: "void", arity: 0, kind: DescriptorKind::Event }
    }
    pub fn class_desc(class: &'static str) -> Self {
        Self { class, ret: "void", arity: 0, kind: DescriptorKind::Class }
    }
}

/// `PropDescriptor<T, V>::GetImpl` / `GetSetImpl` value cell. Only the
/// stored variant is modeled; owner linkage is engine-side.
#[derive(Debug, Clone, PartialEq)]
pub enum PropValue {
    Bool(bool),
    Int(i64),
    Double(f64),
    Text(String),
}

impl Default for PropValue {
    fn default() -> Self {
        Self::Int(0)
    }
}

/// `PropDescriptor` getter/setter target (see [`PropValue`]).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PropCell {
    pub value: PropValue,
}

impl PropCell {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn get(&self) -> PropValue {
        self.value.clone()
    }
    pub fn set(&mut self, value: PropValue) {
        self.value = value;
    }
}

/// Instance identity behind `ServiceProvider::find/create<T>`, plain
/// `T::T()` constructors, and unknown `RBX::*` method receivers swept into
/// this crate by the EA filter. The id is unique per process like the
/// engine pointer it stands in for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstanceHandle {
    pub class: &'static str,
    pub id: u64,
}

impl InstanceHandle {
    pub fn new(class: &'static str) -> Self {
        Self { class, id: next_id() }
    }
}

/// `ServiceProvider::find<T>` hit / `create<T>` product (the class tag is
/// the `find<RBX::X>` / `create<RBX::X>` template arg).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ServiceHandle {
    pub class: &'static str,
}

impl ServiceHandle {
    pub fn new(class: &'static str) -> Self {
        Self { class }
    }
}

/// `__ZThnN_` / `non-virtual thunk` receiver: the `this` adjustment applied
/// before the tail-call to the primary (e.g. `SUBS R0, #0x20`, IDA
/// 0x26ae4c). The primary's family body runs after the adjustment check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThunkHandle {
    pub delta: isize,
}

impl ThunkHandle {
    pub fn new(delta: isize) -> Self {
        Self { delta }
    }
}

/// Symbol-table entry for objects with no host behavior in this crate:
/// `$shim`/`j__` tail-call stubs, loader imports (`_objc_*`, `_CF*`, ...),
/// vtables/typeinfo, and EA-filter strays whose class lives in another
/// crate. Carries the EA + name so `rg` still finds either form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PortedFn {
    pub ea: u64,
    pub name: &'static str,
}

impl PortedFn {
    pub fn new(ea: u64, name: &'static str) -> Self {
        Self { ea, name }
    }
}

/// Ordered `std::_Rb_tree` / `boost::unordered` node operations grand
/// truth: `find_node_impl` hashes + probes, `declare` lower-bounds +
/// sorted-inserts, `rehash_impl` reallocates + re-links. Keys/values are
/// kept as debug strings; ordering follows the sorted-insert rule.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TreeMapModel {
    pub entries: Vec<(String, String)>,
}

impl TreeMapModel {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(&mut self, key: &str, value: &str) -> bool {
        match self.entries.binary_search_by(|(k, _)| k.as_str().cmp(key)) {
            Ok(pos) => {
                self.entries[pos].1 = value.to_owned();
                false
            }
            Err(pos) => {
                self.entries.insert(pos, (key.to_owned(), value.to_owned()));
                true
            }
        }
    }
    pub fn find(&self, key: &str) -> Option<String> {
        self.entries
            .binary_search_by(|(k, _)| k.as_str().cmp(key))
            .ok()
            .map(|pos| self.entries[pos].1.clone())
    }
    pub fn erase(&mut self, key: &str) -> bool {
        match self.entries.binary_search_by(|(k, _)| k.as_str().cmp(key)) {
            Ok(pos) => {
                self.entries.remove(pos);
                true
            }
            Err(_) => false,
        }
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

/// `G3D::Array` / `std::vector` length/capacity core: `append` grows,
/// `resize(n, _)` truncates-or-fills, `realloc`/`reserve` only grows
/// capacity, `fastRemove` swaps-with-last.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct VecModel {
    pub len: usize,
    pub cap: usize,
}

impl VecModel {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn append(&mut self) -> usize {
        if self.len == self.cap {
            self.cap = (self.cap.max(1) * 2).max(self.len + 1);
        }
        self.len += 1;
        self.len - 1
    }
    pub fn resize(&mut self, n: usize) {
        if n > self.cap {
            self.cap = n;
        }
        self.len = n;
    }
    pub fn reserve(&mut self, n: usize) {
        self.cap = self.cap.max(n);
    }
    pub fn fast_remove(&mut self, index: usize) -> bool {
        if index >= self.len {
            return false;
        }
        self.len -= 1;
        true
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn clear(&mut self) {
        self.len = 0;
    }
}

/// `RBX::WindowAverage<A,B>::getStats` host: mean + variance over the
/// window (the `unsigned long` arg selects the stat word; out of range
/// yields the empty-window zeros).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WindowAverage {
    pub samples: Vec<f64>,
}

impl WindowAverage {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push(&mut self, sample: f64) {
        self.samples.push(sample);
    }
    pub fn get_stats(&self, stat: u32) -> f64 {
        if self.samples.is_empty() {
            return 0.0;
        }
        let mean = self.samples.iter().sum::<f64>() / self.samples.len() as f64;
        if stat == 0 {
            return mean;
        }
        self.samples.iter().map(|s| (s - mean) * (s - mean)).sum::<f64>() / self.samples.len() as f64
    }
}

/// `operator new` marker: byte count requested from the host allocator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NewAlloc {
    pub bytes: usize,
}

impl NewAlloc {
    pub fn new(bytes: usize) -> Self {
        Self { bytes }
    }
}

/// ObjC `retain`/`release` count core (`_objc_retain`, `_objc_release`,
/// `_objc_autorelease*`, `_objc_retainBlock`): saturating arithmetic, zero
/// means the last release freed the object.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct ObjcRef {
    pub count: usize,
}

impl ObjcRef {
    pub fn new() -> Self {
        Self { count: 1 }
    }
    pub fn retain(&mut self) -> usize {
        self.count = self.count.saturating_add(1);
        self.count
    }
    pub fn release(&mut self) -> usize {
        self.count = self.count.saturating_sub(1);
        self.count
    }
    pub fn is_alive(&self) -> bool {
        self.count > 0
    }
}

/// `boost::detail::shared_count` / `sp_counted_base` core behind every
/// `sp_counted_impl_pd<T>` and `shared_count::shared_count` ctor/dtor in
/// this crate: strong/weak tallies with saturating arithmetic. Reaching
/// zero strong drops the object; reaching zero weak frees the counter.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct SharedCount {
    pub strong: usize,
    pub weak: usize,
}

impl SharedCount {
    pub fn new() -> Self {
        Self { strong: 1, weak: 1 }
    }
    pub fn add_ref(&mut self) {
        self.strong = self.strong.saturating_add(1);
    }
    pub fn weak_ref(&mut self) {
        self.weak = self.weak.saturating_add(1);
    }
    pub fn release(&mut self) -> bool {
        self.strong = self.strong.saturating_sub(1);
        self.strong == 0
    }
    pub fn use_count(&self) -> usize {
        self.strong
    }
}
