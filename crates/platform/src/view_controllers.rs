//! View controllers — GameViewController, AppDelegate, EAGL2View, UIViewController — mirrors Client/iOS/*
//! Auto-generated from ida/export.json — filtered demangled contains RobloxView|GameViewController|AppDelegate|EAGL2
//! 100 stubs | sorted by EA | SharedPtr = rbx_core::SharedPtr (Arc) not boost::shared_ptr

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
// Shared iOS-bridge model for the GameViewController / AppDelegate leaves below.
// ObjC `id` (nullable object pointer) has no host runtime here; `None` is `nil`.
pub type ObjCId = usize;
pub const NIL_ID: ObjCId = 0;

/// Landscape orientations from `UIInterfaceOrientation` (original compares raw ints).
pub const UI_INTERFACE_ORIENTATION_LANDSCAPE_LEFT: i32 = 3;
pub const UI_INTERFACE_ORIENTATION_LANDSCAPE_RIGHT: i32 = 4;
// 0x4db08 returns 0x18: landscape-left | landscape-right mask.
pub const UI_INTERFACE_ORIENTATION_MASK_LANDSCAPE: u32 = 24;

/// RAII handle mirroring `rbx::signals::scoped_connection` plus its weak slot.
/// `disconnect` maps to `rbx::signals::connection::disconnect`; clearing the
/// slot maps to the conditional `weak_release` (this slot owns the last weak ref).
#[derive(Debug, Default)]
pub struct ScopedConnection {
    connected: bool,
    has_weak_slot: bool,
}

impl ScopedConnection {
    pub fn new() -> Self {
        // A freshly constructed scoped_connection is live; `.cxx_construct`
        // zeroes the weak slot, so it starts empty (cf. IDA 0x1a5ca).
        Self { connected: true, has_weak_slot: false }
    }
    pub fn disconnect(&mut self) {
        self.connected = false;
    }
    pub fn reset_weak_slot(&mut self) {
        self.has_weak_slot = false;
    }
    pub fn is_connected(&self) -> bool {
        self.connected
    }
    pub fn has_weak_slot(&self) -> bool {
        self.has_weak_slot
    }
}

/// Process-wide `NSNotificationCenter` counterpart (`+[NSNotificationCenter defaultCenter]`).
/// Observers are tracked by pointer identity, matching `removeObserver:` semantics.
#[derive(Debug, Default)]
pub struct NotificationCenter {
    observers: parking_lot::Mutex<Vec<ObjCId>>,
}

impl NotificationCenter {
    pub fn default_center() -> &'static Self {
        static CENTER: std::sync::LazyLock<NotificationCenter> =
            std::sync::LazyLock::new(NotificationCenter::default);
        &CENTER
    }
    pub fn add_observer(&self, observer: ObjCId) {
        self.observers.lock().push(observer);
    }
    pub fn remove_observer(&self, observer: ObjCId) {
        self.observers.lock().retain(|&o| o != observer);
    }
    pub fn observer_count(&self) -> usize {
        self.observers.lock().len()
    }
}

/// Minimal `GameView` counterpart: owns the subview list `getControlView` enumerates
/// and counts layout passes for `layoutSubviews` (UIKit internals are out of slice).
#[derive(Debug, Default)]
pub struct GameView {
    subviews: parking_lot::Mutex<Vec<ObjCId>>,
    layout_passes: std::sync::atomic::AtomicU32,
}

impl GameView {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_subview(&self, view: ObjCId) {
        self.subviews.lock().push(view);
    }
    pub fn first_subview(&self) -> Option<ObjCId> {
        self.subviews.lock().first().copied()
    }
    pub fn layout_subviews(&self) {
        self.layout_passes
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn layout_passes(&self) -> u32 {
        self.layout_passes.load(std::sync::atomic::Ordering::SeqCst)
    }
}

/// Minimal `UIWebView` counterpart: detach models `removeFromSuperview`,
/// drop models `release`.
#[derive(Debug, Default)]
pub struct WebView {
    superview: parking_lot::Mutex<ObjCId>,
}

impl WebView {
    pub fn new(superview: ObjCId) -> Self {
        Self { superview: parking_lot::Mutex::new(superview) }
    }
    pub fn remove_from_superview(&self) {
        *self.superview.lock() = NIL_ID;
    }
    pub fn superview(&self) -> ObjCId {
        *self.superview.lock()
    }
}

/// Minimal `UIApplication` counterpart for `setStatusBarHidden:`.
#[derive(Debug, Default)]
pub struct UiApplication {
    status_bar_hidden: std::sync::atomic::AtomicBool,
}

impl UiApplication {
    pub fn shared_application() -> &'static Self {
        static APP: std::sync::LazyLock<UiApplication> =
            std::sync::LazyLock::new(UiApplication::default);
        &APP
    }
    pub fn set_status_bar_hidden(&self, hidden: bool) {
        self.status_bar_hidden
            .store(hidden, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn is_status_bar_hidden(&self) -> bool {
        self.status_bar_hidden.load(std::sync::atomic::Ordering::SeqCst)
    }
}

/// Minimal `PlaceLauncher` counterpart behind `+[PlaceLauncher sharedInstance]`.
#[derive(Debug, Default)]
pub struct PlaceLauncher {
    view_enabled: std::sync::atomic::AtomicBool,
    in_game: std::sync::atomic::AtomicBool,
    leave_calls: std::sync::atomic::AtomicU32,
    memory_warning_calls: std::sync::atomic::AtomicU32,
    start_game_calls: std::sync::atomic::AtomicU32,
    last_start_game: parking_lot::Mutex<Option<StartGameRequest>>,
}

impl PlaceLauncher {
    pub fn shared_instance() -> &'static Self {
        static LAUNCHER: std::sync::LazyLock<PlaceLauncher> = std::sync::LazyLock::new(|| PlaceLauncher {
            view_enabled: std::sync::atomic::AtomicBool::new(true),
            ..PlaceLauncher::default()
        });
        &LAUNCHER
    }
    pub fn disable_view_because_going_to_background(&self) {
        self.view_enabled.store(false, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn is_view_enabled(&self) -> bool {
        self.view_enabled.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `-[PlaceLauncher leaveGame]` (IDA 0x197e6): leaves any running game.
    pub fn leave_game(&self) {
        self.in_game.store(false, std::sync::atomic::Ordering::SeqCst);
        self.leave_calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn is_in_game(&self) -> bool {
        self.in_game.load(std::sync::atomic::Ordering::SeqCst)
    }
    pub fn leave_call_count(&self) -> u32 {
        self.leave_calls.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `-[PlaceLauncher enableViewBecauseGoingToForeground]` (IDA 0x19de0).
    pub fn enable_view_because_going_to_foreground(&self) {
        self.view_enabled.store(true, std::sync::atomic::Ordering::SeqCst);
    }
    /// `-[PlaceLauncher applicationDidReceiveMemoryWarning]` (IDA 0x19b00).
    pub fn application_did_receive_memory_warning(&self) {
        self.memory_warning_calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn memory_warning_call_count(&self) -> u32 {
        self.memory_warning_calls.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `-[PlaceLauncher startGame:controller:request:presentGameAutomatically:]` (IDA 0x1a42a).
    pub fn start_game(&self, place_id: i32, controller: Option<ObjCId>, present_automatically: bool) {
        // `request:` is always nil at this call site (IDA 0x1a42a passes 0); no host counterpart.
        *self.last_start_game.lock() = Some(StartGameRequest { place_id, controller, present_automatically });
        self.start_game_calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn start_game_call_count(&self) -> u32 {
        self.start_game_calls.load(std::sync::atomic::Ordering::SeqCst)
    }
    pub fn last_start_game(&self) -> Option<StartGameRequest> {
        *self.last_start_game.lock()
    }
}

/// `EAGLViewController` base (composition models the ObjC superclass).
/// Tracks appearance plus the base-level memory-warning count; the subclass
/// overrides below forward to these exactly like the `objc_super` sends.
#[derive(Debug, Default)]
pub struct EaglViewController {
    appearing: std::sync::atomic::AtomicBool,
    appeared: std::sync::atomic::AtomicBool,
    memory_warnings: std::sync::atomic::AtomicU32,
}

impl EaglViewController {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn view_will_appear(&self, _animated: bool) {
        self.appearing.store(true, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn view_did_appear(&self, _animated: bool) {
        self.appearing.store(false, std::sync::atomic::Ordering::SeqCst);
        self.appeared.store(true, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn did_receive_memory_warning(&self) {
        self.memory_warnings
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn is_appearing(&self) -> bool {
        self.appearing.load(std::sync::atomic::Ordering::SeqCst)
    }
    pub fn did_appear(&self) -> bool {
        self.appeared.load(std::sync::atomic::Ordering::SeqCst)
    }
    pub fn memory_warning_count(&self) -> u32 {
        self.memory_warnings.load(std::sync::atomic::Ordering::SeqCst)
    }
}

/// `AppDelegate` ivars: atomic `bgTask`, retained `_window`, C++ `messageOutConnection`.
#[derive(Debug, Default)]
pub struct AppDelegate {
    bg_task: std::sync::atomic::AtomicU32,
    window: parking_lot::Mutex<Option<ObjCId>>,
    message_out_connection: parking_lot::Mutex<ScopedConnection>,
}

impl AppDelegate {
    pub fn new() -> Self {
        let this = Self {
            message_out_connection: parking_lot::Mutex::new(ScopedConnection::new()),
            ..Self::default()
        };
        this.cxx_construct();
        this
    }
    fn objc_id(&self) -> ObjCId {
        self as *const Self as ObjCId
    }
}

/// `GameViewController` ivars over the `EAGLViewController` base.
#[derive(Debug, Default)]
pub struct GameViewController {
    base: EaglViewController,
    game_view: GameView,
    external_web_view: parking_lot::Mutex<Option<WebView>>,
}

impl GameViewController {
    pub fn new() -> Self {
        Self::default()
    }
    fn objc_id(&self) -> ObjCId {
        self as *const Self as ObjCId
    }
    pub fn base(&self) -> &EaglViewController {
        &self.base
    }
    pub fn game_view(&self) -> &GameView {
        &self.game_view
    }
    pub fn set_external_web_view(&self, view: Option<WebView>) {
        *self.external_web_view.lock() = view;
    }
    pub fn has_external_web_view(&self) -> bool {
        self.external_web_view.lock().is_some()
    }
}

/// Minimal `Ogre::EAGL2Support` counterpart: config/display-name queries only
/// (window creation and GL context setup are out of slice).
#[derive(Debug, Default)]
pub struct Eagl2Support;

/// Minimal `Ogre::EAGL2Window` counterpart: `setFullscreen`/`reposition` are
/// empty (`BX LR`) on this slice; resize and swap paths stay unimplemented.
#[derive(Debug, Default)]
pub struct Eagl2Window;

/// Minimal `EAGL2View` counterpart: the `mWindowName` std::string ivar plus the
/// `CAEAGLLayer` layer class (backing-store rendering is out of slice).
#[derive(Debug, Default)]
pub struct Eagl2View {
    window_name: parking_lot::Mutex<String>,
}

/// Minimal `EAGL2ViewController` counterpart: the `mGLSupport` assign ivar plus
/// a count of the UIKit super-sends (`init`, `loadView`, ...) that are out of slice.
#[derive(Debug, Default)]
pub struct Eagl2ViewController {
    gl_support: parking_lot::Mutex<ObjCId>,
    super_forwards: std::sync::atomic::AtomicU32,
}

impl Eagl2ViewController {
    pub fn super_forward_count(&self) -> u32 {
        self.super_forwards.load(std::sync::atomic::Ordering::SeqCst)
    }
    fn note_super_forward(&self) {
        self.super_forwards
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

/// `-[PlaceLauncher startGame:controller:request:presentGameAutomatically:]` request (IDA 0x1a42a).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StartGameRequest {
    pub place_id: i32,
    pub controller: Option<ObjCId>,
    pub present_automatically: bool,
}

/// App-wide `appPlaceID`: written by `application:openURL:...` (IDA 0x1a22e),
/// consumed and cleared by `applicationDidBecomeActive:` (IDA 0x19e32..0x19e48).
pub static APP_PLACE_ID: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

/// Host seam for the UIKit hierarchy `TryLaunchPlace:` reads (IDA 0x1a2fc..0x1a316).
/// Tests stage the top controller class name here; empty means "unknown".
static TOP_CONTROLLER_CLASS: std::sync::LazyLock<parking_lot::Mutex<String>> =
    std::sync::LazyLock::new(|| parking_lot::Mutex::new(String::new()));

/// Stages the class name `try_launch_place` dispatches on.
pub fn set_top_controller_class(class: &str) {
    *TOP_CONTROLLER_CLASS.lock() = class.to_owned();
}

/// Class name `try_launch_place` dispatches on.
pub fn top_controller_class() -> String {
    TOP_CONTROLLER_CLASS.lock().clone()
}

/// `NSString -intValue` (IDA 0x1a220): leading optional sign plus digit prefix, else 0.
fn ns_int_value(s: &str) -> i32 {
    let s = s.trim_start();
    let (negative, digits) = match s.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, s.strip_prefix('+').unwrap_or(s)),
    };
    let end = digits.find(|c: char| !c.is_ascii_digit()).unwrap_or(digits.len());
    let value: i32 = digits[..end].parse().unwrap_or(0);
    if negative { value.saturating_neg() } else { value }
}

/// Minimal `NSUserDefaults` counterpart: string values plus registered bool defaults.
#[derive(Debug, Default)]
pub struct UserDefaults {
    values: parking_lot::Mutex<std::collections::HashMap<String, String>>,
    registered: parking_lot::Mutex<Vec<(String, bool)>>,
    sync_calls: std::sync::atomic::AtomicU32,
}

impl UserDefaults {
    pub fn standard() -> &'static Self {
        static DEFAULTS: std::sync::LazyLock<UserDefaults> =
            std::sync::LazyLock::new(UserDefaults::default);
        &DEFAULTS
    }
    pub fn register_defaults(&self, pairs: &[(&str, bool)]) {
        self.registered.lock().extend(pairs.iter().map(|(k, v)| (k.to_string(), *v)));
    }
    pub fn registered_defaults(&self) -> Vec<(String, bool)> {
        self.registered.lock().clone()
    }
    pub fn set_object(&self, value: &str, key: &str) {
        self.values.lock().insert(key.to_owned(), value.to_owned());
    }
    /// Covers both `objectForKey:` and `stringForKey:`.
    pub fn object_for_key(&self, key: &str) -> Option<String> {
        self.values.lock().get(key).cloned()
    }
    pub fn remove_object_for_key(&self, key: &str) {
        self.values.lock().remove(key);
    }
    pub fn synchronize(&self) {
        self.sync_calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn synchronize_call_count(&self) -> u32 {
        self.sync_calls.load(std::sync::atomic::Ordering::SeqCst)
    }
}

/// Minimal `UserInfo CurrentPlayer` counterpart behind `+[UserInfo CurrentPlayer]`.
#[derive(Debug, Default)]
pub struct CurrentPlayer {
    username: parking_lot::Mutex<String>,
    password: parking_lot::Mutex<String>,
}

impl CurrentPlayer {
    pub fn current() -> &'static Self {
        static PLAYER: std::sync::LazyLock<CurrentPlayer> =
            std::sync::LazyLock::new(CurrentPlayer::default);
        &PLAYER
    }
    pub fn set_username(&self, username: &str) {
        *self.username.lock() = username.to_owned();
    }
    pub fn username(&self) -> String {
        self.username.lock().clone()
    }
    pub fn set_password(&self, password: &str) {
        *self.password.lock() = password.to_owned();
    }
    pub fn password(&self) -> String {
        self.password.lock().clone()
    }
}

/// Minimal `SessionReporter` counterpart behind `+[SessionReporter sharedInstance]`.
#[derive(Debug, Default)]
pub struct SessionReporter {
    last_session: std::sync::atomic::AtomicI32,
    report_calls: std::sync::atomic::AtomicU32,
}

impl SessionReporter {
    pub fn shared_instance() -> &'static Self {
        static REPORTER: std::sync::LazyLock<SessionReporter> =
            std::sync::LazyLock::new(|| SessionReporter {
                last_session: std::sync::atomic::AtomicI32::new(-1),
                report_calls: std::sync::atomic::AtomicU32::new(0),
            });
        &REPORTER
    }
    /// `-reportSessionFor:`.
    pub fn report_session_for(&self, session: i32) {
        self.last_session.store(session, std::sync::atomic::Ordering::SeqCst);
        self.report_calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn last_reported_session(&self) -> i32 {
        self.last_session.load(std::sync::atomic::Ordering::SeqCst)
    }
    pub fn report_call_count(&self) -> u32 {
        self.report_calls.load(std::sync::atomic::Ordering::SeqCst)
    }
}

/// Minimal `RobloxGoogleAnalytics` counterpart.
#[derive(Debug, Default)]
pub struct GoogleAnalytics {
    debug_counters_calls: std::sync::atomic::AtomicU32,
    last_page_view: parking_lot::Mutex<Option<String>>,
    page_view_calls: std::sync::atomic::AtomicU32,
}

impl GoogleAnalytics {
    fn shared() -> &'static Self {
        static ANALYTICS: std::sync::LazyLock<GoogleAnalytics> =
            std::sync::LazyLock::new(GoogleAnalytics::default);
        &ANALYTICS
    }
    /// `+debugCountersPrint`.
    pub fn debug_counters_print() {
        Self::shared().debug_counters_calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn debug_counters_call_count() -> u32 {
        Self::shared().debug_counters_calls.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `+setPageViewTracking:`.
    pub fn set_page_view_tracking(page: &str) {
        *Self::shared().last_page_view.lock() = Some(page.to_owned());
        Self::shared().page_view_calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn last_page_view() -> Option<String> {
        Self::shared().last_page_view.lock().clone()
    }
}

/// Minimal `NSHTTPCookieStorage` counterpart.
#[derive(Debug, Default)]
pub struct CookieStorage {
    accept_policy: std::sync::atomic::AtomicU32,
}

impl CookieStorage {
    pub fn shared() -> &'static Self {
        static STORAGE: std::sync::LazyLock<CookieStorage> =
            std::sync::LazyLock::new(|| CookieStorage {
                accept_policy: std::sync::atomic::AtomicU32::new(u32::MAX),
            });
        &STORAGE
    }
    /// `-setCookieAcceptPolicy:`.
    pub fn set_cookie_accept_policy(&self, policy: u32) {
        self.accept_policy.store(policy, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn accept_policy(&self) -> u32 {
        self.accept_policy.load(std::sync::atomic::Ordering::SeqCst)
    }
}

/// `CrashReporter` access counterpart behind `+[CrashReporter sharedInstance]` (IDA 0x19384).
#[derive(Debug, Default)]
pub struct CrashReporter {
    accesses: std::sync::atomic::AtomicU32,
}

impl CrashReporter {
    fn shared_raw() -> &'static Self {
        static REPORTER: std::sync::LazyLock<CrashReporter> =
            std::sync::LazyLock::new(CrashReporter::default);
        &REPORTER
    }
    pub fn shared_instance() -> &'static Self {
        let reporter = Self::shared_raw();
        reporter.accesses.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        reporter
    }
    pub fn access_count() -> u32 {
        Self::shared_raw().accesses.load(std::sync::atomic::Ordering::SeqCst)
    }
}

/// `UpgradeCheckHelper` counterpart behind `+checkForUpdate` (IDA 0x1940a).
#[derive(Debug, Default)]
pub struct UpgradeCheckHelper {
    check_calls: std::sync::atomic::AtomicU32,
}

impl UpgradeCheckHelper {
    fn shared() -> &'static Self {
        static HELPER: std::sync::LazyLock<UpgradeCheckHelper> =
            std::sync::LazyLock::new(UpgradeCheckHelper::default);
        &HELPER
    }
    /// `+checkForUpdate`.
    pub fn check_for_update() {
        Self::shared().check_calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn check_call_count() -> u32 {
        Self::shared().check_calls.load(std::sync::atomic::Ordering::SeqCst)
    }
}

/// `Flurry` session counterpart behind `+startSession:` (IDA 0x1950e).
#[derive(Debug, Default)]
pub struct Flurry {
    session_key: parking_lot::Mutex<Option<String>>,
    start_calls: std::sync::atomic::AtomicU32,
}

impl Flurry {
    fn shared() -> &'static Self {
        static FLURRY: std::sync::LazyLock<Flurry> = std::sync::LazyLock::new(Flurry::default);
        &FLURRY
    }
    /// `+startSession:`.
    pub fn start_session(key: &str) {
        *Self::shared().session_key.lock() = Some(key.to_owned());
        Self::shared().start_calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn session_key() -> Option<String> {
        Self::shared().session_key.lock().clone()
    }
}

/// `Appirater` configuration counterpart (IDA 0x1953a..0x1959a, 0x19bf0).
#[derive(Debug, Default)]
pub struct Appirater {
    app_id: parking_lot::Mutex<String>,
    days_until_prompt: parking_lot::Mutex<f64>,
    uses_until_prompt: std::sync::atomic::AtomicU32,
    time_before_reminding: parking_lot::Mutex<f64>,
    app_launched_calls: std::sync::atomic::AtomicU32,
    entered_foreground_calls: std::sync::atomic::AtomicU32,
}

impl Appirater {
    fn shared() -> &'static Self {
        static APPIRATER: std::sync::LazyLock<Appirater> =
            std::sync::LazyLock::new(Appirater::default);
        &APPIRATER
    }
    /// `+setAppId:`.
    pub fn set_app_id(id: &str) {
        *Self::shared().app_id.lock() = id.to_owned();
    }
    pub fn app_id() -> String {
        Self::shared().app_id.lock().clone()
    }
    /// `+setDaysUntilPrompt:`.
    pub fn set_days_until_prompt(days: f64) {
        *Self::shared().days_until_prompt.lock() = days;
    }
    pub fn days_until_prompt() -> f64 {
        *Self::shared().days_until_prompt.lock()
    }
    /// `+setUsesUntilPrompt:`.
    pub fn set_uses_until_prompt(uses: u32) {
        Self::shared().uses_until_prompt.store(uses, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn uses_until_prompt() -> u32 {
        Self::shared().uses_until_prompt.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `+setTimeBeforeReminding:`.
    pub fn set_time_before_reminding(days: f64) {
        *Self::shared().time_before_reminding.lock() = days;
    }
    pub fn time_before_reminding() -> f64 {
        *Self::shared().time_before_reminding.lock()
    }
    /// `+appLaunched:`.
    pub fn app_launched(_first_launch: bool) {
        Self::shared().app_launched_calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn app_launched_call_count() -> u32 {
        Self::shared().app_launched_calls.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `+appEnteredForeground:`.
    pub fn app_entered_foreground(_entered: bool) {
        Self::shared().entered_foreground_calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn entered_foreground_call_count() -> u32 {
        Self::shared().entered_foreground_calls.load(std::sync::atomic::Ordering::SeqCst)
    }
}

/// Minimal `RobloxMemoryManager` counterpart behind `+[RobloxMemoryManager sharedInstance]`.
#[derive(Debug, Default)]
pub struct MemoryManager {
    bouncer_running: std::sync::atomic::AtomicBool,
    stop_calls: std::sync::atomic::AtomicU32,
}

impl MemoryManager {
    pub fn shared_instance() -> &'static Self {
        static MANAGER: std::sync::LazyLock<MemoryManager> =
            std::sync::LazyLock::new(|| MemoryManager {
                bouncer_running: std::sync::atomic::AtomicBool::new(true),
                stop_calls: std::sync::atomic::AtomicU32::new(0),
            });
        &MANAGER
    }
    /// `-stopMemoryBouncer:0` (IDA 0x19ad8 passes a constant 0): stops the bouncer,
    /// returning whether one was running. `false` forwards to `PlaceLauncher` (IDA 0x19adc).
    pub fn stop_memory_bouncer(&self) -> bool {
        self.stop_calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.bouncer_running.swap(false, std::sync::atomic::Ordering::SeqCst)
    }
    pub fn is_bouncer_running(&self) -> bool {
        self.bouncer_running.load(std::sync::atomic::Ordering::SeqCst)
    }
}

/// Minimal `LoginManager` counterpart behind `+[LoginManager sharedInstance]`.
#[derive(Debug, Default)]
pub struct LoginManager {
    will_terminate_calls: std::sync::atomic::AtomicU32,
}

impl LoginManager {
    pub fn shared_instance() -> &'static Self {
        static MANAGER: std::sync::LazyLock<LoginManager> =
            std::sync::LazyLock::new(LoginManager::default);
        &MANAGER
    }
    /// `-applicationWillTerminate` (IDA 0x1a064).
    pub fn application_will_terminate(&self) {
        self.will_terminate_calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn will_terminate_call_count(&self) -> u32 {
        self.will_terminate_calls.load(std::sync::atomic::Ordering::SeqCst)
    }
}

/// Minimal `LoginViewController` counterpart: `+sharedInstance` plus the place-id
/// sinks `TryLaunchPlace:` drives (IDA 0x1a364..0x1a47a).
#[derive(Debug, Default)]
pub struct LoginViewController {
    login_place_id: std::sync::atomic::AtomicI32,
    jump_to_place_id: std::sync::atomic::AtomicI32,
    jump_to_place_id_game_in_progress: std::sync::atomic::AtomicI32,
    web_button_taps: std::sync::atomic::AtomicU32,
}

impl LoginViewController {
    pub fn shared_instance() -> &'static Self {
        static CONTROLLER: std::sync::LazyLock<LoginViewController> =
            std::sync::LazyLock::new(LoginViewController::default);
        &CONTROLLER
    }
    /// `+mostRecentViewController` resolves to the shared instance here (IDA 0x1a46e).
    pub fn most_recent_view_controller() -> &'static Self {
        Self::shared_instance()
    }
    /// `-setLoginPlaceId:` (IDA 0x1a372).
    pub fn set_login_place_id(&self, place_id: i32) {
        self.login_place_id.store(place_id, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn login_place_id(&self) -> i32 {
        self.login_place_id.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `-setJumpToPlaceID:` (IDA 0x1a3ae).
    pub fn set_jump_to_place_id(&self, place_id: i32) {
        self.jump_to_place_id.store(place_id, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn jump_to_place_id(&self) -> i32 {
        self.jump_to_place_id.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `-buttonForWebDidTouchUpInside:` (IDA 0x1a3be).
    pub fn button_for_web_did_touch_up_inside(&self) {
        self.web_button_taps.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn web_button_tap_count(&self) -> u32 {
        self.web_button_taps.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `-setJumpToPlaceIDGameInProgress:` (IDA 0x1a47a).
    pub fn set_jump_to_place_id_game_in_progress(&self, place_id: i32) {
        self.jump_to_place_id_game_in_progress.store(place_id, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn jump_to_place_id_game_in_progress(&self) -> i32 {
        self.jump_to_place_id_game_in_progress.load(std::sync::atomic::Ordering::SeqCst)
    }
}

/// `RBX::ClientAppSettings` fetch counterpart (IDA 0x19f38..0x19f56).
#[derive(Debug, Default)]
pub struct ClientSettings {
    last_fetch: parking_lot::Mutex<Option<(String, String)>>,
    fetch_calls: std::sync::atomic::AtomicU32,
}

impl ClientSettings {
    fn shared() -> &'static Self {
        static SETTINGS: std::sync::LazyLock<ClientSettings> =
            std::sync::LazyLock::new(ClientSettings::default);
        &SETTINGS
    }
    /// `FetchClientSettingsData`: `Initialize`/`singleton` have no host state; the
    /// fetch itself is recorded.
    pub fn fetch(section: &str, key: &str) {
        *Self::shared().last_fetch.lock() = Some((section.to_owned(), key.to_owned()));
        Self::shared().fetch_calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn last_fetch() -> Option<(String, String)> {
        Self::shared().last_fetch.lock().clone()
    }
}

/// `RobloxWebUtility` counterpart behind
/// `+getiOSSettingsServiceWithForcedReadFromWeb:` (IDA 0x19f78).
#[derive(Debug, Default)]
pub struct WebUtility {
    service_calls: std::sync::atomic::AtomicU32,
    forced_service_calls: std::sync::atomic::AtomicU32,
}

impl WebUtility {
    fn shared() -> &'static Self {
        static UTILITY: std::sync::LazyLock<WebUtility> =
            std::sync::LazyLock::new(WebUtility::default);
        &UTILITY
    }
    /// `+getiOSSettingsServiceWithForcedReadFromWeb:`.
    pub fn get_ios_settings_service_with_forced_read_from_web(forced: bool) {
        Self::shared().service_calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        if forced {
            Self::shared().forced_service_calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
    }
    pub fn settings_service_call_count() -> u32 {
        Self::shared().service_calls.load(std::sync::atomic::Ordering::SeqCst)
    }
}

/// Minimal `UIViewController` graph for `_topMostController` (IDA 0x1a098).
#[derive(Debug, Default)]
pub struct ViewControllerGraph {
    presented: parking_lot::Mutex<std::collections::HashMap<ObjCId, ObjCId>>,
    navigation_controllers: parking_lot::Mutex<std::collections::HashSet<ObjCId>>,
    visible: parking_lot::Mutex<std::collections::HashMap<ObjCId, ObjCId>>,
}

impl ViewControllerGraph {
    pub fn new() -> Self {
        Self::default()
    }
    /// Stages a `presentedViewController` edge.
    pub fn present(&self, base: ObjCId, presented: ObjCId) {
        self.presented.lock().insert(base, presented);
    }
    pub fn presented_view_controller(&self, id: ObjCId) -> Option<ObjCId> {
        self.presented.lock().get(&id).copied()
    }
    /// Marks a controller as a `UINavigationController`.
    pub fn mark_navigation_controller(&self, id: ObjCId) {
        self.navigation_controllers.lock().insert(id);
    }
    pub fn is_navigation_controller(&self, id: ObjCId) -> bool {
        self.navigation_controllers.lock().contains(&id)
    }
    /// Stages a navigation controller's `visibleViewController`.
    pub fn set_visible_view_controller(&self, nav: ObjCId, visible: ObjCId) {
        self.visible.lock().insert(nav, visible);
    }
    pub fn visible_view_controller(&self, nav: ObjCId) -> Option<ObjCId> {
        self.visible.lock().get(&nav).copied()
    }
}

/// `-[AppDelegate TryLaunchPlace:]` dispatch outcome (IDA 0x1a334..0x1a488).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaunchAction {
    LoginPlaceIdSet,
    HomeJumpTriggered,
    GameStarted,
    GameInProgressJumpSet,
    Unknown,
}

// 0x19228 — -[AppDelegate init]
// type: AppDelegate *__cdecl(AppDelegate *self, SEL)
// IDA 0x19228
impl AppDelegate {
    #[doc(alias = "-[AppDelegate init]")]
    #[doc = "-[AppDelegate init]"]
    pub fn init() -> Self {
        // Body is only objc_msgSendSuper2(super, "init") (IDA 0x1924c): no ivar stores.
        // C++ ivar construction lives on the separate .cxx_construct path (IDA 0x1a5bc).
        Self {
            message_out_connection: parking_lot::Mutex::new(ScopedConnection::new()),
            ..Self::default()
        }
    }
}

// 0x19254 — -[AppDelegate dealloc]
// type: void __cdecl(AppDelegate *self, SEL)
// IDA 0x19254
impl AppDelegate {
    #[doc(alias = "-[AppDelegate dealloc]")]
    #[doc = "-[AppDelegate dealloc]"]
    pub fn dealloc(self) {
        // +[RobloxGoogleAnalytics release] (IDA 0x19276): no retained host object.
        *self.window.lock() = None; // IDA 0x1928a: -[UIWindow release]
        // [super dealloc] (IDA 0x192ac) runs as self drops here.
    }
}

// 0x192b4 — -[AppDelegate application:didFinishLaunchingWithOptions:]
// type: char __cdecl(AppDelegate *self, SEL, id, id)
// IDA 0x192b4
impl AppDelegate {
    #[doc(alias = "-[AppDelegate application:didFinishLaunchingWithOptions:]")]
    #[doc = "-[AppDelegate application:didFinishLaunchingWithOptions:]"]
    pub fn application_did_finish_launching(&self) -> bool {
        // +[NSNumber numberWithBool:YES/NO] paired with the keys at off_11CC288
        // ("warnings_preference", "wifionly_preference"), registered as defaults.
        // IDA 0x19302..0x19334, IDA 0x19354..0x19366
        UserDefaults::standard().register_defaults(&[
            ("warnings_preference", true),
            ("wifionly_preference", false),
        ]);
        CrashReporter::shared_instance(); // IDA 0x19384
        SessionReporter::shared_instance().report_session_for(7); // IDA 0x19396..0x193a8
        GoogleAnalytics::debug_counters_print(); // IDA 0x193c4
        // dispatch_get_global_queue + two dispatch_async sends (IDA 0x193d6..0x193ee):
        // the blocks run inline here in issue order.
        did_finish_launching_flurry_block(); // IDA 0x193de -> 0x194ec
        did_finish_launching_appirater_block(); // IDA 0x193ee -> 0x19514
        UpgradeCheckHelper::check_for_update(); // IDA 0x1940a
        CookieStorage::shared().set_cookie_accept_policy(0); // IDA 0x19426..0x19438
        // Restore the persisted login into CurrentPlayer (IDA 0x1945c..0x194ce).
        // The sends are unconditional; a missing key reads as empty, like nil.
        let username = UserDefaults::standard().object_for_key("username").unwrap_or_default(); // IDA 0x19480
        CurrentPlayer::current().set_username(&username); // IDA 0x19494
        let password = UserDefaults::standard().object_for_key("password").unwrap_or_default(); // IDA 0x194ba
        CurrentPlayer::current().set_password(&password); // IDA 0x194ce
        true // IDA 0x194e4: MOVS R0, #1
    }
}

// 0x194ec — ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke
// type: void __cdecl(id)
// IDA 0x194ec
#[doc(alias = "___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke")]
#[doc = "___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke"]
pub fn did_finish_launching_flurry_block() {
    // +[Flurry startSession:] (IDA 0x1950e)
    Flurry::start_session("FM7DNRW56339NC22K8GR");
}

// 0x19514 — ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2
// type: void __cdecl(id)
// IDA 0x19514
#[doc(alias = "___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2")]
#[doc = "___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2"]
pub fn did_finish_launching_appirater_block() {
    Appirater::set_app_id("431946152"); // IDA 0x1953a
    Appirater::set_days_until_prompt(3.0); // IDA 0x19554
    Appirater::set_uses_until_prompt(10); // IDA 0x19568
    Appirater::set_time_before_reminding(10.0); // IDA 0x19582
    Appirater::app_launched(true); // IDA 0x1959a
}

// 0x195a0 — -[AppDelegate applicationWillResignActive:]
// type: void __cdecl(AppDelegate *self, SEL, id)
// IDA 0x195a0
impl AppDelegate {
    #[doc(alias = "-[AppDelegate applicationWillResignActive:]")]
    #[doc = "-[AppDelegate applicationWillResignActive:]"]
    pub fn application_will_resign_active(&self) {
        // RBX::StandardOut::printf has no host counterpart here; stderr keeps the begin/end trace.
        eprintln!("AppDelegate applicationWillResignActive begin"); // IDA 0x19600
        // +[PlaceLauncher sharedInstance] disableViewBecauseGoingToBackground
        PlaceLauncher::shared_instance().disable_view_because_going_to_background(); // IDA 0x19640
        eprintln!("AppDelegate applicationWillResignActive end"); // IDA 0x1965e
    }
}

// 0x196e4 — -[AppDelegate applicationDidEnterBackground:]
// type: void __cdecl(AppDelegate *self, SEL, id)
// IDA 0x196e4
impl AppDelegate {
    #[doc(alias = "-[AppDelegate applicationDidEnterBackground:]")]
    #[doc = "-[AppDelegate applicationDidEnterBackground:]"]
    pub fn application_did_enter_background(&self) {
        let defaults = UserDefaults::standard();
        defaults.set_object("tryBackground", "RobloxAppState"); // IDA 0x19742
        defaults.synchronize(); // IDA 0x1975c
        // RBX::StandardOut::printf has no host counterpart here; stderr keeps the trace.
        eprintln!("AppDelegate applicationDidEnterBackground begin"); // IDA 0x197a4
        PlaceLauncher::shared_instance().leave_game(); // IDA 0x197d4..0x197e6
        defaults.remove_object_for_key("signupusername"); // IDA 0x1981e
        defaults.remove_object_for_key("signupbirthdate"); // IDA 0x1983c
        defaults.remove_object_for_key("signupgender"); // IDA 0x1985a
        // Persist the current login (IDA 0x1986a..0x198fe).
        let username = CurrentPlayer::current().username();
        defaults.set_object(&username, "username"); // IDA 0x198b4
        let password = CurrentPlayer::current().password();
        defaults.set_object(&password, "password"); // IDA 0x198fe
        SessionReporter::shared_instance().report_session_for(1); // IDA 0x19926
        GoogleAnalytics::set_page_view_tracking("RobloxApp/EnterBackGround"); // IDA 0x1994e
        eprintln!("AppDelegate applicationDidEnterBackground end"); // IDA 0x1996c
        // BUG preserved: the original removes the state key it just wrote, then syncs.
        // IDA 0x19992..0x199b6
        defaults.remove_object_for_key("RobloxAppState"); // IDA 0x199a4
        defaults.synchronize(); // IDA 0x199b6
    }
}

// 0x19a30 — -[AppDelegate applicationDidReceiveMemoryWarning:]
// type: void __cdecl(AppDelegate *self, SEL, id)
// IDA 0x19a30
impl AppDelegate {
    #[doc(alias = "-[AppDelegate applicationDidReceiveMemoryWarning:]")]
    #[doc = "-[AppDelegate applicationDidReceiveMemoryWarning:]"]
    pub fn application_did_receive_memory_warning(&self) {
        // RBX::StandardOut::printf has no host counterpart here; stderr keeps the trace.
        eprintln!("Received out of memory warning (applicationDidReceiveMemoryWarning)"); // IDA 0x19a90
        // -[RobloxMemoryManager stopMemoryBouncer:0] (IDA 0x19ac0..0x19ad8, constant 0 folded).
        if !MemoryManager::shared_instance().stop_memory_bouncer() {
            // IDA 0x19adc: TST returns zero -> forward to PlaceLauncher.
            PlaceLauncher::shared_instance().application_did_receive_memory_warning(); // IDA 0x19aee..0x19b00
        }
    }
}

// 0x19b60 — -[AppDelegate applicationWillEnterForeground:]
// type: void __cdecl(AppDelegate *self, SEL, id)
// IDA 0x19b60
impl AppDelegate {
    #[doc(alias = "-[AppDelegate applicationWillEnterForeground:]")]
    #[doc = "-[AppDelegate applicationWillEnterForeground:]"]
    pub fn application_will_enter_foreground(&self) {
        // RBX::StandardOut::printf has no host counterpart here; stderr keeps the begin/end trace.
        eprintln!("AppDelegate applicationWillEnterForeground begin"); // IDA 0x19bc0
        Appirater::app_entered_foreground(true); // IDA 0x19bf0
        UpgradeCheckHelper::check_for_update(); // IDA 0x19c0e
        GoogleAnalytics::set_page_view_tracking("RobloxApp/EnterForeGround"); // IDA 0x19c36
        eprintln!("AppDelegate applicationWillEnterForeground end"); // IDA 0x19c54
    }
}

// 0x19cdc — -[AppDelegate applicationDidBecomeActive:]
// type: void __cdecl(AppDelegate *self, SEL, id)
// IDA 0x19cdc
impl AppDelegate {
    #[doc(alias = "-[AppDelegate applicationDidBecomeActive:]")]
    #[doc = "-[AppDelegate applicationDidBecomeActive:]"]
    pub fn application_did_become_active(&self) {
        let defaults = UserDefaults::standard();
        defaults.set_object("tryForeground", "RobloxAppState"); // IDA 0x19d3c
        defaults.synchronize(); // IDA 0x19d56
        // RBX::StandardOut::printf has no host counterpart here; stderr keeps the begin/end trace.
        eprintln!("AppDelegate applicationDidBecomeActive begin"); // IDA 0x19d9e
        PlaceLauncher::shared_instance().enable_view_because_going_to_foreground(); // IDA 0x19dce..0x19de0
        SessionReporter::shared_instance().report_session_for(0); // IDA 0x19e0a
        // dispatch_async to the global queue (IDA 0x19e14..0x19e22) runs inline here in order.
        did_become_active_fetch_settings_block(); // IDA 0x19e22 -> 0x19f34
        // Pending deep-link place from application:openURL:... (IDA 0x19e32..0x19e48).
        let pending = APP_PLACE_ID.load(std::sync::atomic::Ordering::SeqCst);
        if pending != 0 {
            // IDA 0x19e44: -[AppDelegate TryLaunchPlace:]; the original reads the
            // global for the place id and resolves the top controller from UIKit,
            // staged here via top_controller_class().
            self.try_launch_place(pending, &top_controller_class());
            APP_PLACE_ID.store(0, std::sync::atomic::Ordering::SeqCst); // IDA 0x19e48
        }
        eprintln!("AppDelegate applicationDidBecomeActive end"); // IDA 0x19e64
        defaults.set_object("inApp", "RobloxAppState"); // IDA 0x19ea6
        defaults.synchronize(); // IDA 0x19eb8
    }
}

// 0x19f34 — ___42-[AppDelegate applicationDidBecomeActive:]_block_invoke
// type: void __cdecl(id)
// IDA 0x19f34
#[doc(alias = "___42-[AppDelegate applicationDidBecomeActive:]_block_invoke")]
#[doc = "___42-[AppDelegate applicationDidBecomeActive:]_block_invoke"]
pub fn did_become_active_fetch_settings_block() {
    // RBX::ClientAppSettings::Initialize + singleton feed FetchClientSettingsData.
    // IDA 0x19f38..0x19f56
    ClientSettings::fetch("iOSAppSettings", "D6925E56-BFB9-4908-AAA2-A5B1EC4B2D79");
    WebUtility::get_ios_settings_service_with_forced_read_from_web(false); // IDA 0x19f78
}

// 0x19f7c — -[AppDelegate applicationWillTerminate:]
// type: void __cdecl(AppDelegate *self, SEL, id)
// IDA 0x19f7c
impl AppDelegate {
    #[doc(alias = "-[AppDelegate applicationWillTerminate:]")]
    #[doc = "-[AppDelegate applicationWillTerminate:]"]
    pub fn application_will_terminate(&self) {
        let defaults = UserDefaults::standard();
        // NSLog carries the values; stderr keeps the trace on the host.
        let game_state = defaults.object_for_key("RobloxGameState").unwrap_or_default(); // IDA 0x19fbc
        eprintln!("RobloxGameState: {game_state}"); // IDA 0x19fcc
        let app_state = defaults.object_for_key("RobloxAppState").unwrap_or_default(); // IDA 0x19fe8
        eprintln!("RobloxAppState: {app_state}"); // IDA 0x19ff8
        defaults.set_object("terminated", "RobloxAppState"); // IDA 0x1a01e
        defaults.synchronize(); // IDA 0x1a038
        LoginManager::shared_instance().application_will_terminate(); // IDA 0x1a054..0x1a064
        GoogleAnalytics::set_page_view_tracking("RobloxApp/Exit"); // IDA 0x1a092
    }
}

// 0x1a098 — __Z18_topMostControllerP16UIViewController
// type: id __fastcall(id)
// IDA 0x1a098
#[doc(alias = "_topMostController(UIViewController *)")]
#[doc = "_topMostController(UIViewController *)"]
pub fn top_most_controller(graph: &ViewControllerGraph, root: ObjCId) -> Option<ObjCId> {
    let mut top = root; // IDA 0x1a0b2
    // Walk presentedViewController to the end of the chain (IDA 0x1a0ae..0x1a0ca).
    if graph.presented_view_controller(top).is_some() { // IDA 0x1a0b4
        loop {
            top = graph.presented_view_controller(top).unwrap_or(top); // IDA 0x1a0c2
            if graph.presented_view_controller(top).is_none() { // IDA 0x1a0c4
                break;
            }
        }
    }
    // A navigation controller resolves to its visible view controller (IDA 0x1a0e4..0x1a118).
    if graph.is_navigation_controller(top) { // IDA 0x1a0fc
        if let Some(visible) = graph.visible_view_controller(top) { // IDA 0x1a110..0x1a116
            top = visible; // IDA 0x1a118
        }
    }
    // Nothing presented above the root returns nil (IDA 0x1a11c..0x1a11e).
    if top == root {
        return None; // IDA 0x1a11e
    }
    Some(top) // IDA 0x1a122
}

// 0x1a174 — -[AppDelegate application:openURL:sourceApplication:annotation:]
// type: char __cdecl(AppDelegate *self, SEL, id, id, id, id)
// IDA 0x1a174
impl AppDelegate {
    #[doc(alias = "-[AppDelegate application:openURL:sourceApplication:annotation:]")]
    #[doc = "-[AppDelegate application:openURL:sourceApplication:annotation:]"]
    pub fn application_open_url(
        &self,
        url_absolute_string: &str,
        url_host: &str,
        url_path: &str,
        source_application: &str,
        annotation: &str,
    ) -> bool {
        // NSLog carries the URL, source and annotation; stderr keeps the trace on the host.
        eprintln!( // IDA 0x1a18a
            "AppDelegate::openURL URL:\t{url_absolute_string}\nFrom source:\t{source_application}\nWith annotation:{annotation}"
        );
        // -[NSString hasPrefix:@"robloxmobile"] on the absolute string (IDA 0x1a19c..0x1a1ba).
        if !url_absolute_string.starts_with("robloxmobile") { // IDA 0x1a1c2
            return false; // IDA 0x1a1bc..0x1a1c4
        }
        eprintln!("host {url_host}"); // IDA 0x1a1e6
        eprintln!("path {url_path}"); // IDA 0x1a208
        // appPlaceID = [[url host] intValue] (IDA 0x1a210..0x1a22e).
        APP_PLACE_ID.store(ns_int_value(url_host), std::sync::atomic::Ordering::SeqCst); // IDA 0x1a22e
        true // IDA 0x1a230
    }
}

// 0x1a234 — -[AppDelegate TryLaunchPlace:]
// type: void __cdecl(AppDelegate *self, SEL, int)
// IDA 0x1a234
impl AppDelegate {
    #[doc(alias = "-[AppDelegate TryLaunchPlace:]")]
    #[doc = "-[AppDelegate TryLaunchPlace:]"]
    pub fn try_launch_place(&self, place_id: i32, top_controller_class: &str) -> LaunchAction {
        // The window/rootViewController and keyWindow/lastObject traces (IDA 0x1a24c..0x1a2f2)
        // read live UIKit state with no host counterpart; dispatch below reads the top
        // controller class name from topMostController (IDA 0x1a2fc..0x1a316).
        if top_controller_class == "LoginViewController" { // IDA 0x1a334
            // +[LoginViewController sharedInstance] setLoginPlaceId: (IDA 0x1a364..0x1a3c2).
            LoginViewController::shared_instance().set_login_place_id(place_id);
            LaunchAction::LoginPlaceIdSet
        } else if top_controller_class == "HomeViewController" { // IDA 0x1a386
            LoginViewController::shared_instance().set_jump_to_place_id(place_id); // IDA 0x1a3ae
            // The original reuses the tail send with buttonForWebDidTouchUpInside: and 0.
            // IDA 0x1a3b6..0x1a3c2
            LoginViewController::shared_instance().button_for_web_did_touch_up_inside();
            LaunchAction::HomeJumpTriggered
        } else if top_controller_class == "RobloxNavBarViewController" { // IDA 0x1a3de
            // -[PlaceLauncher startGame:controller:request:presentGameAutomatically:]
            // with request nil and presentGameAutomatically YES (IDA 0x1a40e..0x1a42a).
            // The controller identity is live UIKit state; None stands in for it here.
            PlaceLauncher::shared_instance().start_game(place_id, None, true);
            LaunchAction::GameStarted
        } else if top_controller_class == "GameViewController" { // IDA 0x1a43e
            // +[RobloxNavBarViewController mostRecentViewController]
            // setJumpToPlaceIDGameInProgress: (IDA 0x1a46e..0x1a47a).
            LoginViewController::most_recent_view_controller()
                .set_jump_to_place_id_game_in_progress(place_id);
            LaunchAction::GameInProgressJumpSet
        } else {
            // NSLog "Unknown viewController" (IDA 0x1a488).
            eprintln!("Unknown viewController {top_controller_class}");
            LaunchAction::Unknown
        }
    }
}

// 0x1a494 — -[AppDelegate bgTask]
// type: unsigned int __cdecl(AppDelegate *self, SEL)
// IDA 0x1a494
impl AppDelegate {
    #[doc(alias = "-[AppDelegate bgTask]")]
    #[doc = "-[AppDelegate bgTask]"]
    pub fn bg_task(&self) -> u32 {
        // LDR bgTask ivar + DMB ISH (0x1a4a2): acquire-to-seq fence maps to SeqCst.
        self.bg_task.load(std::sync::atomic::Ordering::SeqCst) // IDA 0x1a4a0
    }
}

// 0x1a4a8 — -[AppDelegate setBgTask:]
// type: void __cdecl(AppDelegate *self, SEL, unsigned int)
// IDA 0x1a4a8
impl AppDelegate {
    #[doc(alias = "-[AppDelegate setBgTask:]")]
    #[doc = "-[AppDelegate setBgTask:]"]
    pub fn set_bg_task(&self, task: u32) {
        // DMB ISH both before (0x1a4b0) and after (0x1a4ba) the STR: SeqCst store.
        self.bg_task.store(task, std::sync::atomic::Ordering::SeqCst) // IDA 0x1a4b8
    }
}

// 0x1a4c0 — -[AppDelegate window]
// type: UIWindow *__cdecl(AppDelegate *self, SEL)
// IDA 0x1a4c0
impl AppDelegate {
    #[doc(alias = "-[AppDelegate window]")]
    #[doc = "-[AppDelegate window]"]
    pub fn window(&self) -> Option<ObjCId> {
        *self.window.lock() // IDA 0x1a4ce: return self->_window
    }
}

// 0x1a4d0 — -[AppDelegate setWindow:]
// type: void __cdecl(AppDelegate *self, SEL, id)
// IDA 0x1a4d0
impl AppDelegate {
    #[doc(alias = "-[AppDelegate setWindow:]")]
    #[doc = "-[AppDelegate setWindow:]"]
    pub fn set_window(&self, window: Option<ObjCId>) {
        // objc_setProperty(self, _cmd, _window, new, atomic=0, copy=0):
        // nonatomic retain-and-store; dropping the old handle models `release`.
        *self.window.lock() = window; // IDA 0x1a4ec
    }
}

// 0x1a4f4 — -[AppDelegate .cxx_destruct]
// type: void __cdecl(AppDelegate *self, SEL)
// IDA 0x1a4f4
impl AppDelegate {
    #[doc(alias = "-[AppDelegate .cxx_destruct]")]
    #[doc = "-[AppDelegate .cxx_destruct]"]
    pub fn cxx_destruct(&self) {
        let mut connection = self.message_out_connection.lock();
        // rbx::signals::connection::disconnect(&messageOutConnection)
        connection.disconnect(); // IDA 0x1a552
        // if (con.weak_slot.p_) intrusive_ptr_weak_release(...): this slot owns
        // the last weak ref, so clearing it performs the release.
        if connection.has_weak_slot() { // IDA 0x1a558
            connection.reset_weak_slot(); // IDA 0x1a560
        }
    }
}

// 0x1a5bc — -[AppDelegate .cxx_construct]
// type: id __cdecl(AppDelegate *self, SEL)
// IDA 0x1a5bc
impl AppDelegate {
    #[doc(alias = "-[AppDelegate .cxx_construct]")]
    #[doc = "-[AppDelegate .cxx_construct]"]
    pub fn cxx_construct(&self) {
        // self->messageOutConnection.con.weak_slot.p_ = 0; return self.
        // The `new()` constructor returns Self instead of the ObjC `id`.
        self.message_out_connection.lock().reset_weak_slot(); // IDA 0x1a5ca
    }
}

// 0x26768 — -[PlaceLauncher presentGameViewController]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher presentGameViewController]")]
pub fn stub_26768() -> ! {
    todo!("0x26768 -[PlaceLauncher presentGameViewController]")
}

// 0x4d70c — -[GameViewController initWithNibName:bundle:]
// type: GameViewController *__cdecl(GameViewController *self, SEL, id, id)
#[doc(alias = "-[GameViewController initWithNibName:bundle:]")]
pub fn stub_4d70c() -> ! {
    todo!("0x4d70c -[GameViewController initWithNibName:bundle:]")
}

// 0x4d8cc — -[GameViewController dealloc]
// type: void __cdecl(GameViewController *self, SEL)
// IDA 0x4d8cc
impl GameViewController {
    #[doc(alias = "-[GameViewController dealloc]")]
    #[doc = "-[GameViewController dealloc]"]
    pub fn dealloc(self) {
        let me = self.objc_id();
        // if (externalWebView) { [removeFromSuperview]; [release]; }
        if let Some(web) = self.external_web_view.lock().take() { // IDA 0x4d8e2
            web.remove_from_superview(); // IDA 0x4d8f0
            drop(web); // IDA 0x4d902
        }
        // [[NSNotificationCenter defaultCenter] removeObserver:self]
        NotificationCenter::default_center().remove_observer(me); // IDA 0x4d930
        // -[GameView release] + [super dealloc]: game_view and base drop with self.
        // IDA 0x4d94e, IDA 0x4d970
    }
}

// 0x4d978 — -[GameViewController viewWillAppear:]
// type: void __cdecl(GameViewController *self, SEL, char)
// IDA 0x4d978
impl GameViewController {
    #[doc(alias = "-[GameViewController viewWillAppear:]")]
    #[doc = "-[GameViewController viewWillAppear:]"]
    pub fn view_will_appear(&self, animated: bool) {
        // [super viewWillAppear:animated]
        self.base.view_will_appear(animated); // IDA 0x4d99c
        // [[UIApplication sharedApplication] setStatusBarHidden:YES]
        UiApplication::shared_application().set_status_bar_hidden(true); // IDA 0x4d9ca
    }
}

// 0x4d9d4 — -[GameViewController viewDidAppear:]
// type: void __cdecl(GameViewController *self, SEL, char)
// IDA 0x4d9d4
impl GameViewController {
    #[doc(alias = "-[GameViewController viewDidAppear:]")]
    #[doc = "-[GameViewController viewDidAppear:]"]
    pub fn view_did_appear(&self, animated: bool) {
        // [super viewDidAppear:animated]; nothing else. // IDA 0x4d9f8
        self.base.view_did_appear(animated);
    }
}

// 0x4da00 — -[GameViewController viewDidLoad]
// type: void __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController viewDidLoad]")]
pub fn stub_4da00() -> ! {
    todo!("0x4da00 -[GameViewController viewDidLoad]")
}

// 0x4dab8 — -[GameViewController didReceiveMemoryWarning]
// type: void __cdecl(GameViewController *self, SEL)
// IDA 0x4dab8
impl GameViewController {
    #[doc(alias = "-[GameViewController didReceiveMemoryWarning]")]
    #[doc = "-[GameViewController didReceiveMemoryWarning]"]
    pub fn did_receive_memory_warning(&self) {
        // [super didReceiveMemoryWarning]; nothing else. // IDA 0x4dadc
        self.base.did_receive_memory_warning();
    }
}

// 0x4dae4 — -[GameViewController resizeGameView]
// type: void __cdecl(GameViewController *self, SEL)
// IDA 0x4dae4
impl GameViewController {
    #[doc(alias = "-[GameViewController resizeGameView]")]
    #[doc = "-[GameViewController resizeGameView]"]
    pub fn resize_game_view(&self) {
        // [gameView layoutSubviews]; tail-called via objc_msgSend. // IDA 0x4dafe
        self.game_view.layout_subviews();
    }
}

// 0x4db04 — -[GameViewController shouldAutorotate]
// type: char __cdecl(GameViewController *self, SEL)
// IDA 0x4db04
impl GameViewController {
    #[doc(alias = "-[GameViewController shouldAutorotate]")]
    #[doc = "-[GameViewController shouldAutorotate]"]
    pub fn should_autorotate(&self) -> bool {
        true // IDA 0x4db06: MOVS R0, #1 (BOOL YES)
    }
}

// 0x4db08 — -[GameViewController supportedInterfaceOrientations]
// type: unsigned int __cdecl(GameViewController *self, SEL)
// IDA 0x4db08
impl GameViewController {
    #[doc(alias = "-[GameViewController supportedInterfaceOrientations]")]
    #[doc = "-[GameViewController supportedInterfaceOrientations]"]
    pub fn supported_interface_orientations(&self) -> u32 {
        // MOVS R0, #0x18: landscape-left | landscape-right mask. // IDA 0x4db0a
        UI_INTERFACE_ORIENTATION_MASK_LANDSCAPE
    }
}

// 0x4db0c — -[GameViewController shouldAutorotateToInterfaceOrientation:]
// type: char __cdecl(GameViewController *self, SEL, int)
// IDA 0x4db0c
impl GameViewController {
    #[doc(alias = "-[GameViewController shouldAutorotateToInterfaceOrientation:]")]
    #[doc = "-[GameViewController shouldAutorotateToInterfaceOrientation:]"]
    pub fn should_autorotate_to_interface_orientation(&self, orientation: i32) -> bool {
        // MOVS R0,#1; CMP R2,#4; BXEQ when landscape-right. // IDA 0x4db0c
        if orientation == UI_INTERFACE_ORIENTATION_LANDSCAPE_RIGHT { // IDA 0x4db10
            return true; // IDA 0x4db12
        }
        orientation == UI_INTERFACE_ORIENTATION_LANDSCAPE_LEFT // IDA 0x4db1a
    }
}

// 0x4db20 — -[GameViewController getControlView]
// type: id __cdecl(GameViewController *self, SEL)
// IDA 0x4db20
impl GameViewController {
    #[doc(alias = "-[GameViewController getControlView]")]
    #[doc = "-[GameViewController getControlView]"]
    pub fn get_control_view(&self) -> Option<ObjCId> {
        // Fast-enumerates gameView.subviews and returns the first object,
        // nil when the list is empty. IDA 0x4db62, IDA 0x4db88, IDA 0x4db80
        self.game_view.first_subview()
    }
}

// 0x4db9c — -[GameViewController webView:shouldStartLoadWithRequest:navigationType:]
// type: char __cdecl(GameViewController *self, SEL, id, id, int)
#[doc(alias = "-[GameViewController webView:shouldStartLoadWithRequest:navigationType:]")]
pub fn stub_4db9c() -> ! {
    todo!("0x4db9c -[GameViewController webView:shouldStartLoadWithRequest:navigationType:]")
}

// 0x4dbe8 — -[GameViewController signalGuiServiceUrlWindowClosedOnDataModel:]
// type: void __cdecl(GameViewController *self, SEL, DataModel *)
#[doc(alias = "-[GameViewController signalGuiServiceUrlWindowClosedOnDataModel:]")]
pub fn stub_4dbe8() -> ! {
    todo!("0x4dbe8 -[GameViewController signalGuiServiceUrlWindowClosedOnDataModel:]")
}

// 0x4dc08 — -[GameViewController closeUrlWindow:]
// type: void __cdecl(GameViewController *self, SEL, id)
#[doc(alias = "-[GameViewController closeUrlWindow:]")]
pub fn stub_4dc08() -> ! {
    todo!("0x4dc08 -[GameViewController closeUrlWindow:]")
}

// 0x4de58 — ___37-[GameViewController closeUrlWindow:]_block_invoke
// type: id __fastcall(_DWORD *)
#[doc(alias = "___37-[GameViewController closeUrlWindow:]_block_invoke")]
pub fn stub_4de58() -> ! {
    todo!("0x4de58 ___37-[GameViewController closeUrlWindow:]_block_invoke")
}

// 0x4df1c — ___37-[GameViewController closeUrlWindow:]_block_invoke_2
// type: id __fastcall(int)
#[doc(alias = "___37-[GameViewController closeUrlWindow:]_block_invoke_2")]
pub fn stub_4df1c() -> ! {
    todo!("0x4df1c ___37-[GameViewController closeUrlWindow:]_block_invoke_2")
}

// 0x4dfec — ___37-[GameViewController closeUrlWindow:]_block_invoke93
// type: id __fastcall(int)
#[doc(alias = "___37-[GameViewController closeUrlWindow:]_block_invoke93")]
pub fn stub_4dfec() -> ! {
    todo!("0x4dfec ___37-[GameViewController closeUrlWindow:]_block_invoke93")
}

// 0x4e070 — -[GameViewController closeUrlWindow]
// type: void __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController closeUrlWindow]")]
pub fn stub_4e070() -> ! {
    todo!("0x4e070 -[GameViewController closeUrlWindow]")
}

// 0x4e084 — -[GameViewController openUrlWindow:]
// type: void __cdecl(GameViewController *self, SEL, basic_string<char, std::char_traits<char>, std::allocator<char> >)
#[doc(alias = "-[GameViewController openUrlWindow:]")]
pub fn stub_4e084() -> ! {
    todo!("0x4e084 -[GameViewController openUrlWindow:]")
}

// 0x4e2ac — ___36-[GameViewController openUrlWindow:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___36-[GameViewController openUrlWindow:]_block_invoke")]
pub fn stub_4e2ac() -> ! {
    todo!("0x4e2ac ___36-[GameViewController openUrlWindow:]_block_invoke")
}

// 0x4e4dc — ___36-[GameViewController openUrlWindow:]_block_invoke136
// type: id __fastcall(int)
#[doc(alias = "___36-[GameViewController openUrlWindow:]_block_invoke136")]
pub fn stub_4e4dc() -> ! {
    todo!("0x4e4dc ___36-[GameViewController openUrlWindow:]_block_invoke136")
}

// 0x4e5fc — ___36-[GameViewController openUrlWindow:]_block_invoke_2
// type: id __fastcall(_DWORD *)
#[doc(alias = "___36-[GameViewController openUrlWindow:]_block_invoke_2")]
pub fn stub_4e5fc() -> ! {
    todo!("0x4e5fc ___36-[GameViewController openUrlWindow:]_block_invoke_2")
}

// 0x4e730 — -[GameViewController handlePromptLoginSignal]
// type: void __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController handlePromptLoginSignal]")]
pub fn stub_4e730() -> ! {
    todo!("0x4e730 -[GameViewController handlePromptLoginSignal]")
}

// 0x4e780 — ___45-[GameViewController handlePromptLoginSignal]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___45-[GameViewController handlePromptLoginSignal]_block_invoke")]
pub fn stub_4e780() -> ! {
    todo!("0x4e780 ___45-[GameViewController handlePromptLoginSignal]_block_invoke")
}

// 0x4e868 — -[GameViewController handlePromptSignupSignal]
// type: void __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController handlePromptSignupSignal]")]
pub fn stub_4e868() -> ! {
    todo!("0x4e868 -[GameViewController handlePromptSignupSignal]")
}

// 0x4e8b8 — ___46-[GameViewController handlePromptSignupSignal]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___46-[GameViewController handlePromptSignupSignal]_block_invoke")]
pub fn stub_4e8b8() -> ! {
    todo!("0x4e8b8 ___46-[GameViewController handlePromptSignupSignal]_block_invoke")
}

// 0x4e9a0 — -[GameViewController handleSignupNotification:]
// type: void __cdecl(GameViewController *self, SEL, id)
#[doc(alias = "-[GameViewController handleSignupNotification:]")]
pub fn stub_4e9a0() -> ! {
    todo!("0x4e9a0 -[GameViewController handleSignupNotification:]")
}

// 0x4ea30 — -[GameViewController handleLoginNotification:]
// type: void __cdecl(GameViewController *self, SEL, id)
#[doc(alias = "-[GameViewController handleLoginNotification:]")]
pub fn stub_4ea30() -> ! {
    todo!("0x4ea30 -[GameViewController handleLoginNotification:]")
}

// 0x4eac8 — ___46-[GameViewController handleLoginNotification:]_block_invoke
// type: void __fastcall(id *)
#[doc(alias = "___46-[GameViewController handleLoginNotification:]_block_invoke")]
pub fn stub_4eac8() -> ! {
    todo!("0x4eac8 ___46-[GameViewController handleLoginNotification:]_block_invoke")
}

// 0xe844ec — __ZN4Ogre12EAGL2SupportC1Ev
// type: _DWORD __fastcall(Ogre::EAGL2Support *__hidden this)
#[doc(alias = "Ogre::EAGL2Support::EAGL2Support(void)")]
pub fn stub_e844ec() -> ! {
    todo!("0xe844ec Ogre::EAGL2Support::EAGL2Support(void)")
}

// 0xe8455c — __ZN4Ogre12EAGL2SupportD0Ev
// type: void __fastcall(Ogre::EAGL2Support *__hidden this)
#[doc(alias = "Ogre::EAGL2Support::~EAGL2Support()")]
pub fn stub_e8455c() -> ! {
    todo!("0xe8455c Ogre::EAGL2Support::~EAGL2Support()")
}

// 0xe84570 — __ZN4Ogre12EAGL2SupportD1Ev
// type: void __fastcall(Ogre::EAGL2Support *__hidden this)
#[doc(alias = "Ogre::EAGL2Support::~EAGL2Support()")]
pub fn stub_e84570() -> ! {
    todo!("0xe84570 Ogre::EAGL2Support::~EAGL2Support()")
}

// 0xe8457c — __ZN4Ogre12EAGL2Support9addConfigEv
// type: _DWORD __fastcall(Ogre::EAGL2Support *__hidden this)
#[doc(alias = "Ogre::EAGL2Support::addConfig(void)")]
pub fn stub_e8457c() -> ! {
    todo!("0xe8457c Ogre::EAGL2Support::addConfig(void)")
}

// 0xe862b0 — __ZN4Ogre12EAGL2Support14validateConfigEv
// type: _DWORD __fastcall(Ogre::EAGL2Support *__hidden this)
// IDA 0xe862b0
impl Eagl2Support {
    #[doc(alias = "Ogre::EAGL2Support::validateConfig(void)")]
    #[doc = "Ogre::EAGL2Support::validateConfig(void)"]
    pub fn validate_config(&self) -> String {
        // Copy-constructs the return from `Ogre::StringUtil::BLANK`
        // (IDA 0xe862be..0xe862c0), which is the empty string.
        String::new() // IDA 0xe862c4
    }
}

// 0xe862c8 — __ZN4Ogre12EAGL2Support14getDisplayNameEv
// type: _DWORD __fastcall(Ogre::EAGL2Support *__hidden this)
// IDA 0xe862c8
impl Eagl2Support {
    #[doc(alias = "Ogre::EAGL2Support::getDisplayName(void)")]
    #[doc = "Ogre::EAGL2Support::getDisplayName(void)"]
    pub fn get_display_name(&self) -> String {
        // Builds the return from the `aTodo` literal (IDA 0xe862ce..0xe862da).
        "todo".to_owned() // IDA 0xe862e0
    }
}

// 0xe862e4 — __ZN4Ogre12EAGL2Support12createWindowEbPNS_17GLES2RenderSystemERKSs
// type: _DWORD __fastcall(Ogre::EAGL2Support *__hidden this, bool, Ogre::GLES2RenderSystem *, const std::string *)
#[doc(alias = "Ogre::EAGL2Support::createWindow(bool,Ogre::GLES2RenderSystem *,std::string const&)")]
pub fn stub_e862e4() -> ! {
    todo!("0xe862e4 Ogre::EAGL2Support::createWindow(bool,Ogre::GLES2RenderSystem *,std::string const&)")
}

// 0xe86aa0 — __ZN4Ogre12EAGL2Support9newWindowERKSsjjbPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, int, int)
#[doc(alias = "Ogre::EAGL2Support::newWindow(std::string const&,unsigned int,unsigned int,bool,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
pub fn stub_e86aa0() -> ! {
    todo!("0xe86aa0 Ogre::EAGL2Support::newWindow(std::string const&,unsigned int,unsigned int,bool,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")
}

// 0xe86b80 — __ZNK4Ogre12EAGL2Support16createNewContextERPK14__CFDictionaryP11CAEAGLLayerP14EAGLSharegroup
// type: _DWORD __fastcall(Ogre::EAGL2Support *__hidden this, const __CFDictionary **, CAEAGLLayer *, EAGLSharegroup *)
#[doc(alias = "Ogre::EAGL2Support::createNewContext(__CFDictionary const*&,CAEAGLLayer *,EAGLSharegroup *)const")]
pub fn stub_e86b80() -> ! {
    todo!("0xe86b80 Ogre::EAGL2Support::createNewContext(__CFDictionary const*&,CAEAGLLayer *,EAGLSharegroup *)const")
}

// 0xe86d80 — __ZN4Ogre12EAGL2Support14getProcAddressERKSs
// type: void
#[doc(alias = "Ogre::EAGL2Support::getProcAddress(std::string const&)")]
pub fn stub_e86d80() -> ! {
    todo!("0xe86d80 Ogre::EAGL2Support::getProcAddress(std::string const&)")
}

// 0xe86d84 — __ZN4Ogre12EAGL2Support5startEv
// type: _DWORD __fastcall(Ogre::EAGL2Support *__hidden this)
// IDA 0xe86d84
impl Eagl2Support {
    #[doc(alias = "Ogre::EAGL2Support::start(void)")]
    #[doc = "Ogre::EAGL2Support::start(void)"]
    pub fn start(&self) {
        // Single `BX LR` (IDA 0xe86d84): empty body, nothing to start on this slice.
    }
}

// 0xe86d88 — __ZN4Ogre12EAGL2Support4stopEv
// type: _DWORD __fastcall(Ogre::EAGL2Support *__hidden this)
// IDA 0xe86d88
impl Eagl2Support {
    #[doc(alias = "Ogre::EAGL2Support::stop(void)")]
    #[doc = "Ogre::EAGL2Support::stop(void)"]
    pub fn stop(&self) {
        // Single `BX LR` (IDA 0xe86d88): empty body, nothing to stop on this slice.
    }
}

// 0xe87e38 — -[EAGL2View description]
// type: id __cdecl(EAGL2View *self, SEL)
#[doc(alias = "-[EAGL2View description]")]
pub fn stub_e87e38() -> ! {
    todo!("0xe87e38 -[EAGL2View description]")
}

// 0xe87f28 — +[EAGL2View layerClass]
// type: Class __cdecl(id, SEL)
// IDA 0xe87f28
impl Eagl2View {
    #[doc(alias = "+[EAGL2View layerClass]")]
    #[doc = "+[EAGL2View layerClass]"]
    pub fn layer_class() -> &'static str {
        // +[CAEAGLLayer class] via objc_msgSend (IDA 0xe87f44..0xe87f48).
        "CAEAGLLayer" // IDA 0xe87f48
    }
}

// 0xe87f4c — -[EAGL2View layoutSubviews]
// type: void __cdecl(EAGL2View *self, SEL)
#[doc(alias = "-[EAGL2View layoutSubviews]")]
pub fn stub_e87f4c() -> ! {
    todo!("0xe87f4c -[EAGL2View layoutSubviews]")
}

// 0xe880b4 — -[EAGL2View mWindowName]
// type: basic_string<char, std::char_traits<char>, std::allocator<char> > __cdecl(EAGL2View *self, SEL)
// IDA 0xe880b4
impl Eagl2View {
    #[doc(alias = "-[EAGL2View mWindowName]")]
    #[doc = "-[EAGL2View mWindowName]"]
    pub fn m_window_name(&self) -> String {
        // Copy-constructs the return from the `mWindowName` ivar
        // (IDA 0xe880c4..0xe880ca).
        self.window_name.lock().clone() // IDA 0xe880ca
    }
}

// 0xe880cc — -[EAGL2View setMWindowName:]
// type: void __cdecl(EAGL2View *self, SEL, basic_string<char, std::char_traits<char>, std::allocator<char> >)
// IDA 0xe880cc
impl Eagl2View {
    #[doc(alias = "-[EAGL2View setMWindowName:]")]
    #[doc = "-[EAGL2View setMWindowName:]"]
    pub fn set_m_window_name(&self, name: &str) {
        // `std::string::assign` over the `mWindowName` ivar (IDA 0xe880e0).
        *self.window_name.lock() = name.to_owned(); // IDA 0xe880e4
    }
}

// 0xe880e8 — -[EAGL2View .cxx_destruct]
// type: void __cdecl(EAGL2View *self, SEL)
#[doc(alias = "-[EAGL2View .cxx_destruct]")]
pub fn stub_e880e8() -> ! {
    todo!("0xe880e8 -[EAGL2View .cxx_destruct]")
}

// 0xe88140 — -[EAGL2View .cxx_construct]
// type: id __cdecl(EAGL2View *self, SEL)
// IDA 0xe88140
impl Eagl2View {
    #[doc(alias = "-[EAGL2View .cxx_construct]")]
    #[doc = "-[EAGL2View .cxx_construct]"]
    pub fn new() -> Self {
        // Points `mWindowName` at the shared empty rep (IDA 0xe88158..0xe8815a):
        // exactly `String::new()`, which `Default` already gives it.
        Self::default()
    }
}

// 0xe88194 — -[EAGL2ViewController init]
// type: EAGL2ViewController *__cdecl(EAGL2ViewController *self, SEL)
// IDA 0xe88194
impl Eagl2ViewController {
    #[doc(alias = "-[EAGL2ViewController init]")]
    #[doc = "-[EAGL2ViewController init]"]
    pub fn init() -> Self {
        // Only `objc_msgSendSuper2` init (IDA 0xe881b8): UIKit init is out of slice.
        Self::default()
    }
}

// 0xe881c0 — -[EAGL2ViewController initWithNibName:bundle:]
// type: EAGL2ViewController *__cdecl(EAGL2ViewController *self, SEL, id, id)
// IDA 0xe881c0
impl Eagl2ViewController {
    #[doc(alias = "-[EAGL2ViewController initWithNibName:bundle:]")]
    #[doc = "-[EAGL2ViewController initWithNibName:bundle:]"]
    pub fn init_with_nib_name(nib_name: Option<ObjCId>, bundle: Option<ObjCId>) -> Self {
        // Forwards both ids to super (IDA 0xe881ec); UIKit init is out of slice.
        let _ = (nib_name, bundle);
        Self::default()
    }
}

// 0xe881f0 — -[EAGL2ViewController dealloc]
// type: void __cdecl(EAGL2ViewController *self, SEL)
// IDA 0xe881f0
impl Eagl2ViewController {
    #[doc(alias = "-[EAGL2ViewController dealloc]")]
    #[doc = "-[EAGL2ViewController dealloc]"]
    pub fn dealloc(self) {
        // Only `objc_msgSendSuper2` dealloc (IDA 0xe88214): super runs as self drops here.
    }
}

// 0xe8821c — -[EAGL2ViewController didReceiveMemoryWarning]
// type: void __cdecl(EAGL2ViewController *self, SEL)
// IDA 0xe8821c
impl Eagl2ViewController {
    #[doc(alias = "-[EAGL2ViewController didReceiveMemoryWarning]")]
    #[doc = "-[EAGL2ViewController didReceiveMemoryWarning]"]
    pub fn did_receive_memory_warning(&self) {
        // `objc_msgSendSuper2` didReceiveMemoryWarning (IDA 0xe88240);
        // UIKit internals are out of slice.
        self.note_super_forward();
    }
}

// 0xe88248 — -[EAGL2ViewController loadView]
// type: void __cdecl(EAGL2ViewController *self, SEL)
// IDA 0xe88248
impl Eagl2ViewController {
    #[doc(alias = "-[EAGL2ViewController loadView]")]
    #[doc = "-[EAGL2ViewController loadView]"]
    pub fn load_view(&self) {
        // `objc_msgSendSuper2` loadView (IDA 0xe8826c); UIKit internals are out of slice.
        self.note_super_forward();
    }
}

// 0xe88274 — -[EAGL2ViewController viewDidLoad]
// type: void __cdecl(EAGL2ViewController *self, SEL)
// IDA 0xe88274
impl Eagl2ViewController {
    #[doc(alias = "-[EAGL2ViewController viewDidLoad]")]
    #[doc = "-[EAGL2ViewController viewDidLoad]"]
    pub fn view_did_load(&self) {
        // `objc_msgSendSuper2` viewDidLoad (IDA 0xe88298); UIKit internals are out of slice.
        self.note_super_forward();
    }
}

// 0xe882a0 — -[EAGL2ViewController viewDidUnload]
// type: void __cdecl(EAGL2ViewController *self, SEL)
#[doc(alias = "-[EAGL2ViewController viewDidUnload]")]
pub fn stub_e882a0() -> ! {
    todo!("0xe882a0 -[EAGL2ViewController viewDidUnload]")
}

// 0xe882cc — -[EAGL2ViewController shouldAutorotate]
// type: char __cdecl(EAGL2ViewController *self, SEL)
#[doc(alias = "-[EAGL2ViewController shouldAutorotate]")]
pub fn stub_e882cc() -> ! {
    todo!("0xe882cc -[EAGL2ViewController shouldAutorotate]")
}

// 0xe88310 — -[EAGL2ViewController supportedInterfaceOrientations]
// type: unsigned int __cdecl(EAGL2ViewController *self, SEL)
// IDA 0xe88310
impl Eagl2ViewController {
    #[doc(alias = "-[EAGL2ViewController supportedInterfaceOrientations]")]
    #[doc = "-[EAGL2ViewController supportedInterfaceOrientations]"]
    pub fn supported_interface_orientations(&self) -> u32 {
        // MOVS R0, #0x18: landscape-left | landscape-right mask. // IDA 0xe88312
        UI_INTERFACE_ORIENTATION_MASK_LANDSCAPE
    }
}

// 0xe88314 — -[EAGL2ViewController shouldAutorotateToInterfaceOrientation:]
// type: char __cdecl(EAGL2ViewController *self, SEL, int)
// IDA 0xe88314
impl Eagl2ViewController {
    #[doc(alias = "-[EAGL2ViewController shouldAutorotateToInterfaceOrientation:]")]
    #[doc = "-[EAGL2ViewController shouldAutorotateToInterfaceOrientation:]"]
    pub fn should_autorotate_to_interface_orientation(&self, orientation: i32) -> bool {
        // MOVS R0,#1; CMP R2,#4; BXEQ when landscape-right. // IDA 0xe88314
        if orientation == UI_INTERFACE_ORIENTATION_LANDSCAPE_RIGHT { // IDA 0xe88318
            return true; // IDA 0xe8831a
        }
        orientation == UI_INTERFACE_ORIENTATION_LANDSCAPE_LEFT // IDA 0xe88322
    }
}

// 0xe88328 — -[EAGL2ViewController mGLSupport]
// type: EAGL2Support *__cdecl(EAGL2ViewController *self, SEL)
// IDA 0xe88328
impl Eagl2ViewController {
    #[doc(alias = "-[EAGL2ViewController mGLSupport]")]
    #[doc = "-[EAGL2ViewController mGLSupport]"]
    pub fn m_gl_support(&self) -> ObjCId {
        // Assign-ivar load plus the DMB ISH barrier (IDA 0xe88334..0xe88336).
        let support = *self.gl_support.lock();
        std::sync::atomic::fence(std::sync::atomic::Ordering::SeqCst); // IDA 0xe88336
        support // IDA 0xe8833a
    }
}

// 0xe8833c — -[EAGL2ViewController setMGLSupport:]
// type: void __cdecl(EAGL2ViewController *self, SEL, EAGL2Support *)
// IDA 0xe8833c
impl Eagl2ViewController {
    #[doc(alias = "-[EAGL2ViewController setMGLSupport:]")]
    #[doc = "-[EAGL2ViewController setMGLSupport:]"]
    pub fn set_m_gl_support(&self, support: ObjCId) {
        // Assign-ivar store fenced both sides by DMB ISH (IDA 0xe88344..0xe8834e).
        std::sync::atomic::fence(std::sync::atomic::Ordering::SeqCst); // IDA 0xe88344
        *self.gl_support.lock() = support; // IDA 0xe8834c
        std::sync::atomic::fence(std::sync::atomic::Ordering::SeqCst); // IDA 0xe8834e
    }
}

// 0xe88388 — __ZN4Ogre11EAGL2WindowC1EPNS_12EAGL2SupportE
// type: _DWORD __fastcall(Ogre::EAGL2Window *__hidden this, Ogre::EAGL2Support *)
#[doc(alias = "Ogre::EAGL2Window::EAGL2Window(Ogre::EAGL2Support *)")]
pub fn stub_e88388() -> ! {
    todo!("0xe88388 Ogre::EAGL2Window::EAGL2Window(Ogre::EAGL2Support *)")
}

// 0xe884e4 — __ZN4Ogre11EAGL2WindowD0Ev
// type: void __fastcall(Ogre::EAGL2Window *__hidden this)
#[doc(alias = "Ogre::EAGL2Window::~EAGL2Window()")]
pub fn stub_e884e4() -> ! {
    todo!("0xe884e4 Ogre::EAGL2Window::~EAGL2Window()")
}

// 0xe885b8 — __ZN4Ogre11EAGL2WindowD1Ev
// type: void __fastcall(Ogre::EAGL2Window *__hidden this)
#[doc(alias = "Ogre::EAGL2Window::~EAGL2Window()")]
pub fn stub_e885b8() -> ! {
    todo!("0xe885b8 Ogre::EAGL2Window::~EAGL2Window()")
}

// 0xe88680 — __ZN4Ogre11EAGL2Window7destroyEv
// type: _DWORD __fastcall(Ogre::EAGL2Window *__hidden this)
#[doc(alias = "Ogre::EAGL2Window::destroy(void)")]
pub fn stub_e88680() -> ! {
    todo!("0xe88680 Ogre::EAGL2Window::destroy(void)")
}

// 0xe886f8 — __ZN4Ogre11EAGL2Window13setFullscreenEbjj
// type: _DWORD __fastcall(Ogre::EAGL2Window *__hidden this, bool, unsigned int, unsigned int)
// IDA 0xe886f8
impl Eagl2Window {
    #[doc(alias = "Ogre::EAGL2Window::setFullscreen(bool,unsigned int,unsigned int)")]
    #[doc = "Ogre::EAGL2Window::setFullscreen(bool,unsigned int,unsigned int)"]
    pub fn set_fullscreen(&self, fullscreen: bool, width: u32, height: u32) {
        // Single `BX LR` (IDA 0xe886f8): no-op on this slice.
        let _ = (fullscreen, width, height);
    }
}

// 0xe886fc — __ZN4Ogre11EAGL2Window10repositionEii
// type: _DWORD __fastcall(Ogre::EAGL2Window *__hidden this, int, int)
// IDA 0xe886fc
impl Eagl2Window {
    #[doc(alias = "Ogre::EAGL2Window::reposition(int,int)")]
    #[doc = "Ogre::EAGL2Window::reposition(int,int)"]
    pub fn reposition(&self, x: i32, y: i32) {
        // Single `BX LR` (IDA 0xe886fc): no-op on this slice.
        let _ = (x, y);
    }
}

// 0xe88700 — __ZN4Ogre11EAGL2Window6resizeEjj
// type: _DWORD __fastcall(Ogre::EAGL2Window *__hidden this, unsigned int, unsigned int)
#[doc(alias = "Ogre::EAGL2Window::resize(unsigned int,unsigned int)")]
pub fn stub_e88700() -> ! {
    todo!("0xe88700 Ogre::EAGL2Window::resize(unsigned int,unsigned int)")
}

// 0xe88800 — __ZN4Ogre11EAGL2Window20windowMovedOrResizedEv
// type: _DWORD __fastcall(Ogre::EAGL2Window *__hidden this)
#[doc(alias = "Ogre::EAGL2Window::windowMovedOrResized(void)")]
pub fn stub_e88800() -> ! {
    todo!("0xe88800 Ogre::EAGL2Window::windowMovedOrResized(void)")
}

// 0xe88894 — __ZN4Ogre11EAGL2Window12_beginUpdateEv
// type: _DWORD __fastcall(Ogre::EAGL2Window *__hidden this)
#[doc(alias = "Ogre::EAGL2Window::_beginUpdate(void)")]
pub fn stub_e88894() -> ! {
    todo!("0xe88894 Ogre::EAGL2Window::_beginUpdate(void)")
}

// 0xe888bc — __ZN4Ogre11EAGL2Window23initNativeCreatedWindowEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIKSsSsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
// type: void
#[doc(alias = "Ogre::EAGL2Window::initNativeCreatedWindow(std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
pub fn stub_e888bc() -> ! {
    todo!("0xe888bc Ogre::EAGL2Window::initNativeCreatedWindow(std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")
}

// 0xe89488 — __ZN4Ogre11EAGL2Window6createERKSsjjbPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
// type: void
#[doc(alias = "Ogre::EAGL2Window::create(std::string const&,unsigned int,unsigned int,bool,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
pub fn stub_e89488() -> ! {
    todo!("0xe89488 Ogre::EAGL2Window::create(std::string const&,unsigned int,unsigned int,bool,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")
}

// 0xe89c80 — __ZN4Ogre11EAGL2Window11swapBuffersEb
// type: _DWORD __fastcall(Ogre::EAGL2Window *__hidden this, bool)
#[doc(alias = "Ogre::EAGL2Window::swapBuffers(bool)")]
pub fn stub_e89c80() -> ! {
    todo!("0xe89c80 Ogre::EAGL2Window::swapBuffers(bool)")
}

// 0xe89f88 — __ZN4Ogre11EAGL2Window18getCustomAttributeERKSsPv
// type: _DWORD __fastcall(Ogre::EAGL2Window *__hidden this, const std::string *, void *)
#[doc(alias = "Ogre::EAGL2Window::getCustomAttribute(std::string const&,void *)")]
pub fn stub_e89f88() -> ! {
    todo!("0xe89f88 Ogre::EAGL2Window::getCustomAttribute(std::string const&,void *)")
}

// 0xe8a038 — __ZN4Ogre11EAGL2Window20copyContentsToMemoryERKNS_8PixelBoxENS_12RenderTarget11FrameBufferE
// type: void
#[doc(alias = "Ogre::EAGL2Window::copyContentsToMemory(Ogre::PixelBox const&,Ogre::RenderTarget::FrameBuffer)")]
pub fn stub_e8a038() -> ! {
    todo!("0xe8a038 Ogre::EAGL2Window::copyContentsToMemory(Ogre::PixelBox const&,Ogre::RenderTarget::FrameBuffer)")
}

// 0xe8a554 — __ZNK4Ogre11EAGL2Window23requiresTextureFlippingEv
// type: _DWORD __fastcall(Ogre::EAGL2Window *__hidden this)
#[doc(alias = "Ogre::EAGL2Window::requiresTextureFlipping(void)const")]
pub fn stub_e8a554() -> ! {
    todo!("0xe8a554 Ogre::EAGL2Window::requiresTextureFlipping(void)const")
}

// 0xe8a568 — __ZNK4Ogre11EAGL2Window9isVisibleEv
// type: _DWORD __fastcall(Ogre::EAGL2Window *__hidden this)
#[doc(alias = "Ogre::EAGL2Window::isVisible(void)const")]
pub fn stub_e8a568() -> ! {
    todo!("0xe8a568 Ogre::EAGL2Window::isVisible(void)const")
}

// 0xe8a570 — __ZN4Ogre11EAGL2Window10setVisibleEb
// type: _DWORD __fastcall(Ogre::EAGL2Window *__hidden this, bool)
#[doc(alias = "Ogre::EAGL2Window::setVisible(bool)")]
pub fn stub_e8a570() -> ! {
    todo!("0xe8a570 Ogre::EAGL2Window::setVisible(bool)")
}

// 0xe8a590 — __ZNK4Ogre11EAGL2Window8isClosedEv
// type: _DWORD __fastcall(Ogre::EAGL2Window *__hidden this)
#[doc(alias = "Ogre::EAGL2Window::isClosed(void)const")]
pub fn stub_e8a590() -> ! {
    todo!("0xe8a590 Ogre::EAGL2Window::isClosed(void)const")
}
