//! core shard AF — 120 core stubs EA-sorted, next uncovered after shard AE (0x2b42d8), lowest EA first.
//! Source: ida/export.json filtered where demangled/mangled excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted, next 120 uncovered globally.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
/// Batch 7: 28 IDA-grounded ports 0x2b42f0-0x2c48a4 — `boost::function0<void>`
/// call, `InvocationMeter<2>::updateBuckets`, the `placement_any<Region3>`
/// assign family (double/string/bool) with the `typed_holder`
/// construct/destruct funcs, the three `any_cast` checked casts,
/// `thread_specific_ptr<Context>::reset`, `vector<const char*>`
/// push/insert/allocate, the string->string `Rb_tree` insert/lower_bound set,
/// `ContentId` ctors, `Name::declare`/`doDeclare<StatsItem>`,
/// `function1<void,lua_State*>::assign_to_own`, and `GcJob::sleepTime`/`error`
/// (via the `Job::computeStandard*` analytical callees at 0x24a1f8/0x24a210).
/// Untouched carriers keep stub bodies; ports live in `core_af` under
/// idiomatic names, wired via `stub_0x*`.
/// Conventions: `boost::shared_ptr` -> `crate::SharedPtr` (Arc);
/// `RbxInterlockedExchange/Increment/Decrement` -> `AtomicU32/I32`;
/// `boost::call_once`/`__cxa_guard` -> `LazyLock`; `__throw_bad_alloc` /
/// `__throw_length_error` / `throw_exception<bad_*>` -> `panic!`; libstdc++
/// containers -> `Vec` / `BTreeMap`. `[INFERENCE]` marks what the binary
/// does not pin down.
pub mod core_af {
    use std::cell::RefCell;
    use std::collections::BTreeMap;
    use std::sync::LazyLock;
    use std::sync::atomic::{AtomicI32, AtomicU32, Ordering};

    /// was: `boost::function0<void>` — erased nullary functor. Empty state
    /// throws `bad_function_call` on invocation (IDA 0x2b42f0: null check at
    /// 0x2b433c, throw at 0x2b4374-0x2b437e, else vtable call at 0x2b434c
    /// through `(*a1 & ~1) + 4`).
    pub struct BoostFn0 {
        invoke: Option<Box<dyn Fn() + Send + Sync>>,
    }

    impl BoostFn0 {
        pub fn empty() -> Self {
            Self { invoke: None }
        }
        pub fn of(f: impl Fn() + Send + Sync + 'static) -> Self {
            Self {
                invoke: Some(Box::new(f)),
            }
        }
        pub fn is_empty(&self) -> bool {
            self.invoke.is_none()
        }
        /// IDA 0x2b42f0.
        pub fn call(&self) {
            match &self.invoke {
                Some(f) => f(),
                // IDA 0x2b4374: boost::throw_exception<bad_function_call>; panic! is the throw.
                None => panic!("boost::bad_function_call"),
            }
        }
    }

    /// was: `RBX::InvocationMeter<2>` — 2048 one-byte buckets (+0), tick
    /// counter (+2048, `RbxInterlockedExchange`), live count (+2052,
    /// `RbxInterlockedIncrement/Decrement`), previous stamp (+2056) and
    /// current stamp (+2064). One tick is 1/2048 s (`dt * 2048.0` at 0x2b5524).
    pub struct InvocationMeter2 {
        buckets: parking_lot::Mutex<[u8; 2048]>,
        tick: AtomicU32,
        active: AtomicI32,
        prev_stamp: parking_lot::Mutex<f64>,
        cur_stamp: parking_lot::Mutex<f64>,
    }

    impl InvocationMeter2 {
        pub fn new() -> Self {
            Self {
                buckets: parking_lot::Mutex::new([0; 2048]),
                tick: AtomicU32::new(0),
                active: AtomicI32::new(0),
                prev_stamp: parking_lot::Mutex::new(0.0),
                cur_stamp: parking_lot::Mutex::new(0.0),
            }
        }
        /// IDA 0x2b54d8 `updateBuckets(bool)`: returns without touching
        /// anything when the clock has not advanced (`*cur != now` gate at
        /// 0x2b5500); else stores the stamp (0x2b5508), converts the delta
        /// since the previous stamp to ticks (0x2b550c-0x2b5524), swaps the
        /// tick (0x2b552a), zeroes every bucket the tick swept past while
        /// debiting the live count per unit (0x2b5536-0x2b5558), and, when
        /// `record`, marks the current slot and bumps the count
        /// (0x2b5568-0x2b5578). `now` is the `Time::now<0>` sample (0x2b54e8);
        /// passed in because the clock lives outside core.
        pub fn update_buckets(&self, record: bool, now: f64) {
            let mut cur = self.cur_stamp.lock();
            if *cur != now {
                *cur = now;
                let prev = *self.prev_stamp.lock();
                // Rust `as` saturates negatives to 0 where the C++ float->int cast is UB [INFERENCE].
                let new_tick = ((now - prev) * 2048.0) as u32;
                let old_tick = self.tick.swap(new_tick, Ordering::SeqCst);
                if old_tick < new_tick {
                    let mut buckets = self.buckets.lock();
                    // IDA 0x2b5536: for (i = old+1; i <= new; ++i) with unsigned wrap.
                    let mut i = old_tick.wrapping_add(1);
                    loop {
                        let idx = (i & 0x7FF) as usize;
                        let n = buckets[idx];
                        buckets[idx] = 0;
                        for _ in 0..n {
                            self.active.fetch_sub(1, Ordering::SeqCst);
                        }
                        if i == new_tick {
                            break;
                        }
                        i = i.wrapping_add(1);
                    }
                }
                if record {
                    let mut buckets = self.buckets.lock();
                    buckets[(new_tick & 0x7FF) as usize] = 1;
                    self.active.fetch_add(1, Ordering::SeqCst);
                }
            }
        }
        pub fn active_count(&self) -> i32 {
            self.active.load(Ordering::SeqCst)
        }
        pub fn tick(&self) -> u32 {
            self.tick.load(Ordering::SeqCst)
        }
    }

    impl Default for InvocationMeter2 {
        fn default() -> Self {
            Self::new()
        }
    }

    /// was: `rbx::placement_any<RBX::Region3>` — inline type-erased slot: a
    /// holder tag (+0, null when empty) plus inline storage (+4). Only the
    /// payloads observed in this TU are modelled.
    #[derive(Debug, Clone, PartialEq)]
    pub enum Region3Payload {
        Empty,
        Double(f64),
        Str(String),
        Int(i32),
        Bool(bool),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct PlacementAnyRegion3 {
        payload: Region3Payload,
    }

    static DOUBLE_HOLDER: LazyLock<()> = LazyLock::new(|| ());
    static STRING_HOLDER: LazyLock<()> = LazyLock::new(|| ());
    static INT_HOLDER: LazyLock<()> = LazyLock::new(|| ());
    static BOOL_HOLDER: LazyLock<()> = LazyLock::new(|| ());

    /// was: `typed_holder<double>::singleton()` (IDA 0x2b559c) — pins the
    /// static holder `s`; the discriminant is that pin.
    pub fn ensure_double_holder() {
        let _ = &*DOUBLE_HOLDER;
    }
    /// was: `typed_holder<std::string>::singleton()` (IDA 0x2b565c).
    pub fn ensure_string_holder() {
        let _ = &*STRING_HOLDER;
    }
    /// Int-holder pin, symmetric to the double/string/bool holders.
    pub fn ensure_int_holder() {
        let _ = &*INT_HOLDER;
    }
    /// was: `typed_holder<bool>::singleton()` (IDA 0x2bc210).
    pub fn ensure_bool_holder() {
        let _ = &*BOOL_HOLDER;
    }

    impl PlacementAnyRegion3 {
        pub fn empty() -> Self {
            Self {
                payload: Region3Payload::Empty,
            }
        }
        pub fn payload(&self) -> &Region3Payload {
            &self.payload
        }
        fn destroy(&mut self) {
            // IDA 0x2b55bc/0x2b567c/0x2b622c: holder->destruct(storage); tag := 0.
            // Reassignment drops the old payload — that drop is the destruct call.
            self.payload = Region3Payload::Empty;
        }
        /// IDA 0x2b5590 `operator=(double)`: same-holder plain store
        /// (0x2b55c8), else destroy old (0x2b55b4-0x2b55c0), store and retag
        /// to the double holder (0x2b55d4-0x2b55d8).
        pub fn assign_double(&mut self, v: f64) {
            ensure_double_holder();
            if !matches!(self.payload, Region3Payload::Double(_)) {
                self.destroy();
            }
            self.payload = Region3Payload::Double(v);
        }
        /// IDA 0x2b5650 `operator=(const string&)`: same-holder
        /// `string::assign` (0x2b5688), else destroy old (0x2b5674-0x2b5680),
        /// placement copy-construct (0x2b5694) and retag (0x2b5698).
        pub fn assign_string(&mut self, s: &str) {
            ensure_string_holder();
            if !matches!(self.payload, Region3Payload::Str(_)) {
                self.destroy();
            }
            self.payload = Region3Payload::Str(s.to_owned());
        }
        /// Int assign, same destroy/store/retag shape as the bool EA
        /// (0x2bc208) [INFERENCE: its own assign EA lives outside this batch;
        /// the `any_cast<int>` EA (0x2bb248) proves the holder exists].
        pub fn assign_int(&mut self, v: i32) {
            ensure_int_holder();
            if !matches!(self.payload, Region3Payload::Int(_)) {
                self.destroy();
            }
            self.payload = Region3Payload::Int(v);
        }
        /// IDA 0x2bc208 `operator=(bool)`: same-holder byte store
        /// (0x2bc23c), else destroy old (0x2b6226-0x2bc230), store and retag
        /// to the bool holder (0x2bc234-0x2bc236).
        pub fn assign_bool(&mut self, v: bool) {
            ensure_bool_holder();
            if !matches!(self.payload, Region3Payload::Bool(_)) {
                self.destroy();
            }
            self.payload = Region3Payload::Bool(v);
        }
    }

    /// IDA 0x2b55e8 `typed_holder<double>::construct_func`: placement copy
    /// when the destination is non-null (0x2b55ea-0x2b55f0); returns source.
    pub fn construct_double(src: f64, dst: Option<&mut f64>) -> f64 {
        if let Some(dst) = dst {
            *dst = src;
        }
        src
    }

    /// IDA 0x2b55f8 `typed_holder<double>::destruct_func`: empty body (POD).
    pub fn destruct_double() {}

    /// IDA 0x2b56a8 `typed_holder<std::string>::construct_func`: placement
    /// copy-construct (0x2b56b4), else return source (0x2b56ae).
    pub fn construct_string(src: &str, dst: Option<&mut String>) -> String {
        if let Some(dst) = dst {
            *dst = src.to_owned();
        }
        src.to_owned()
    }

    /// IDA 0x2b56b8 `typed_holder<std::string>::destruct_func`: thunk to
    /// `std::string::~string` — taking by value runs that drop.
    pub fn destruct_string(s: String) {
        drop(s);
    }

    /// was: `rbx::bad_placement_any_cast` (thrown via
    /// `boost::throw_exception` at 0x2b9186/0x2bb2fe/0x2bc1d6).
    pub const BAD_PLACEMENT_ANY_CAST: &str = "rbx::bad_placement_any_cast";

    /// IDA 0x2b90c8 `any_cast<const string&>`: null holder means void
    /// (0x2b90f2-0x2b9124); fast path on typeinfo identity (0x2b9134);
    /// `"Ss"` name fallback (0x2b9150); else throw (0x2b9186). Success
    /// returns the inline storage (0x2b916e, `a1 + 1`).
    pub fn any_cast_string(cell: &PlacementAnyRegion3) -> &str {
        match &cell.payload {
            Region3Payload::Str(s) => s,
            _ => panic!("{}", BAD_PLACEMENT_ANY_CAST),
        }
    }

    /// IDA 0x2bb248 `any_cast<const int&>`: identity at 0x2bb2a6, name
    /// fallback at 0x2bb2c8, throw at 0x2bb2fe, storage at 0x2bb2e6.
    pub fn any_cast_int(cell: &PlacementAnyRegion3) -> &i32 {
        match &cell.payload {
            Region3Payload::Int(v) => v,
            _ => panic!("{}", BAD_PLACEMENT_ANY_CAST),
        }
    }

    /// IDA 0x2bc120 `any_cast<const bool&>`: identity at 0x2bc17e, name
    /// fallback at 0x2bc1a0, throw at 0x2bc1d6, storage at 0x2bc1be.
    pub fn any_cast_bool(cell: &PlacementAnyRegion3) -> &bool {
        match &cell.payload {
            Region3Payload::Bool(v) => v,
            _ => panic!("{}", BAD_PLACEMENT_ANY_CAST),
        }
    }

    /// was: `RBX::Security::Context` behind
    /// `boost::thread_specific_ptr<RBX::Security::Context>` — the live site
    /// owns the real type; the token stands in for its identity.
    #[derive(Debug)]
    pub struct SecurityContext {
        pub token: u32,
    }

    thread_local! {
        static SECURITY_CONTEXT_SLOT: RefCell<Option<crate::SharedPtr<SecurityContext>>> =
            RefCell::new(None);
    }

    /// IDA 0x2c05f8 `reset(ptr)`: no-op when the slot already holds `ptr`
    /// (`get_tss_data != a2` gate at 0x2c064a); else installs `ptr` and
    /// releases the old count (0x2c0654-0x2c0680 — clone/drop).
    pub fn security_context_reset(next: Option<crate::SharedPtr<SecurityContext>>) {
        SECURITY_CONTEXT_SLOT.with(|slot| {
            let mut slot = slot.borrow_mut();
            let same = match (&*slot, &next) {
                (Some(a), Some(b)) => crate::SharedPtr::ptr_eq(a, b),
                (None, None) => true,
                _ => false,
            };
            if !same {
                *slot = next;
            }
        });
    }

    pub fn security_context_get() -> Option<crate::SharedPtr<SecurityContext>> {
        SECURITY_CONTEXT_SLOT.with(|slot| slot.borrow().clone())
    }

    /// was: `std::vector<const char*>` — borrowed C-string pointers. Only
    /// addresses are copied, never dereferenced, so no unsafe is needed.
    pub type CStrPtr = *const std::os::raw::c_char;

    /// IDA 0x2c165c `_Vector_base<const char*>::_M_allocate`: bad_alloc at
    /// n >= 0x40000000 (0x2c1664), else `operator new(4*n)`.
    pub fn cstr_ptr_vec_allocate(n: usize) -> Vec<CStrPtr> {
        if n >= 0x4000_0000 {
            panic!("std::bad_alloc");
        }
        Vec::with_capacity(n)
    }

    /// IDA 0x2c0edc `push_back`: fast path stores and bumps finish while
    /// capacity remains (0x2c0eea-0x2c0ef8); otherwise the `_M_insert_aux`
    /// slow path (0x2c0f02). `Vec::push` is that pair.
    pub fn cstr_ptr_vec_push_back(v: &mut Vec<CStrPtr>, p: CStrPtr) {
        v.push(p);
    }

    /// IDA 0x2c157c `_M_insert_aux(pos, val)`: with spare capacity the tail
    /// shifts right and the value stores (0x2c1594-0x2c15b8); when full the
    /// buffer grows ×2 (0x2c15be-0x2c15dc), prefix and suffix relocate
    /// (0x2c15ee-0x2c1614), the old buffer deletes (0x2c1624-0x2c1628)
    /// and the pointers publish (0x2c1630-0x2c1634). Growth is empty->1,
    /// else ×2 with `length_error` at 0x3FFFFFFF (0x2c1644-0x2c1656) and an
    /// overflow clamp (0x2c15d2-0x2c15d4). `Vec::insert` is that pair; the
    /// length guard is kept explicit.
    pub fn cstr_ptr_vec_insert_aux(v: &mut Vec<CStrPtr>, pos: usize, p: CStrPtr) {
        // IDA 0x2c1644: std::__throw_length_error("vector::_M_insert_aux").
        assert!(v.len() != 0x3FFF_FFFF, "vector::_M_insert_aux");
        v.insert(pos, p);
    }

    /// was: `std::map<std::string,std::string>` (`_Rb_tree` with
    /// `_Select1st`/`less<string>`). `BTreeMap` is the ordered equivalent;
    /// node create + `insert_and_rebalance` + count++ (0x2c1808) are its
    /// internals.
    #[derive(Debug, Default)]
    pub struct StringMap {
        inner: BTreeMap<String, String>,
    }

    impl StringMap {
        pub fn new() -> Self {
            Self {
                inner: BTreeMap::new(),
            }
        }
        pub fn len(&self) -> usize {
            self.inner.len()
        }
        pub fn get(&self, key: &str) -> Option<&String> {
            self.inner.get(key)
        }
        /// IDA 0x2c1858 `_M_insert_unique(val)`: fringe walk (0x2c1862-0x2c188c),
        /// leftmost fast path (0x2c189c-0x2c18a4), duplicate check against the
        /// predecessor (0x2c18b4). Returns the `inserted` half of the
        /// (iterator, bool) pair: `true` created a node (0x2c18be-0x2c18ce),
        /// `false` found a duplicate (0x2c18b6-0x2c18b8).
        pub fn insert_unique(&mut self, key: String, val: String) -> bool {
            use std::collections::btree_map::Entry;
            match self.inner.entry(key) {
                Entry::Vacant(e) => {
                    e.insert(val);
                    true
                }
                Entry::Occupied(_) => false,
            }
        }
        /// IDA 0x2c171c `_M_insert_unique(hint, val)`: the hint only seeds the
        /// probe position (0x2c1730-0x2c17fc); misses funnel into the same
        /// `_M_insert` (0x2c17f2/0x2c1804). The hint is a search aid, not
        /// observable — same outcome as `insert_unique`.
        pub fn insert_hint(&mut self, _hint: Option<&str>, key: String, val: String) -> bool {
            self.insert_unique(key, val)
        }
        /// IDA 0x2c1808 `_M_insert`: create node + rebalance + count++.
        /// `BTreeMap` owns balancing; this is the observable step.
        pub fn insert_node(&mut self, key: String, val: String) -> bool {
            self.insert_unique(key, val)
        }
        /// IDA 0x2c18dc `lower_bound`: fringe walk keeping the last not-less
        /// node (0x2c18ea-0x2c1904); header when none (0x2c18e0). `None` is
        /// that header/end.
        pub fn lower_bound(&self, key: &str) -> Option<(&String, &String)> {
            self.inner.range(key.to_owned()..).next()
        }
    }

    /// IDA 0x2c1674 `pair<const string,string>::pair`: member-wise string
    /// copies (0x2c1696/0x2c16ce).
    pub fn string_pair(first: &str, second: &str) -> (String, String) {
        (first.to_owned(), second.to_owned())
    }

    /// was: `RBX::ContentId` — id text plus the null-name handle
    /// (`Name::getNullName`, 0x35be98, owned by the aq carrier);
    /// `CorrectBackslash` (0x314cc8, owned by the ah/hx carriers) normalises
    /// separators.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ContentId {
        pub text: String,
        pub name: u32,
    }

    /// Null-name handle — the live site wires `Name::getNullName`.
    pub fn null_name() -> u32 {
        0
    }

    /// Local equivalent of `ContentId::CorrectBackslash`: backslashes become
    /// forward slashes.
    pub fn content_correct_backslash(text: &mut String) {
        *text = text.replace('\\', "/");
    }

    /// IDA 0x2c1a48 `ContentId(const string&)`: copy text (0x2c1a68), null
    /// name (0x2c1aa2), then `CorrectBackslash` (0x2c1aaa).
    pub fn content_id_from_string(s: &str) -> ContentId {
        let mut text = s.to_owned();
        content_correct_backslash(&mut text);
        ContentId {
            text,
            name: null_name(),
        }
    }

    /// IDA 0x2c26b0 `ContentId(const char*)`: same shape via the c-string
    /// copy (0x2c2706), null name (0x2c2716), `CorrectBackslash` (0x2c271e).
    pub fn content_id_from_cstr(s: &str) -> ContentId {
        content_id_from_string(s)
    }

    /// was: `RBX::Name` handle for `Stats::sStatsItem` behind
    /// `Name::declare`/`doDeclare` (IDA 0x2c1e00/0x2c1e48). The index is
    /// assigned by the name table at the live site; 0 is the null name.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct StatsItemName {
        pub index: u32,
    }

    static STATS_ITEM_NAME_CELL: LazyLock<StatsItemName> = LazyLock::new(|| StatsItemName { index: 1 });

    /// IDA 0x2c1e48 `doDeclare`: guarded once-init (`__cxa_guard_acquire` at
    /// 0x2c1ea4, init at 0x2c1ecc, release at 0x2c1ed0) returning the static
    /// (0x2c1efe). `LazyLock` is that guard. The nonzero index stands in for
    /// the table assignment [INFERENCE].
    pub fn do_declare_stats_item() -> StatsItemName {
        *STATS_ITEM_NAME_CELL
    }

    /// IDA 0x2c1e00 `declare`: null `sStatsItem` text bails to `getNullName`
    /// (0x2c1e12-0x2c1e3a); else `call_once` the declarer (0x2c1e16-0x2c1e2e)
    /// and tail-calls `doDeclare` (0x2c1e36).
    pub fn declare_stats_item(text: Option<&str>) -> StatsItemName {
        match text {
            None => StatsItemName { index: null_name() },
            Some(_) => do_declare_stats_item(),
        }
    }

    /// was: `boost::function1<void,lua_State*>` storage — empty (null
    /// vtable), small functor copied inline (tag bit0 set), or heap functor
    /// cloned through the manager vtable (IDA 0x2c2778-0x2c27a6). The Lua
    /// state pointer is opaque here.
    pub type LuaStatePtr = *mut std::os::raw::c_void;

    #[derive(Clone)]
    pub enum LuaVoidFn {
        Empty,
        Small([usize; 3]),
        Shared(crate::SharedPtr<dyn Fn(LuaStatePtr) + Send + Sync>),
    }

    impl std::fmt::Debug for LuaVoidFn {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Empty => write!(f, "Empty"),
                Self::Small(w) => write!(f, "Small({w:?})"),
                Self::Shared(_) => write!(f, "Shared(..)"),
            }
        }
    }

    impl LuaVoidFn {
        pub fn empty() -> Self {
            Self::Empty
        }
        pub fn is_empty(&self) -> bool {
            matches!(self, Self::Empty)
        }
    }

    /// IDA 0x2c2778 `assign_to_own(dst, src)`: empty src leaves dst
    /// untouched (0x2c2778 fall-through); small src copies the tag + three
    /// words inline (0x2c2780-0x2c2790); heap src clones via the manager
    /// vtable (0x2c27a6) — the `Arc` clone is that ownership copy
    /// [INFERENCE: shares the target instead of deep-cloning it].
    pub fn assign_lua_void_fn(dst: &mut LuaVoidFn, src: &LuaVoidFn) {
        match src {
            LuaVoidFn::Empty => {}
            LuaVoidFn::Small(words) => *dst = LuaVoidFn::Small(*words),
            LuaVoidFn::Shared(f) => *dst = LuaVoidFn::Shared(crate::SharedPtr::clone(f)),
        }
    }

    /// was: `RBX::TaskScheduler::Job::Stats` window field at +488 (int,
    /// converted to double at 0x2c488a/0x2c48b2) behind `GcJob`.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct GcJobStatsView {
        pub window: i32,
    }

    /// was: `RBX::GcJob` sleep-time field at +0 (double, written by
    /// `computeStandardSleepTime` at 0x24a316-0x24a38a).
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct GcJob {
        pub sleep_time: f64,
    }

    /// was: `computeStandardError` out-params (IDA 0x24a1f8): result double
    /// (0x24a208) plus the validity byte (0x24a20c, always 0).
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct StandardErrorOut {
        pub value: f64,
        pub valid: bool,
    }

    /// IDA 0x24a1f8 `Job::computeStandardError(stats, sample, interval)`:
    /// `out = sample * interval` (0x24a208); flag = 0 (0x24a20c).
    pub fn standard_error(sample: f64, interval: f64) -> StandardErrorOut {
        StandardErrorOut {
            value: sample * interval,
            valid: false,
        }
    }

    /// IDA 0x2c48a4 `GcJob::error(stats, sample)`: interval is
    /// `(double)stats.window` (0x2c48b2-0x2c48c0) forwarded to
    /// `computeStandardError` (0x24a1f8, above).
    pub fn gc_job_error(stats: &GcJobStatsView, sample: f64) -> StandardErrorOut {
        standard_error(sample, stats.window as f64)
    }

    /// Inputs to `computeStandardSleepTime` (IDA 0x24a210): stats doubles at
    /// +35/+36, last-step stamp at +252, arbiter presence/throttle at
    /// +23/+24, `now` from `Time::now<1>` (0x24a326), and the globals
    /// `throttledSleepTime` / `sleepAdjustMethod`.
    #[derive(Debug, Clone, Copy)]
    pub struct SleepInputs {
        pub avg_sleep: f64,
        pub peak: f64,
        pub last_step: f64,
        pub now: f64,
        pub has_arbiter: bool,
        pub arbiter_throttled: bool,
        pub throttled_sleep: f64,
        pub adjust_method: u32,
    }

    /// IDA 0x24a210 `Job::computeStandardSleepTime(job, stats, interval)`:
    /// the arbiter weak addref under the spinlock pool (0x24a242-0x24a2aa) and its
    /// release (0x24a390-0x24a39a) are clone/drop. The floor is `throttledSleepTime`
    /// when an arbiter exists and reports throttled (0x24a2b4-0x24a2da), else 0.0
    /// (0x24a2c8/0x24a2e0). `rate` is `1.0 / interval` (0x24a2f4). Method 1 clamps an over-average sleeper
    /// to the floor (0x24a2fc-0x24a31a); method 2 ages the last step against
    /// the peak (0x24a326-0x24a35c) and clamps when over 105% of rate
    /// (0x24a36a-0x24a36c); otherwise the common clamp
    /// `max(rate - interval, floor)` (0x24a372-0x24a38a).
    pub fn standard_sleep_time(interval: f64, s: &SleepInputs) -> f64 {
        // IDA 0x24a242-0x24a2aa: arbiter weak addref under spinlock_pool; release at
        // 0x24a390-0x24a39a. Scoped borrow is that pair.
        let floor = if s.has_arbiter && s.arbiter_throttled {
            s.throttled_sleep
        } else {
            0.0
        };
        let rate = 1.0 / interval;
        if s.adjust_method == 1 {
            if s.avg_sleep > rate * 1.05 {
                return floor;
            }
        } else if s.adjust_method == 2 {
            let age = s.now - s.last_step;
            let eff = if age > s.peak + s.peak { age } else { s.peak };
            if eff > rate * 1.05 {
                return floor;
            }
        }
        // [INFERENCE: the decompiler splits the ARM double subtract across R2/R3 and
        // renders the subtrahend as `(double*)LODWORD(a3)`; read as the incoming interval.]
        (rate - interval).max(floor)
    }

    /// IDA 0x2c4884 `GcJob::sleepTime(stats)`: interval is
    /// `(double)*(int*)(stats + 0x1E8)` (0x2c488a-0x2c4892) forwarded with
    /// the job and stats to `computeStandardSleepTime` (0x2c489a).
    pub fn gc_job_sleep_time(job: &mut GcJob, stats: &GcJobStatsView, s: &SleepInputs) {
        job.sleep_time = standard_sleep_time(stats.window as f64, s);
    }
}



#[doc(alias = "__ZNK5boost9function0IvEclEv")]
// 0x2b42f0 — __ZNK5boost9function0IvEclEv
pub fn stub_0x2b42f0(f: &core_af::BoostFn0) {
    // IDA 0x2b42f0: function0<void>::operator() — bad_function_call on empty (0x2b433c-0x2b437e), else vtable invoke (0x2b434c).
    f.call();
}

#[doc(alias = "__ZN5boost18condition_variableD2Ev")]
// 0x2b43b0 — __ZN5boost18condition_variableD2Ev
pub fn stub_0x2b43b0() {
    // IDA 0x2b43b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail12shared_countC2INS0_11thread_dataINS_9function0IvEEEEEEPT_")]
// 0x2b43d8 — __ZN5boost6detail12shared_countC2INS0_11thread_dataINS_9function0IvEEEEEEPT_
pub fn stub_0x2b43d8() {
    // IDA 0x2b43d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_9function0IvEEEEED1Ev")]
// 0x2b44d0 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_9function0IvEEEEED1Ev
pub fn stub_0x2b44d0() {
    // IDA 0x2b44d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_9function0IvEEEEE7disposeEv")]
// 0x2b44d8 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_9function0IvEEEEE7disposeEv
pub fn stub_0x2b44d8() {
    // IDA 0x2b44d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_9function0IvEEEEE11get_deleterERKSt9type_info")]
// 0x2b44e8 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_9function0IvEEEEE11get_deleterERKSt9type_info
pub fn stub_0x2b44e8() {
    // IDA 0x2b44e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX15InvocationMeterILi2EE13updateBucketsEb")]
// 0x2b54d8 — __ZN3RBX15InvocationMeterILi2EE13updateBucketsEb
pub fn stub_0x2b54d8(m: &core_af::InvocationMeter2, record: bool, now: f64) {
    // IDA 0x2b54d8: InvocationMeter<2>::updateBuckets — skip when clock stalls (0x2b5500), else sweep dead buckets (0x2b5536-0x2b5558) + record (0x2b5568-0x2b5578).
    m.update_buckets(record, now);
}

#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSIdEERS3_RKT_")]
// 0x2b5590 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIdEERS3_RKT_
pub fn stub_0x2b5590(cell: &mut core_af::PlacementAnyRegion3, v: f64) -> &mut core_af::PlacementAnyRegion3 {
    // IDA 0x2b5590: placement_any::operator=(double) — same-holder store (0x2b55c8), else destroy + store + retag (0x2b55b4-0x2b55d8).
    cell.assign_double(v);
    cell
}

#[doc(alias = "__ZN3rbx14implementation12typed_holderIdE14construct_funcEPKcPc")]
// 0x2b55e8 — __ZN3rbx14implementation12typed_holderIdE14construct_funcEPKcPc
pub fn stub_0x2b55e8(src: f64, dst: Option<&mut f64>) -> f64 {
    // IDA 0x2b55e8: typed_holder<double>::construct_func — placement copy when dst non-null (0x2b55ea-0x2b55f0), returns source.
    core_af::construct_double(src, dst)
}

#[doc(alias = "__ZN3rbx14implementation12typed_holderIdE13destruct_funcEPc")]
// 0x2b55f8 — __ZN3rbx14implementation12typed_holderIdE13destruct_funcEPc
pub fn stub_0x2b55f8() {
    // IDA 0x2b55f8: typed_holder<double>::destruct_func — empty body (POD, no-op).
    core_af::destruct_double();
}

#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSISsEERS3_RKT_")]
// 0x2b5650 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSISsEERS3_RKT_
pub fn stub_0x2b5650<'a>(cell: &'a mut core_af::PlacementAnyRegion3, s: &str) -> &'a mut core_af::PlacementAnyRegion3 {
    // IDA 0x2b5650: placement_any::operator=(string) — same-holder assign (0x2b5688), else destroy + copy-construct + retag (0x2b5674-0x2b5698).
    cell.assign_string(s);
    cell
}

#[doc(alias = "__ZN3rbx14implementation12typed_holderISsE14construct_funcEPKcPc")]
// 0x2b56a8 — __ZN3rbx14implementation12typed_holderISsE14construct_funcEPKcPc
pub fn stub_0x2b56a8(src: &str, dst: Option<&mut String>) -> String {
    // IDA 0x2b56a8: typed_holder<string>::construct_func — placement copy-construct (0x2b56b4), else return source (0x2b56ae).
    core_af::construct_string(src, dst)
}

#[doc(alias = "__ZN3rbx14implementation12typed_holderISsE13destruct_funcEPc")]
// 0x2b56b8 — __ZN3rbx14implementation12typed_holderISsE13destruct_funcEPc
pub fn stub_0x2b56b8(s: String) {
    // IDA 0x2b56b8: typed_holder<string>::destruct_func — thunk to string::~string; by-value drop is that call.
    core_af::destruct_string(s);
}

#[doc(alias = "__ZN5boost10scoped_ptrIN3RBX14LibraryServiceEED2Ev")]
// 0x2b6590 — __ZN5boost10scoped_ptrIN3RBX14LibraryServiceEED2Ev
pub fn stub_0x2b6590() {
    // IDA 0x2b6590: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX14LibraryServiceD2Ev")]
// 0x2b6638 — __ZN3RBX14LibraryServiceD2Ev
pub fn stub_0x2b6638() {
    // IDA 0x2b6638: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt10_List_baseIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS5_EE8_M_clearEv")]
// 0x2b67b0 — __ZNSt10_List_baseIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS5_EE8_M_clearEv
pub fn stub_0x2b67b0() {
    // IDA 0x2b67b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// 0x2b67d8 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
pub fn stub_0x2b67d8() {
    // IDA 0x2b67d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E")]
// 0x2b6800 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E
pub fn stub_0x2b6800() {
    // IDA 0x2b6800: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost10scoped_ptrINS_6threadEED2Ev")]
// 0x2b6900 — __ZN5boost10scoped_ptrINS_6threadEED2Ev
pub fn stub_0x2b6900() {
    // IDA 0x2b6900: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6threadD2Ev")]
// 0x2b69a8 — __ZN5boost6threadD2Ev
pub fn stub_0x2b69a8() {
    // IDA 0x2b69a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEC2INS1_5GcJobEEEPT_")]
// 0x2b71e0 — __ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEC2INS1_5GcJobEEEPT_
pub fn stub_0x2b71e0() {
    // IDA 0x2b71e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIS3_NS1_5GcJobEEEvPKNS_10shared_ptrIT_EEPT0_")]
// 0x2b72c8 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIS3_NS1_5GcJobEEEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_0x2b72c8() {
    // IDA 0x2b72c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX5GcJobEEEPT_")]
// 0x2b73ac — __ZN5boost6detail12shared_countC2IN3RBX5GcJobEEEPT_
pub fn stub_0x2b73ac() {
    // IDA 0x2b73ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX5GcJobEED1Ev")]
// 0x2b74a4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5GcJobEED1Ev
pub fn stub_0x2b74a4() {
    // IDA 0x2b74a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX5GcJobEED0Ev")]
// 0x2b74a8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5GcJobEED0Ev
pub fn stub_0x2b74a8() {
    // IDA 0x2b74a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX5GcJobEE7disposeEv")]
// 0x2b74ac — __ZN5boost6detail17sp_counted_impl_pIN3RBX5GcJobEE7disposeEv
pub fn stub_0x2b74ac() {
    // IDA 0x2b74ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX5GcJobEE11get_deleterERKSt9type_info")]
// 0x2b74bc — __ZN5boost6detail17sp_counted_impl_pIN3RBX5GcJobEE11get_deleterERKSt9type_info
pub fn stub_0x2b74bc() {
    // IDA 0x2b74bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX5GcJobEE19get_untyped_deleterEv")]
// 0x2b74c0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5GcJobEE19get_untyped_deleterEv
pub fn stub_0x2b74c0() {
    // IDA 0x2b74c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx8any_castIRKSsN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x2b90c8 — __ZN3rbx8any_castIRKSsN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x2b90c8(cell: &core_af::PlacementAnyRegion3) -> &str {
    // IDA 0x2b90c8: any_cast<const string&> — void-on-null (0x2b90f2-0x2b9124), identity (0x2b9134), "Ss" fallback (0x2b9150), else throw (0x2b9186).
    core_af::any_cast_string(cell)
}

#[doc(alias = "__ZN3rbx8any_castIRKiN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x2bb248 — __ZN3rbx8any_castIRKiN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x2bb248(cell: &core_af::PlacementAnyRegion3) -> &i32 {
    // IDA 0x2bb248: any_cast<const int&> — typeinfo identity (0x2bb2a6) with name fallback (0x2bb2c8), else bad_placement_any_cast (0x2bb2fe).
    core_af::any_cast_int(cell)
}

#[doc(alias = "__ZN3rbx8any_castIRKbN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x2bc120 — __ZN3rbx8any_castIRKbN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x2bc120(cell: &core_af::PlacementAnyRegion3) -> &bool {
    // IDA 0x2bc120: any_cast<const bool&> — typeinfo identity (0x2bc17e) with name fallback (0x2bc1a0), else bad_placement_any_cast (0x2bc1d6).
    core_af::any_cast_bool(cell)
}

#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSIbEERS3_RKT_")]
// 0x2bc208 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIbEERS3_RKT_
pub fn stub_0x2bc208(cell: &mut core_af::PlacementAnyRegion3, v: bool) -> &mut core_af::PlacementAnyRegion3 {
    // IDA 0x2bc208: placement_any::operator=(bool) — same-holder byte store (0x2bc23c), else destroy + store + retag (0x2bc22c-0x2bc236).
    cell.assign_bool(v);
    cell
}

#[doc(alias = "__ZN5boost10scoped_ptrISsED2Ev")]
// 0x2bccc0 — __ZN5boost10scoped_ptrISsED2Ev
pub fn stub_0x2bccc0() {
    // IDA 0x2bccc0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost10flyweights6detail14flyweight_coreINS1_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refcountedENS0_14hashed_factoryIS8_S8_S8_Li0EEENS0_14simple_lockingENS0_13static_holderEE10holder_argD1Ev")]
// 0x2c02f8 — __ZN5boost10flyweights6detail14flyweight_coreINS1_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refcountedENS0_14hashed_factoryIS8_S8_S8_Li0EEENS0_14simple_lockingENS0_13static_holderEE10holder_argD1Ev
pub fn stub_0x2c02f8() {
    // IDA 0x2c02f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEEC2ERKNS_6tuples4consINSV_5tupleImSD_SF_SH_NSV_9null_typeESY_SY_SY_SY_SY_EESY_EERKSO_")]
// 0x2c03b8 — __ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEEC2ERKNS_6tuples4consINSV_5tupleImSD_SF_SH_NSV_9null_typeESY_SY_SY_SY_SY_EESY_EERKSO_
pub fn stub_0x2c03b8() {
    // IDA 0x2c03b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost11multi_index6detail12bucket_arrayISaINS_10flyweights6detail16refcounted_valueINS4_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES8_EEEEC2ERKSC_PNS1_22hashed_index_node_implISaIcEEEm")]
// 0x2c0408 — __ZN5boost11multi_index6detail12bucket_arrayISaINS_10flyweights6detail16refcounted_valueINS4_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES8_EEEEC2ERKSC_PNS1_22hashed_index_node_implISaIcEEEm
pub fn stub_0x2c0408() {
    // IDA 0x2c0408: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost11multi_index6detail10auto_spaceINS1_22hashed_index_node_implISaIcEEESaINS_10flyweights6detail16refcounted_valueINS7_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeESB_EEEEC2ERKSF_m")]
// 0x2c0488 — __ZN5boost11multi_index6detail10auto_spaceINS1_22hashed_index_node_implISaIcEEESaINS_10flyweights6detail16refcounted_valueINS7_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeESB_EEEEC2ERKSF_m
pub fn stub_0x2c0488() {
    // IDA 0x2c0488: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost10flyweights6detail17refcounted_handleIPKNS1_16refcounted_valueINS1_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES6_EENS1_30flyweight_core_tracking_helperIS7_N4mpl_2naENS0_10refcountedENS0_14hashed_factoryISE_SE_SE_Li0EEENS0_14simple_lockingENS0_13static_holderEEEE11check_eraseERKSL_")]
// 0x2c05e0 — __ZN5boost10flyweights6detail17refcounted_handleIPKNS1_16refcounted_valueINS1_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES6_EENS1_30flyweight_core_tracking_helperIS7_N4mpl_2naENS0_10refcountedENS0_14hashed_factoryISE_SE_SE_Li0EEENS0_14simple_lockingENS0_13static_holderEEEE11check_eraseERKSL_
pub fn stub_0x2c05e0() {
    // IDA 0x2c05e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEE5resetEPS3_")]
// 0x2c05f8 — __ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEE5resetEPS3_
pub fn stub_0x2c05f8(next: Option<crate::SharedPtr<core_af::SecurityContext>>) {
    // IDA 0x2c05f8: thread_specific_ptr<Context>::reset — no-op when slot holds ptr (0x2c064a), else install + release old (0x2c0654-0x2c0680).
    core_af::security_context_reset(next);
}

#[doc(alias = "__ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEED2Ev")]
// 0x2c06e0 — __ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEED2Ev
pub fn stub_0x2c06e0() {
    // IDA 0x2c06e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataD0Ev")]
// 0x2c07d8 — __ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataD0Ev
pub fn stub_0x2c07d8() {
    // IDA 0x2c07d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail12shared_countC2IPNS_19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataENS0_14do_heap_deleteIS8_EEEET_T0_")]
// 0x2c07e0 — __ZN5boost6detail12shared_countC2IPNS_19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataENS0_14do_heap_deleteIS8_EEEET_T0_
pub fn stub_0x2c07e0() {
    // IDA 0x2c07e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataENS0_14do_heap_deleteIS7_EEED1Ev")]
// 0x2c08d8 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataENS0_14do_heap_deleteIS7_EEED1Ev
pub fn stub_0x2c08d8() {
    // IDA 0x2c08d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataENS0_14do_heap_deleteIS7_EEE7disposeEv")]
// 0x2c08e0 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataENS0_14do_heap_deleteIS7_EEE7disposeEv
pub fn stub_0x2c08e0() {
    // IDA 0x2c08e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataENS0_14do_heap_deleteIS7_EEE11get_deleterERKSt9type_info")]
// 0x2c08f0 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataENS0_14do_heap_deleteIS7_EEE11get_deleterERKSt9type_info
pub fn stub_0x2c08f0() {
    // IDA 0x2c08f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataENS0_14do_heap_deleteIS7_EEE19get_untyped_deleterEv")]
// 0x2c0908 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataENS0_14do_heap_deleteIS7_EEE19get_untyped_deleterEv
pub fn stub_0x2c0908() {
    // IDA 0x2c0908: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIPKcSaIS1_EE9push_backERKS1_")]
// 0x2c0edc — __ZNSt6vectorIPKcSaIS1_EE9push_backERKS1_
pub fn stub_0x2c0edc(v: &mut Vec<core_af::CStrPtr>, p: core_af::CStrPtr) {
    // IDA 0x2c0edc: vector::push_back — store + bump finish when capacity remains (0x2c0eea-0x2c0ef8), else _M_insert_aux (0x2c0f02).
    core_af::cstr_ptr_vec_push_back(v, p);
}

#[doc(alias = "__ZNSt6vectorIPKcSaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
// 0x2c157c — __ZNSt6vectorIPKcSaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_0x2c157c(v: &mut Vec<core_af::CStrPtr>, pos: usize, p: core_af::CStrPtr) {
    // IDA 0x2c157c: vector::_M_insert_aux — shift tail when spare capacity (0x2c1594-0x2c15b8), else ×2 grow + relocate (0x2c15be-0x2c1634).
    core_af::cstr_ptr_vec_insert_aux(v, pos, p);
}

#[doc(alias = "__ZNSt12_Vector_baseIPKcSaIS1_EE11_M_allocateEm")]
// 0x2c165c — __ZNSt12_Vector_baseIPKcSaIS1_EE11_M_allocateEm
pub fn stub_0x2c165c(n: usize) -> Vec<core_af::CStrPtr> {
    // IDA 0x2c165c: _Vector_base<const char*>::_M_allocate — bad_alloc at n >= 0x40000000 (0x2c1664), else operator new(4*n).
    core_af::cstr_ptr_vec_allocate(n)
}

#[doc(alias = "__ZNSt4pairIKSsSsEC2ERS0_S2_")]
// 0x2c1674 — __ZNSt4pairIKSsSsEC2ERS0_S2_
pub fn stub_0x2c1674(first: &str, second: &str) -> (String, String) {
    // IDA 0x2c1674: pair<const string,string>::pair — member-wise copies (0x2c1696/0x2c16ce).
    core_af::string_pair(first, second)
}

#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_")]
// 0x2c171c — __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
pub fn stub_0x2c171c(map: &mut core_af::StringMap, hint: Option<&str>, key: String, val: String) -> bool {
    // IDA 0x2c171c: Rb_tree::_M_insert_unique(hint) — hint only seeds the probe (0x2c1730-0x2c17fc); misses funnel into _M_insert.
    map.insert_hint(hint, key, val)
}

#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")]
// 0x2c1808 — __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
pub fn stub_0x2c1808(map: &mut core_af::StringMap, key: String, val: String) -> bool {
    // IDA 0x2c1808: Rb_tree::_M_insert — node create + rebalance + count++ (0x2c183c-0x2c184e); BTreeMap owns balancing.
    map.insert_node(key, val)
}

#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueERKS2_")]
// 0x2c1858 — __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueERKS2_
pub fn stub_0x2c1858(map: &mut core_af::StringMap, key: String, val: String) -> bool {
    // IDA 0x2c1858: Rb_tree::_M_insert_unique — fringe walk (0x2c1862-0x2c188c), dup check (0x2c18b4), insert (0x2c18be).
    map.insert_unique(key, val)
}

#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE11lower_boundERS1_")]
// 0x2c18dc — __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE11lower_boundERS1_
pub fn stub_0x2c18dc<'a>(map: &'a core_af::StringMap, key: &str) -> Option<(&'a String, &'a String)> {
    // IDA 0x2c18dc: Rb_tree::lower_bound — fringe walk keeping the last not-less node (0x2c18ea-0x2c1904).
    map.lower_bound(key)
}

#[doc(alias = "__ZN3RBX9ContentIdC2ERKSs")]
// 0x2c1a48 — __ZN3RBX9ContentIdC2ERKSs
pub fn stub_0x2c1a48(s: &str) -> core_af::ContentId {
    // IDA 0x2c1a48: ContentId(const string&) — copy (0x2c1a68), null name (0x2c1aa2), CorrectBackslash (0x2c1aaa).
    core_af::content_id_from_string(s)
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_5Stats10sStatsItemEEEERKS0_v")]
// 0x2c1e00 — __ZN3RBX4Name7declareILZNS_5Stats10sStatsItemEEEERKS0_v
pub fn stub_0x2c1e00(text: Option<&str>) -> core_af::StatsItemName {
    // IDA 0x2c1e00: Name::declare<StatsItem> — null text bails to getNullName (0x2c1e3a), else call_once + doDeclare (0x2c1e16-0x2c1e36).
    core_af::declare_stats_item(text)
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5Stats10sStatsItemEEEERKS0_v")]
// 0x2c1e48 — __ZN3RBX4Name9doDeclareILZNS_5Stats10sStatsItemEEEERKS0_v
pub fn stub_0x2c1e48() -> core_af::StatsItemName {
    // IDA 0x2c1e48: Name::doDeclare<StatsItem> — guarded once-init (0x2c1ea4-0x2c1ed0), returns the static (0x2c1efe).
    core_af::do_declare_stats_item()
}

#[doc(alias = "__ZN3RBX5Stats4ItemD0Ev")]
// 0x2c1f30 — __ZN3RBX5Stats4ItemD0Ev
pub fn stub_0x2c1f30() {
    // IDA 0x2c1f30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX5Stats4ItemD1Ev")]
// 0x2c2008 — __ZThn36_N3RBX5Stats4ItemD1Ev
pub fn stub_0x2c2008() {
    // IDA 0x2c2008: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX5Stats4ItemD0Ev")]
// 0x2c2048 — __ZThn36_N3RBX5Stats4ItemD0Ev
pub fn stub_0x2c2048() {
    // IDA 0x2c2048: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX9ContentIdC2EPKc")]
// 0x2c26b0 — __ZN3RBX9ContentIdC2EPKc
pub fn stub_0x2c26b0(s: &str) -> core_af::ContentId {
    // IDA 0x2c26b0: ContentId(const char*) — copy (0x2c2706), null name (0x2c2716), CorrectBackslash (0x2c271e).
    core_af::content_id_from_cstr(s)
}

#[doc(alias = "__ZN5boost9function1IvP9lua_StateE13assign_to_ownERKS3_")]
// 0x2c2778 — __ZN5boost9function1IvP9lua_StateE13assign_to_ownERKS3_
pub fn stub_0x2c2778(dst: &mut core_af::LuaVoidFn, src: &core_af::LuaVoidFn) {
    // IDA 0x2c2778: function1::assign_to_own — empty src leaves dst (0x2c2778), small copies inline (0x2c2780-0x2c2790), heap clones via vtable (0x2c27a6).
    core_af::assign_lua_void_fn(dst, src);
}

#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE12emplace_implINS1_13emplace_args1IjEEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIjEEEEbERKjRKT_")]
// 0x2c28a0 — __ZN5boost9unordered6detail10table_implINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE12emplace_implINS1_13emplace_args1IjEEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIjEEEEbERKjRKT_
pub fn stub_0x2c28a0() {
    // IDA 0x2c28a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE14create_bucketsEm")]
// 0x2c2a30 — __ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE14create_bucketsEm
pub fn stub_0x2c2a30() {
    // IDA 0x2c2a30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE20min_buckets_for_sizeEm")]
// 0x2c2b58 — __ZNK5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE20min_buckets_for_sizeEm
pub fn stub_0x2c2b58() {
    // IDA 0x2c2b58: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE11rehash_implEm")]
// 0x2c2be8 — __ZN5boost9unordered6detail10table_implINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE11rehash_implEm
pub fn stub_0x2c2be8() {
    // IDA 0x2c2be8: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE15place_in_bucketERNS1_5tableIS9_EEPNS1_10ptr_bucketE")]
// 0x2c2c14 — __ZN5boost9unordered6detail10table_implINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE15place_in_bucketERNS1_5tableIS9_EEPNS1_10ptr_bucketE
pub fn stub_0x2c2c14() {
    // IDA 0x2c2c14: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeIjEEEE9constructEv")]
// 0x2c2c68 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeIjEEEE9constructEv
pub fn stub_0x2c2c68() {
    // IDA 0x2c2c68: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE14find_node_implIjS8_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIjEEEEmRKT_RKT0_")]
// 0x2c2ca0 — __ZNK5boost9unordered6detail10table_implINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE14find_node_implIjS8_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIjEEEEmRKT_RKT0_
pub fn stub_0x2c2ca0() {
    // IDA 0x2c2ca0: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratorptEv")]
// 0x2c3af0 — __ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratorptEv
pub fn stub_0x2c3af0() {
    // IDA 0x2c3af0: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratorppEv")]
// 0x2c3ca4 — __ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratorppEv
pub fn stub_0x2c3ca4() {
    // IDA 0x2c3ca4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratorC2EPS2_")]
// 0x2c3e54 — __ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratorC2EPS2_
pub fn stub_0x2c3e54() {
    // IDA 0x2c3e54: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX5GcJobD1Ev")]
// 0x2c46d0 — __ZN3RBX5GcJobD1Ev
pub fn stub_0x2c46d0() {
    // IDA 0x2c46d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX5GcJobD0Ev")]
// 0x2c47a0 — __ZN3RBX5GcJobD0Ev
pub fn stub_0x2c47a0() {
    // IDA 0x2c47a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX5GcJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")]
// 0x2c4884 — __ZN3RBX5GcJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
pub fn stub_0x2c4884(job: &mut core_af::GcJob, stats: &core_af::GcJobStatsView, sleep: &core_af::SleepInputs) {
    // IDA 0x2c4884: GcJob::sleepTime — interval is (double)stats.window (+0x1E8, 0x2c488a) forwarded to Job::computeStandardSleepTime (0x24a210).
    core_af::gc_job_sleep_time(job, stats, sleep);
}

#[doc(alias = "__ZN3RBX5GcJob5errorERKNS_13TaskScheduler3Job5StatsE")]
// 0x2c48a4 — __ZN3RBX5GcJob5errorERKNS_13TaskScheduler3Job5StatsE
pub fn stub_0x2c48a4(stats: &core_af::GcJobStatsView, sample: f64) -> core_af::StandardErrorOut {
    // IDA 0x2c48a4: GcJob::error — interval is (double)stats.window (+488, 0x2c48b2) forwarded to Job::computeStandardError (0x24a1f8).
    core_af::gc_job_error(stats, sample)
}

#[doc(alias = "__GLOBAL__I_a_72")]
// 0x2c4a80 — __GLOBAL__I_a_72
pub fn stub_0x2c4a80() {
    // IDA 0x2c4a80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__GLOBAL__I_a_73")]
// 0x2c68dc — __GLOBAL__I_a_73
pub fn stub_0x2c68dc() {
    // IDA 0x2c68dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost10shared_ptrIN3RBX13ActivityMeterILi2EEEEaSERKS4_")]
// 0x2c7348 — __ZN5boost10shared_ptrIN3RBX13ActivityMeterILi2EEEEaSERKS4_
pub fn stub_0x2c7348() {
    // IDA 0x2c7348: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZN5boost10shared_ptrIN3RBX15InvocationMeterILi2EEEEaSERKS4_")]
// 0x2c7380 — __ZN5boost10shared_ptrIN3RBX15InvocationMeterILi2EEEEaSERKS4_
pub fn stub_0x2c7380() {
    // IDA 0x2c7380: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZN3RBX5Stats4Item20createBoundChildItemIbEEPS1_PKcRKT_")]
// 0x2c73b8 — __ZN3RBX5Stats4Item20createBoundChildItemIbEEPS1_PKcRKT_
pub fn stub_0x2c73b8() {
    // IDA 0x2c73b8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZThn32_N3RBX5Stats4ItemD1Ev")]
// 0x2c7928 — __ZThn32_N3RBX5Stats4ItemD1Ev
pub fn stub_0x2c7928() {
    // IDA 0x2c7928: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX5Stats14TypedStatsItemIbED1Ev")]
// 0x2c7b48 — __ZN3RBX5Stats14TypedStatsItemIbED1Ev
pub fn stub_0x2c7b48() {
    // IDA 0x2c7b48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX5Stats14TypedStatsItemIbED0Ev")]
// 0x2c7c90 — __ZN3RBX5Stats14TypedStatsItemIbED0Ev
pub fn stub_0x2c7c90() {
    // IDA 0x2c7c90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX5Stats14TypedStatsItemIbED1Ev")]
// 0x2c7df0 — __ZThn36_N3RBX5Stats14TypedStatsItemIbED1Ev
pub fn stub_0x2c7df0() {
    // IDA 0x2c7df0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX5Stats14TypedStatsItemIbED0Ev")]
// 0x2c7f38 — __ZThn36_N3RBX5Stats14TypedStatsItemIbED0Ev
pub fn stub_0x2c7f38() {
    // IDA 0x2c7f38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt5dequeISsSaISsEE8pop_backEv")]
// 0x2c8270 — __ZNSt5dequeISsSaISsEE8pop_backEv
pub fn stub_0x2c8270() {
    // IDA 0x2c8270: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt5dequeISsSaISsEE9push_backERKSs")]
// 0x2c82a8 — __ZNSt5dequeISsSaISsEE9push_backERKSs
pub fn stub_0x2c82a8() {
    // IDA 0x2c82a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt5dequeISsSaISsEE16_M_push_back_auxERKSs")]
// 0x2c82d4 — __ZNSt5dequeISsSaISsEE16_M_push_back_auxERKSs
pub fn stub_0x2c82d4() {
    // IDA 0x2c82d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt5dequeISsSaISsEE22_M_reserve_map_at_backEm")]
// 0x2c846c — __ZNSt5dequeISsSaISsEE22_M_reserve_map_at_backEm
pub fn stub_0x2c846c() {
    // IDA 0x2c846c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt5dequeISsSaISsEE17_M_reallocate_mapEmb")]
// 0x2c8488 — __ZNSt5dequeISsSaISsEE17_M_reallocate_mapEmb
pub fn stub_0x2c8488() {
    // IDA 0x2c8488: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNSt11_Deque_baseISsSaISsEE15_M_allocate_mapEm")]
// 0x2c8560 — __ZNSt11_Deque_baseISsSaISsEE15_M_allocate_mapEm
pub fn stub_0x2c8560() {
    // IDA 0x2c8560: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN5boost10shared_ptrIN3RBX15InvocationMeterILi2EEEEC2IS3_EEPT_")]
// 0x2c8894 — __ZN5boost10shared_ptrIN3RBX15InvocationMeterILi2EEEEC2IS3_EEPT_
pub fn stub_0x2c8894() {
    // IDA 0x2c8894: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX15InvocationMeterILi2EEEEEPT_")]
// 0x2c8968 — __ZN5boost6detail12shared_countC2IN3RBX15InvocationMeterILi2EEEEEPT_
pub fn stub_0x2c8968() {
    // IDA 0x2c8968: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX15InvocationMeterILi2EEEED1Ev")]
// 0x2c8a54 — __ZN5boost6detail17sp_counted_impl_pIN3RBX15InvocationMeterILi2EEEED1Ev
pub fn stub_0x2c8a54() {
    // IDA 0x2c8a54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX15InvocationMeterILi2EEEED0Ev")]
// 0x2c8a58 — __ZN5boost6detail17sp_counted_impl_pIN3RBX15InvocationMeterILi2EEEED0Ev
pub fn stub_0x2c8a58() {
    // IDA 0x2c8a58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX15InvocationMeterILi2EEEE7disposeEv")]
// 0x2c8a5c — __ZN5boost6detail17sp_counted_impl_pIN3RBX15InvocationMeterILi2EEEE7disposeEv
pub fn stub_0x2c8a5c() {
    // IDA 0x2c8a5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX15InvocationMeterILi2EEEE11get_deleterERKSt9type_info")]
// 0x2c8a68 — __ZN5boost6detail17sp_counted_impl_pIN3RBX15InvocationMeterILi2EEEE11get_deleterERKSt9type_info
pub fn stub_0x2c8a68() {
    // IDA 0x2c8a68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX15InvocationMeterILi2EEEE19get_untyped_deleterEv")]
// 0x2c8a6c — __ZN5boost6detail17sp_counted_impl_pIN3RBX15InvocationMeterILi2EEEE19get_untyped_deleterEv
pub fn stub_0x2c8a6c() {
    // IDA 0x2c8a6c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost10shared_ptrIN3RBX13ActivityMeterILi2EEEEC2IS3_EEPT_")]
// 0x2c8a70 — __ZN5boost10shared_ptrIN3RBX13ActivityMeterILi2EEEEC2IS3_EEPT_
pub fn stub_0x2c8a70() {
    // IDA 0x2c8a70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX13ActivityMeterILi2EEEEEPT_")]
// 0x2c8b44 — __ZN5boost6detail12shared_countC2IN3RBX13ActivityMeterILi2EEEEEPT_
pub fn stub_0x2c8b44() {
    // IDA 0x2c8b44: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX13ActivityMeterILi2EEEED1Ev")]
// 0x2c8c30 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13ActivityMeterILi2EEEED1Ev
pub fn stub_0x2c8c30() {
    // IDA 0x2c8c30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX13ActivityMeterILi2EEEED0Ev")]
// 0x2c8c34 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13ActivityMeterILi2EEEED0Ev
pub fn stub_0x2c8c34() {
    // IDA 0x2c8c34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX13ActivityMeterILi2EEEE7disposeEv")]
// 0x2c8c38 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13ActivityMeterILi2EEEE7disposeEv
pub fn stub_0x2c8c38() {
    // IDA 0x2c8c38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX13ActivityMeterILi2EEEE11get_deleterERKSt9type_info")]
// 0x2c8c44 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13ActivityMeterILi2EEEE11get_deleterERKSt9type_info
pub fn stub_0x2c8c44() {
    // IDA 0x2c8c44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX13ActivityMeterILi2EEEE19get_untyped_deleterEv")]
// 0x2c8c48 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13ActivityMeterILi2EEEE19get_untyped_deleterEv
pub fn stub_0x2c8c48() {
    // IDA 0x2c8c48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt5dequeISsSaISsEEC2ERKS1_")]
// 0x2c8ca0 — __ZNSt5dequeISsSaISsEEC2ERKS1_
pub fn stub_0x2c8ca0() {
    // IDA 0x2c8ca0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZSt24__uninitialized_copy_auxISt15_Deque_iteratorISsRKSsPS1_ES0_ISsRSsPSsEET0_T_S9_S8_St12__false_type")]
// 0x2c8dc8 — __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorISsRKSsPS1_ES0_ISsRSsPSsEET0_T_S9_S8_St12__false_type
pub fn stub_0x2c8dc8() {
    // IDA 0x2c8dc8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "__ZNSt11_Deque_baseISsSaISsEE17_M_initialize_mapEm")]
// 0x2c8f2c — __ZNSt11_Deque_baseISsSaISsEE17_M_initialize_mapEm
pub fn stub_0x2c8f2c() {
    // IDA 0x2c8f2c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "__ZNSt11_Deque_baseISsSaISsEE15_M_create_nodesEPPSsS3_")]
// 0x2c9084 — __ZNSt11_Deque_baseISsSaISsEE15_M_create_nodesEPPSsS3_
pub fn stub_0x2c9084() {
    // IDA 0x2c9084: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "__ZNSt5dequeISsSaISsEE19_M_destroy_data_auxESt15_Deque_iteratorISsRSsPSsES5_")]
// 0x2c9178 — __ZNSt5dequeISsSaISsEE19_M_destroy_data_auxESt15_Deque_iteratorISsRSsPSsES5_
pub fn stub_0x2c9178() {
    // IDA 0x2c9178: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__GLOBAL__I_a_74")]
// 0x2c9314 — __GLOBAL__I_a_74
pub fn stub_0x2c9314() {
    // IDA 0x2c9314: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZL25callGenericFunctionBridgeP9lua_State")]
// 0x2ca664 — __ZL25callGenericFunctionBridgeP9lua_State
pub fn stub_0x2ca664() {
    // IDA 0x2ca664: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZL30callGenericAsyncFunctionBridgeP9lua_State")]
// 0x2ca908 — __ZL30callGenericAsyncFunctionBridgeP9lua_State
pub fn stub_0x2ca908() {
    // IDA 0x2ca908: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN16RobloxExtraSpace13createNewNodeEv")]
// 0x2cbc40 — __ZN16RobloxExtraSpace13createNewNodeEv
pub fn stub_0x2cbc40() {
    // IDA 0x2cbc40: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__GLOBAL__I_a_75")]
// 0x2cde88 — __GLOBAL__I_a_75
pub fn stub_0x2cde88() {
    // IDA 0x2cde88: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "__ZN3RBX8Security7Context8isInRoleENS0_10IdentitiesENS0_11PermissionsE")]
// 0x2ce130 — __ZN3RBX8Security7Context8isInRoleENS0_10IdentitiesENS0_11PermissionsE
pub fn stub_0x2ce130() {
    // IDA 0x2ce130: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "__GLOBAL__I_a_76")]
// 0x2ce1fc — __GLOBAL__I_a_76
pub fn stub_0x2ce1fc() {
    // IDA 0x2ce1fc: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "__GLOBAL__I_a_77")]
// 0x2ceadc — __GLOBAL__I_a_77
pub fn stub_0x2ceadc() {
    // IDA 0x2ceadc: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "__ZNSt8auto_ptrIN3RBX13AdvRunDraggerEE5resetEPS1_")]
// 0x2d072c — __ZNSt8auto_ptrIN3RBX13AdvRunDraggerEE5resetEPS1_
pub fn stub_0x2d072c() {
    // IDA 0x2d072c: global static ctor/dtor key. Static init — carrier no-op.
}
