//! Auto-generated skeletons for rbx-network — RBX::Network|RakNet filtered EA-sorted ascending
//! Filter: RakNet|Network|Replicat|Socket -> 5198 funcs (cs) / 5282 (ci), 1 remaining before batch (0xecd6e8 _TFCreateCrashSocket); filler global ascending
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +100 stubs | range 0x191d4..0x1d29c | existing 16959 -> 17059 total (filler global ascending EA-sorted, rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use crate::generated_149::{AppiraterState, stub_18b18, stub_18cc0, stub_18e0c};
/// Host shell for the iOS `AppDelegate` (IDA 0x19228..): init/dealloc and the
/// window/bg-task accessors below; UIKit objects fold, handles stay.
/// Launch/lifecycle (`didFinishLaunching`, background/foreground transitions,
/// `openURL:`, `TryLaunchPlace:`, `main`) keep the decision-relevant values:
/// registered defaults, session kinds, cookie policy, stored credentials,
/// `RobloxAppState`, view/game flags, the `appPlaceID` pending slot and the
/// Flurry/Appirater/page-view side channels as plain data.
#[derive(Debug, Default)]
pub struct AppDelegateState {
    pub window_present: bool,
    pub window: Option<String>,
    pub bg_task: u32,
    pub defaults_registered: bool,
    pub crash_reporter_init: bool,
    pub session_kind: Option<i32>,
    pub analytics_debug_printed: bool,
    pub upgrade_checks: u32,
    pub cookie_accept_policy: u32,
    pub username: Option<String>,
    pub password: Option<String>,
    pub flurry_session_key: Option<String>,
    pub app_state: String,
    pub game_state: String,
    pub view_enabled: bool,
    pub in_game: bool,
    pub pending_place_id: Option<i32>,
    pub launched_place_id: Option<i32>,
    pub login_place_id: Option<i32>,
    pub login_terminated: bool,
    pub last_page_view: Option<String>,
    pub memory_bouncer_active: bool,
    pub memory_warnings: u32,
    pub memory_warning_forwarded: u32,
    pub message_out_connected: bool,
    pub settings_fetched: bool,
}

/// Host outcome of `-[AppDelegate TryLaunchPlace:]` (IDA 0x1a234): the
/// top-controller class name selects one of four launch paths; anything else
/// logs `UnknowViewcont...` and is ignored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TryLaunchAction {
    SetLoginPlaceId(i32),
    HomeJumpAndPlay(i32),
    StartGame(i32),
    SetJumpToPlaceIdGameInProgress(i32),
    IgnoredUnknown(String),
}

/// Host node for `_topMostController`/`topMostController` (IDA 0x1a098/0x1a124):
/// UIKit pointers fold into indices; the presented/nav/visible relations stay.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewControllerNode {
    pub name: String,
    pub presented: Option<usize>,
    pub is_nav: bool,
    pub visible: Option<usize>,
}
/// Host shell for `DebugSettingsViewController` (IDA 0x1a970..0x1b304): the
/// iPad/phone window rect, the fixed six-entry display picker list, the
/// `GuiBuilder::getDebugDisplay` selection and the picker show/dismiss flags.
/// UIKit views/frames/animations fold; options, selection and applied bounds stay.
#[derive(Debug, Clone)]
pub struct DebugSettingsState {
    pub is_pad: bool,
    pub window_rect: (f32, f32, f32, f32),
    pub keyboard_offset: i32,
    pub display_options: Vec<String>,
    pub debug_display: u32,
    pub picker_visible: bool,
    pub picker_selection: i32,
    pub loaded: bool,
    pub dismissed: bool,
}

impl Default for DebugSettingsState {
    fn default() -> Self {
        Self {
            is_pad: false,
            window_rect: (0.0, 0.0, 0.0, 0.0),
            keyboard_offset: 0,
            display_options: Vec::new(),
            debug_display: 0,
            picker_visible: false,
            picker_selection: -1,
            loaded: false,
            dismissed: false,
        }
    }
}

/// `displayPickerArray` contents (IDA 0x1a970, 0x1ab06..0x1ab1a): index doubles
/// as the `GuiBuilder::setDebugDisplay` value read back in `setDisplayUI`.
pub const DEBUG_DISPLAY_OPTIONS: [&str; 6] =
    ["None", "FPS", "Summary", "Physics", "PhysicsAndOwner", "Render"];


/// Host shell for `HomeViewController` (IDA 0x1b3d0..0x1c788): webview warmup,
/// localized labels, version text, keyboard/tap flags, player-info display
/// strings, login/logout flow flags. UIKit views/alerts/animations fold;
/// decisions, texts and ids stay.
#[derive(Debug, Default, Clone)]
pub struct HomeViewState {
    pub webviews_preloaded: bool,
    pub signup_observer: bool,
    pub loaded: bool,
    pub tall_phone_bg: bool,
    pub tap_recognizer_enabled: bool,
    pub keyboard_visible: bool,
    pub keyboard_listening: bool,
    pub labels: Vec<(String, String)>,
    pub version: String,
    pub user_refreshed: bool,
    pub robux_text: String,
    pub tix_text: String,
    pub player_name: Option<String>,
    pub avatar_url: Option<String>,
    pub avatar_highlighted: bool,
    pub logged_in_view: bool,
    pub login_pending: Option<(String, String)>,
    pub logout_alert_shown: bool,
    pub logged_out: bool,
    pub logout_page_view: Option<String>,
    pub button_view_alpha_zero: bool,
    pub dismissed_no_anim: bool,
    pub search_unhidden: bool,
}

/// `localizeAndStyleLabels` keys in image order (IDA 0x1bc10..0x1bef4).
pub const HOME_LABEL_KEYS: [&str; 11] = [
    "GameWord",
    "CatalogWord",
    "InventoryWord",
    "BuildersClubWord",
    "ProfileWord",
    "MessagesWord",
    "CommunityWord",
    "WelcomeToRoblox",
    "YouAreCurrentlyLoggedInAs",
    "SignupButton",
    "LoginButton",
];


// 0x191d4 — -[Appirater ratingAlert]
// demangled: -[Appirater ratingAlert]
// type: UIAlertView *__cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater ratingAlert]")]
pub fn stub_191d4(state: &AppiraterState) -> bool {
    // IDA 0x191d4: -[Appirater ratingAlert] — returns the ratingAlert ivar (0x191e2); the host carries visibility (cf. showRatingAlert/hideRatingAlert).
        state.alert_visible}

// 0x191e4 — -[Appirater setRatingAlert:]
// demangled: -[Appirater setRatingAlert:]
// type: void __cdecl(Appirater *self, SEL, id)
#[doc(alias = "-[Appirater setRatingAlert:]")]
pub fn stub_191e4(state: &mut AppiraterState, visible: bool) {
    // IDA 0x191e4: -[Appirater setRatingAlert:] — objc_setProperty retain into the ivar (0x19200); the host stores visibility.
        state.alert_visible = visible;}

// 0x19208 — -[Appirater delegate]
// demangled: -[Appirater delegate]
// type: AppiraterDelegate *__cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater delegate]")]
pub fn stub_19208(state: &AppiraterState) -> bool {
    // IDA 0x19208: -[Appirater delegate] — returns the _delegate ivar (0x19216); the host keeps presence.
        state.has_delegate}

// 0x19218 — -[Appirater setDelegate:]
// demangled: -[Appirater setDelegate:]
// type: void __cdecl(Appirater *self, SEL, id)
#[doc(alias = "-[Appirater setDelegate:]")]
pub fn stub_19218(state: &mut AppiraterState, has_delegate: bool) {
    // IDA 0x19218: -[Appirater setDelegate:] — ivar store (0x19224); distinct from the +[Appirater setDelegate:] global at 0x17e58.
        state.has_delegate = has_delegate;}

// 0x19228 — -[AppDelegate init]
// demangled: -[AppDelegate init]
// type: AppDelegate *__cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate init]")]
pub fn stub_19228() -> AppDelegateState {
    // IDA 0x19228: -[AppDelegate init] — super init only (0x19242..0x19252); the host returns default shell state.
        AppDelegateState::default()}

// 0x19254 — -[AppDelegate dealloc]
// demangled: -[AppDelegate dealloc]
// type: void __cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate dealloc]")]
pub fn stub_19254(state: AppDelegateState) {
    // IDA 0x19254: -[AppDelegate dealloc] — analytics release (0x19276), window release (0x1928a), super dealloc (0x192ac); drops fold into Rust ownership (window handle below).
        drop(state.window); }

// 0x192b4 — -[AppDelegate application:didFinishLaunchingWithOptions:]
// demangled: -[AppDelegate application:didFinishLaunchingWithOptions:]
// type: char __cdecl(AppDelegate *self, SEL, id, id)
#[doc(alias = "-[AppDelegate application:didFinishLaunchingWithOptions:]")]
pub fn stub_192b4(state: &mut AppDelegateState, username: Option<&str>, password: Option<&str>) -> bool {
    // IDA 0x192b4: -[AppDelegate application:didFinishLaunchingWithOptions:] — registerDefaults YES/NO pair (0x192f8..0x1933c), CrashReporter + SessionReporter sharedInstance, reportSessionFor:7 (0x19356..0x19366), debugCountersPrint, two global-queue dispatches (Flurry/Appirater blocks 0x194ec/0x19514), checkForUpdate, cookie policy 0 (0x193b2..0x193c2), restore saved username/password into CurrentPlayer (0x193c6..0x194e8); UIKit/dispatch fold, stored values + YES return stay.
    state.defaults_registered = true;
    state.crash_reporter_init = true;
    state.session_kind = Some(7);
    state.analytics_debug_printed = true;
    state.upgrade_checks += 1;
    state.cookie_accept_policy = 0;
    state.username = username.map(str::to_owned);
    state.password = password.map(str::to_owned);
    true}

// 0x194ec — ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke
// demangled: ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke")]
pub fn stub_194ec(state: &mut AppDelegateState) {
    // IDA 0x194ec: didFinishLaunching block 1 — Flurry startSession:@"FM7DNRW56339NC22K8GR" (0x194f4..0x19510); the async dispatch folds, the session key stays.
    state.flurry_session_key = Some("FM7DNRW56339NC22K8GR".to_owned());}

// 0x19514 — ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2
// demangled: ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2
// type: void __cdecl(id)
#[doc(alias = "___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2")]
pub fn stub_19514(state: &mut AppiraterState, now: f64) -> bool {
    // IDA 0x19514: didFinishLaunching block 2 — setAppId:@"431946152", setDaysUntilPrompt:3.0, setUsesUntilPrompt:10, setTimeBeforeReminding:10.0, appLaunched:YES (0x1951c..0x19598); reuses the Appirater host (generated_149).
    state.app_id = "431946152".to_owned();
    state.days_until_prompt = 3.0;
    state.uses_until_prompt = 10;
    state.time_before_reminding = 10.0;
    stub_18cc0(state, true, now)}

// 0x195a0 — -[AppDelegate applicationWillResignActive:]
// demangled: -[AppDelegate applicationWillResignActive:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationWillResignActive:]")]
pub fn stub_195a0(state: &mut AppDelegateState) {
    // IDA 0x195a0: applicationWillResignActive — StandardOut begin/end logs fold, PlaceLauncher disableViewBecauseGoingToBackground (0x196a8..0x196bc) clears the view flag.
    state.view_enabled = false;}

// 0x196e4 — -[AppDelegate applicationDidEnterBackground:]
// demangled: -[AppDelegate applicationDidEnterBackground:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationDidEnterBackground:]")]
pub fn stub_196e4(state: &mut AppDelegateState, username: Option<&str>, password: Option<&str>) {
    // IDA 0x196e4: applicationDidEnterBackground — RobloxAppState:="tryBackground" + synchronize, leaveGame (0x1979e..0x197b2), drop signup keys, persist CurrentPlayer username/password, reportSessionFor:1, page "RobloxApp/EnterBackGround", then remove RobloxAppState + synchronize (0x199e2..0x19a2c); the key removals fold, end state stays.
    state.in_game = false;
    state.username = username.map(str::to_owned);
    state.password = password.map(str::to_owned);
    state.session_kind = Some(1);
    state.last_page_view = Some("RobloxApp/EnterBackGround".to_owned());
    state.app_state.clear();}

// 0x19a30 — -[AppDelegate applicationDidReceiveMemoryWarning:]
// demangled: -[AppDelegate applicationDidReceiveMemoryWarning:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationDidReceiveMemoryWarning:]")]
pub fn stub_19a30(state: &mut AppDelegateState) {
    // IDA 0x19a30: applicationDidReceiveMemoryWarning — out-of-memory log folds; stopMemoryBouncer:NO returns nonzero while the bouncer runs, and only when it returns zero does the warning forward to PlaceLauncher (0x19b34..0x19b5c).
    state.memory_warnings += 1;
    if state.memory_bouncer_active {
        state.memory_bouncer_active = false;
    } else {
        state.memory_warning_forwarded += 1;
    }}

// 0x19b60 — -[AppDelegate applicationWillEnterForeground:]
// demangled: -[AppDelegate applicationWillEnterForeground:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationWillEnterForeground:]")]
pub fn stub_19b60(state: &mut AppDelegateState, appirater: &mut AppiraterState, now: f64) -> bool {
    // IDA 0x19b60: applicationWillEnterForeground — appEnteredForeground:YES (0x19bd4..0x19be8), checkForUpdate, page "RobloxApp/EnterForeGround" (0x19bec..0x19c20); StandardOut begin/end logs fold.
    state.upgrade_checks += 1;
    state.last_page_view = Some("RobloxApp/EnterForeGround".to_owned());
    stub_18e0c(appirater, true, now)}

// 0x19cdc — -[AppDelegate applicationDidBecomeActive:]
// demangled: -[AppDelegate applicationDidBecomeActive:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationDidBecomeActive:]")]
pub fn stub_19cdc(state: &mut AppDelegateState) -> Option<i32> {
    // IDA 0x19cdc: applicationDidBecomeActive — RobloxAppState:="tryForeground" + synchronize, enableViewBecauseGoingToForeground, reportSessionFor:0, global-queue settings block (0x19f34), pending appPlaceID drains through TryLaunchPlace: (0x19e70..0x19e90) and is cleared, end log folds, RobloxAppState:="inApp" + synchronize (0x19f04..0x19f30).
    state.view_enabled = true;
    state.session_kind = Some(0);
    state.settings_fetched = true;
    state.launched_place_id = state.pending_place_id.take();
    state.app_state = "inApp".to_owned();
    state.launched_place_id}

// 0x19f34 — ___42-[AppDelegate applicationDidBecomeActive:]_block_invoke
// demangled: ___42-[AppDelegate applicationDidBecomeActive:]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___42-[AppDelegate applicationDidBecomeActive:]_block_invoke")]
pub fn stub_19f34(state: &mut AppDelegateState) {
    // IDA 0x19f34: didBecomeActive block — ClientAppSettings::Initialize + singleton, FetchClientSettingsData("iOSAppSettings","D6925E56-BFB9-4908-AAA2-A5B1EC4B2D79"), getiOSSettingsServiceWithForcedReadFromWeb:NO (0x19f3c..0x19f78); the fetch folds, the fact it ran stays.
    state.settings_fetched = true;}

// 0x19f7c — -[AppDelegate applicationWillTerminate:]
// demangled: -[AppDelegate applicationWillTerminate:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationWillTerminate:]")]
pub fn stub_19f7c(state: &mut AppDelegateState, game_state: &str) {
    // IDA 0x19f7c: applicationWillTerminate — log RobloxGameState/RobloxAppState (fold), RobloxAppState:="terminated" + synchronize (0x1a026..0x1a04e), LoginManager applicationWillTerminate (0x1a052..0x1a066), page "RobloxApp/Exit" (0x1a06a..0x1a094).
    state.game_state = game_state.to_owned();
    state.app_state = "terminated".to_owned();
    state.login_terminated = true;
    state.last_page_view = Some("RobloxApp/Exit".to_owned());}

// 0x1a098 — __Z18_topMostControllerP16UIViewController
// demangled: _topMostController(UIViewController *)
// type: id __fastcall(id)
#[doc(alias = "_topMostController(UIViewController *)")]
pub fn stub_1a098(nodes: &[ViewControllerNode], root: usize) -> Option<usize> {
    // IDA 0x1a098: _topMostController — walk presentedViewController to the tail (0x1a0a4..0x1a0b4), fold through UINavigationController.visibleViewController (0x1a0bc..0x1a0d2), return 0 when the walk never leaves the input (0x1a0d8..0x1a0e0).
    let mut top = root;
    while let Some(next) = nodes.get(top).and_then(|n| n.presented) {
        top = next;
    }
    if nodes.get(top).is_some_and(|n| n.is_nav) {
        if let Some(visible) = nodes.get(top).and_then(|n| n.visible) {
            top = visible;
        }
    }
    (top != root).then_some(top)}

// 0x1a124 — __Z17topMostControllerv
// demangled: topMostController(void)
// type: _DWORD __fastcall()
#[doc(alias = "topMostController(void)")]
pub fn stub_1a124(nodes: &[ViewControllerNode], root: usize) -> usize {
    // IDA 0x1a124: topMostController — keyWindow.rootViewController (0x1a134..0x1a148), then repeat _topMostController until it returns 0 (0x1a14c..0x1a15e); the UIApplication/UIWindow reads fold into the root index.
    let mut top = root;
    while let Some(next) = stub_1a098(nodes, top) {
        top = next;
    }
    top}

// 0x1a174 — -[AppDelegate application:openURL:sourceApplication:annotation:]
// demangled: -[AppDelegate application:openURL:sourceApplication:annotation:]
// type: char __cdecl(AppDelegate *self, SEL, id, id, id, id)
#[doc(alias = "-[AppDelegate application:openURL:sourceApplication:annotation:]")]
pub fn stub_1a174(state: &mut AppDelegateState, url: &str) -> bool {
    // IDA 0x1a174: application:openURL: — NSLog folds; absoluteString hasPrefix:@"robloxmobile" gates (0x1a196..0x1a1ac), host NSLog folds, host intValue becomes appPlaceID (0x1a1de..0x1a1ee), return YES (0x1a1f0); otherwise NO (0x1a230).
    let rest = url.strip_prefix("robloxmobile").unwrap_or("");
    if rest.is_empty() {
        return false;
    }
    let host = rest.strip_prefix("://").unwrap_or(rest).split('/').next().unwrap_or("");
    match host.parse::<i32>() {
        Ok(place_id) => {
            state.pending_place_id = Some(place_id);
            true
        }
        Err(_) => false,
    }}

// 0x1a234 — -[AppDelegate TryLaunchPlace:]
// demangled: -[AppDelegate TryLaunchPlace:]
// type: void __cdecl(AppDelegate *self, SEL, int)
#[doc(alias = "-[AppDelegate TryLaunchPlace:]")]
pub fn stub_1a234(top_controller: &str, place_id: i32) -> TryLaunchAction {
    // IDA 0x1a234: TryLaunchPlace: — root/top NSLogs fold; LoginViewController -> sharedInstance setLoginPlaceId: (0x1a2c8..0x1a2e0), HomeViewController -> setJumpToPlaceID: + buttonForWebDidTouchUpInside:0 (0x1a2e8..0x1a30c), RobloxNavBarViewController -> PlaceLauncher startGame:controller:request:presentGameAutomatically:(place, top, 0, 1) (0x1a314..0x1a348), GameViewController -> mostRecentViewController setJumpToPlaceIdGameInProgress: (0x1a350..0x1a372), else UnknowViewcont log + ignore (0x1a374..0x1a38c).
    match top_controller {
        "LoginViewController" => TryLaunchAction::SetLoginPlaceId(place_id),
        "HomeViewController" => TryLaunchAction::HomeJumpAndPlay(place_id),
        "RobloxNavBarViewController" => TryLaunchAction::StartGame(place_id),
        "GameViewController" => TryLaunchAction::SetJumpToPlaceIdGameInProgress(place_id),
        other => TryLaunchAction::IgnoredUnknown(other.to_owned()),
    }}

// 0x1a494 — -[AppDelegate bgTask]
// demangled: -[AppDelegate bgTask]
// type: unsigned int __cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate bgTask]")]
pub fn stub_1a494(state: &AppDelegateState) -> u32 {
    // IDA 0x1a494: -[AppDelegate bgTask] — dmb-guarded ivar load (0x1a498..0x1a4a0); the barrier folds, the value stays.
    state.bg_task}

// 0x1a4a8 — -[AppDelegate setBgTask:]
// demangled: -[AppDelegate setBgTask:]
// type: void __cdecl(AppDelegate *self, SEL, unsigned int)
#[doc(alias = "-[AppDelegate setBgTask:]")]
pub fn stub_1a4a8(state: &mut AppDelegateState, bg_task: u32) {
    // IDA 0x1a4a8: -[AppDelegate setBgTask:] — dmb, ivar store, dmb (0x1a4ac..0x1a4b8); barriers fold.
    state.bg_task = bg_task;}

// 0x1a4c0 — -[AppDelegate window]
// demangled: -[AppDelegate window]
// type: UIWindow *__cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate window]")]
pub fn stub_1a4c0(state: &AppDelegateState) -> Option<String> {
    // IDA 0x1a4c0: -[AppDelegate window] — _window ivar load (0x1a4c4); the handle stays as data.
    state.window.clone()}

// 0x1a4d0 — -[AppDelegate setWindow:]
// demangled: -[AppDelegate setWindow:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate setWindow:]")]
pub fn stub_1a4d0(state: &mut AppDelegateState, window: Option<String>) {
    // IDA 0x1a4d0: -[AppDelegate setWindow:] — objc_setProperty retain into _window+12 (0x1a4dc..0x1a4ec); retain folds into ownership.
    state.window_present = window.is_some();
    state.window = window;}

// 0x1a4f4 — -[AppDelegate .cxx_destruct]
// demangled: -[AppDelegate .cxx_destruct]
// type: void __cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate .cxx_destruct]")]
pub fn stub_1a4f4(state: &mut AppDelegateState) {
    // IDA 0x1a4f4: -[AppDelegate .cxx_destruct] — rbx::signals::connection::disconnect on messageOutConnection (0x1a4fa..0x1a50e) + intrusive weak release (0x1a512..0x1a520); the slot release folds into the flag.
    state.message_out_connected = false;}

// 0x1a5bc — -[AppDelegate .cxx_construct]
// demangled: -[AppDelegate .cxx_construct]
// type: id __cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate .cxx_construct]")]
pub fn stub_1a5bc(state: &mut AppDelegateState) {
    // IDA 0x1a5bc: -[AppDelegate .cxx_construct] — messageOutConnection weak_slot.p_ := 0 (0x1a5c2..0x1a5ca); the connection starts empty.
    state.message_out_connected = false;}

// 0x1a5d0 — __GLOBAL__I_a_1
// demangled: global constructor keyed to_a_1
// type: 
#[doc(alias = "global constructor keyed to_a_1")]
pub fn stub_1a5d0() {
    // IDA 0x1a5d0: __GLOBAL__I_a — boost::system generic/system category stores + std::ios_base::Init + bad_alloc exception_ptr guard (disasm 0x1a5d0..0x1a766); was: boost::system -> std::io error categories — static-init no-op shell.
}

// 0x1a768 — _main
// demangled: _main
// type: int __fastcall(int argc, const char **argv, const char **envp)
#[doc(alias = "_main")]
pub fn stub_1a768() -> i32 {
    // IDA 0x1a768: _main — NSAutoreleasePool alloc/init (0x1a772..0x1a782), UIApplicationMain(argc, argv, @"Uiapplication", @"AppDelegate") (0x1a786..0x1a7a2), pool release (0x1a7a6..0x1a7ae); the runloop never returns in-image, the host returns the zero exit code.
    0}

// 0x1a7d4 — __GLOBAL__I_a_2
// demangled: global constructor keyed to_a_2
// type: 
#[doc(alias = "global constructor keyed to_a_2")]
pub fn stub_1a7d4() {
    // IDA 0x1a7d4: __GLOBAL__I_a — same static-init shape as 0x1a5d0 (disasm 0x1a7d4..0x1a96a: generic/system categories, ios_base::Init, exception_ptr guard); was: boost::system -> std::io — static-init no-op shell.
}

// 0x1a970 — -[DebugSettingsViewController initWithCoder:]
// demangled: -[DebugSettingsViewController initWithCoder:]
// type: DebugSettingsViewController *__cdecl(DebugSettingsViewController *self, SEL, id)
#[doc(alias = "-[DebugSettingsViewController initWithCoder:]")]
pub fn stub_1a970(is_pad: bool, screen_bounds: Option<(f32, f32, f32, f32)>) -> DebugSettingsState {
    // IDA 0x1a970: initWithCoder — super init (0x1a9a0..0x1a9ac); iPad idiom (0x1a9c6..0x1a9e2) takes window (0,0,540,508), otherwise mainScreen bounds (0x1aa5c..0x1aa9c); keyboardOffset := 114, displayPickerArray := the six debug-display names (0x1aadc..0x1ab1a).
    let window_rect = if is_pad { (0.0, 0.0, 540.0, 508.0) } else { screen_bounds.unwrap_or((0.0, 0.0, 0.0, 0.0)) };
    DebugSettingsState {
        is_pad,
        window_rect,
        keyboard_offset: 114,
        display_options: DEBUG_DISPLAY_OPTIONS.iter().map(|s| (*s).to_owned()).collect(),
        ..DebugSettingsState::default()
    }}

// 0x1ab20 — -[DebugSettingsViewController dealloc]
// demangled: -[DebugSettingsViewController dealloc]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController dealloc]")]
pub fn stub_1ab20(state: DebugSettingsState) {
    // IDA 0x1ab20: dealloc — displayPickerArray release (0x1ab2c..0x1ab40) then super dealloc; drops fold into Rust ownership.
    drop(state);}

// 0x1ab6c — -[DebugSettingsViewController reloadOldData]
// demangled: -[DebugSettingsViewController reloadOldData]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController reloadOldData]")]
pub fn stub_1ab6c() {
    // IDA 0x1ab6c: reloadOldData — empty body (disasm: BX LR); faithful no-op shell.
}

// 0x1ab70 — -[DebugSettingsViewController viewDidLoad]
// demangled: -[DebugSettingsViewController viewDidLoad]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController viewDidLoad]")]
pub fn stub_1ab70(state: &mut DebugSettingsState) {
    // IDA 0x1ab70: viewDidLoad — super viewDidLoad + reloadOldData (0x1ab84..0x1ab98); reloadOldData is empty, the load flag stays.
    stub_1ab6c();
    state.loaded = true;}

// 0x1abb0 — -[DebugSettingsViewController setDisplayUI]
// demangled: -[DebugSettingsViewController setDisplayUI]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController setDisplayUI]")]
pub fn stub_1abb0(state: &DebugSettingsState) -> &'static str {
    // IDA 0x1abb0: setDisplayUI — viewWithTag:100 (0x1abc4..0x1abd4), GuiBuilder::getDebugDisplay switch 1..5 -> FPS/Summary/Physics/PhysicsAndOwner/Render else None (0x1abdc..0x1ac5c), setText: (0x1ac60..0x1ac74); the view lookup folds, the selected label stays observable.
    match state.debug_display {
        1 => "FPS",
        2 => "Summary",
        3 => "Physics",
        4 => "PhysicsAndOwner",
        5 => "Render",
        _ => "None",
    }}

// 0x1ac80 — -[DebugSettingsViewController displayPickerDoneClicked:]
// demangled: -[DebugSettingsViewController displayPickerDoneClicked:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, id)
#[doc(alias = "-[DebugSettingsViewController displayPickerDoneClicked:]")]
pub fn stub_1ac80(state: &mut DebugSettingsState) {
    // IDA 0x1ac80: displayPickerDoneClicked — both tag-5012/5011 views required (0x1acb0..0x1acc8), animateWithDuration block 0x1ad78 (folds), selectedRowInComponent:0 >= 0 applies GuiBuilder::setDebugDisplay (0x1ad20..0x1ad48), then setDisplayUI (0x1ad5c..0x1ad70); the picker dismisses.
    if state.picker_visible {
        if state.picker_selection >= 0 {
            state.debug_display = (state.picker_selection as u32).min(5);
        }
        state.picker_visible = false;
    }
    let _ = stub_1abb0(state);}

// 0x1ad78 — ___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke
// demangled: ___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke")]
pub fn stub_1ad78() {
    // IDA 0x1ad78: displayPickerDoneClicked block — pure UIView setFrame arithmetic over the two captured views (frames/origins fold on the host); faithful no-op shell.
}

// 0x1ae78 — ___copy_helper_block__0
// demangled: ___copy_helper_block__0
// type: void __fastcall(int, const void **)
#[doc(alias = "___copy_helper_block__0")]
pub fn stub_1ae78() {
    // IDA 0x1ae78: __copy_helper_block — three _Block_object_assign slots (cf. 0x18c98 pattern); block retain has no host carrier — faithful no-op shell.
}

// 0x1aea8 — ___destroy_helper_block__0
// demangled: ___destroy_helper_block__0
// type: 
#[doc(alias = "___destroy_helper_block__0")]
pub fn stub_1aea8() {
    // IDA 0x1aea8: __destroy_helper_block — three _Block_object_dispose slots; block release has no host carrier — faithful no-op shell.
}

// 0x1aed0 — -[DebugSettingsViewController displayTouchUp:]
// demangled: -[DebugSettingsViewController displayTouchUp:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, id)
#[doc(alias = "-[DebugSettingsViewController displayTouchUp:]")]
pub fn stub_1aed0(state: &mut DebugSettingsState) {
    // IDA 0x1aed0: displayTouchUp — both tag-5012/5011 views required (0x1af00..0x1af18), animateWithDuration block 0x1afa0 (folds); the tap presents the picker.
    state.picker_visible = true;}

// 0x1afa0 — ___46-[DebugSettingsViewController displayTouchUp:]_block_invoke
// demangled: ___46-[DebugSettingsViewController displayTouchUp:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___46-[DebugSettingsViewController displayTouchUp:]_block_invoke")]
pub fn stub_1afa0() {
    // IDA 0x1afa0: displayTouchUp block — pure UIView setFrame arithmetic over the two captured views (frames fold on the host); faithful no-op shell.
}

// 0x1b11c — ___copy_helper_block_66
// demangled: ___copy_helper_block_66
// type: 
#[doc(alias = "___copy_helper_block_66")]
pub fn stub_1b11c() {
    // IDA 0x1b11c: __copy_helper_block — three _Block_object_assign slots; block retain has no host carrier — faithful no-op shell.
}

// 0x1b14c — ___destroy_helper_block_67
// demangled: ___destroy_helper_block_67
// type: 
#[doc(alias = "___destroy_helper_block_67")]
pub fn stub_1b14c() {
    // IDA 0x1b14c: __destroy_helper_block — three _Block_object_dispose slots; block release has no host carrier — faithful no-op shell.
}

// 0x1b170 — -[DebugSettingsViewController didReceiveMemoryWarning]
// demangled: -[DebugSettingsViewController didReceiveMemoryWarning]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController didReceiveMemoryWarning]")]
pub fn stub_1b170() {
    // IDA 0x1b170: didReceiveMemoryWarning — super only (0x1b17c..0x1b190); faithful no-op shell.
}

// 0x1b19c — -[DebugSettingsViewController shouldAutorotateToInterfaceOrientation:]
// demangled: -[DebugSettingsViewController shouldAutorotateToInterfaceOrientation:]
// type: char __cdecl(DebugSettingsViewController *self, SEL, int)
#[doc(alias = "-[DebugSettingsViewController shouldAutorotateToInterfaceOrientation:]")]
pub fn stub_1b19c(is_pad: bool, orientation: i32) -> bool {
    // IDA 0x1b19c: shouldAutorotateToInterfaceOrientation — non-idiom devices (0x1bb0..0x1bbc) and phones (0x1bc8..0x1bd6) allow portrait(1) only; iPads (0x1bd8..0x1bf0) reject 1 and 2, allow the rest; falls through to 0 otherwise.
    if is_pad {
        orientation != 1 && orientation != 2
    } else {
        orientation == 1
    }}

// 0x1b224 — -[DebugSettingsViewController viewWillAppear:]
// demangled: -[DebugSettingsViewController viewWillAppear:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, char)
#[doc(alias = "-[DebugSettingsViewController viewWillAppear:]")]
pub fn stub_1b224(state: &DebugSettingsState) -> (f32, f32, f32, f32) {
    // IDA 0x1b224: viewWillAppear — super (0x1b234..0x1b244), superview setBounds: := stored window rect (0x1b248..0x1b26c); the applied bounds stay observable.
    state.window_rect}

// 0x1b2a8 — -[DebugSettingsViewController doneTouchUp:]
// demangled: -[DebugSettingsViewController doneTouchUp:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, id)
#[doc(alias = "-[DebugSettingsViewController doneTouchUp:]")]
pub fn stub_1b2a8(state: &mut DebugSettingsState) {
    // IDA 0x1b2a8: doneTouchUp — dismissViewControllerAnimated:YES completion:nil (0x1b2b0..0x1b2b8).
    state.dismissed = true;
    state.picker_visible = false;}

// 0x1b2bc — -[DebugSettingsViewController numberOfComponentsInPickerView:]
// demangled: -[DebugSettingsViewController numberOfComponentsInPickerView:]
// type: int __cdecl(DebugSettingsViewController *self, SEL, id)
#[doc(alias = "-[DebugSettingsViewController numberOfComponentsInPickerView:]")]
pub fn stub_1b2bc() -> i32 {
    // IDA 0x1b2bc: numberOfComponentsInPickerView — returns 1 (0x1b2c0).
    1}

// 0x1b2c0 — -[DebugSettingsViewController pickerView:numberOfRowsInComponent:]
// demangled: -[DebugSettingsViewController pickerView:numberOfRowsInComponent:]
// type: int __cdecl(DebugSettingsViewController *self, SEL, id, int)
#[doc(alias = "-[DebugSettingsViewController pickerView:numberOfRowsInComponent:]")]
pub fn stub_1b2c0(state: &DebugSettingsState) -> i32 {
    // IDA 0x1b2c0: pickerView:numberOfRowsInComponent — displayPickerArray count (0x1b2cc..0x1b2dc), component index unused.
    state.display_options.len() as i32}

// 0x1b2e0 — -[DebugSettingsViewController pickerView:titleForRow:forComponent:]
// demangled: -[DebugSettingsViewController pickerView:titleForRow:forComponent:]
// type: id __cdecl(DebugSettingsViewController *self, SEL, id, int, int)
#[doc(alias = "-[DebugSettingsViewController pickerView:titleForRow:forComponent:]")]
pub fn stub_1b2e0(state: &DebugSettingsState, row: i32) -> Option<String> {
    // IDA 0x1b2e0: pickerView:titleForRow — displayPickerArray objectAtIndex:row (0x1b2ec..0x1b2fc), component index unused; out-of-range has no image path (picker row count gates), host returns None.
    usize::try_from(row).ok().and_then(|i| state.display_options.get(i).cloned())}

// 0x1b300 — -[DebugSettingsViewController disablesAutomaticKeyboardDismissal]
// demangled: -[DebugSettingsViewController disablesAutomaticKeyboardDismissal]
// type: char __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController disablesAutomaticKeyboardDismissal]")]
pub fn stub_1b300() -> bool {
    // IDA 0x1b300: disablesAutomaticKeyboardDismissal — returns NO (0x1b300).
    false}

// 0x1b304 — -[DebugSettingsViewController .cxx_construct]
// demangled: -[DebugSettingsViewController .cxx_construct]
// type: id __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController .cxx_construct]")]
pub fn stub_1b304() {
    // IDA 0x1b304: .cxx_construct — returns self, no member init (disasm: MOV R0,R4; BX LR equivalent); faithful no-op shell.
}

// 0x1b308 — __GLOBAL__I_a_3
// demangled: global constructor keyed to_a_3
// type: 
#[doc(alias = "global constructor keyed to_a_3")]
pub fn stub_1b308() {
    // IDA 0x1b308: __GLOBAL__I_a — same static-init shape as 0x1a5d0 (disasm 0x1b308..: generic/system categories, ios_base::Init, exception_ptr guard); was: boost::system -> std::io — static-init no-op shell.
}

// 0x1b3d0 — -[HomeViewController initWithCoder:]
// demangled: -[HomeViewController initWithCoder:]
// type: HomeViewController *__cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController initWithCoder:]")]
pub fn stub_1b3d0() -> HomeViewState {
    // IDA 0x1b3d0: initWithCoder — super RobloxAnimatingPageViewController init (0x1b3f8..0x1b408); UIWebViewCacheManager preloadDesignatedWebViews else designatedWebviewsToHomePages (0x1b41c..0x1b43c); observe SignupViewController signup-finished -> handleSignupNotification: (0x1b440..0x1b470).
    HomeViewState { webviews_preloaded: true, signup_observer: true, ..HomeViewState::default() }}

// 0x1b4b0 — -[HomeViewController dealloc]
// demangled: -[HomeViewController dealloc]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController dealloc]")]
pub fn stub_1b4b0(state: HomeViewState) {
    // IDA 0x1b4b0: dealloc — thirty outlet releases (0x1b4bc..0x1b6ec) then super dealloc; drops fold into Rust ownership.
    drop(state);}

// 0x1b75c — -[HomeViewController viewDidLoad]
// demangled: -[HomeViewController viewDidLoad]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController viewDidLoad]")]
pub fn stub_1b75c(state: &mut HomeViewState, is_phone: bool, screen_h: f32, bundle_version: &str, search_url: &str) {
    // IDA 0x1b75c: viewDidLoad — super (0x1b780..0x1b790), hide debug fields/buttons (0x1b794..0x1b7dc), phone 568h blueFrame image (0x1b7e0..0x1b8a4), tap recognizer init disabled + added (0x1ba8..0x1baf0), localizeAndStyleLabels + updateUserInfoDisplay:NO (0x1baf4..0x1bb0c), global-queue searchUrl block 0x1bae4 (0x1bb10..0x1bb3c), keyboard show/hide observers (0x1bb40..0x1bb8c), versionLabel := CFBundleVersion (0x1bb90..0x1bbd6).
    state.loaded = true;
    state.tall_phone_bg = is_phone && screen_h == 568.0;
    state.tap_recognizer_enabled = false;
    stub_1bc10(state);
    stub_1bf0c(state, false, "", "", None, None);
    state.search_unhidden = stub_1bae4(search_url);
    state.keyboard_listening = true;
    state.version = bundle_version.to_owned();}

// 0x1bae4 — ___33-[HomeViewController viewDidLoad]_block_invoke
// demangled: ___33-[HomeViewController viewDidLoad]_block_invoke
// type: 
#[doc(alias = "___33-[HomeViewController viewDidLoad]_block_invoke")]
pub fn stub_1bae4(search_url: &str) -> bool {
    // IDA 0x1bae4: viewDidLoad block — RobloxInfo searchUrl length gates a main-queue dispatch of block 0x1bb64 (0x1baf0..0x1bb18); the queues fold, the gate stays.
    !search_url.is_empty()}

// 0x1bb64 — ___33-[HomeViewController viewDidLoad]_block_invoke_2
// demangled: ___33-[HomeViewController viewDidLoad]_block_invoke_2
// type: id __fastcall(int)
#[doc(alias = "___33-[HomeViewController viewDidLoad]_block_invoke_2")]
pub fn stub_1bb64(state: &mut HomeViewState) {
    // IDA 0x1bb64: viewDidLoad block 2 — outlet+284 setHidden:NO (0x1bb6c..0x1bb7c); the outlet folds into the flag.
    state.search_unhidden = true;}

// 0x1bb88 — ___copy_helper_block__1
// demangled: ___copy_helper_block__1
// type: 
#[doc(alias = "___copy_helper_block__1")]
pub fn stub_1bb88() {
    // IDA 0x1bb88: __copy_helper_block — single _Block_object_assign slot; block retain has no host carrier — faithful no-op shell.
}

// 0x1bb94 — ___destroy_helper_block__1
// demangled: ___destroy_helper_block__1
// type: 
#[doc(alias = "___destroy_helper_block__1")]
pub fn stub_1bb94() {
    // IDA 0x1bb94: __destroy_helper_block — single _Block_object_dispose slot; block release has no host carrier — faithful no-op shell.
}

// 0x1bb9c — ___copy_helper_block_80
// demangled: ___copy_helper_block_80
// type: 
#[doc(alias = "___copy_helper_block_80")]
pub fn stub_1bb9c() {
    // IDA 0x1bb9c: __copy_helper_block — single _Block_object_assign slot; block retain has no host carrier — faithful no-op shell.
}

// 0x1bba8 — ___destroy_helper_block_81
// demangled: ___destroy_helper_block_81
// type: 
#[doc(alias = "___destroy_helper_block_81")]
pub fn stub_1bba8() {
    // IDA 0x1bba8: __destroy_helper_block — single _Block_object_dispose slot; block release has no host carrier — faithful no-op shell.
}

// 0x1bbb0 — -[HomeViewController keyboardDidShow:]
// demangled: -[HomeViewController keyboardDidShow:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController keyboardDidShow:]")]
pub fn stub_1bbb0(state: &mut HomeViewState) {
    // IDA 0x1bbb0: keyboardDidShow — tapRecognizer setEnabled:YES (0x1bbb8..0x1bbc8).
    state.tap_recognizer_enabled = true;
    state.keyboard_visible = true;}

// 0x1bbd0 — -[HomeViewController keyboardDidHide:]
// demangled: -[HomeViewController keyboardDidHide:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController keyboardDidHide:]")]
pub fn stub_1bbd0(state: &mut HomeViewState) {
    // IDA 0x1bbd0: keyboardDidHide — tapRecognizer setEnabled:NO (0x1bbd8..0x1bbe8).
    state.tap_recognizer_enabled = false;
    state.keyboard_visible = false;}

// 0x1bbf0 — -[HomeViewController dismissKeyboard]
// demangled: -[HomeViewController dismissKeyboard]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController dismissKeyboard]")]
pub fn stub_1bbf0(state: &mut HomeViewState) {
    // IDA 0x1bbf0: dismissKeyboard — searchTextField resignFirstResponder (0x1bbf8..0x1bc08); the field folds, the keyboard hides.
    state.keyboard_visible = false;}

// 0x1bc10 — -[HomeViewController localizeAndStyleLabels]
// demangled: -[HomeViewController localizeAndStyleLabels]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController localizeAndStyleLabels]")]
pub fn stub_1bc10(state: &mut HomeViewState) {
    // IDA 0x1bc10: localizeAndStyleLabels — eleven mainBundle localizedStringForKey: -> setText: pairs (0x1bc30..0x1bef4, HOME_LABEL_KEYS); the bundle folds, keys stay as values.
    state.labels = HOME_LABEL_KEYS.iter().map(|k| ((*k).to_owned(), (*k).to_owned())).collect();}

// 0x1bf0c — -[HomeViewController updateUserInfoDisplay:]
// demangled: -[HomeViewController updateUserInfoDisplay:]
// type: void __cdecl(HomeViewController *self, SEL, bool)
#[doc(alias = "-[HomeViewController updateUserInfoDisplay:]")]
pub fn stub_1bf0c(state: &mut HomeViewState, refresh: bool, robux: &str, tix: &str, username: Option<&str>, thumb_url: Option<&str>) {
    // IDA 0x1bf0c: updateUserInfoDisplay — refresh gates CurrentPlayer UpdatePlayerInfo (0x1bf24..0x1bf3c); lblRobux/lblTix := ": "+Robux/Tix (0x1bf40..0x1bf9c); username present gates lblPlayerName (0x1bfa0..0x1bfdc); thumbnail URL present gates synchronous avatar fetch + setImage, else setHighlighted:YES (0x1bfe0..0x1c108).
    state.user_refreshed = refresh;
    state.robux_text = format!(": {robux}");
    state.tix_text = format!(": {tix}");
    if let Some(name) = username {
        state.player_name = Some(name.to_owned());
    }
    match thumb_url {
        Some(url) => {
            state.avatar_url = Some(url.to_owned());
            state.avatar_highlighted = false;
        }
        None => {
            state.avatar_highlighted = true;
        }
    }}

// 0x1c134 — -[HomeViewController viewDidUnload]
// demangled: -[HomeViewController viewDidUnload]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController viewDidUnload]")]
pub fn stub_1c134(state: &mut HomeViewState) {
    // IDA 0x1c134: viewDidUnload — seventeen outlet setters := nil (0x1c140..0x1c220) then super; outlets fold into cleared host fields.
    state.labels.clear();
    state.version.clear();
    state.robux_text.clear();
    state.tix_text.clear();
    state.player_name = None;
    state.avatar_url = None;
    state.loaded = false;}

// 0x1c2bc — -[HomeViewController handleSignupNotification:]
// demangled: -[HomeViewController handleSignupNotification:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController handleSignupNotification:]")]
pub fn stub_1c2bc(state: &mut HomeViewState, username: &str, password: &str, user_logged_in: bool) {
    // IDA 0x1c2bc: handleSignupNotification — userInfo username/password retain (0x1c2d4..0x1c2f4), LoginManager doLoginWithUsername:password: (0x1c2f8..0x1c30c), showCorrectLoggedInState (0x1c310..0x1c324); retains fold, credentials + login view stay.
    state.login_pending = Some((username.to_owned(), password.to_owned()));
    stub_1c788(state, user_logged_in);}

// 0x1c37c — -[HomeViewController logoutTouchUp:]
// demangled: -[HomeViewController logoutTouchUp:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController logoutTouchUp:]")]
pub fn stub_1c37c(state: &mut HomeViewState) {
    // IDA 0x1c37c: logoutTouchUp — UIAlertView RobloxWord/LogoutConfirmation/CancelWord/LogoutWord, delegate self, show + release (0x1c384..0x1c49c); the alert folds, its presentation stays.
    state.logout_alert_shown = true;}

// 0x1c4b0 — -[HomeViewController alertView:didDismissWithButtonIndex:]
// demangled: -[HomeViewController alertView:didDismissWithButtonIndex:]
// type: void __cdecl(HomeViewController *self, SEL, id, int)
#[doc(alias = "-[HomeViewController alertView:didDismissWithButtonIndex:]")]
pub fn stub_1c4b0(state: &mut HomeViewState, button: i32) {
    // IDA 0x1c4b0: alertView:didDismissWithButtonIndex — button 1 only: LoginManager doLogout + UserInfo logout (0x1c4c4..0x1c4dc), UIView animations/completion blocks 0x1c5c8/0x1c608 (0x1c4e0..0x1c520), page "Logout/Success" (0x1c524..0x1c552); button 0 (cancel) is a no-op.
    if button == 1 {
        state.logged_out = true;
        state.login_pending = None;
        state.player_name = None;
        state.avatar_url = None;
        stub_1c5c8(state);
        stub_1c608(state);
        state.logout_page_view = Some("Logout/Success".to_owned());
    }}

// 0x1c5c8 — ___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke
// demangled: ___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke
// type: 
#[doc(alias = "___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke")]
pub fn stub_1c5c8(state: &mut HomeViewState) {
    // IDA 0x1c5c8: logout animation block — buttonView setAlpha:0 (0x1c5d0..0x1c5e4); the view folds into the flag.
    state.button_view_alpha_zero = true;}

// 0x1c5f4 — ___copy_helper_block_224
// demangled: ___copy_helper_block_224
// type: 
#[doc(alias = "___copy_helper_block_224")]
pub fn stub_1c5f4() {
    // IDA 0x1c5f4: __copy_helper_block — single _Block_object_assign slot; block retain has no host carrier — faithful no-op shell.
}

// 0x1c600 — ___destroy_helper_block_225
// demangled: ___destroy_helper_block_225
// type: 
#[doc(alias = "___destroy_helper_block_225")]
pub fn stub_1c600() {
    // IDA 0x1c600: __destroy_helper_block — single _Block_object_dispose slot; block release has no host carrier — faithful no-op shell.
}

// 0x1c608 — ___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke227
// demangled: ___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke227
// type: 
#[doc(alias = "___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke227")]
pub fn stub_1c608(state: &mut HomeViewState) {
    // IDA 0x1c608: logout completion block — presentingViewController gate + foreground/background initial-X capture (0x1c618..0x1c6f4) fold, then dismissViewControllerAnimated:NO (0x1c6f8..0x1c710).
    state.dismissed_no_anim = true;}

// 0x1c734 — ___copy_helper_block_246
// demangled: ___copy_helper_block_246
// type: 
#[doc(alias = "___copy_helper_block_246")]
pub fn stub_1c734() {
    // IDA 0x1c734: __copy_helper_block — single _Block_object_assign slot; block retain has no host carrier — faithful no-op shell.
}

// 0x1c740 — ___destroy_helper_block_247
// demangled: ___destroy_helper_block_247
// type: 
#[doc(alias = "___destroy_helper_block_247")]
pub fn stub_1c740() {
    // IDA 0x1c740: __destroy_helper_block — single _Block_object_dispose slot; block release has no host carrier — faithful no-op shell.
}

// 0x1c748 — -[HomeViewController viewWillAppear:]
// demangled: -[HomeViewController viewWillAppear:]
// type: void __cdecl(HomeViewController *self, SEL, char)
#[doc(alias = "-[HomeViewController viewWillAppear:]")]
pub fn stub_1c748(state: &mut HomeViewState, user_logged_in: bool) {
    // IDA 0x1c748: viewWillAppear — super RobloxPageViewController viewWillAppear: + showCorrectLoggedInState (0x1c754..0x1c76c).
    stub_1c788(state, user_logged_in);}

// 0x1c788 — -[HomeViewController showCorrectLoggedInState]
// demangled: -[HomeViewController showCorrectLoggedInState]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController showCorrectLoggedInState]")]
pub fn stub_1c788(state: &mut HomeViewState, user_logged_in: bool) {
    // IDA 0x1c788: showCorrectLoggedInState — userLoggedIn hides notLoggedInView XOR loggedInView (0x1c798..0x1c7d4), then global-queue refresh block 0x1c860 (0x1c7d8..0x1c800, folds); the views fold into the flag.
    state.logged_in_view = user_logged_in;}

// 0x1c860 — ___46-[HomeViewController showCorrectLoggedInState]_block_invoke
// demangled: ___46-[HomeViewController showCorrectLoggedInState]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___46-[HomeViewController showCorrectLoggedInState]_block_invoke")]
pub fn stub_1c860() -> ! {
    todo!("0x1c860 ___46-[HomeViewController showCorrectLoggedInState]_block_invoke")
}

// 0x1c874 — ___copy_helper_block_261
// demangled: ___copy_helper_block_261
// type: 
#[doc(alias = "___copy_helper_block_261")]
pub fn stub_1c874() -> ! {
    todo!("0x1c874 ___copy_helper_block_261")
}

// 0x1c880 — ___destroy_helper_block_262
// demangled: ___destroy_helper_block_262
// type: 
#[doc(alias = "___destroy_helper_block_262")]
pub fn stub_1c880() -> ! {
    todo!("0x1c880 ___destroy_helper_block_262")
}

// 0x1c888 — -[HomeViewController viewDidAppear:]
// demangled: -[HomeViewController viewDidAppear:]
// type: void __cdecl(HomeViewController *self, SEL, char)
#[doc(alias = "-[HomeViewController viewDidAppear:]")]
pub fn stub_1c888() -> ! {
    todo!("0x1c888 -[HomeViewController viewDidAppear:]")
}

// 0x1c8e8 — -[HomeViewController handleStartGameFailure]
// demangled: -[HomeViewController handleStartGameFailure]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController handleStartGameFailure]")]
pub fn stub_1c8e8() -> ! {
    todo!("0x1c8e8 -[HomeViewController handleStartGameFailure]")
}

// 0x1c958 — -[HomeViewController handleStartGameSuccess]
// demangled: -[HomeViewController handleStartGameSuccess]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController handleStartGameSuccess]")]
pub fn stub_1c958() -> ! {
    todo!("0x1c958 -[HomeViewController handleStartGameSuccess]")
}

// 0x1c95c — -[HomeViewController placeIdClicked:]
// demangled: -[HomeViewController placeIdClicked:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController placeIdClicked:]")]
pub fn stub_1c95c() -> ! {
    todo!("0x1c95c -[HomeViewController placeIdClicked:]")
}

// 0x1ca9c — -[HomeViewController searchEditingDidEnd:]
// demangled: -[HomeViewController searchEditingDidEnd:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController searchEditingDidEnd:]")]
pub fn stub_1ca9c() -> ! {
    todo!("0x1ca9c -[HomeViewController searchEditingDidEnd:]")
}

// 0x1caa0 — -[HomeViewController searchDidEndOnExit:]
// demangled: -[HomeViewController searchDidEndOnExit:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController searchDidEndOnExit:]")]
pub fn stub_1caa0() -> ! {
    todo!("0x1caa0 -[HomeViewController searchDidEndOnExit:]")
}

// 0x1cac8 — -[HomeViewController signUpButtonDidTouchUpInside:]
// demangled: -[HomeViewController signUpButtonDidTouchUpInside:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController signUpButtonDidTouchUpInside:]")]
pub fn stub_1cac8() -> ! {
    todo!("0x1cac8 -[HomeViewController signUpButtonDidTouchUpInside:]")
}

// 0x1cacc — -[HomeViewController logInButtonDidTouchUpInside:]
// demangled: -[HomeViewController logInButtonDidTouchUpInside:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController logInButtonDidTouchUpInside:]")]
pub fn stub_1cacc() -> ! {
    todo!("0x1cacc -[HomeViewController logInButtonDidTouchUpInside:]")
}

// 0x1cae0 — -[HomeViewController buttonForWebDidTouchUpInside:]
// demangled: -[HomeViewController buttonForWebDidTouchUpInside:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController buttonForWebDidTouchUpInside:]")]
pub fn stub_1cae0() -> ! {
    todo!("0x1cae0 -[HomeViewController buttonForWebDidTouchUpInside:]")
}

// 0x1cbac — -[HomeViewController btnTouchPlayButtonDisabled:]
// demangled: -[HomeViewController btnTouchPlayButtonDisabled:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController btnTouchPlayButtonDisabled:]")]
pub fn stub_1cbac() -> ! {
    todo!("0x1cbac -[HomeViewController btnTouchPlayButtonDisabled:]")
}

// 0x1cc1c — +[HomeViewController getUrlForButtonTag:recordPageView:]
// demangled: +[HomeViewController getUrlForButtonTag:recordPageView:]
// type: id __cdecl(id, SEL, int, char)
#[doc(alias = "+[HomeViewController getUrlForButtonTag:recordPageView:]")]
pub fn stub_1cc1c() -> ! {
    todo!("0x1cc1c +[HomeViewController getUrlForButtonTag:recordPageView:]")
}

// 0x1cc54 — +[HomeViewController getUrlForButtonTag:recordPageView:query:]
// demangled: +[HomeViewController getUrlForButtonTag:recordPageView:query:]
// type: id __cdecl(id, SEL, int, char, id)
#[doc(alias = "+[HomeViewController getUrlForButtonTag:recordPageView:query:]")]
pub fn stub_1cc54() -> ! {
    todo!("0x1cc54 +[HomeViewController getUrlForButtonTag:recordPageView:query:]")
}

// 0x1cfe8 — -[HomeViewController prepareForSegue:sender:]
// demangled: -[HomeViewController prepareForSegue:sender:]
// type: void __cdecl(HomeViewController *self, SEL, id, id)
#[doc(alias = "-[HomeViewController prepareForSegue:sender:]")]
pub fn stub_1cfe8() -> ! {
    todo!("0x1cfe8 -[HomeViewController prepareForSegue:sender:]")
}

// 0x1d238 — -[HomeViewController viewMustSegueAfterLoad]
// demangled: -[HomeViewController viewMustSegueAfterLoad]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController viewMustSegueAfterLoad]")]
pub fn stub_1d238() -> ! {
    todo!("0x1d238 -[HomeViewController viewMustSegueAfterLoad]")
}

// 0x1d248 — -[HomeViewController setJumpToPlaceID:]
// demangled: -[HomeViewController setJumpToPlaceID:]
// type: void __cdecl(HomeViewController *self, SEL, int)
#[doc(alias = "-[HomeViewController setJumpToPlaceID:]")]
pub fn stub_1d248() -> ! {
    todo!("0x1d248 -[HomeViewController setJumpToPlaceID:]")
}

// 0x1d258 — -[HomeViewController blueFrame]
// demangled: -[HomeViewController blueFrame]
// type: UIImageView *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController blueFrame]")]
pub fn stub_1d258() -> ! {
    todo!("0x1d258 -[HomeViewController blueFrame]")
}

// 0x1d268 — -[HomeViewController setBlueFrame:]
// demangled: -[HomeViewController setBlueFrame:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setBlueFrame:]")]
pub fn stub_1d268() -> ! {
    todo!("0x1d268 -[HomeViewController setBlueFrame:]")
}

// 0x1d28c — -[HomeViewController imgAvatar]
// demangled: -[HomeViewController imgAvatar]
// type: UIImageView *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController imgAvatar]")]
pub fn stub_1d28c() -> ! {
    todo!("0x1d28c -[HomeViewController imgAvatar]")
}

// 0x1d29c — -[HomeViewController setImgAvatar:]
// demangled: -[HomeViewController setImgAvatar:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setImgAvatar:]")]
pub fn stub_1d29c() -> ! {
    todo!("0x1d29c -[HomeViewController setImgAvatar:]")
}
