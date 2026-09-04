//! audio generated_138 — next 100 stubs EA-sorted, from ida/export.json
//! Filter: FMOD|Soundscape exhausted (2398 distinct) — filler EA-sorted asc, skip existing, rbx_core::SharedPtr not boost
//! Batch: 100 stubs | skeleton batch | range 0x18ca0..0x1cacc EA-sorted asc filler after 0x18c98, skip existing, rbx_core::SharedPtr not boost
//! Generated: 2026-09-01

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use std::collections::HashMap;
use crate::generated_137::AudioAppirater;

// Ensure SharedPtr is seen as used — mirrors boost::shared_ptr<T> -> rbx_core::SharedPtr<T>
const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x18ca0 — +[Appirater appLaunched]
#[doc(alias = "+[Appirater appLaunched]")]
pub fn stub_18ca0(current_version: &str, now_secs: f64) {
    // IDA 0x18ca0 (`+[Appirater appLaunched]`): forwards `YES` to
    // `appLaunched:` (`stub_18cc0`). Same as the platform 0x18ca0 anchor.
    stub_18cc0(true, current_version, now_secs);
}

// 0x18cc0 — +[Appirater appLaunched:]
#[doc(alias = "+[Appirater appLaunched:]")]
pub fn stub_18cc0(first_launch: bool, current_version: &str, now_secs: f64) {
    // IDA 0x18cc0 (`+[Appirater appLaunched:]`): captures the flag into the
    // stack block and `dispatch_async`es it to a global queue. The queue
    // hop collapses; the block is `stub_18d10`. Same as the platform 0x18cc0
    // anchor.
    stub_18d10(first_launch, current_version, now_secs);
}

// 0x18d10 — ___25+[Appirater appLaunched:]_block_invoke
#[doc(alias = "___25+[Appirater appLaunched:]_block_invoke")]
pub fn stub_18d10(can_rate: bool, current_version: &str, now_secs: f64) {
    // IDA 0x18d10 (`__25+[Appirater appLaunched:]_block_invoke`):
    // `sharedInstance` then `incrementAndRate:` with the captured flag.
    // Same as the platform 0x18d10 anchor.
    AudioAppirater::shared_note_app_launched();
    crate::generated_137::stub_18b18(can_rate, current_version, now_secs);
}

// 0x18d4c — -[Appirater hideRatingAlert]
#[doc(alias = "-[Appirater hideRatingAlert]")]
pub fn stub_18d4c() -> bool {
    // IDA 0x18d4c (`-[Appirater hideRatingAlert]`): dismisses `ratingAlert`
    // when visible; the `_debug` `NSLog` has no host sink. Reports whether
    // an alert was dismissed. Same as the platform 0x18d4c anchor.
    AudioAppirater::shared_hide_rating_alert()
}

// 0x18dbc — +[Appirater appWillResignActive]
#[doc(alias = "+[Appirater appWillResignActive]")]
pub fn stub_18dbc() {
    // IDA 0x18dbc (`+[Appirater appWillResignActive]`): `_debug` `NSLog`
    // (no host sink), then `hideRatingAlert` on `sharedInstance`. Same as
    // the platform 0x18dbc anchor.
    AudioAppirater::shared_hide_rating_alert();
}

// 0x18e0c — +[Appirater appEnteredForeground:]
#[doc(alias = "+[Appirater appEnteredForeground:]")]
pub fn stub_18e0c(entered: bool, current_version: &str, now_secs: f64) {
    // IDA 0x18e0c (`+[Appirater appEnteredForeground:]`): same shape as
    // 0x18cc0 — capture the flag, `dispatch_async` to a global queue; the
    // block is `stub_18e5c`. Same as the platform 0x18e0c anchor.
    stub_18e5c(entered, current_version, now_secs);
}

// 0x18e5c — ___34+[Appirater appEnteredForeground:]_block_invoke
#[doc(alias = "___34+[Appirater appEnteredForeground:]_block_invoke")]
pub fn stub_18e5c(can_rate: bool, current_version: &str, now_secs: f64) {
    // IDA 0x18e5c (`__34+[Appirater appEnteredForeground:]_block_invoke`):
    // `sharedInstance` then `incrementAndRate:`. Same as the platform
    // 0x18e5c anchor.
    AudioAppirater::shared_note_entered_foreground();
    crate::generated_137::stub_18b18(can_rate, current_version, now_secs);
}

// 0x18e98 — +[Appirater userDidSignificantEvent:]
#[doc(alias = "+[Appirater userDidSignificantEvent:]")]
pub fn stub_18e98(significant: bool, current_version: &str, now_secs: f64) {
    // IDA 0x18e98 (`+[Appirater userDidSignificantEvent:]`): same dispatch
    // shape over `incrementSignificantEventAndRate:`; the block is
    // `stub_18ee8`. Same as the platform 0x18e98 anchor.
    stub_18ee8(significant, current_version, now_secs);
}

// 0x18ee8 — ___37+[Appirater userDidSignificantEvent:]_block_invoke
#[doc(alias = "___37+[Appirater userDidSignificantEvent:]_block_invoke")]
pub fn stub_18ee8(can_rate: bool, current_version: &str, now_secs: f64) {
    // IDA 0x18ee8 (`__37+[Appirater userDidSignificantEvent:]_block_invoke`):
    // `sharedInstance` then `incrementSignificantEventAndRate:`. Same as the
    // platform 0x18ee8 anchor.
    crate::generated_137::stub_18bdc(can_rate, current_version, now_secs);
}

// 0x18f24 — +[Appirater rateApp]
#[doc(alias = "+[Appirater rateApp]")]
pub fn stub_18f24() -> String {
    // IDA 0x18f24 (`+[Appirater rateApp]`): review URL from the template
    // with `APP_ID` replaced, flag `kAppiraterRatedCurrentVersion`,
    // `openURL:`. Returns the opened URL. Same as the platform 0x18f24
    // anchor.
    AudioAppirater::shared_rate_app()
}

// 0x19028 — -[Appirater alertView:clickedButtonAtIndex:]
#[doc(alias = "-[Appirater alertView:clickedButtonAtIndex:]")]
pub fn stub_19028(button_index: i32, now_secs: f64) {
    // IDA 0x19028 (`-[Appirater alertView:clickedButtonAtIndex:]`): the
    // three-way button switch with delegate callbacks; see
    // `AudioAppirater::alert_view_clicked_button`. Same as the platform
    // 0x19028 anchor.
    AudioAppirater::shared_alert_button(button_index, now_secs);
}

// 0x191d4 — -[Appirater ratingAlert]
#[doc(alias = "-[Appirater ratingAlert]")]
pub fn stub_191d4() -> u64 {
    // IDA 0x191d4 (`-[Appirater ratingAlert]`): returns the `ratingAlert`
    // ivar. Same as the platform 0x191d4 anchor; `0` is `nil`.
    AudioAppirater::shared_rating_alert()
}

// 0x191e4 — -[Appirater setRatingAlert:]
#[doc(alias = "-[Appirater setRatingAlert:]")]
pub fn stub_191e4(alert: u64) {
    // IDA 0x191e4 (`-[Appirater setRatingAlert:]`): retained-property store
    // via `objc_setProperty`. Same as the platform 0x191e4 anchor.
    AudioAppirater::shared_set_rating_alert(alert);
}

// 0x19208 — -[Appirater delegate]
#[doc(alias = "-[Appirater delegate]")]
pub fn stub_19208() -> u64 {
    // IDA 0x19208 (`-[Appirater delegate]`): returns the `_delegate` ivar.
    // Same as the platform 0x19208 anchor; `0` is `nil`.
    AudioAppirater::shared_delegate()
}

// 0x19218 — -[Appirater setDelegate:]
#[doc(alias = "-[Appirater setDelegate:]")]
pub fn stub_19218(delegate: u64) {
    // IDA 0x19218 (`-[Appirater setDelegate:]`): plain ivar store. Same as
    // the platform 0x19218 anchor.
    AudioAppirater::shared_set_delegate(delegate);
}

/// Audio-crate host for the `AppDelegate` ObjC cluster (IDA 0x19228..0x19f7c):
/// lifecycle counters, launch defaults, login persistence slots, session
/// reports, and the deep-link place id. `NSUserDefaults`/`UIAlertView`/
/// `PlaceLauncher` collaborators collapse to host state; `id`/`UIWindow`
/// tokens are `u64` (`0`/`None` is `nil`). Mirrors the platform crate
/// `AppDelegate` model (which owns the full machine); audio cannot depend
/// on platform (AGENTS.md DAG), so the slots these filler EAs touch live
/// here.
#[derive(Debug, Default)]
pub struct AudioAppDelegate {
    window: parking_lot::Mutex<Option<u64>>,
    bg_task: std::sync::atomic::AtomicU32,
    deallocated: std::sync::atomic::AtomicBool,
    launch_calls: std::sync::atomic::AtomicU32,
    resign_calls: std::sync::atomic::AtomicU32,
    background_calls: std::sync::atomic::AtomicU32,
    foreground_calls: std::sync::atomic::AtomicU32,
    active_calls: std::sync::atomic::AtomicU32,
    terminate_calls: std::sync::atomic::AtomicU32,
    mem_warning_calls: std::sync::atomic::AtomicU32,
    upgrade_checks: std::sync::atomic::AtomicU32,
    cookie_policy: std::sync::atomic::AtomicU32,
    defaults_str: parking_lot::Mutex<HashMap<String, String>>,
    defaults_bool: parking_lot::Mutex<HashMap<String, bool>>,
    sync_calls: std::sync::atomic::AtomicU32,
    session_reports: parking_lot::Mutex<Vec<u32>>,
    last_page_view: parking_lot::Mutex<String>,
    flurry_key: parking_lot::Mutex<Option<String>>,
    login_username: parking_lot::Mutex<String>,
    login_password: parking_lot::Mutex<String>,
    pending_place_id: std::sync::atomic::AtomicU32,
    launched_place_ids: parking_lot::Mutex<Vec<u32>>,
    fetch_settings_calls: std::sync::atomic::AtomicU32,
    last_fetch_settings: parking_lot::Mutex<(String, String)>,
    memory_bouncer_running: std::sync::atomic::AtomicBool,
    place_launcher_mem_warnings: std::sync::atomic::AtomicU32,
    place_launcher_view_disables: std::sync::atomic::AtomicU32,
    place_launcher_view_enables: std::sync::atomic::AtomicU32,
    place_launcher_leaves: std::sync::atomic::AtomicU32,
    connection_weak_slot: parking_lot::Mutex<u64>,
    message_connection_alive: std::sync::atomic::AtomicBool,
    login_place_id: parking_lot::Mutex<Option<i32>>,
    jump_to_place_id: parking_lot::Mutex<Option<i32>>,
    web_touch_ups: std::sync::atomic::AtomicU32,
    started_games: parking_lot::Mutex<Vec<(i32, bool)>>,
    open_url_calls: std::sync::atomic::AtomicU32,
}

/// Host view-controller presentation graph for `_topMostController` (IDA
/// 0x1a098) / `topMostController` (IDA 0x1a124): presented links, navigation
/// flags, and navigation visible controllers. `id` tokens are `u64` (`0` is
/// `nil`). Mirrors the platform crate `ViewControllerGraph`.
#[derive(Debug, Default)]
pub struct AudioViewControllerGraph {
    presented: parking_lot::Mutex<HashMap<u64, u64>>,
    is_navigation: parking_lot::Mutex<HashMap<u64, bool>>,
    visible: parking_lot::Mutex<HashMap<u64, u64>>,
}

impl AudioViewControllerGraph {
    pub fn presented_view_controller(&self, id: u64) -> Option<u64> {
        self.presented.lock().get(&id).copied()
    }
    pub fn is_navigation_controller(&self, id: u64) -> bool {
        self.is_navigation.lock().get(&id).copied().unwrap_or(false)
    }
    pub fn visible_view_controller(&self, id: u64) -> Option<u64> {
        self.visible.lock().get(&id).copied()
    }
    pub fn set_presented_view_controller(&self, id: u64, presented: u64) {
        self.presented.lock().insert(id, presented);
    }
    pub fn set_visible_view_controller(&self, nav: u64, visible: u64) {
        self.is_navigation.lock().insert(nav, true);
        self.visible.lock().insert(nav, visible);
    }
}

/// `_topMostController` (IDA 0x1a098): walk `presentedViewController` to the
/// chain end, resolve a navigation controller to its visible controller,
/// `nil` when nothing sits above the root. Mirrors the platform twin.
pub fn audio_top_most_controller(graph: &AudioViewControllerGraph, root: u64) -> Option<u64> {
    let mut top = root;
    if graph.presented_view_controller(top).is_some() {
        loop {
            top = graph.presented_view_controller(top).unwrap_or(top);
            if graph.presented_view_controller(top).is_none() {
                break;
            }
        }
    }
    if graph.is_navigation_controller(top) {
        if let Some(visible) = graph.visible_view_controller(top) {
            top = visible;
        }
    }
    if top == root {
        return None;
    }
    Some(top)
}

/// `-[AppDelegate TryLaunchPlace:]` dispatch outcome (IDA 0x1a334..0x1a488).
/// Mirrors the platform crate `LaunchAction`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioLaunchAction {
    LoginPlaceIdSet,
    HomeJumpTriggered,
    GameStarted,
    GameInProgressJumpSet,
    Unknown,
}

/// Global deep-link place id stashed by `application:openURL:...` (IDA
/// 0x1a22e `appPlaceID`), consumed by `applicationDidBecomeActive`.
/// Mirrors the platform crate `APP_PLACE_ID`.
pub static AUDIO_APP_PLACE_ID: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);

/// `_main` invocation record (IDA 0x1a768).
pub static AUDIO_MAIN_CALLS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);

impl AudioAppDelegate {
    /// `-[AppDelegate init]` (IDA 0x19228): only `objc_msgSendSuper2`
    /// init; no ivar stores. Host `Default` covers it.
    pub fn init() -> Self {
        Self::default()
    }

    /// `-[AppDelegate dealloc]` (IDA 0x19254): `+[RobloxGoogleAnalytics
    /// release]` (no retained host object), `-[UIWindow release]`, then
    /// super dealloc (runs as drop).
    pub fn dealloc(self) {
        *self.window.lock() = None;
        self.deallocated
            .store(true, std::sync::atomic::Ordering::SeqCst);
    }

    fn register_bool_default(&self, key: &str, value: bool) {
        self.defaults_bool.lock().insert(key.to_owned(), value);
    }
    fn set_object(&self, value: &str, key: &str) {
        self.defaults_str
            .lock()
            .insert(key.to_owned(), value.to_owned());
    }
    fn object_for_key(&self, key: &str) -> String {
        self.defaults_str.lock().get(key).cloned().unwrap_or_default()
    }
    fn remove_object_for_key(&self, key: &str) {
        self.defaults_str.lock().remove(key);
    }
    fn synchronize(&self) {
        self.sync_calls
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    fn report_session_for(&self, id: u32) {
        self.session_reports.lock().push(id);
    }
    fn check_for_update(&self) {
        self.upgrade_checks
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }

    /// `-[AppDelegate application:didFinishLaunchingWithOptions:]` (IDA
    /// 0x192b4): defaults registration, reporter/GA counters, the two
    /// global-queue blocks (run inline in order), upgrade check, cookie
    /// policy, login restore. Returns 1. Mirrors the platform twin.
    pub fn application_did_finish_launching(&self) -> bool {
        self.register_bool_default("warnings_preference", true);
        self.register_bool_default("wifionly_preference", false);
        self.report_session_for(7);
        stub_194ec(self);
        stub_19514();
        self.check_for_update();
        self.cookie_policy
            .store(0, std::sync::atomic::Ordering::SeqCst);
        // Restore the persisted login (missing key reads as empty, like
        // nil).
        *self.login_username.lock() = self.object_for_key("username");
        *self.login_password.lock() = self.object_for_key("password");
        self.launch_calls
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        true
    }

    /// `-[AppDelegate applicationWillResignActive:]` (IDA 0x195a0):
    /// begin/end trace + `disableViewBecauseGoingToBackground`.
    pub fn application_will_resign_active(&self) {
        eprintln!("AppDelegate applicationWillResignActive begin");
        self.place_launcher_view_disables
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.resign_calls
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        eprintln!("AppDelegate applicationWillResignActive end");
    }

    /// `-[AppDelegate applicationDidEnterBackground:]` (IDA 0x196e4):
    /// state persist, login persist, session report, page view — plus the
    /// preserved BUG (removes the state key just written, then syncs).
    /// Mirrors the platform twin.
    pub fn application_did_enter_background(&self) {
        eprintln!("AppDelegate applicationDidEnterBackground begin");
        self.set_object("tryBackground", "RobloxAppState");
        self.synchronize();
        self.place_launcher_leaves
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.remove_object_for_key("signupusername");
        self.remove_object_for_key("signupbirthdate");
        self.remove_object_for_key("signupgender");
        let username = self.login_username.lock().clone();
        self.set_object(&username, "username");
        let password = self.login_password.lock().clone();
        self.set_object(&password, "password");
        self.report_session_for(1);
        *self.last_page_view.lock() = "RobloxApp/EnterBackGround".to_owned();
        eprintln!("AppDelegate applicationDidEnterBackground end");
        // BUG preserved: the original removes the state key it just wrote,
        // then syncs (IDA 0x19992..0x199b6).
        self.remove_object_for_key("RobloxAppState");
        self.synchronize();
        self.background_calls
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }

    /// `-[AppDelegate applicationDidReceiveMemoryWarning:]` (IDA 0x19a30):
    /// trace + `stopMemoryBouncer:0`; when the bouncer reports nothing
    /// stopped, forwards to `PlaceLauncher`. Mirrors the platform twin.
    pub fn application_did_receive_memory_warning(&self) {
        eprintln!("Received out of memory warning (applicationDidReceiveMemoryWarning)");
        self.mem_warning_calls
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        if !self.stop_memory_bouncer() {
            self.place_launcher_mem_warnings
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
    }
    fn stop_memory_bouncer(&self) -> bool {
        self.memory_bouncer_running
            .swap(false, std::sync::atomic::Ordering::SeqCst)
    }

    /// `-[AppDelegate applicationWillEnterForeground:]` (IDA 0x19b60):
    /// begin/end trace + Appirater foreground + upgrade check + page view.
    /// Mirrors the platform twin.
    pub fn application_will_enter_foreground(&self) {
        use crate::generated_137::AudioAppirater;
        eprintln!("AppDelegate applicationWillEnterForeground begin");
        AudioAppirater::shared_note_entered_foreground();
        self.check_for_update();
        *self.last_page_view.lock() = "RobloxApp/EnterForeGround".to_owned();
        self.foreground_calls
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        eprintln!("AppDelegate applicationWillEnterForeground end");
    }

    /// `-[AppDelegate applicationDidBecomeActive:]` (IDA 0x19cdc): state
    /// persist, enable-view, session report, inline fetch-settings block,
    /// pending deep-link place launch. Mirrors the platform twin.
    pub fn application_did_become_active(&self) {
        eprintln!("AppDelegate applicationDidBecomeActive begin");
        self.set_object("tryForeground", "RobloxAppState");
        self.synchronize();
        self.place_launcher_view_enables
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.report_session_for(0);
        stub_19f34(self);
        let pending = self
            .pending_place_id
            .load(std::sync::atomic::Ordering::SeqCst);
        if pending != 0 {
            self.launched_place_ids.lock().push(pending);
            self.pending_place_id
                .store(0, std::sync::atomic::Ordering::SeqCst);
        }
        eprintln!("AppDelegate applicationDidBecomeActive end");
        self.set_object("inApp", "RobloxAppState");
        self.synchronize();
        self.active_calls
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }

    /// `-[AppDelegate applicationWillTerminate:]` (IDA 0x19f7c).
    pub fn application_will_terminate(&self) {
        self.terminate_calls
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }

    /// `-[AppDelegate application:openURL:sourceApplication:annotation:]`
    /// (IDA 0x1a174): logs the open, requires the `robloxmobile` prefix,
    /// stashes `appPlaceID = [host intValue]`, returns 1. Mirrors the
    /// platform twin.
    pub fn application_open_url(
        &self,
        url_absolute_string: &str,
        url_host: &str,
        url_path: &str,
        source_application: &str,
        annotation: &str,
    ) -> bool {
        eprintln!(
            "AppDelegate::openURL URL:\t{url_absolute_string}\nFrom source:\t{source_application}\nWith annotation:{annotation}"
        );
        if !url_absolute_string.starts_with("robloxmobile") {
            return false;
        }
        eprintln!("host {url_host}");
        eprintln!("path {url_path}");
        AUDIO_APP_PLACE_ID.store(
            url_host.parse::<u32>().unwrap_or(0),
            std::sync::atomic::Ordering::SeqCst,
        );
        self.open_url_calls
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        true
    }

    /// `-[AppDelegate TryLaunchPlace:]` (IDA 0x1a234): dispatch over the
    /// top controller class. `LoginViewController` records the place id;
    /// `HomeViewController` records the jump + web touch;
    /// `RobloxNavBarViewController` starts the game; `GameViewController`
    /// records the in-progress jump. Mirrors the platform twin (which
    /// drives the live singletons).
    pub fn try_launch_place(&self, place_id: i32, top_controller_class: &str) -> AudioLaunchAction {
        if top_controller_class == "LoginViewController" {
            *self.login_place_id.lock() = Some(place_id);
            AudioLaunchAction::LoginPlaceIdSet
        } else if top_controller_class == "HomeViewController" {
            *self.jump_to_place_id.lock() = Some(place_id);
            self.web_touch_ups
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            AudioLaunchAction::HomeJumpTriggered
        } else if top_controller_class == "RobloxNavBarViewController" {
            self.started_games.lock().push((place_id, true));
            AudioLaunchAction::GameStarted
        } else if top_controller_class == "GameViewController" {
            *self.jump_to_place_id.lock() = Some(place_id);
            AudioLaunchAction::GameInProgressJumpSet
        } else {
            AudioLaunchAction::Unknown
        }
    }

    /// `-[AppDelegate bgTask]` / `-[AppDelegate setBgTask:]` (IDA
    /// 0x1a494/0x1a4a8): atomic ivar with `DMB ISH` barriers (host
    /// `SeqCst`). Mirrors the platform twin.
    pub fn bg_task(&self) -> u32 {
        self.bg_task.load(std::sync::atomic::Ordering::SeqCst)
    }
    pub fn set_bg_task(&self, task: u32) {
        self.bg_task
            .store(task, std::sync::atomic::Ordering::SeqCst);
    }

    /// `-[AppDelegate window]` / `-[AppDelegate setWindow:]` (IDA
    /// 0x1a4c0/0x1a4d0, retained-property store). `None` is `nil`.
    pub fn window(&self) -> Option<u64> {
        *self.window.lock()
    }
    pub fn set_window(&self, window: Option<u64>) {
        *self.window.lock() = window;
    }

    /// `-[AppDelegate .cxx_construct]` (IDA 0x1a5bc): zeroes
    /// `messageOutConnection.con.weak_slot.p_`. Mirrors the platform twin.
    pub fn cxx_construct(&self) {
        *self.connection_weak_slot.lock() = 0;
        self.message_connection_alive
            .store(true, std::sync::atomic::Ordering::SeqCst);
    }

    /// `-[AppDelegate .cxx_destruct]` (IDA 0x1a4f4): `connection::disconnect`
    /// + weak-slot release. Mirrors the platform twin.
    pub fn cxx_destruct(&self) {
        self.message_connection_alive
            .store(false, std::sync::atomic::Ordering::SeqCst);
        *self.connection_weak_slot.lock() = 0;
    }
}

// 0x19228 — -[AppDelegate init]
#[doc(alias = "-[AppDelegate init]")]
pub fn stub_19228() -> AudioAppDelegate {
    // IDA 0x19228 (`-[AppDelegate init]`): only `objc_msgSendSuper2` init;
    // no ivar stores. Same as the platform 0x19228 anchor.
    AudioAppDelegate::init()
}

// 0x19254 — -[AppDelegate dealloc]
#[doc(alias = "-[AppDelegate dealloc]")]
pub fn stub_19254(delegate: AudioAppDelegate) {
    // IDA 0x19254 (`-[AppDelegate dealloc]`): analytics release, window
    // release, super dealloc (runs as drop). Same as the platform 0x19254
    // anchor.
    delegate.dealloc();
}

// 0x192b4 — -[AppDelegate application:didFinishLaunchingWithOptions:]
#[doc(alias = "-[AppDelegate application:didFinishLaunchingWithOptions:]")]
pub fn stub_192b4(delegate: &AudioAppDelegate) -> bool {
    // IDA 0x192b4 (`-[AppDelegate
    // application:didFinishLaunchingWithOptions:]`): defaults, reporters,
    // launch blocks, upgrade check, cookie policy, login restore, returns
    // 1. Same as the platform 0x192b4 anchor.
    delegate.application_did_finish_launching()
}

// 0x194ec — ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke
#[doc(alias = "___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke")]
pub fn stub_194ec(delegate: &AudioAppDelegate) {
    // IDA 0x194ec (launch block 1): `+[Flurry startSession:]` with the
    // session key. Same as the platform 0x194ec anchor.
    *delegate.flurry_key.lock() = Some("FM7DNRW56339NC22K8GR".to_owned());
}

// 0x19514 — ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2
#[doc(alias = "___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2")]
pub fn stub_19514() {
    // IDA 0x19514 (launch block 2): Appirater app id / days / uses / remind
    // config + `appLaunched`. Same as the platform 0x19514 anchor.
    crate::generated_137::stub_17df0("431946152");
    crate::generated_137::stub_17e00(3.0);
    crate::generated_137::stub_17e14(10);
    crate::generated_137::stub_17e34(10.0);
    crate::generated_137::AudioAppirater::shared_note_app_launched();
}

// 0x195a0 — -[AppDelegate applicationWillResignActive:]
#[doc(alias = "-[AppDelegate applicationWillResignActive:]")]
pub fn stub_195a0(delegate: &AudioAppDelegate) {
    // IDA 0x195a0 (`-[AppDelegate applicationWillResignActive:]`): trace +
    // `disableViewBecauseGoingToBackground`. Same as the platform 0x195a0
    // anchor.
    delegate.application_will_resign_active();
}

// 0x196e4 — -[AppDelegate applicationDidEnterBackground:]
#[doc(alias = "-[AppDelegate applicationDidEnterBackground:]")]
pub fn stub_196e4(delegate: &AudioAppDelegate) {
    // IDA 0x196e4 (`-[AppDelegate applicationDidEnterBackground:]`): state
    // + login persist, session report, page view, preserved state-key BUG.
    // Same as the platform 0x196e4 anchor.
    delegate.application_did_enter_background();
}

// 0x19a30 — -[AppDelegate applicationDidReceiveMemoryWarning:]
#[doc(alias = "-[AppDelegate applicationDidReceiveMemoryWarning:]")]
pub fn stub_19a30(delegate: &AudioAppDelegate) {
    // IDA 0x19a30 (`-[AppDelegate applicationDidReceiveMemoryWarning:]`):
    // trace + memory-bouncer stop with PlaceLauncher forward. Same as the
    // platform 0x19a30 anchor.
    delegate.application_did_receive_memory_warning();
}

// 0x19b60 — -[AppDelegate applicationWillEnterForeground:]
#[doc(alias = "-[AppDelegate applicationWillEnterForeground:]")]
pub fn stub_19b60(delegate: &AudioAppDelegate) {
    // IDA 0x19b60 (`-[AppDelegate applicationWillEnterForeground:]`):
    // trace + Appirater foreground + upgrade check + page view. Same as the
    // platform 0x19b60 anchor.
    delegate.application_will_enter_foreground();
}

// 0x19cdc — -[AppDelegate applicationDidBecomeActive:]
#[doc(alias = "-[AppDelegate applicationDidBecomeActive:]")]
pub fn stub_19cdc(delegate: &AudioAppDelegate) {
    // IDA 0x19cdc (`-[AppDelegate applicationDidBecomeActive:]`): state
    // persist, enable-view, session report, fetch-settings block,
    // deep-link launch. Same as the platform 0x19cdc anchor.
    delegate.application_did_become_active();
}

// 0x19f34 — ___42-[AppDelegate applicationDidBecomeActive:]_block_invoke
#[doc(alias = "___42-[AppDelegate applicationDidBecomeActive:]_block_invoke")]
pub fn stub_19f34(delegate: &AudioAppDelegate) {
    // IDA 0x19f34 (become-active block): client-settings init + fetch with
    // the iOS settings service. Same as the platform 0x19f34 anchor.
    delegate
        .fetch_settings_calls
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    *delegate.last_fetch_settings.lock() = (
        "iOSAppSettings".to_owned(),
        "D6925E56-BFB9-4908-AAA2-A5B1EC4B2D79".to_owned(),
    );
}

// 0x19f7c — -[AppDelegate applicationWillTerminate:]
#[doc(alias = "-[AppDelegate applicationWillTerminate:]")]
pub fn stub_19f7c(delegate: &AudioAppDelegate) {
    // IDA 0x19f7c (`-[AppDelegate applicationWillTerminate:]`). Same as the
    // platform 0x19f7c anchor.
    delegate.application_will_terminate();
}

// 0x1a098 — __Z18_topMostControllerP16UIViewController
#[doc(alias = "_topMostController(UIViewController *)")]
pub fn stub_1a098(graph: &AudioViewControllerGraph, root: u64) -> Option<u64> {
    // IDA 0x1a098: walk `presentedViewController` to the chain end, resolve
    // a navigation controller to its visible controller, nil when nothing
    // sits above the root. Same as the platform 0x1a098 anchor.
    audio_top_most_controller(graph, root)
}

// 0x1a124 — __Z17topMostControllerv
#[doc(alias = "topMostController(void)")]
pub fn stub_1a124(graph: &AudioViewControllerGraph, key_window_root: u64) -> u64 {
    // IDA 0x1a124: `sharedApplication` -> `keyWindow` ->
    // `rootViewController` (passed in on the host), then loop
    // `_topMostController` until nil and return the last controller. Same
    // as the platform 0x1a124 anchor.
    let mut top = key_window_root;
    while let Some(next) = audio_top_most_controller(graph, top) {
        top = next;
    }
    top
}

// 0x1a174 — -[AppDelegate application:openURL:sourceApplication:annotation:]
#[doc(alias = "-[AppDelegate application:openURL:sourceApplication:annotation:]")]
pub fn stub_1a174(
    delegate: &AudioAppDelegate,
    url_absolute_string: &str,
    url_host: &str,
    url_path: &str,
    source_application: &str,
    annotation: &str,
) -> bool {
    // IDA 0x1a174: logs the open, requires the `robloxmobile` prefix, logs
    // host/path, stashes `appPlaceID = [host intValue]`, returns 1. Same as
    // the platform 0x1a174 anchor.
    delegate.application_open_url(
        url_absolute_string,
        url_host,
        url_path,
        source_application,
        annotation,
    )
}

// 0x1a234 — -[AppDelegate TryLaunchPlace:]
#[doc(alias = "-[AppDelegate TryLaunchPlace:]")]
pub fn stub_1a234(
    delegate: &AudioAppDelegate,
    place_id: i32,
    top_controller_class: &str,
) -> AudioLaunchAction {
    // IDA 0x1a234: window/root + keyWindow trace feeds the
    // `topMostController` class read; dispatch over Login/Home/NavBar/Game
    // controllers lives in the model. Same as the platform 0x1a234 anchor.
    delegate.try_launch_place(place_id, top_controller_class)
}

// 0x1a494 — -[AppDelegate bgTask]
#[doc(alias = "-[AppDelegate bgTask]")]
pub fn stub_1a494(delegate: &AudioAppDelegate) -> u32 {
    // IDA 0x1a494: `LDR` the `bgTask` ivar + `DMB ISH`. Same as the
    // platform 0x1a494 anchor.
    delegate.bg_task()
}

// 0x1a4a8 — -[AppDelegate setBgTask:]
#[doc(alias = "-[AppDelegate setBgTask:]")]
pub fn stub_1a4a8(delegate: &AudioAppDelegate, task: u32) {
    // IDA 0x1a4a8: `DMB ISH`, store the `bgTask` ivar, `DMB ISH`. Same as
    // the platform 0x1a4a8 anchor.
    delegate.set_bg_task(task);
}

// 0x1a4c0 — -[AppDelegate window]
#[doc(alias = "-[AppDelegate window]")]
pub fn stub_1a4c0(delegate: &AudioAppDelegate) -> Option<u64> {
    // IDA 0x1a4c0: returns `self->_window`. Same as the platform 0x1a4c0
    // anchor; `None` is `nil`.
    delegate.window()
}

// 0x1a4d0 — -[AppDelegate setWindow:]
#[doc(alias = "-[AppDelegate setWindow:]")]
pub fn stub_1a4d0(delegate: &AudioAppDelegate, window: Option<u64>) {
    // IDA 0x1a4d0: retained-property store via `objc_setProperty`. Same as
    // the platform 0x1a4d0 anchor.
    delegate.set_window(window);
}

// 0x1a4f4 — -[AppDelegate .cxx_destruct]
#[doc(alias = "-[AppDelegate .cxx_destruct]")]
pub fn stub_1a4f4(delegate: &AudioAppDelegate) {
    // IDA 0x1a4f4: `connection::disconnect` + weak-slot release. Same as
    // the platform 0x1a4f4 anchor.
    delegate.cxx_destruct();
}

// 0x1a5bc — -[AppDelegate .cxx_construct]
#[doc(alias = "-[AppDelegate .cxx_construct]")]
pub fn stub_1a5bc(delegate: &AudioAppDelegate) {
    // IDA 0x1a5bc: zeroes `messageOutConnection.con.weak_slot.p_`,
    // returns self (the host returns `()`). Same as the platform 0x1a5bc
    // anchor.
    delegate.cxx_construct();
}

// 0x1a5d0 — __GLOBAL__I_a_1
// was: global constructor keyed to_a_1
#[doc(alias = "global constructor keyed to_a_1")]
pub fn stub_1a5d0() {
    // IDA 0x1a5d0 (`__GLOBAL__I_a_1`): `generic_category()` x2 +
    // `system_category()` stores, `std::ios_base::Init` with `__cxa_atexit`
    // teardown, guarded statics. Host statics initialize on use; nothing
    // to run. Same shape as 0x16e4c.
}

// 0x1a768 — _main
#[doc(alias = "_main")]
pub fn stub_1a768(argc: i32) -> i32 {
    // IDA 0x1a768..0x1a7d2 (`main`: `NSAutoreleasePool` alloc/init,
    // `UIApplicationMain(argc, argv, @"UIApplication", @"AppDelegate")`,
    // pool release, return the status): the pool and app-main have no host
    // counterpart; records the launch and returns the status (0).
    let _ = argc;
    AUDIO_MAIN_CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    0
}

// 0x1a7d4 — __GLOBAL__I_a_2
// was: global constructor keyed to_a_2
#[doc(alias = "global constructor keyed to_a_2")]
pub fn stub_1a7d4() {
    // IDA 0x1a7d4 (`__GLOBAL__I_a_2`): static-init key twin of 0x16e4c /
    // 0x1a5d0. Static init — carrier no-op.
}

/// Audio-crate host for the nib-loaded `DebugSettingsViewController` panel
/// (IDA 0x1a970..0x1b14c): nib-loaded state (`window` frame,
/// `keyboardOffset`, `displayPickerArray`) plus the debug-display mode the
/// panel edits. UIKit views have no host counterpart. Mirrors the platform
/// crate `DebugSettingsViewController` model (which owns the full panel);
/// audio cannot depend on platform (AGENTS.md DAG), so the slots these
/// filler EAs touch live here.
#[derive(Debug, Default)]
pub struct AudioDebugSettingsViewController {
    window_frame: parking_lot::Mutex<(f64, f64, f64, f64)>,
    keyboard_offset: std::sync::atomic::AtomicI32,
    display_picker_items: parking_lot::Mutex<Vec<String>>,
    debug_display: std::sync::atomic::AtomicI32,
    view_did_load_calls: std::sync::atomic::AtomicU32,
    animation_runs: std::sync::atomic::AtomicU32,
}

impl AudioDebugSettingsViewController {
    /// `-[DebugSettingsViewController initWithCoder:]` (IDA 0x1a970):
    /// super init first (nil stays nil); iPad gets the fixed 540x508 panel,
    /// otherwise the main-screen bounds; `keyboardOffset = 114`; six-item
    /// picker array. Mirrors the platform twin.
    pub fn init_with_coder(
        super_ok: bool,
        idiom_pad: bool,
        screen_bounds: Option<(f64, f64, f64, f64)>,
    ) -> Option<Self> {
        if !super_ok {
            return None;
        }
        let frame = if idiom_pad {
            (0.0, 0.0, 540.0, 508.0)
        } else {
            screen_bounds.unwrap_or_default()
        };
        Some(Self {
            window_frame: parking_lot::Mutex::new(frame),
            keyboard_offset: std::sync::atomic::AtomicI32::new(114),
            display_picker_items: parking_lot::Mutex::new(
                ["None", "FPS", "Summary", "Physics", "PhysicsAndOwner", "Render"]
                    .into_iter()
                    .map(str::to_owned)
                    .collect(),
            ),
            ..Self::default()
        })
    }

    /// `-[DebugSettingsViewController dealloc]` (IDA 0x1ab20):
    /// picker-array release + super dealloc (runs as drop).
    pub fn dealloc(self) {}

    /// `-[DebugSettingsViewController reloadOldData]` (IDA 0x1ab6c):
    /// empty body.
    pub fn reload_old_data(&self) {}

    /// `-[DebugSettingsViewController viewDidLoad]` (IDA 0x1ab70): super
    /// `viewDidLoad` then `reloadOldData`.
    pub fn view_did_load(&self) {
        self.view_did_load_calls
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.reload_old_data();
    }

    /// `getDebugDisplay` label mapping (IDA 0x1abe6..0x1ac02).
    pub fn display_label(&self) -> &'static str {
        match self.debug_display.load(std::sync::atomic::Ordering::SeqCst) {
            1 => "FPS",
            2 => "Summary",
            3 => "Physics",
            4 => "PhysicsAndOwner",
            5 => "Render",
            _ => "None",
        }
    }

    /// `-[DebugSettingsViewController setDisplayUI]` (IDA 0x1abb0):
    /// `viewWithTag:100` is always present on the host; the switch result
    /// is `setText:`. Returns the label.
    pub fn set_display_ui(&self) -> &'static str {
        self.display_label()
    }

    /// `-[DebugSettingsViewController displayPickerDoneClicked:]` (IDA
    /// 0x1ac80): animation dispatch (recorded), `selectedRowInComponent:0
    /// >= 0` stores the debug display, finishes with `setDisplayUI`.
    /// Returns the label.
    pub fn display_picker_done_clicked(&self, selected_row: i32) -> &'static str {
        self.animation_runs
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        if selected_row >= 0 {
            self.debug_display
                .store(selected_row, std::sync::atomic::Ordering::SeqCst);
        }
        self.set_display_ui()
    }

    /// `__56-[... displayPickerDoneClicked:]_block_invoke` (IDA 0x1ad78)
    /// / `__46-[... displayTouchUp:]_block_invoke` (IDA 0x1afa0): frame
    /// shuffle between the picker, self and the toolbar — pure UIKit
    /// geometry, recorded.
    pub fn display_picker_animation_frame(&self) {
        self.animation_runs
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }

    /// `-[DebugSettingsViewController displayTouchUp:]` (IDA 0x1aed0):
    /// same tag lookup + animation dispatch as done-clicked without the
    /// picker store.
    pub fn display_touch_up(&self) {
        self.animation_runs
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x1a970 — -[DebugSettingsViewController initWithCoder:]
#[doc(alias = "-[DebugSettingsViewController initWithCoder:]")]
pub fn stub_1a970(
    super_ok: bool,
    idiom_pad: bool,
    screen_bounds: Option<(f64, f64, f64, f64)>,
) -> Option<AudioDebugSettingsViewController> {
    // IDA 0x1a970: super init, iPad fixed frame vs screen bounds,
    // `keyboardOffset = 114`, six-item picker array. Same as the platform
    // 0x1a970 anchor.
    AudioDebugSettingsViewController::init_with_coder(super_ok, idiom_pad, screen_bounds)
}
// 0x1ab20 — -[DebugSettingsViewController dealloc]
#[doc(alias = "-[DebugSettingsViewController dealloc]")]
pub fn stub_1ab20(controller: AudioDebugSettingsViewController) {
    // IDA 0x1ab20: picker-array release + super dealloc (runs as drop).
    // Same as the platform 0x1ab20 anchor.
    controller.dealloc();
}
// 0x1ab6c — -[DebugSettingsViewController reloadOldData]
#[doc(alias = "-[DebugSettingsViewController reloadOldData]")]
pub fn stub_1ab6c(controller: &AudioDebugSettingsViewController) {
    // IDA 0x1ab6c: empty body. Same as the platform 0x1ab6c anchor.
    controller.reload_old_data();
}
// 0x1ab70 — -[DebugSettingsViewController viewDidLoad]
#[doc(alias = "-[DebugSettingsViewController viewDidLoad]")]
pub fn stub_1ab70(controller: &AudioDebugSettingsViewController) {
    // IDA 0x1ab70: super `viewDidLoad` then `reloadOldData`. Same as the
    // platform 0x1ab70 anchor.
    controller.view_did_load();
}
// 0x1abb0 — -[DebugSettingsViewController setDisplayUI]
#[doc(alias = "-[DebugSettingsViewController setDisplayUI]")]
pub fn stub_1abb0(controller: &AudioDebugSettingsViewController) -> &'static str {
    // IDA 0x1abb0: `viewWithTag:100` then the `getDebugDisplay` switch
    // into `setText:`. Returns the label. Same as the platform 0x1abb0
    // anchor.
    controller.set_display_ui()
}
// 0x1ac80 — -[DebugSettingsViewController displayPickerDoneClicked:]
#[doc(alias = "-[DebugSettingsViewController displayPickerDoneClicked:]")]
pub fn stub_1ac80(
    controller: &AudioDebugSettingsViewController,
    selected_row: i32,
) -> &'static str {
    // IDA 0x1ac80: tag lookup, animation dispatch, picker store,
    // `setDisplayUI`. Returns the label. Same as the platform 0x1ac80
    // anchor.
    controller.display_picker_done_clicked(selected_row)
}
// 0x1ad78 — ___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke
#[doc(alias = "___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke")]
pub fn stub_1ad78(controller: &AudioDebugSettingsViewController) {
    // IDA 0x1ad78: `setFrame:` shuffle over the picker/self/toolbar
    // frames. Same as the platform 0x1ad78 anchor.
    controller.display_picker_animation_frame();
}
// 0x1ae78 — ___copy_helper_block__0
#[doc(alias = "___copy_helper_block__0")]
pub fn stub_1ae78(
    picker_slot: &mut u64,
    self_slot: &mut u64,
    toolbar_slot: &mut u64,
    picker_src: u64,
    self_src: u64,
    toolbar_src: u64,
) {
    // IDA 0x1ae78: `_Block_object_assign` x3 retaining the captures (two
    // direct + one shim). Same as the platform 0x1ae78 anchor; `0` is
    // `nil`.
    *picker_slot = picker_src;
    *self_slot = self_src;
    *toolbar_slot = toolbar_src;
}
// 0x1aea8 — ___destroy_helper_block__0
#[doc(alias = "___destroy_helper_block__0")]
pub fn stub_1aea8(
    picker_slot: &mut u64,
    self_slot: &mut u64,
    toolbar_slot: &mut u64,
) {
    // IDA 0x1aea8: `_Block_object_dispose` x3 releasing the captures.
    // Same as the platform 0x1aea8 anchor.
    *picker_slot = 0;
    *self_slot = 0;
    *toolbar_slot = 0;
}
// 0x1aed0 — -[DebugSettingsViewController displayTouchUp:]
#[doc(alias = "-[DebugSettingsViewController displayTouchUp:]")]
pub fn stub_1aed0(controller: &AudioDebugSettingsViewController) {
    // IDA 0x1aed0: same tag lookup + animation dispatch as 0x1ac80,
    // without the picker store. Same as the platform 0x1aed0 anchor.
    controller.display_touch_up();
}
// 0x1afa0 — ___46-[DebugSettingsViewController displayTouchUp:]_block_invoke
#[doc(alias = "___46-[DebugSettingsViewController displayTouchUp:]_block_invoke")]
pub fn stub_1afa0(controller: &AudioDebugSettingsViewController) {
    // IDA 0x1afa0 (decompiled frame-shuffle block): same family as the
    // 0x1ad78 animation block — pure UIKit geometry, recorded.
    controller.display_picker_animation_frame();
}
// 0x1b11c — ___copy_helper_block_66
#[doc(alias = "___copy_helper_block_66")]
pub fn stub_1b11c(
    first_slot: &mut u64,
    second_slot: &mut u64,
    third_slot: &mut u64,
    first_src: u64,
    second_src: u64,
    third_src: u64,
) {
    // IDA 0x1b11c (disasm `__Block_object_assign` x3 at +0x14/+0x18/+0x1C):
    // retain the three captures. Same shape as 0x1ae78.
    *first_slot = first_src;
    *second_slot = second_src;
    *third_slot = third_src;
}
// 0x1b14c — ___destroy_helper_block_67
#[doc(alias = "___destroy_helper_block_67")]
pub fn stub_1b14c(first_slot: &mut u64, second_slot: &mut u64, third_slot: &mut u64) {
    // IDA 0x1b14c (disasm `__Block_object_dispose` x3 at +0x14/+0x18/+0x1C):
    // release the three captures. Same shape as 0x1aea8; `0` is `nil`.
    *first_slot = 0;
    *second_slot = 0;
    *third_slot = 0;
}

/// Audio-crate host for `HomeViewController` (IDA 0x1b3d0..0x1c2bc):
/// init/dealloc flags, `viewDidLoad` composition state, keyboard/search
/// slots, localized labels, user-info display, and unload/signup-notification
/// records. UIKit outlets have no host counterpart. Mirrors the platform
/// crate `HomeViewController` model (which owns the full controller);
/// audio cannot depend on platform (AGENTS.md DAG), so the slots these
/// filler EAs touch live here.
#[derive(Debug, Default)]
pub struct AudioHomeViewController {
    initialized: std::sync::atomic::AtomicBool,
    webviews_preloaded: std::sync::atomic::AtomicBool,
    signup_observer_registered: std::sync::atomic::AtomicBool,
    deallocated: std::sync::atomic::AtomicBool,
    released_ivar_count: std::sync::atomic::AtomicU32,
    view_loaded: std::sync::atomic::AtomicBool,
    debug_views_hidden: std::sync::atomic::AtomicBool,
    tap_recognizer_installed: std::sync::atomic::AtomicBool,
    tap_recognizer_enabled: std::sync::atomic::AtomicBool,
    version_text: parking_lot::Mutex<String>,
    keyboard_observers_registered: std::sync::atomic::AtomicU32,
    search_resigns: std::sync::atomic::AtomicU32,
    localized_keys: parking_lot::Mutex<Vec<&'static str>>,
    labels_localized: std::sync::atomic::AtomicU32,
    last_update_refresh: std::sync::atomic::AtomicBool,
    avatar_highlighted: std::sync::atomic::AtomicBool,
    user_info_updates: std::sync::atomic::AtomicU32,
    unloaded_outlets: std::sync::atomic::AtomicU32,
    unloaded: std::sync::atomic::AtomicBool,
    last_signup_credentials: parking_lot::Mutex<Option<(String, String)>>,
    signup_logins: std::sync::atomic::AtomicU32,
    logged_in_state_shows: std::sync::atomic::AtomicU32,
}

impl AudioHomeViewController {
    /// `-[HomeViewController initWithCoder:]` (IDA 0x1b3d0): super init,
    /// webview preload branch, signup-notification observer. Mirrors the
    /// platform twin.
    pub fn init_with_coder(&self) -> bool {
        use std::sync::atomic::Ordering::SeqCst;
        self.webviews_preloaded.store(true, SeqCst);
        self.signup_observer_registered.store(true, SeqCst);
        self.initialized.store(true, SeqCst);
        true
    }

    /// `-[HomeViewController dealloc]` (IDA 0x1b4b0): releases the 30
    /// retained outlets/ivars (Rust drops cover the stores) then super
    /// dealloc. Mirrors the platform twin.
    pub fn dealloc(&self) {
        use std::sync::atomic::Ordering::SeqCst;
        self.released_ivar_count.store(30, SeqCst);
        self.deallocated.store(true, SeqCst);
    }

    /// `-[HomeViewController viewDidLoad]` (IDA 0x1b75c): hides the debug
    /// leaves, installs the disabled `dismissKeyboard` tap recognizer,
    /// localizes labels, refreshes user info, registers the two keyboard
    /// observers, stamps `CFBundleVersion`. Mirrors the platform twin.
    pub fn view_did_load(&self, bundle_version: &str) {
        use std::sync::atomic::Ordering::SeqCst;
        self.debug_views_hidden.store(true, SeqCst);
        self.tap_recognizer_installed.store(true, SeqCst);
        self.tap_recognizer_enabled.store(false, SeqCst);
        self.localize_and_style_labels();
        self.update_user_info_display(false);
        self.keyboard_observers_registered.store(2, SeqCst);
        *self.version_text.lock() = bundle_version.to_owned();
        self.view_loaded.store(true, SeqCst);
    }

    /// `__33-[HomeViewController viewDidLoad]_block_invoke` (IDA 0x1bae4):
    /// background prefetch — only when `searchUrl.length > 0` does it hop
    /// back to main for `block_invoke_2`. Mirrors the platform twin.
    pub fn view_did_load_search_block(&self, search_url_len: usize) -> bool {
        search_url_len > 0
    }

    /// `__33-[HomeViewController viewDidLoad]_block_invoke_2` (IDA 0x1bb64):
    /// `setHidden:NO` on the search field. Mirrors the platform twin.
    pub fn view_did_load_search_apply(&self) {
        self.debug_views_hidden
            .store(false, std::sync::atomic::Ordering::SeqCst);
    }

    /// `-[HomeViewController keyboardDidShow:]` / `keyboardDidHide:`
    /// (IDA 0x1bbb0/0x1bbd0): `tapRecognizer.enabled` YES/NO. Mirrors the
    /// platform twins.
    pub fn keyboard_did_show(&self) {
        self.tap_recognizer_enabled
            .store(true, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn keyboard_did_hide(&self) {
        self.tap_recognizer_enabled
            .store(false, std::sync::atomic::Ordering::SeqCst);
    }

    /// `-[HomeViewController dismissKeyboard]` (IDA 0x1bbf0):
    /// `[_searchTextField resignFirstResponder]`. Mirrors the platform
    /// twin.
    pub fn dismiss_keyboard(&self) {
        self.search_resigns
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }

    /// `NSBundle` keys `localizeAndStyleLabels` stamps (IDA 0x1bc48..0x1bf08).
    pub const LOCALIZED_LABEL_KEYS: [&'static str; 11] = [
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

    /// `-[HomeViewController localizeAndStyleLabels]` (IDA 0x1bc10):
    /// eleven localized stamps; the bundle lookup lives out of slice.
    /// Mirrors the platform twin.
    pub fn localize_and_style_labels(&self) {
        *self.localized_keys.lock() = Self::LOCALIZED_LABEL_KEYS.to_vec();
        self.labels_localized.store(
            Self::LOCALIZED_LABEL_KEYS.len() as u32,
            std::sync::atomic::Ordering::SeqCst,
        );
    }

    /// `-[HomeViewController updateUserInfoDisplay:]` (IDA 0x1bf0c): with
    /// the flag set, `UpdatePlayerInfo` first; Robux/Tix labels, username
    /// when non-nil, avatar thumbnail (web fetch out of slice — the
    /// avatar-present branch is recorded). Mirrors the platform twin.
    pub fn update_user_info_display(&self, refresh: bool) {
        self.last_update_refresh
            .store(refresh, std::sync::atomic::Ordering::SeqCst);
        self.avatar_highlighted
            .store(false, std::sync::atomic::Ordering::SeqCst);
        self.user_info_updates
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }

    /// `-[HomeViewController viewDidUnload]` (IDA 0x1c134): nils the 18
    /// outlet setters then super `viewDidUnload` (Rust drops cover the
    /// stores). Mirrors the platform twin.
    pub fn view_did_unload(&self) {
        use std::sync::atomic::Ordering::SeqCst;
        self.unloaded_outlets.store(18, SeqCst);
        self.unloaded.store(true, SeqCst);
    }

    /// `-[HomeViewController handleSignupNotification:]` (IDA 0x1c2bc):
    /// retains the `username`/`password` pair, drives login, shows the
    /// logged-in state. Mirrors the platform twin.
    pub fn handle_signup_notification(&self, username: &str, password: &str) {
        *self.last_signup_credentials.lock() =
            Some((username.to_owned(), password.to_owned()));
        self.signup_logins
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.logged_in_state_shows
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x1b3d0 — -[HomeViewController initWithCoder:]
#[doc(alias = "-[HomeViewController initWithCoder:]")]
pub fn stub_1b3d0(controller: &AudioHomeViewController) -> bool {
    // IDA 0x1b3d0 (`-[HomeViewController initWithCoder:]`): super init,
    // webview preload, signup observer. Same as the platform 0x1b3d0
    // anchor.
    controller.init_with_coder()
}

// 0x1b4b0 — -[HomeViewController dealloc]
#[doc(alias = "-[HomeViewController dealloc]")]
pub fn stub_1b4b0(controller: &AudioHomeViewController) {
    // IDA 0x1b4b0 (`-[HomeViewController dealloc]`): releases the 30
    // retained outlets then super dealloc. Same as the platform 0x1b4b0
    // anchor.
    controller.dealloc();
}

// 0x1b75c — -[HomeViewController viewDidLoad]
#[doc(alias = "-[HomeViewController viewDidLoad]")]
pub fn stub_1b75c(controller: &AudioHomeViewController, bundle_version: &str) {
    // IDA 0x1b75c (`-[HomeViewController viewDidLoad]`): debug-leaf hides,
    // tap recognizer, labels, user info, prefetch dispatch, keyboard
    // observers, version stamp. Same as the platform 0x1b75c anchor.
    controller.view_did_load(bundle_version);
}

// 0x1bae4 — ___33-[HomeViewController viewDidLoad]_block_invoke
#[doc(alias = "___33-[HomeViewController viewDidLoad]_block_invoke")]
pub fn stub_1bae4(controller: &AudioHomeViewController, search_url_len: usize) -> bool {
    // IDA 0x1bae4 (prefetch block): main-queue hop only when the search
    // URL is non-empty. Same as the platform 0x1bae4 anchor.
    controller.view_did_load_search_block(search_url_len)
}

// 0x1bb64 — ___33-[HomeViewController viewDidLoad]_block_invoke_2
#[doc(alias = "___33-[HomeViewController viewDidLoad]_block_invoke_2")]
pub fn stub_1bb64(controller: &AudioHomeViewController) {
    // IDA 0x1bb64 (prefetch-apply block): `setHidden:NO` on the search
    // field. Same as the platform 0x1bb64 anchor.
    controller.view_did_load_search_apply();
}

// 0x1bb88 — ___copy_helper_block__1
#[doc(alias = "___copy_helper_block__1")]
pub fn stub_1bb88(slot: &mut u64, src: u64) {
    // IDA 0x1bb88 (disasm one `__Block_object_assign`): retain the
    // capture. Same shape as 0x18094; `0` is `nil`.
    *slot = src;
}

// 0x1bb94 — ___destroy_helper_block__1
#[doc(alias = "___destroy_helper_block__1")]
pub fn stub_1bb94(slot: &mut u64) {
    // IDA 0x1bb94 (disasm one `__Block_object_dispose`): release the
    // capture. Same shape as 0x180a0.
    *slot = 0;
}

// 0x1bb9c — ___copy_helper_block_80
#[doc(alias = "___copy_helper_block_80")]
pub fn stub_1bb9c(slot: &mut u64, src: u64) {
    // IDA 0x1bb9c (disasm one `__Block_object_assign`): retain the
    // capture. Same shape as 0x18094.
    *slot = src;
}

// 0x1bba8 — ___destroy_helper_block_81
#[doc(alias = "___destroy_helper_block_81")]
pub fn stub_1bba8(slot: &mut u64) {
    // IDA 0x1bba8 (disasm one `__Block_object_dispose`): release the
    // capture. Same shape as 0x180a0.
    *slot = 0;
}

// 0x1bbb0 — -[HomeViewController keyboardDidShow:]
#[doc(alias = "-[HomeViewController keyboardDidShow:]")]
pub fn stub_1bbb0(controller: &AudioHomeViewController) {
    // IDA 0x1bbb0 (`-[HomeViewController keyboardDidShow:]`):
    // `tapRecognizer.enabled = YES`. Same as the platform 0x1bbb0 anchor.
    controller.keyboard_did_show();
}

// 0x1bbd0 — -[HomeViewController keyboardDidHide:]
#[doc(alias = "-[HomeViewController keyboardDidHide:]")]
pub fn stub_1bbd0(controller: &AudioHomeViewController) {
    // IDA 0x1bbd0 (`-[HomeViewController keyboardDidHide:]`):
    // `tapRecognizer.enabled = NO`. Same as the platform 0x1bbd0 anchor.
    controller.keyboard_did_hide();
}

// 0x1bbf0 — -[HomeViewController dismissKeyboard]
#[doc(alias = "-[HomeViewController dismissKeyboard]")]
pub fn stub_1bbf0(controller: &AudioHomeViewController) {
    // IDA 0x1bbf0 (`-[HomeViewController dismissKeyboard]`):
    // `[_searchTextField resignFirstResponder]`. Same as the platform
    // 0x1bbf0 anchor.
    controller.dismiss_keyboard();
}

// 0x1bc10 — -[HomeViewController localizeAndStyleLabels]
#[doc(alias = "-[HomeViewController localizeAndStyleLabels]")]
pub fn stub_1bc10(controller: &AudioHomeViewController) {
    // IDA 0x1bc10 (`-[HomeViewController localizeAndStyleLabels]`):
    // eleven localized stamps. Same as the platform 0x1bc10 anchor.
    controller.localize_and_style_labels();
}

// 0x1bf0c — -[HomeViewController updateUserInfoDisplay:]
#[doc(alias = "-[HomeViewController updateUserInfoDisplay:]")]
pub fn stub_1bf0c(controller: &AudioHomeViewController, refresh: bool) {
    // IDA 0x1bf0c (`-[HomeViewController updateUserInfoDisplay:]`):
    // conditional player-info update, Robux/Tix/username labels, avatar.
    // Same as the platform 0x1bf0c anchor.
    controller.update_user_info_display(refresh);
}

// 0x1c134 — -[HomeViewController viewDidUnload]
#[doc(alias = "-[HomeViewController viewDidUnload]")]
pub fn stub_1c134(controller: &AudioHomeViewController) {
    // IDA 0x1c134 (`-[HomeViewController viewDidUnload]`): nils the 18
    // outlet setters then super `viewDidUnload`. Same as the platform
    // 0x1c134 anchor.
    controller.view_did_unload();
}

// 0x1c2bc — -[HomeViewController handleSignupNotification:]
#[doc(alias = "-[HomeViewController handleSignupNotification:]")]
pub fn stub_1c2bc(controller: &AudioHomeViewController, username: &str, password: &str) {
    // IDA 0x1c2bc (`-[HomeViewController handleSignupNotification:]`):
    // retain credentials, login, show logged-in state. Same as the
    // platform 0x1c2bc anchor.
    controller.handle_signup_notification(username, password);
}

// 0x1c37c — -[HomeViewController logoutTouchUp:]
#[doc(alias = "-[HomeViewController logoutTouchUp:]")]
pub fn stub_1c37c() -> ! {
    todo!("0x1c37c -[HomeViewController logoutTouchUp:]")
}

// 0x1c4b0 — -[HomeViewController alertView:didDismissWithButtonIndex:]
#[doc(alias = "-[HomeViewController alertView:didDismissWithButtonIndex:]")]
pub fn stub_1c4b0() -> ! {
    todo!("0x1c4b0 -[HomeViewController alertView:didDismissWithButtonIndex:]")
}

// 0x1c5c8 — ___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke
#[doc(alias = "___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke")]
pub fn stub_1c5c8() -> ! {
    todo!("0x1c5c8 ___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke")
}

// 0x1c5f4 — ___copy_helper_block_224
#[doc(alias = "___copy_helper_block_224")]
pub fn stub_1c5f4() -> ! {
    todo!("0x1c5f4 ___copy_helper_block_224")
}

// 0x1c600 — ___destroy_helper_block_225
#[doc(alias = "___destroy_helper_block_225")]
pub fn stub_1c600() -> ! {
    todo!("0x1c600 ___destroy_helper_block_225")
}

// 0x1c608 — ___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke227
#[doc(alias = "___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke227")]
pub fn stub_1c608() -> ! {
    todo!("0x1c608 ___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke227")
}

// 0x1c734 — ___copy_helper_block_246
#[doc(alias = "___copy_helper_block_246")]
pub fn stub_1c734() -> ! {
    todo!("0x1c734 ___copy_helper_block_246")
}

// 0x1c740 — ___destroy_helper_block_247
#[doc(alias = "___destroy_helper_block_247")]
pub fn stub_1c740() -> ! {
    todo!("0x1c740 ___destroy_helper_block_247")
}

// 0x1c748 — -[HomeViewController viewWillAppear:]
#[doc(alias = "-[HomeViewController viewWillAppear:]")]
pub fn stub_1c748() -> ! {
    todo!("0x1c748 -[HomeViewController viewWillAppear:]")
}

// 0x1c788 — -[HomeViewController showCorrectLoggedInState]
#[doc(alias = "-[HomeViewController showCorrectLoggedInState]")]
pub fn stub_1c788() -> ! {
    todo!("0x1c788 -[HomeViewController showCorrectLoggedInState]")
}

// 0x1c860 — ___46-[HomeViewController showCorrectLoggedInState]_block_invoke
#[doc(alias = "___46-[HomeViewController showCorrectLoggedInState]_block_invoke")]
pub fn stub_1c860() -> ! {
    todo!("0x1c860 ___46-[HomeViewController showCorrectLoggedInState]_block_invoke")
}

// 0x1c874 — ___copy_helper_block_261
#[doc(alias = "___copy_helper_block_261")]
pub fn stub_1c874() -> ! {
    todo!("0x1c874 ___copy_helper_block_261")
}

// 0x1c880 — ___destroy_helper_block_262
#[doc(alias = "___destroy_helper_block_262")]
pub fn stub_1c880() -> ! {
    todo!("0x1c880 ___destroy_helper_block_262")
}

// 0x1c888 — -[HomeViewController viewDidAppear:]
#[doc(alias = "-[HomeViewController viewDidAppear:]")]
pub fn stub_1c888() -> ! {
    todo!("0x1c888 -[HomeViewController viewDidAppear:]")
}

// 0x1c8e8 — -[HomeViewController handleStartGameFailure]
#[doc(alias = "-[HomeViewController handleStartGameFailure]")]
pub fn stub_1c8e8() -> ! {
    todo!("0x1c8e8 -[HomeViewController handleStartGameFailure]")
}

// 0x1c958 — -[HomeViewController handleStartGameSuccess]
#[doc(alias = "-[HomeViewController handleStartGameSuccess]")]
pub fn stub_1c958() -> ! {
    todo!("0x1c958 -[HomeViewController handleStartGameSuccess]")
}

// 0x1c95c — -[HomeViewController placeIdClicked:]
#[doc(alias = "-[HomeViewController placeIdClicked:]")]
pub fn stub_1c95c() -> ! {
    todo!("0x1c95c -[HomeViewController placeIdClicked:]")
}

// 0x1ca9c — -[HomeViewController searchEditingDidEnd:]
#[doc(alias = "-[HomeViewController searchEditingDidEnd:]")]
pub fn stub_1ca9c() -> ! {
    todo!("0x1ca9c -[HomeViewController searchEditingDidEnd:]")
}

// 0x1caa0 — -[HomeViewController searchDidEndOnExit:]
#[doc(alias = "-[HomeViewController searchDidEndOnExit:]")]
pub fn stub_1caa0() -> ! {
    todo!("0x1caa0 -[HomeViewController searchDidEndOnExit:]")
}

// 0x1cac8 — -[HomeViewController signUpButtonDidTouchUpInside:]
#[doc(alias = "-[HomeViewController signUpButtonDidTouchUpInside:]")]
pub fn stub_1cac8() -> ! {
    todo!("0x1cac8 -[HomeViewController signUpButtonDidTouchUpInside:]")
}

// 0x1cacc — -[HomeViewController logInButtonDidTouchUpInside:]
#[doc(alias = "-[HomeViewController logInButtonDidTouchUpInside:]")]
pub fn stub_1cacc() -> ! {
    todo!("0x1cacc -[HomeViewController logInButtonDidTouchUpInside:]")
}
