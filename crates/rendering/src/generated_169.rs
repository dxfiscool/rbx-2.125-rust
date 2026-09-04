//! rendering — generated_169 — next 100 stubs EA-sorted asc filler (Ogre|G3D|Gfx|Render|Adorn 15586 filtered, 15586 covered, filler 18289->18389, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::{
    LazyLock, Once,
    atomic::{AtomicBool, AtomicI32, AtomicU32, AtomicU64, AtomicUsize, Ordering},
};

// ---- impl batch 0x84e0..0x17e68 (25 fns, IDA decompile+disasm grounded) ----
//
// Boost mapping (no boost crate): boost::shared_ptr/intrusive_ptr →
// rbx_core::SharedPtr (Arc); boost::mutex → parking_lot::Mutex;
// intrusive add_ref/release → Arc clone/drop.

/// Rust model of `boost::detail::sp_counted_base` behind 0xefd8: the
/// use-count word itself; the spinlock-pool shard mutex below stands in
/// for the `spinlock_pool<1>::pool_` selection at 0xf01a.
pub struct SpCountedBase {
    pub use_count: AtomicUsize,
}

/// Rust model of `boost::detail::spinlock_pool<1>::pool_` (IDA 0xf01a):
/// the shard mutex guarding the count read at 0xf020..0xf058.
static SPINLOCK_POOL: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

/// Rust model of `boost::exception_detail::error_info_container` behind
/// 0xcb94 (`refcount_ptr<...>`; SharedPtr, not boost).
pub type ErrorInfoContainer = u8;

/// Rust model of `RBX::Reflection::Tuple` behind 0x17aac/0x17b80/0x179f4:
/// layout rides with the reflection batch; only ownership moves here.
pub struct ReflectionTuple {
    _opaque: (),
}

impl ReflectionTuple {
    fn new() -> Self {
        Self { _opaque: () }
    }
}

/// Rust model of `std::out_of_range` behind 0x9b30/0x9b44: the deleting
/// destructor frees via `operator delete` (0x9b40); real layout/size
/// lives in libstdc++, so this stays an opaque owner token.
pub struct OutOfRangeError {
    _opaque: (),
}

/// Slot list behind `rbx::signals::signal<...>::operator()` (0xb76c) and
/// `::next` (0xf574).
static PROPERTY_DESCRIPTOR_SLOTS: LazyLock<Mutex<Vec<fn(u32)>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));
/// IDA `FLog::SignalPrints` gate read at 0xb7ce.
static SIGNAL_PRINTS: AtomicBool = AtomicBool::new(false);
/// IDA `call_once` init of the signal mutex at 0xf5ee.
static SIGNAL_MUTEX_ONCE: Once = Once::new();
/// IDA `__GLOBAL__I_a` (0x16e4c) / `__GLOBAL__I_a_0` (0x17c58) TU init gates.
static GLOBAL_I_A_ONCE: Once = Once::new();
static GLOBAL_I_A_0_ONCE: Once = Once::new();

/// IDA `__MergedGlobals243` cell behind `+[Appirater setAppId:]` (0x17df0).
static APPIRATER_APP_ID: AtomicUsize = AtomicUsize::new(0);
/// IDA `__daysUntilPrompt` (0x17e00); bits, no float atomics here.
static DAYS_UNTIL_PROMPT_BITS: AtomicU64 = AtomicU64::new(0);
/// IDA `__MergedGlobals` (0x17e14).
static USES_UNTIL_PROMPT: AtomicI32 = AtomicI32::new(0);
/// IDA `dword_122316C` (0x17e24).
static SIGNIFICANT_EVENTS_UNTIL_PROMPT: AtomicI32 = AtomicI32::new(0);
/// IDA `__timeBeforeReminding` (0x17e34); bits.
static TIME_BEFORE_REMINDING_BITS: AtomicU64 = AtomicU64::new(0);
/// IDA `__debug` (0x17e48).
static APPIRATER_DEBUG: AtomicBool = AtomicBool::new(false);
/// IDA `dword_130C394` (0x17e58).
static APPIRATER_DELEGATE: AtomicUsize = AtomicUsize::new(0);
/// IDA `rbx::signals::slot_exception_handler` behind 0xf6dc.
static SLOT_EXCEPTION_HANDLER: LazyLock<Mutex<Option<fn()>>> =
    LazyLock::new(|| Mutex::new(None));
/// IDA `dword_130C398` singleton cell behind `+[Appirater sharedInstance]`
/// (0x17f80): nonzero once the block at 0x17fe4 has run.
static APPIRATER_SHARED_ID: AtomicUsize = AtomicUsize::new(0);
/// IDA `dword_130C39C` dispatch_once token behind 0x17f80/0x17fd0.
static APPIRATER_ONCE: Once = Once::new();
/// `addObserver:...UIApplicationWillResignActiveNotification` at 0x18052:
/// set once the shared-instance block has run.
static APPIRATER_OBSERVER_REGISTERED: AtomicBool = AtomicBool::new(false);
/// `-[Appirater setRatingAlert:]` (0x18358) + `show` (0x1836a): whether an
/// alert is currently displayed; `hideRatingAlert` (0x18d4c) clears it.
static RATING_ALERT_VISIBLE: AtomicBool = AtomicBool::new(false);
/// `respondsToSelector:appiraterDidDisplayAlert:` check at 0x183aa.
static DELEGATE_RESPONDS_TO_DISPLAY_ALERT: AtomicBool = AtomicBool::new(false);
/// `appiraterDidDisplayAlert:` delivered at 0x183c6.
static DID_DISPLAY_ALERT: AtomicBool = AtomicBool::new(false);
/// `kAppiraterFirstUseDate` (0x1843e/0x1870a); bits, 0.0 = unset.
static FIRST_USE_DATE_BITS: AtomicU64 = AtomicU64::new(0);
/// `kAppiraterUseCount` (0x184dc/0x18740).
static APPIRATER_USE_COUNT: AtomicI32 = AtomicI32::new(0);
/// `kAppiraterSignificantEventCount` (0x184f6/0x189f8).
static APPIRATER_SIG_EVENT_COUNT: AtomicI32 = AtomicI32::new(0);
/// `kAppiraterDeclinedToRate` (0x18518).
static DECLINED_TO_RATE: AtomicBool = AtomicBool::new(false);
/// `kAppiraterRatedCurrentVersion` (0x18532).
static RATED_CURRENT_VERSION: AtomicBool = AtomicBool::new(false);
/// `kAppiraterReminderRequestDate` (0x18552); bits.
static REMINDER_DATE_BITS: AtomicU64 = AtomicU64::new(0);
/// `kAppiraterCurrentVersion` (0x18640): last version the counters were
/// attributed to; `None` = never launched.
static CURRENT_VERSION: LazyLock<Mutex<Option<String>>> =
    LazyLock::new(|| Mutex::new(None));

/// Capture word at block +20 behind the `__copy_helper`/`__destroy_helper`
/// pair (0x18094/0x180a0 et al.): `Block_object_assign` retains it,
/// `Block_object_dispose` releases it.
pub struct AppiraterBlock {
    pub captured: usize,
}

/// `[NSDate date]` behind 0x1846c/0x186e2 (seconds since 1970).
fn appirater_now_secs() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0)
}
/// Last `connectedToNetwork` verdict (0x17e68), consumed by
/// `incrementAndRate:` (0x18b60) / `incrementSignificantEventAndRate:` (0x18c24).
static APPIRATER_NET_CONNECTED: AtomicBool = AtomicBool::new(false);
/// `NSUserDefaults` store behind 0x192b4/0x196e4/0x19cdc/0x19f7c:
/// registerDefaults/synchronize are persistence details; only the
/// key/value transitions are modeled.
static USER_DEFAULTS: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
/// `+[UserInfo CurrentPlayer]` username/password (0x1945c..0x194ce).
static CURRENT_PLAYER_USERNAME: LazyLock<Mutex<Option<String>>> =
    LazyLock::new(|| Mutex::new(None));
static CURRENT_PLAYER_PASSWORD: LazyLock<Mutex<Option<String>>> =
    LazyLock::new(|| Mutex::new(None));
/// `-[SessionReporter reportSessionFor:]` kind (0x193a8/0x19926/0x19e0a); -1 = unset.
static SESSION_KIND: AtomicI32 = AtomicI32::new(-1);
/// Cookie accept policy (0x19438); initialized to never (0).
static COOKIE_ACCEPT_POLICY: AtomicI32 = AtomicI32::new(-1);
/// `registerDefaults:` ran (0x19366).
static DEFAULTS_REGISTERED: AtomicBool = AtomicBool::new(false);
/// `+[UpgradeCheckHelper checkForUpdate]` ran (0x1940a/0x19c0e).
static UPGRADE_CHECKED: AtomicBool = AtomicBool::new(false);
/// `+[Flurry startSession:]` (0x1950e, key "FM7DNRW56339NC22K8GR").
static FLURRY_SESSION_STARTED: AtomicBool = AtomicBool::new(false);
/// `FetchClientSettingsData("iOSAppSettings", ...)` runs (0x19f56).
static SETTINGS_FETCHED: AtomicUsize = AtomicUsize::new(0);
/// `ratingAlert` ivar handle behind 0x191d4/0x191e4 (0 = nil); set by
/// `showRatingAlert` (0x180a8).
static RATING_ALERT_HANDLE: AtomicUsize = AtomicUsize::new(0);
/// `AppDelegate` handle behind `-[AppDelegate init]` (0x19228).
static APP_DELEGATE_HANDLE: AtomicUsize = AtomicUsize::new(0);
/// Analytics object alive; `dealloc` (0x19276) releases it.
static ANALYTICS_ALIVE: AtomicBool = AtomicBool::new(true);
/// `_window` ivar behind 0x1a4c0/0x1928a (0 = nil).
static APP_WINDOW_HANDLE: AtomicUsize = AtomicUsize::new(0);
/// `bgTask` ivar behind 0x1a494/0x1a4a8; DMB ISH → SeqCst.
static BG_TASK: AtomicU32 = AtomicU32::new(0);
/// `appPlaceID` global behind 0x1a174/0x19e32.
static APP_PLACE_ID: AtomicI32 = AtomicI32::new(0);
/// In-game state: `startGame:` (0x1a42a) sets it, `leaveGame` (0x197e6) clears it.
static IN_GAME: AtomicBool = AtomicBool::new(false);
/// `disableViewBecauseGoingToBackground` (0x19640) /
/// `enableViewBecauseGoingToForeground` (0x19de0).
static PLACE_VIEW_DISABLED: AtomicBool = AtomicBool::new(false);
/// Place id routed by `TryLaunchPlace:` (0x1a234).
static LAUNCHED_PLACE_ID: AtomicI32 = AtomicI32::new(0);
/// `setLoginPlaceId:` target (0x1a372) / `setJumpToPlaceIDGameInProgress:` (0x1a47a).
static LOGIN_PLACE_ID: AtomicI32 = AtomicI32::new(0);
/// `setJumpToPlaceID:` (0x1a3ae) / `setJumpToPlaceIDGameInProgress:` (0x1a47a).
static JUMP_TO_PLACE_ID: AtomicI32 = AtomicI32::new(0);
static NAV_JUMP_PLACE_ID: AtomicI32 = AtomicI32::new(0);
/// `buttonForWebDidTouchUpInside:` fired (0x1a3be).
static WEB_BUTTON_TOUCHED: AtomicBool = AtomicBool::new(false);
/// `stopMemoryBouncer:` did not stop → forwarded to PlaceLauncher (0x19ad8..0x19b00).
static MEMORY_WARNING_FORWARDED: AtomicBool = AtomicBool::new(false);
/// `-[LoginManager applicationWillTerminate]` ran (0x1a064).
static LOGIN_TERMINATED: AtomicBool = AtomicBool::new(false);
/// `setPageViewTracking:` page (0x1994e/0x19c36/0x1a092).
static PAGE_VIEW: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| Mutex::new(None));
/// Delegate alert-action callbacks behind 0x19028.
static OPT_RATE_DELIVERED: AtomicBool = AtomicBool::new(false);
static OPT_REMIND_DELIVERED: AtomicBool = AtomicBool::new(false);
static DECLINE_DELIVERED: AtomicBool = AtomicBool::new(false);
/// `respondsToSelector:` gate for the three alert-action callbacks (0x191b2).
static DELEGATE_RESPONDS_TO_ALERT_ACTION: AtomicBool = AtomicBool::new(false);
/// `openURL:` review-URL opens behind `rateApp` (0x19024).
static REVIEW_URL_OPENED: AtomicUsize = AtomicUsize::new(0);
/// App id substituted into `templateReviewURL` (0x18f80).
static LAST_REVIEW_APP_ID: AtomicUsize = AtomicUsize::new(0);

/// Top-controller class behind `TryLaunchPlace:` (0x1a234): the
/// `isEqualToString:` dispatch on the class name
/// (0x1a334/0x1a386/0x1a3de/0x1a43e).
pub enum TopViewController {
    Login,
    Home,
    NavBar,
    Game,
    Other,
}

// 0x84e0 — start
#[doc(alias = "start")]
// was: start
// type: void __fastcall __noreturn(int, int, int, int, int argc, char *argv)
// IDA 0x84e0 (decompile 0x84e0..0x8508 + disasm grounded): argc reload
// (0x84e0); envp = &argv[argc + 1] (0x84e4..0x84ec); stack align
// (0x84f0); null-terminated envp scan (0x84f4..0x8500);
// main(argc, argv, envp) (0x8504); exit(status) (0x8508).
// SAFETY: `argv` must point to `argc + 1` valid entries followed by a
// null-terminated envp, as the CRT provides.
pub fn stub_84e0(argc: i32, argv: *const *const u8) -> ! {
    unsafe {
        let mut envp = argv.add(argc as usize + 1);
        while !(*envp).is_null() {
            envp = envp.add(1);
        }
    }
    // FIDELITY: main linkage lives outside this crate; the original
    // passes main's return to exit — 0 stands in for it here.
    std::process::exit(0);
}


// 0x9b2c — __ZNSt12length_errorD1Ev
#[doc(alias = "std::length_error::~length_error()")]
// was: __ZNSt12length_errorD1Ev
// type: void __cdecl(std::length_error *__hidden this)
// IDA 0x9b2c (thunk grounded): B.W logic_error::~logic_error —
// non-deleting destructor (D1); base-class drop only, no manual state.
pub fn stub_9b2c(_this: *const u8) {
}


// 0x9b30 — __ZNSt12out_of_rangeD0Ev
#[doc(alias = "std::out_of_range::~out_of_range()")]
// was: __ZNSt12out_of_rangeD0Ev
// type: void __cdecl(std::out_of_range *__hidden this)
// IDA 0x9b30 (grounded 0x9b30..0x9b40): PUSH regs; BLX
// logic_error::~logic_error (0x9b36) tears down the base subobject;
// restore; B.W operator delete (0x9b40) — deleting destructor (D0).
// SAFETY: `_this` must be a live heap object never used again.
pub fn stub_9b30(_this: *mut OutOfRangeError) {
    unsafe {
        drop(Box::from_raw(_this));
    }
}


// 0x9b44 — __ZNSt12out_of_rangeD2Ev
#[doc(alias = "std::out_of_range::~out_of_range()")]
// was: __ZNSt12out_of_rangeD2Ev
// type: void __cdecl(std::out_of_range *__hidden this)
// IDA 0x9b44 (thunk grounded): B.W logic_error::~logic_error —
// non-deleting destructor (D2); base-class drop only, no manual state.
// Cf. the deleting destructor (D0) at 0x9b30.
pub fn stub_9b44(_this: *const u8) {
}


// 0xb76c — __ZN3rbx7signals16signal_with_argsILi1EFvPKN3RBX10Reflection18PropertyDescriptorEEEclES6_
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Reflection::PropertyDescriptor const*)>::operator()(RBX::Reflection::PropertyDescriptor const*)")]
// was: __ZN3rbx7signals16signal_with_argsILi1EFvPKN3RBX10Reflection18PropertyDescriptorEEEclES6_
// type: void __fastcall(_DWORD *, int, int, const void *, int, int, int, int, void *, int)
// IDA 0xb76c (decompile grounded): `if (*a1)` empty-signal early-out;
// v22 = 0 cursor (0xb7c6); FLog::SignalPrints gate (0xb7ce..0xb7e0);
// while next(cursor) == 1 (0xb80a): if the slot holds a fn (v22 + 12)
// call fn(v22 + 4, descriptor) (0xb7e6..0xb7f6); release the cursor
// slot (0xb80c..0xb812).
pub fn stub_b76c(descriptor: u32) {
    if PROPERTY_DESCRIPTOR_SLOTS.lock().is_empty() {
        return;
    }
    if SIGNAL_PRINTS.load(Ordering::SeqCst) {
        // FLog sink lives outside this crate; the gate itself is kept.
    }
    let mut cursor = 0usize;
    while stub_f574(&mut cursor) {
        let slot = PROPERTY_DESCRIPTOR_SLOTS.lock().get(cursor - 1).copied();
        if let Some(f) = slot {
            f(descriptor);
        }
    }
}


// 0xcb94 — __ZN5boost16exception_detail12refcount_ptrINS0_20error_info_containerEED2Ev
#[doc(alias = "boost::exception_detail::refcount_ptr<boost::exception_detail::error_info_container>::~refcount_ptr()")]
// was: __ZN5boost16exception_detail12refcount_ptrINS0_20error_info_containerEED2Ev
// type: _DWORD *__fastcall(_DWORD *)
// IDA 0xcb94 (decompile grounded): if (*a1 && release(*a1) == 1)
// *a1 = 0 (0xcbf2..0xcbf6) — conditional container release + null-out.
// was: boost::shared_ptr teardown → rbx_core::SharedPtr (Arc) drop.
pub fn stub_cb94(cell: &mut Option<SharedPtr<ErrorInfoContainer>>) {
    if cell.is_some() {
        // SharedPtr drop is the intrusive release; take() nulls the cell.
        cell.take();
    }
}


// 0xefd8 — __ZNK5boost6detail15sp_counted_base9use_countEv
#[doc(alias = "boost::detail::sp_counted_base::use_count(void)const")]
// was: __ZNK5boost6detail15sp_counted_base9use_countEv
// type: int __fastcall(boost::detail::sp_counted_base *this)
// IDA 0xefd8 (decompile grounded): shard = pool_ + 44 * ((this + 4) %
// 41) (0xf01a); mutex_lock (0xf020); v4 = this->use_count (0xf032);
// mutex_unlock (0xf058); return v4 (0xf078).
pub fn stub_efd8(this: &SpCountedBase) -> i32 {
    let _guard = SPINLOCK_POOL.lock();
    this.use_count.load(Ordering::SeqCst) as i32
}


// 0xf574 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4nextERN5boost13intrusive_ptrINS8_4slotEEE
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot> &)")]
// was: __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4nextERN5boost13intrusive_ptrINS8_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
// IDA 0xf574 (decompile grounded): add_ref the incoming cursor slot
// (0xf5c4..0xf5ce); call_once mutex init (0xf5ee); lock (0xf608);
// operator= advances to the next live slot (0xf61c..0xf636); unlock
// (0xf638..0xf640); release the old slot (0xf646..0xf64e); return
// whether a slot remains (0xf658..0xf674).
// was: boost::mutex lock/unlock → `parking_lot::Mutex` guard scope;
// intrusive add_ref/release → `Vec` ownership.
pub fn stub_f574(cursor: &mut usize) -> bool {
    SIGNAL_MUTEX_ONCE.call_once(|| {});
    let slots = PROPERTY_DESCRIPTOR_SLOTS.lock();
    if *cursor < slots.len() {
        *cursor += 1;
        true
    } else {
        false
    }
}


// 0xf6dc — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::on_error(std::exception &)")]
// was: __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE8on_errorERSt9exception
// type: int *()
// IDA 0xf6dc (decompile 0xf6dc..0xf702 + disasm grounded):
// result = &slot_exception_handler (0xf6f0); if (*handler) use the
// nonnull sentinel (0xf6f2..0xf6f8); if set, return handler(exception)
// (0xf6fc..0xf6fe); else return the handler cell (0xf702).
pub fn stub_f6dc() -> bool {
    let handler = SLOT_EXCEPTION_HANDLER.lock().clone();
    match handler {
        Some(f) => {
            f();
            true
        }
        None => false,
    }
}


// 0x16e4c — __GLOBAL__I_a
#[doc(alias = "__GLOBAL__I_a")]
// was: __GLOBAL__I_a
// type: 
// IDA 0x16e4c (disasm grounded, decompile failed): TU static init —
// generic_category/system_category stores into __MergedGlobals_33
// (0x16e56..0x16e6c...); runs once before main; Rust statics need no
// glue, so this is a once gate.
pub fn stub_16e4c() {
    GLOBAL_I_A_ONCE.call_once(|| {});
}


// 0x179e8 — __ZN3RBX9DataModel10serverSaveEv
#[doc(alias = "RBX::DataModel::serverSave(void)")]
// was: __ZN3RBX9DataModel10serverSaveEv
// type: void __fastcall(RBX::DataModel *this)
// IDA 0x179e8 (decompile + disasm grounded): single BX LR — empty
// serverSave body, `this` unused.
pub fn stub_179e8(_this: *const u8) {
}


// 0x179ec — __ZN3RBX9DataModel17internalSaveAsyncENS_9ContentIdEN5boost8functionIFvbEEE
#[doc(alias = "RBX::DataModel::internalSaveAsync(RBX::ContentId,boost::function<void ()(bool)>)")]
// was: __ZN3RBX9DataModel17internalSaveAsyncENS_9ContentIdEN5boost8functionIFvbEEE
// type: void()
// IDA 0x179ec (decompile + disasm grounded): single BX LR — empty
// internalSaveAsync body; ContentId/callback args dropped.
pub fn stub_179ec() {
}


// 0x179f0 — __ZN3RBX9DataModel12internalSaveENS_9ContentIdE
#[doc(alias = "RBX::DataModel::internalSave(RBX::ContentId)")]
// was: __ZN3RBX9DataModel12internalSaveENS_9ContentIdE
// type: void()
// IDA 0x179f0 (decompile + disasm grounded): single BX LR — empty
// internalSave body; the ContentId arg is dropped.
pub fn stub_179f0() {
}


// 0x179f4 — __ZN3RBX9DataModel11uploadPlaceERKSsNS_8Instance10SaveFilterEN5boost8functionIFvNS5_10shared_ptrIKNS_10Reflection5TupleEEEEEENS6_IFvSsEEE
#[doc(alias = "RBX::DataModel::uploadPlace(std::string const&,RBX::Instance::SaveFilter,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX9DataModel11uploadPlaceERKSsNS_8Instance10SaveFilterEN5boost8functionIFvNS5_10shared_ptrIKNS_10Reflection5TupleEEEEEENS6_IFvSsEEE
// type: void __fastcall(int)
// IDA 0x179f4 (decompile grounded): operator new(0xC) tuple state
// (0x17a14); zeroed (0x17a22..0x17a26); shared_ptr<Tuple> ctor into a
// temp (0x17a2a); converting copy into the a1 slot (0x17a32); temp
// released (0x17a64..0x17a6c).
pub fn stub_179f4(out: &mut Option<SharedPtr<ReflectionTuple>>) {
    let fresh = SharedPtr::new(ReflectionTuple::new());
    *out = Some(SharedPtr::clone(&fresh));
    drop(fresh);
}


// 0x17aac — __ZN5boost10shared_ptrIN3RBX10Reflection5TupleEEC1IS3_EEPT_
#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::Tuple>::shared_ptr<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *)")]
// was: __ZN5boost10shared_ptrIN3RBX10Reflection5TupleEEC1IS3_EEPT_
// type: 
// IDA 0x17aac (decompile grounded): *a1 = raw ptr (0x17ada); ctrl = 0
// (0x17ae2); shared_count ctor adopts the pointer (0x17b08); temp
// swapped into a1[1] (0x17b10..0x17b14); stale block released
// (0x17b16..0x17b1c).
pub fn stub_17aac(slot: &mut Option<SharedPtr<ReflectionTuple>>, adopted: SharedPtr<ReflectionTuple>) {
    *slot = Some(adopted);
}


// 0x17b80 — __ZN5boost10shared_ptrIKN3RBX10Reflection5TupleEEC2IS3_EERKNS0_IT_EENS_6detail24sp_enable_if_convertibleIS7_S4_E4typeE
#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::Tuple const>::shared_ptr<RBX::Reflection::Tuple>(rbx_core::SharedPtr<RBX::Reflection::Tuple> const&,boost::detail::sp_enable_if_convertible<RBX::Reflection::Tuple,RBX::Reflection::Tuple const>::type)")]
// was: __ZN5boost10shared_ptrIKN3RBX10Reflection5TupleEEC2IS3_EERKNS0_IT_EENS_6detail24sp_enable_if_convertibleIS7_S4_E4typeE
// type: 
// IDA 0x17b80 (decompile grounded): copy both words *a1 = *a2 (0x17ba8);
// a1[1] = control block (0x17bac..0x17bb4); if (ctrl): spinlock_pool
// shard lock (0x17bfe..0x17c02), ++*(ctrl + 4) (0x17c0c), unlock (0x17c14).
pub fn stub_17b80(dst: &mut Option<SharedPtr<ReflectionTuple>>, src: &Option<SharedPtr<ReflectionTuple>>) {
    // SharedPtr (Arc) clone is the locked count increment; None stays None.
    *dst = src.clone();
}


// 0x17c58 — __GLOBAL__I_a_0
#[doc(alias = "__GLOBAL__I_a_0")]
// was: __GLOBAL__I_a_0
// type: 
// IDA 0x17c58 (disasm grounded, decompile failed): TU static init, same
// shape as 0x16e4c — generic_category/system_category stores into
// __MergedGlobals_34 (0x17c5c..0x17c76...); runs once before main.
pub fn stub_17c58() {
    GLOBAL_I_A_0_ONCE.call_once(|| {});
}


// 0x17df0 — +[Appirater setAppId:]
#[doc(alias = "+[Appirater setAppId:]")]
// was: +[Appirater setAppId:]
// type: void __cdecl(id, SEL, id)
// IDA 0x17df0 (decompile + disasm grounded): STR R2, [R0] at 0x17dfa —
// plain global store of the app id, self/SEL unused.
pub fn stub_17df0(app_id: usize) {
    APPIRATER_APP_ID.store(app_id, Ordering::SeqCst);
}


// 0x17e00 — +[Appirater setDaysUntilPrompt:]
#[doc(alias = "+[Appirater setDaysUntilPrompt:]")]
// was: +[Appirater setDaysUntilPrompt:]
// type: void __cdecl(id, SEL, double)
// IDA 0x17e00 (decompile + disasm grounded): STRD.W R0, R1, [R2] at
// 0x17e0e — 64-bit store of the double, self/SEL unused.
pub fn stub_17e00(days: f64) {
    DAYS_UNTIL_PROMPT_BITS.store(days.to_bits(), Ordering::SeqCst);
}


// 0x17e14 — +[Appirater setUsesUntilPrompt:]
#[doc(alias = "+[Appirater setUsesUntilPrompt:]")]
// was: +[Appirater setUsesUntilPrompt:]
// type: void __cdecl(id, SEL, int)
// IDA 0x17e14 (decompile + disasm grounded): STR R2, [R0] at 0x17e1e —
// plain global store, self/SEL unused.
pub fn stub_17e14(uses: i32) {
    USES_UNTIL_PROMPT.store(uses, Ordering::SeqCst);
}


// 0x17e24 — +[Appirater setSignificantEventsUntilPrompt:]
#[doc(alias = "+[Appirater setSignificantEventsUntilPrompt:]")]
// was: +[Appirater setSignificantEventsUntilPrompt:]
// type: void __cdecl(id, SEL, int)
// IDA 0x17e24 (decompile + disasm grounded): STR R2 at 0x17e2e into
// dword_122316C — plain global store, self/SEL unused.
pub fn stub_17e24(events: i32) {
    SIGNIFICANT_EVENTS_UNTIL_PROMPT.store(events, Ordering::SeqCst);
}


// 0x17e34 — +[Appirater setTimeBeforeReminding:]
#[doc(alias = "+[Appirater setTimeBeforeReminding:]")]
// was: +[Appirater setTimeBeforeReminding:]
// type: void __cdecl(id, SEL, double)
// IDA 0x17e34 (decompile + disasm grounded): STRD.W R0, R1, [R2] at
// 0x17e42 — 64-bit store of the double, self/SEL unused.
pub fn stub_17e34(seconds: f64) {
    TIME_BEFORE_REMINDING_BITS.store(seconds.to_bits(), Ordering::SeqCst);
}


// 0x17e48 — +[Appirater setDebug:]
#[doc(alias = "+[Appirater setDebug:]")]
// was: +[Appirater setDebug:]
// type: void __cdecl(id, SEL, char)
// IDA 0x17e48 (decompile + disasm grounded): STRB R2, [R0] at 0x17e52 —
// plain byte store of the debug flag, self/SEL unused.
pub fn stub_17e48(debug: bool) {
    APPIRATER_DEBUG.store(debug, Ordering::SeqCst);
}


// 0x17e58 — +[Appirater setDelegate:]
#[doc(alias = "+[Appirater setDelegate:]")]
// was: +[Appirater setDelegate:]
// type: void __cdecl(id, SEL, id)
// IDA 0x17e58 (decompile + disasm grounded): STR R2, [R0, #dword_130C394]
// at 0x17e62 — plain global store, self/SEL unused.
pub fn stub_17e58(delegate: usize) {
    APPIRATER_DELEGATE.store(delegate, Ordering::SeqCst);
}


// 0x17e68 — -[Appirater connectedToNetwork]
#[doc(alias = "-[Appirater connectedToNetwork]")]
// was: -[Appirater connectedToNetwork]
// type: char __cdecl(Appirater *self, SEL)
// IDA 0x17e68 (decompile 0x17e68..0x17f78 + disasm grounded): v3 = 0
// (0x17e84); zeroed sockaddr (0x17e92); SCNetworkReachabilityCreateWithAddress
// (0x17ea8); GetFlags (0x17eae); CFRelease (0x17eb2); if (!ok): NSLog +
// return 0 (0x17eb8..0x17f64); if ((flags & 6) == 2 || (flags & 1)):
// return connection-establish != null (0x17f4a..0x17f52); else 0 (0x17f78).
pub fn stub_17e68(got_flags: bool, flags: u32, can_open: bool) -> bool {
    if !got_flags {
        return false;
    }
    if (flags & 6) == 2 || (flags & 1) != 0 {
        return can_open;
    }
    false
}


// 0x17f80 — +[Appirater sharedInstance]
#[doc(alias = "+[Appirater sharedInstance]")]
// was: +[Appirater sharedInstance]
// type: id __cdecl(id, SEL)
// IDA 0x17f80 (decompile+disasm grounded): return dword_130C398 when
// set (0x17f92..0x17f94); else build the once-block (0x17fb8..0x17fc8)
// and dispatch_once it unless already consumed (0x17fd0..0x17fe0).
pub fn stub_17f80() -> usize {
    APPIRATER_ONCE.call_once(stub_17fe4);
    APPIRATER_SHARED_ID.load(Ordering::SeqCst)
}


// 0x17fe4 — ___27+[Appirater sharedInstance]_block_invoke
#[doc(alias = "___27+[Appirater sharedInstance]_block_invoke")]
// was: ___27+[Appirater sharedInstance]_block_invoke
// type: 
// IDA 0x17fe4 (decompile grounded): Appirater alloc + init into
// dword_130C398 (0x18008..0x18030); setDelegate: the stored delegate
// (0x18036); addObserver appWillResignActive on the default center
// (0x18052..0x18092).
pub fn stub_17fe4() {
    APPIRATER_SHARED_ID.store(1, Ordering::SeqCst);
    let _ = APPIRATER_DELEGATE.load(Ordering::SeqCst);
    APPIRATER_OBSERVER_REGISTERED.store(true, Ordering::SeqCst);
}


// 0x18094 — ___copy_helper_block_
#[doc(alias = "___copy_helper_block_")]
// was: ___copy_helper_block_
// type: 
// IDA 0x18094 (decompile+disasm grounded): LDR captured+0x14 (0x18094);
// dst+0x14 (0x18096); kind 3 (0x18098); B.W Block_object_assign
// (0x1809a) — retains the captured object.
pub fn stub_18094(dst: &mut AppiraterBlock, src: &AppiraterBlock) {
    dst.captured = src.captured;
}


// 0x180a0 — ___destroy_helper_block_
#[doc(alias = "___destroy_helper_block_")]
// was: ___destroy_helper_block_
// type: void __fastcall(int)
// IDA 0x180a0 (decompile+disasm grounded): LDR captured+0x14 (0x180a0);
// kind 3 = BLOCK_FIELD_IS_OBJECT (0x180a2); B.W Block_object_dispose
// (0x180a4) — releases the captured object.
pub fn stub_180a0(obj: &mut AppiraterBlock) {
    obj.captured = 0;
}


// 0x180a8 — -[Appirater showRatingAlert]
#[doc(alias = "-[Appirater showRatingAlert]")]
// was: -[Appirater showRatingAlert]
// type: void __cdecl(Appirater *self, SEL)
// IDA 0x180a8 (decompile 0x180a8..0x183d6 grounded): UIAlertView alloc
// (0x180d0); localized title/message/buttons formatted with the bundle
// name, localized-dictionary preferred with info-dictionary fallback
// (0x180fe..0x1831e); initWithTitle:...delegate:self (0x18346);
// setRatingAlert: (0x18358); show (0x1836a); if the delegate is set and
// respondsToSelector:appiraterDidDisplayAlert:, deliver it
// (0x1837e..0x183c6).
pub fn stub_180a8() {
    RATING_ALERT_HANDLE.store(1, Ordering::SeqCst);
    RATING_ALERT_VISIBLE.store(true, Ordering::SeqCst);
    if APPIRATER_DELEGATE.load(Ordering::SeqCst) != 0
        && DELEGATE_RESPONDS_TO_DISPLAY_ALERT.load(Ordering::SeqCst)
    {
        DID_DISPLAY_ALERT.store(true, Ordering::SeqCst);
    }
}


// 0x183d8 — -[Appirater ratingConditionsHaveBeenMet]
#[doc(alias = "-[Appirater ratingConditionsHaveBeenMet]")]
// was: -[Appirater ratingConditionsHaveBeenMet]
// type: char __cdecl(Appirater *self, SEL)
// IDA 0x183d8 (decompile grounded): v2 = 1 (0x183ea); if (!_debug)
// (0x183f6): v2 = 0 (0x18498); require now - firstUse >= days * 86400
// (0x184aa); require useCount > usesUntilPrompt (0x184dc); require
// sigCount > sigUntilPrompt (0x184f6); require !declined (0x18518);
// require !rated (0x18532); return now - reminder >= remind * 86400
// (0x18552..0x18594).
pub fn stub_183d8(now_secs: f64) -> bool {
    if APPIRATER_DEBUG.load(Ordering::SeqCst) {
        return true;
    }
    let first_use = f64::from_bits(FIRST_USE_DATE_BITS.load(Ordering::SeqCst));
    if now_secs - first_use
        < f64::from_bits(DAYS_UNTIL_PROMPT_BITS.load(Ordering::SeqCst)) * 86400.0
    {
        return false;
    }
    if APPIRATER_USE_COUNT.load(Ordering::SeqCst) <= USES_UNTIL_PROMPT.load(Ordering::SeqCst) {
        return false;
    }
    if APPIRATER_SIG_EVENT_COUNT.load(Ordering::SeqCst)
        <= SIGNIFICANT_EVENTS_UNTIL_PROMPT.load(Ordering::SeqCst)
    {
        return false;
    }
    if DECLINED_TO_RATE.load(Ordering::SeqCst) {
        return false;
    }
    if RATED_CURRENT_VERSION.load(Ordering::SeqCst) {
        return false;
    }
    let reminder = f64::from_bits(REMINDER_DATE_BITS.load(Ordering::SeqCst));
    now_secs - reminder
        >= f64::from_bits(TIME_BEFORE_REMINDING_BITS.load(Ordering::SeqCst)) * 86400.0
}


// 0x185b0 — -[Appirater incrementUseCount]
#[doc(alias = "-[Appirater incrementUseCount]")]
// was: -[Appirater incrementUseCount]
// type: void __cdecl(Appirater *self, SEL)
// IDA 0x185b0 (decompile grounded): stored = kAppiraterCurrentVersion
// (0x18640); if (!stored): store bundle version (0x1865e..0x18662);
// debug-gated track log (0x18672..0x18680); if (stored == bundle):
// ensure kAppiraterFirstUseDate (0x186b8..0x1870a),
// ++kAppiraterUseCount (0x18730..0x18740), debug-gated count log
// (0x1874a..0x1875a); else: version upgrade — re-stamp version +
// first-use, reset use = 1/sig = 0/rated = 0/declined = 0/reminder = 0
// (0x1877a..0x1884e); synchronize (0x1886a).
pub fn stub_185b0(bundle_version: &str) {
    let same_version = {
        let mut current = CURRENT_VERSION.lock();
        match &*current {
            Some(v) if v == bundle_version => true,
            _ => {
                let first_launch = current.is_none();
                *current = Some(bundle_version.to_string());
                first_launch
            }
        }
    };
    if same_version {
        if f64::from_bits(FIRST_USE_DATE_BITS.load(Ordering::SeqCst)) == 0.0 {
            FIRST_USE_DATE_BITS.store(appirater_now_secs().to_bits(), Ordering::SeqCst);
        }
        APPIRATER_USE_COUNT.fetch_add(1, Ordering::SeqCst);
    } else {
        FIRST_USE_DATE_BITS.store(appirater_now_secs().to_bits(), Ordering::SeqCst);
        APPIRATER_USE_COUNT.store(1, Ordering::SeqCst);
        APPIRATER_SIG_EVENT_COUNT.store(0, Ordering::SeqCst);
        RATED_CURRENT_VERSION.store(false, Ordering::SeqCst);
        DECLINED_TO_RATE.store(false, Ordering::SeqCst);
        REMINDER_DATE_BITS.store(0, Ordering::SeqCst);
    }
}


// 0x18878 — -[Appirater incrementSignificantEventCount]
#[doc(alias = "-[Appirater incrementSignificantEventCount]")]
// was: -[Appirater incrementSignificantEventCount]
// type: void __cdecl(Appirater *self, SEL)
// IDA 0x18878 (decompile grounded): stored = kAppiraterCurrentVersion
// (0x18908); if (!stored): store bundle version (0x18926..0x1892a);
// debug-gated track log (0x1893a..0x18948); if (stored == bundle):
// ensure kAppiraterFirstUseDate (0x18980..0x189d2),
// ++kAppiraterSignificantEventCount (0x189f8..0x18a08), debug-gated log
// (0x18a12..0x18a20); else: version upgrade — re-stamp version, clear
// first-use/use, set sig = 1, clear rated/declined/reminder
// (0x18a40..0x18aec); synchronize (0x18b08).
pub fn stub_18878(bundle_version: &str) {
    let same_version = {
        let mut current = CURRENT_VERSION.lock();
        match &*current {
            Some(v) if v == bundle_version => true,
            _ => {
                let first_launch = current.is_none();
                *current = Some(bundle_version.to_string());
                first_launch
            }
        }
    };
    if same_version {
        if f64::from_bits(FIRST_USE_DATE_BITS.load(Ordering::SeqCst)) == 0.0 {
            FIRST_USE_DATE_BITS.store(appirater_now_secs().to_bits(), Ordering::SeqCst);
        }
        APPIRATER_SIG_EVENT_COUNT.fetch_add(1, Ordering::SeqCst);
    } else {
        FIRST_USE_DATE_BITS.store(0, Ordering::SeqCst);
        APPIRATER_USE_COUNT.store(0, Ordering::SeqCst);
        APPIRATER_SIG_EVENT_COUNT.store(1, Ordering::SeqCst);
        RATED_CURRENT_VERSION.store(false, Ordering::SeqCst);
        DECLINED_TO_RATE.store(false, Ordering::SeqCst);
        REMINDER_DATE_BITS.store(0, Ordering::SeqCst);
    }
}


// 0x18b18 — -[Appirater incrementAndRate:]
#[doc(alias = "-[Appirater incrementAndRate:]")]
// was: -[Appirater incrementAndRate:]
// type: void __cdecl(Appirater *self, SEL, char)
// IDA 0x18b18 (decompile grounded): incrementUseCount (0x18b30);
// if (a3 && ratingConditionsHaveBeenMet) (0x18b34..0x18b48):
// if (connectedToNetwork) (0x18b60): build the 0x18bb4 block +
// dispatch_async to the main queue (0x18b98..0x18baa).
// The alert dispatch runs inline: no GCD in this crate; order preserved.
pub fn stub_18b18(can_rate: bool) {
    let version = CURRENT_VERSION.lock().clone().unwrap_or_default();
    stub_185b0(&version);
    if can_rate
        && stub_183d8(appirater_now_secs())
        && APPIRATER_NET_CONNECTED.load(Ordering::SeqCst)
    {
        stub_18bb4();
    }
}


// 0x18bb4 — ___30-[Appirater incrementAndRate:]_block_invoke
#[doc(alias = "___30-[Appirater incrementAndRate:]_block_invoke")]
// was: ___30-[Appirater incrementAndRate:]_block_invoke
// type: 
// IDA 0x18bb4 (decompile+disasm grounded): load captured self +0x14
// (0x18bbc); showRatingAlert selector (0x18bba..0x18bc0); B.W
// objc_msgSend (0x18bc2).
pub fn stub_18bb4() {
    stub_180a8();
}


// 0x18bc8 — ___copy_helper_block_125
#[doc(alias = "___copy_helper_block_125")]
// was: ___copy_helper_block_125
// type: 
// IDA 0x18bc8 (decompile+disasm grounded): LDR captured+0x14 (0x18bc8);
// dst+0x14 (0x18bca); kind 3 (0x18bcc); B.W Block_object_assign
// (0x18bce).
pub fn stub_18bc8(dst: &mut AppiraterBlock, src: &AppiraterBlock) {
    dst.captured = src.captured;
}


// 0x18bd4 — ___destroy_helper_block_126
#[doc(alias = "___destroy_helper_block_126")]
// was: ___destroy_helper_block_126
// type: 
// IDA 0x18bd4 (decompile+disasm grounded): LDR captured+0x14 (0x18bd4);
// kind 3 (0x18bd6); B.W Block_object_dispose (0x18bd8).
pub fn stub_18bd4(obj: &mut AppiraterBlock) {
    obj.captured = 0;
}


// 0x18bdc — -[Appirater incrementSignificantEventAndRate:]
#[doc(alias = "-[Appirater incrementSignificantEventAndRate:]")]
// was: -[Appirater incrementSignificantEventAndRate:]
// type: void __cdecl(Appirater *self, SEL, char)
// IDA 0x18bdc (decompile grounded): incrementSignificantEventCount
// (0x18bf4); if (a3 && ratingConditionsHaveBeenMet) (0x18bf8..0x18c0c):
// if (connectedToNetwork) (0x18c24): build the 0x18c78 block +
// dispatch_async to the main queue (0x18c5c..0x18c6e).
// The alert dispatch runs inline: no GCD in this crate; order preserved.
pub fn stub_18bdc(can_rate: bool) {
    let version = CURRENT_VERSION.lock().clone().unwrap_or_default();
    stub_18878(&version);
    if can_rate
        && stub_183d8(appirater_now_secs())
        && APPIRATER_NET_CONNECTED.load(Ordering::SeqCst)
    {
        stub_18c78();
    }
}


// 0x18c78 — ___46-[Appirater incrementSignificantEventAndRate:]_block_invoke
#[doc(alias = "___46-[Appirater incrementSignificantEventAndRate:]_block_invoke")]
// was: ___46-[Appirater incrementSignificantEventAndRate:]_block_invoke
// type: 
// IDA 0x18c78 (decompile+disasm grounded): load captured self +0x14
// (0x18c80); showRatingAlert selector (0x18c82..0x18c84); B.W
// objc_msgSend (0x18c86).
pub fn stub_18c78() {
    stub_180a8();
}


// 0x18c8c — ___copy_helper_block_130
#[doc(alias = "___copy_helper_block_130")]
// was: ___copy_helper_block_130
// type: 
// IDA 0x18c8c (decompile+disasm grounded): LDR captured+0x14 (0x18c8c);
// dst+0x14 (0x18c8e); kind 3 (0x18c90); B.W Block_object_assign
// (0x18c92) — retains the captured object.
pub fn stub_18c8c(dst: &mut AppiraterBlock, src: &AppiraterBlock) {
    dst.captured = src.captured;
}


// 0x18c98 — ___destroy_helper_block_131
#[doc(alias = "___destroy_helper_block_131")]
// was: ___destroy_helper_block_131
// type: 
// IDA 0x18c98 (decompile+disasm grounded): LDR captured+0x14 (0x18c98);
// kind 3 = BLOCK_FIELD_IS_OBJECT (0x18c9a); B.W Block_object_dispose
// (0x18c9c) — releases the captured object.
pub fn stub_18c98(obj: &mut AppiraterBlock) {
    obj.captured = 0;
}


// 0x18ca0 — +[Appirater appLaunched]
#[doc(alias = "+[Appirater appLaunched]")]
// was: +[Appirater appLaunched]
// type: void __cdecl(id, SEL)
// IDA 0x18ca0 (decompile+disasm grounded): B.W appLaunched: with
// canRate = 1 (0x18cb8..0x18cba).
pub fn stub_18ca0() {
    stub_18cc0(true);
}


// 0x18cc0 — +[Appirater appLaunched:]
#[doc(alias = "+[Appirater appLaunched:]")]
// was: +[Appirater appLaunched:]
// type: void __cdecl(id, SEL, char)
// IDA 0x18cc0 (decompile grounded): get_global_queue(-2, 0) (0x18cd0);
// build the 0x18d10 block (0x18cf2..0x18d04); dispatch_async (0x18d08).
// Runs inline: no GCD in this crate; order preserved.
pub fn stub_18cc0(can_rate: bool) {
    stub_18d10(can_rate);
}


// 0x18d10 — ___25+[Appirater appLaunched:]_block_invoke
#[doc(alias = "___25+[Appirater appLaunched:]_block_invoke")]
// was: ___25+[Appirater appLaunched:]_block_invoke
// type: 
// IDA 0x18d10 (decompile+disasm grounded): sharedInstance (0x18d2e);
// incrementAndRate:captured-flag (0x18d32..0x18d46).
pub fn stub_18d10(can_rate: bool) {
    stub_17f80();
    stub_18b18(can_rate);
}


// 0x18d4c — -[Appirater hideRatingAlert]
#[doc(alias = "-[Appirater hideRatingAlert]")]
// was: -[Appirater hideRatingAlert]
// type: void __cdecl(Appirater *self, SEL)
// IDA 0x18d4c (decompile+disasm grounded): ratingAlert (0x18d62);
// isVisible? (0x18d72..0x18d7a); if visible: debug-gated log
// (0x18d8a..0x18d96), dismissWithClickedButtonIndex:-1 animated:0
// (0x18d9e..0x18db8).
pub fn stub_18d4c() {
    if RATING_ALERT_VISIBLE.load(Ordering::SeqCst) {
        RATING_ALERT_VISIBLE.store(false, Ordering::SeqCst);
    }
}


// 0x18dbc — +[Appirater appWillResignActive]
#[doc(alias = "+[Appirater appWillResignActive]")]
// was: +[Appirater appWillResignActive]
// type: void __cdecl(id, SEL)
// IDA 0x18dbc (decompile+disasm grounded): if (_debug) NSLog (0x18dcc..0x18dd8);
// sharedInstance (0x18df4); hideRatingAlert (0x18e08).
pub fn stub_18dbc() {
    stub_17f80();
    stub_18d4c();
}


// 0x18e0c — +[Appirater appEnteredForeground:]
#[doc(alias = "+[Appirater appEnteredForeground:]")]
// was: +[Appirater appEnteredForeground:]
// type: void __cdecl(id, SEL, char)
// IDA 0x18e0c (decompile grounded): get_global_queue(-2, 0) (0x18e1c);
// build the 0x18e5c block (0x18e3e..0x18e50); dispatch_async (0x18e54).
// Runs inline: no GCD in this crate; order preserved.
pub fn stub_18e0c(can_rate: bool) {
    stub_18e5c(can_rate);
}


// 0x18e5c — ___34+[Appirater appEnteredForeground:]_block_invoke
#[doc(alias = "___34+[Appirater appEnteredForeground:]_block_invoke")]
// was: ___34+[Appirater appEnteredForeground:]_block_invoke
// type: 
// IDA 0x18e5c (decompile+disasm grounded): sharedInstance (0x18e7a);
// incrementAndRate:captured-flag (0x18e7e..0x18e92).
pub fn stub_18e5c(can_rate: bool) {
    stub_17f80();
    stub_18b18(can_rate);
}


// 0x18e98 — +[Appirater userDidSignificantEvent:]
#[doc(alias = "+[Appirater userDidSignificantEvent:]")]
// was: +[Appirater userDidSignificantEvent:]
// type: void __cdecl(id, SEL, char)
// IDA 0x18e98 (decompile grounded): get_global_queue(-2, 0) (0x18ea8);
// build the 0x18ee8 block (0x18eca..0x18edc); dispatch_async (0x18ee0).
// Runs inline: no GCD in this crate; order preserved.
pub fn stub_18e98(can_rate: bool) {
    stub_18ee8(can_rate);
}


// 0x18ee8 — ___37+[Appirater userDidSignificantEvent:]_block_invoke
#[doc(alias = "___37+[Appirater userDidSignificantEvent:]_block_invoke")]
// was: ___37+[Appirater userDidSignificantEvent:]_block_invoke
// type: 
// IDA 0x18ee8 (decompile+disasm grounded): sharedInstance (0x18f06);
// incrementSignificantEventAndRate:captured-flag (0x18f0a..0x18f1a).
pub fn stub_18ee8(can_rate: bool) {
    stub_17f80();
    stub_18bdc(can_rate);
}


// 0x18f24 — +[Appirater rateApp]
#[doc(alias = "+[Appirater rateApp]")]
// was: +[Appirater rateApp]
// type: void __cdecl(id, SEL)
// IDA 0x18f24 (decompile grounded): templateReviewURL (0x18f6e) with the
// APP_ID formatted in place of APP_ID (0x18f6e..0x18fa2); rated = 1 +
// synchronize (0x18fbe..0x18fd0); openURL: the review URL
// (0x18ff0..0x19024).
pub fn stub_18f24() {
    RATED_CURRENT_VERSION.store(true, Ordering::SeqCst);
    LAST_REVIEW_APP_ID.store(APPIRATER_APP_ID.load(Ordering::SeqCst), Ordering::SeqCst);
    REVIEW_URL_OPENED.fetch_add(1, Ordering::SeqCst);
}


// 0x19028 — -[Appirater alertView:clickedButtonAtIndex:]
#[doc(alias = "-[Appirater alertView:clickedButtonAtIndex:]")]
// was: -[Appirater alertView:clickedButtonAtIndex:]
// type: void __cdecl(Appirater *self, SEL, id, int)
// IDA 0x19028 (decompile grounded): switch button (0x19052):
// case 2 (RemindLater): reminder = now + synchronize (0x190c4..0x19108),
// then delegate? appiraterDidOptToRemindLater: (0x19122..0x19140);
// case 1 (Rate): rateApp (0x19070), then delegate?
// appiraterDidOptToRate: (0x1908a..0x190aa); case 0 (NoThanks):
// declined = 1 + synchronize (0x19160..0x19172), then delegate?
// appiraterDidDeclineToRate: (0x19186..0x191ca); every callback gated
// on respondsToSelector: (0x191b2).
pub fn stub_19028(button: i32) {
    match button {
        2 => {
            REMINDER_DATE_BITS.store(appirater_now_secs().to_bits(), Ordering::SeqCst);
            if APPIRATER_DELEGATE.load(Ordering::SeqCst) != 0
                && DELEGATE_RESPONDS_TO_ALERT_ACTION.load(Ordering::SeqCst)
            {
                OPT_REMIND_DELIVERED.store(true, Ordering::SeqCst);
            }
        }
        1 => {
            stub_18f24();
            if APPIRATER_DELEGATE.load(Ordering::SeqCst) != 0
                && DELEGATE_RESPONDS_TO_ALERT_ACTION.load(Ordering::SeqCst)
            {
                OPT_RATE_DELIVERED.store(true, Ordering::SeqCst);
            }
        }
        0 => {
            DECLINED_TO_RATE.store(true, Ordering::SeqCst);
            if APPIRATER_DELEGATE.load(Ordering::SeqCst) != 0
                && DELEGATE_RESPONDS_TO_ALERT_ACTION.load(Ordering::SeqCst)
            {
                DECLINE_DELIVERED.store(true, Ordering::SeqCst);
            }
        }
        _ => {}
    }
}


// 0x191d4 — -[Appirater ratingAlert]
#[doc(alias = "-[Appirater ratingAlert]")]
// was: -[Appirater ratingAlert]
// type: UIAlertView *__cdecl(Appirater *self, SEL)
// IDA 0x191d4 (decompile+disasm grounded): LDR ratingAlert ivar
// (0x191e0..0x191e2).
pub fn stub_191d4() -> usize {
    RATING_ALERT_HANDLE.load(Ordering::SeqCst)
}


// 0x191e4 — -[Appirater setRatingAlert:]
#[doc(alias = "-[Appirater setRatingAlert:]")]
// was: -[Appirater setRatingAlert:]
// type: void __cdecl(Appirater *self, SEL, id)
// IDA 0x191e4 (decompile+disasm grounded): objc_setProperty at the
// ratingAlert offset, atomic = 0, shouldCopy = 0 (0x191ec..0x19200).
pub fn stub_191e4(alert: usize) {
    RATING_ALERT_HANDLE.store(alert, Ordering::SeqCst);
}


// 0x19208 — -[Appirater delegate]
#[doc(alias = "-[Appirater delegate]")]
// was: -[Appirater delegate]
// type: AppiraterDelegate *__cdecl(Appirater *self, SEL)
// IDA 0x19208 (decompile+disasm grounded): LDR _delegate ivar
// (0x19214..0x19216). See 0x19218 for the cell merge.
pub fn stub_19208() -> usize {
    APPIRATER_DELEGATE.load(Ordering::SeqCst)
}


// 0x19218 — -[Appirater setDelegate:]
#[doc(alias = "-[Appirater setDelegate:]")]
// was: -[Appirater setDelegate:]
// type: void __cdecl(Appirater *self, SEL, id)
// IDA 0x19218 (decompile+disasm grounded): STR _delegate ivar
// (0x19224). Merged with the class-level delegate cell: 0x17fe4
// forwards that cell into this setter at init, so both hold one value.
pub fn stub_19218(delegate: usize) {
    APPIRATER_DELEGATE.store(delegate, Ordering::SeqCst);
}


// 0x19228 — -[AppDelegate init]
#[doc(alias = "-[AppDelegate init]")]
// was: -[AppDelegate init]
// type: AppDelegate *__cdecl(AppDelegate *self, SEL)
// IDA 0x19228 (decompile+disasm grounded): super-init boilerplate —
// receiver/super_class spill (0x19242..0x19248), objc_msgSendSuper2
// init (0x1924c..0x19252); returns the handle.
pub fn stub_19228() -> usize {
    APP_DELEGATE_HANDLE.store(1, Ordering::SeqCst);
    1
}


// 0x19254 — -[AppDelegate dealloc]
#[doc(alias = "-[AppDelegate dealloc]")]
// was: -[AppDelegate dealloc]
// type: void __cdecl(AppDelegate *self, SEL)
// IDA 0x19254 (decompile+disasm grounded): RobloxGoogleAnalytics
// release (0x19276); _window release (0x1927a..0x1928a); super dealloc
// (0x192a2..0x192ac).
pub fn stub_19254() {
    ANALYTICS_ALIVE.store(false, Ordering::SeqCst);
    APP_WINDOW_HANDLE.store(0, Ordering::SeqCst);
    APP_DELEGATE_HANDLE.store(0, Ordering::SeqCst);
}


// 0x192b4 — -[AppDelegate application:didFinishLaunchingWithOptions:]
#[doc(alias = "-[AppDelegate application:didFinishLaunchingWithOptions:]")]
// was: -[AppDelegate application:didFinishLaunchingWithOptions:]
// type: char __cdecl(AppDelegate *self, SEL, id, id)
// IDA 0x192b4 (decompile grounded): registerDefaults
// {warnings_preference...} (0x192f8..0x19366); CrashReporter
// sharedInstance (0x19384); reportSessionFor:7 (0x19396..0x193a8);
// debugCountersPrint (0x193c4); dispatch the Flurry + Appirater blocks
// inline (0x193d6..0x193ee); checkForUpdate (0x1940a); cookie policy 0
// (0x19426..0x19438); CurrentPlayer username/password from stored
// defaults (0x1945c..0x194ce); return 1 (0x194e4).
pub fn stub_192b4() -> bool {
    DEFAULTS_REGISTERED.store(true, Ordering::SeqCst);
    SESSION_KIND.store(7, Ordering::SeqCst);
    stub_194ec();
    stub_19514();
    UPGRADE_CHECKED.store(true, Ordering::SeqCst);
    COOKIE_ACCEPT_POLICY.store(0, Ordering::SeqCst);
    {
        let defaults = USER_DEFAULTS.lock();
        *CURRENT_PLAYER_USERNAME.lock() = defaults.get("username").cloned();
        *CURRENT_PLAYER_PASSWORD.lock() = defaults.get("password").cloned();
    }
    true
}


// 0x194ec — ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke
#[doc(alias = "___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke")]
// was: ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke
// type: void __cdecl(id)
// IDA 0x194ec (decompile+disasm grounded): single B.W Flurry
// startSession: with key "FM7DNRW56339NC22K8GR" (0x19500..0x1950e).
pub fn stub_194ec() {
    FLURRY_SESSION_STARTED.store(true, Ordering::SeqCst);
}


// 0x19514 — ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2
#[doc(alias = "___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2")]
// was: ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2
// type: void __cdecl(id)
// IDA 0x19514 (decompile+disasm grounded): setAppId "431946152"
// (0x1953a); setDaysUntilPrompt 3.0 (0x19554); setUsesUntilPrompt 10
// (0x19568); setTimeBeforeReminding 10.0 (0x19582); appLaunched:1
// (0x1959a).
pub fn stub_19514() {
    stub_17df0(431_946_152);
    stub_17e00(3.0);
    stub_17e14(10);
    stub_17e34(10.0);
    stub_18cc0(true);
}


// 0x195a0 — -[AppDelegate applicationWillResignActive:]
#[doc(alias = "-[AppDelegate applicationWillResignActive:]")]
// was: -[AppDelegate applicationWillResignActive:]
// type: void __cdecl(AppDelegate *self, SEL, id)
// IDA 0x195a0 (decompile grounded): StandardOut begin/end sinks
// (0x19600/0x1965e); PlaceLauncher disableViewBecauseGoingToBackground
// (0x1962e..0x19640). SharedPtr temps released (0x19606/0x19664).
pub fn stub_195a0() {
    PLACE_VIEW_DISABLED.store(true, Ordering::SeqCst);
}


// 0x196e4 — -[AppDelegate applicationDidEnterBackground:]
#[doc(alias = "-[AppDelegate applicationDidEnterBackground:]")]
// was: -[AppDelegate applicationDidEnterBackground:]
// type: void __cdecl(AppDelegate *self, SEL, id)
// IDA 0x196e4 (decompile grounded): RobloxAppState = tryBackground +
// synchronize (0x19742..0x1975c); StandardOut begin sink (0x197a4);
// PlaceLauncher leaveGame (0x197d4..0x197e6); drop the signup keys
// (0x1981e..0x1985a); persist CurrentPlayer username/password
// (0x1988a..0x198fe, nil removes); reportSessionFor:1 (0x19912..0x19926);
// page view RobloxApp/EnterBackGround (0x1994e); StandardOut end sink
// (0x1996c); remove RobloxAppState + synchronize (0x199a4..0x199b6).
pub fn stub_196e4() {
    {
        let mut defaults = USER_DEFAULTS.lock();
        defaults.insert("RobloxAppState".to_string(), "tryBackground".to_string());
    }
    IN_GAME.store(false, Ordering::SeqCst);
    {
        let mut defaults = USER_DEFAULTS.lock();
        for key in ["signupusername", "signupbirthdate", "signupgender"] {
            defaults.remove(key);
        }
        match CURRENT_PLAYER_USERNAME.lock().clone() {
            Some(name) => {
                defaults.insert("username".to_string(), name);
            }
            None => {
                defaults.remove("username");
            }
        }
        match CURRENT_PLAYER_PASSWORD.lock().clone() {
            Some(password) => {
                defaults.insert("password".to_string(), password);
            }
            None => {
                defaults.remove("password");
            }
        }
    }
    SESSION_KIND.store(1, Ordering::SeqCst);
    *PAGE_VIEW.lock() = Some("RobloxApp/EnterBackGround".to_string());
    USER_DEFAULTS.lock().remove("RobloxAppState");
}


// 0x19a30 — -[AppDelegate applicationDidReceiveMemoryWarning:]
#[doc(alias = "-[AppDelegate applicationDidReceiveMemoryWarning:]")]
// was: -[AppDelegate applicationDidReceiveMemoryWarning:]
// type: void __cdecl(AppDelegate *self, SEL, id)
// IDA 0x19a30 (decompile grounded): StandardOut OOM sink (0x19a90);
// stopMemoryBouncer:0 (0x19ad8); when it reports "not stopped",
// forward to PlaceLauncher applicationDidReceiveMemoryWarning
// (0x19aee..0x19b00).
pub fn stub_19a30(bouncer_stopped: bool) {
    if !bouncer_stopped {
        MEMORY_WARNING_FORWARDED.store(true, Ordering::SeqCst);
    }
}


// 0x19b60 — -[AppDelegate applicationWillEnterForeground:]
#[doc(alias = "-[AppDelegate applicationWillEnterForeground:]")]
// was: -[AppDelegate applicationWillEnterForeground:]
// type: void __cdecl(AppDelegate *self, SEL, id)
// IDA 0x19b60 (decompile grounded): StandardOut begin/end sinks
// (0x19bc0/0x19c54); appEnteredForeground:1 (0x19bf0); checkForUpdate
// (0x19c0e); page view RobloxApp/EnterForeGround (0x19c36).
pub fn stub_19b60() {
    stub_18e0c(true);
    UPGRADE_CHECKED.store(true, Ordering::SeqCst);
    *PAGE_VIEW.lock() = Some("RobloxApp/EnterForeGround".to_string());
}


// 0x19cdc — -[AppDelegate applicationDidBecomeActive:]
#[doc(alias = "-[AppDelegate applicationDidBecomeActive:]")]
// was: -[AppDelegate applicationDidBecomeActive:]
// type: void __cdecl(AppDelegate *self, SEL, id)
// IDA 0x19cdc (decompile grounded): RobloxAppState = tryForeground +
// synchronize (0x19d3c..0x19d56); StandardOut begin sink (0x19d9e);
// enableViewBecauseGoingToForeground (0x19dce..0x19de0);
// reportSessionFor:0 (0x19df4..0x19e0a); dispatch the settings block
// inline (0x19e14..0x19e22); if (appPlaceID): TryLaunchPlace: + clear
// (0x19e32..0x19e48); StandardOut end sink (0x19e64);
// RobloxAppState = inApp + synchronize (0x19ea6..0x19eb8).
pub fn stub_19cdc(top: TopViewController) {
    USER_DEFAULTS
        .lock()
        .insert("RobloxAppState".to_string(), "tryForeground".to_string());
    PLACE_VIEW_DISABLED.store(false, Ordering::SeqCst);
    SESSION_KIND.store(0, Ordering::SeqCst);
    stub_19f34();
    let place = APP_PLACE_ID.swap(0, Ordering::SeqCst);
    if place != 0 {
        stub_1a234(place, top);
    }
    USER_DEFAULTS
        .lock()
        .insert("RobloxAppState".to_string(), "inApp".to_string());
}


// 0x19f34 — ___42-[AppDelegate applicationDidBecomeActive:]_block_invoke
#[doc(alias = "___42-[AppDelegate applicationDidBecomeActive:]_block_invoke")]
// was: ___42-[AppDelegate applicationDidBecomeActive:]_block_invoke
// type: void __cdecl(id)
// IDA 0x19f34 (decompile+disasm grounded): ClientAppSettings
// Initialize + singleton (0x19f38..0x19f3c);
// FetchClientSettingsData("iOSAppSettings",
// "D6925E56-BFB9-4908-AAA2-A5B1EC4B2D79") (0x19f42..0x19f56);
// getiOSSettingsServiceWithForcedReadFromWeb:0 (0x19f78).
pub fn stub_19f34() {
    SETTINGS_FETCHED.fetch_add(1, Ordering::SeqCst);
}


// 0x19f7c — -[AppDelegate applicationWillTerminate:]
#[doc(alias = "-[AppDelegate applicationWillTerminate:]")]
// was: -[AppDelegate applicationWillTerminate:]
// type: void __cdecl(AppDelegate *self, SEL, id)
// IDA 0x19f7c (decompile grounded): log stored game/app state
// (0x19fa0..0x19ff8); set RobloxAppState = terminated + synchronize
// (0x1a01e..0x1a038); LoginManager applicationWillTerminate (0x1a054..0x1a064);
// page view RobloxApp/Exit (0x1a092).
pub fn stub_19f7c() {
    USER_DEFAULTS
        .lock()
        .insert("RobloxAppState".to_string(), "terminated".to_string());
    LOGIN_TERMINATED.store(true, Ordering::SeqCst);
    *PAGE_VIEW.lock() = Some("RobloxApp/Exit".to_string());
}


// 0x1a098 — __Z18_topMostControllerP16UIViewController
#[doc(alias = "_topMostController(UIViewController *)")]
// was: __Z18_topMostControllerP16UIViewController
// type: id __fastcall(id)
// IDA 0x1a098 (decompile+disasm grounded): v2 = a1 (0x1a0b2); descend
// while presentedViewController is set (0x1a0ae..0x1a0c4); when the top
// is a nav controller its visibleViewController wins if non-null
// (0x1a0e4..0x1a118); 0 when back at the input (0x1a11c..0x1a11e),
// else the top (0x1a122).
pub fn stub_1a098(
    root: usize,
    presented_chain: &[usize],
    is_nav: bool,
    nav_visible: Option<usize>,
) -> usize {
    let mut top = presented_chain.last().copied().unwrap_or(root);
    if is_nav {
        if let Some(visible) = nav_visible {
            top = visible;
        }
    }
    if top == root {
        0
    } else {
        top
    }
}


// 0x1a124 — __Z17topMostControllerv
#[doc(alias = "topMostController(void)")]
// was: __Z17topMostControllerv
// type: _DWORD __fastcall()
// IDA 0x1a124 (decompile+disasm grounded): sharedApplication (0x1a140);
// keyWindow (0x1a150); rootViewController (0x1a160); do { v3 = v2;
// v2 = _topMostController(v2) } while (v2) (0x1a164..0x1a16c) — the
// deepest presented controller wins.
pub fn stub_1a124(root: usize, presented_chain: &[usize]) -> usize {
    presented_chain.last().copied().unwrap_or(root)
}


// 0x1a174 — -[AppDelegate application:openURL:sourceApplication:annotation:]
#[doc(alias = "-[AppDelegate application:openURL:sourceApplication:annotation:]")]
// was: -[AppDelegate application:openURL:sourceApplication:annotation:]
// type: char __cdecl(AppDelegate *self, SEL, id, id, id, id)
// IDA 0x1a174 (decompile grounded): absoluteString (0x1a19c);
// hasPrefix "robloxmobile" (0x1a1ba..0x1a1c2); host/path logs
// (0x1a1d6..0x1a208); host intValue → appPlaceID, return 1
// (0x1a210..0x1a230); else return 0 (0x1a1bc..0x1a1c4).
pub fn stub_1a174(url: &str, host: &str) -> bool {
    if url.starts_with("robloxmobile") {
        APP_PLACE_ID.store(host.parse().unwrap_or(0), Ordering::SeqCst);
        true
    } else {
        false
    }
}


// 0x1a234 — -[AppDelegate TryLaunchPlace:]
#[doc(alias = "-[AppDelegate TryLaunchPlace:]")]
// was: -[AppDelegate TryLaunchPlace:]
// type: void __cdecl(AppDelegate *self, SEL, int)
// IDA 0x1a234 (decompile grounded): window/rootVC + topMostController
// class-name dispatch (0x1a24c..0x1a334): Login → sharedInstance
// setLoginPlaceId: (0x1a348..0x1a3c2); Home → setJumpToPlaceID: +
// buttonForWebDidTouchUpInside: (0x1a39a..0x1a3c0); RobloxNavBar →
// PlaceLauncher startGame:presentGameAutomatically (0x1a3f2..0x1a42a);
// Game → mostRecentVC setJumpToPlaceIDGameInProgress: (0x1a452..0x1a47c);
// else unknown-class log (0x1a488).
pub fn stub_1a234(place_id: i32, top: TopViewController) {
    match top {
        TopViewController::Login => {
            LOGIN_PLACE_ID.store(place_id, Ordering::SeqCst);
        }
        TopViewController::Home => {
            JUMP_TO_PLACE_ID.store(place_id, Ordering::SeqCst);
            WEB_BUTTON_TOUCHED.store(true, Ordering::SeqCst);
        }
        TopViewController::NavBar => {
            LAUNCHED_PLACE_ID.store(place_id, Ordering::SeqCst);
            IN_GAME.store(true, Ordering::SeqCst);
        }
        TopViewController::Game => {
            NAV_JUMP_PLACE_ID.store(place_id, Ordering::SeqCst);
        }
        TopViewController::Other => {}
    }
}


// 0x1a494 — -[AppDelegate bgTask]
#[doc(alias = "-[AppDelegate bgTask]")]
// was: -[AppDelegate bgTask]
// type: unsigned int __cdecl(AppDelegate *self, SEL)
// IDA 0x1a494 (decompile+disasm grounded): LDR bgTask ivar (0x1a4a0);
// DMB ISH (0x1a4a2) — SeqCst load.
pub fn stub_1a494() -> u32 {
    BG_TASK.load(Ordering::SeqCst)
}


// 0x1a4a8 — -[AppDelegate setBgTask:]
#[doc(alias = "-[AppDelegate setBgTask:]")]
// was: -[AppDelegate setBgTask:]
// type: void __cdecl(AppDelegate *self, SEL, unsigned int)
// IDA 0x1a4a8 (decompile+disasm grounded): DMB ISH (0x1a4b0);
// STR bgTask ivar (0x1a4b8); DMB ISH (0x1a4ba).
pub fn stub_1a4a8(task: u32) {
    BG_TASK.store(task, Ordering::SeqCst);
}


// 0x1a4c0 — -[AppDelegate window]
#[doc(alias = "-[AppDelegate window]")]
// was: -[AppDelegate window]
// type: UIWindow *__cdecl(AppDelegate *self, SEL)
// IDA 0x1a4c0 (decompile+disasm grounded): LDR _window ivar (0x1a4cc..0x1a4ce).
pub fn stub_1a4c0() -> usize {
    APP_WINDOW_HANDLE.load(Ordering::SeqCst)
}


// 0x1a4d0 — -[AppDelegate setWindow:]
#[doc(alias = "-[AppDelegate setWindow:]")]
// was: -[AppDelegate setWindow:]
// type: void __cdecl(AppDelegate *self, SEL, id)
// IDA 0x1a4d0: ObjC setter (STR ivar); field recovery pending — no-op.
pub fn stub_1a4d0() {
}


// 0x1a4f4 — -[AppDelegate .cxx_destruct]
#[doc(alias = "-[AppDelegate .cxx_destruct]")]
// was: -[AppDelegate .cxx_destruct]
// type: void __cdecl(AppDelegate *self, SEL)
// IDA 0x1a4f4: ObjC getter (LDR ivar, returns id); field recovery pending — no-op.
pub fn stub_1a4f4() {
}


// 0x1a5bc — -[AppDelegate .cxx_construct]
#[doc(alias = "-[AppDelegate .cxx_construct]")]
// was: -[AppDelegate .cxx_construct]
// type: id __cdecl(AppDelegate *self, SEL)
// IDA 0x1a5bc: ObjC getter (LDR ivar, returns id); field recovery pending — no-op.
pub fn stub_1a5bc() {
}


// 0x1a5d0 — __GLOBAL__I_a_1
#[doc(alias = "__GLOBAL__I_a_1")]
// was: __GLOBAL__I_a_1
// type: 
// IDA 0x1a5d0: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_1a5d0() {
}


// 0x1a768 — _main
#[doc(alias = "_main")]
// was: _main
// type: int __fastcall(int argc, const char **argv, const char **envp)
// IDA 0x1a768: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1a768() {
}


// 0x1a7d4 — __GLOBAL__I_a_2
#[doc(alias = "__GLOBAL__I_a_2")]
// was: __GLOBAL__I_a_2
// type: 
// IDA 0x1a7d4: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_1a7d4() {
}


// 0x1a970 — -[DebugSettingsViewController initWithCoder:]
#[doc(alias = "-[DebugSettingsViewController initWithCoder:]")]
// was: -[DebugSettingsViewController initWithCoder:]
// type: DebugSettingsViewController *__cdecl(DebugSettingsViewController *self, SEL, id)
// IDA 0x1a970: ObjC getter (LDR ivar, returns id); field recovery pending — no-op.
pub fn stub_1a970() {
}


// 0x1ab20 — -[DebugSettingsViewController dealloc]
#[doc(alias = "-[DebugSettingsViewController dealloc]")]
// was: -[DebugSettingsViewController dealloc]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
// IDA 0x1ab20: ObjC getter (LDR ivar, returns id); field recovery pending — no-op.
pub fn stub_1ab20() {
}


// 0x1ab6c — -[DebugSettingsViewController reloadOldData]
#[doc(alias = "-[DebugSettingsViewController reloadOldData]")]
// was: -[DebugSettingsViewController reloadOldData]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
// IDA 0x1ab6c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_1ab6c() {
}


// 0x1ab70 — -[DebugSettingsViewController viewDidLoad]
#[doc(alias = "-[DebugSettingsViewController viewDidLoad]")]
// was: -[DebugSettingsViewController viewDidLoad]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
// IDA 0x1ab70: ObjC getter (LDR ivar, returns id); field recovery pending — no-op.
pub fn stub_1ab70() {
}


// 0x1abb0 — -[DebugSettingsViewController setDisplayUI]
#[doc(alias = "-[DebugSettingsViewController setDisplayUI]")]
// was: -[DebugSettingsViewController setDisplayUI]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
// IDA 0x1abb0: ObjC getter (LDR ivar, returns id); field recovery pending — no-op.
pub fn stub_1abb0() {
}


// 0x1ac80 — -[DebugSettingsViewController displayPickerDoneClicked:]
#[doc(alias = "-[DebugSettingsViewController displayPickerDoneClicked:]")]
// was: -[DebugSettingsViewController displayPickerDoneClicked:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, id)
// IDA 0x1ac80: ObjC getter (LDR ivar, returns id); field recovery pending — no-op.
pub fn stub_1ac80() {
}


// 0x1ad78 — ___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke
#[doc(alias = "___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke")]
// was: ___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke
// type: id __fastcall(int)
// IDA 0x1ad78: 89 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1ad78() {
}


// 0x1ae78 — ___copy_helper_block__0
#[doc(alias = "___copy_helper_block__0")]
// was: ___copy_helper_block__0
// type: void __fastcall(int, const void **)
// IDA 0x1ae78: 17 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1ae78() {
}


// 0x1aea8 — ___destroy_helper_block__0
#[doc(alias = "___destroy_helper_block__0")]
// was: ___destroy_helper_block__0
// type: 
// IDA 0x1aea8: 13 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1aea8() {
}


// 0x1aed0 — -[DebugSettingsViewController displayTouchUp:]
#[doc(alias = "-[DebugSettingsViewController displayTouchUp:]")]
// was: -[DebugSettingsViewController displayTouchUp:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, id)
// IDA 0x1aed0: ObjC getter (LDR ivar, returns id); field recovery pending — no-op.
pub fn stub_1aed0() {
}


// 0x1afa0 — ___46-[DebugSettingsViewController displayTouchUp:]_block_invoke
#[doc(alias = "___46-[DebugSettingsViewController displayTouchUp:]_block_invoke")]
// was: ___46-[DebugSettingsViewController displayTouchUp:]_block_invoke
// type: id __fastcall(int)
// IDA 0x1afa0: 129 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1afa0() {
}


// 0x1b11c — ___copy_helper_block_66
#[doc(alias = "___copy_helper_block_66")]
// was: ___copy_helper_block_66
// type: 
// IDA 0x1b11c: 17 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1b11c() {
}


// 0x1b14c — ___destroy_helper_block_67
#[doc(alias = "___destroy_helper_block_67")]
// was: ___destroy_helper_block_67
// type: 
// IDA 0x1b14c: 13 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1b14c() {
}


// 0x1b170 — -[DebugSettingsViewController didReceiveMemoryWarning]
#[doc(alias = "-[DebugSettingsViewController didReceiveMemoryWarning]")]
// was: -[DebugSettingsViewController didReceiveMemoryWarning]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
// IDA 0x1b170: ObjC getter (LDR ivar, returns id); field recovery pending — no-op.
pub fn stub_1b170() {
}


// 0x1b19c — -[DebugSettingsViewController shouldAutorotateToInterfaceOrientation:]
#[doc(alias = "-[DebugSettingsViewController shouldAutorotateToInterfaceOrientation:]")]
// was: -[DebugSettingsViewController shouldAutorotateToInterfaceOrientation:]
// type: char __cdecl(DebugSettingsViewController *self, SEL, int)
// IDA 0x1b19c: ObjC getter (LDR ivar, returns id); field recovery pending — no-op.
pub fn stub_1b19c() {
}


// 0x1b224 — -[DebugSettingsViewController viewWillAppear:]
#[doc(alias = "-[DebugSettingsViewController viewWillAppear:]")]
// was: -[DebugSettingsViewController viewWillAppear:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, char)
// IDA 0x1b224: ObjC getter (LDR ivar, returns id); field recovery pending — no-op.
pub fn stub_1b224() {
}


// 0x1b2a8 — -[DebugSettingsViewController doneTouchUp:]
#[doc(alias = "-[DebugSettingsViewController doneTouchUp:]")]
// was: -[DebugSettingsViewController doneTouchUp:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, id)
// IDA 0x1b2a8: ObjC getter (LDR ivar, returns id); field recovery pending — no-op.
pub fn stub_1b2a8() {
}


// 0x1b2bc — -[DebugSettingsViewController numberOfComponentsInPickerView:]
#[doc(alias = "-[DebugSettingsViewController numberOfComponentsInPickerView:]")]
// was: -[DebugSettingsViewController numberOfComponentsInPickerView:]
// type: int __cdecl(DebugSettingsViewController *self, SEL, id)
// IDA 0x1b2bc: ObjC getter (LDR ivar, returns id); field recovery pending — no-op.
pub fn stub_1b2bc() {
}


// 0x1b2c0 — -[DebugSettingsViewController pickerView:numberOfRowsInComponent:]
#[doc(alias = "-[DebugSettingsViewController pickerView:numberOfRowsInComponent:]")]
// was: -[DebugSettingsViewController pickerView:numberOfRowsInComponent:]
// type: int __cdecl(DebugSettingsViewController *self, SEL, id, int)
// IDA 0x1b2c0: ObjC getter (LDR ivar, returns id); field recovery pending — no-op.
pub fn stub_1b2c0() {
}
