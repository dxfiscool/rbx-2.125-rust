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
}

impl PlaceLauncher {
    pub fn shared_instance() -> &'static Self {
        static LAUNCHER: std::sync::LazyLock<PlaceLauncher> = std::sync::LazyLock::new(|| PlaceLauncher {
            view_enabled: std::sync::atomic::AtomicBool::new(true),
        });
        &LAUNCHER
    }
    pub fn disable_view_because_going_to_background(&self) {
        self.view_enabled.store(false, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn is_view_enabled(&self) -> bool {
        self.view_enabled.load(std::sync::atomic::Ordering::SeqCst)
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

// 0x19228 — -[AppDelegate init]
// type: AppDelegate *__cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate init]")]
pub fn stub_19228() -> ! {
    todo!("0x19228 -[AppDelegate init]")
}

// 0x19254 — -[AppDelegate dealloc]
// type: void __cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate dealloc]")]
pub fn stub_19254() -> ! {
    todo!("0x19254 -[AppDelegate dealloc]")
}

// 0x192b4 — -[AppDelegate application:didFinishLaunchingWithOptions:]
// type: char __cdecl(AppDelegate *self, SEL, id, id)
#[doc(alias = "-[AppDelegate application:didFinishLaunchingWithOptions:]")]
pub fn stub_192b4() -> ! {
    todo!("0x192b4 -[AppDelegate application:didFinishLaunchingWithOptions:]")
}

// 0x194ec — ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke")]
pub fn stub_194ec() -> ! {
    todo!("0x194ec ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke")
}

// 0x19514 — ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2
// type: void __cdecl(id)
#[doc(alias = "___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2")]
pub fn stub_19514() -> ! {
    todo!("0x19514 ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2")
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
#[doc(alias = "-[AppDelegate applicationDidEnterBackground:]")]
pub fn stub_196e4() -> ! {
    todo!("0x196e4 -[AppDelegate applicationDidEnterBackground:]")
}

// 0x19a30 — -[AppDelegate applicationDidReceiveMemoryWarning:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationDidReceiveMemoryWarning:]")]
pub fn stub_19a30() -> ! {
    todo!("0x19a30 -[AppDelegate applicationDidReceiveMemoryWarning:]")
}

// 0x19b60 — -[AppDelegate applicationWillEnterForeground:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationWillEnterForeground:]")]
pub fn stub_19b60() -> ! {
    todo!("0x19b60 -[AppDelegate applicationWillEnterForeground:]")
}

// 0x19cdc — -[AppDelegate applicationDidBecomeActive:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationDidBecomeActive:]")]
pub fn stub_19cdc() -> ! {
    todo!("0x19cdc -[AppDelegate applicationDidBecomeActive:]")
}

// 0x19f34 — ___42-[AppDelegate applicationDidBecomeActive:]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___42-[AppDelegate applicationDidBecomeActive:]_block_invoke")]
pub fn stub_19f34() -> ! {
    todo!("0x19f34 ___42-[AppDelegate applicationDidBecomeActive:]_block_invoke")
}

// 0x19f7c — -[AppDelegate applicationWillTerminate:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationWillTerminate:]")]
pub fn stub_19f7c() -> ! {
    todo!("0x19f7c -[AppDelegate applicationWillTerminate:]")
}

// 0x1a098 — __Z18_topMostControllerP16UIViewController
// type: id __fastcall(id)
#[doc(alias = "_topMostController(UIViewController *)")]
pub fn stub_1a098() -> ! {
    todo!("0x1a098 _topMostController(UIViewController *)")
}

// 0x1a174 — -[AppDelegate application:openURL:sourceApplication:annotation:]
// type: char __cdecl(AppDelegate *self, SEL, id, id, id, id)
#[doc(alias = "-[AppDelegate application:openURL:sourceApplication:annotation:]")]
pub fn stub_1a174() -> ! {
    todo!("0x1a174 -[AppDelegate application:openURL:sourceApplication:annotation:]")
}

// 0x1a234 — -[AppDelegate TryLaunchPlace:]
// type: void __cdecl(AppDelegate *self, SEL, int)
#[doc(alias = "-[AppDelegate TryLaunchPlace:]")]
pub fn stub_1a234() -> ! {
    todo!("0x1a234 -[AppDelegate TryLaunchPlace:]")
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
#[doc(alias = "Ogre::EAGL2Support::validateConfig(void)")]
pub fn stub_e862b0() -> ! {
    todo!("0xe862b0 Ogre::EAGL2Support::validateConfig(void)")
}

// 0xe862c8 — __ZN4Ogre12EAGL2Support14getDisplayNameEv
// type: _DWORD __fastcall(Ogre::EAGL2Support *__hidden this)
#[doc(alias = "Ogre::EAGL2Support::getDisplayName(void)")]
pub fn stub_e862c8() -> ! {
    todo!("0xe862c8 Ogre::EAGL2Support::getDisplayName(void)")
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
#[doc(alias = "Ogre::EAGL2Support::start(void)")]
pub fn stub_e86d84() -> ! {
    todo!("0xe86d84 Ogre::EAGL2Support::start(void)")
}

// 0xe86d88 — __ZN4Ogre12EAGL2Support4stopEv
// type: _DWORD __fastcall(Ogre::EAGL2Support *__hidden this)
#[doc(alias = "Ogre::EAGL2Support::stop(void)")]
pub fn stub_e86d88() -> ! {
    todo!("0xe86d88 Ogre::EAGL2Support::stop(void)")
}

// 0xe87e38 — -[EAGL2View description]
// type: id __cdecl(EAGL2View *self, SEL)
#[doc(alias = "-[EAGL2View description]")]
pub fn stub_e87e38() -> ! {
    todo!("0xe87e38 -[EAGL2View description]")
}

// 0xe87f28 — +[EAGL2View layerClass]
// type: Class __cdecl(id, SEL)
#[doc(alias = "+[EAGL2View layerClass]")]
pub fn stub_e87f28() -> ! {
    todo!("0xe87f28 +[EAGL2View layerClass]")
}

// 0xe87f4c — -[EAGL2View layoutSubviews]
// type: void __cdecl(EAGL2View *self, SEL)
#[doc(alias = "-[EAGL2View layoutSubviews]")]
pub fn stub_e87f4c() -> ! {
    todo!("0xe87f4c -[EAGL2View layoutSubviews]")
}

// 0xe880b4 — -[EAGL2View mWindowName]
// type: basic_string<char, std::char_traits<char>, std::allocator<char> > __cdecl(EAGL2View *self, SEL)
#[doc(alias = "-[EAGL2View mWindowName]")]
pub fn stub_e880b4() -> ! {
    todo!("0xe880b4 -[EAGL2View mWindowName]")
}

// 0xe880cc — -[EAGL2View setMWindowName:]
// type: void __cdecl(EAGL2View *self, SEL, basic_string<char, std::char_traits<char>, std::allocator<char> >)
#[doc(alias = "-[EAGL2View setMWindowName:]")]
pub fn stub_e880cc() -> ! {
    todo!("0xe880cc -[EAGL2View setMWindowName:]")
}

// 0xe880e8 — -[EAGL2View .cxx_destruct]
// type: void __cdecl(EAGL2View *self, SEL)
#[doc(alias = "-[EAGL2View .cxx_destruct]")]
pub fn stub_e880e8() -> ! {
    todo!("0xe880e8 -[EAGL2View .cxx_destruct]")
}

// 0xe88140 — -[EAGL2View .cxx_construct]
// type: id __cdecl(EAGL2View *self, SEL)
#[doc(alias = "-[EAGL2View .cxx_construct]")]
pub fn stub_e88140() -> ! {
    todo!("0xe88140 -[EAGL2View .cxx_construct]")
}

// 0xe88194 — -[EAGL2ViewController init]
// type: EAGL2ViewController *__cdecl(EAGL2ViewController *self, SEL)
#[doc(alias = "-[EAGL2ViewController init]")]
pub fn stub_e88194() -> ! {
    todo!("0xe88194 -[EAGL2ViewController init]")
}

// 0xe881c0 — -[EAGL2ViewController initWithNibName:bundle:]
// type: EAGL2ViewController *__cdecl(EAGL2ViewController *self, SEL, id, id)
#[doc(alias = "-[EAGL2ViewController initWithNibName:bundle:]")]
pub fn stub_e881c0() -> ! {
    todo!("0xe881c0 -[EAGL2ViewController initWithNibName:bundle:]")
}

// 0xe881f0 — -[EAGL2ViewController dealloc]
// type: void __cdecl(EAGL2ViewController *self, SEL)
#[doc(alias = "-[EAGL2ViewController dealloc]")]
pub fn stub_e881f0() -> ! {
    todo!("0xe881f0 -[EAGL2ViewController dealloc]")
}

// 0xe8821c — -[EAGL2ViewController didReceiveMemoryWarning]
// type: void __cdecl(EAGL2ViewController *self, SEL)
#[doc(alias = "-[EAGL2ViewController didReceiveMemoryWarning]")]
pub fn stub_e8821c() -> ! {
    todo!("0xe8821c -[EAGL2ViewController didReceiveMemoryWarning]")
}

// 0xe88248 — -[EAGL2ViewController loadView]
// type: void __cdecl(EAGL2ViewController *self, SEL)
#[doc(alias = "-[EAGL2ViewController loadView]")]
pub fn stub_e88248() -> ! {
    todo!("0xe88248 -[EAGL2ViewController loadView]")
}

// 0xe88274 — -[EAGL2ViewController viewDidLoad]
// type: void __cdecl(EAGL2ViewController *self, SEL)
#[doc(alias = "-[EAGL2ViewController viewDidLoad]")]
pub fn stub_e88274() -> ! {
    todo!("0xe88274 -[EAGL2ViewController viewDidLoad]")
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
#[doc(alias = "-[EAGL2ViewController supportedInterfaceOrientations]")]
pub fn stub_e88310() -> ! {
    todo!("0xe88310 -[EAGL2ViewController supportedInterfaceOrientations]")
}

// 0xe88314 — -[EAGL2ViewController shouldAutorotateToInterfaceOrientation:]
// type: char __cdecl(EAGL2ViewController *self, SEL, int)
#[doc(alias = "-[EAGL2ViewController shouldAutorotateToInterfaceOrientation:]")]
pub fn stub_e88314() -> ! {
    todo!("0xe88314 -[EAGL2ViewController shouldAutorotateToInterfaceOrientation:]")
}

// 0xe88328 — -[EAGL2ViewController mGLSupport]
// type: EAGL2Support *__cdecl(EAGL2ViewController *self, SEL)
#[doc(alias = "-[EAGL2ViewController mGLSupport]")]
pub fn stub_e88328() -> ! {
    todo!("0xe88328 -[EAGL2ViewController mGLSupport]")
}

// 0xe8833c — -[EAGL2ViewController setMGLSupport:]
// type: void __cdecl(EAGL2ViewController *self, SEL, EAGL2Support *)
#[doc(alias = "-[EAGL2ViewController setMGLSupport:]")]
pub fn stub_e8833c() -> ! {
    todo!("0xe8833c -[EAGL2ViewController setMGLSupport:]")
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
#[doc(alias = "Ogre::EAGL2Window::setFullscreen(bool,unsigned int,unsigned int)")]
pub fn stub_e886f8() -> ! {
    todo!("0xe886f8 Ogre::EAGL2Window::setFullscreen(bool,unsigned int,unsigned int)")
}

// 0xe886fc — __ZN4Ogre11EAGL2Window10repositionEii
// type: _DWORD __fastcall(Ogre::EAGL2Window *__hidden this, int, int)
#[doc(alias = "Ogre::EAGL2Window::reposition(int,int)")]
pub fn stub_e886fc() -> ! {
    todo!("0xe886fc Ogre::EAGL2Window::reposition(int,int)")
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
