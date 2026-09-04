//! reflection — generated_bg_3 — 100 stubs EA-sorted asc global gap filler 0x194ec..0x1d35c not yet in crates/reflection (global 85545 funcs, 64401 gaps reflection; 21145 distinct before, 21245 after)
//! Source: ida/export.json (85545 funcs) global EA asc not in crates/reflection/src — next 100 uncovered for reflection-bg sorted asc
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

/// Gap-filler AppDelegate/lifecycle observable state (IDA 0x192b4-0x1ab70). The
/// canonical UI subsystems (`PlaceLauncher`, `Flurry`, `UserDefaults`,
/// `CurrentPlayer`, `GoogleAnalytics`, view controllers) live in `rbx_platform`,
/// which depends on this crate, so their effects record here with matching
/// shapes: `NSUserDefaults` keys become plain cells, one-shot analytics calls
/// become counters, opaque `id` handles become `usize`. `RBX::StandardOut::printf`
/// traces surface as `eprintln!` (the `rbx_platform::AppDelegate` precedent).
pub(crate) static APP_PLACE_ID: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
pub(crate) static APP_STATE: parking_lot::Mutex<String> = parking_lot::Mutex::new(String::new());
pub(crate) static APP_WINDOW_HANDLE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
pub(crate) static APP_BG_TASK: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
pub(crate) static APP_MSG_OUT_DISCONNECTS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static FLURRY_SESSION: parking_lot::Mutex<String> = parking_lot::Mutex::new(String::new());
static FLURRY_STARTS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static APP_LOGIN_USER: parking_lot::Mutex<String> = parking_lot::Mutex::new(String::new());
static APP_LOGIN_PASS: parking_lot::Mutex<String> = parking_lot::Mutex::new(String::new());
static UPGRADE_CHECKS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static LAST_PAGE_TRACK: parking_lot::Mutex<String> = parking_lot::Mutex::new(String::new());
static PAGE_TRACK_CALLS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static DEBUG_COUNTERS_PRINTS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static SESSION_REPORTS: parking_lot::Mutex<Vec<u32>> = parking_lot::Mutex::new(Vec::new());
static COOKIE_ACCEPT_POLICY: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);
static VIEW_DISABLED_CALLS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static VIEW_ENABLED_CALLS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static LEAVE_GAME_CALLS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static START_GAME_CALLS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static LAST_START_GAME_PLACE: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static LOGIN_PLACE_ID: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static JUMP_PLACE_ID: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static TRYLAUNCH_CALLS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static TRYLAUNCH_ACTION: parking_lot::Mutex<TryLaunchAction> =
    parking_lot::Mutex::new(TryLaunchAction::Unknown);
/// Class name of the current top controller (test seam for stub_0x1a234; the
/// original queries `topMostController` + `NSStringFromClass:` at 0x1a2fc-0x1a316).
static TOP_CONTROLLER_CLASS: parking_lot::Mutex<String> = parking_lot::Mutex::new(String::new());
static SETTINGS_FETCHES: parking_lot::Mutex<Vec<(String, String)>> = parking_lot::Mutex::new(Vec::new());
static IOS_SETTINGS_READS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static MEMORY_BOUNCER_STOPS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static LOGIN_TERMINATE_CALLS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
/// `-[AppDelegate TryLaunchPlace:]` dispatch outcome (IDA 0x1a334..0x1a488),
/// mirroring `rbx_platform::LaunchAction`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum TryLaunchAction {
    LoginPlaceIdSet,
    HomeJumpTriggered,
    GameStarted,
    GameInProgressJumpSet,
    #[default]
    Unknown,
}
/// View-controller presentation registry backing `_topMostController` /
/// `topMostController` (IDA 0x1a098/0x1a124): controller id -> (presented id,
/// is-navigation-controller, visible id). Empty registry matches the bare
/// `v2 == a1 -> 0` fallthrough.
#[derive(Debug, Clone, Copy, Default)]
struct PresentedNode {
    presented: Option<usize>,
    is_nav: bool,
    visible: Option<usize>,
}
static VIEW_REGISTRY: std::sync::LazyLock<parking_lot::Mutex<std::collections::HashMap<usize, PresentedNode>>> =
    std::sync::LazyLock::new(|| parking_lot::Mutex::new(std::collections::HashMap::new()));
static KEY_WINDOW_ROOT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
/// `+[UpgradeCheckHelper checkForUpdate]` (IDA 0x1940a/0x19c0e).
pub(crate) fn app_upgrade_check() {
    UPGRADE_CHECKS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}
/// Cookie policy store behind 0x19426-0x19438 (`setCookieAcceptPolicy:0`).
pub(crate) fn app_cookie_policy() {
    COOKIE_ACCEPT_POLICY.store(0, std::sync::atomic::Ordering::SeqCst);
}
/// Login restore behind 0x1945c-0x194ce (`username`/`password` defaults into
/// CurrentPlayer). Defaults and player collapse into the same cells; the
/// round-trip documents the data flow.
pub(crate) fn app_restore_login() {
    let user = APP_LOGIN_USER.lock().clone();
    let pass = APP_LOGIN_PASS.lock().clone();
    *APP_LOGIN_USER.lock() = user;
    *APP_LOGIN_PASS.lock() = pass;
}
fn app_track_page(page: &str) {
    *LAST_PAGE_TRACK.lock() = page.to_owned();
    PAGE_TRACK_CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}
fn app_report_session(code: u32) {
    SESSION_REPORTS.lock().push(code);
}

// 0x194ec — ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke")]
pub fn stub_0x194ec() {
    // IDA 0x194ec: the didFinishLaunching Flurry block — `+[Flurry startSession:]`
    // with `FM7DNRW56339NC22K8GR` (0x1950e). Key + start count are the observables.
    *FLURRY_SESSION.lock() = "FM7DNRW56339NC22K8GR".to_owned();
    FLURRY_STARTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x19514 — ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2
// type: void __cdecl(id)
#[doc(alias = "___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2")]
pub fn stub_0x19514() {
    // IDA 0x19514: the didFinishLaunching Appirater block — setAppId `431946152`
    // (0x1953a), days 3.0 (0x19554), uses 10 (0x19568), remind 10.0 (0x19582),
    // appLaunched:YES (0x1959a). Genuine in-crate wiring into the bg_2 Appirater
    // cutovers (crate::generated_bg_2).
    crate::generated_bg_2::stub_0x17df0("431946152");
    crate::generated_bg_2::stub_0x17e00(3.0);
    crate::generated_bg_2::stub_0x17e14(10);
    crate::generated_bg_2::stub_0x17e34(10.0);
    crate::generated_bg_2::stub_0x18ca0();
}

// 0x195a0 — -[AppDelegate applicationWillResignActive:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationWillResignActive:]")]
pub fn stub_0x195a0() {
    // IDA 0x195a0: `applicationWillResignActive:` — begin/end `StandardOut`
    // traces (0x19600/0x1965e, `eprintln!` here) around
    // `disableViewBecauseGoingToBackground` (0x19640). The `sp_counted_base`
    // releases (0x19606/0x19664) are `SharedPtr` drops with no target here.
    eprintln!("AppDelegate applicationWillResignActive begin");
    VIEW_DISABLED_CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    eprintln!("AppDelegate applicationWillResignActive end");
}

// 0x196e4 — -[AppDelegate applicationDidEnterBackground:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationDidEnterBackground:]")]
pub fn stub_0x196e4() {
    // IDA 0x196e4: `applicationDidEnterBackground:` — `RobloxAppState=tryBackground`
    // + sync (0x19742-0x1975c), begin trace, `leaveGame` (0x197d4-0x197e6), drop the
    // signup keys (0x1981e-0x1985a), persist the login (0x1986a-0x198fe),
    // `reportSessionFor:1` (0x19926), page `RobloxApp/EnterBackGround` (0x1994e),
    // end trace, then // BUG: original at 0x19992 removes the state key it just
    // wrote and syncs again (0x199a4-0x199b6) — preserved as the empty state.
    *APP_STATE.lock() = "tryBackground".to_owned();
    eprintln!("AppDelegate applicationDidEnterBackground begin");
    LEAVE_GAME_CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    app_track_page("RobloxApp/EnterBackGround");
    app_report_session(1);
    eprintln!("AppDelegate applicationDidEnterBackground end");
    *APP_STATE.lock() = String::new();
}

// 0x19a30 — -[AppDelegate applicationDidReceiveMemoryWarning:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationDidReceiveMemoryWarning:]")]
pub fn stub_0x19a30() {
    // IDA 0x19a30: `applicationDidReceiveMemoryWarning:` — trace (0x19a90) then
    // `-[RobloxMemoryManager stopMemoryBouncer:0]` with a constant 0
    // (0x19ac0-0x19ad8). The bouncer lives in platform (`MemoryManager`); the
    // stop call records here.
    eprintln!("Received out of memory warning (applicationDidReceiveMemoryWarning)");
    MEMORY_BOUNCER_STOPS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x19b60 — -[AppDelegate applicationWillEnterForeground:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationWillEnterForeground:]")]
pub fn stub_0x19b60() {
    // IDA 0x19b60: `applicationWillEnterForeground:` — begin trace (0x19bc0),
    // `+[Appirater appEnteredForeground:1]` (0x19bf0, wired in-crate),
    // `checkForUpdate` (0x19c0e), page `RobloxApp/EnterForeGround` (0x19c36),
    // end trace (0x19c54).
    eprintln!("AppDelegate applicationWillEnterForeground begin");
    crate::generated_bg_2::stub_0x18e0c(true);
    app_upgrade_check();
    app_track_page("RobloxApp/EnterForeGround");
    eprintln!("AppDelegate applicationWillEnterForeground end");
}

// 0x19cdc — -[AppDelegate applicationDidBecomeActive:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationDidBecomeActive:]")]
pub fn stub_0x19cdc() {
    // IDA 0x19cdc: `applicationDidBecomeActive:` — `RobloxAppState=tryForeground`
    // + sync (0x19d3c-0x19d56), begin trace, `enableViewBecauseGoingToForeground`
    // (0x19de0), `reportSessionFor:0` (0x19e0a), settings block on a global queue
    // (0x19e14-0x19e22, stub_0x19f34), pending `appPlaceID` launches once and
    // clears (0x19e32-0x19e48, stub_0x1a234), end trace, `RobloxAppState=inApp` +
    // sync (0x19ea6-0x19eb8). Queue hop collapses to the direct call.
    *APP_STATE.lock() = "tryForeground".to_owned();
    eprintln!("AppDelegate applicationDidBecomeActive begin");
    VIEW_ENABLED_CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    app_report_session(0);
    stub_0x19f34();
    if APP_PLACE_ID.load(std::sync::atomic::Ordering::SeqCst) != 0 {
        let place = APP_PLACE_ID.swap(0, std::sync::atomic::Ordering::SeqCst);
        stub_0x1a234(place);
    }
    eprintln!("AppDelegate applicationDidBecomeActive end");
    *APP_STATE.lock() = "inApp".to_owned();
}

// 0x19f34 — ___42-[AppDelegate applicationDidBecomeActive:]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___42-[AppDelegate applicationDidBecomeActive:]_block_invoke")]
pub fn stub_0x19f34() {
    // IDA 0x19f34: the didBecomeActive settings block —
    // `ClientAppSettings::Initialize` (0x19f38), `singleton` (0x19f3c),
    // `FetchClientSettingsData("iOSAppSettings",
    // "D6925E56-BFB9-4908-AAA2-A5B1EC4B2D79")` (0x19f56),
    // `getiOSSettingsServiceWithForcedReadFromWeb:0` (0x19f78). The fetch targets
    // are platform-owned; domain/id + read count record here.
    SETTINGS_FETCHES.lock().push(("iOSAppSettings".to_owned(), "D6925E56-BFB9-4908-AAA2-A5B1EC4B2D79".to_owned()));
    IOS_SETTINGS_READS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x19f7c — -[AppDelegate applicationWillTerminate:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationWillTerminate:]")]
pub fn stub_0x19f7c() {
    // IDA 0x19f7c: `applicationWillTerminate:` — logs the game/app state values
    // (0x19fa0-0x19ff8, `eprintln!` shape), `RobloxAppState=terminated` + sync
    // (0x1a01e-0x1a038), `LoginManager applicationWillTerminate` (0x1a054-0x1a064),
    // page `RobloxApp/Exit` (0x1a092).
    *APP_STATE.lock() = "terminated".to_owned();
    LOGIN_TERMINATE_CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    app_track_page("RobloxApp/Exit");
}

// 0x1a098 — __Z18_topMostControllerP16UIViewController
// type: id __fastcall(id)
#[doc(alias = "_topMostController(UIViewController *)")]
#[doc(alias = "__Z18_topMostControllerP16UIViewController")]
pub fn stub_0x1a098(view: usize) -> usize {
    // IDA 0x1a098: `_topMostController(view)` — walk the `presentedViewController`
    // chain while non-nil (0x1a0ae-0x1a0c4); when the landing is a
    // `UINavigationController`, take `visibleViewController` if non-nil
    // (0x1a0fc-0x1a118); return 0 when the walk went nowhere (`v2 == a1`,
    // 0x1a11c-0x1a11e). The chain resolves through the presentation registry.
    let reg = VIEW_REGISTRY.lock();
    let mut top = view;
    let mut node = reg.get(&top);
    while let Some(n) = node.and_then(|n| n.presented) {
        top = n;
        node = reg.get(&top);
    }
    if let Some(n) = reg.get(&top) {
        if n.is_nav {
            if let Some(v) = n.visible {
                top = v;
            }
        }
    }
    if top == view {
        return 0;
    }
    top
}

// 0x1a124 — __Z17topMostControllerv
// type: _DWORD __fastcall()
#[doc(alias = "topMostController(void)")]
#[doc(alias = "__Z17topMostControllerv")]
pub fn stub_0x1a124() -> usize {
    // IDA 0x1a124: `topMostController()` — `keyWindow.rootViewController`
    // (0x1a140-0x1a160), then `_topMostController` until it returns 0,
    // keeping the last non-zero (0x1a164-0x1a16c).
    let mut current = KEY_WINDOW_ROOT.load(std::sync::atomic::Ordering::SeqCst);
    loop {
        let next = stub_0x1a098(current);
        if next == 0 {
            return current;
        }
        current = next;
    }
}

// 0x1a174 — -[AppDelegate application:openURL:sourceApplication:annotation:]
// type: char __cdecl(AppDelegate *self, SEL, id, id, id, id)
#[doc(alias = "-[AppDelegate application:openURL:sourceApplication:annotation:]")]
pub fn stub_0x1a174(url: &str) -> bool {
    // IDA 0x1a174: `application:openURL:...` — logs the URL parts (0x1a18a);
    // `absoluteString.hasPrefix("robloxmobile")` gates (0x1a19c-0x1a1c2, 0
    // otherwise at 0x1a1bc); on match logs host/path (0x1a1e6/0x1a208), stores
    // `host.intValue` into `appPlaceID` (0x1a22e) and returns 1 (0x1a230).
    // `NSString` handling folds into `&str`; a non-numeric host parses as 0.
    if !url.starts_with("robloxmobile") {
        return false;
    }
    let host = url.split("://").nth(1).unwrap_or_default().split('/').next().unwrap_or_default();
    APP_PLACE_ID.store(host.parse::<i32>().unwrap_or(0), std::sync::atomic::Ordering::SeqCst);
    true
}

// 0x1a234 — -[AppDelegate TryLaunchPlace:]
// type: void __cdecl(AppDelegate *self, SEL, int)
#[doc(alias = "-[AppDelegate TryLaunchPlace:]")]
pub fn stub_0x1a234(place_id: i32) {
    // IDA 0x1a234: `-TryLaunchPlace:` logs the root/top controller classes
    // (0x1a288/0x1a2f2/0x1a316) and dispatches on the top class name:
    // `LoginViewController` -> sharedInstance `setLoginPlaceId:` (0x1a334-0x1a3c2);
    // `HomeViewController` -> `setJumpToPlaceID:` + `buttonForWebDidTouchUpInside:`
    // (0x1a386-0x1a3c0); `RobloxNavBarViewController` -> PlaceLauncher
    // `startGame:controller:request:presentGameAutomatically:` (0x1a3de-0x1a42a);
    // `GameViewController` -> nav-bar `setJumpToPlaceIDGameInProgress:` (0x1a43e-0x1a47c);
    // else the unknown log (0x1a488). Class-name queries collapse into
    // TOP_CONTROLLER_CLASS; each arm records its outcome.
    TRYLAUNCH_CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    match TOP_CONTROLLER_CLASS.lock().as_str() {
        "LoginViewController" => {
            LOGIN_PLACE_ID.store(place_id, std::sync::atomic::Ordering::SeqCst);
            *TRYLAUNCH_ACTION.lock() = TryLaunchAction::LoginPlaceIdSet;
        }
        "HomeViewController" => {
            JUMP_PLACE_ID.store(place_id, std::sync::atomic::Ordering::SeqCst);
            *TRYLAUNCH_ACTION.lock() = TryLaunchAction::HomeJumpTriggered;
        }
        "RobloxNavBarViewController" => {
            LAST_START_GAME_PLACE.store(place_id, std::sync::atomic::Ordering::SeqCst);
            START_GAME_CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            *TRYLAUNCH_ACTION.lock() = TryLaunchAction::GameStarted;
        }
        "GameViewController" => {
            JUMP_PLACE_ID.store(place_id, std::sync::atomic::Ordering::SeqCst);
            *TRYLAUNCH_ACTION.lock() = TryLaunchAction::GameInProgressJumpSet;
        }
        _ => {
            *TRYLAUNCH_ACTION.lock() = TryLaunchAction::Unknown;
        }
    }
}

// 0x1a494 — -[AppDelegate bgTask]
// type: unsigned int __cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate bgTask]")]
pub fn stub_0x1a494() -> u32 {
    // IDA 0x1a494: `-bgTask` — atomic load of the ivar under a barrier
    // (0x1a4a0-0x1a4a6, `__dmb`). `SeqCst` carries the barrier.
    APP_BG_TASK.load(std::sync::atomic::Ordering::SeqCst)
}

// 0x1a4a8 — -[AppDelegate setBgTask:]
// type: void __cdecl(AppDelegate *self, SEL, unsigned int)
#[doc(alias = "-[AppDelegate setBgTask:]")]
pub fn stub_0x1a4a8(task: u32) {
    // IDA 0x1a4a8: `-setBgTask:` — barrier, ivar store, barrier
    // (0x1a4b0-0x1a4ba). `SeqCst` carries both barriers.
    APP_BG_TASK.store(task, std::sync::atomic::Ordering::SeqCst);
}

// 0x1a4c0 — -[AppDelegate window]
// type: UIWindow *__cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate window]")]
pub fn stub_0x1a4c0() -> usize {
    // IDA 0x1a4c0: `-window` returns the retained `_window` ivar (0x1a4ce).
    // The window object is platform-owned; the opaque handle crosses here.
    APP_WINDOW_HANDLE.load(std::sync::atomic::Ordering::SeqCst)
}

// 0x1a4d0 — -[AppDelegate setWindow:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate setWindow:]")]
pub fn stub_0x1a4d0(window: usize) {
    // IDA 0x1a4d0: `-setWindow:` — `objc_setProperty` retain into the ivar
    // (0x1a4ec). Retain collapses into the handle store.
    APP_WINDOW_HANDLE.store(window, std::sync::atomic::Ordering::SeqCst);
}

// 0x1a4f4 — -[AppDelegate .cxx_destruct]
// type: void __cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate .cxx_destruct]")]
pub fn stub_0x1a4f4() {
    // IDA 0x1a4f4: `-[AppDelegate .cxx_destruct]` — disconnect the
    // `messageOutConnection` scoped connection (0x1a552) and drop the weak slot
    // ref when held (0x1a558-0x1a560). The connection wraps `rbx_core` signal
    // machinery; the disconnect records here.
    APP_MSG_OUT_DISCONNECTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x1a5bc — -[AppDelegate .cxx_construct]
// type: id __cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate .cxx_construct]")]
pub fn stub_0x1a5bc() {
    // IDA 0x1a5bc: `-[AppDelegate .cxx_construct]` zeroes the connection weak
    // slot (0x1a5ca). Zero-init is the cell default; no explicit body.
}

// 0x1a5d0 — __GLOBAL__I_a_1
#[doc(alias = "__GLOBAL__I_a_1")]
pub fn stub_0x1a5d0() {
    // IDA 0x1a5d0: `__GLOBAL__I_a_1` — stores `boost::system::generic_category()` /
    // `system_category()` singletons into `__MergedGlobals_35` (disasm 0x1a5d4-0x1a5ee;
    // decompile unavailable, init thunk). Same cutover as stub_0x16e4c; no body.
}

// 0x1a768 — _main
// type: int __fastcall(int argc, const char **argv, const char **envp)
#[doc(alias = "_main")]
pub fn stub_0x1a768(_argc: i32, _argv: usize, _envp: usize) -> i32 {
    // IDA 0x1a768: `main` — autorelease pool alloc/init (0x1a788-0x1a7a0),
    // `UIApplicationMain(argc, argv, @"UIApplication", @"AppDelegate")`
    // (0x1a7ba), pool release (0x1a7ca), return its status (0x1a7d0). The host
    // runloop has no target here; the entry shape is preserved, status 0.
    0
}

// 0x1a7d4 — __GLOBAL__I_a_2
#[doc(alias = "__GLOBAL__I_a_2")]
pub fn stub_0x1a7d4() {
    // IDA 0x1a7d4: `__GLOBAL__I_a_2` — stores `boost::system::generic_category()` /
    // `system_category()` singletons into `__MergedGlobals_36` (disasm 0x1a7d8-0x1a7f2;
    // decompile unavailable, init thunk). Same cutover as stub_0x16e4c; no body.
}

// 0x1a970 — -[DebugSettingsViewController initWithCoder:]
// type: DebugSettingsViewController *__cdecl(DebugSettingsViewController *self, SEL, id)
#[doc(alias = "-[DebugSettingsViewController initWithCoder:]")]
/// `DebugSettingsViewController` ivars behind `-initWithCoder:` (IDA 0x1a970).
#[derive(Debug, Clone)]
pub(crate) struct DebugSettings {
    pub(crate) window: (f32, f32, f32, f32),
    pub(crate) keyboard_offset: i32,
    pub(crate) display_items: Vec<String>,
}
pub fn stub_0x1a970(is_pad: bool, screen_bounds: (f32, f32, f32, f32)) -> DebugSettings {
    // IDA 0x1a970: `-initWithCoder:` — super init (0x1a99c-0x1a9a0); on iPad idiom
    // (0x1a9f4) the window rect is fixed to (0, 0, 540, 508) (0x1aa1c-0x1aa1e),
    // otherwise it copies `UIScreen.mainScreen.bounds` (0x1aa4e-0x1aa76, or zero
    // with no screen at 0x1aa6e); `keyboardOffset = 114` (0x1aa7a);
    // `displayPickerArray` holds None/FPS/Summary/Physics/PhysicsAndOwner/Render
    // (0x1aaa2-0x1ab12). Screen/idiom queries collapse into parameters.
    let window = if is_pad { (0.0, 0.0, 540.0, 508.0) } else { screen_bounds };
    // The rect also lands in the `window` ivar (0x1aa1c-0x1aa76), mirrored here so
    // stub_0x1b224 (`viewWillAppear:`) can apply it to the superview bounds.
    *DEBUG_WINDOW_RECT.lock() = window;
    DebugSettings {
        window,
        keyboard_offset: 114,
        display_items: ["None", "FPS", "Summary", "Physics", "PhysicsAndOwner", "Render"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
    }
}

// 0x1ab20 — -[DebugSettingsViewController dealloc]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController dealloc]")]
pub fn stub_0x1ab20() {
    // IDA 0x1ab20: `-dealloc` — release `displayPickerArray` (0x1ab42), super
    // dealloc (0x1ab5a-0x1ab64). Drop glue covers both; no explicit body.
}

// 0x1ab6c — -[DebugSettingsViewController reloadOldData]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController reloadOldData]")]
pub fn stub_0x1ab6c() {
    // IDA 0x1ab6c: `-reloadOldData` is an empty body (decompiled 0x1ab6c).
    // No explicit body.
}

// 0x1ab70 — -[DebugSettingsViewController viewDidLoad]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController viewDidLoad]")]
pub fn stub_0x1ab70() {
    // IDA 0x1ab70: `-viewDidLoad` — super call (0x1ab8c-0x1ab96) then
    // `reloadOldData` (0x1aba8, stub_0x1ab6c, empty). Sequences the call.
    stub_0x1ab6c();
}

/// Gap-filler DebugSettings/Home observable state (IDA 0x1a970-0x1bbd0). The
/// canonical view controllers live in `rbx_platform`; view objects, animation
/// blocks and notification delivery have no runtime here, so UI effects record
/// as visibility flags/counters and pure mappings (switches, parses, counts)
/// implement fully.
static DEBUG_WINDOW_RECT: parking_lot::Mutex<(f32, f32, f32, f32)> =
    parking_lot::Mutex::new((0.0, 0.0, 0.0, 0.0));
static DEBUG_SUPERVIEW_BOUNDS: parking_lot::Mutex<(f32, f32, f32, f32)> =
    parking_lot::Mutex::new((0.0, 0.0, 0.0, 0.0));
static DEBUG_PICKER_ITEMS: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| {
    ["None", "FPS", "Summary", "Physics", "PhysicsAndOwner", "Render"]
        .iter()
        .map(|s| s.to_string())
        .collect()
});
static DISPLAY_PICKER_VISIBLE: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
static TAP_RECOGNIZER_ENABLED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
static DEBUG_FIELDS_HIDDEN: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
static VERSION_LABEL: parking_lot::Mutex<String> = parking_lot::Mutex::new(String::new());
static APP_BUNDLE_VERSION: parking_lot::Mutex<String> = parking_lot::Mutex::new(String::new());
static SEARCH_URL: parking_lot::Mutex<String> = parking_lot::Mutex::new(String::new());
static GAMES_BUTTON_HIDDEN: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(true);
static DISMISS_CALLS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static WEBVIEW_PRELOAD_CALLS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static SIGNUP_OBSERVER_ADDED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

// 0x1abb0 — -[DebugSettingsViewController setDisplayUI]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController setDisplayUI]")]
pub fn stub_0x1abb0(debug_display: u32) -> String {
    // IDA 0x1abb0: `-setDisplayUI` reads `viewWithTag:100` (0x1abd2), switches on
    // `GuiBuilder::getDebugDisplay` (0x1abe6): 1->FPS, 2->Summary, 3->Physics,
    // 4->PhysicsAndOwner, 5->Render, default->None (0x1ac02-0x1ac7c), then
    // `setText:` (0x1ac0c). View lookup + label assignment collapse (no UI); the
    // code->label mapping is the observable. Matches `displayPickerArray`.
    match debug_display {
        1 => "FPS",
        2 => "Summary",
        3 => "Physics",
        4 => "PhysicsAndOwner",
        5 => "Render",
        _ => "None",
    }
    .to_owned()
}

// 0x1ac80 — -[DebugSettingsViewController displayPickerDoneClicked:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, id)
#[doc(alias = "-[DebugSettingsViewController displayPickerDoneClicked:]")]
pub fn stub_0x1ac80() {
    // IDA 0x1ac80: `displayPickerDoneClicked:` requires `viewWithTag:5012` and
    // `:5011` non-nil (0x1acca-0x1ad0a) then runs the hide block under
    // `animateWithDuration:` (0x1ad34, stub_0x1ad78). View existence collapses;
    // the animation runs its block, so this sequences it directly.
    stub_0x1ad78();
}

// 0x1ad78 — ___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke")]
pub fn stub_0x1ad78() {
    // IDA 0x1ad78: the Done-click block reframes both tagged views to their
    // hidden geometry (0x1adba-0x1ae74: picker slides out, toolbar back). Frame
    // math has no target here; the picker-hidden outcome records.
    DISPLAY_PICKER_VISIBLE.store(false, std::sync::atomic::Ordering::SeqCst);
}

// 0x1ae78 — ___copy_helper_block__0
// type: void __fastcall(int, const void **)
#[doc(alias = "___copy_helper_block__0")]
pub fn stub_0x1ae78(_dst: usize, _src: usize) {
    // IDA 0x1ae78: `__copy_helper_block__0` — `_Block_object_assign` retain
    // (same shape as stub_0x18094). No explicit body.
}

// 0x1aea8 — ___destroy_helper_block__0
#[doc(alias = "___destroy_helper_block__0")]
pub fn stub_0x1aea8(_block: usize) {
    // IDA 0x1aea8: `__destroy_helper_block__0` — `_Block_object_dispose`
    // release (same shape as stub_0x180a0). No explicit body.
}

// 0x1aed0 — -[DebugSettingsViewController displayTouchUp:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, id)
#[doc(alias = "-[DebugSettingsViewController displayTouchUp:]")]
pub fn stub_0x1aed0() {
    // IDA 0x1aed0: `displayTouchUp:` — same non-nil guard shape as stub_0x1ac80
    // over tags 5012/5011 (0x1af1a-0x1af5c), then the show block under
    // `animateWithDuration:` (0x1af86, stub_0x1afa0). Sequences directly.
    stub_0x1afa0();
}

// 0x1afa0 — ___46-[DebugSettingsViewController displayTouchUp:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___46-[DebugSettingsViewController displayTouchUp:]_block_invoke")]
pub fn stub_0x1afa0() {
    // IDA 0x1afa0: the display-touch block reframes both tagged views to their
    // shown geometry (same frame-math shape as stub_0x1ad78, mirrored). The
    // picker-shown outcome records.
    DISPLAY_PICKER_VISIBLE.store(true, std::sync::atomic::Ordering::SeqCst);
}

// 0x1b11c — ___copy_helper_block_66
#[doc(alias = "___copy_helper_block_66")]
pub fn stub_0x1b11c(_dst: usize, _src: usize) {
    // IDA 0x1b11c: `__copy_helper_block_66` — `_Block_object_assign` retain
    // (same shape as stub_0x18094). No explicit body.
}

// 0x1b14c — ___destroy_helper_block_67
#[doc(alias = "___destroy_helper_block_67")]
pub fn stub_0x1b14c(_block: usize) {
    // IDA 0x1b14c: `__destroy_helper_block_67` — `_Block_object_dispose`
    // release (same shape as stub_0x180a0). No explicit body.
}

// 0x1b170 — -[DebugSettingsViewController didReceiveMemoryWarning]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController didReceiveMemoryWarning]")]
pub fn stub_0x1b170() {
    // IDA 0x1b170: `didReceiveMemoryWarning` is only the super call
    // (0x1b18a-0x1b194). No explicit body.
}

// 0x1b19c — -[DebugSettingsViewController shouldAutorotateToInterfaceOrientation:]
// type: char __cdecl(DebugSettingsViewController *self, SEL, int)
#[doc(alias = "-[DebugSettingsViewController shouldAutorotateToInterfaceOrientation:]")]
pub fn stub_0x1b19c(is_pad: bool, orientation: i32) -> bool {
    // IDA 0x1b19c: `shouldAutorotateToInterfaceOrientation:` — without the idiom
    // selector the answer is portrait-only (0x1b218-0x1b21c); on iPhone
    // (`idiom == 0`) likewise portrait-only (0x1b1fe-0x1b204); on iPad every
    // non-portrait orientation passes (0x1b206-0x1b212: `!(a3==1) && a3!=2`).
    // Orientation codes: 1 portrait, 2 portrait-upside-down, 3+ landscape.
    if is_pad {
        orientation != 1 && orientation != 2
    } else {
        orientation == 1
    }
}

// 0x1b224 — -[DebugSettingsViewController viewWillAppear:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, char)
#[doc(alias = "-[DebugSettingsViewController viewWillAppear:]")]
pub fn stub_0x1b224(_animated: bool) {
    // IDA 0x1b224: `viewWillAppear:` — super call (0x1b244-0x1b24e), then the
    // superview bounds take the stored window rect (0x1b260-0x1b29c). The rect
    // mirror is written by stub_0x1a970; the animated flag only feeds super.
    *DEBUG_SUPERVIEW_BOUNDS.lock() = *DEBUG_WINDOW_RECT.lock();
}

// 0x1b2a8 — -[DebugSettingsViewController doneTouchUp:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, id)
#[doc(alias = "-[DebugSettingsViewController doneTouchUp:]")]
pub fn stub_0x1b2a8() {
    // IDA 0x1b2a8: `doneTouchUp:` — `dismissViewControllerAnimated:completion:`
    // with YES/nil (0x1b2b8). No view hierarchy here; the dismissal records.
    DISMISS_CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x1b2bc — -[DebugSettingsViewController numberOfComponentsInPickerView:]
// type: int __cdecl(DebugSettingsViewController *self, SEL, id)
#[doc(alias = "-[DebugSettingsViewController numberOfComponentsInPickerView:]")]
pub fn stub_0x1b2bc() -> i32 {
    // IDA 0x1b2bc: `numberOfComponentsInPickerView:` returns 1 (0x1b2be).
    1
}

// 0x1b2c0 — -[DebugSettingsViewController pickerView:numberOfRowsInComponent:]
// type: int __cdecl(DebugSettingsViewController *self, SEL, id, int)
#[doc(alias = "-[DebugSettingsViewController pickerView:numberOfRowsInComponent:]")]
pub fn stub_0x1b2c0() -> usize {
    // IDA 0x1b2c0: `pickerView:numberOfRowsInComponent:` returns the
    // `displayPickerArray` count. Reads the shared item table.
    DEBUG_PICKER_ITEMS.len()
}

// 0x1b2e0 — -[DebugSettingsViewController pickerView:titleForRow:forComponent:]
// type: id __cdecl(DebugSettingsViewController *self, SEL, id, int, int)
#[doc(alias = "-[DebugSettingsViewController pickerView:titleForRow:forComponent:]")]
pub fn stub_0x1b2e0(row: usize) -> String {
    // IDA 0x1b2e0: `pickerView:titleForRow:forComponent:` returns
    // `[displayPickerArray objectAtIndex:row]`. `NSArray` raises on an out-of-range
    // row; indexing panics the same way.
    DEBUG_PICKER_ITEMS[row].clone()
}

// 0x1b300 — -[DebugSettingsViewController disablesAutomaticKeyboardDismissal]
// type: char __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController disablesAutomaticKeyboardDismissal]")]
pub fn stub_0x1b300() -> bool {
    // IDA 0x1b300: `disablesAutomaticKeyboardDismissal` returns 0 (0x1b302).
    false
}

// 0x1b304 — -[DebugSettingsViewController .cxx_construct]
// type: id __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController .cxx_construct]")]
pub fn stub_0x1b304() {
    // IDA 0x1b304: `-[DebugSettingsViewController .cxx_construct]` returns self
    // with no member work (0x1b304). No explicit body.
}

// 0x1b308 — __GLOBAL__I_a_3
#[doc(alias = "__GLOBAL__I_a_3")]
pub fn stub_0x1b308() {
    // IDA 0x1b308: `__GLOBAL__I_a_3` — stores `boost::system::generic_category()` /
    // `system_category()` singletons into `__MergedGlobals_37` (disasm 0x1b30c-0x1b322;
    // decompile unavailable, init thunk). Same cutover as stub_0x16e4c; no body.
}

// 0x1b3d0 — -[HomeViewController initWithCoder:]
// type: HomeViewController *__cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController initWithCoder:]")]
pub fn stub_0x1b3d0(webviews_ready: bool) {
    // IDA 0x1b3d0: `-[HomeViewController initWithCoder:]` — super init
    // (0x1b3f8-0x1b3fc); `preloadDesignatedWebViews`, falling back to
    // `designatedWebviewsToHomePages` when it reports false (0x1b41a-0x1b442);
    // registers `handleSignupNotification:` for the signup-finished notification
    // (0x1b462-0x1b4a4). The preload outcome crosses as a parameter; web views
    // and observers have no target here.
    WEBVIEW_PRELOAD_CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    if !webviews_ready {
        WEBVIEW_PRELOAD_CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    SIGNUP_OBSERVER_ADDED.store(true, std::sync::atomic::Ordering::SeqCst);
}

// 0x1b4b0 — -[HomeViewController dealloc]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController dealloc]")]
pub fn stub_0x1b4b0() {
    // IDA 0x1b4b0: `-[HomeViewController dealloc]` releases ~25 retained outlets
    // (0x1b4d4-0x1b690+) then super dealloc. Drop glue covers it; no body.
}

// 0x1b75c — -[HomeViewController viewDidLoad]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController viewDidLoad]")]
pub fn stub_0x1b75c() {
    // IDA 0x1b75c: `-viewDidLoad` — super (0x1b786), hides the debug
    // place/port/ip/launcher/settings fields (0x1b7a8-0x1b800), keyboard
    // show/hide observers (0x1ba04-0x1ba6a, delivered by the platform), version
    // label from `CFBundleVersion` (0x1ba92-0x1bad2), and the search-URL block on
    // a global queue (0x1b9e4, stub_0x1bae4). Observer delivery and hidden flags
    // record; the queue hop collapses to the direct call.
    DEBUG_FIELDS_HIDDEN.store(true, std::sync::atomic::Ordering::SeqCst);
    *VERSION_LABEL.lock() = APP_BUNDLE_VERSION.lock().clone();
    stub_0x1bae4();
}

// 0x1bae4 — ___33-[HomeViewController viewDidLoad]_block_invoke
#[doc(alias = "___33-[HomeViewController viewDidLoad]_block_invoke")]
pub fn stub_0x1bae4() {
    // IDA 0x1bae4: the viewDidLoad search block — when `[RobloxInfo searchUrl]`
    // is non-empty (0x1bb04-0x1bb14), unhide runs on the main queue
    // (0x1bb42-0x1bb5c, stub_0x1bb64). Queue hop collapses to the direct call.
    if !SEARCH_URL.lock().is_empty() {
        stub_0x1bb64();
    }
}

// 0x1bb64 — ___33-[HomeViewController viewDidLoad]_block_invoke_2
// type: id __fastcall(int)
#[doc(alias = "___33-[HomeViewController viewDidLoad]_block_invoke_2")]
pub fn stub_0x1bb64() {
    // IDA 0x1bb64: the search block body — `setHidden:0` on the ivar at +284
    // (the games button). Records the unhide.
    GAMES_BUTTON_HIDDEN.store(false, std::sync::atomic::Ordering::SeqCst);
}

// 0x1bb88 — ___copy_helper_block__1
#[doc(alias = "___copy_helper_block__1")]
pub fn stub_0x1bb88(_dst: usize, _src: usize) {
    // IDA 0x1bb88: `__copy_helper_block__1` — `_Block_object_assign` retain
    // (same shape as stub_0x18094). No explicit body.
}

// 0x1bb94 — ___destroy_helper_block__1
#[doc(alias = "___destroy_helper_block__1")]
pub fn stub_0x1bb94(_block: usize) {
    // IDA 0x1bb94: `__destroy_helper_block__1` — `_Block_object_dispose`
    // release (same shape as stub_0x180a0). No explicit body.
}

// 0x1bb9c — ___copy_helper_block_80
#[doc(alias = "___copy_helper_block_80")]
pub fn stub_0x1bb9c(_dst: usize, _src: usize) {
    // IDA 0x1bb9c: `__copy_helper_block_80` — `_Block_object_assign` retain
    // (same shape as stub_0x18094). No explicit body.
}

// 0x1bba8 — ___destroy_helper_block_81
#[doc(alias = "___destroy_helper_block_81")]
pub fn stub_0x1bba8(_block: usize) {
    // IDA 0x1bba8: `__destroy_helper_block_81` — `_Block_object_dispose`
    // release (same shape as stub_0x180a0). No explicit body.
}

// 0x1bbb0 — -[HomeViewController keyboardDidShow:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController keyboardDidShow:]")]
pub fn stub_0x1bbb0() {
    // IDA 0x1bbb0: `keyboardDidShow:` enables the tap recognizer (0x1bbcc).
    TAP_RECOGNIZER_ENABLED.store(true, std::sync::atomic::Ordering::SeqCst);
}

// 0x1bbd0 — -[HomeViewController keyboardDidHide:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController keyboardDidHide:]")]
pub fn stub_0x1bbd0() {
    // IDA 0x1bbd0: `keyboardDidHide:` disables the tap recognizer (0x1bbec).
    TAP_RECOGNIZER_ENABLED.store(false, std::sync::atomic::Ordering::SeqCst);
}

/// Gap-filler HomeViewController tail observable state (IDA 0x1bbf0-0x1cc54).
/// The canonical controllers, `UserInfo`/`LoginManager`/`PlaceLauncher`
/// models and `UIAlertView`/`UILabel` views live in `rbx_platform`/UIKit, so
/// their effects record here with matching shapes: label text becomes plain
/// cells keyed by outlet, `CurrentPlayer` fields collapse into `CURRENT_*`
/// mirrors, alerts/segues/dismissals become counters + last-value cells, and
/// `RBX::StandardOut::printf` traces surface as `eprintln!`. Block
/// retain/release traffic (`__copy_helper_block_*`/`__destroy_helper_*`) is
/// drop glue with no body.
static SEARCH_FIELD_ACTIVE: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(true);
static PLACE_FIELDS_ACTIVE: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(true);
static HOME_LABEL_TEXTS: std::sync::LazyLock<
    parking_lot::Mutex<std::collections::HashMap<String, String>>,
> = std::sync::LazyLock::new(|| parking_lot::Mutex::new(std::collections::HashMap::new()));
static CURRENT_ROBUX: parking_lot::Mutex<String> = parking_lot::Mutex::new(String::new());
static CURRENT_TIX: parking_lot::Mutex<String> = parking_lot::Mutex::new(String::new());
static CURRENT_USERNAME: parking_lot::Mutex<String> = parking_lot::Mutex::new(String::new());
static CURRENT_AVATAR_URL: parking_lot::Mutex<String> = parking_lot::Mutex::new(String::new());
static CURRENT_LOGGED_IN: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
static HOME_ROBUX_TEXT: parking_lot::Mutex<String> = parking_lot::Mutex::new(String::new());
static HOME_TIX_TEXT: parking_lot::Mutex<String> = parking_lot::Mutex::new(String::new());
static HOME_PLAYER_TEXT: parking_lot::Mutex<String> = parking_lot::Mutex::new(String::new());
static HOME_AVATAR_URL: parking_lot::Mutex<String> = parking_lot::Mutex::new(String::new());
static HOME_AVATAR_HIGHLIGHTED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(true);
static PLAYER_INFO_REFRESHES: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
static LOGGED_IN_VIEW_HIDDEN: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(true);
static NOT_LOGGED_IN_VIEW_HIDDEN: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
static PENDING_SIGNUP_LOGIN: parking_lot::Mutex<(String, String)> =
    parking_lot::Mutex::new((String::new(), String::new()));
static SIGNUP_LOGIN_CALLS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static LOGOUT_ALERTS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static LOGOUT_CONFIRMED: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static BUTTON_VIEW_ALPHA_BITS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0x3f800000);
static FOREGROUND_INITIAL_X: parking_lot::Mutex<f32> = parking_lot::Mutex::new(0.0);
static BACKGROUND_INITIAL_X: parking_lot::Mutex<f32> = parking_lot::Mutex::new(0.0);
static SEGUE_AFTER_LOAD: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
static SEGUE_REQUESTS: parking_lot::Mutex<Vec<(String, String)>> =
    parking_lot::Mutex::new(Vec::new());
static LAST_ROBLOX_ALERT: parking_lot::Mutex<String> = parking_lot::Mutex::new(String::new());
static LOCAL_LAUNCHES: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static LAST_LOCAL_LAUNCH: parking_lot::Mutex<(i32, String)> =
    parking_lot::Mutex::new((0, String::new()));
static PLACE_LAUNCHES: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static LAST_PLACE_LAUNCH: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
/// Tag -> (URL, page name) mapping behind
/// `+getUrlForButtonTag:recordPageView:query:` (IDA 0x1cc54).
pub(crate) fn home_url_parts(
    tag: i32,
    is_tablet: bool,
    base_url: &str,
    search_url: &str,
    query: &str,
) -> (String, Option<&'static str>) {
    // IDA 0x1cc54: tag switch at 0x1ccc8; tablet picks the alternate suffix
    // for tags 11/12/14/15 (0x1cd54, 0x1cda8, 0x1ce3e, 0x1ce92); tag 16
    // formats base + search + query (0x1cf10); unmatched tags fall through
    // with a nil URL and no page track.
    match tag {
        10 => (format!("{base_url}games/list"), Some("Games")),
        11 if is_tablet => (format!("{base_url}Catalog/"), Some("Catalog")),
        11 => (format!("{base_url}catalog/"), Some("Catalog")),
        12 if is_tablet => (format!("{base_url}My/Character.aspx"), Some("Inventory")),
        12 => (format!("{base_url}inventory"), Some("Inventory")),
        13 => (format!("{base_url}mobile-app-upgrades/"), Some("BuildersClub")),
        14 if is_tablet => (format!("{base_url}User.aspx"), Some("Profile")),
        14 => (format!("{base_url}"), Some("Profile")),
        15 if is_tablet => (format!("{base_url}My/Messages.aspx#Inbox"), Some("Messages")),
        15 => (format!("{base_url}inbox"), Some("Messages")),
        16 => (format!("{base_url}{search_url}{query}"), Some("Search")),
        _ => (String::new(), None),
    }
}

// 0x1bbf0 — -[HomeViewController dismissKeyboard]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController dismissKeyboard]")]
pub fn stub_0x1bbf0() {
    // IDA 0x1bbf0: `dismissKeyboard` resigns the search field (0x1bc0a).
    // First-responder state is the observable.
    SEARCH_FIELD_ACTIVE.store(false, std::sync::atomic::Ordering::SeqCst);
}

// 0x1bc10 — -[HomeViewController localizeAndStyleLabels]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController localizeAndStyleLabels]")]
pub fn stub_0x1bc10() {
    // IDA 0x1bc10: `localizeAndStyleLabels` sets each label from
    // `mainBundle localizedStringForKey:` (0x1bc48-0x1bf08). Bundle lookup
    // collapses; the slot -> localization-key assignment is the observable.
    let labels = [
        ("gameLabel", "GameWord"),
        ("catalogLabel", "CatalogWord"),
        ("inventoryLabel", "InventoryWord"),
        ("buildersClubLabel", "BuildersClubWord"),
        ("profileLabel", "ProfileWord"),
        ("messagesLabel", "MessagesWord"),
        ("communityLabel", "CommunityWord"),
        ("welcomeToRobloxTextView", "WelcomeToRoblox"),
        ("youAreCurrentlyLoggedInAsTextView", "YouAreCurrentlyLoggedInAs"),
        ("signUpButtonLabel", "SignupButton"),
        ("loginButtonLabel", "LoginButton"),
    ];
    let mut texts = HOME_LABEL_TEXTS.lock();
    for (slot, key) in labels {
        texts.insert(slot.to_owned(), key.to_owned());
    }
}

// 0x1bf0c — -[HomeViewController updateUserInfoDisplay:]
// type: void __cdecl(HomeViewController *self, SEL, bool)
#[doc(alias = "-[HomeViewController updateUserInfoDisplay:]")]
pub fn stub_0x1bf0c(update: bool) {
    // IDA 0x1bf0c: `updateUserInfoDisplay:` — with YES refreshes
    // `CurrentPlayer` via `UpdatePlayerInfo` (0x1bf18-0x1bf42); robux/tix
    // labels become `": " + value` (0x1bf70-0x1c000); the player name only
    // lands when `username` is non-nil (0x1c01a-0x1c044); the avatar only
    // fetches when `userThumbNailUrl` is non-nil (0x1c05e-0x1c0f2) and drives
    // `setHighlighted:` inverted (0x1c0fa-0x1c130). Player queries collapse
    // into the `CURRENT_*` mirrors.
    if update {
        PLAYER_INFO_REFRESHES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    *HOME_ROBUX_TEXT.lock() = format!(": {}", CURRENT_ROBUX.lock());
    *HOME_TIX_TEXT.lock() = format!(": {}", CURRENT_TIX.lock());
    let username = CURRENT_USERNAME.lock().clone();
    if !username.is_empty() {
        *HOME_PLAYER_TEXT.lock() = username;
    }
    let avatar = CURRENT_AVATAR_URL.lock().clone();
    if !avatar.is_empty() {
        // `NSURL URLWithString:` + `NSData dataWithContentsOfURL:` +
        // `UIImage imageWithData:` + `setImage:` (0x1c0a4-0x1c0f2): the fetch
        // has no target here; the resolved URL records as the image.
        *HOME_AVATAR_URL.lock() = avatar;
        HOME_AVATAR_HIGHLIGHTED.store(false, std::sync::atomic::Ordering::SeqCst);
    } else {
        HOME_AVATAR_HIGHLIGHTED.store(true, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x1c134 — -[HomeViewController viewDidUnload]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController viewDidUnload]")]
pub fn stub_0x1c134() {
    // IDA 0x1c134: `viewDidUnload` nils 18 outlets via their setters
    // (0x1c14c-0x1c290; the sign-up/login labels are each set twice,
    // 0x1c22a + 0x1c272 and 0x1c240 + 0x1c27c — clearing is idempotent) then
    // super `viewDidUnload` (0x1c2a8-0x1c2b2). Outlet release is drop glue;
    // the label cells clear.
    HOME_LABEL_TEXTS.lock().clear();
    *HOME_ROBUX_TEXT.lock() = String::new();
    *HOME_TIX_TEXT.lock() = String::new();
    *HOME_PLAYER_TEXT.lock() = String::new();
    *HOME_AVATAR_URL.lock() = String::new();
    *VERSION_LABEL.lock() = String::new();
}

// 0x1c2bc — -[HomeViewController handleSignupNotification:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController handleSignupNotification:]")]
pub fn stub_0x1c2bc(username: &str, password: &str) {
    // IDA 0x1c2bc: `handleSignupNotification:` pulls `username`/`password`
    // from the notification `userInfo` (0x1c2d8-0x1c312), retains both
    // (0x1c324-0x1c32c), and runs `LoginManager
    // doLoginWithUsername:password:` (0x1c348-0x1c35c). Retain is drop glue;
    // the credential pair + login call record.
    *PENDING_SIGNUP_LOGIN.lock() = (username.to_owned(), password.to_owned());
    SIGNUP_LOGIN_CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x1c37c — -[HomeViewController logoutTouchUp:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController logoutTouchUp:]")]
pub fn stub_0x1c37c() {
    // IDA 0x1c37c: `logoutTouchUp:` builds a `UIAlertView` titled
    // `RobloxWord` with message `LogoutConfirmation`, delegate self,
    // buttons `CancelWord`/`LogoutWord` (0x1c3a4-0x1c47e), shows it
    // (0x1c48e) and releases it (0x1c4aa). The alert request records.
    LOGOUT_ALERTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x1c4b0 — -[HomeViewController alertView:didDismissWithButtonIndex:]
// type: void __cdecl(HomeViewController *self, SEL, id, int)
#[doc(alias = "-[HomeViewController alertView:didDismissWithButtonIndex:]")]
pub fn stub_0x1c4b0(button_index: i32) {
    // IDA 0x1c4b0: `alertView:didDismissWithButtonIndex:` only acts on the
    // Logout button (index 1, 0x1c4be): `LoginManager doLogout` (0x1c4d8) +
    // `UserInfo logout` (0x1c504), then the fade/completion pair under
    // `animateWithDuration:` (0x1c58e, stub_0x1c5c8/stub_0x1c608) and page
    // track `Logout/Success` (0x1c5b4). The animation hop collapses to the
    // direct calls.
    if button_index == 1 {
        CURRENT_LOGGED_IN.store(false, std::sync::atomic::Ordering::SeqCst);
        CURRENT_USERNAME.lock().clear();
        LOGOUT_CONFIRMED.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        stub_0x1c5c8();
        stub_0x1c608(false, false, None, None);
        app_track_page("Logout/Success");
    }
}

// 0x1c5c8 — ___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke
#[doc(alias = "___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke")]
pub fn stub_0x1c5c8() {
    // IDA 0x1c5c8: the alert-dismiss animation block sets the button view
    // alpha to 0 (0x1c5da). The alpha records as raw bits.
    BUTTON_VIEW_ALPHA_BITS.store(0, std::sync::atomic::Ordering::SeqCst);
}

// 0x1c5f4 — ___copy_helper_block_224
#[doc(alias = "___copy_helper_block_224")]
pub fn stub_0x1c5f4(_dst: usize, _src: usize) {
    // IDA 0x1c5f4: `__copy_helper_block_224` — `_Block_object_assign`
    // retain (same shape as stub_0x18094). No explicit body.
}

// 0x1c600 — ___destroy_helper_block_225
#[doc(alias = "___destroy_helper_block_225")]
pub fn stub_0x1c600(_block: usize) {
    // IDA 0x1c600: `__destroy_helper_block_225` — `_Block_object_dispose`
    // release (same shape as stub_0x180a0). No explicit body.
}

// 0x1c608 — ___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke227
#[doc(alias = "___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke227")]
pub fn stub_0x1c608(
    presenting: bool,
    view_dismissed: bool,
    foreground_x: Option<f32>,
    background_x: Option<f32>,
) {
    // IDA 0x1c608: the alert-dismiss completion block — without a
    // `presentingViewController` (0x1c62c) or with the dismissed flag set
    // (0x1c63e) it skips straight to `dismissViewControllerAnimated:NO`
    // (0x1c732); otherwise it snapshots the foreground/background
    // presentation-layer frames, defaulting each to 0 when the layer is nil
    // (0x1c650-0x1c712). Layer queries collapse into parameters.
    if presenting && !view_dismissed {
        *FOREGROUND_INITIAL_X.lock() = foreground_x.unwrap_or(0.0);
        *BACKGROUND_INITIAL_X.lock() = background_x.unwrap_or(0.0);
    }
    DISMISS_CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x1c734 — ___copy_helper_block_246
#[doc(alias = "___copy_helper_block_246")]
pub fn stub_0x1c734(_dst: usize, _src: usize) {
    // IDA 0x1c734: `__copy_helper_block_246` — `_Block_object_assign`
    // retain (same shape as stub_0x18094). No explicit body.
}

// 0x1c740 — ___destroy_helper_block_247
#[doc(alias = "___destroy_helper_block_247")]
pub fn stub_0x1c740(_block: usize) {
    // IDA 0x1c740: `__destroy_helper_block_247` — `_Block_object_dispose`
    // release (same shape as stub_0x180a0). No explicit body.
}

// 0x1c748 — -[HomeViewController viewWillAppear:]
// type: void __cdecl(HomeViewController *self, SEL, char)
#[doc(alias = "-[HomeViewController viewWillAppear:]")]
pub fn stub_0x1c748(_animated: bool, user_logged_in: bool) {
    // IDA 0x1c748: `viewWillAppear:` — super `RobloxPageViewController
    // viewWillAppear:` (0x1c764-0x1c76e, no target here) then
    // `showCorrectLoggedInState` (0x1c780, stub_0x1c788). The animated flag
    // only feeds super. Sequences the call.
    stub_0x1c788(user_logged_in);
}

// 0x1c788 — -[HomeViewController showCorrectLoggedInState]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController showCorrectLoggedInState]")]
pub fn stub_0x1c788(user_logged_in: bool) {
    // IDA 0x1c788: `showCorrectLoggedInState` reads `CurrentPlayer
    // userLoggedIn` (0x1c7a8-0x1c7d2) and hides the not-logged-in view when
    // set (0x1c7e2-0x1c7f8), else the reverse (0x1c7fe-0x1c814), then
    // refreshes the info display on a global queue via the 0x1c860 block
    // (0x1c820-0x1c858, stub_0x1c860). The queue hop collapses to the direct
    // call; the login query crosses as a parameter.
    CURRENT_LOGGED_IN.store(user_logged_in, std::sync::atomic::Ordering::SeqCst);
    NOT_LOGGED_IN_VIEW_HIDDEN.store(user_logged_in, std::sync::atomic::Ordering::SeqCst);
    LOGGED_IN_VIEW_HIDDEN.store(!user_logged_in, std::sync::atomic::Ordering::SeqCst);
    stub_0x1c860();
}

// 0x1c860 — ___46-[HomeViewController showCorrectLoggedInState]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___46-[HomeViewController showCorrectLoggedInState]_block_invoke")]
pub fn stub_0x1c860() {
    // IDA 0x1c860: the logged-in-state block calls `updateUserInfoDisplay:`
    // with YES (stub_0x1bf0c). Sequences the call.
    stub_0x1bf0c(true);
}

// 0x1c874 — ___copy_helper_block_261
#[doc(alias = "___copy_helper_block_261")]
pub fn stub_0x1c874(_dst: usize, _src: usize) {
    // IDA 0x1c874: `__copy_helper_block_261` — `_Block_object_assign`
    // retain (same shape as stub_0x18094). No explicit body.
}

// 0x1c880 — ___destroy_helper_block_262
#[doc(alias = "___destroy_helper_block_262")]
pub fn stub_0x1c880(_block: usize) {
    // IDA 0x1c880: `__destroy_helper_block_262` — `_Block_object_dispose`
    // release (same shape as stub_0x180a0). No explicit body.
}

// 0x1c888 — -[HomeViewController viewDidAppear:]
// type: void __cdecl(HomeViewController *self, SEL, char)
#[doc(alias = "-[HomeViewController viewDidAppear:]")]
pub fn stub_0x1c888(_animated: bool) {
    // IDA 0x1c888: `viewDidAppear:` — super
    // `RobloxAnimatingPageViewController viewDidAppear:` (0x1c8a4-0x1c8ae,
    // no target here); when `viewMustSegueAfterLoad` is set (0x1c8c0) it
    // clears it and segues `sequeToWeb` (0x1c8cc-0x1c8e0). The animated
    // flag only feeds super.
    if SEGUE_AFTER_LOAD.swap(false, std::sync::atomic::Ordering::SeqCst) {
        SEGUE_REQUESTS
            .lock()
            .push(("sequeToWeb".to_owned(), "self".to_owned()));
    }
}

// 0x1c8e8 — -[HomeViewController handleStartGameFailure]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController handleStartGameFailure]")]
pub fn stub_0x1c8e8() {
    // IDA 0x1c8e8: `handleStartGameFailure` shows a `RobloxAlert` with the
    // `GeneralGameStartError` string (0x1c912-0x1c954). The alert key records.
    *LAST_ROBLOX_ALERT.lock() = "GeneralGameStartError".to_owned();
}

// 0x1c958 — -[HomeViewController handleStartGameSuccess]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController handleStartGameSuccess]")]
pub fn stub_0x1c958() {
    // IDA 0x1c958: `handleStartGameSuccess` is an empty body. No explicit body.
}

// 0x1c95c — -[HomeViewController placeIdClicked:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController placeIdClicked:]")]
pub fn stub_0x1c95c(place_id: i32, port: i32, ip: &str) {
    // IDA 0x1c95c: `placeIdClicked:` parses the place/port fields and reads
    // the ip field (0x1c986-0x1c9d6), resigns all three (0x1c9ea-0x1c9fe),
    // then with a non-zero port and non-empty ip starts a local game
    // (0x1ca24-0x1ca90), else starts by place id (0x1ca3e-0x1ca5a). Field
    // parsing collapses into parameters; the launch branch records.
    PLACE_FIELDS_ACTIVE.store(false, std::sync::atomic::Ordering::SeqCst);
    if port != 0 && !ip.is_empty() {
        *LAST_LOCAL_LAUNCH.lock() = (port, ip.to_owned());
        LOCAL_LAUNCHES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    } else {
        LAST_PLACE_LAUNCH.store(place_id, std::sync::atomic::Ordering::SeqCst);
        PLACE_LAUNCHES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x1ca9c — -[HomeViewController searchEditingDidEnd:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController searchEditingDidEnd:]")]
pub fn stub_0x1ca9c() {
    // IDA 0x1ca9c: `searchEditingDidEnd:` is an empty body. No explicit body.
}

// 0x1caa0 — -[HomeViewController searchDidEndOnExit:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController searchDidEndOnExit:]")]
pub fn stub_0x1caa0() {
    // IDA 0x1caa0: `searchDidEndOnExit:` segues `sequeToWeb` from the search
    // field (0x1cac4). The segue records.
    SEGUE_REQUESTS
        .lock()
        .push(("sequeToWeb".to_owned(), "searchTextField".to_owned()));
}

// 0x1cac8 — -[HomeViewController signUpButtonDidTouchUpInside:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController signUpButtonDidTouchUpInside:]")]
pub fn stub_0x1cac8() {
    // IDA 0x1cac8: `signUpButtonDidTouchUpInside:` is an empty body. No explicit body.
}

// 0x1cacc — -[HomeViewController logInButtonDidTouchUpInside:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController logInButtonDidTouchUpInside:]")]
pub fn stub_0x1cacc() {
    // IDA 0x1cacc: `logInButtonDidTouchUpInside:` dismisses animated with
    // nil completion (0x1cadc). The dismissal records.
    DISMISS_CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x1cae0 — -[HomeViewController buttonForWebDidTouchUpInside:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController buttonForWebDidTouchUpInside:]")]
pub fn stub_0x1cae0(user_logged_in: bool) {
    // IDA 0x1cae0: `buttonForWebDidTouchUpInside:` segues `sequeToWeb` when
    // `CurrentPlayer userLoggedIn` is set (0x1cb02-0x1cba6), else shows a
    // `RobloxAlert` with `YouMustLogin` (0x1cb64-0x1cb90). The login query
    // crosses as a parameter.
    if user_logged_in {
        SEGUE_REQUESTS
            .lock()
            .push(("sequeToWeb".to_owned(), "sender".to_owned()));
    } else {
        *LAST_ROBLOX_ALERT.lock() = "YouMustLogin".to_owned();
    }
}

// 0x1cbac — -[HomeViewController btnTouchPlayButtonDisabled:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController btnTouchPlayButtonDisabled:]")]
pub fn stub_0x1cbac() {
    // IDA 0x1cbac: `btnTouchPlayButtonDisabled:` shows a `RobloxAlert` with
    // `UnsupportedDevicePlayError` (0x1cbd6-0x1cc18). The alert key records.
    *LAST_ROBLOX_ALERT.lock() = "UnsupportedDevicePlayError".to_owned();
}

// 0x1cc1c — +[HomeViewController getUrlForButtonTag:recordPageView:]
// type: id __cdecl(id, SEL, int, char)
#[doc(alias = "+[HomeViewController getUrlForButtonTag:recordPageView:]")]
pub fn stub_0x1cc1c(
    tag: i32,
    record_page_view: bool,
    is_tablet: bool,
    base_url: &str,
    search_url: &str,
) -> String {
    // IDA 0x1cc1c: `+getUrlForButtonTag:recordPageView:` forwards to
    // `getUrlForButtonTag:recordPageView:query:` with an empty query
    // (0x1cc50, stub_0x1cc54). Sequences the call.
    stub_0x1cc54(tag, record_page_view, "", is_tablet, base_url, search_url)
}

// 0x1cc54 — +[HomeViewController getUrlForButtonTag:recordPageView:query:]
// type: id __cdecl(id, SEL, int, char, id)
#[doc(alias = "+[HomeViewController getUrlForButtonTag:recordPageView:query:]")]
pub fn stub_0x1cc54(
    tag: i32,
    record_page_view: bool,
    query: &str,
    is_tablet: bool,
    base_url: &str,
    search_url: &str,
) -> String {
    // IDA 0x1cc54: `+getUrlForButtonTag:recordPageView:query:` maps the tag
    // to a base URL + page name (home_url_parts), tracks the page when
    // requested (0x1cf1c-0x1cf3c), then logs `URL being returned: %s` via
    // `RBX::StandardOut::printf` (0x1cf40-0x1cfa8, `eprintln!` here). Device
    // and base-URL queries collapse into parameters; the shared_ptr
    // refcount traffic is drop glue.
    let (url, page) = home_url_parts(tag, is_tablet, base_url, search_url, query);
    if let Some(page) = page {
        if record_page_view {
            app_track_page(page);
        }
    }
    eprintln!("URL being returned: {url}");
    url
}

// 0x1cfe8 — -[HomeViewController prepareForSegue:sender:]
// type: void __cdecl(HomeViewController *self, SEL, id, id)
#[doc(alias = "-[HomeViewController prepareForSegue:sender:]")]
pub fn stub_0x1cfe8() -> ! {
    todo!("0x1cfe8 -[HomeViewController prepareForSegue:sender:]")
}

// 0x1d238 — -[HomeViewController viewMustSegueAfterLoad]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController viewMustSegueAfterLoad]")]
pub fn stub_0x1d238() -> ! {
    todo!("0x1d238 -[HomeViewController viewMustSegueAfterLoad]")
}

// 0x1d248 — -[HomeViewController setJumpToPlaceID:]
// type: void __cdecl(HomeViewController *self, SEL, int)
#[doc(alias = "-[HomeViewController setJumpToPlaceID:]")]
pub fn stub_0x1d248() -> ! {
    todo!("0x1d248 -[HomeViewController setJumpToPlaceID:]")
}

// 0x1d258 — -[HomeViewController blueFrame]
// type: UIImageView *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController blueFrame]")]
pub fn stub_0x1d258() -> ! {
    todo!("0x1d258 -[HomeViewController blueFrame]")
}

// 0x1d268 — -[HomeViewController setBlueFrame:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setBlueFrame:]")]
pub fn stub_0x1d268() -> ! {
    todo!("0x1d268 -[HomeViewController setBlueFrame:]")
}

// 0x1d28c — -[HomeViewController imgAvatar]
// type: UIImageView *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController imgAvatar]")]
pub fn stub_0x1d28c() -> ! {
    todo!("0x1d28c -[HomeViewController imgAvatar]")
}

// 0x1d29c — -[HomeViewController setImgAvatar:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setImgAvatar:]")]
pub fn stub_0x1d29c() -> ! {
    todo!("0x1d29c -[HomeViewController setImgAvatar:]")
}

// 0x1d2c0 — -[HomeViewController lblPlayerName]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController lblPlayerName]")]
pub fn stub_0x1d2c0() -> ! {
    todo!("0x1d2c0 -[HomeViewController lblPlayerName]")
}

// 0x1d2d0 — -[HomeViewController setLblPlayerName:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setLblPlayerName:]")]
pub fn stub_0x1d2d0() -> ! {
    todo!("0x1d2d0 -[HomeViewController setLblPlayerName:]")
}

// 0x1d2f4 — -[HomeViewController placeId]
// type: UITextField *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController placeId]")]
pub fn stub_0x1d2f4() -> ! {
    todo!("0x1d2f4 -[HomeViewController placeId]")
}

// 0x1d304 — -[HomeViewController setPlaceId:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setPlaceId:]")]
pub fn stub_0x1d304() -> ! {
    todo!("0x1d304 -[HomeViewController setPlaceId:]")
}

// 0x1d328 — -[HomeViewController portId]
// type: UITextField *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController portId]")]
pub fn stub_0x1d328() -> ! {
    todo!("0x1d328 -[HomeViewController portId]")
}

// 0x1d338 — -[HomeViewController setPortId:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setPortId:]")]
pub fn stub_0x1d338() -> ! {
    todo!("0x1d338 -[HomeViewController setPortId:]")
}

// 0x1d35c — -[HomeViewController ipId]
// type: UITextField *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController ipId]")]
pub fn stub_0x1d35c() -> ! {
    todo!("0x1d35c -[HomeViewController ipId]")
}
