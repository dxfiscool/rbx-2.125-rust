// Auto-generated skeletons for rbx-script — Lua|Script|CodeGen batch
// Filter: Lua|Script|CodeGen (4456 filtered, 0 remaining) -> global gap filler EA-sorted asc next 150 not yet in script crate
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +150 stubs | range 0x24204..0x30a24 EA-sorted asc next 150 global not yet in script crate (script 16083 -> 16233 total, global-free 0->0 distinct)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

use crate::generated_165::{BlockCapture, IosSettingsService};

/// C++ static-init cell for `__GLOBAL__I_a_7` (IDA 0x24540): same
/// static-init family as `__GLOBAL__I_a_6` (disasm `BL
/// generic_category` head; `boost::system` statics,
/// `std::ios_base::Init`, exception-guard chain).
#[derive(Debug, Clone, Default)]
pub struct CxxRuntimeA7 {
    pub initialized: bool,
}

/// Host-side `Teleporter` (IDA 0x246d8): the C++ peer owned by the
/// `PlaceLauncher` (`shared_ptr<Teleporter>`); vtable/offsets fold
/// into host ownership.
#[derive(Debug, Clone, Default)]
pub struct TeleporterState {
    /// `RBX::FunctionMarshaller::GetWindow` result captured at `init`.
    pub window: Option<u32>,
}

/// Host-side `PlaceLauncher` state (PlaceLauncher.m, IDA 0x246d8..0x24a48).
/// UIKit/RBX objects (`Teleporter`, `NSString`s) live on their
/// respective sides; only the observable latches are modeled here.
#[derive(Debug, Clone, Default)]
pub struct PlaceLauncherState {
    /// `init` ran (IDA 0x246d8).
    pub initialized: bool,
    /// `dealloc` ran (IDA 0x248dc).
    pub released: bool,
    /// `rbxView` (nil at `init`, IDA 0x246d8).
    pub rbx_view: Option<u32>,
    /// `hasReceivedMemoryWarning` (IDA 0x246d8).
    pub memory_warning: bool,
    /// `isCurrentlyPlayingGame` (IDA 0x246d8/0x24a18).
    pub currently_playing: bool,
    /// `isLeavingGame` (IDA 0x299a2 sets, 0x29bb4/0x29684 clear).
    pub is_leaving_game: bool,
    /// `childConnection`/`playerConnection` live flags (IDA 0x2b1bc
    /// connects, 0x2b5e0/0x2b548 disconnect).
    pub child_connected: bool,
    pub player_connected: bool,
    /// `lastPlaceId` (IDA 0x246d8).
    pub last_place_id: i32,
    /// `teleporter` shared_ptr (IDA 0x246d8/0x248dc).
    pub teleporter: Option<TeleporterState>,
    /// `RBX::TeleportService::SetCallback` live registration (IDA
    /// 0x246d8 sets, 0x248dc clears).
    pub teleport_callback_set: bool,
    /// `RBXDidLeaveGameNotification` (IDA 0x246d8/0x24a28).
    pub did_leave_game_notification: String,
    /// `RBXStartLeaveGameNotification` (IDA 0x246d8/0x24a38).
    pub start_leave_game_notification: String,
    /// `RBXGameFinishedLoadingNotification` (IDA 0x246d8/0x24a48).
    pub game_finished_loading_notification: String,
    /// `resourcePath + "/content"` via `ContentProvider::setAssetFolder`
    /// + `Game::globalInit` + `TeleportService::SetBaseUrl` (IDA 0x24ab0).
    pub asset_folder: Option<String>,
    pub game_global_init: bool,
    pub teleport_base_url_set: bool,
    /// `RBX::DataModel::hash = "ios,ios"` (IDA 0x24ab0).
    pub datamodel_hash: Option<String>,
    /// `GlobalBasicSettings::loadState("")` ran (IDA 0x24ab0).
    pub basic_settings_loaded: bool,
    /// `TaskScheduler::setThreadCount` from settings ran (IDA 0x24ab0).
    pub scheduler_threads_set: bool,
    /// Last `RobloxAlertWithMessage:` key (IDA 0x24ab0/0x2512c).
    pub last_alert: Option<String>,
    /// Last `StandardOut::printf` line (IDA 0x24ab0).
    pub last_log: Option<String>,
    /// `gameFinishedLoadingNotification` posted by
    /// `placeDidFinishLoading` (IDA 0x253e0).
    pub finished_notification_posted: bool,
    pub finished_notification: Option<String>,
    /// Last posted `startLeaveGame`/`didLeaveGame` notification name
    /// (IDA 0x295c0/0x29684).
    pub posted_notification: Option<String>,
    /// `stopFreeMemoryChecker` ran via `deleteRobloxView` (IDA 0x25440).
    pub memory_checker_stopped: bool,
    /// Last non-game controller (via `MainViewController`, IDA 0x24a58).
    pub last_non_game_controller: Option<u32>,
    /// `handleStartGameFailure` forwarded to it (IDA 0x24a58).
    pub failure_forwarded: bool,
    /// `TooManyParts` warning latched by the part-count block (IDA 0x2512c).
    pub part_warning: Option<PartWarning>,
    /// Last `RobloxGoogleAnalytics` event `(category, action, label)`
    /// (IDA 0x2512c).
    pub ga_event: Option<GaEvent>,
}

/// `+[PlaceLauncher sharedInstance]` singleton cell (IDA 0x24974):
/// `dword_130C440` predicate + `dword_130C444` instance.
#[derive(Debug, Clone, Default)]
pub struct PlaceLauncherRegistry {
    /// `dispatch_once` predicate (IDA 0x24974).
    pub once_token: bool,
    pub launcher: PlaceLauncherState,
}

/// Host `shared_ptr<RBX::Game>` built by
/// `setupGame:unsecuredGame:isApp:` (IDA 0x26558): `SecurePlayerGame`
/// vs `UnsecuredStudioGame` (`GetBaseURL()`, `isApp` normalized to 0/1)
/// with `ClientAppSettings` initialized, the settings fetch done, the
/// idle timer disabled, and `isCurrentlyPlayingGame` latched.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GameToken {
    /// `UnsecuredStudioGame` (`a5`) vs `SecurePlayerGame`.
    pub unsecured: bool,
    /// Normalized `isApp` flag.
    pub is_app: bool,
}
// 0x24204 — __ZN18iOSSettingsService45ReadValueMemoryBouncerEnforceRateMilliSecondsEPKc
// type: int __fastcall(iOSSettingsService *this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueMemoryBouncerEnforceRateMilliSeconds(char const*)")]
pub fn stub_0x24204(service: &mut IosSettingsService, value: &str) -> i32 {
    // IDA 0x24204: `atoi(value)` into `_thisPtr + 164`.
    let parsed = crate::generated_165::c_atoi(value);
    service.memory_bouncer_enforce_rate_ms = parsed;
    parsed
}

// 0x24220 — __ZN18iOSSettingsService40ReadValueMemoryBouncerThresholdKiloBytesEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueMemoryBouncerThresholdKiloBytes(char const*)")]
pub fn stub_0x24220(service: &mut IosSettingsService, value: &str) -> i32 {
    // IDA 0x24220: `atoi(value)` into `_thisPtr + 168`.
    let parsed = crate::generated_165::c_atoi(value);
    service.memory_bouncer_threshold_kb = parsed;
    parsed
}

// 0x2423c — __ZN18iOSSettingsService36ReadValueMemoryBouncerLimitMegaBytesEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueMemoryBouncerLimitMegaBytes(char const*)")]
pub fn stub_0x2423c(service: &mut IosSettingsService, value: &str) -> i32 {
    // IDA 0x2423c: `atoi(value)` into `_thisPtr + 172`.
    let parsed = crate::generated_165::c_atoi(value);
    service.memory_bouncer_limit_mb = parsed;
    parsed
}

// 0x24258 — __ZN18iOSSettingsService52ReadValueMemoryBouncerLimitMegaBytesForLowMemDevicesEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueMemoryBouncerLimitMegaBytesForLowMemDevices(char const*)")]
pub fn stub_0x24258(service: &mut IosSettingsService, value: &str) -> i32 {
    // IDA 0x24258: `atoi(value)` into `_thisPtr + 176`.
    let parsed = crate::generated_165::c_atoi(value);
    service.memory_bouncer_limit_lowmem_mb = parsed;
    parsed
}

// 0x24540 — __GLOBAL__I_a_7
#[doc(alias = "global constructor keyed to_a_7")]
pub fn stub_0x24540(runtime: &mut CxxRuntimeA7) {
    // IDA 0x24540 `__GLOBAL__I_a_7` (0x24540..0x246d6): C++ static init
    // of the same family as `__GLOBAL__I_a_6` (cf. 0x21c18).
    runtime.initialized = true;
}

// 0x246d8 — -[PlaceLauncher init]
// type: PlaceLauncher *__cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher init]")]
pub fn stub_0x246d8(state: &mut PlaceLauncherState, window: Option<u32>) {
    // IDA 0x246d8 `-[PlaceLauncher init]`: super `init`, zeroed
    // ivars, fresh `Teleporter(self, GetWindow())` into the
    // shared_ptr (old value released; vtable/offsets fold into host
    // ownership) + `TeleportService::SetCallback`, and the three
    // `RBX*Notification` strings.
    state.initialized = true;
    state.released = false;
    state.rbx_view = None;
    state.memory_warning = false;
    state.currently_playing = false;
    state.is_leaving_game = false;
    state.child_connected = false;
    state.player_connected = false;
    state.last_place_id = 0;
    state.teleporter = Some(TeleporterState { window });
    state.teleport_callback_set = true;
    state.did_leave_game_notification = "RBXDidLeaveGameNotification".to_string();
    state.start_leave_game_notification = "RBXStartLeaveGameNotification".to_string();
    state.game_finished_loading_notification = "RBXGameFinishedLoadingNotification".to_string();
}

// 0x248dc — -[PlaceLauncher dealloc]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher dealloc]")]
pub fn stub_0x248dc(state: &mut PlaceLauncherState) {
    // IDA 0x248dc `-[PlaceLauncher dealloc]`: `SetCallback(0)`,
    // teleporter release, the three notification releases, super
    // `dealloc` (releases fold into host ownership).
    state.teleport_callback_set = false;
    state.teleporter = None;
    state.did_leave_game_notification.clear();
    state.start_leave_game_notification.clear();
    state.game_finished_loading_notification.clear();
    state.released = true;
}

// 0x24974 — +[PlaceLauncher sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[PlaceLauncher sharedInstance]")]
pub fn stub_0x24974(reg: &mut PlaceLauncherRegistry) -> &mut PlaceLauncherState {
    // IDA 0x24974 `+[PlaceLauncher sharedInstance]`: `dispatch_once`
    // singleton (0x2498e..0x2499a -> 0x249d0); the predicate folds
    // into `once_token` (cf. 0x20e78).
    if !reg.once_token {
        stub_0x249d0(reg);
        reg.once_token = true;
    }
    &mut reg.launcher
}

// 0x249d0 — ___31+[PlaceLauncher sharedInstance]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___31+[PlaceLauncher sharedInstance]_block_invoke")]
pub fn stub_0x249d0(reg: &mut PlaceLauncherRegistry) {
    // IDA 0x249d0: `alloc` + `init` into `dword_130C444`
    // (`GetWindow` resolves on the platform side; `None` here).
    reg.launcher = PlaceLauncherState::default();
    stub_0x246d8(&mut reg.launcher, None);
}

// 0x24a04 — ___copy_helper_block__4
#[doc(alias = "___copy_helper_block__4")]
pub fn stub_0x24a04(dst: &mut BlockCapture, src: &BlockCapture) {
    // IDA 0x24a04 `__copy_helper_block__4`: single
    // `_Block_object_assign` retain (cf. 0x1f660).
    *dst = src.clone();
}

// 0x24a10 — ___destroy_helper_block__4
#[doc(alias = "___destroy_helper_block__4")]
pub fn stub_0x24a10(slot: &mut BlockCapture) {
    // IDA 0x24a10 `__destroy_helper_block__4`: single
    // `_Block_object_dispose` release (cf. 0x1f4a0).
    *slot = BlockCapture::default();
}

// 0x24a18 — -[PlaceLauncher getIsCurrentlyPlayingGame]
// type: char __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher getIsCurrentlyPlayingGame]")]
pub fn stub_0x24a18(state: &PlaceLauncherState) -> bool {
    // IDA 0x24a18: `isCurrentlyPlayingGame` IVAR load.
    state.currently_playing
}

// 0x24a28 — -[PlaceLauncher getDidLeaveGameNotification]
// type: id __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher getDidLeaveGameNotification]")]
pub fn stub_0x24a28(state: &PlaceLauncherState) -> &str {
    // IDA 0x24a28: `didLeaveGameNotification` IVAR load.
    &state.did_leave_game_notification
}

// 0x24a38 — -[PlaceLauncher getStartLeaveGameNotification]
// type: id __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher getStartLeaveGameNotification]")]
pub fn stub_0x24a38(state: &PlaceLauncherState) -> &str {
    // IDA 0x24a38: `startLeaveGameNotification` IVAR load.
    &state.start_leave_game_notification
}

// 0x24a48 — -[PlaceLauncher getGameFinishedLoadingNotification]
// type: id __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher getGameFinishedLoadingNotification]")]
pub fn stub_0x24a48(state: &PlaceLauncherState) -> &str {
    // IDA 0x24a48: `gameFinishedLoadingNotification` IVAR load.
    &state.game_finished_loading_notification
}

/// `TooManyParts` warning (IDA 0x2512c): `WarnPlaceIsNotIdeal` title +
/// `WarnTooManyParts(partCount, maxParts)` body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartWarning {
    pub max_parts: i32,
    pub part_count: i32,
}

/// `RobloxGoogleAnalytics setEventTracking:` latch (IDA 0x2512c).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GaEvent {
    pub category: String,
    pub action: String,
    pub label: i32,
}

// 0x24a58 — -[PlaceLauncher handleStartGameFailure]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher handleStartGameFailure]")]
pub fn stub_0x24a58(state: &mut PlaceLauncherState, controller: Option<u32>) {
    // IDA 0x24a58: forwards `handleStartGameFailure` to the
    // `MainViewController getLastNonGameController` when present,
    // then clears `isCurrentlyPlayingGame`.
    if controller.is_some() {
        state.failure_forwarded = true;
    }
    state.currently_playing = false;
}

// 0x24ab0 — -[PlaceLauncher prepareGame]
// type: bool __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher prepareGame]")]
pub fn stub_0x24ab0(state: &mut PlaceLauncherState, reachability: i32, wifi_only: bool, content_path: &str) -> bool {
    // IDA 0x24ab0 `-[PlaceLauncher prepareGame]`: asset folder +
    // `globalInit` + teleport base URL; reachability 2 (WiFi) with the
    // `wifionly_preference` set alerts `WiFiOnlyError`; reachability 0
    // logs `No Network Connection` and alerts `ConnectionError`;
    // otherwise stamps `DataModel::hash = "ios,ios"`, loads the basic
    // settings, sizes the task-scheduler pool, and returns true
    // (`NSString stringForKey/boolValue` resolve platform-side).
    state.asset_folder = Some(format!("{content_path}/content"));
    state.game_global_init = true;
    state.teleport_base_url_set = true;
    if reachability == 2 {
        if wifi_only {
            state.last_alert = Some("WiFiOnlyError".to_string());
            return false;
        }
    } else if reachability == 0 {
        state.last_log = Some("PlaceLauncher: No Network Connection available".to_string());
        state.last_alert = Some("ConnectionError".to_string());
        return false;
    }
    state.datamodel_hash = Some("ios,ios".to_string());
    state.basic_settings_loaded = true;
    state.scheduler_threads_set = true;
    true
}

// 0x25080 — -[PlaceLauncher setLastPlaceId:]
// type: void __cdecl(PlaceLauncher *self, SEL, int)
#[doc(alias = "-[PlaceLauncher setLastPlaceId:]")]
pub fn stub_0x25080(state: &mut PlaceLauncherState, place_id: i32) {
    // IDA 0x25080: `lastPlaceId` IVAR store.
    state.last_place_id = place_id;
}

// 0x25090 — -[PlaceLauncher checkPlacePartCount]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher checkPlacePartCount]")]
pub fn stub_0x25090(state: &mut PlaceLauncherState, warnings_enabled: bool, max_parts: i32, part_count: Option<i32>) {
    // IDA 0x25090: when the `warnings_preference` bool is set,
    // `dispatch_async`s the part-count block on the global queue
    // (synchronous here; `stringForKey/boolValue` resolve
    // platform-side).
    if warnings_enabled {
        stub_0x2512c(state, max_parts, part_count);
    }
}

// 0x2512c — ___36-[PlaceLauncher checkPlacePartCount]_block_invoke
#[doc(alias = "___36-[PlaceLauncher checkPlacePartCount]_block_invoke")]
pub fn stub_0x2512c(state: &mut PlaceLauncherState, max_parts: i32, part_count: Option<i32>) {
    // IDA 0x2512c: reads the settings-service ideal-parts value
    // (`+15`); below 1, or a null game/datamodel link, warns nothing;
    // more parts than ideal alerts `WarnPlaceIsNotIdeal` /
    // `WarnTooManyParts(partCount, maxParts)` and tracks
    // `PlayErrors`/`TooManyParts` labeled with `lastPlaceId`.
    if max_parts < 1 {
        return;
    }
    let Some(parts) = part_count else {
        return;
    };
    if max_parts < parts {
        state.part_warning = Some(PartWarning { max_parts, part_count: parts });
        state.ga_event = Some(GaEvent {
            category: "PlayErrors".to_string(),
            action: "TooManyParts".to_string(),
            label: state.last_place_id,
        });
        state.last_alert = Some("WarnPlaceIsNotIdeal/WarnTooManyParts".to_string());
    }
}

// 0x253cc — ___copy_helper_block_98
#[doc(alias = "___copy_helper_block_98")]
pub fn stub_0x253cc(dst: &mut BlockCapture, src: &BlockCapture) {
    // IDA 0x253cc `__copy_helper_block_98`: single
    // `_Block_object_assign` retain (cf. 0x1f660).
    *dst = src.clone();
}

// 0x253d8 — ___destroy_helper_block_99
#[doc(alias = "___destroy_helper_block_99")]
pub fn stub_0x253d8(slot: &mut BlockCapture) {
    // IDA 0x253d8 `__destroy_helper_block_99`: single
    // `_Block_object_dispose` release (cf. 0x1f4a0).
    *slot = BlockCapture::default();
}

// 0x253e0 — -[PlaceLauncher placeDidFinishLoading]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher placeDidFinishLoading]")]
pub fn stub_0x253e0(state: &mut PlaceLauncherState, warnings_enabled: bool, max_parts: i32, part_count: Option<i32>) {
    // IDA 0x253e0: posts `gameFinishedLoadingNotification` on the
    // default center, then runs `checkPlacePartCount`.
    state.finished_notification = Some(state.game_finished_loading_notification.clone());
    state.finished_notification_posted = true;
    stub_0x25090(state, warnings_enabled, max_parts, part_count);
}

// 0x25440 — -[PlaceLauncher deleteRobloxView]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher deleteRobloxView]")]
pub fn stub_0x25440(state: &mut PlaceLauncherState) {
    // IDA 0x25440: nils `rbxView`, runs the `RobloxView` dtor, and
    // stops the free-memory checker (dtor folds into host ownership).
    if state.rbx_view.take().is_some() {
        state.memory_checker_stopped = true;
    }
}

// 0x25498 — -[PlaceLauncher finishGameSetup:gameViewController:]
// type: void __cdecl(PlaceLauncher *self, SEL, shared_ptr<RBX::Game>, id)
#[doc(alias = "-[PlaceLauncher finishGameSetup:gameViewController:]")]
pub fn stub_0x25498(
    state: &mut PlaceLauncherState,
    game: &GameToken,
    view_token: u32,
    screen_size: Option<[u32; 2]>,
    place_finished: bool,
    has_overlay_datamodel: bool,
    create_view: &mut dyn FnMut(&GameToken, u32, Option<[u32; 2]>) -> u32,
    finish_now: &mut dyn FnMut(&mut PlaceLauncherState),
    defer_finish: &mut dyn FnMut(),
    setup_connections: &mut dyn FnMut(bool),
) {
    // IDA 0x25498: stringstreams the window/view ids, sizes from
    // `mainScreen bounds` (zeroed when headless), then
    // `RobloxView::create_view` into `rbxView`. When the datamodel link
    // (`+3108`) is already finished it runs `placeDidFinishLoading`
    // immediately, else connects it to the finish signal; then wires the
    // main datamodel and, when present, the overlay datamodel.
    state.rbx_view = Some(create_view(game, view_token, screen_size));
    if place_finished {
        finish_now(state);
    } else {
        defer_finish();
    }
    setup_connections(false);
    if has_overlay_datamodel {
        setup_connections(true);
    }
}

// 0x25e00 — -[PlaceLauncher setupDatamodelConnections:]
// type: void __cdecl(PlaceLauncher *self, SEL, shared_ptr<RBX::DataModel>)
#[doc(alias = "-[PlaceLauncher setupDatamodelConnections:]")]
pub fn stub_0x25e00(
    gui_present: bool,
    login_created: bool,
    connect_open_url: &mut dyn FnMut(),
    dispatch_main: &mut dyn FnMut(),
    connect_child_added: &mut dyn FnMut(),
    connect_login_prompt: &mut dyn FnMut(),
) {
    // IDA 0x25e00: with a `GuiService`, connects the ogre controller's
    // `openUrlWindow:` to its open-URL signal; `dispatch_async`s the main
    // block; connects `childAdded:` on the players link; with a fresh
    // `LoginService`, connects `handlePromptLoginSignal`.
    if gui_present {
        connect_open_url();
    }
    dispatch_main();
    connect_child_added();
    if login_created {
        connect_login_prompt();
    }
}

// 0x2613c — ___43-[PlaceLauncher setupDatamodelConnections:]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___43-[PlaceLauncher setupDatamodelConnections:]_block_invoke")]
pub fn stub_0x2613c(start_checker: &mut dyn FnMut()) {
    // IDA 0x2613c: `RobloxMemoryManager startFreeMemoryChecker`.
    start_checker();
}

// 0x26170 — -[PlaceLauncher setLastNonGameController:]
// type: void __cdecl(PlaceLauncher *self, SEL, id)
#[doc(alias = "-[PlaceLauncher setLastNonGameController:]")]
pub fn stub_0x26170(
    state: &mut PlaceLauncherState,
    controller: Option<u32>,
    prepare_game: &mut dyn FnMut() -> bool,
    set_platform_controller: &mut dyn FnMut(Option<u32>),
) {
    // IDA 0x26170: `MainViewController setLastNonGameController:` first;
    // with a non-null controller, `prepareGame` (0x24ab0) runs and failure
    // routes to `handleStartGameFailure` (0x24a58, `failure_forwarded` +
    // playing cleared — mirrored here through the shared latches).
    set_platform_controller(controller);
    state.last_non_game_controller = controller;
    if controller.is_some() && !prepare_game() {
        state.failure_forwarded = true;
        state.currently_playing = false;
    }
}

// 0x261d8 — -[PlaceLauncher createGame:presentGameAutomatically:]
// type: void __cdecl(PlaceLauncher *self, SEL, shared_ptr<RBX::Game>, char)
#[doc(alias = "-[PlaceLauncher createGame:presentGameAutomatically:]")]
pub fn stub_0x261d8(
    state: &mut PlaceLauncherState,
    game: &GameToken,
    present_automatically: bool,
    has_last_non_game_controller: bool,
    alloc_controllers: &mut dyn FnMut(),
    finish_setup: &mut dyn FnMut(&mut PlaceLauncherState, &GameToken),
    submit_control_view: &mut dyn FnMut(),
) {
    // IDA 0x261d8: clears `hasReceivedMemoryWarning`, `deleteRobloxView`
    // (0x25440); with a last non-game controller it allocs/inits the
    // `GameViewController`, installs it as the ogre controller, runs
    // `finishGameSetup:gameViewController:`, then `GetWindow` +
    // `DataModel::submitTask` with the `initControlView` bind. The
    // `presentGameAutomatically` flag has no observable use in the body.
    let _ = present_automatically;
    state.memory_warning = false;
    stub_0x25440(state);
    if has_last_non_game_controller {
        alloc_controllers();
        finish_setup(state, game);
        submit_control_view();
    }
}

// 0x2643c — __ZL15initControlViewP10RobloxViewaPN3RBX18FunctionMarshallerE
// type: _DWORD __fastcall(RobloxView *, signed __int8, RBX::FunctionMarshaller *)
#[doc(alias = "initControlView(RobloxView *,signed char,RBX::FunctionMarshaller *)")]
pub fn stub_0x2643c(view: u32, flag: bool, execute: &mut dyn FnMut(u32, bool)) {
    // IDA 0x2643c: binds `initControlViewHelper(view, flag)`
    // (`boost::bind` → closure) into `FunctionMarshaller::Execute`, then
    // clears the functor.
    execute(view, flag);
}

// 0x26520 — -[PlaceLauncher setupGame:isApp:]
// type: shared_ptr<RBX::Game> *__cdecl(shared_ptr<RBX::Game> *__return_ptr __struct_ptr retstr, PlaceLauncher *self, SEL, id, char)
#[doc(alias = "-[PlaceLauncher setupGame:isApp:]")]
pub fn stub_0x26520(
    state: &mut PlaceLauncherState,
    launcher_present: bool,
    controller: Option<u32>,
    is_app: bool,
    init_settings: &mut dyn FnMut(),
    set_idle_timer: &mut dyn FnMut(bool),
    set_controller: &mut dyn FnMut(Option<u32>),
    prepare_game: &mut dyn FnMut() -> bool,
) -> Option<GameToken> {
    // IDA 0x26520: nil self returns the null game; otherwise forwards to
    // `setupGame:unsecuredGame:isApp:` with `unsecured = 0`.
    if !launcher_present {
        return None;
    }
    stub_0x26558(state, controller, false, is_app, init_settings, set_idle_timer, set_controller, prepare_game)
}

// 0x26558 — -[PlaceLauncher setupGame:unsecuredGame:isApp:]
// type: shared_ptr<RBX::Game> *__cdecl(shared_ptr<RBX::Game> *__return_ptr __struct_ptr retstr, PlaceLauncher *self, SEL, id, char, char)
#[doc(alias = "-[PlaceLauncher setupGame:unsecuredGame:isApp:]")]
pub fn stub_0x26558(
    state: &mut PlaceLauncherState,
    controller: Option<u32>,
    unsecured: bool,
    is_app: bool,
    init_settings: &mut dyn FnMut(),
    set_idle_timer: &mut dyn FnMut(bool),
    set_controller: &mut dyn FnMut(Option<u32>),
    prepare_game: &mut dyn FnMut() -> bool,
) -> Option<GameToken> {
    // IDA 0x26558: with a game already playing, returns null; otherwise
    // initializes `ClientAppSettings`, fetches `iOSAppSettings`, refreshes
    // the settings service, disables the idle timer, latches
    // `isCurrentlyPlayingGame`, records the non-game controller
    // (`setLastNonGameController:`, 0x26170), and builds the
    // `UnsecuredStudioGame`/`SecurePlayerGame` (`isApp` normalized to 0/1).
    if state.currently_playing {
        return None;
    }
    init_settings();
    set_idle_timer(true);
    state.currently_playing = true;
    stub_0x26170(state, controller, prepare_game, set_controller);
    Some(GameToken { unsecured, is_app })
}

// 0x26768 — -[PlaceLauncher presentGameViewController]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher presentGameViewController]")]
pub fn stub_0x26768(dispatch_main: &mut dyn FnMut()) {
    // IDA 0x26768: `dispatch_async`s the presentation block on the main
    // queue (synchronous here).
    dispatch_main();
}

// 0x26784 — -[PlaceLauncher setupPreloadedGameWithNonGameController:unsecuredGame:isApp:]
// type: shared_ptr<RBX::Game> *__cdecl(shared_ptr<RBX::Game> *__return_ptr __struct_ptr retstr, PlaceLauncher *self, SEL, id, char, char)
#[doc(alias = "-[PlaceLauncher setupPreloadedGameWithNonGameController:unsecuredGame:isApp:]")]
pub fn stub_0x26784(
    state: &mut PlaceLauncherState,
    launcher_present: bool,
    controller: Option<u32>,
    unsecured: bool,
    is_app: bool,
    init_settings: &mut dyn FnMut(),
    set_idle_timer: &mut dyn FnMut(bool),
    set_controller: &mut dyn FnMut(Option<u32>),
    prepare_game: &mut dyn FnMut() -> bool,
) -> Option<GameToken> {
    // IDA 0x26784: nil self returns the null game; otherwise forwards to
    // `setupGame:unsecuredGame:isApp:`.
    if !launcher_present {
        return None;
    }
    stub_0x26558(state, controller, unsecured, is_app, init_settings, set_idle_timer, set_controller, prepare_game)
}

// 0x267bc — -[PlaceLauncher setupPreloadedGameWithNonGameController:isApp:]
// type: shared_ptr<RBX::Game> *__cdecl(shared_ptr<RBX::Game> *__return_ptr __struct_ptr retstr, PlaceLauncher *self, SEL, id, char)
#[doc(alias = "-[PlaceLauncher setupPreloadedGameWithNonGameController:isApp:]")]
pub fn stub_0x267bc(
    state: &mut PlaceLauncherState,
    launcher_present: bool,
    controller: Option<u32>,
    is_app: bool,
    init_settings: &mut dyn FnMut(),
    set_idle_timer: &mut dyn FnMut(bool),
    set_controller: &mut dyn FnMut(Option<u32>),
    prepare_game: &mut dyn FnMut() -> bool,
) -> Option<GameToken> {
    // IDA 0x267bc: nil self returns the null game; otherwise forwards to
    // `setupGame:isApp:` (0x26520).
    if !launcher_present {
        return None;
    }
    stub_0x26520(state, true, controller, is_app, init_settings, set_idle_timer, set_controller, prepare_game)
}

// 0x26bb8 — -[PlaceLauncher startGameLocal:ipAddress:controller:presentGameAutomatically:]
// type: char __cdecl(PlaceLauncher *self, SEL, int, id, id, char)
#[doc(alias = "-[PlaceLauncher startGameLocal:ipAddress:controller:presentGameAutomatically:]")]
pub fn stub_0x26bb8(
    state: &mut PlaceLauncherState,
    launcher_present: bool,
    port: i32,
    ip_address: &str,
    controller: Option<u32>,
    present_automatically: bool,
    setup_game: &mut dyn FnMut(&mut PlaceLauncherState, Option<u32>) -> Option<GameToken>,
    join_script: &mut dyn FnMut(i32, &str, &GameToken),
    start_preloaded: &mut dyn FnMut(&mut dyn FnMut(), Option<u32>, &GameToken, bool) -> bool,
) -> bool {
    // IDA 0x26bb8: nil self returns 0; otherwise
    // `setupPreloadedGameWithNonGameController:unsecuredGame:isApp:`
    // (unsecured path), and with a game, binds `joinLocalGame(port, ip,
    // game)` (`boost::bind` → `join_script` closure) into
    // `startGame:controller:preloadedGame:presentGameAutomatically:`.
    // `a3` is the `serverPort` consumed by `joinLocalGame` (IDA 0x26dd4).
    if !launcher_present {
        return false;
    }
    let Some(game) = setup_game(state, controller) else {
        return false;
    };
    let mut script = || join_script(port, ip_address, &game);
    start_preloaded(&mut script, controller, &game, present_automatically)
}

// 0x26dd4 — __ZL13joinLocalGameiRKSsN5boost10shared_ptrIN3RBX4GameEEE
#[doc(alias = "joinLocalGame(int,std::string const&,rbx_core::SharedPtr<RBX::Game>)")]
pub fn stub_0x26dd4(
    port: i32,
    server: &str,
    game: &GameToken,
    base_url: &str,
    execute_url_script: &mut dyn FnMut(&str, &GameToken),
) {
    // IDA 0x26dd4: formats
    // `"%sGame/Join.ashx?userID=0&serverPort=%i&server=%s"` from
    // `RobloxInfo getBaseUrl` and runs `executeUrlScript`.
    execute_url_script(
        &format!("{base_url}Game/Join.ashx?userID=0&serverPort={port}&server={server}"),
        game,
    );
}

// 0x27054 — -[PlaceLauncher startAppWithFile:controller:presentGameAutomatically:]
// type: char __cdecl(PlaceLauncher *self, SEL, id, id, char)
#[doc(alias = "-[PlaceLauncher startAppWithFile:controller:presentGameAutomatically:]")]
pub fn stub_0x27054(
    state: &mut PlaceLauncherState,
    launcher_present: bool,
    file_path: &str,
    controller: Option<u32>,
    present_automatically: bool,
    setup_game: &mut dyn FnMut(&mut PlaceLauncherState, Option<u32>) -> Option<GameToken>,
    load_script: &mut dyn FnMut(&str, &GameToken),
    start_preloaded: &mut dyn FnMut(&mut dyn FnMut(), Option<u32>, &GameToken, bool) -> bool,
) -> bool {
    // IDA 0x27054: nil self returns 0; otherwise the preloaded setup, and
    // with a game, binds `loadLocalApp(path, game)` into
    // `startGame:controller:preloadedGame:presentGameAutomatically:`.
    if !launcher_present {
        return false;
    }
    let Some(game) = setup_game(state, controller) else {
        return false;
    };
    let mut script = || load_script(file_path, &game);
    start_preloaded(&mut script, controller, &game, present_automatically)
}

// 0x27268 — __ZL12loadLocalAppRKSsN5boost10shared_ptrIN3RBX4GameEEE
#[doc(alias = "loadLocalApp(std::string const&,rbx_core::SharedPtr<RBX::Game>)")]
pub fn stub_0x27268(
    path: &str,
    game: &GameToken,
    has_datamodel: bool,
    local_player_id: Option<i32>,
    execute_script: &mut dyn FnMut(&str, &GameToken),
    create_local_player: &mut dyn FnMut(i32),
) {
    // IDA 0x27268: `Game:Load('rbxasset://%s')` runs `executeScript` while
    // the datamodel link is live; with a live players link it enters
    // impersonation level 7, creates `Network::Players`, resolves
    // `CurrentPlayer userinfo intValue`, and creates the local player
    // (the `Security::Context` reset folds into host ownership).
    if has_datamodel {
        execute_script(&format!("Game:Load('rbxasset://{path}')"), game);
    }
    if let Some(player_id) = local_player_id {
        create_local_player(player_id);
    }
}

// 0x276b0 — -[PlaceLauncher startAppWithId:controller:presentGameAutomatically:]
// type: char __cdecl(PlaceLauncher *self, SEL, int, id, char)
#[doc(alias = "-[PlaceLauncher startAppWithId:controller:presentGameAutomatically:]")]
pub fn stub_0x276b0(
    state: &mut PlaceLauncherState,
    launcher_present: bool,
    place_id: i32,
    controller: Option<u32>,
    present_automatically: bool,
    setup_game: &mut dyn FnMut(&mut PlaceLauncherState, Option<u32>) -> Option<GameToken>,
    join_script: &mut dyn FnMut(i32, &GameToken, i32),
    start_preloaded: &mut dyn FnMut(&mut dyn FnMut(), Option<u32>, &GameToken, bool) -> bool,
) -> bool {
    // IDA 0x276b0: nil self returns 0; otherwise
    // `setupPreloadedGameWithNonGameController:isApp:` (`isApp = 1`), and
    // with a game, binds `joinGamePlaceId(placeId, game, request = 2)`
    // into `startGame:controller:preloadedGame:presentGameAutomatically:`.
    if !launcher_present {
        return false;
    }
    let Some(game) = setup_game(state, controller) else {
        return false;
    };
    let mut script = || join_script(place_id, &game, 2);
    start_preloaded(&mut script, controller, &game, present_automatically)
}

/// `joinScriptUrl` extraction (IDA 0x27e24..0x27ef6): the value after
/// `"joinScriptUrl"` (+ key length + 3) up to the next comma (closing
/// quote stripped), with `\/` escapes collapsed (the discarded
/// comma-find result + single-pos `substr` in the decompile imply
/// comma truncation).
fn join_script_url(response: &str) -> String {
    let key = "joinScriptUrl";
    let Some(pos) = response.find(key) else {
        return String::new();
    };
    let start = pos + key.len() + 3;
    if start >= response.len() {
        return String::new();
    }
    let end = response[start..].find(',').map(|i| start + i).unwrap_or(response.len());
    let raw = &response[start..end];
    raw.strip_suffix('"').unwrap_or(raw).replace("\\/", "/")
}

// 0x278a8 — __ZL15joinGamePlaceIdiN5boost10shared_ptrIN3RBX4GameEEE15JoinGameRequest
#[doc(alias = "joinGamePlaceId(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest)")]
pub fn stub_0x278a8(
    state: &mut PlaceLauncherState,
    place_id: i32,
    game: &GameToken,
    request: i32,
    overlay_enabled: bool,
    base_url: &str,
    register_user_agent: &mut dyn FnMut(),
    http_get: &mut dyn FnMut(&str) -> String,
    sleep_us: &mut dyn FnMut(u32),
    execute_signed_script: &mut dyn FnMut(&GameToken, &str),
    execute_url_script: &mut dyn FnMut(&GameToken, &str),
    report_session: &mut dyn FnMut(i32, i32),
    track_page_view: &mut dyn FnMut(&str),
    leave_game: &mut dyn FnMut(&mut PlaceLauncherState),
    handle_failure: &mut dyn FnMut(&mut PlaceLauncherState),
) {
    // IDA 0x278a8: logs the join, registers the `UserAgent` default, then
    // with `OverlayDataModelEnabled` + `request == 2` fetches
    // `Game/AppStart.ashx?appid=` and runs the signed script; otherwise
    // polls `Game/PlaceLauncher.ashx` (`placeId/RequestGame` for request
    // 0, `userId/RequestFollowUser` for 1) until `"status":2` — transient
    // 0/1 statuses sleep 0x3D090us without consuming retries, other
    // statuses sleep 0xF3E58us and consume one of 5 retries (disasm
    // 0x27d18/0x27d22 `MOV R0` immediates) — then runs the `joinScriptUrl`
    // (`\/` unescaped) and records the join. Exhaustion alerts
    // `ConnectionErrorGameEnded` (status 5), `ConnectionErrorGameFull`
    // (status 6), else `ConnectionError`, then leaves and forwards the
    // failure. String retains/releases fold into host ownership.
    state.last_log = Some(format!("PlaceLauncher::joinGamePlaceId {place_id}"));
    register_user_agent();
    let (key, req) = match request {
        0 => ("placeId", "RequestGame"),
        1 => ("userId", "RequestFollowUser"),
        _ => ("", ""),
    };
    if overlay_enabled && request == 2 {
        let response = http_get(&format!("{base_url}Game/AppStart.ashx?appid={place_id}"));
        execute_signed_script(game, &response);
    } else {
        let url = format!(
            "{base_url}Game/PlaceLauncher.ashx?request={req}&{key}={place_id}&isPartyLeader=false&gender=&isTeleport=false"
        );
        let mut retries = 5;
        let mut response = String::new();
        loop {
            if retries < 0 {
                state.last_log = Some(if request != 0 {
                    format!("PlaceLauncher: Cannot follow user {place_id}, return = {response}")
                } else {
                    format!("PlaceLauncher: Cannot connect to place {place_id}, return = {response}")
                });
                state.last_alert = Some(
                    if response.contains("\"status\":5") {
                        "ConnectionErrorGameEnded"
                    } else if response.contains("\"status\":6") {
                        "ConnectionErrorGameFull"
                    } else {
                        "ConnectionError"
                    }
                    .to_string(),
                );
                leave_game(state);
                handle_failure(state);
                return;
            }
            response = http_get(&url);
            if response.contains("\"status\":2") {
                break;
            }
            let transient = response.contains("\"status\":0") || response.contains("\"status\":1");
            if transient {
                sleep_us(0x3d090);
            } else {
                retries -= 1;
                sleep_us(0xf3e58);
            }
        }
        if request == 2 {
            execute_signed_script(game, &response);
        } else {
            execute_url_script(game, &join_script_url(&response));
        }
    }
    stub_0x25080(state, place_id);
    report_session(3, place_id);
    track_page_view("Visit/Success/Join");
}

// 0x289a8 — -[PlaceLauncher startGame:controller:request:presentGameAutomatically:]
// type: char __cdecl(PlaceLauncher *self, SEL, int, id, int, char)
#[doc(alias = "-[PlaceLauncher startGame:controller:request:presentGameAutomatically:]")]
pub fn stub_0x289a8(
    state: &mut PlaceLauncherState,
    launcher_present: bool,
    place_id: i32,
    controller: Option<u32>,
    request: i32,
    present_automatically: bool,
    setup_game: &mut dyn FnMut(&mut PlaceLauncherState, Option<u32>) -> Option<GameToken>,
    join_script: &mut dyn FnMut(i32, &GameToken, i32),
    start_preloaded: &mut dyn FnMut(&mut dyn FnMut(), Option<u32>, &GameToken, bool) -> bool,
) -> bool {
    // IDA 0x289a8: nil self returns 0; otherwise
    // `setupPreloadedGameWithNonGameController:isApp:` (`isApp` is
    // `request == 2`), and with a game, binds
    // `joinGamePlaceId(placeId, game, request)` into
    // `startGame:controller:preloadedGame:presentGameAutomatically:`.
    if !launcher_present {
        return false;
    }
    let Some(game) = setup_game(state, controller) else {
        return false;
    };
    let mut script = || join_script(place_id, &game, request);
    start_preloaded(&mut script, controller, &game, present_automatically)
}

// 0x28ba8 — -[PlaceLauncher startGameSolo:controller:presentGameAutomatically:]
// type: char __cdecl(PlaceLauncher *self, SEL, int, id, char)
#[doc(alias = "-[PlaceLauncher startGameSolo:controller:presentGameAutomatically:]")]
pub fn stub_0x28ba8(
    state: &mut PlaceLauncherState,
    launcher_present: bool,
    place_id: i32,
    controller: Option<u32>,
    present_automatically: bool,
    setup_game: &mut dyn FnMut(&mut PlaceLauncherState, Option<u32>) -> Option<GameToken>,
    join_script: &mut dyn FnMut(i32, &GameToken),
    start_preloaded: &mut dyn FnMut(&mut dyn FnMut(), Option<u32>, &GameToken, bool) -> bool,
) -> bool {
    // IDA 0x28ba8: nil self returns 0; otherwise the preloaded setup, and
    // with a game, binds `joinGamePlaceIdSolo(placeId, game)` into
    // `startGame:controller:preloadedGame:presentGameAutomatically:`.
    if !launcher_present {
        return false;
    }
    let Some(game) = setup_game(state, controller) else {
        return false;
    };
    let mut script = || join_script(place_id, &game);
    start_preloaded(&mut script, controller, &game, present_automatically)
}

// 0x28d98 — __ZL19joinGamePlaceIdSoloiN5boost10shared_ptrIN3RBX4GameEEE
#[doc(alias = "joinGamePlaceIdSolo(int,rbx_core::SharedPtr<RBX::Game>)")]
pub fn stub_0x28d98(
    place_id: i32,
    game: &GameToken,
    base_url: &str,
    register_user_agent: &mut dyn FnMut(),
    execute_script: &mut dyn FnMut(&str, &GameToken),
    set_last_place_id: &mut dyn FnMut(i32),
    track_page: &mut dyn FnMut(&str),
) {
    // IDA 0x28d98: registers the `UserAgent` default, then for
    // `placeId < 1` runs
    // `game:Load('rbxasset://places/workshop/workshopStartPlace.rbxl')
    // loadfile('%sgame/visit.ashx')()`, else
    // `loadfile('%sgame/visit.ashx?placeid=%d')()`; then records the place
    // id (`setLastPlaceId:`, 0x25080) and tracks `VisitSolo/Success/Join`.
    register_user_agent();
    if place_id < 1 {
        execute_script(
            &format!("game:Load('rbxasset://places/workshop/workshopStartPlace.rbxl') loadfile('{base_url}game/visit.ashx')()"),
            game,
        );
    } else {
        execute_script(
            &format!("loadfile('{base_url}game/visit.ashx?placeid={place_id}')()"),
            game,
        );
    }
    set_last_place_id(place_id);
    track_page("VisitSolo/Success/Join");
}

// 0x29490 — -[PlaceLauncher startGame:controller:preloadedGame:presentGameAutomatically:]
// type: char __cdecl(PlaceLauncher *self, SEL, function0<void>, id, shared_ptr<RBX::Game>, char)
#[doc(alias = "-[PlaceLauncher startGame:controller:preloadedGame:presentGameAutomatically:]")]
pub fn stub_0x29490(
    start_script: &mut dyn FnMut(),
    controller: Option<u32>,
    game: &GameToken,
    present_automatically: bool,
    spawn_thread: &mut dyn FnMut(&str, &mut dyn FnMut()),
    create_game: &mut dyn FnMut(&GameToken, bool),
) -> bool {
    // IDA 0x29490: wraps the join closure (`thread_wrapper`,
    // `boost::thread` → `spawn_thread`, detached) as `GameStartScript`,
    // then `createGame:presentGameAutomatically:`; always returns 1.
    // The controller rides the thread cookie; the host keeps it explicit.
    let _ = controller;
    spawn_thread("GameStartScript", start_script);
    create_game(game, present_automatically);
    true
}

// 0x295c0 — -[PlaceLauncher leaveGameShutdown]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher leaveGameShutdown]")]
pub fn stub_0x295c0(
    state: &mut PlaceLauncherState,
    dismiss: &mut dyn FnMut(&mut dyn FnMut()),
    completion: &mut dyn FnMut(),
) {
    // IDA 0x295c0: posts `startLeaveGameNotification` on the default
    // center, then dismisses the ogre controller with the
    // `leaveGameShutdown` completion block (0x29684, supplied by the caller
    // and invoked by `dismiss` on completion).
    state.posted_notification = Some(state.start_leave_game_notification.clone());
    dismiss(completion);
}

// 0x29684 — ___34-[PlaceLauncher leaveGameShutdown]_block_invoke
#[doc(alias = "___34-[PlaceLauncher leaveGameShutdown]_block_invoke")]
pub fn stub_0x29684(
    state: &mut PlaceLauncherState,
    release_controllers: &mut dyn FnMut(),
    post_notification: &mut dyn FnMut(&str),
    clear_game_state: &mut dyn FnMut(),
    end_background_task: &mut dyn FnMut(),
) {
    // IDA 0x29684: releases the ogre controller/view/window,
    // `deleteRobloxView` (0x25440), clears the memory/playing latches
    // (`hasReceivedMemoryWarning`/`isCurrentlyPlayingGame`, 0x2971c/0x29738),
    // posts the did-leave notification (`didLeaveGameNotification`),
    // logs, drops the `RobloxGameState` default (+ synchronize), clears
    // `isLeavingGame` (0x297e8; disasm `STRB` via the `isLeavingGame`
    // IVAR ref — not the teleport callback), and ends the delegate
    // background task.
    release_controllers();
    stub_0x25440(state);
    state.currently_playing = false;
    state.memory_warning = false;
    let notification = state.did_leave_game_notification.clone();
    post_notification(&notification);
    state.posted_notification = Some(notification);
    clear_game_state();
    state.is_leaving_game = false;
    end_background_task();
}

// 0x298a0 — ___copy_helper_block_191
#[doc(alias = "___copy_helper_block_191")]
pub fn stub_0x298a0(dst: &mut BlockCapture, src: &BlockCapture) {
    // IDA 0x298a0 `__copy_helper_block_191`: two `_Block_object_assign`
    // retains (+20/+24, cf. 0x1f660); the host retains the whole capture.
    *dst = src.clone();
}

// 0x298c4 — ___destroy_helper_block_192
#[doc(alias = "___destroy_helper_block_192")]
pub fn stub_0x298c4(slot: &mut BlockCapture) {
    // IDA 0x298c4 `__destroy_helper_block_192`: two
    // `_Block_object_dispose` releases (+20/+24, cf. 0x1f4a0).
    *slot = BlockCapture::default();
}
#[cfg(test)]
mod launcher_setup_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    fn setup_callbacks() -> (
        impl FnMut(),
        impl FnMut(bool),
        impl FnMut(Option<u32>),
        impl FnMut() -> bool,
    ) {
        let init = || {};
        let idle = |_: bool| {};
        let controller = |_: Option<u32>| {};
        let prepare = || true;
        (init, idle, controller, prepare)
    }

    #[test]
    fn setup_game_builds_token_and_latches() {
        let mut state = PlaceLauncherState::default();
        let (mut init, mut idle, mut set_controller, mut prepare) = setup_callbacks();
        let token = stub_0x26558(&mut state, Some(3), true, true, &mut init, &mut idle, &mut set_controller, &mut prepare);
        assert_eq!(token, Some(GameToken { unsecured: true, is_app: true }));
        assert!(state.currently_playing);
        assert_eq!(state.last_non_game_controller, Some(3));
        // Playing again returns null without touching anything.
        let (mut init2, mut idle2, mut set2, mut prep2) = setup_callbacks();
        assert_eq!(stub_0x26558(&mut state, Some(3), false, false, &mut init2, &mut idle2, &mut set2, &mut prep2), None);
        // Forwarders thread through; nil launcher returns null.
        let (mut a, mut b, mut c, mut d) = setup_callbacks();
        assert!(stub_0x26520(&mut state, false, None, false, &mut a, &mut b, &mut c, &mut d).is_none());
        assert!(stub_0x26784(&mut PlaceLauncherState::default(), false, None, false, false, &mut a, &mut b, &mut c, &mut d).is_none());
        assert!(stub_0x267bc(&mut PlaceLauncherState::default(), false, None, false, &mut a, &mut b, &mut c, &mut d).is_none());
    }

    #[test]
    fn failed_prepare_forwards_failure() {
        let mut state = PlaceLauncherState::default();
        state.currently_playing = true;
        let mut set = |_: Option<u32>| {};
        let mut prepare = || false;
        stub_0x26170(&mut state, Some(9), &mut prepare, &mut set);
        assert!(state.failure_forwarded);
        assert!(!state.currently_playing);
        assert_eq!(state.last_non_game_controller, Some(9));
    }

    #[test]
    fn start_variants_bind_and_launch() {
        let mut state = PlaceLauncherState::default();
        let token = GameToken { unsecured: true, is_app: false };
        let mut setup = |_: &mut PlaceLauncherState, _: Option<u32>| Some(token);
        let joined = RefCell::new(Vec::new());
        let mut join_local = |port: i32, ip: &str, game: &GameToken| {
            joined.borrow_mut().push((port, ip.to_owned(), *game));
        };
        let launched = RefCell::new(Vec::new());
        let mut start = |script: &mut dyn FnMut(), controller: Option<u32>, game: &GameToken, present: bool| {
            script();
            launched.borrow_mut().push((controller, *game, present));
            true
        };
        assert!(stub_0x26bb8(&mut state, true, 5362, "127.0.0.1", Some(4), true, &mut setup, &mut join_local, &mut start));
        assert_eq!(*joined.borrow(), [(5362, "127.0.0.1".to_owned(), token)]);
        assert_eq!(*launched.borrow(), [(Some(4), token, true)]);
        assert!(!stub_0x26bb8(&mut state, false, 5362, "127.0.0.1", Some(4), true, &mut setup, &mut join_local, &mut start));
        // Preloaded start spawns the GameStartScript thread then creates.
        let spawned = RefCell::new(Vec::new());
        let mut spawn = |name: &str, script: &mut dyn FnMut()| {
            spawned.borrow_mut().push(name.to_owned());
            script();
        };
        let created = RefCell::new(Vec::new());
        let mut create = |game: &GameToken, present: bool| created.borrow_mut().push((*game, present));
        let ran = Cell::new(0);
        let mut script = || ran.set(ran.get() + 1);
        assert!(stub_0x29490(&mut script, Some(4), &token, false, &mut spawn, &mut create));
        assert_eq!(*spawned.borrow(), ["GameStartScript"]);
        assert_eq!(ran.get(), 1);
        assert_eq!(*created.borrow(), [(token, false)]);
    }

    #[test]
    fn url_script_shapes_match_binary() {
        let token = GameToken::default();
        let seen = RefCell::new(Vec::new());
        let mut exec = |url: &str, _: &GameToken| seen.borrow_mut().push(url.to_owned());
        stub_0x26dd4(5362, "127.0.0.1", &token, "http://base/", &mut exec);
        assert_eq!(*seen.borrow(), ["http://base/Game/Join.ashx?userID=0&serverPort=5362&server=127.0.0.1"]);
        let loaded = RefCell::new(Vec::new());
        let mut load = |script: &str, _: &GameToken| loaded.borrow_mut().push(script.to_owned());
        let players = RefCell::new(Vec::new());
        let mut mk_player = |id: i32| players.borrow_mut().push(id);
        stub_0x27268("my place", &token, true, Some(42), &mut load, &mut mk_player);
        assert_eq!(*loaded.borrow(), ["Game:Load('rbxasset://my place')"]);
        assert_eq!(*players.borrow(), [42]);
        stub_0x27268("my place", &token, false, None, &mut load, &mut mk_player);
        assert_eq!(loaded.borrow().len(), 1);
        assert!(players.borrow().len() == 1);
    }

    #[test]
    fn solo_join_branches_on_place_id() {
        let token = GameToken::default();
        let scripts = RefCell::new(Vec::new());
        let mut exec = |script: &str, _: &GameToken| scripts.borrow_mut().push(script.to_owned());
        let last = RefCell::new(Vec::new());
        let mut set_id = |id: i32| last.borrow_mut().push(id);
        let pages = RefCell::new(Vec::new());
        let mut track = |page: &str| pages.borrow_mut().push(page.to_owned());
        let agent = Cell::new(0);
        let mut register = || agent.set(agent.get() + 1);
        stub_0x28d98(0, &token, "http://base/", &mut register, &mut exec, &mut set_id, &mut track);
        assert_eq!(*scripts.borrow(), ["game:Load('rbxasset://places/workshop/workshopStartPlace.rbxl') loadfile('http://base/game/visit.ashx')()"]);
        stub_0x28d98(1818, &token, "http://base/", &mut register, &mut exec, &mut set_id, &mut track);
        assert_eq!(scripts.borrow()[1], "loadfile('http://base/game/visit.ashx?placeid=1818')()");
        assert_eq!(*last.borrow(), [0, 1818]);
        assert_eq!(*pages.borrow(), ["VisitSolo/Success/Join", "VisitSolo/Success/Join"]);
        assert_eq!(agent.get(), 2);
    }

    #[test]
    fn finish_setup_creates_view_and_wires() {
        let mut state = PlaceLauncherState::default();
        let token = GameToken { unsecured: false, is_app: true };
        let created = RefCell::new(Vec::new());
        let mut create = |game: &GameToken, view: u32, size: Option<[u32; 2]>| {
            created.borrow_mut().push((*game, view, size));
            77u32
        };
        let finished = Cell::new(0);
        let mut finish_now = |_: &mut PlaceLauncherState| finished.set(finished.get() + 1);
        let deferred = Cell::new(0);
        let mut defer = || deferred.set(deferred.get() + 1);
        let wired = RefCell::new(Vec::new());
        let mut wire = |overlay: bool| wired.borrow_mut().push(overlay);
        stub_0x25498(&mut state, &token, 5, Some([320, 480]), true, true, &mut create, &mut finish_now, &mut defer, &mut wire);
        assert_eq!(state.rbx_view, Some(77));
        assert_eq!(finished.get(), 1);
        assert_eq!(deferred.get(), 0);
        assert_eq!(*wired.borrow(), [false, true]);
        assert_eq!(*created.borrow(), [(token, 5, Some([320, 480]))]);
    }

    #[test]
    fn create_game_gates_on_controller() {
        let mut state = PlaceLauncherState::default();
        state.rbx_view = Some(1);
        let token = GameToken::default();
        let steps = RefCell::new(Vec::new());
        let mut alloc = || steps.borrow_mut().push("alloc");
        let mut finish = |_: &mut PlaceLauncherState, _: &GameToken| steps.borrow_mut().push("finish");
        let mut submit = || steps.borrow_mut().push("submit");
        stub_0x261d8(&mut state, &token, true, false, &mut alloc, &mut finish, &mut submit);
        assert!(state.rbx_view.is_none());
        assert!(steps.borrow().is_empty());
        stub_0x261d8(&mut state, &token, true, true, &mut alloc, &mut finish, &mut submit);
        assert_eq!(*steps.borrow(), ["alloc", "finish", "submit"]);
    }

    #[test]
    fn shutdown_posts_and_tears_down() {
        let mut state = PlaceLauncherState::default();
        state.start_leave_game_notification = "START".to_string();
        state.did_leave_game_notification = "DID".to_string();
        state.currently_playing = true;
        state.teleport_callback_set = true;
        let dismissed = Cell::new(0);
        {
            let mut dismiss = |completion: &mut dyn FnMut()| {
                dismissed.set(dismissed.get() + 1);
                completion();
            };
            let mut completion = || {};
            stub_0x295c0(&mut state, &mut dismiss, &mut completion);
        }
        assert_eq!(dismissed.get(), 1);
        let released = Cell::new(0);
        let mut release = || released.set(released.get() + 1);
        let posted = RefCell::new(Vec::new());
        let mut post = |name: &str| posted.borrow_mut().push(name.to_owned());
        let cleared = Cell::new(0);
        let mut clear = || cleared.set(cleared.get() + 1);
        let bg = Cell::new(0);
        let mut end_bg = || bg.set(bg.get() + 1);
        stub_0x29684(&mut state, &mut release, &mut post, &mut clear, &mut end_bg);
        assert_eq!((released.get(), cleared.get(), bg.get()), (1, 1, 1));
        assert_eq!(*posted.borrow(), ["DID"]);
    }

    #[test]
    fn datamodel_connections_gate_each_signal() {
        let opened = Cell::new(0);
        let mut open = || opened.set(opened.get() + 1);
        let main = Cell::new(0);
        let mut dispatch = || main.set(main.get() + 1);
        let added = Cell::new(0);
        let mut child = || added.set(added.get() + 1);
        let login = Cell::new(0);
        let mut prompt = || login.set(login.get() + 1);
        stub_0x25e00(true, true, &mut open, &mut dispatch, &mut child, &mut prompt);
        assert_eq!((opened.get(), main.get(), added.get(), login.get()), (1, 1, 1, 1));
        stub_0x25e00(false, false, &mut open, &mut dispatch, &mut child, &mut prompt);
        assert_eq!((opened.get(), main.get(), added.get(), login.get()), (1, 2, 2, 1));
    }

    #[test]
    fn block_helpers_round_trip() {
        let view = Cell::new(0);
        let mut init_view = |v: u32, f: bool| view.set(if f { v } else { 0 });
        stub_0x2643c(9, true, &mut init_view);
        assert_eq!(view.get(), 9);
        let main = Cell::new(0);
        let mut dispatch = || main.set(main.get() + 1);
        stub_0x26768(&mut dispatch);
        let checks = Cell::new(0);
        let mut checker = || checks.set(checks.get() + 1);
        stub_0x2613c(&mut checker);
        assert_eq!((main.get(), checks.get()), (1, 1));
        let src = BlockCapture { target: Some(5) };
        let mut dst = BlockCapture::default();
        stub_0x298a0(&mut dst, &src);
        assert_eq!(dst.target, Some(5));
        stub_0x298c4(&mut dst);
        assert_eq!(dst.target, None);
    }
}

#[cfg(test)]
mod launcher_leave_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    fn armed() -> PlaceLauncherState {
        let mut state = PlaceLauncherState::default();
        state.currently_playing = true;
        state.child_connected = true;
        state.rbx_view = Some(7);
        state
    }
    #[test]
    fn leave_game_guards_and_latches() {
        let idle: RefCell<Vec<bool>> = RefCell::new(vec![]);
        let gs: RefCell<Vec<String>> = RefCell::new(vec![]);
        let sync = Cell::new(0);
        let sessions: RefCell<Vec<i32>> = RefCell::new(vec![]);
        let pages: RefCell<Vec<String>> = RefCell::new(vec![]);
        let bg_set: RefCell<Vec<u32>> = RefCell::new(vec![]);
        let bg_end: RefCell<Vec<u32>> = RefCell::new(vec![]);
        let dispatched = Cell::new(0);
        let shutdowns = Cell::new(0);
        let run = |present: bool, version: f32| {
            let mut state = armed();
            if !present {
                state.currently_playing = false;
            }
            let mut set_idle = |v: bool| idle.borrow_mut().push(v);
            let mut set_gs = |v: &str| gs.borrow_mut().push(v.to_owned());
            let mut sync_fn = || sync.set(sync.get() + 1);
            let mut report = |kind: i32| sessions.borrow_mut().push(kind);
            let mut track = |p: &str| pages.borrow_mut().push(p.to_owned());
            let mut begin = |_: &mut dyn FnMut(u32)| 11u32;
            let mut set_bg = |v: u32| bg_set.borrow_mut().push(v);
            let mut end_bg = |v: u32| bg_end.borrow_mut().push(v);
            let mut dispatch = |f: &mut dyn FnMut()| {
                dispatched.set(dispatched.get() + 1);
                f();
            };
            let mut shutdown = |_: &mut PlaceLauncherState| shutdowns.set(shutdowns.get() + 1);
            stub_0x298e0(
                &mut state, present, version, &mut set_idle, &mut set_gs, &mut sync_fn,
                &mut report, &mut track, &mut begin, &mut set_bg, &mut end_bg,
                &mut dispatch, &mut shutdown,
            );
            state
        };
        let idle_before = idle.borrow().len();
        let quiet = run(false, 7.0);
        assert!(!quiet.is_leaving_game);
        assert_eq!(idle.borrow().len(), idle_before);
        let leaving = run(true, 5.0);
        assert!(leaving.is_leaving_game);
        assert!(!leaving.child_connected);
        assert!(leaving.memory_checker_stopped);
        assert_eq!((shutdowns.get(), dispatched.get()), (1, 0));
        let modern = run(true, 7.0);
        assert!(modern.is_leaving_game);
        assert_eq!((shutdowns.get(), dispatched.get()), (2, 1));
        assert_eq!(gs.borrow().last().map(String::as_str), Some("leaveGame"));
        assert_eq!(pages.borrow().last().map(String::as_str), Some("Visit/Success/LeaveGame"));
        assert_eq!(sessions.borrow().last(), Some(&4));
        assert_eq!(bg_set.borrow().last(), Some(&11));
        let _ = bg_end;
    }
    #[test]
    fn leave_game_double_leave_noop() {
        let mut state = armed();
        state.is_leaving_game = true;
        let mut idle_calls = 0;
        let mut set_idle = |_: bool| idle_calls += 1;
        let mut set_gs = |_: &str| {};
        let mut sync_fn = || {};
        let mut report = |_: i32| {};
        let mut track = |_: &str| {};
        let mut begin = |_: &mut dyn FnMut(u32)| 0u32;
        let mut set_bg = |_: u32| {};
        let mut end_bg = |_: u32| {};
        let mut dispatch = |_: &mut dyn FnMut()| {};
        let mut shutdown = |_: &mut PlaceLauncherState| {};
        stub_0x298e0(
            &mut state, true, 7.0, &mut set_idle, &mut set_gs, &mut sync_fn,
            &mut report, &mut track, &mut begin, &mut set_bg, &mut end_bg,
            &mut dispatch, &mut shutdown,
        );
        assert_eq!(idle_calls, 0);
    }
    #[test]
    fn expiration_handler_clears_leaving_and_task() {
        let mut state = armed();
        state.is_leaving_game = true;
        let ended = Cell::new(0u32);
        let mut end_bg = |v: u32| ended.set(v);
        let reset = Cell::new(99u32);
        let mut set_bg = |v: u32| reset.set(v);
        stub_0x29bb4(&mut state, 11, &mut end_bg, &mut set_bg);
        assert!(!state.is_leaving_game);
        assert_eq!((ended.get(), reset.get()), (11, 0));
    }
    #[test]
    fn bg_fg_view_gates_on_rbx_view() {
        let live = armed();
        let calls = Cell::new(0);
        let mut stop = |v: u32| {
            assert_eq!(v, 7);
            calls.set(calls.get() + 1);
        };
        stub_0x29c9c(&live, &mut stop);
        stub_0x29cb4(&live, &mut stop);
        assert_eq!(calls.get(), 2);
        let mut dead = PlaceLauncherState::default();
        dead.rbx_view = None;
        stub_0x29c9c(&dead, &mut stop);
        stub_0x29cb4(&dead, &mut stop);
        assert_eq!(calls.get(), 2);
    }
    #[test]
    fn memory_warning_branches() {
        let mut out = armed();
        out.last_place_id = 42;
        let printed: RefCell<Vec<u32>> = RefCell::new(vec![]);
        let mut print = |v: u32| printed.borrow_mut().push(v);
        let reported: RefCell<Vec<(i32, i32)>> = RefCell::new(vec![]);
        let mut report = |kind: i32, place: i32| reported.borrow_mut().push((kind, place));
        let left = Cell::new(0);
        let mut leave = |_: &mut PlaceLauncherState| left.set(left.get() + 1);
        stub_0x2ae54(&mut out, 1024, true, &mut print, &mut report, &mut leave);
        assert_eq!(*reported.borrow(), [(5, 42)]);
        assert_eq!(out.ga_event.as_ref().map(|e| e.action.as_str()), Some("OutOfMemory_EarlyExit"));
        assert_eq!(out.last_alert.as_deref(), Some("MemoryError"));
        assert!(out.last_log.as_deref().unwrap().contains("in-game shutdown"));
        assert!(!out.child_connected && !out.player_connected);
        assert_eq!(left.get(), 1);
        let mut quiet = PlaceLauncherState::default();
        stub_0x2ae54(&mut quiet, 0, false, &mut print, &mut report, &mut leave);
        assert!(quiet.last_log.as_deref().unwrap().contains("ignoring"));
        assert_eq!(left.get(), 1);
        assert_eq!(stub_0x2ae44(&out), true);
        assert_eq!(stub_0x2ae44(&quiet), false);
    }
    #[test]
    fn child_and_player_connection_flow() {
        let mut state = armed();
        state.player_connected = true;
        let linked = Cell::new(0);
        let mut link = || linked.set(linked.get() + 1);
        stub_0x2b1bc(&mut state, true, true, true, &mut link);
        assert_eq!(linked.get(), 1);
        assert!(state.player_connected && !state.child_connected);
        stub_0x2b1bc(&mut state, false, false, false, &mut link);
        assert_eq!(linked.get(), 1);
        let mut loaded = armed();
        loaded.player_connected = true;
        loaded.child_connected = true;
        let stamped = RefCell::new(vec![]);
        let mut stamp = |v: &str| stamped.borrow_mut().push(v.to_owned());
        let syncs = Cell::new(0);
        let mut sync_fn = || syncs.set(syncs.get() + 1);
        stub_0x2b548(&mut loaded, &mut stamp, &mut sync_fn);
        assert!(!loaded.player_connected && !loaded.child_connected);
        assert_eq!(*stamped.borrow(), ["inGame"]);
        assert_eq!(syncs.get(), 1);
        assert!(loaded.memory_checker_stopped);
        let mut built = PlaceLauncherState::default();
        stub_0x2b724(&mut built);
        assert!(built.teleporter.is_none());
        built.teleporter = Some(TeleporterState { window: None });
        built.player_connected = true;
        stub_0x2b654(&mut built);
        assert!(built.teleporter.is_none() && !built.player_connected);
    }
    #[test]
    fn teleport_helpers_round_trip() {
        let game = GameToken { unsecured: false, is_app: true };
        let calls = RefCell::new(vec![]);
        let mut http = |url: &str| {
            calls.borrow_mut().push(url.to_owned());
            format!("resp:{url}")
        };
        let ran = RefCell::new(vec![]);
        let mut exec = |resp: &str, extra: &str, _: &GameToken| {
            ran.borrow_mut().push((resp.to_owned(), extra.to_owned()));
        };
        let ok = Cell::new(0);
        let mut success = || ok.set(ok.get() + 1);
        stub_0x2a350("u", "s", "x", &game, true, &mut http, &mut exec, &mut success);
        assert_eq!(*calls.borrow(), ["u?suggest=s"]);
        assert_eq!(*ran.borrow(), [("resp:u?suggest=s".to_string(), "x".to_string())]);
        assert_eq!(ok.get(), 1);
        stub_0x2a350("u", "", "x", &game, false, &mut http, &mut exec, &mut success);
        assert_eq!(calls.borrow().last().map(String::as_str), Some("u"));
        assert_eq!(ok.get(), 1);
        let marshalled: RefCell<Vec<u32>> = RefCell::new(vec![]);
        let mut marshal = |v: u32, _: &GameToken| marshalled.borrow_mut().push(v);
        stub_0x2aba4(3, &game, &mut marshal);
        assert_eq!(*marshalled.borrow(), [3]);
        let cap = FinishTeleportCapture {
            first: Some(1),
            second: Some(2),
            raw: 5,
            game: Some(game),
        };
        let src = cap.clone();
        let mut dst = FinishTeleportCapture::default();
        stub_0x2acec(&mut dst, &src);
        assert_eq!((dst.first, dst.second, dst.raw, dst.game), (Some(1), Some(2), 5, Some(game)));
        stub_0x2ada4(&mut dst);
        assert_eq!((dst.first, dst.raw, dst.game), (None, 0, None));
        let _ = cap;
    }
    #[test]
    fn teleport_finish_animation_flow() {
        let game = GameToken { unsecured: true, is_app: false };
        let gamed = Cell::new(0);
        let mut set_game = |v: u32, _: &GameToken| gamed.set(v);
        let framed = RefCell::new(vec![]);
        let mut set_frame = |v: u32, b: [i32; 4]| framed.borrow_mut().push((v, b));
        let clips: RefCell<Vec<(u32, bool)>> = RefCell::new(vec![]);
        let mut set_clips = |v: u32, c: bool| clips.borrow_mut().push((v, c));
        let animated = Cell::new(0.0f64);
        let mut animate = |d: f64, a: &mut dyn FnMut(), c: &mut dyn FnMut()| {
            animated.set(d);
            a();
            c();
        };
        stub_0x2b754(
            9, &game, true, Some(4), Some(6), Some([1, 2, 3, 4]),
            &mut set_game, &mut set_frame, &mut set_clips, &mut animate,
        );
        assert_eq!(gamed.get(), 6);
        assert_eq!(*framed.borrow(), [(4, [1, 2, 3, 4])]);
        assert_eq!(*clips.borrow(), [(4, false)]);
        assert_eq!(animated.get(), 0.5);
        stub_0x2b754(
            9, &game, false, Some(4), Some(6), None,
            &mut set_game, &mut set_frame, &mut set_clips, &mut animate,
        );
        assert_eq!(framed.borrow().len(), 1);
        stub_0x2b980(None, None, &mut set_frame);
        stub_0x2ba14(None, &mut set_clips);
        assert_eq!((framed.borrow().len(), clips.borrow().len()), (1, 1));
        let src = BlockCapture { target: Some(8) };
        for (copy, drop_fn) in [
            (stub_0x2a988 as fn(&mut BlockCapture, &BlockCapture), stub_0x2a994 as fn(&mut BlockCapture)),
            (stub_0x2ba00 as fn(&mut BlockCapture, &BlockCapture), stub_0x2ba0c as fn(&mut BlockCapture)),
            (stub_0x2ba40 as fn(&mut BlockCapture, &BlockCapture), stub_0x2ba4c as fn(&mut BlockCapture)),
            (stub_0x29c34 as fn(&mut BlockCapture, &BlockCapture), stub_0x29c58 as fn(&mut BlockCapture)),
            (stub_0x29c88 as fn(&mut BlockCapture, &BlockCapture), stub_0x29c94 as fn(&mut BlockCapture)),
        ] {
            let mut dst = BlockCapture::default();
            copy(&mut dst, &src);
            assert_eq!(dst.target, Some(8));
            drop_fn(&mut dst);
            assert_eq!(dst.target, None);
        }
        let mut state = armed();
        let mut shutdowns = 0;
        let mut shutdown = |_: &mut PlaceLauncherState| shutdowns += 1;
        stub_0x29c74(&mut state, &mut shutdown);
        assert_eq!(shutdowns, 1);
    }

    #[test]
    fn join_game_place_id_app_start_path() {
        let mut state = PlaceLauncherState::default();
        let game = GameToken { unsecured: false, is_app: true };
        let fetched = RefCell::new(vec![]);
        let mut http = |url: &str| {
            fetched.borrow_mut().push(url.to_owned());
            "signed-blob".to_string()
        };
        let ran = RefCell::new(vec![]);
        let mut signed = |_: &GameToken, body: &str| ran.borrow_mut().push(body.to_owned());
        let mut url_script = |_: &GameToken, _: &str| panic!("must not run url script");
        let mut sleeps = 0;
        let mut sleep = |_: u32| sleeps += 1;
        let mut ua = 0;
        let mut register = || ua += 1;
        let mut reported = vec![];
        let mut report = |kind: i32, place: i32| reported.push((kind, place));
        let mut pages = vec![];
        let mut track = |p: &str| pages.push(p.to_owned());
        let mut leave = |_: &mut PlaceLauncherState| panic!("must not leave");
        let mut fail = |_: &mut PlaceLauncherState| panic!("must not fail");
        stub_0x278a8(
            &mut state, 7, &game, 2, true, "http://base/", &mut register, &mut http,
            &mut sleep, &mut signed, &mut url_script, &mut report, &mut track,
            &mut leave, &mut fail,
        );
        assert_eq!(*fetched.borrow(), ["http://base/Game/AppStart.ashx?appid=7"]);
        assert_eq!(*ran.borrow(), ["signed-blob"]);
        assert_eq!((ua, sleeps), (1, 0));
        assert_eq!(state.last_place_id, 7);
        assert_eq!((reported, pages), (vec![(3, 7)], vec!["Visit/Success/Join".to_string()]));
        assert!(state.last_log.as_deref().unwrap().contains("joinGamePlaceId 7"));
    }
    #[test]
    fn join_game_place_id_poll_success_and_exhaustion() {
        let game = GameToken { unsecured: false, is_app: false };
        let scripted = RefCell::new(vec![]);
        let mut url_script = |_: &GameToken, url: &str| scripted.borrow_mut().push(url.to_owned());
        let mut signed = |_: &GameToken, _: &str| panic!("must not run signed script");
        let sleeps: RefCell<Vec<u32>> = RefCell::new(vec![]);
        let mut sleep = |us: u32| sleeps.borrow_mut().push(us);
        let mut register = || {};
        let mut report = |_: i32, _: i32| {};
        let mut track = |_: &str| {};
        let mut leave = |_: &mut PlaceLauncherState| {};
        let mut fail = |_: &mut PlaceLauncherState| {};
        let bodies = RefCell::new(vec![
            "{\"status\":0}".to_string(),
            "{\"status\":2,\"joinScriptUrl\":\"http:\\/\\/h\\/v\",\"x\":1}".to_string(),
        ]);
        let mut http = |_: &str| bodies.borrow_mut().remove(0);
        let mut state = PlaceLauncherState::default();
        stub_0x278a8(
            &mut state, 9, &game, 0, false, "http://base/", &mut register, &mut http,
            &mut sleep, &mut signed, &mut url_script, &mut report, &mut track,
            &mut leave, &mut fail,
        );
        assert_eq!(*sleeps.borrow(), [0x3d090]);
        assert_eq!(*scripted.borrow(), ["http://h/v"]);
        assert_eq!(state.last_place_id, 9);
        assert_eq!(state.last_alert, None);
        let bodies = RefCell::new(vec!["{\"status\":9}".to_string()]);
        let mut http = |_: &str| bodies.borrow().last().unwrap().clone();
        let mut left = 0;
        let mut leave = |_: &mut PlaceLauncherState| left += 1;
        let mut failed = 0;
        let mut fail = |_: &mut PlaceLauncherState| failed += 1;
        let mut state = PlaceLauncherState::default();
        stub_0x278a8(
            &mut state, 3, &game, 1, false, "http://base/", &mut register, &mut http,
            &mut sleep, &mut signed, &mut url_script, &mut report, &mut track,
            &mut leave, &mut fail,
        );
        assert_eq!((left, failed), (1, 1));
        assert_eq!(sleeps.borrow().iter().filter(|s| **s == 0xf3e58).count(), 6);
        assert_eq!(state.last_alert.as_deref(), Some("ConnectionError"));
        assert!(state.last_log.as_deref().unwrap().contains("Cannot follow user 3"));
        assert_eq!(state.last_place_id, 0);
    }
    #[test]
    fn join_script_url_parsing() {
        assert_eq!(join_script_url("{\"joinScriptUrl\":\"a\\/b\",}"), "a/b");
        assert_eq!(join_script_url("no key here"), "");
        assert_eq!(join_script_url("\"joinScriptUrl\""), "");
    }
    #[test]
    fn present_game_view_gates_and_presents() {
        let presented: RefCell<Vec<(u32, u32)>> = RefCell::new(vec![]);
        let completed = Cell::new(0);
        let mut handle = || completed.set(completed.get() + 1);
        let mut present = |non_game: u32, ogre: u32, done: &mut dyn FnMut()| {
            presented.borrow_mut().push((non_game, ogre));
            done();
        };
        stub_0x2c138(true, Some(5), Some(6), false, &mut present, &mut handle);
        assert_eq!(*presented.borrow(), [(6, 5)]);
        assert_eq!(completed.get(), 1);
        stub_0x2c138(true, Some(5), Some(6), true, &mut present, &mut handle);
        stub_0x2c138(false, Some(5), Some(6), false, &mut present, &mut handle);
        stub_0x2c138(true, None, Some(6), false, &mut present, &mut handle);
        stub_0x2c138(true, Some(5), None, false, &mut present, &mut handle);
        assert_eq!((presented.borrow().len(), completed.get()), (1, 1));
        stub_0x2c1f8(false, &mut handle);
        assert_eq!(completed.get(), 1);
    }
    #[test]
    fn control_view_helper_flow() {
        let game = GameToken { unsecured: false, is_app: true };
        let resolved = RefCell::new(vec![]);
        let mut resolve = |name: &str| {
            resolved.borrow_mut().push(name.to_owned());
            name.len() as u32
        };
        let views = RefCell::new(vec![]);
        let mut set_view = |v: u32| views.borrow_mut().push(v);
        let built = RefCell::new(vec![]);
        let mut create = |parent: u32, bounds: Option<[i32; 4]>, _: &GameToken| {
            built.borrow_mut().push((parent, bounds));
            50u32
        };
        let subs = RefCell::new(vec![]);
        let mut add = |parent: u32, child: u32| subs.borrow_mut().push((parent, child));
        let wins = RefCell::new(vec![]);
        let mut set_win = |v: u32| wins.borrow_mut().push(v);
        let mains = Cell::new(0);
        let mut dispatch = || mains.set(mains.get() + 1);
        stub_0x2c224(
            9, &game, true, true, true, Some([1, 2, 3, 4]), &mut resolve, &mut set_view,
            &mut create, &mut add, &mut set_win, &mut dispatch,
        );
        assert_eq!(*resolved.borrow(), ["VIEW", "WINDOW"]);
        assert_eq!(*views.borrow(), [4]);
        assert_eq!(*built.borrow(), [(4, Some([1, 2, 3, 4]))]);
        assert_eq!(*subs.borrow(), [(4, 50), (4, 50)]);
        assert_eq!(*wins.borrow(), [6]);
        assert_eq!(mains.get(), 1);
        stub_0x2c224(
            9, &game, true, false, true, None, &mut resolve, &mut set_view,
            &mut create, &mut add, &mut set_win, &mut dispatch,
        );
        stub_0x2c224(
            9, &game, false, true, false, None, &mut resolve, &mut set_view,
            &mut create, &mut add, &mut set_win, &mut dispatch,
        );
        assert_eq!((built.borrow().len(), mains.get()), (1, 1));
    }
    #[test]
    fn service_singleton_and_wrappers() {
        let mut cell = TaskSchedulerSettingsCell::default();
        let mut creates = 0;
        let mut create = || creates += 1;
        assert!(stub_0x2c5b0(&mut cell, &mut create));
        assert!(stub_0x2c5b0(&mut cell, &mut create));
        assert_eq!(creates, 1);
        let stored: RefCell<Vec<u32>> = RefCell::new(vec![]);
        let mut store = |v: u32| stored.borrow_mut().push(v);
        let mut find = || Some(12u32);
        assert_eq!(stub_0x2c764(Some(7), &mut find, &mut store), Some(7));
        assert_eq!(stored.borrow().len(), 0);
        assert_eq!(stub_0x2c764(None, &mut find, &mut store), Some(12));
        assert_eq!(*stored.borrow(), [12]);
        let mut linked = 0;
        let mut link = || linked += 1;
        stub_0x2c8c0(&mut link);
        assert_eq!(linked, 1);
        let game = GameToken { unsecured: true, is_app: false };
        assert_eq!(stub_0x2c9a8(&game), game);
        let src = BlockCapture { target: Some(3) };
        let mut dst = BlockCapture::default();
        stub_0x2c210(&mut dst, &src);
        assert_eq!(dst.target, Some(3));
        stub_0x2c21c(&mut dst);
        assert_eq!(dst.target, None);
    }
}

// 0x298e0 — -[PlaceLauncher leaveGame]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher leaveGame]")]
pub fn stub_0x298e0(
    state: &mut PlaceLauncherState,
    ogre_controller_present: bool,
    system_version: f32,
    set_idle_timer_disabled: &mut dyn FnMut(bool),
    set_game_state: &mut dyn FnMut(&str),
    synchronize_defaults: &mut dyn FnMut(),
    report_session: &mut dyn FnMut(i32),
    track_page_view: &mut dyn FnMut(&str),
    begin_bg_task: &mut dyn FnMut(&mut dyn FnMut(u32)) -> u32,
    set_bg_task: &mut dyn FnMut(u32),
    end_bg_task: &mut dyn FnMut(u32),
    dispatch_main: &mut dyn FnMut(&mut dyn FnMut()),
    leave_shutdown: &mut dyn FnMut(&mut PlaceLauncherState),
) {
    // IDA 0x298e0: no-ops unless playing, not already leaving, and the
    // ogre controller exists (0x2996e/0x29978/0x2998e); otherwise latches
    // `isLeavingGame`, re-enables the idle timer, stamps
    // `RobloxGameState = "leaveGame"` + synchronize, closes the child
    // connections (0x2b5e0), reports session 4, tracks
    // `Visit/Success/LeaveGame`, begins the delegate background task
    // (expiration block 0x29bb4), and `dispatch_async`s the
    // `leaveGameShutdown` block (0x29c74) on iOS 6+ (disasm `VMOV.F32
    // D0, #6.0` + `VCMPE`, so a genuine `f32` compare) or shuts down
    // inline on older systems (0x29b72).
    if !state.currently_playing || state.is_leaving_game || !ogre_controller_present {
        return;
    }
    state.is_leaving_game = true;
    set_idle_timer_disabled(false);
    set_game_state("leaveGame");
    synchronize_defaults();
    stub_0x2b5e0(state);
    report_session(4);
    track_page_view("Visit/Success/LeaveGame");
    let mut expiration = |current_task: u32| {
        stub_0x29bb4(state, current_task, end_bg_task, set_bg_task);
    };
    let bg_task = begin_bg_task(&mut expiration);
    set_bg_task(bg_task);
    if system_version >= 6.0 {
        let mut shutdown = || leave_shutdown(state);
        dispatch_main(&mut shutdown);
    } else {
        leave_shutdown(state);
    }
}

// 0x29bb4 — ___26-[PlaceLauncher leaveGame]_block_invoke
#[doc(alias = "___26-[PlaceLauncher leaveGame]_block_invoke")]
pub fn stub_0x29bb4(
    state: &mut PlaceLauncherState,
    delegate_bg_task: u32,
    end_bg_task: &mut dyn FnMut(u32),
    set_bg_task: &mut dyn FnMut(u32),
) {
    // IDA 0x29bb4: background-task expiration handler — clears
    // `isLeavingGame` (disasm `STRB` via the `isLeavingGame` IVAR ref,
    // 0x29bde), ends the delegate `bgTask`, and resets it to
    // `UIBackgroundTaskInvalid`.
    state.is_leaving_game = false;
    end_bg_task(delegate_bg_task);
    set_bg_task(0);
}

// 0x29c34 — ___copy_helper_block_217
#[doc(alias = "___copy_helper_block_217")]
pub fn stub_0x29c34(dst: &mut BlockCapture, src: &BlockCapture) {
    // IDA 0x29c34 `__copy_helper_block_217`: two `_Block_object_assign`
    // retains (+20/+24, cf. 0x1f660); the host retains the whole capture.
    *dst = src.clone();
}

// 0x29c58 — ___destroy_helper_block_218
#[doc(alias = "___destroy_helper_block_218")]
pub fn stub_0x29c58(slot: &mut BlockCapture) {
    // IDA 0x29c58 `__destroy_helper_block_218`: two
    // `_Block_object_dispose` releases (+20/+24, cf. 0x1f4a0).
    *slot = BlockCapture::default();
}

// 0x29c74 — ___26-[PlaceLauncher leaveGame]_block_invoke231
#[doc(alias = "___26-[PlaceLauncher leaveGame]_block_invoke231")]
pub fn stub_0x29c74(
    state: &mut PlaceLauncherState,
    leave_shutdown: &mut dyn FnMut(&mut PlaceLauncherState),
) {
    // IDA 0x29c74: main-queue block — `leaveGameShutdown` (0x295c0).
    leave_shutdown(state);
}

// 0x29c88 — ___copy_helper_block_232
#[doc(alias = "___copy_helper_block_232")]
pub fn stub_0x29c88(dst: &mut BlockCapture, src: &BlockCapture) {
    // IDA 0x29c88 `__copy_helper_block_232`: single
    // `_Block_object_assign` retain (+20, cf. 0x1f660).
    *dst = src.clone();
}

// 0x29c94 — ___destroy_helper_block_233
#[doc(alias = "___destroy_helper_block_233")]
pub fn stub_0x29c94(slot: &mut BlockCapture) {
    // IDA 0x29c94 `__destroy_helper_block_233`: single
    // `_Block_object_dispose` release (+20, cf. 0x1f4a0).
    *slot = BlockCapture::default();
}

// 0x29c9c — -[PlaceLauncher disableViewBecauseGoingToBackground]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher disableViewBecauseGoingToBackground]")]
pub fn stub_0x29c9c(state: &PlaceLauncherState, stop_rendering: &mut dyn FnMut(u32)) {
    // IDA 0x29c9c: with a live `rbxView`, `requestStopRenderingForBackgroundMode`.
    if let Some(view) = state.rbx_view {
        stop_rendering(view);
    }
}

// 0x29cb4 — -[PlaceLauncher enableViewBecauseGoingToForeground]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher enableViewBecauseGoingToForeground]")]
pub fn stub_0x29cb4(state: &PlaceLauncherState, resume_rendering: &mut dyn FnMut(u32)) {
    // IDA 0x29cb4: with a live `rbxView`, `requestResumeRendering`.
    if let Some(view) = state.rbx_view {
        resume_rendering(view);
    }
}

// 0x2a350 — __ZL16joinGameTeleportSsSsSsP8NSObjectN5boost10shared_ptrIN3RBX4GameEEE
#[doc(alias = "joinGameTeleport(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>)")]
pub fn stub_0x2a350(
    url: &str,
    suggest: &str,
    extra: &str,
    game: &GameToken,
    controller_present: bool,
    http_get: &mut dyn FnMut(&str) -> String,
    execute_url_script: &mut dyn FnMut(&str, &str, &GameToken),
    handle_start_game_success: &mut dyn FnMut(),
) {
    // IDA 0x2a350: appends `?suggest=<suggest>` when the suggest string is
    // non-empty (0x2a3b8 length check), `RBX::Http(url).get` against
    // `GetBaseURL()`, then `executeUrlScript(result, extra)` (the third
    // string rides along verbatim), then `handleStartGameSuccess` when a
    // controller was supplied (string retains/releases fold into host
    // ownership).
    let full = if suggest.is_empty() {
        url.to_owned()
    } else {
        format!("{url}?suggest={suggest}")
    };
    let response = http_get(&full);
    execute_url_script(&response, extra, game);
    if controller_present {
        handle_start_game_success();
    }
}

// 0x2a988 — ___copy_helper_block_243
#[doc(alias = "___copy_helper_block_243")]
pub fn stub_0x2a988(dst: &mut BlockCapture, src: &BlockCapture) {
    // IDA 0x2a988 `__copy_helper_block_243`: single
    // `_Block_object_assign` retain (+20, cf. 0x1f660).
    *dst = src.clone();
}

// 0x2a994 — ___destroy_helper_block_244
#[doc(alias = "___destroy_helper_block_244")]
pub fn stub_0x2a994(slot: &mut BlockCapture) {
    // IDA 0x2a994 `__destroy_helper_block_244`: single
    // `_Block_object_dispose` release (+20, cf. 0x1f4a0).
    *slot = BlockCapture::default();
}

// 0x2aba4 — __ZL14finishTeleportP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEEPNS3_18FunctionMarshallerE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "finishTeleport(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *)")]
pub fn stub_0x2aba4(
    view: u32,
    game: &GameToken,
    execute: &mut dyn FnMut(u32, &GameToken),
) {
    // IDA 0x2aba4: binds `finishTeleportHelper(view, game)`
    // (`boost::bind` → closure) into `FunctionMarshaller::Execute`, then
    // clears the functor (cf. 0x2643c).
    execute(view, game);
}

/// Block capture for `finishTeleport` (IDA 0x2acec/0x2ada4): two retained
/// ObjC slots (+20/+24), a raw word (+28), and the copied
/// `shared_ptr<RBX::Game>` (+32).
#[derive(Debug, Clone, Default)]
pub struct FinishTeleportCapture {
    pub first: Option<u32>,
    pub second: Option<u32>,
    pub raw: u32,
    pub game: Option<GameToken>,
}

// 0x2acec — ___copy_helper_block_247
// type: void __fastcall(_DWORD *, const shared_count *)
#[doc(alias = "___copy_helper_block_247")]
pub fn stub_0x2acec(dst: &mut FinishTeleportCapture, src: &FinishTeleportCapture) {
    // IDA 0x2acec `__copy_helper_block_247`: two `_Block_object_assign`
    // retains (+20/+24), a raw word copy (+28), and a `shared_count`
    // copy (+32); the host clones the whole capture.
    *dst = src.clone();
}

// 0x2ada4 — ___destroy_helper_block_248
#[doc(alias = "___destroy_helper_block_248")]
pub fn stub_0x2ada4(slot: &mut FinishTeleportCapture) {
    // IDA 0x2ada4 `__destroy_helper_block_248`: two
    // `_Block_object_dispose` releases (+20/+24) plus the `shared_count`
    // release (+32); the host drops the whole capture.
    *slot = FinishTeleportCapture::default();
}

// 0x2ae44 — -[PlaceLauncher isCurrentlyPlayingGame]
// type: char __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher isCurrentlyPlayingGame]")]
pub fn stub_0x2ae44(state: &PlaceLauncherState) -> bool {
    // IDA 0x2ae44: `isCurrentlyPlayingGame` IVAR load (same latch as
    // `getIsCurrentlyPlayingGame`, 0x24a18).
    state.currently_playing
}

// 0x2ae54 — -[PlaceLauncher applicationDidReceiveMemoryWarning]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher applicationDidReceiveMemoryWarning]")]
pub fn stub_0x2ae54(
    state: &mut PlaceLauncherState,
    free_memory_bytes: u32,
    warnings_enabled: bool,
    print_free_memory: &mut dyn FnMut(u32),
    report_session: &mut dyn FnMut(i32, i32),
    leave_game: &mut dyn FnMut(&mut PlaceLauncherState),
) {
    // IDA 0x2ae54: out of game it just logs and ignores (0x2afc2); in
    // game it prints free memory, tracks `PlayErrors/OutOfMemory_EarlyExit`
    // + session 5 when a child/player connection is live else
    // `PlayErrors/OutOfMemory` + session 6 (0x2aeea/0x2af06), closes the
    // child connections (0x2b056), alerts `MemoryError` when the warnings
    // preference is set, logs the in-game shutdown, and leaves the game.
    if !state.currently_playing {
        state.last_log = Some("PlaceLauncher: applicationDidReceiveMemoryWarning receive while out of game, ignoring".to_string());
        return;
    }
    print_free_memory(free_memory_bytes);
    let place_id = state.last_place_id;
    if state.child_connected || state.player_connected {
        state.ga_event = Some(GaEvent {
            category: "PlayErrors".to_string(),
            action: "OutOfMemory_EarlyExit".to_string(),
            label: place_id,
        });
        report_session(5, place_id);
    } else {
        state.ga_event = Some(GaEvent {
            category: "PlayErrors".to_string(),
            action: "OutOfMemory".to_string(),
            label: place_id,
        });
        report_session(6, place_id);
    }
    stub_0x2b5e0(state);
    if warnings_enabled {
        state.last_alert = Some("MemoryError".to_string());
    }
    state.last_log = Some("PlaceLauncher: applicationDidReceiveMemoryWarning resulting in in-game shutdown".to_string());
    leave_game(state);
}

// 0x2b1bc — -[PlaceLauncher childAdded:]
// type: void __cdecl(PlaceLauncher *self, SEL, shared_ptr<RBX::Instance>)
#[doc(alias = "-[PlaceLauncher childAdded:]")]
pub fn stub_0x2b1bc(
    state: &mut PlaceLauncherState,
    players_present: bool,
    local_player_present: bool,
    added_is_local_player: bool,
    connect_player_loaded: &mut dyn FnMut(),
) {
    // IDA 0x2b1bc: with no `rbxView`, datamodel, `Players` service, or
    // local player it closes the child connections (0x2b326/0x2b34e/
    // 0x2b378/0x2b3a2); otherwise it connects `playerLoaded:` on the
    // player signal into `playerConnection` and disconnects
    // `childConnection` — both the local-child and other-child arms
    // connect identically, differing only in the log line.
    if state.rbx_view.is_none() || !players_present || !local_player_present {
        stub_0x2b5e0(state);
        return;
    }
    let _ = added_is_local_player;
    connect_player_loaded();
    state.player_connected = true;
    state.child_connected = false;
}

// 0x2b548 — -[PlaceLauncher playerLoaded:]
// type: void __cdecl(PlaceLauncher *self, SEL, shared_ptr<RBX::Instance>)
#[doc(alias = "-[PlaceLauncher playerLoaded:]")]
pub fn stub_0x2b548(
    state: &mut PlaceLauncherState,
    set_game_state: &mut dyn FnMut(&str),
    synchronize_defaults: &mut dyn FnMut(),
) {
    // IDA 0x2b548: disconnects `playerConnection`, closes the child
    // connections, then stamps `RobloxGameState = "inGame"` + synchronize.
    state.player_connected = false;
    stub_0x2b5e0(state);
    set_game_state("inGame");
    synchronize_defaults();
}

// 0x2b5e0 — -[PlaceLauncher closeChildConnections]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher closeChildConnections]")]
pub fn stub_0x2b5e0(state: &mut PlaceLauncherState) {
    // IDA 0x2b5e0: disconnects `childConnection`/`playerConnection`
    // when connected (0x2b5fc/0x2b61a), then stops the free-memory
    // checker (intrusive releases fold into host ownership).
    state.child_connected = false;
    state.player_connected = false;
    state.memory_checker_stopped = true;
}

// 0x2b654 — -[PlaceLauncher .cxx_destruct]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher .cxx_destruct]")]
pub fn stub_0x2b654(state: &mut PlaceLauncherState) {
    // IDA 0x2b654 `-[PlaceLauncher .cxx_destruct]`: releases the
    // player/child connection slots and the teleporter (intrusive
    // releases fold into host ownership).
    state.player_connected = false;
    state.child_connected = false;
    state.teleporter = None;
}

// 0x2b724 — -[PlaceLauncher .cxx_construct]
// type: id __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher .cxx_construct]")]
pub fn stub_0x2b724(state: &mut PlaceLauncherState) {
    // IDA 0x2b724 `-[PlaceLauncher .cxx_construct]`: zeroes the
    // teleporter and both connection slots.
    state.teleporter = None;
    state.child_connected = false;
    state.player_connected = false;
}

// 0x2b754 — __ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE
#[doc(alias = "finishTeleportHelper(RobloxView *,rbx_core::SharedPtr<RBX::Game>)")]
pub fn stub_0x2b754(
    view: u32,
    game: &GameToken,
    main_controller_present: bool,
    ogre_view: Option<u32>,
    first_subview: Option<u32>,
    screen_bounds: Option<[i32; 4]>,
    set_game: &mut dyn FnMut(u32, &GameToken),
    set_frame: &mut dyn FnMut(u32, [i32; 4]),
    set_clips_to_bounds: &mut dyn FnMut(u32, bool),
    animate: &mut dyn FnMut(f64, &mut dyn FnMut(), &mut dyn FnMut()),
) {
    // IDA 0x2b754: no-ops without the shared `MainViewController`;
    // otherwise `setGame:`s the first enumerated ogre-controller subview
    // (0x2b838/0x2b86c) and runs `animateWithDuration:0.5` (disasm
    // `VMOV.F64 D16, #0.5`, so a genuine `f64`) with the frame-sizing
    // animations block (0x2b980) and the clips completion (0x2ba14).
    // The view rides the animation cookies; the host keeps it explicit.
    let _ = view;
    if !main_controller_present {
        return;
    }
    if let Some(subview) = first_subview {
        set_game(subview, game);
    }
    let mut animations = || {
        stub_0x2b980(ogre_view, screen_bounds, set_frame);
    };
    let mut completion = || {
        stub_0x2ba14(ogre_view, set_clips_to_bounds);
    };
    animate(0.5, &mut animations, &mut completion);
}
// 0x2b980 — ____ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE_block_invoke
#[doc(alias = "____ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE_block_invoke")]
pub fn stub_0x2b980(
    view: Option<u32>,
    screen_bounds: Option<[i32; 4]>,
    set_frame: &mut dyn FnMut(u32, [i32; 4]),
) {
    // IDA 0x2b980: sizes the ogre controller's view to the main-screen
    // bounds (zeroed when headless, cf. 0x25498); nil views no-op.
    if let Some(view) = view {
        set_frame(view, screen_bounds.unwrap_or([0, 0, 0, 0]));
    }
}

// 0x2ba00 — ___copy_helper_block_425
#[doc(alias = "___copy_helper_block_425")]
pub fn stub_0x2ba00(dst: &mut BlockCapture, src: &BlockCapture) {
    // IDA 0x2ba00 `__copy_helper_block_425`: single
    // `_Block_object_assign` retain (+20, cf. 0x1f660).
    *dst = src.clone();
}
// 0x2ba0c — ___destroy_helper_block_426
#[doc(alias = "___destroy_helper_block_426")]
pub fn stub_0x2ba0c(slot: &mut BlockCapture) {
    // IDA 0x2ba0c `__destroy_helper_block_426`: single
    // `_Block_object_dispose` release (+20, cf. 0x1f4a0).
    *slot = BlockCapture::default();
}

// 0x2ba14 — ____ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE_block_invoke428
#[doc(alias = "____ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE_block_invoke428")]
pub fn stub_0x2ba14(view: Option<u32>, set_clips_to_bounds: &mut dyn FnMut(u32, bool)) {
    // IDA 0x2ba14: completion block — `setClipsToBounds:NO` on the ogre
    // controller's view; nil views no-op.
    if let Some(view) = view {
        set_clips_to_bounds(view, false);
    }
}
// 0x2ba40 — ___copy_helper_block_429
#[doc(alias = "___copy_helper_block_429")]
pub fn stub_0x2ba40(dst: &mut BlockCapture, src: &BlockCapture) {
    // IDA 0x2ba40 `__copy_helper_block_429`: single
    // `_Block_object_assign` retain (+20, cf. 0x1f660).
    *dst = src.clone();
}

// 0x2ba4c — ___destroy_helper_block_430
#[doc(alias = "___destroy_helper_block_430")]
pub fn stub_0x2ba4c(slot: &mut BlockCapture) {
    // IDA 0x2ba4c `__destroy_helper_block_430`: single
    // `_Block_object_dispose` release (+20, cf. 0x1f4a0).
    *slot = BlockCapture::default();
}

// 0x2c138 — ____ZL15presentGameViewv_block_invoke
// type: void __cdecl(id)
#[doc(alias = "____ZL15presentGameViewv_block_invoke")]
pub fn stub_0x2c138(
    main_controller_present: bool,
    ogre_controller: Option<u32>,
    last_non_game_controller: Option<u32>,
    presented_is_ogre: bool,
    present: &mut dyn FnMut(u32, u32, &mut dyn FnMut()),
    handle_success: &mut dyn FnMut(),
) {
    // IDA 0x2c138: with a shared `MainViewController`, an ogre
    // controller, and a last non-game controller not already presenting
    // the ogre controller (0x2c15e/0x2c176/0x2c18c/0x2c1a2), presents it
    // unanimated with the success completion (0x2c1f8).
    let (Some(ogre), Some(non_game)) = (ogre_controller, last_non_game_controller) else {
        return;
    };
    if !main_controller_present || presented_is_ogre {
        return;
    }
    let mut completion = || stub_0x2c1f8(true, handle_success);
    present(non_game, ogre, &mut completion);
}

// 0x2c1f8 — ____ZL15presentGameViewv_block_invoke_2
// type: id __fastcall(int)
#[doc(alias = "____ZL15presentGameViewv_block_invoke_2")]
pub fn stub_0x2c1f8(controller_present: bool, handle_success: &mut dyn FnMut()) {
    // IDA 0x2c1f8: completion block — `handleStartGameSuccess` unless
    // the captured controller is nil.
    if controller_present {
        handle_success();
    }
}

// 0x2c210 — ___copy_helper_block_499
#[doc(alias = "___copy_helper_block_499")]
pub fn stub_0x2c210(dst: &mut BlockCapture, src: &BlockCapture) {
    // IDA 0x2c210 `__copy_helper_block_499`: single
    // `_Block_object_assign` retain (+20, cf. 0x1f660).
    *dst = src.clone();
}

// 0x2c21c — ___destroy_helper_block_500
#[doc(alias = "___destroy_helper_block_500")]
pub fn stub_0x2c21c(slot: &mut BlockCapture) {
    // IDA 0x2c21c `__destroy_helper_block_500`: single
    // `_Block_object_dispose` release (+20, cf. 0x1f4a0).
    *slot = BlockCapture::default();
}
// 0x2c224 — __ZL21initControlViewHelperP10RobloxViewa
// type: _DWORD __fastcall(RobloxView *, signed __int8)
#[doc(alias = "initControlViewHelper(RobloxView *,signed char)")]
pub fn stub_0x2c224(
    view: u32,
    game: &GameToken,
    flag: bool,
    main_controller_present: bool,
    render_window_present: bool,
    screen_bounds: Option<[i32; 4]>,
    resolve_target: &mut dyn FnMut(&str) -> u32,
    set_ogre_view: &mut dyn FnMut(u32),
    create_control_view: &mut dyn FnMut(u32, Option<[i32; 4]>, &GameToken) -> u32,
    add_subview: &mut dyn FnMut(u32, u32),
    set_ogre_window: &mut dyn FnMut(u32),
    dispatch_main: &mut dyn FnMut(),
) {
    // IDA 0x2c224: no-ops without the shared `MainViewController` or the
    // render window (0x2c290/0x2c296); otherwise resolves the `VIEW`
    // target, installs the ogre view, builds the `ControlView` (screen
    // bounds, zeroed when headless, cf. 0x25498), adds it as a subview
    // twice (0x2c3de/0x2c43c), installs the `WINDOW` target, and with the
    // flag dispatches the global control-view block on the main queue.
    // The view rides the resolve cookies; the host keeps it explicit.
    let _ = view;
    if !main_controller_present || !render_window_present {
        return;
    }
    let ogre_view = resolve_target("VIEW");
    set_ogre_view(ogre_view);
    let control = create_control_view(ogre_view, screen_bounds, game);
    add_subview(ogre_view, control);
    set_ogre_window(resolve_target("WINDOW"));
    add_subview(ogre_view, control);
    if flag {
        dispatch_main();
    }
}

/// `TaskSchedulerSettings` singleton cell (IDA 0x2c5b0): the cached
/// `sing` instance, created once under the `GlobalAdvancedSettings`
/// lock.
#[derive(Debug, Clone, Default)]
pub struct TaskSchedulerSettingsCell {
    pub live: bool,
}

// 0x2c5b0 — __ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEE9singletonEv
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, boost::mutex *, char, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEE9singletonEv")]
pub fn stub_0x2c5b0(cell: &mut TaskSchedulerSettingsCell, create: &mut dyn FnMut()) -> bool {
    // IDA 0x2c5b0: returns the cached `sing`; otherwise creates the
    // `TaskSchedulerSettings` under `GlobalAdvancedSettings` and caches
    // it (double-checked lock + assert fold into the live latch).
    if !cell.live {
        create();
        cell.live = true;
    }
    cell.live
}

// 0x2c764 — __ZNK3RBX15ServiceProvider4findINS_10GuiServiceEEEPT_v
// type: int __fastcall(pthread_mutex_t *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::GuiService * RBX::ServiceProvider::find<RBX::GuiService>(void)const")]
pub fn stub_0x2c764(
    cached: Option<u32>,
    find_by_name: &mut dyn FnMut() -> Option<u32>,
    store: &mut dyn FnMut(u32),
) -> Option<u32> {
    // IDA 0x2c764: the `call_once` class-index init folds into the host;
    // returns the cached service slot, else finds `GuiService` by class
    // name, caches it in the slot, and returns it (null class name →
    // `None`).
    if let Some(service) = cached {
        return Some(service);
    }
    let found = find_by_name();
    if let Some(service) = found {
        store(service);
    }
    found
}

// 0x2c8c0 — __ZN3rbx7signals6signalIFvSsEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string)>::connect<boost::function<void ()(std::string)>>(boost::function<void ()(std::string)> const&)")]
pub fn stub_0x2c8c0(connect: &mut dyn FnMut()) {
    // IDA 0x2c8c0: allocates the string-signal slot and inserts it
    // (`rbx_core::Signal::connect` on the host).
    connect();
}

// 0x2c9a8 — __ZN5boost10shared_ptrIN3RBX4GameEEC1INS1_16SecurePlayerGameEEEPT_
#[doc(alias = "rbx_core::SharedPtr<RBX::Game>::shared_ptr<RBX::SecurePlayerGame>(RBX::SecurePlayerGame *)")]
pub fn stub_0x2c9a8(game: &GameToken) -> GameToken {
    // IDA 0x2c9a8: `shared_ptr<Game>` adopt of a `SecurePlayerGame`
    // (refcount init folds into `Arc`); the token carries over.
    *game
}

// 0x2ca7c — __ZN5boost4bindIvRKSsNS_10shared_ptrIN3RBX4GameEEEPKcS6_EENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list_av_2<char const*,rbx_core::SharedPtr<RBX::Game>>::type> boost::bind<void,std::string const&,rbx_core::SharedPtr<RBX::Game>,char const*,rbx_core::SharedPtr<RBX::Game>>(void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),char const*,rbx_core::SharedPtr<RBX::Game>)")]
pub fn stub_0x2ca7c() -> ! {
    todo!("0x2ca7c boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list_av_2<char const*,rbx_core::SharedPtr<RBX::Game>>::type> boost::bind<void,std::s")
}

// 0x2cb64 — __ZN5boost4bindIviRKSsNS_10shared_ptrIN3RBX4GameEEEiPKcS6_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_ENS9_9list_av_3IT3_T4_T5_E4typeEEESG_SI_SJ_SK_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list_av_3<int,char const*,rbx_core::SharedPtr<RBX::Game>>::type> boost::bind<void,int,std::string const&,rbx_core::SharedPtr<RBX::Game>,int,char const*,rbx_core::SharedPtr<RBX::Game>>(void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),int,char const*,rbx_core::SharedPtr<RBX::Game>)")]
pub fn stub_0x2cb64() -> ! {
    todo!("0x2cb64 boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list_av_3<int,char const*,rbx_core::SharedPtr<RBX::Game>>::type> boost::bind<voi")
}

// 0x2cc54 — __ZN5boost4bindIviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestiS4_S5_EENS_3_bi6bind_tIT_PFS8_T0_T1_T2_ENS6_9list_av_3IT3_T4_T5_E4typeEEESD_SF_SG_SH_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list_av_3<int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest>::type> boost::bind<void,int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest,int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest>(void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest)")]
pub fn stub_0x2cc54() -> ! {
    todo!("0x2cc54 boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list_av_3<int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest>::type> boost::bind<vo")
}

// 0x2cd44 — __ZN5boost4bindIviNS_10shared_ptrIN3RBX4GameEEEiS4_EENS_3_bi6bind_tIT_PFS7_T0_T1_ENS5_9list_av_2IT2_T3_E4typeEEESB_SD_SE_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list_av_2<int,rbx_core::SharedPtr<RBX::Game>>::type> boost::bind<void,int,rbx_core::SharedPtr<RBX::Game>,int,rbx_core::SharedPtr<RBX::Game>>(void (*)(int,rbx_core::SharedPtr<RBX::Game>),int,rbx_core::SharedPtr<RBX::Game>)")]
pub fn stub_0x2cd44() -> ! {
    todo!("0x2cd44 boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list_av_2<int,rbx_core::SharedPtr<RBX::Game>>::type> boost::bind<void,int,rbx_core::SharedPtr<RBX::")
}

// 0x2ce2c — __ZN5boost4bindIvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEESsSsSsP24RobloxPageViewControllerS6_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_T3_T4_ENS9_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESI_SK_SL_SM_SN_SO_
// type: int __fastcall(int, int, std::string *, int, std::string *, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list_av_5<std::string,std::string,std::string,RobloxPageViewController *,rbx_core::SharedPtr<RBX::Game>>::type> boost::bind<void,std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>,std::string,std::string,std::string,RobloxPageViewController *,rbx_core::SharedPtr<RBX::Game>>(void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),std::string,std::string,std::string,RobloxPageViewController *,rbx_core::SharedPtr<RBX::Game>)")]
pub fn stub_0x2ce2c() -> ! {
    todo!("0x2ce2c boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list_av_5<std::string,std::string,std::string,RobloxPage")
}

// 0x2d280 — __ZN5boost4bindIvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS4_18FunctionMarshallerES2_S6_S8_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_ENS9_9list_av_3IT3_T4_T5_E4typeEEESG_SI_SJ_SK_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list_av_3<RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *>::type> boost::bind<void,RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *,RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *>(void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *)")]
pub fn stub_0x2d280() -> ! {
    todo!("0x2d280 boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list_av_3<RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::Funct")
}

// 0x2d370 — __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS7_5list3INS7_5valueISA_EENSJ_ISD_EENSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS7_5list3INS7_5valueISA_EENSJ_ISD_EENSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x2d370() -> ! {
    todo!("0x2d370 __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS7_5list3INS7_5valueISA_EENSJ_ISD_EENSJ_ISF_EEEE")
}

// 0x2d458 — __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, struct _Unwind_Exception *lpuexcpt, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x2d458() -> ! {
    todo!("0x2d458 __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEE")
}

// 0x2d544 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEEEEEvT_
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>)")]
pub fn stub_0x2d544() -> ! {
    todo!("0x2d544 void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<bo")
}

// 0x2d644 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x2d644() -> ! {
    todo!("0x2d644 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value")
}

// 0x2d660 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEvPNS8_9DataModelEE6invokeERNS1_15function_bufferESN_
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
pub fn stub_0x2d660() -> ! {
    todo!("0x2d660 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost:")
}

// 0x2d67c — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS3_4GameEEEPNS3_18FunctionMarshallerEENS8_5list3INS8_5valueISB_EENSK_ISE_EENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x2d67c() -> ! {
    todo!("0x2d67c bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *")
}

// 0x2d768 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS3_4GameEEEPNS3_18FunctionMarshallerEENS8_5list3INS8_5valueISB_EENSK_ISE_EENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x2d768() -> ! {
    todo!("0x2d768 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *")
}

// 0x2d884 — __ZN5boost3_bi5list3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEclIPFvS4_S9_SC_ENS0_5list1IRPNS7_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::operator()<void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *) &,boost::_bi::list1<RBX::DataModel *&> &,int)")]
pub fn stub_0x2d884() -> ! {
    todo!("0x2d884 void boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::operator()<void (*)(RobloxVi")
}

// 0x2d964 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x2d964() -> ! {
    todo!("0x2d964 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value")
}

// 0x2da9c — __ZN5boost3_bi5list3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEC2ES5_SA_SD_
#[doc(alias = "boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::list3(boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>)")]
pub fn stub_0x2da9c() -> ! {
    todo!("0x2da9c boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::list3(boost::_bi::value<RobloxVie")
}

// 0x2db54 — __ZN5boost3_bi8storage3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEC2ES5_SA_SD_
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::storage3(boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>)")]
pub fn stub_0x2db54() -> ! {
    todo!("0x2db54 boost::_bi::storage3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::storage3(boost::_bi::value<Rob")
}

// 0x2dc24 — __ZN5boost6threadC2INS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEEEOT_
#[doc(alias = "boost::thread::thread<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>> &&)")]
pub fn stub_0x2dc24() -> ! {
    todo!("0x2dc24 boost::thread::thread<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::strin")
}

// 0x2dfac — __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEEC2EOSK_
// type: int __fastcall(int, int, int, int, std::string *, std::string *, int, int, int, int)
#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::thread_data(boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>&&)")]
pub fn stub_0x2dfac() -> ! {
    todo!("0x2dfac boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::")
}

// 0x2e0f4 — __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEED1Ev
#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::~thread_data()")]
pub fn stub_0x2e0f4() -> ! {
    todo!("0x2e0f4 boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::")
}

// 0x2e1bc — __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEED0Ev
#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::~thread_data() [0x2e1bc]")]
pub fn stub_0x2e1bc() -> ! {
    todo!("0x2e1bc boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::")
}

// 0x2e284 — __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEE3runEv
#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::run(void)")]
pub fn stub_0x2e284() -> ! {
    todo!("0x2e284 boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::")
}

// 0x2e2a0 — __ZN5boost3_bi5list5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFvSsSsSsP8NSObjectSA_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(std::string *)
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::operator()<void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>) &,boost::_bi::list0 &,int)")]
pub fn stub_0x2e2a0() -> ! {
    todo!("0x2e2a0 void boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<")
}

// 0x2e518 — __ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS6_5list5INS6_5valueISsEESI_SI_NSH_IP24RobloxPageViewControllerEENSH_ISD_EEEEEEEEEEvPKNSA_IT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>>(rbx_core::SharedPtr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>> *)const")]
pub fn stub_0x2e518() -> ! {
    todo!("0x2e518 void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::_bi::bind_t<void,void")
}

// 0x2e5ec — __ZN5boost6detail12shared_countC2INS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS4_5list5INS4_5valueISsEESG_SG_NSF_IP24RobloxPageViewControllerEENSF_ISB_EEEEEEEEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>>(boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>> *)")]
pub fn stub_0x2e5ec() -> ! {
    todo!("0x2e5ec boost::detail::shared_count::shared_count<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>)")
}

// 0x2e6e0 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS3_5list5INS3_5valueISsEESF_SF_NSE_IP24RobloxPageViewControllerEENSE_ISA_EEEEEEEEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>>::~sp_counted_impl_p()")]
pub fn stub_0x2e6e0() -> ! {
    todo!("0x2e6e0 boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_")
}

// 0x2e6e4 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS3_5list5INS3_5valueISsEESF_SF_NSE_IP24RobloxPageViewControllerEENSE_ISA_EEEEEEEEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>>::~sp_counted_impl_p() [0x2e6e4]")]
pub fn stub_0x2e6e4() -> ! {
    todo!("0x2e6e4 boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_")
}

// 0x2e6e8 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS3_5list5INS3_5valueISsEESF_SF_NSE_IP24RobloxPageViewControllerEENSE_ISA_EEEEEEEEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>>::dispose(void)")]
pub fn stub_0x2e6e8() -> ! {
    todo!("0x2e6e8 boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_")
}

// 0x2e6f8 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS3_5list5INS3_5valueISsEESF_SF_NSE_IP24RobloxPageViewControllerEENSE_ISA_EEEEEEEEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>>::get_deleter(std::type_info const&)")]
pub fn stub_0x2e6f8() -> ! {
    todo!("0x2e6f8 boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_")
}

// 0x2e6fc — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS3_5list5INS3_5valueISsEESF_SF_NSE_IP24RobloxPageViewControllerEENSE_ISA_EEEEEEEEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>>::get_untyped_deleter(void)")]
pub fn stub_0x2e6fc() -> ! {
    todo!("0x2e6fc boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_")
}

// 0x2e700 — __ZN5boost3_bi5list5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S3_S3_S6_SB_
#[doc(alias = "boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::list5(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>)")]
pub fn stub_0x2e700() -> ! {
    todo!("0x2e700 boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_c")
}

// 0x2e970 — __ZN5boost3_bi8storage5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S3_S3_S6_SB_
#[doc(alias = "boost::_bi::storage5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::storage5(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>)")]
pub fn stub_0x2e970() -> ! {
    todo!("0x2e970 boost::_bi::storage5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<rb")
}

// 0x2ebbc — __ZN5boost3_bi8storage4INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEEEC2ES3_S3_S3_S6_
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>>::storage4(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>)")]
pub fn stub_0x2ebbc() -> ! {
    todo!("0x2ebbc boost::_bi::storage4<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>>::storage4(boost::_b")
}

// 0x2edec — __ZN5boost3_bi8storage3INS0_5valueISsEES3_S3_EC2ES3_S3_S3_
// type: int(void)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>)")]
pub fn stub_0x2edec() -> ! {
    todo!("0x2edec boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<std::s")
}

// 0x2efb4 — __ZN5boost3_bi8storage2INS0_5valueISsEES3_EC2ES3_S3_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<std::string>,boost::_bi::value<std::string>)")]
pub fn stub_0x2efb4() -> ! {
    todo!("0x2efb4 boost::_bi::storage2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<std::string>,boost::_bi::value<std::string>)")
}

// 0x2f1d8 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>)")]
pub fn stub_0x2f1d8() -> ! {
    todo!("0x2f1d8 void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::Sha")
}

// 0x2f2d0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x2f2d0() -> ! {
    todo!("0x2f2d0 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::S")
}

// 0x2f2ec — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_0x2f2ec() -> ! {
    todo!("0x2f2ec boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<")
}

// 0x2f300 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIiEENSE_ISA_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x2f300() -> ! {
    todo!("0x2f300 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_b")
}

// 0x2f3e8 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIiEENSE_ISA_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x2f3e8() -> ! {
    todo!("0x2f3e8 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_b")
}

// 0x2f4fc — __ZN5boost3_bi5list2INS0_5valueIiEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFviS7_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::operator()<void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(int,rbx_core::SharedPtr<RBX::Game>) &,boost::_bi::list0 &,int)")]
pub fn stub_0x2f4fc() -> ! {
    todo!("0x2f4fc void boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::operator()<void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list0>(boost::")
}

// 0x2f5d4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x2f5d4() -> ! {
    todo!("0x2f5d4 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::S")
}

// 0x2f708 — __ZN5boost3_bi5list2INS0_5valueIiEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S8_
#[doc(alias = "boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::list2(boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>)")]
pub fn stub_0x2f708() -> ! {
    todo!("0x2f708 boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::list2(boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>)")
}

// 0x2f8bc — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEEEvT_
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>(boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>)")]
pub fn stub_0x2f8bc() -> ! {
    todo!("0x2f8bc void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::val")
}

// 0x2f9bc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x2f9bc() -> ! {
    todo!("0x2f9bc boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::v")
}

// 0x2f9d8 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_0x2f9d8() -> ! {
    todo!("0x2f9d8 boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,bo")
}

// 0x2f9ec — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS5_5list3INS5_5valueIiEENSF_ISA_EENSF_ISB_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>(boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x2f9ec() -> ! {
    todo!("0x2f9ec bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::valu")
}

// 0x2fad8 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS5_5list3INS5_5valueIiEENSF_ISA_EENSF_ISB_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>(boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x2fad8() -> ! {
    todo!("0x2fad8 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::valu")
}

// 0x2fbf4 — __ZN5boost3_bi5list3INS0_5valueIiEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_I15JoinGameRequestEEEclIPFviS7_S9_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::operator()<void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list0>(boost::_bi::type<void>,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest) &,boost::_bi::list0 &,int)")]
pub fn stub_0x2fbf4() -> ! {
    todo!("0x2fbf4 void boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::operator()<void (*)(int,rbx_core::SharedPtr<RBX")
}

// 0x2fcd4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x2fcd4() -> ! {
    todo!("0x2fcd4 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::v")
}

// 0x2fe0c — __ZN5boost3_bi5list3INS0_5valueIiEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_I15JoinGameRequestEEEC2ES3_S8_SA_
#[doc(alias = "boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::list3(boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>)")]
pub fn stub_0x2fe0c() -> ! {
    todo!("0x2fe0c boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::list3(boost::_bi::value<int>,boost::_bi::value<rbx_c")
}

// 0x2fec4 — __ZN5boost3_bi8storage3INS0_5valueIiEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_I15JoinGameRequestEEEC2ES3_S8_SA_
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::storage3(boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>)")]
pub fn stub_0x2fec4() -> ! {
    todo!("0x2fec4 boost::_bi::storage3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::storage3(boost::_bi::value<int>,boost::_bi::value")
}

// 0x30080 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>)")]
pub fn stub_0x30080() -> ! {
    todo!("0x30080 void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::")
}

// 0x3017c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x3017c() -> ! {
    todo!("0x3017c boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi")
}

// 0x30198 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_0x30198() -> ! {
    todo!("0x30198 boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>")
}

// 0x301ac — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS5_5list3INS5_5valueIiEENSG_IPKcEENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, boost::detail::sp_counted_base *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x301ac() -> ! {
    todo!("0x301ac bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::v")
}

// 0x30298 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS5_5list3INS5_5valueIiEENSG_IPKcEENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x30298() -> ! {
    todo!("0x30298 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::v")
}

// 0x303b8 — __ZN5boost3_bi5list3INS0_5valueIiEENS2_IPKcEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFviRKSsSA_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: int(void)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::operator()<void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>) &,boost::_bi::list0 &,int)")]
pub fn stub_0x303b8() -> ! {
    todo!("0x303b8 void boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::operator()<void (*)(int,std::string const&,rbx_core")
}

// 0x30534 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x30534() -> ! {
    todo!("0x30534 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi")
}

// 0x3066c — __ZN5boost3_bi5list3INS0_5valueIiEENS2_IPKcEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S6_SB_
#[doc(alias = "boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::list3(boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>)")]
pub fn stub_0x3066c() -> ! {
    todo!("0x3066c boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::list3(boost::_bi::value<int>,boost::_bi::value<char cons")
}

// 0x3073c — __ZN5boost6threadC2INS_9function0IvEEEEOT_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::thread::thread<boost::function0<void>>(boost::function0<void> &&)")]
pub fn stub_0x3073c() -> ! {
    todo!("0x3073c boost::thread::thread<boost::function0<void>>(boost::function0<void> &&)")
}

// 0x30878 — __ZN5boost6detail11thread_dataINS_9function0IvEEEC2EOS3_
#[doc(alias = "boost::detail::thread_data<boost::function0<void>>::thread_data(boost::function0<void>&&)")]
pub fn stub_0x30878() -> ! {
    todo!("0x30878 boost::detail::thread_data<boost::function0<void>>::thread_data(boost::function0<void>&&)")
}

// 0x30a24 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>)")]
pub fn stub_0x30a24() -> ! {
    todo!("0x30a24 void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_")
}
