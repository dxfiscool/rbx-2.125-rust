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
    url: parking_lot::Mutex<String>,
    delegate: parking_lot::Mutex<Option<ObjCId>>,
    scales_page_to_fit: std::sync::atomic::AtomicBool,
    user_interaction_enabled: std::sync::atomic::AtomicBool,
    frame: parking_lot::Mutex<(f64, f64, f64, f64)>,
    load_requests: std::sync::atomic::AtomicU32,
}

impl WebView {
    pub fn new(superview: ObjCId) -> Self {
        Self { superview: parking_lot::Mutex::new(superview), ..Self::default() }
    }
    pub fn remove_from_superview(&self) {
        *self.superview.lock() = NIL_ID;
    }
    pub fn superview(&self) -> ObjCId {
        *self.superview.lock()
    }
}
impl WebView {
    pub fn set_delegate(&self, delegate: Option<ObjCId>) {
        *self.delegate.lock() = delegate;
    }
    pub fn set_scales_page_to_fit(&self, fit: bool) {
        self.scales_page_to_fit.store(fit, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn set_user_interaction_enabled(&self, enabled: bool) {
        self.user_interaction_enabled.store(enabled, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn set_frame(&self, frame: (f64, f64, f64, f64)) {
        *self.frame.lock() = frame;
    }
    pub fn frame(&self) -> (f64, f64, f64, f64) {
        *self.frame.lock()
    }
    pub fn load_request(&self, url: &str) {
        *self.url.lock() = url.to_owned();
        self.load_requests.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn url(&self) -> String {
        self.url.lock().clone()
    }
    pub fn load_request_count(&self) -> u32 {
        self.load_requests.load(std::sync::atomic::Ordering::SeqCst)
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
    present_requests: std::sync::atomic::AtomicU32,
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
    pub fn present_request_count(&self) -> u32 {
        self.present_requests.load(std::sync::atomic::Ordering::SeqCst)
    }
}

// 0x26768 — -[PlaceLauncher presentGameViewController]
// type: void __cdecl(PlaceLauncher *self, SEL)
// IDA 0x26768
impl PlaceLauncher {
    #[doc(alias = "-[PlaceLauncher presentGameViewController]")]
    #[doc = "-[PlaceLauncher presentGameViewController]"]
    pub fn present_game_view_controller(&self) {
        // `dispatch_async(main, __block_literal_global505)` (IDA 0x2677e);
        // the queue hop has no host counterpart, so the block runs inline.
        self.present_requests.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

/// `EAGLViewController` base (composition models the ObjC superclass).
/// Tracks appearance plus the base-level memory-warning count; the subclass
/// overrides below forward to these exactly like the `objc_super` sends.
#[derive(Debug, Default)]
pub struct EaglViewController {
    appearing: std::sync::atomic::AtomicBool,
    appeared: std::sync::atomic::AtomicBool,
    loaded: std::sync::atomic::AtomicBool,
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
    pub fn view_did_load(&self) {
        self.loaded.store(true, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn did_load(&self) -> bool {
        self.loaded.load(std::sync::atomic::Ordering::SeqCst)
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
/// `webviewTweenTime` (0.3s close animation), the signup/login controller slots
/// (`+168` externalWebView lives in `external_web_view`), the presented controller,
/// plus counters for the animation/signal blocks below.
#[derive(Debug, Default)]
pub struct GameViewController {
    base: EaglViewController,
    game_view: GameView,
    external_web_view: parking_lot::Mutex<Option<WebView>>,
    webview_tween_time: parking_lot::Mutex<f32>,
    signup_view_controller: parking_lot::Mutex<Option<ObjCId>>,
    login_view_controller: parking_lot::Mutex<Option<ObjCId>>,
    presented_view_controller: parking_lot::Mutex<Option<ObjCId>>,
    close_animation_requests: std::sync::atomic::AtomicU32,
    url_window_closed_signals: std::sync::atomic::AtomicU32,
    login_prompt_dispatches: std::sync::atomic::AtomicU32,
    signup_prompt_dispatches: std::sync::atomic::AtomicU32,
    login_ok_emits: std::sync::atomic::AtomicU32,
    login_failed_emits: std::sync::atomic::AtomicU32,
    last_login_ok_user: parking_lot::Mutex<String>,
    last_login_error: parking_lot::Mutex<String>,
    notification_registrations: std::sync::atomic::AtomicU32,
    pending_login_success: parking_lot::Mutex<Option<bool>>,
    login_notification_blocks: std::sync::atomic::AtomicU32,
    pending_open_url: parking_lot::Mutex<Option<String>>,
    url_window_opens: std::sync::atomic::AtomicU32,
}
impl GameViewController {
    fn objc_id(&self) -> ObjCId {
        self as *const Self as ObjCId
    }
    pub fn webview_tween_time(&self) -> f32 {
        *self.webview_tween_time.lock()
    }
    pub fn signup_view_controller_id(&self) -> Option<ObjCId> {
        *self.signup_view_controller.lock()
    }
    pub fn login_view_controller_id(&self) -> Option<ObjCId> {
        *self.login_view_controller.lock()
    }
    pub fn presented_view_controller(&self) -> Option<ObjCId> {
        *self.presented_view_controller.lock()
    }
    pub fn close_animation_request_count(&self) -> u32 {
        self.close_animation_requests.load(std::sync::atomic::Ordering::SeqCst)
    }
    pub fn url_window_closed_signal_count(&self) -> u32 {
        self.url_window_closed_signals.load(std::sync::atomic::Ordering::SeqCst)
    }
    pub fn login_prompt_dispatch_count(&self) -> u32 {
        self.login_prompt_dispatches.load(std::sync::atomic::Ordering::SeqCst)
    }
    pub fn signup_prompt_dispatch_count(&self) -> u32 {
        self.signup_prompt_dispatches.load(std::sync::atomic::Ordering::SeqCst)
    }
    pub fn login_ok_emit_count(&self) -> u32 {
        self.login_ok_emits.load(std::sync::atomic::Ordering::SeqCst)
    }
    pub fn login_failed_emit_count(&self) -> u32 {
        self.login_failed_emits.load(std::sync::atomic::Ordering::SeqCst)
    }
    pub fn last_login_ok_user(&self) -> String {
        self.last_login_ok_user.lock().clone()
    }
    pub fn last_login_error(&self) -> String {
        self.last_login_error.lock().clone()
    }
}
// 0x4d70c — -[GameViewController initWithNibName:bundle:]
// type: GameViewController *__cdecl(GameViewController *self, SEL, id, id)
// IDA 0x4d70c
impl GameViewController {
    #[doc(alias = "-[GameViewController initWithNibName:bundle:]")]
    #[doc = "-[GameViewController initWithNibName:bundle:]"]
    pub fn init_with_nib_name(nib: Option<ObjCId>, bundle: Option<ObjCId>) -> Self {
        // Super `initWithNibName:bundle:`, a `GameView` sized to the main-screen
        // bounds installed via `setView:`, then signup/login-finished
        // notification registrations (IDA 0x4d70c..0x4d8c0); nib/bundle ids
        // only forward to super, out of slice.
        let _ = (nib, bundle);
        let this = Self::default();
        this.game_view.add_subview(1);
        this.notification_registrations
            .fetch_add(2, std::sync::atomic::Ordering::SeqCst);
        this
    }
    pub fn notification_registration_count(&self) -> u32 {
        self.notification_registrations.load(std::sync::atomic::Ordering::SeqCst)
    }
}

// 0x4dc08 — -[GameViewController closeUrlWindow:]
// type: void __cdecl(GameViewController *self, SEL, id)
// IDA 0x4dc08
impl GameViewController {
    #[doc(alias = "-[GameViewController closeUrlWindow:]")]
    #[doc = "-[GameViewController closeUrlWindow:]"]
    pub fn close_url_window(&self, sender: ObjCId) {
        // `getControlView` → `getGame`, then
        // `signalGuiServiceUrlWindowClosedOnDataModel:` over the bound game,
        // then the close animation block on the main queue
        // (IDA 0x4dc08..0x4de50); `sender` only selects the control, out of slice.
        let _ = sender;
        self.signal_gui_service_url_window_closed_on_data_model(1);
        self.close_animation_requests
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x4de58 — ___37-[GameViewController closeUrlWindow:]_block_invoke
// type: id __fastcall(_DWORD *)
// IDA 0x4de58
impl GameViewController {
    #[doc(alias = "___37-[GameViewController closeUrlWindow:]_block_invoke")]
    #[doc = "GameViewController closeUrlWindow animation block"]
    pub fn animate_close_url_window(&self) -> f32 {
        // `animateWithDuration:webviewTweenTime ... animations:completion:`
        // (IDA 0x4de58..0x4df14); returns the tween time the animation runs.
        self.webview_tween_time()
    }
}

// 0x4df1c — ___37-[GameViewController closeUrlWindow:]_block_invoke_2
// type: id __fastcall(int)
// IDA 0x4df1c
impl GameViewController {
    #[doc(alias = "___37-[GameViewController closeUrlWindow:]_block_invoke_2")]
    #[doc = "GameViewController closeUrlWindow animations block"]
    pub fn close_url_window_animation_frame(&self, frame: (f64, f64, f64, f64)) {
        // Animations block: `setFrame:` to the fullscreen frame
        // (IDA 0x4df1c..0x4dfe4).
        if let Some(web) = self.external_web_view.lock().as_ref() {
            web.set_frame(frame);
        }
    }
}

// 0x4dfec — ___37-[GameViewController closeUrlWindow:]_block_invoke93
// type: id __fastcall(int)
// IDA 0x4dfec
impl GameViewController {
    #[doc(alias = "___37-[GameViewController closeUrlWindow:]_block_invoke93")]
    #[doc = "GameViewController closeUrlWindow completion block"]
    pub fn close_url_window_animation_done(&self) {
        // Completion block: `removeFromSuperview` + `release`
        // (IDA 0x4dfec..0x4e068); taking the slot drops the release.
        if let Some(web) = self.external_web_view.lock().take() {
            web.remove_from_superview();
        }
    }
}

// 0x4e070 — -[GameViewController closeUrlWindow]
// type: void __cdecl(GameViewController *self, SEL)
// IDA 0x4e070
impl GameViewController {
    #[doc(alias = "-[GameViewController closeUrlWindow]")]
    #[doc = "-[GameViewController closeUrlWindow]"]
    pub fn close_url_window_now(&self) {
        // Forwards nil sender: `closeUrlWindow:nil` (IDA 0x4e070..0x4e082).
        self.close_url_window(NIL_ID);
    }
}

// 0x4e084 — -[GameViewController openUrlWindow:]
// type: void __cdecl(GameViewController *self, SEL, basic_string<char, std::char_traits<char>, std::allocator<char> >)
// IDA 0x4e084
impl GameViewController {
    #[doc(alias = "-[GameViewController openUrlWindow:]")]
    #[doc = "-[GameViewController openUrlWindow:]"]
    pub fn open_url_window(&self, url: &str) {
        // Sizes to the main-screen bounds, checks the idiom, then runs the
        // build block on the main queue (IDA 0x4e084..0x4e2a4).
        *self.pending_open_url.lock() = Some(url.to_owned());
        self.url_window_opens.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn pending_open_url(&self) -> Option<String> {
        self.pending_open_url.lock().clone()
    }
    pub fn url_window_open_count(&self) -> u32 {
        self.url_window_opens.load(std::sync::atomic::Ordering::SeqCst)
    }
}

// 0x4e2ac — ___36-[GameViewController openUrlWindow:]_block_invoke
// type: id __fastcall(int)
// IDA 0x4e2ac
impl GameViewController {
    #[doc(alias = "___36-[GameViewController openUrlWindow:]_block_invoke")]
    #[doc = "GameViewController openUrlWindow build block"]
    pub fn open_url_window_build_webview(&self, frame: (f64, f64, f64, f64)) {
        // `UIWebView alloc` + `initWithFrame:`, `setDelegate:self`,
        // `setUserInteractionEnabled:YES`, `setScalesPageToFit:YES`
        // (IDA 0x4e2ac..0x4e4d4).
        let web = WebView::new(self.objc_id());
        web.set_delegate(Some(self.objc_id()));
        web.set_user_interaction_enabled(true);
        web.set_scales_page_to_fit(true);
        web.set_frame(frame);
        *self.external_web_view.lock() = Some(web);
    }
}

// 0x4e4dc — ___36-[GameViewController openUrlWindow:]_block_invoke136
// type: id __fastcall(int)
// IDA 0x4e4dc
impl GameViewController {
    #[doc(alias = "___36-[GameViewController openUrlWindow:]_block_invoke136")]
    #[doc = "GameViewController openUrlWindow load block"]
    pub fn open_url_window_load(&self, url: &str) {
        // `stringWithUTF8String:` → `URLWithString:` → `requestWithURL:` →
        // `loadRequest:`, then the present animation (IDA 0x4e4dc..0x4e5f4).
        if let Some(web) = self.external_web_view.lock().as_ref() {
            web.load_request(url);
        }
        self.close_animation_requests
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x4e5fc — ___36-[GameViewController openUrlWindow:]_block_invoke_2
// type: id __fastcall(_DWORD *)
// IDA 0x4e5fc
impl GameViewController {
    #[doc(alias = "___36-[GameViewController openUrlWindow:]_block_invoke_2")]
    #[doc = "GameViewController openUrlWindow layout block"]
    pub fn open_url_window_layout(&self, frame: (f64, f64, f64, f64)) {
        // Layout block: `setFrame:` to the presented frame
        // (IDA 0x4e5fc..0x4e728).
        if let Some(web) = self.external_web_view.lock().as_ref() {
            web.set_frame(frame);
        }
    }
}

// 0x4e730 — -[GameViewController handlePromptLoginSignal]
// type: void __cdecl(GameViewController *self, SEL)
// IDA 0x4e730
impl GameViewController {
    #[doc(alias = "-[GameViewController handlePromptLoginSignal]")]
    #[doc = "-[GameViewController handlePromptLoginSignal]"]
    pub fn handle_prompt_login_signal(&self) {
        // `dispatch_async(main, login-prompt block)` (IDA 0x4e730..0x4e778).
        self.login_prompt_dispatches
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x4e780 — ___45-[GameViewController handlePromptLoginSignal]_block_invoke
// type: id __fastcall(int)
// IDA 0x4e780
impl GameViewController {
    #[doc(alias = "___45-[GameViewController handlePromptLoginSignal]_block_invoke")]
    #[doc = "GameViewController login prompt block"]
    pub fn present_login_view_controller(&self) -> ObjCId {
        // Instantiates `LoginViewController` from `UIMainStoryboardFile` and
        // presents it animated (IDA 0x4e780..0x4e860).
        let _ = main_storyboard_file();
        let id = self.objc_id().wrapping_add(1);
        *self.login_view_controller.lock() = Some(id);
        *self.presented_view_controller.lock() = Some(id);
        id
    }
}

// 0x4e868 — -[GameViewController handlePromptSignupSignal]
// type: void __cdecl(GameViewController *self, SEL)
// IDA 0x4e868
impl GameViewController {
    #[doc(alias = "-[GameViewController handlePromptSignupSignal]")]
    #[doc = "-[GameViewController handlePromptSignupSignal]"]
    pub fn handle_prompt_signup_signal(&self) {
        // `dispatch_async(main, signup-prompt block)` (IDA 0x4e868..0x4e8b0).
        self.signup_prompt_dispatches
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x4e8b8 — ___46-[GameViewController handlePromptSignupSignal]_block_invoke
// type: id __fastcall(int)
// IDA 0x4e8b8
impl GameViewController {
    #[doc(alias = "___46-[GameViewController handlePromptSignupSignal]_block_invoke")]
    #[doc = "GameViewController signup prompt block"]
    pub fn present_signup_view_controller(&self) -> ObjCId {
        // Instantiates `SignupViewController` from `UIMainStoryboardFile` and
        // presents it animated (IDA 0x4e8b8..0x4e998).
        let _ = main_storyboard_file();
        let id = self.objc_id().wrapping_add(2);
        *self.signup_view_controller.lock() = Some(id);
        *self.presented_view_controller.lock() = Some(id);
        id
    }
}

// 0x4e9a0 — -[GameViewController handleSignupNotification:]
// type: void __cdecl(GameViewController *self, SEL, id)
// IDA 0x4e9a0
impl GameViewController {
    #[doc(alias = "-[GameViewController handleSignupNotification:]")]
    #[doc = "-[GameViewController handleSignupNotification:]"]
    pub fn handle_signup_notification(&self, username: &str, password: &str) {
        // `userInfo["username"]` / `userInfo["password"]` into
        // `LoginManager::doLoginWithUsername:password:`
        // (IDA 0x4e9a0..0x4ea28).
        LoginManager::shared_instance().do_login_with_username_password(username, password);
    }
}

// 0x4ea30 — -[GameViewController handleLoginNotification:]
// type: void __cdecl(GameViewController *self, SEL, id)
// IDA 0x4ea30
impl GameViewController {
    #[doc(alias = "-[GameViewController handleLoginNotification:]")]
    #[doc = "-[GameViewController handleLoginNotification:]"]
    pub fn handle_login_notification(&self, success: bool) {
        // Captures `userInfo["success"].boolValue` and runs the login block
        // on the main queue (IDA 0x4ea30..0x4eac0).
        *self.pending_login_success.lock() = Some(success);
        self.login_notification_blocks
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn login_notification_block_count(&self) -> u32 {
        self.login_notification_blocks.load(std::sync::atomic::Ordering::SeqCst)
    }
}

// 0x4eac8 — ___46-[GameViewController handleLoginNotification:]_block_invoke
// type: void __fastcall(id *)
// IDA 0x4eac8
impl GameViewController {
    #[doc(alias = "___46-[GameViewController handleLoginNotification:]_block_invoke")]
    #[doc = "GameViewController login notification block"]
    pub fn apply_login_notification(&self, success: bool, username: &str, error: &str) {
        // `getControlView` → `getGame` → `create<LoginService>`; on success
        // emits the username signal, otherwise the error signal
        // (IDA 0x4eac8..0x4efa0).
        if success {
            *self.last_login_ok_user.lock() = username.to_owned();
            self.login_ok_emits.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        } else {
            *self.last_login_error.lock() = error.to_owned();
            self.login_failed_emits
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
    }
}

/// `FFlag::OpenNativeBrowserWindowFromLua` (IDA 0x4dbde): when set, URL loads
/// detour through the in-app-purchase check instead of loading directly.
pub static OPEN_NATIVE_BROWSER_WINDOW_FROM_LUA: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

/// Sets `FFlag::OpenNativeBrowserWindowFromLua` (test seam).
pub fn set_open_native_browser_window_from_lua(enabled: bool) {
    OPEN_NATIVE_BROWSER_WINDOW_FROM_LUA.store(enabled, std::sync::atomic::Ordering::SeqCst);
}

/// Minimal `RobloxInfo` counterpart behind `+[RobloxInfo getUserAgentString]`
/// (IDA 0x4da4c): the UA string registered into `NSUserDefaults`; extended
/// with the device/URL classifier cluster (IDA 0x36058..0x36e04). `sysctl`,
/// `UIDevice`, and `NSBundle` reads live out of slice: pure classifiers take
/// those values as parameters, while the `dword_130C460/464/468` caches are
/// mutex-guarded slots.
#[derive(Debug, Default)]
pub struct RobloxInfo {
    user_agent: parking_lot::Mutex<String>,
    base_url: parking_lot::Mutex<Option<String>>,
    api_base_url: parking_lot::Mutex<Option<String>>,
    domain: parking_lot::Mutex<Option<String>>,
    base_url_sets: std::sync::atomic::AtomicU32,
    base_url_posts: std::sync::atomic::AtomicU32,
    settings_refresh_dispatches: std::sync::atomic::AtomicU32,
}

impl RobloxInfo {
    pub fn shared() -> &'static Self {
        static INFO: std::sync::LazyLock<RobloxInfo> =
            std::sync::LazyLock::new(RobloxInfo::default);
        &INFO
    }
    pub fn user_agent_string(&self) -> String {
        self.user_agent.lock().clone()
    }
    pub fn set_user_agent(&self, agent: &str) {
        *self.user_agent.lock() = agent.to_owned();
    }
    // 0x36058 — +[RobloxInfo getDeviceType]
    // type: id __cdecl(id, SEL)
    // IDA 0x36058
    #[doc(alias = "+[RobloxInfo getDeviceType]")]
    #[doc = "+[RobloxInfo getDeviceType]"]
    pub fn device_class(device_type: Option<&str>) -> &'static str {
        // `rangeOfString:` probes for iPad, then iPhone, then iPod
        // (IDA 0x36098..0x360f6), else `Unknown`.
        // BUG: original at 0x36104..0x36108 returns `iPad` when `deviceType`
        // is nil.
        match device_type {
            None => "iPad",
            Some(t) if t.contains("iPad") => "iPad",
            Some(t) if t.contains("iPhone") => "iPhone",
            Some(t) if t.contains("iPod") => "iPod",
            _ => "Unknown",
        }
    }
    /// Digit right after `token` in `haystack` (`characterAtIndex:loc+len`
    /// + `atoi`; non-digit reads as 0, IDA 0x361fa..0x36228).
    fn digit_after(haystack: &str, token: &str) -> i32 {
        let pos = match haystack.find(token) {
            Some(pos) => pos + token.len(),
            None => return 0,
        };
        haystack[pos..].chars().next().and_then(|c| c.to_digit(10)).unwrap_or(0) as i32
    }
    // 0x36114 — +[RobloxInfo getDeviceModelNumber]
    // type: int __cdecl(id, SEL)
    // IDA 0x36114
    #[doc(alias = "+[RobloxInfo getDeviceModelNumber]")]
    #[doc = "+[RobloxInfo getDeviceModelNumber]"]
    pub fn device_model_number(device_type: Option<&str>, tablet: bool) -> i32 {
        // Tablet: `atoi` past `iPad` (IDA 0x3615e..0x36180, -1 without it);
        // phone: `iPod` first (IDA 0x36198..0x361a4), else past `iPhone`
        // (IDA 0x361b6..0x361c2, -1 without it). A nil `deviceType` reads 0
        // through the nil receiver (IDA 0x361e0..0x361e4, 0x36202..0x36208).
        let Some(device) = device_type else {
            return 0;
        };
        if tablet {
            if !device.contains("iPad") {
                return -1;
            }
            return Self::digit_after(device, "iPad");
        }
        if device.contains("iPod") {
            return Self::digit_after(device, "iPod");
        }
        if !device.contains("iPhone") {
            return -1;
        }
        Self::digit_after(device, "iPhone")
    }
    // 0x3622c — +[RobloxInfo thisDeviceIsATablet]
    // type: char __cdecl(id, SEL)
    // IDA 0x3622c
    #[doc(alias = "+[RobloxInfo thisDeviceIsATablet]")]
    #[doc = "+[RobloxInfo thisDeviceIsATablet]"]
    pub fn this_device_is_a_tablet(supports_idiom: bool, idiom: i32) -> bool {
        // `respondsToSelector:userInterfaceIdiom` gate (IDA 0x3626c..0x36274);
        // the Pad idiom (1) survives the `!= 1 → 0` fold (IDA 0x36282..0x3628a).
        supports_idiom && idiom == 1
    }
    // 0x36290 — +[RobloxInfo deviceType]
    // type: id __cdecl(id, SEL)
    // IDA 0x36290
    #[doc(alias = "+[RobloxInfo deviceType]")]
    #[doc = "+[RobloxInfo deviceType]"]
    pub fn device_type(machine: &str) -> String {
        // `sysctlbyname("hw.machine")` → `stringWithUTF8String:`
        // (IDA 0x362b2..0x362fa); the sysctl itself lives out of slice.
        machine.to_owned()
    }
    // 0x362fc — +[RobloxInfo deviceOSVersion]
    // type: id __cdecl(id, SEL)
    // IDA 0x362fc
    #[doc(alias = "+[RobloxInfo deviceOSVersion]")]
    #[doc = "+[RobloxInfo deviceOSVersion]"]
    pub fn device_os_version(version: &str) -> String {
        // `UIDevice.systemVersion` (IDA 0x36318..0x36322).
        version.to_owned()
    }
    // 0x36330 — +[RobloxInfo appVersion]
    // type: id __cdecl(id, SEL)
    // IDA 0x36330
    #[doc(alias = "+[RobloxInfo appVersion]")]
    #[doc = "+[RobloxInfo appVersion]"]
    pub fn app_version(version: &str) -> String {
        // `objectForInfoDictionaryKey:CFBundleShortVersionString`
        // (IDA 0x3634c..0x36356).
        version.to_owned()
    }
    // 0x36370 — +[RobloxInfo friendlyDeviceName]
    // type: id __cdecl(id, SEL)
    // IDA 0x36370
    #[doc(alias = "+[RobloxInfo friendlyDeviceName]")]
    #[doc = "+[RobloxInfo friendlyDeviceName]"]
    pub fn friendly_device_name(machine: &str) -> &'static str {
        // `isEqualToString:` ladder over `hw.machine` (IDA 0x36390..0x36836).
        match machine {
            "iPhone1,1" => "iPhone 2G",
            "iPhone1,2" => "iPhone 3G",
            "iPhone2,1" => "iPhone 3GS",
            "iPhone3,1" | "iPhone3,2" => "iPhone 4",
            "iPhone3,3" => "iPhone 4 (CDMA)",
            "iPhone4,1" => "iPhone 4S",
            "iPhone5,1" => "iPhone 5",
            "iPhone5,2" => "iPhone 5 (GSM+CDMA)",
            "iPod1,1" => "iPod Touch (1 Gen)",
            "iPod2,1" => "iPod Touch (2 Gen)",
            "iPod3,1" => "iPod Touch (3 Gen)",
            "iPod4,1" => "iPod Touch (4 Gen)",
            "iPod5,1" => "iPod Touch (5 Gen)",
            "iPad1,1" => "iPad",
            "iPad1,2" => "iPad 3G",
            "iPad2,1" => "iPad 2 (WiFi)",
            "iPad2,2" | "iPad2,4" => "iPad 2",
            "iPad2,3" => "iPad 2 (CDMA)",
            "iPad2,5" => "iPad Mini (WiFi)",
            "iPad2,6" => "iPad Mini",
            "iPad2,7" => "iPad Mini (GSM+CDMA)",
            "iPad3,1" => "iPad 3 (WiFi)",
            "iPad3,2" => "iPad 3 (GSM+CDMA)",
            "iPad3,3" => "iPad 3",
            "iPad3,4" => "iPad 4 (WiFi)",
            "iPad3,5" => "iPad 4",
            "iPad3,6" => "iPad 4 (GSM+CDMA)",
            "i386" => "Simulator 32 bit intel",
            "x86_64" => "Simulator 64 bit intel",
            _ => "Unknown",
        }
    }
    // 0x3683c — +[RobloxInfo getUserAgentString]
    // type: id __cdecl(id, SEL)
    // IDA 0x3683c
    #[doc(alias = "+[RobloxInfo getUserAgentString]")]
    #[doc = "+[RobloxInfo getUserAgentString]"]
    pub fn build_user_agent_string(model: &str, device_type: &str, os_version: &str, app_version: &str) -> String {
        // `model`, `deviceType`, `systemVersion`, `CFBundleShortVersionString`
        // into the Mozilla/5.0 template (IDA 0x36870..0x36914).
        format!(
            "Mozilla/5.0 ({model}; {device_type}; CPU iPhone OS {os_version} like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko) Mobile/9B176 ROBLOX iOS App {app_version}"
        )
    }
    // 0x36918 — +[RobloxInfo getBaseUrl]
    // type: id __cdecl(id, SEL)
    // IDA 0x36918
    #[doc(alias = "+[RobloxInfo getBaseUrl]")]
    #[doc = "+[RobloxInfo getBaseUrl]"]
    pub fn get_base_url(tablet: bool, plist_url: &str) -> String {
        // Cached in `dword_130C460` (IDA 0x36926..0x3692c); on miss reads
        // `RbxBaseUrl` (tablet, IDA 0x36988..0x36994) or `RbxBaseMobileUrl`
        // (IDA 0x36994..0x369a0), stores, then re-enters `setBaseUrl:`
        // (IDA 0x369ae..0x369b6).
        // BUG: original at 0x369b6 re-enters `setBaseUrl:` with no URL
        // argument; the plist value is threaded through here instead.
        if let Some(cached) = Self::shared().base_url.lock().clone() {
            return cached;
        }
        let _ = tablet;
        Self::set_base_url(plist_url)
    }
    /// `https://api` + first-dot suffix of `base` (IDA 0x36a18..0x36a9e);
    /// empty base stays nil (IDA 0x36a10).
    pub fn api_base_url_for(base: &str) -> Option<String> {
        if base.is_empty() {
            return None;
        }
        let trimmed = base.trim_end_matches('/');
        let dot = trimmed.find('.')?;
        Some(format!("https://api{}", &trimmed[dot..]))
    }
    // 0x369c0 — +[RobloxInfo getApiBaseUrl]
    // type: id __cdecl(id, SEL)
    // IDA 0x369c0
    #[doc(alias = "+[RobloxInfo getApiBaseUrl]")]
    #[doc = "+[RobloxInfo getApiBaseUrl]"]
    pub fn get_api_base_url(base: &str) -> Option<String> {
        // Cached in `dword_130C464` (IDA 0x369d4..0x36aac).
        if let Some(cached) = Self::shared().api_base_url.lock().clone() {
            return Some(cached);
        }
        let url = Self::api_base_url_for(base)?;
        *Self::shared().api_base_url.lock() = Some(url.clone());
        Some(url)
    }
    /// First-dot suffix of `base` minus scheme and `/`
    /// (IDA 0x36b30..0x36bb0); empty base stays nil (IDA 0x36b06).
    pub fn domain_string_for(base: &str) -> Option<String> {
        if base.is_empty() {
            return None;
        }
        let no_scheme = base.strip_prefix("http://").unwrap_or(base);
        let dot = no_scheme.find('.')?;
        Some(no_scheme[dot..].replace('/', ""))
    }
    // 0x36ab0 — +[RobloxInfo getDomainString]
    // type: id __cdecl(id, SEL)
    // IDA 0x36ab0
    #[doc(alias = "+[RobloxInfo getDomainString]")]
    #[doc = "+[RobloxInfo getDomainString]"]
    pub fn get_domain_string(base: &str) -> Option<String> {
        // Cached in `dword_130C468` (IDA 0x36aca..0x36bc6).
        if let Some(cached) = Self::shared().domain.lock().clone() {
            return Some(cached);
        }
        let domain = Self::domain_string_for(base)?;
        *Self::shared().domain.lock() = Some(domain.clone());
        Some(domain)
    }
    // 0x36bc8 — +[RobloxInfo getBaseUrlChangedNotification]
    // type: id __cdecl(id, SEL)
    // IDA 0x36bc8
    #[doc(alias = "+[RobloxInfo getBaseUrlChangedNotification]")]
    #[doc = "+[RobloxInfo getBaseUrlChangedNotification]"]
    pub fn base_url_changed_notification() -> &'static str {
        // `RBXBaseUrlChangedNotifier` (IDA 0x36bd2).
        "RBXBaseUrlChangedNotifier"
    }
    // 0x36bd4 — +[RobloxInfo setBaseUrl:]
    // type: void __cdecl(id, SEL, id)
    // IDA 0x36bd4
    #[doc(alias = "+[RobloxInfo setBaseUrl:]")]
    #[doc = "+[RobloxInfo setBaseUrl:]"]
    pub fn set_base_url(url: &str) -> String {
        // Stores `dword_130C460` (IDA 0x36c08), appends `/` unless suffixed
        // (IDA 0x36c48..0x36c70), pushes the UTF-8 bytes through `SetBaseURL`
        // (IDA 0x36c86..0x36c9e; the `std::string` rep dance is a Rust drop),
        // dispatches the settings refresh to main (IDA 0x36cce), posts
        // `RBXBaseUrlChangedNotifier` (IDA 0x36cf0..0x36d12), and initializes
        // analytics (IDA 0x36d30).
        let normalized = if url.ends_with('/') {
            url.to_owned()
        } else {
            format!("{url}/")
        };
        *Self::shared().base_url.lock() = Some(normalized.clone());
        Self::shared().base_url_sets.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Self::set_base_url_block();
        Self::shared().base_url_posts.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        normalized
    }
    // 0x36de4 — ___25+[RobloxInfo setBaseUrl:]_block_invoke
    // type: void __cdecl(id)
    // IDA 0x36de4
    #[doc(alias = "___25+[RobloxInfo setBaseUrl:]_block_invoke")]
    #[doc = "___25+[RobloxInfo setBaseUrl:]_block_invoke"]
    pub fn set_base_url_block() {
        // `getiOSSettingsServiceWithForcedReadFromWeb:NO` (IDA 0x36dfe).
        Self::shared().settings_refresh_dispatches.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        WebUtility::get_ios_settings_service_with_forced_read_from_web(false);
    }
    // 0x36e04 — +[RobloxInfo searchUrl]
    // type: id __cdecl(id, SEL)
    // IDA 0x36e04
    #[doc(alias = "+[RobloxInfo searchUrl]")]
    #[doc = "+[RobloxInfo searchUrl]"]
    pub fn search_url(tablet: bool, phone_url: &str, pad_url: &str) -> String {
        // Settings service with no forced web read (IDA 0x36e2a..0x36e58);
        // the tablet flag picks `var30` (pad, IDA 0x36e68..0x36e6a) over
        // `var31` (phone).
        WebUtility::get_ios_settings_service_with_forced_read_from_web(false);
        if tablet {
            pad_url.to_owned()
        } else {
            phone_url.to_owned()
        }
    }
}

/// Minimal `RobloxNavBarViewController` counterpart behind
/// `+checkForInAppPurchases:navigationType:` (IDA 0x4dbde): nonzero means the
/// navigation was consumed by the store sheet, so the web view must not load it.
#[derive(Debug, Default)]
pub struct RobloxNavBarViewController {
    in_app_check_result: std::sync::atomic::AtomicI32,
}

impl RobloxNavBarViewController {
    pub fn shared() -> &'static Self {
        static NAV: std::sync::LazyLock<RobloxNavBarViewController> =
            std::sync::LazyLock::new(RobloxNavBarViewController::default);
        &NAV
    }
    pub fn check_for_in_app_purchases(&self, _request: ObjCId, _navigation_type: i32) -> i32 {
        self.in_app_check_result.load(std::sync::atomic::Ordering::SeqCst)
    }
    pub fn set_in_app_check_result(&self, result: i32) {
        self.in_app_check_result.store(result, std::sync::atomic::Ordering::SeqCst);
    }
}

/// Process-wide count of `RBX::GuiService` url-window-closed emissions
/// (`signal+0x78`, IDA 0x4dbe8..0x4dc06): `find<GuiService>` on a nil
/// `DataModel` short-circuits before any emit.
pub static GUI_SERVICE_URL_WINDOW_CLOSED_SIGNALS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);

/// `UIMainStoryboardFile` from the main bundle info dictionary (IDA 0x4e7e4):
/// the storyboard the login/signup prompt blocks instantiate from.
static MAIN_STORYBOARD_FILE: std::sync::LazyLock<parking_lot::Mutex<String>> =
    std::sync::LazyLock::new(|| parking_lot::Mutex::new(String::from("Main")));

/// Stages `UIMainStoryboardFile` for the prompt blocks (test seam).
pub fn set_main_storyboard_file(name: &str) {
    *MAIN_STORYBOARD_FILE.lock() = name.to_owned();
}

/// Storyboard name the login/signup prompt blocks instantiate from.
pub fn main_storyboard_file() -> String {
    MAIN_STORYBOARD_FILE.lock().clone()
}

/// Next presenter-allocated controller id (models `instantiateViewControllerWithIdentifier:`).
static NEXT_CONTROLLER_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);

/// Allocates the id `instantiateViewControllerWithIdentifier:` would return.
fn next_controller_id() -> ObjCId {
    NEXT_CONTROLLER_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

/// Minimal `Ogre::EAGL2Support` counterpart: the `addConfig` option list plus a
/// count of created windows (GL context setup itself is out of slice).
#[derive(Debug, Default)]
pub struct Eagl2Support {
    configs: parking_lot::Mutex<Vec<RenderConfigOption>>,
    windows_created: std::sync::atomic::AtomicU32,
    next_context_id: std::sync::atomic::AtomicU32,
}

/// One `Ogre::ConfigOption` from `EAGL2Support::addConfig`: the name, the
/// possible values, and the selected value.
#[derive(Debug, Clone, Default)]
pub struct RenderConfigOption {
    pub name: String,
    pub values: Vec<String>,
    pub selected: String,
}

/// Minimal `Ogre::EAGL2Window` counterpart: the `+148` closed and `+149`
/// visible bytes, geometry, context/support ids, and counters for the
/// framebuffer/swap/viewport paths.
#[derive(Debug, Default)]
pub struct Eagl2Window {
    support: parking_lot::Mutex<Option<ObjCId>>,
    width: std::sync::atomic::AtomicU32,
    height: std::sync::atomic::AtomicU32,
    fullscreen: std::sync::atomic::AtomicBool,
    visible: std::sync::atomic::AtomicBool,
    closed: std::sync::atomic::AtomicBool,
    context: parking_lot::Mutex<Option<ObjCId>>,
    video_mode: parking_lot::Mutex<String>,
    buffers_swapped: std::sync::atomic::AtomicU32,
    framebuffer_rebuilds: std::sync::atomic::AtomicU32,
    viewport_updates: std::sync::atomic::AtomicU32,
    begin_updates: std::sync::atomic::AtomicU32,
    releases: std::sync::atomic::AtomicU32,
}

/// Minimal `EAGL2View` counterpart: the `mWindowName` std::string ivar plus the
/// `CAEAGLLayer` layer class (backing-store rendering is out of slice).
#[derive(Debug, Default)]
pub struct Eagl2View {
    window_name: parking_lot::Mutex<String>,
    layout_passes: std::sync::atomic::AtomicU32,
    last_orientation: std::sync::atomic::AtomicI32,
}

/// Minimal `EAGL2ViewController` counterpart: the `mGLSupport` assign ivar plus
/// a count of the UIKit super-sends (`init`, `loadView`, ...) that are out of slice.
/// `window_hidden` backs `shouldAutorotate` (IDA 0xe882cc): rotation is allowed
/// exactly while `[[self view] window]` is visible (`isHidden == 0`).
#[derive(Debug, Default)]
pub struct Eagl2ViewController {
    gl_support: parking_lot::Mutex<ObjCId>,
    super_forwards: std::sync::atomic::AtomicU32,
    window_hidden: std::sync::atomic::AtomicBool,
}
impl Eagl2ViewController {
    pub fn super_forward_count(&self) -> u32 {
        self.super_forwards.load(std::sync::atomic::Ordering::SeqCst)
    }
    fn note_super_forward(&self) {
        self.super_forwards
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn set_window_hidden(&self, hidden: bool) {
        self.window_hidden.store(hidden, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn is_window_hidden(&self) -> bool {
        self.window_hidden.load(std::sync::atomic::Ordering::SeqCst)
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

/// `Appirater` configuration counterpart (IDA 0x1953a..0x1959a, 0x19bf0) plus the
/// full rating-prompt state machine from the `Appirater` class cluster
/// (IDA 0x17df0..0x19224): class-level config globals (`_MergedGlobals243`,
/// `_daysUntilPrompt`, `_MergedGlobals`, `dword_122316C`,
/// `_timeBeforeReminding`, `_debug`, `dword_130C394`) and the shared-instance
/// ivars (`ratingAlert`, `_delegate`). `NSUserDefaults` persistence
/// (`kAppiraterFirstUseDate`, `kAppiraterUseCount`, ...) is modeled as plain
/// host state; `UIAlertView`/`NSURLConnection` objects collapse to flags and
/// `ObjCId` tokens (`None`/`0` is `nil`).
#[derive(Debug, Default)]
pub struct Appirater {
    app_id: parking_lot::Mutex<String>,
    days_until_prompt: parking_lot::Mutex<f64>,
    uses_until_prompt: std::sync::atomic::AtomicU32,
    significant_events_until_prompt: std::sync::atomic::AtomicU32,
    time_before_reminding: parking_lot::Mutex<f64>,
    debug: std::sync::atomic::AtomicBool,
    pending_delegate: parking_lot::Mutex<ObjCId>,
    delegate: parking_lot::Mutex<ObjCId>,
    rating_alert: parking_lot::Mutex<ObjCId>,
    rating_alert_visible: std::sync::atomic::AtomicBool,
    rating_alert_shows: std::sync::atomic::AtomicU32,
    resign_active_observed: std::sync::atomic::AtomicBool,
    use_count: std::sync::atomic::AtomicU32,
    significant_event_count: std::sync::atomic::AtomicU32,
    first_use_date_secs: parking_lot::Mutex<f64>,
    reminder_request_date_secs: parking_lot::Mutex<f64>,
    declined_to_rate: std::sync::atomic::AtomicBool,
    rated_current_version: std::sync::atomic::AtomicBool,
    network_reachable: std::sync::atomic::AtomicBool,
    delegate_display_notifies: std::sync::atomic::AtomicU32,
    app_launched_calls: std::sync::atomic::AtomicU32,
    entered_foreground_calls: std::sync::atomic::AtomicU32,
}

impl Appirater {
    fn shared() -> &'static Self {
        static APPIRATER: std::sync::LazyLock<Appirater> = std::sync::LazyLock::new(|| {
            let appirater = Appirater::default();
            // Reachability flags read clean on a live device, so the
            // `connectedToNetwork` (IDA 0x17e68) fast path starts reachable.
            appirater.network_reachable.store(true, std::sync::atomic::Ordering::SeqCst);
            appirater
        });
        &APPIRATER
    }
    /// `+sharedInstance` (IDA 0x17f80): `dispatch_once` materialization.
    pub fn shared_instance() -> &'static Self {
        Self::shared()
    }
    /// Block `__27+[Appirater sharedInstance]_block_invoke` (IDA 0x17fe4):
    /// `[[Appirater alloc] init]`, `setDelegate:` from the class-level
    /// delegate slot, observer for `UIApplicationWillResignActiveNotification`.
    pub fn init_shared() -> &'static Self {
        let inst = Self::shared();
        *inst.delegate.lock() = *inst.pending_delegate.lock();
        inst.resign_active_observed.store(true, std::sync::atomic::Ordering::SeqCst);
        inst
    }
    pub fn resign_active_observed() -> bool {
        Self::shared().resign_active_observed.load(std::sync::atomic::Ordering::SeqCst)
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
    /// `+setSignificantEventsUntilPrompt:`.
    pub fn set_significant_events_until_prompt(count: u32) {
        Self::shared().significant_events_until_prompt.store(count, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn significant_events_until_prompt() -> u32 {
        Self::shared().significant_events_until_prompt.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `+setTimeBeforeReminding:`.
    pub fn set_time_before_reminding(days: f64) {
        *Self::shared().time_before_reminding.lock() = days;
    }
    pub fn time_before_reminding() -> f64 {
        *Self::shared().time_before_reminding.lock()
    }
    /// `+setDebug:`.
    pub fn set_debug(debug: bool) {
        Self::shared().debug.store(debug, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn is_debug() -> bool {
        Self::shared().debug.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `+setDelegate:` (class-level slot consumed by `init_shared`).
    pub fn set_class_delegate(delegate: ObjCId) {
        *Self::shared().pending_delegate.lock() = delegate;
    }
    pub fn class_delegate() -> ObjCId {
        *Self::shared().pending_delegate.lock()
    }
    /// `-[Appirater delegate]` / `-[Appirater setDelegate:]` (instance ivar).
    pub fn set_delegate(&self, delegate: ObjCId) {
        *self.delegate.lock() = delegate;
    }
    pub fn delegate(&self) -> ObjCId {
        *self.delegate.lock()
    }
    /// `-[Appirater ratingAlert]` / `-[Appirater setRatingAlert:]` (retained ivar).
    pub fn set_rating_alert(&self, alert: ObjCId) {
        *self.rating_alert.lock() = alert;
    }
    pub fn rating_alert(&self) -> ObjCId {
        *self.rating_alert.lock()
    }
    /// `-[Appirater connectedToNetwork]` (IDA 0x17e68): zeroed `sockaddr`
    /// reachability probe plus a test `NSURLConnection` to apple.com. The
    /// connection alloc cannot fail on device, so a good-flags result is
    /// exactly "reachable"; the host keeps only that flag.
    pub fn connected_to_network(&self) -> bool {
        self.network_reachable.load(std::sync::atomic::Ordering::SeqCst)
    }
    pub fn set_network_reachable(reachable: bool) {
        Self::shared().network_reachable.store(reachable, std::sync::atomic::Ordering::SeqCst);
    }
    /// `-[Appirater showRatingAlert]` (IDA 0x180a8): builds the `UIAlertView`
    /// from the `RatingTitle`/`RatingString`/button locals, retains it into
    /// `ratingAlert`, shows it, and pings `appiraterDidDisplayAlert:` when the
    /// delegate answers it. UIKit strings collapse; the token is non-`nil`.
    pub fn show_rating_alert(&self) {
        *self.rating_alert.lock() = 1;
        self.rating_alert_visible.store(true, std::sync::atomic::Ordering::SeqCst);
        self.rating_alert_shows.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        if *self.delegate.lock() != NIL_ID {
            self.delegate_display_notifies.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
    }
    pub fn rating_alert_show_count(&self) -> u32 {
        self.rating_alert_shows.load(std::sync::atomic::Ordering::SeqCst)
    }
    pub fn is_rating_alert_visible(&self) -> bool {
        self.rating_alert_visible.load(std::sync::atomic::Ordering::SeqCst)
    }
    pub fn delegate_display_notify_count(&self) -> u32 {
        self.delegate_display_notifies.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// `-[Appirater ratingConditionsHaveBeenMet]` (IDA 0x183d8): short-circuit
    /// chain over the `kAppirater*` defaults. `_debug` forces true (0x183f6);
    /// time gates use `>=`, count gates use strict `>` (0x184aa/0x184dc/0x184f6).
    pub fn rating_conditions_have_been_met(&self, now_secs: f64) -> bool {
        if self.debug.load(std::sync::atomic::Ordering::SeqCst) {
            return true;
        }
        if now_secs - *self.first_use_date_secs.lock()
            < *self.days_until_prompt.lock() * 86400.0
        {
            return false;
        }
        if self.use_count.load(std::sync::atomic::Ordering::SeqCst)
            <= self.uses_until_prompt.load(std::sync::atomic::Ordering::SeqCst)
        {
            return false;
        }
        if self.significant_event_count.load(std::sync::atomic::Ordering::SeqCst)
            <= self.significant_events_until_prompt.load(std::sync::atomic::Ordering::SeqCst)
        {
            return false;
        }
        if self.declined_to_rate.load(std::sync::atomic::Ordering::SeqCst) {
            return false;
        }
        if self.rated_current_version.load(std::sync::atomic::Ordering::SeqCst) {
            return false;
        }
        now_secs - *self.reminder_request_date_secs.lock()
            >= *self.time_before_reminding.lock() * 86400.0
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
    login_calls: std::sync::atomic::AtomicU32,
    last_login: parking_lot::Mutex<Option<(String, String)>>,
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
    /// `-doLoginWithUsername:password:` (IDA 0x4ea2c): records the credential
    /// pair the signup notification forwards; the network exchange is out of slice.
    pub fn do_login_with_username_password(&self, username: &str, password: &str) {
        *self.last_login.lock() = Some((username.to_owned(), password.to_owned()));
        self.login_calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn login_call_count(&self) -> u32 {
        self.login_calls.load(std::sync::atomic::Ordering::SeqCst)
    }
    pub fn last_login(&self) -> Option<(String, String)> {
        self.last_login.lock().clone()
    }
}

/// Minimal `LoginViewController` counterpart: `+sharedInstance` plus the place-id
/// sinks `TryLaunchPlace:` drives (IDA 0x1a364..0x1a47a) and the
/// `segueToHomeViewController:` animation chain (IDA 0x1f840..0x201a8).
/// UIKit objects (storyboard, logo, button views) live out of slice;
/// flags/counters record the observable flow.
#[derive(Debug, Default)]
pub struct LoginViewController {
    login_place_id: std::sync::atomic::AtomicI32,
    jump_to_place_id: std::sync::atomic::AtomicI32,
    jump_to_place_id_game_in_progress: std::sync::atomic::AtomicI32,
    web_button_taps: std::sync::atomic::AtomicU32,
    home_segues: std::sync::atomic::AtomicU32,
    home_segue_dispatches: std::sync::atomic::AtomicU32,
    last_segue_after_load: std::sync::atomic::AtomicBool,
    home_instantiations: std::sync::atomic::AtomicU32,
    home_after_load_marks: std::sync::atomic::AtomicU32,
    logo_fades: std::sync::atomic::AtomicU32,
    background_pan_stops: std::sync::atomic::AtomicU32,
    foreground_captures: std::sync::atomic::AtomicU32,
    home_presentations: std::sync::atomic::AtomicU32,
    logging_in_stops: std::sync::atomic::AtomicU32,
    button_restores: std::sync::atomic::AtomicU32,
    roblox_logo: parking_lot::Mutex<ObjCId>,
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
    // 0x1f840 — -[LoginViewController externalSegueToHomeViewController:]
    // type: void __cdecl(LoginViewController *self, SEL, id)
    // IDA 0x1f840
    #[doc(alias = "-[LoginViewController externalSegueToHomeViewController:]")]
    #[doc = "-[LoginViewController externalSegueToHomeViewController:]"]
    pub fn external_segue_to_home_view_controller(&self) {
        // Forwards to `segueToHomeViewController:NO` (IDA 0x1f84e).
        self.segue_to_home_view_controller(false);
    }
    // 0x1f854 — -[LoginViewController segueToHomeViewController:]
    // type: void __cdecl(LoginViewController *self, SEL, char)
    // IDA 0x1f854
    #[doc(alias = "-[LoginViewController segueToHomeViewController:]")]
    #[doc = "-[LoginViewController segueToHomeViewController:]"]
    pub fn segue_to_home_view_controller(&self, after_load: bool) {
        // Captures `a3` into the stack block and hops to the main queue for
        // `block_invoke` (IDA 0x1f888..0x1f8a4); the queue hop lives out of
        // slice, so the dispatch is recorded with its flag.
        self.last_segue_after_load.store(after_load, std::sync::atomic::Ordering::SeqCst);
        self.home_segue_dispatches.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.home_segues.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    // 0x1f8b0 — ___49-[LoginViewController segueToHomeViewController:]_block_invoke
    // IDA 0x1f8b0
    #[doc(alias = "___49-[LoginViewController segueToHomeViewController:]_block_invoke")]
    #[doc = "___49-[LoginViewController segueToHomeViewController:]_block_invoke"]
    pub fn segue_animation_setup(&self, after_load: bool) -> bool {
        // Instantiates `HomeViewController` from `UIMainStoryboardFile`
        // (IDA 0x1f8e6..0x1f94c); when the captured flag is set, marks
        // `viewMustSegueAfterLoad` on it (IDA 0x1f958..0x1f96c), then runs the
        // 0.3s fade `animateWithDuration:animations:completion:` pair
        // (IDA 0x1f9b4..0x1fa0e). The storyboard/animator live out of slice.
        self.home_instantiations.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        if after_load {
            self.home_after_load_marks.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
        after_load
    }
    // 0x1fa18 — ___49-[LoginViewController segueToHomeViewController:]_block_invoke_2
    // type: id __fastcall(int)
    // IDA 0x1fa18
    #[doc(alias = "___49-[LoginViewController segueToHomeViewController:]_block_invoke_2")]
    #[doc = "___49-[LoginViewController segueToHomeViewController:]_block_invoke_2"]
    pub fn segue_logo_fade(&self) {
        // `robloxLogo.alpha = 0` (IDA 0x1fa2a).
        self.logo_fades.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    // 0x1fa58 — ___49-[LoginViewController segueToHomeViewController:]_block_invoke342
    // IDA 0x1fa58
    #[doc(alias = "___49-[LoginViewController segueToHomeViewController:]_block_invoke342")]
    #[doc = "___49-[LoginViewController segueToHomeViewController:]_block_invoke342"]
    pub fn segue_completion(&self, after_load: bool, presented: bool, animating: bool) -> bool {
        // `stopBackgroundPan` (IDA 0x1fa72); with a live page animator
        // (`!v2[169]`, IDA 0x1fa84..0x1fa86) snapshots the foreground /
        // background presentation-layer X into the home controller
        // (IDA 0x1fa94..0x1fb62, zeroed without a layer); when the captured
        // flag is set, marks `viewMustSegueAfterLoad` (IDA 0x1fb6a..0x1fb7a),
        // then `presentViewController:animated:NO` with the 2353 completion
        // (IDA 0x1fb98..0x1fbd6).
        self.background_pan_stops.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        if presented && !animating {
            self.foreground_captures.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
        if after_load {
            self.home_after_load_marks.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
        self.home_presentations.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        true
    }
    // 0x1fbd8 — ___49-[LoginViewController segueToHomeViewController:]_block_invoke_2353
    // IDA 0x1fbd8
    #[doc(alias = "___49-[LoginViewController segueToHomeViewController:]_block_invoke_2353")]
    #[doc = "___49-[LoginViewController segueToHomeViewController:]_block_invoke_2353"]
    pub fn segue_present_completion(&self) {
        // `stopShowLoggingIn` (IDA 0x1fbee) then restores `buttonView.alpha`
        // via the 0.3s animation block (IDA 0x1fc2a..0x1fc56).
        self.logging_in_stops.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.segue_button_restore();
    }
    // 0x1fc60 — ___49-[LoginViewController segueToHomeViewController:]_block_invoke_3
    // type: id __fastcall(int)
    // IDA 0x1fc60
    #[doc(alias = "___49-[LoginViewController segueToHomeViewController:]_block_invoke_3")]
    #[doc = "___49-[LoginViewController segueToHomeViewController:]_block_invoke_3"]
    pub fn segue_button_restore(&self) {
        // `buttonView.alpha = 1.0` (IDA 0x1fc72, 1065353216 = 1.0f).
        self.button_restores.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    // 0x201a8 — -[LoginViewController setRobloxLogo:]
    // type: void __cdecl(LoginViewController *self, SEL, id)
    // IDA 0x201a8
    #[doc(alias = "-[LoginViewController setRobloxLogo:]")]
    #[doc = "-[LoginViewController setRobloxLogo:]"]
    pub fn set_roblox_logo(&self, logo: ObjCId) {
        // `objc_setProperty(self, a2, 244, a3, 0, 0)` (IDA 0x201c4):
        // retained ivar store at offset 244.
        *self.roblox_logo.lock() = logo;
    }
    pub fn roblox_logo(&self) -> ObjCId {
        *self.roblox_logo.lock()
    }
}

/// Minimal `RobloxAlert` counterpart: the `UIAlertView` rows behind
/// `+RobloxAlertWithMessage:[...]` (IDA 0x35d3c..0x35fdc). Presentation lives
/// out of slice; requests are recorded with their localized keys.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AlertRequest {
    pub title_key: &'static str,
    pub message: String,
    pub delegate: ObjCId,
    pub cancel_key: &'static str,
    pub other_key: Option<&'static str>,
}

#[derive(Debug, Default)]
pub struct RobloxAlert {
    alerts: parking_lot::Mutex<Vec<AlertRequest>>,
    main_dispatches: std::sync::atomic::AtomicU32,
}

impl RobloxAlert {
    fn shared() -> &'static Self {
        static ALERTS: std::sync::LazyLock<RobloxAlert> =
            std::sync::LazyLock::new(RobloxAlert::default);
        &ALERTS
    }
    // 0x35d3c — +[RobloxAlert RobloxAlertWithMessage:]
    // type: void __cdecl(id, SEL, id)
    // IDA 0x35d3c
    #[doc(alias = "+[RobloxAlert RobloxAlertWithMessage:]")]
    #[doc = "+[RobloxAlert RobloxAlertWithMessage:]"]
    pub fn alert_with_message(message: &str) {
        // Captures `a3` into the stack block and hops to the main queue for
        // `block_invoke` (IDA 0x35d70..0x35d82); the queue hop lives out of
        // slice, so the dispatch is recorded and the block runs inline.
        Self::shared().main_dispatches.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Self::alert_with_message_block(message);
    }
    // 0x35d8c — ___38+[RobloxAlert RobloxAlertWithMessage:]_block_invoke
    // IDA 0x35d8c
    #[doc(alias = "___38+[RobloxAlert RobloxAlertWithMessage:]_block_invoke")]
    #[doc = "___38+[RobloxAlert RobloxAlertWithMessage:]_block_invoke"]
    pub fn alert_with_message_block(message: &str) {
        // Title `RobloxWord` (the key pointer at IDA 0x35de8..0x35e06 resolves
        // to the `RobloxWord` CFString), message = captured `a3`
        // (IDA 0x35e0e), delegate nil, cancel `OkWord` (IDA 0x35e12..0x35e4c);
        // `show` then `release` (IDA 0x35e5c).
        Self::shared().alerts.lock().push(AlertRequest {
            title_key: "RobloxWord",
            message: message.to_owned(),
            delegate: NIL_ID,
            cancel_key: "OkWord",
            other_key: None,
        });
    }
    // 0x35e90 — +[RobloxAlert RobloxAlertWithMessageAndDelegate:Delegate:]
    // type: void __cdecl(id, SEL, id, id)
    // IDA 0x35e90
    #[doc(alias = "+[RobloxAlert RobloxAlertWithMessageAndDelegate:Delegate:]")]
    #[doc = "+[RobloxAlert RobloxAlertWithMessageAndDelegate:Delegate:]"]
    pub fn alert_with_message_and_delegate(message: &str, delegate: ObjCId) {
        // Captures `(a3, a4)` into the block and hops to the main queue
        // (IDA 0x35ec4..0x35edc); runs inline here.
        Self::shared().main_dispatches.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Self::alert_with_message_and_delegate_block(message, delegate);
    }
    // 0x35ee4 — ___58+[RobloxAlert RobloxAlertWithMessageAndDelegate:Delegate:]_block_invoke
    // IDA 0x35ee4
    #[doc(alias = "___58+[RobloxAlert RobloxAlertWithMessageAndDelegate:Delegate:]_block_invoke")]
    #[doc = "___58+[RobloxAlert RobloxAlertWithMessageAndDelegate:Delegate:]_block_invoke"]
    pub fn alert_with_message_and_delegate_block(message: &str, delegate: ObjCId) {
        // Title `RobloxWord`, cancel `CancelWord`, other `OkWord`, delegate =
        // captured `a4` (IDA 0x35f2a..0x35fcc); `show` then `release`
        // (IDA 0x35fdc).
        Self::shared().alerts.lock().push(AlertRequest {
            title_key: "RobloxWord",
            message: message.to_owned(),
            delegate,
            cancel_key: "CancelWord",
            other_key: Some("OkWord"),
        });
    }
    pub fn alert_count() -> u32 {
        Self::shared().alerts.lock().len() as u32
    }
    pub fn last_alert() -> Option<AlertRequest> {
        Self::shared().alerts.lock().last().cloned()
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
    }
    pub fn cxx_construct(&self) {
        // self->messageOutConnection.con.weak_slot.p_ = 0; return self.
        // The `new()` constructor returns Self instead of the ObjC `id`.
        self.message_out_connection.lock().reset_weak_slot(); // IDA 0x1a5ca
    }
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
        // [super viewWillAppear:animated]; nothing else. // IDA 0x4d990
        self.base.view_will_appear(animated);
    }
}
// 0x4da00 — -[GameViewController viewDidLoad]
// type: void __cdecl(GameViewController *self, SEL)
// IDA 0x4da00
impl GameViewController {
    #[doc(alias = "-[GameViewController viewDidLoad]")]
    pub fn view_did_load(&self) {
        // [super viewDidLoad] (IDA 0x4da24).
        self.base.view_did_load();
        // Register the UA string as a `UserAgent` default (IDA 0x4da4c..0x4da9e);
        // the temporary dictionary is released right after (IDA 0x4dab0).
        let agent = RobloxInfo::shared().user_agent_string();
        UserDefaults::standard().set_object(&agent, "UserAgent");
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
        orientation == 4
    }
}
// 0x4db9c — -[GameViewController webView:shouldStartLoadWithRequest:navigationType:]
// type: char __cdecl(GameViewController *self, SEL, id, id, int)
// IDA 0x4db9c
impl GameViewController {
    #[doc(alias = "-[GameViewController webView:shouldStartLoadWithRequest:navigationType:]")]
    #[doc = "-[GameViewController webView:shouldStartLoadWithRequest:navigationType:]"]
    pub fn web_view_should_start_load_with_request(&self, request: ObjCId, navigation_type: i32) -> bool {
        // `!FFlag::OpenNativeBrowserWindowFromLua || check == 0` (IDA 0x4dbde).
        if !OPEN_NATIVE_BROWSER_WINDOW_FROM_LUA.load(std::sync::atomic::Ordering::SeqCst) {
            return true;
        }
        RobloxNavBarViewController::shared().check_for_in_app_purchases(request, navigation_type) == 0
    }
}
// 0x4dbe8 — -[GameViewController signalGuiServiceUrlWindowClosedOnDataModel:]
// type: void __cdecl(GameViewController *self, SEL, DataModel *)
// IDA 0x4dbe8
impl GameViewController {
    #[doc(alias = "-[GameViewController signalGuiServiceUrlWindowClosedOnDataModel:]")]
    #[doc = "-[GameViewController signalGuiServiceUrlWindowClosedOnDataModel:]"]
    pub fn signal_gui_service_url_window_closed_on_data_model(&self, data_model: ObjCId) {
        // Nil data model returns before the `find<GuiService>` lookup (IDA 0x4dbea..0x4dbf0).
        if data_model == NIL_ID {
            return;
        }
        // `find<GuiService>` hit (IDA 0x4dbf4..0x4dbfa), then emit `signal+0x78`
        // (IDA 0x4dbfc..0x4dc02); a miss (BEQ 0x4dc06) emits nothing.
        self.url_window_closed_signals.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        GUI_SERVICE_URL_WINDOW_CLOSED_SIGNALS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}
// IDA 0x4db30 getControlView
impl GameViewController {
    pub fn get_control_view(&self) -> Option<ObjCId> {
        // Fast-enumerates gameView.subviews and returns the first object,
        // nil when the list is empty. IDA 0x4db62, IDA 0x4db88, IDA 0x4db80
        self.game_view.first_subview()
    }
}



// 0xe844ec — Ogre::EAGL2Support::EAGL2Support(void)
// IDA 0xe844ec
impl Eagl2Support {
    #[doc(alias = "Ogre::EAGL2Support::EAGL2Support(void)")]
    #[doc = "Ogre::EAGL2Support::EAGL2Support(void)"]
    pub fn new() -> Self {
        // `GLES2Support` base ctor plus zeroed fields (IDA 0xe844ec..0xe8455a);
        // the GL base itself is out of slice.
        Self::default()
    }
}

// 0xe8455c — Ogre::EAGL2Support::~EAGL2Support()
// IDA 0xe8455c
impl Eagl2Support {
    #[doc(alias = "Ogre::EAGL2Support::~EAGL2Support()")]
    #[doc = "Ogre::EAGL2Support::~EAGL2Support()"]
    pub fn delete_d0(&self) {
        // D0 runs D1 then `operator delete` (IDA 0xe8455c..0xe8456e).
        self.destroy_d1();
    }
}

// 0xe84570 — Ogre::EAGL2Support::~EAGL2Support()
// IDA 0xe84570
impl Eagl2Support {
    #[doc(alias = "Ogre::EAGL2Support::~EAGL2Support()")]
    #[doc = "Ogre::EAGL2Support::~EAGL2Support()"]
    pub fn destroy_d1(&self) {
        // `GLES2Support::D2` (IDA 0xe84570..0xe8457a) is out of slice;
        // dropping the option list is the owned-state teardown.
        *self.configs.lock() = Vec::new();
    }
}

// 0xe8457c — Ogre::EAGL2Support::addConfig(void)
// IDA 0xe8457c
impl Eagl2Support {
    #[doc(alias = "Ogre::EAGL2Support::addConfig(void)")]
    #[doc = "Ogre::EAGL2Support::addConfig(void)"]
    pub fn add_config(&self) {
        // Builds the `ConfigOption` map (IDA 0xe8457c..0xe862dc): fullscreen
        // variants, video modes from `applicationFrame`, frequency, content
        // scale, FSAA levels, and RTT modes. First value is selected, per Ogre.
        let opt = |name: &str, values: &[&str]| RenderConfigOption {
            name: name.to_owned(),
            values: values.iter().map(|v| v.to_string()).collect(),
            selected: values[0].to_owned(),
        };
        *self.configs.lock() = vec![
            opt("Full Screen", &["Yes", "No"]),
            opt("Video Mode", &["320 x 480", "768 x 1024"]),
            opt("Display Frequency", &["0 Hz"]),
            opt("Content Scaling Factor", &["1.0", "1.33", "1.5", "2.0"]),
            opt("FSAA", &["0", "2", "4"]),
            opt("RTT Preferred Mode", &["Copy", "FBO"]),
        ];
    }
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

// 0xe862e4 — Ogre::EAGL2Support::createWindow(bool,Ogre::GLES2RenderSystem *,std::string const&)
// IDA 0xe862e4
impl Eagl2Support {
    #[doc(alias = "Ogre::EAGL2Support::createWindow(bool,Ogre::GLES2RenderSystem *,std::string const&)")]
    #[doc = "Ogre::EAGL2Support::createWindow(bool,Ogre::GLES2RenderSystem *,std::string const&)"]
    pub fn create_window(&self, fullscreen: bool, name: &str) -> Eagl2Window {
        // Sizes from `applicationFrame`, builds the window (IDA 0xe862e4..0xe8687c);
        // the render-system half is out of slice. `name` keys the view layer.
        let _ = name;
        self.windows_created.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Eagl2Window {
            fullscreen: std::sync::atomic::AtomicBool::new(fullscreen),
            ..Eagl2Window::default()
        }
    }
}

// 0xe86aa0 — Ogre::EAGL2Support::newWindow(std::string const&,unsigned int,unsigned int,bool,std::map<std::string,std::string> const*)
// IDA 0xe86aa0
impl Eagl2Support {
    #[doc(alias = "Ogre::EAGL2Support::newWindow(std::string const&,unsigned int,unsigned int,bool,std::map<std::string,std::string> const*)")]
    #[doc = "Ogre::EAGL2Support::newWindow(std::string const&,unsigned int,unsigned int,bool,std::map<std::string,std::string> const*)"]
    pub fn new_window(&self, name: &str, width: u32, height: u32, fullscreen: bool) -> Eagl2Window {
        // `NedPoolingImpl::allocBytes` + `EAGL2WindowC1` (IDA 0xe86aa0..0xe86b78);
        // `Box` is the pool. `name` keys the view layer.
        let _ = name;
        self.windows_created.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Eagl2Window {
            width: std::sync::atomic::AtomicU32::new(width),
            height: std::sync::atomic::AtomicU32::new(height),
            fullscreen: std::sync::atomic::AtomicBool::new(fullscreen),
            ..Eagl2Window::default()
        }
    }
}

// 0xe86b80 — Ogre::EAGL2Support::createNewContext(__CFDictionary const*&,CAEAGLLayer *,EAGLSharegroup *)const
// IDA 0xe86b80
impl Eagl2Support {
    #[doc(alias = "Ogre::EAGL2Support::createNewContext(__CFDictionary const*&,CAEAGLLayer *,EAGLSharegroup *)const")]
    #[doc = "Ogre::EAGL2Support::createNewContext(__CFDictionary const*&,CAEAGLLayer *,EAGLSharegroup *)const"]
    pub fn create_new_context(&self, layer: Option<ObjCId>) -> Result<ObjCId, String> {
        // `EAGLES2Context(layer, sharegroup)`; a nil layer throws
        // `Ogre::Exception("Fail to create new context")` (IDA 0xe86b80..0xe86d76).
        match layer {
            Some(_) => Ok(self.next_context_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst) as ObjCId + 1),
            None => Err("Fail to create new context".to_owned()),
        }
    }
}

// 0xe86d80 — Ogre::EAGL2Support::getProcAddress(std::string const&)
// IDA 0xe86d80
impl Eagl2Support {
    #[doc(alias = "Ogre::EAGL2Support::getProcAddress(std::string const&)")]
    #[doc = "Ogre::EAGL2Support::getProcAddress(std::string const&)"]
    pub fn get_proc_address(&self, _name: &str) -> Option<ObjCId> {
        // Single `BX LR` returning null (IDA 0xe86d80..0xe86d83).
        None
    }
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
// IDA 0xe87e38
impl Eagl2View {
    #[doc(alias = "-[EAGL2View description]")]
    #[doc = "-[EAGL2View description]"]
    pub fn describe(&self, frame: (f64, f64, f64, f64)) -> String {
        // `stringWithFormat:` over the view frame (IDA 0xe87e38..0xe87f1c).
        format!("{{{}, {}}}, {{{}, {}}}", frame.0, frame.1, frame.2, frame.3)
    }
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
// IDA 0xe87f4c
impl Eagl2View {
    #[doc(alias = "-[EAGL2View layoutSubviews]")]
    #[doc = "-[EAGL2View layoutSubviews]"]
    pub fn layout_subviews(&self, orientation: i32) {
        // Device-orientation notifications bracket the Root/render-system/
        // viewport-camera refresh (IDA 0xe87f4c..0xe880ac); GL is out of slice.
        self.layout_passes.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.last_orientation.store(orientation, std::sync::atomic::Ordering::SeqCst);
    }
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
// IDA 0xe880e8
impl Eagl2View {
    #[doc(alias = "-[EAGL2View .cxx_destruct]")]
    #[doc = "-[EAGL2View .cxx_destruct]"]
    pub fn cxx_destruct(&self) {
        // Destroys the `mWindowName` std::string (IDA 0xe880e8..0xe8813c).
        *self.window_name.lock() = String::new();
    }
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
// IDA 0xe882a0
impl Eagl2ViewController {
    #[doc(alias = "-[EAGL2ViewController viewDidUnload]")]
    #[doc = "-[EAGL2ViewController viewDidUnload]"]
    pub fn view_did_unload(&self) {
        // Only `objc_msgSendSuper2` viewDidUnload (IDA 0xe882a0..0xe882c8).
        self.note_super_forward();
    }
}

// 0xe882cc — -[EAGL2ViewController shouldAutorotate]
// IDA 0xe882cc
impl Eagl2ViewController {
    #[doc(alias = "-[EAGL2ViewController shouldAutorotate]")]
    #[doc = "-[EAGL2ViewController shouldAutorotate]"]
    pub fn should_autorotate(&self) -> bool {
        // `![[[self view] window] isHidden]` (IDA 0xe882cc..0xe88310).
        !self.is_window_hidden()
    }
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

// 0xe88388 — Ogre::EAGL2Window::EAGL2Window(Ogre::EAGL2Support *)
// IDA 0xe88388
impl Eagl2Window {
    #[doc(alias = "Ogre::EAGL2Window::EAGL2Window(Ogre::EAGL2Support *)")]
    #[doc = "Ogre::EAGL2Window::EAGL2Window(Ogre::EAGL2Support *)"]
    pub fn new(support: ObjCId) -> Self {
        // `RenderWindow` base plus the device `systemVersion` check
        // (IDA 0xe88388..0xe884dc); the Ogre base is out of slice.
        // A fresh window is visible (`+149 = 1`), not closed (`+148 = 0`).
        Self {
            support: parking_lot::Mutex::new(Some(support)),
            visible: std::sync::atomic::AtomicBool::new(true),
            ..Self::default()
        }
    }
}

// 0xe884e4 — Ogre::EAGL2Window::~EAGL2Window()
// IDA 0xe884e4
impl Eagl2Window {
    #[doc(alias = "Ogre::EAGL2Window::~EAGL2Window()")]
    #[doc = "Ogre::EAGL2Window::~EAGL2Window()"]
    pub fn delete_d0(&self) {
        // D0 runs D1 then `deallocBytes` (IDA 0xe884e4..0xe885b0).
        self.destroy_d1();
    }
}

// 0xe885b8 — Ogre::EAGL2Window::~EAGL2Window()
// IDA 0xe885b8
impl Eagl2Window {
    #[doc(alias = "Ogre::EAGL2Window::~EAGL2Window()")]
    #[doc = "Ogre::EAGL2Window::~EAGL2Window()"]
    pub fn destroy_d1(&self) {
        // `destroy` then `RenderTarget::~RenderTarget` (IDA 0xe885b8..0xe88676).
        self.destroy();
    }
}

// 0xe88680 — Ogre::EAGL2Window::destroy(void)
// IDA 0xe88680
impl Eagl2Window {
    #[doc(alias = "Ogre::EAGL2Window::destroy(void)")]
    #[doc = "Ogre::EAGL2Window::destroy(void)"]
    pub fn destroy(&self) {
        // `removeRenderWindow` plus three ObjC `release`s (IDA 0xe88680..0xe886f6).
        *self.context.lock() = None;
        self.closed.store(true, std::sync::atomic::Ordering::SeqCst);
        self.releases.fetch_add(3, std::sync::atomic::Ordering::SeqCst);
    }
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

// 0xe88700 — Ogre::EAGL2Window::resize(unsigned int,unsigned int)
// IDA 0xe88700
impl Eagl2Window {
    #[doc(alias = "Ogre::EAGL2Window::resize(unsigned int,unsigned int)")]
    #[doc = "Ogre::EAGL2Window::resize(unsigned int,unsigned int)"]
    pub fn resize(&self, width: u32, height: u32) {
        // Orientation check, framebuffer destroy/create, viewport dimension
        // refresh over all viewports (IDA 0xe88700..0xe887fe).
        self.width.store(width, std::sync::atomic::Ordering::SeqCst);
        self.height.store(height, std::sync::atomic::Ordering::SeqCst);
        self.framebuffer_rebuilds.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.viewport_updates.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0xe88800 — Ogre::EAGL2Window::windowMovedOrResized(void)
// IDA 0xe88800
impl Eagl2Window {
    #[doc(alias = "Ogre::EAGL2Window::windowMovedOrResized(void)")]
    #[doc = "Ogre::EAGL2Window::windowMovedOrResized(void)"]
    pub fn window_moved_or_resized(&self) {
        // Refreshes every viewport from the view frame (IDA 0xe88800..0xe88894).
        self.viewport_updates.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0xe88894 — Ogre::EAGL2Window::_beginUpdate(void)
// IDA 0xe88894
impl Eagl2Window {
    #[doc(alias = "Ogre::EAGL2Window::_beginUpdate(void)")]
    #[doc = "Ogre::EAGL2Window::_beginUpdate(void)"]
    pub fn begin_update(&self) {
        // `RenderTarget::_beginUpdate` then `glBindFramebuffer`
        // (IDA 0xe88894..0xe888bc); GL is out of slice.
        self.begin_updates.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0xe888bc — Ogre::EAGL2Window::initNativeCreatedWindow(params)
// IDA 0xe888bc
impl Eagl2Window {
    #[doc(alias = "Ogre::EAGL2Window::initNativeCreatedWindow(params)")]
    #[doc = "Ogre::EAGL2Window::initNativeCreatedWindow(params)"]
    pub fn init_native_created_window(&self, video_mode: &str) {
        // Parses `Video Mode` and friends out of the misc-params map
        // (IDA 0xe888bc..0xe892f8); string conversion is the observable half.
        *self.video_mode.lock() = video_mode.to_owned();
    }
}

// 0xe89488 — Ogre::EAGL2Window::create(name,width,height,fullscreen,params)
// IDA 0xe89488
impl Eagl2Window {
    #[doc(alias = "Ogre::EAGL2Window::create(name,width,height,fullscreen,params)")]
    #[doc = "Ogre::EAGL2Window::create(name,width,height,fullscreen,params)"]
    pub fn create(&self, name: &str, width: u32, height: u32, fullscreen: bool) {
        // Parses FSAA/content-scaling/misc params, creates the view
        // (IDA 0xe89488..0xe89c78); `name` keys the view layer.
        let _ = name;
        self.width.store(width, std::sync::atomic::Ordering::SeqCst);
        self.height.store(height, std::sync::atomic::Ordering::SeqCst);
        self.fullscreen.store(fullscreen, std::sync::atomic::Ordering::SeqCst);
        self.closed.store(false, std::sync::atomic::Ordering::SeqCst);
        self.visible.store(true, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0xe89c80 — Ogre::EAGL2Window::swapBuffers(bool)
// IDA 0xe89c80
impl Eagl2Window {
    #[doc(alias = "Ogre::EAGL2Window::swapBuffers(bool)")]
    #[doc = "Ogre::EAGL2Window::swapBuffers(bool)"]
    pub fn swap_buffers(&self, vsync: bool) -> bool {
        // Multisample resolve, `presentRenderbuffer:`; failure logs
        // `Failed to swap buffers in ...` (IDA 0xe89c80..0xe89ec0).
        let _ = vsync;
        self.buffers_swapped.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        true
    }
}

// 0xe89f88 — Ogre::EAGL2Window::getCustomAttribute(std::string const&,void *)
// IDA 0xe89f88
impl Eagl2Window {
    #[doc(alias = "Ogre::EAGL2Window::getCustomAttribute(std::string const&,void *)")]
    #[doc = "Ogre::EAGL2Window::getCustomAttribute(std::string const&,void *)"]
    pub fn custom_attribute(&self, name: &str) -> Option<ObjCId> {
        // `GLCONTEXT` yields the context, `SHAREGROUP` the support
        // (IDA 0xe89f88..0xe8a036).
        match name {
            "GLCONTEXT" => *self.context.lock(),
            "SHAREGROUP" => *self.support.lock(),
            _ => None,
        }
    }
}

// 0xe8a038 — Ogre::EAGL2Window::copyContentsToMemory(pixelbox,buffer)
// IDA 0xe8a038
impl Eagl2Window {
    #[doc(alias = "Ogre::EAGL2Window::copyContentsToMemory(pixelbox,buffer)")]
    #[doc = "Ogre::EAGL2Window::copyContentsToMemory(pixelbox,buffer)"]
    pub fn copy_contents_to_memory(&self, valid_box: bool, width: u32, height: u32) -> Result<(), String> {
        // A bad box throws `Ogre::Exception("Invalid box.")`
        // (IDA 0xe8a038..0xe8a52c).
        if !valid_box {
            return Err("Invalid box.".to_owned());
        }
        let _ = (width, height);
        Ok(())
    }
}

// 0xe8a554 — Ogre::EAGL2Window::requiresTextureFlipping(void)const
// IDA 0xe8a554
impl Eagl2Window {
    #[doc(alias = "Ogre::EAGL2Window::requiresTextureFlipping(void)const")]
    #[doc = "Ogre::EAGL2Window::requiresTextureFlipping(void)const"]
    pub fn requires_texture_flipping(&self) -> bool {
        false // IDA 0xe8a554..0xe8a558: `return 0`
    }
}

// 0xe8a568 — Ogre::EAGL2Window::isVisible(void)const
// IDA 0xe8a568
impl Eagl2Window {
    #[doc(alias = "Ogre::EAGL2Window::isVisible(void)const")]
    #[doc = "Ogre::EAGL2Window::isVisible(void)const"]
    pub fn is_visible(&self) -> bool {
        // Byte `+149` (IDA 0xe8a568..0xe8a56e).
        self.visible.load(std::sync::atomic::Ordering::SeqCst)
    }
}

// 0xe8a570 — Ogre::EAGL2Window::setVisible(bool)
// IDA 0xe8a570
impl Eagl2Window {
    #[doc(alias = "Ogre::EAGL2Window::setVisible(bool)")]
    #[doc = "Ogre::EAGL2Window::setVisible(bool)"]
    pub fn set_visible(&self, visible: bool) {
        // Byte `+149` store (IDA 0xe8a570..0xe8a576).
        self.visible.store(visible, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0xe8a590 — Ogre::EAGL2Window::isClosed(void)const
// IDA 0xe8a590
impl Eagl2Window {
    #[doc(alias = "Ogre::EAGL2Window::isClosed(void)const")]
    #[doc = "Ogre::EAGL2Window::isClosed(void)const"]
    pub fn is_closed(&self) -> bool {
        // Byte `+148` (IDA 0xe8a590..0xe8a596).
        self.closed.load(std::sync::atomic::Ordering::SeqCst)
    }
}
