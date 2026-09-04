//! AppDelegate — iOS app delegate + GameViewController/RobloxView lifecycle bridge.
//!
//! Mirrors `Client/iOS/AppDelegate.*` and the `GameViewController` lifecycle
//! (`Client/iOS/GameViewController.*`). The heavy `AppDelegate` /
//! `GameViewController` models live in `crate::view_controllers` (IDA-grounded);
//! this module keeps that re-export for compatibility and adds the thin
//! ObjC-selector free functions plus the `RobloxView` lifecycle tracker used by
//! the generated `stub_0xADDR` leaves.
//!
//! `SharedPtr` = `rbx_core::SharedPtr` (`Arc`), never `boost::shared_ptr`.
//! ObjC `id` (nullable object pointer) has no host runtime here; `None` is `nil`.

#![allow(dead_code)]

pub use crate::view_controllers::*;

/// `UIApplication` lifecycle as driven through the `AppDelegate` callbacks
/// (`applicationDidBecomeActive:` / `WillResignActive:` /
/// `DidEnterBackground:` / `WillEnterForeground:` / `WillTerminate:`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LifecycleState {
    #[default]
    NotLaunched,
    Active,
    Inactive,
    Background,
    Terminated,
}

/// Process-wide lifecycle tracker backing the `AppDelegate` callbacks.
/// UIKit posts the transitions; the counters below stand in for the hosted app
/// state (`RobloxAppState` defaults, background task, game view presentation).
#[derive(Debug, Default)]
pub struct AppLifecycle {
    state: parking_lot::Mutex<LifecycleState>,
    transitions: std::sync::atomic::AtomicU32,
    game_view_controller: parking_lot::Mutex<Option<ObjCId>>,
    roblox_view: parking_lot::Mutex<Option<ObjCId>>,
    view_loads: std::sync::atomic::AtomicU32,
}

impl AppLifecycle {
    pub fn new() -> Self {
        Self::default()
    }

    fn transition(&self, next: LifecycleState) {
        *self.state.lock() = next;
        self.transitions
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn state(&self) -> LifecycleState {
        *self.state.lock()
    }

    pub fn transition_count(&self) -> u32 {
        self.transitions.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn attach_game_view_controller(&self, id: ObjCId) {
        *self.game_view_controller.lock() = Some(id);
    }

    pub fn game_view_controller(&self) -> Option<ObjCId> {
        *self.game_view_controller.lock()
    }

    /// `RobloxView` creation side of the lifecycle (`initControlViewHelper`
    /// installs the view; teardown clears it).
    pub fn attach_roblox_view(&self, id: ObjCId) {
        *self.roblox_view.lock() = Some(id);
    }

    pub fn detach_roblox_view(&self) {
        *self.roblox_view.lock() = None;
    }

    pub fn roblox_view(&self) -> Option<ObjCId> {
        *self.roblox_view.lock()
    }

    pub fn view_load_count(&self) -> u32 {
        self.view_loads.load(std::sync::atomic::Ordering::SeqCst)
    }
}

// 0x4da00 — -[GameViewController viewDidLoad]
/// ObjC-only mapping for `-[GameViewController viewDidLoad]`
/// (`[super viewDidLoad]` + `UserAgent` default registration).
// IDA 0x4da00: super send (0x4da24), `+[RobloxInfo getUserAgentString]`
// (0x4da4c), `registerDefaults:` + temp-dictionary release (0x4da72..0x4dab0).
#[doc(alias = "-[GameViewController viewDidLoad]")]
#[doc = "-[GameViewController viewDidLoad]"]
pub fn view_did_load(controller: &GameViewController, lifecycle: &AppLifecycle) {
    controller.view_did_load();
    lifecycle
        .view_loads
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x4d990 — -[GameViewController viewWillAppear:]
/// ObjC-only mapping for `-[GameViewController viewWillAppear:]`
/// (`[super viewWillAppear:animated]`; nothing else).
// IDA 0x4d990.
#[doc(alias = "-[GameViewController viewWillAppear:]")]
#[doc = "-[GameViewController viewWillAppear:]"]
pub fn view_will_appear(controller: &GameViewController, animated: bool) {
    controller.view_will_appear(animated);
}

// 0x4d9d4 — -[GameViewController viewDidAppear:]
/// ObjC-only mapping for `-[GameViewController viewDidAppear:]`
/// (`[super viewDidAppear:animated]`; nothing else).
// IDA 0x4d9d4.
#[doc(alias = "-[GameViewController viewDidAppear:]")]
#[doc = "-[GameViewController viewDidAppear:]"]
pub fn view_did_appear(controller: &GameViewController, animated: bool) {
    controller.view_did_appear(animated);
}

// 0x4dab8 — -[GameViewController didReceiveMemoryWarning]
/// ObjC-only mapping for `-[GameViewController didReceiveMemoryWarning]`
/// (`[super didReceiveMemoryWarning]`; nothing else).
// IDA 0x4dab8.
#[doc(alias = "-[GameViewController didReceiveMemoryWarning]")]
#[doc = "-[GameViewController didReceiveMemoryWarning]"]
pub fn did_receive_memory_warning(controller: &GameViewController) {
    controller.did_receive_memory_warning();
}

// 0x19302 — -[AppDelegate application:didFinishLaunchingWithOptions:]
/// ObjC-only mapping for `-[AppDelegate application:didFinishLaunchingWithOptions:]`.
/// Registers the `warnings_preference` / `wifionly_preference` defaults, then
/// moves the lifecycle to `Active`.
// IDA 0x19302..0x19366.
#[doc(alias = "-[AppDelegate application:didFinishLaunchingWithOptions:]")]
#[doc = "-[AppDelegate application:didFinishLaunchingWithOptions:]"]
pub fn application_did_finish_launching(
    delegate: &AppDelegate,
    lifecycle: &AppLifecycle,
) -> bool {
    let ok = delegate.application_did_finish_launching();
    if ok {
        lifecycle.transition(LifecycleState::Active);
    }
    ok
}

// 0x19600 — -[AppDelegate applicationWillResignActive:]
/// ObjC-only mapping for `-[AppDelegate applicationWillResignActive:]`.
// IDA 0x19600.
#[doc(alias = "-[AppDelegate applicationWillResignActive:]")]
#[doc = "-[AppDelegate applicationWillResignActive:]"]
pub fn application_will_resign_active(delegate: &AppDelegate, lifecycle: &AppLifecycle) {
    delegate.application_will_resign_active();
    lifecycle.transition(LifecycleState::Inactive);
}

// 0x19742 — -[AppDelegate applicationDidEnterBackground:]
/// ObjC-only mapping for `-[AppDelegate applicationDidEnterBackground:]`
/// (`RobloxAppState=tryBackground` + synchronize).
// IDA 0x19742..0x1975c.
#[doc(alias = "-[AppDelegate applicationDidEnterBackground:]")]
#[doc = "-[AppDelegate applicationDidEnterBackground:]"]
pub fn application_did_enter_background(delegate: &AppDelegate, lifecycle: &AppLifecycle) {
    delegate.application_did_enter_background();
    lifecycle.transition(LifecycleState::Background);
}

// 0x19bc0 — -[AppDelegate applicationWillEnterForeground:]
/// ObjC-only mapping for `-[AppDelegate applicationWillEnterForeground:]`.
// IDA 0x19bc0.
#[doc(alias = "-[AppDelegate applicationWillEnterForeground:]")]
#[doc = "-[AppDelegate applicationWillEnterForeground:]"]
pub fn application_will_enter_foreground(delegate: &AppDelegate, lifecycle: &AppLifecycle) {
    delegate.application_will_enter_foreground();
    lifecycle.transition(LifecycleState::Inactive);
}

// 0x19d3c — -[AppDelegate applicationDidBecomeActive:]
/// ObjC-only mapping for `-[AppDelegate applicationDidBecomeActive:]`
/// (`RobloxAppState=tryForeground` + synchronize).
// IDA 0x19d3c..0x19d56.
#[doc(alias = "-[AppDelegate applicationDidBecomeActive:]")]
#[doc = "-[AppDelegate applicationDidBecomeActive:]"]
pub fn application_did_become_active(delegate: &AppDelegate, lifecycle: &AppLifecycle) {
    delegate.application_did_become_active();
    lifecycle.transition(LifecycleState::Active);
}

// 0x19fbc — -[AppDelegate applicationWillTerminate:]
/// ObjC-only mapping for `-[AppDelegate applicationWillTerminate:]`.
// IDA 0x19fbc.
#[doc(alias = "-[AppDelegate applicationWillTerminate:]")]
#[doc = "-[AppDelegate applicationWillTerminate:]"]
pub fn application_will_terminate(delegate: &AppDelegate, lifecycle: &AppLifecycle) {
    delegate.application_will_terminate();
    lifecycle.transition(LifecycleState::Terminated);
}
