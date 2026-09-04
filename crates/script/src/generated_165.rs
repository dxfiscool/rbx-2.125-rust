// Auto-generated skeletons for rbx-script — Script|Lua|Yield|lua|Luau|CodeGen batch
// Filter: Script|Lua|Yield|lua|Luau|CodeGen (5401 filtered, all 5401 already stubbed) -> global gap filler EA-sorted asc next 150 not yet in script crate
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +150 stubs | range 0x19b60..0x241e8 EA-sorted asc next 150 global not yet in any crate (script 15933 -> 16083 total, global-free 69612->69462)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use std::collections::HashMap;

/// Host-side `AppDelegate` state (AppDelegate.m, IDA 0x19b60..0x1a5d0).
/// UIKit objects (`UIWindow`, `UIAlertView`, `PlaceLauncher`, GCD queues)
/// live on the platform side; only the observable latches are modeled here.
/// Originals stay searchable via the `#[doc(alias)]` lines on each fn.
#[derive(Debug, Default)]
pub struct AppForegroundState {
    /// `NSUserDefaults` `RobloxAppState` suite (IDA 0x19cdc/0x19f7c).
    pub app_state: HashMap<String, String>,
    /// `applicationWillEnterForeground:` ran (IDA 0x19b60).
    pub entered_foreground: bool,
    /// `applicationDidBecomeActive:` run count (IDA 0x19cdc).
    pub became_active_count: u32,
    /// `applicationWillTerminate:` ran (IDA 0x19f7c).
    pub terminated: bool,
    /// `-[LoginManager applicationWillTerminate]` notified (IDA 0x1a054).
    pub login_terminated: bool,
    /// Last `RobloxGoogleAnalytics setPageViewTracking:` page (IDA
    /// 0x19c36/0x1a092).
    pub page_view: Option<String>,
    /// `+[Appirater appEnteredForeground:]` notified (IDA 0x19bf0).
    pub appirater_foreground_notified: bool,
    /// `+[UpgradeCheckHelper checkForUpdate]` requested (IDA 0x19c0e).
    pub upgrade_check_requested: bool,
    /// `-[PlaceLauncher enableViewBecauseGoingToForeground]` (IDA 0x19de0).
    pub place_view_enabled: bool,
    /// `-[SessionReporter reportSessionFor:0]` (IDA 0x19e0a).
    pub session_reported: bool,
    /// `FetchClientSettingsData("iOSAppSettings", ...)` result (IDA 0x19f34).
    pub client_settings_key: Option<String>,
    /// `getiOSSettingsServiceWithForcedReadFromWeb:` flag (IDA 0x19f78).
    pub forced_settings_read: bool,
    /// `appPlaceID` global set by `openURL:` (IDA 0x1a22e), consumed by
    /// `applicationDidBecomeActive:` (IDA 0x19e32..0x19e48).
    pub pending_place_id: Option<i32>,
    /// Places dispatched through `TryLaunchPlace:` (IDA 0x1a234).
    pub launched_places: Vec<i32>,
    /// Last `TryLaunchPlace:` controller dispatch (IDA 0x1a334..0x1a488).
    pub launch_dispatch: Option<LaunchDispatch>,
    /// `bgTask` ivar with acquire/release barriers (IDA 0x1a494/0x1a4a8);
    /// `__dmb` folds into the host field access.
    pub bg_task: u32,
    /// `_window` ivar (IDA 0x1a4c0/0x1a4d0); opaque platform handle.
    pub window: Option<u32>,
    /// `messageOutConnection` scoped-connection live flag (IDA
    /// 0x1a4f4/0x1a5bc; `rbx::signals` -> [`rbx_core::signal::Signal`]).
    pub message_out_connected: bool,
}

/// `TryLaunchPlace:` controller dispatch (IDA 0x1a234..0x1a488).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LaunchDispatch {
    LoginShared { place_id: i32 },
    HomeJump { place_id: i32 },
    StartGame { place_id: i32 },
    GameInProgress { place_id: i32 },
    UnknownController(String),
}

/// `UIViewController` link node for `_topMostController` (IDA 0x1a098).
#[derive(Debug, Clone, Default)]
pub struct ViewControllerNode {
    pub presented: Option<u32>,
    pub is_nav: bool,
    pub visible: Option<u32>,
}

/// `DebugSettingsViewController` state (IDA 0x1a970..0x1ab6c).
#[derive(Debug, Clone, Default)]
pub struct DebugSettingsState {
    /// Settings window frame x/y/w/h (IDA 0x1aa1c/0x1aa76).
    pub window_frame: [f32; 4],
    /// `keyboardOffset = 114` (IDA 0x1aa7a).
    pub keyboard_offset: i32,
    /// `displayPickerArray` (IDA 0x1ab12).
    pub display_options: Vec<String>,
    pub view_loaded: bool,
    /// `GuiBuilder::getDebugDisplay` value behind `setDisplayUI` (IDA 0x1abe6).
    pub debug_display: u32,
    /// Last `setText:` label written by `setDisplayUI` (IDA 0x1ac0c).
    pub display_label: Option<String>,
    /// Picker slide animation end state (IDA 0x1ad78 hides, 0x1afa0 shows).
    pub picker_visible: bool,
    /// `dismissViewControllerAnimated:` ran (IDA 0x1b2b8).
    pub dismissed: bool,
    /// Last `setBounds:` pushed to the superview (IDA 0x1b260..0x1b29c).
    pub last_bounds_set: Option<[f32; 4]>,
}

/// Captured block triple (`picker`, `self`, `toolbar`) retained by the
/// `__copy_helper_block__*` shims and released by the
/// `__destroy_helper_block__*` shims (IDA 0x1ae78..0x1aec6, 0x1b11c..0x1b16a;
/// `_Block_object_assign`/`_Block_object_dispose` -> host ownership).
#[derive(Debug, Clone, Default)]
pub struct DisplayPickerCaptures {
    pub picker: Option<u32>,
    pub owner: Option<u32>,
    pub toolbar: Option<u32>,
}

/// Host-side `HomeViewController` state (HomeViewController.m, IDA
/// 0x1b3d0..0x1c748). UIKit views (`UITextField`, `UIAlertView`, gesture
/// recognizers) live on the platform side; only the observable latches are
/// modeled here.
#[derive(Debug, Clone, Default)]
pub struct HomeViewState {
    /// `handleSignupNotification:` observer installed (IDA 0x1b462..0x1b4a4).
    pub signup_observer: bool,
    /// `preloadDesignatedWebViews` result (IDA 0x1b41a..0x1b42e).
    pub webviews_preloaded: bool,
    /// `designatedWebviewsToHomePages` fallback ran (IDA 0x1b442).
    pub webviews_home_fallback: bool,
    /// Debug `placeId`/`portId`/`ipId` + launcher buttons hidden (IDA
    /// 0x1b7a8..0x1b800).
    pub debug_fields_hidden: bool,
    /// 568h tall background image selected (IDA 0x1b878..0x1b8ec).
    pub tall_background: bool,
    /// Tap-to-dismiss-keyboard recognizer installed (IDA 0x1b914..0x1b97c).
    pub tap_recognizer_installed: bool,
    /// Recognizer enabled flag flipped by keyboard show/hide (IDA
    /// 0x1bbcc/0x1bbec).
    pub tap_recognizer_enabled: bool,
    pub keyboard_visible: bool,
    /// `keyboardDidShow:`/`keyboardDidHide:` observers installed (IDA
    /// 0x1ba04..0x1ba6a).
    pub keyboard_observers: bool,
    /// `dismissKeyboard` resigned the search field (IDA 0x1bc0a).
    pub search_resigned: bool,
    /// Localized label table from `localizeAndStyleLabels` (IDA 0x1bc10).
    pub labels: HashMap<String, String>,
    /// `CFBundleVersion` text stamped in `viewDidLoad` (IDA 0x1ba92..0x1bad2).
    pub version_text: Option<String>,
    /// `updateUserInfoDisplay:` outcome (IDA 0x1bf0c).
    pub user_display: UserDisplay,
    /// `UpdatePlayerInfo` refresh requested (IDA 0x1bf32..0x1bf42).
    pub info_refreshed: bool,
    /// `handleSignupNotification:` credentials (IDA 0x1c2d8..0x1c35c).
    pub login_attempt: Option<(String, String)>,
    /// `logoutTouchUp:` alert shown (IDA 0x1c3a4..0x1c4aa).
    pub logout_alert_shown: bool,
    /// Logout confirmed via alert button 1 (IDA 0x1c4be..0x1c504).
    pub logged_out: bool,
    /// `setPageViewTracking:` page after logout (IDA 0x1c5b4).
    pub logout_page_view: Option<String>,
    /// `buttonView.alpha = 0` animation block ran (IDA 0x1c5c8..0x1c5da).
    pub button_alpha_zero: bool,
    /// Foreground/background initial-X latches from the completion block
    /// (IDA 0x1c626..0x1c712).
    pub foreground_x: Option<f32>,
    pub background_x: Option<f32>,
    /// Completion-block dismiss ran (IDA 0x1c732).
    pub completion_dismissed: bool,
    /// `searchUrl` non-empty at `viewDidLoad` (IDA 0x1bb04..0x1bb14).
    pub search_url_pending: bool,
    /// Search field unhidden by the main-queue block (IDA 0x1bb64).
    pub search_field_visible: bool,
    pub view_loaded: bool,
    pub appeared: bool,
    /// `loggedInView` shown (`notLoggedInView` hidden) by
    /// `showCorrectLoggedInState` (IDA 0x1c7da..0x1c814).
    pub logged_in_shown: bool,
    /// `jumpToPlaceID` global, 0 = none (IDA 0x1d060/0x1d252).
    pub jump_to_place_id: Option<i32>,
    /// `viewMustSegueAfterLoad` global (IDA 0x1c8c0/0x1d244).
    pub segue_after_load: bool,
    /// Last `performSegueWithIdentifier:` (`sequeToWeb`, IDA 0x1c8e0/0x1cac4).
    pub segue_performed: Option<String>,
    /// Last `RobloxAlertWithMessage:` key (IDA 0x1c954/0x1cb3c/0x1cc18).
    pub last_alert: Option<String>,
    /// `placeIdClicked:` launch dispatch (IDA 0x1c95c..0x1ca90).
    pub last_launch: Option<PlaceLaunch>,
    /// Debug `placeId`/`portId`/`ipId` text content (IDA 0x1c986..0x1c9d6).
    pub place_id_text: String,
    pub port_id_text: String,
    pub ip_id_text: String,
    /// Debug fields resigned by `placeIdClicked:` (0x1c9ea..0x1c9fe).
    pub input_resigned: bool,
    /// `dismissViewControllerAnimated:` ran (IDA 0x1cadc).
    pub dismissed: bool,
    /// `prepareForSegue:` resolved web URL / launch id (IDA 0x1d084..0x1d1d0).
    pub web_url: Option<String>,
    pub web_launch_id: Option<i32>,
    /// Outlet handles (`blueFrame`, `imgAvatar`, `lblPlayerName`; IDA
    /// 0x1d258..0x1d2ce).
    pub blue_frame: Option<u32>,
    pub avatar_view: Option<u32>,
    pub player_label: Option<u32>,
    /// Remaining outlet handles by ivar name (`btnGames`, `lblRobux`, ...;
    /// IDA 0x1d390+). Getters return the handle, setters retain via the
    /// `Option` (cf. `objc_setProperty` slots).
    pub outlets: HashMap<String, Option<u32>>,
}

impl HomeViewState {
    /// Outlet getter backing the `-[HomeViewController <name>]` family
    /// (IDA 0x1d390..0x1f494): ivar load (disasm `LDR [R0, ivar]`).
    pub fn outlet(&self, name: &str) -> Option<u32> {
        self.outlets.get(name).copied().flatten()
    }
    /// Outlet setter backing the `-[HomeViewController set<Name>:]` family:
    /// `objc_setProperty` retain/setter (disasm `PUSH/MOV R9, R2` prologue).
    pub fn set_outlet(&mut self, name: &str, view: Option<u32>) {
        self.outlets.insert(name.to_string(), view);
    }
}

/// `placeIdClicked:` launch dispatch (IDA 0x1c95c..0x1ca90).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaceLaunch {
    Local { port: i32, ip: String },
    Remote { place_id: i32 },
}

/// `prepareForSegue:sender:` sender kinds (IDA 0x1d0d8..0x1d1bc).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SegueSender {
    Button(i32),
    SearchField(i32),
    Home,
}

/// `updateUserInfoDisplay:` label/avatar outcome (IDA 0x1bf0c..0x1c130).
#[derive(Debug, Clone, Default)]
pub struct UserDisplay {
    pub robux_text: String,
    pub tix_text: String,
    pub player_name: Option<String>,
    pub avatar_from_url: bool,
    pub avatar_highlighted: bool,
}

/// `UserInfo CurrentPlayer` snapshot feeding `updateUserInfoDisplay:`.
#[derive(Debug, Clone, Default)]
pub struct PlayerInfo {
    pub robux: String,
    pub tix: String,
    pub username: Option<String>,
    pub thumb_url: Option<String>,
}

/// Single retained block capture (`self`) for the `__copy_helper_block_N` /
/// `__destroy_helper_block_N` pairs that move one object (IDA 0x1bb88..0x1bbac,
/// 0x1c5f4..0x1c604, 0x1c734..0x1c744).
#[derive(Debug, Clone, Default)]
pub struct BlockCapture {
    pub target: Option<u32>,
}

/// Host-side `UpgradeCheckHelper` state (UpgradeCheckHelper.m, IDA
/// 0x20e78..0x21ba0). UIKit objects (`UIAlertView`, `NSURLConnection`,
/// GCD queues) live on the platform side; only the observable latches
/// are modeled here.
#[derive(Debug, Clone, Default)]
pub struct UpgradeCheckState {
    /// `init` ran (IDA 0x20f1c).
    pub initialized: bool,
    /// `dealloc` ran (IDA 0x21038).
    pub released: bool,
    /// Alert-view buttons; index 0 is the `UpgradeButtonText` button
    /// added by `init` (IDA 0x20f1c).
    pub buttons: Vec<AlertButton>,
    /// `upgradeResponseData` bytes (IDA 0x20f1c/0x21254/0x21b24).
    pub response_data: Vec<u8>,
    /// `upgradeConnection` handle; `None` = nil (IDA 0x20f1c/0x21254/0x21b58).
    pub connection: Option<u32>,
    /// Next connection handle (host-side `alloc` identity).
    pub next_connection: u32,
    /// Request URL behind the live connection (IDA 0x21254).
    pub connection_request: Option<String>,
    /// `NSLog(@"ItsBloodyNil...")` fired on a nil request (IDA 0x21254).
    pub nil_request_logged: bool,
    /// Alert title/message latched by `processCheckForUpdateResponse`
    /// (IDA 0x214a4).
    pub alert_title: Option<String>,
    pub alert_message: Option<String>,
    /// Main-queue `show` block dispatched (IDA 0x214a4 -> 0x21abc/0x21af0);
    /// the hop is synchronous here.
    pub show_dispatched: bool,
    /// `show` ran on the alert view (IDA 0x21abc/0x21af0).
    pub alert_shown: bool,
    /// JSON parse error text (IDA 0x214a4).
    pub last_parse_error: Option<String>,
    /// Last `RobloxAlertWithMessage:` key (IDA 0x212cc).
    pub last_alert: Option<String>,
    /// App-store URL opened from button 0 (IDA 0x21ba0).
    pub opened_url: Option<String>,
}

/// Alert-view button (`UIButton` subview of the `UIAlertView`, IDA 0x2111c).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlertButton {
    pub title: String,
    pub enabled: bool,
    pub handle: u32,
}

/// `+[UpgradeCheckHelper getUpgradeCheckHelper]` singleton cell (IDA
/// 0x20e78): `dword_130C410` predicate + `dword_130C414` instance.
#[derive(Debug, Clone, Default)]
pub struct UpgradeCheckRegistry {
    /// `dispatch_once` predicate (IDA 0x20e78).
    pub once_token: bool,
    pub helper: UpgradeCheckState,
}

/// C++ static-init cell for `__GLOBAL__I_a_6` (IDA 0x21c18): the
/// `boost::system` category statics, `std::ios_base::Init`, and the
/// `boost::exception_detail` static-exception guards.
#[derive(Debug, Clone, Default)]
pub struct CxxRuntimeA6 {
    pub initialized: bool,
}

/// `iOSSettingsService` reader key (IDA 0x21ce0): the
/// `std::map<std::string, void (*)(char const *)>` value behind each
/// registered setting name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SettingsHandler {
    IPadMinimumVersion,
    IPadMaximumVersion,
    IPhoneMinimumVersion,
    IPhoneMaximumVersion,
    IPodMinimumVersion,
    IPodMaximumVersion,
    DisablePlayButtonForAll,
    DisablePlayButtonForNonBC,
    IPad1MaximumIdealParts,
    IPad2MaximumIdealParts,
    IPad3MaximumIdealParts,
    IPad4MaximumIdealParts,
    IPod4MaximumIdealParts,
    IPod5MaximumIdealParts,
    IPhone4sMaximumIdealParts,
    IPhone5MaximumIdealParts,
    TimeIntervalBetweenRobuxPurchaseInMinutes,
    TimeIntervalBetweenBCPurchaseInMinutes,
    TimeIntervalBetweenCatalogPurchaseInMinutes,
    TimeLimitForBillingServiceRetriesBeforeGivingUp,
    TestFlightLoggingLevel,
    TestFlightPercentage,
    BugSensePercentage,
    BugSenseLogLines,
    BugSenseLogLevel,
    IOSGoogleAnalyticsAccount2,
    IOSGoogleAnalyticsSampleRate,
    SearchEndpointIPad,
    SearchEndpointIPhone,
    CacheUIWebViews,
    ThumbstickControlStyle,
    FreeMemoryCheckerActive,
    FreeMemoryCheckerRateMilliSeconds,
    FreeMemoryCheckerThresholdKiloBytes,
    MemoryBouncerActive,
    MemoryBouncerEnforceRateMilliSeconds,
    MemoryBouncerThresholdKiloBytes,
    MemoryBouncerLimitMegaBytes,
    MemoryBouncerLimitMegaBytesForLowMemDevices,
}

/// Host-side `iOSSettingsService` (IDA 0x21ce0..): `var1` is the
/// name -> reader map, `var3` the init flag, `+44` a trailing word
/// zeroed by `Init`. Value fields mirror the `_thisPtr` layout the
/// `ReadValue*` family writes through (IDA 0x239ec..0x24024).
#[derive(Debug, Clone, Default)]
pub struct IosSettingsService {
    /// `this->var3 = 1` (IDA 0x21ce0).
    pub initialized: bool,
    /// `this->var1` reader map (IDA 0x21ce0).
    pub handlers: HashMap<String, SettingsHandler>,
    /// `*(this + 44) = 0` (IDA 0x21ce0).
    pub reserved_44: u32,
    /// `_thisPtr + 28` (IDA 0x239ec).
    pub ipad_minimum_version: i32,
    /// `_thisPtr + 32` (IDA 0x23b50).
    pub ipad_maximum_version: i32,
    /// `_thisPtr + 36` (IDA 0x23b68).
    pub iphone_minimum_version: i32,
    /// `_thisPtr + 40` (IDA 0x23b80).
    pub iphone_maximum_version: i32,
    /// `_thisPtr + 44` (IDA 0x23b98).
    pub ipod_minimum_version: i32,
    /// `_thisPtr + 48` (IDA 0x23bb0).
    pub ipod_maximum_version: i32,
    /// `_thisPtr + 52` (IDA 0x23bc8).
    pub disable_play_button_for_all: bool,
    /// `_thisPtr + 53` (IDA 0x23be4).
    pub disable_play_button_for_non_bc: bool,
    /// `_thisPtr + 56` (IDA 0x23c00).
    pub ipad1_maximum_ideal_parts: i32,
    /// `_thisPtr + 60` (IDA 0x23c18).
    pub ipad2_maximum_ideal_parts: i32,
    /// `_thisPtr + 64` (IDA 0x23c30).
    pub ipad3_maximum_ideal_parts: i32,
    /// `_thisPtr + 68` (IDA 0x23c48).
    pub ipad4_maximum_ideal_parts: i32,
    /// `_thisPtr + 72` (IDA 0x23c60).
    pub ipod4_maximum_ideal_parts: i32,
    /// `_thisPtr + 76` (IDA 0x23c78).
    pub ipod5_maximum_ideal_parts: i32,
    /// `_thisPtr + 80` (IDA 0x23c90).
    pub iphone4s_maximum_ideal_parts: i32,
    /// `_thisPtr + 84` (IDA 0x23ca8).
    pub iphone5_maximum_ideal_parts: i32,
    /// `_thisPtr + 88` (IDA 0x23d9c).
    pub google_analytics_account2: String,
    /// `_thisPtr + 92` (IDA 0x23ed4).
    pub google_analytics_sample_rate: i32,
    /// `_thisPtr + 96` (IDA 0x23cc0).
    pub interval_robux_purchase_minutes: i32,
    /// `_thisPtr + 100` (IDA 0x23cd8).
    pub interval_bc_purchase_minutes: i32,
    /// `_thisPtr + 104` (IDA 0x23cf0).
    pub interval_catalog_purchase_minutes: i32,
    /// `_thisPtr + 108` (IDA 0x23d08).
    pub billing_retry_limit: i32,
    /// `_thisPtr + 112` (IDA 0x23d20).
    pub testflight_logging_level: i32,
    /// `_thisPtr + 116` (IDA 0x23d38).
    pub testflight_percentage: i32,
    /// `_thisPtr + 120` (IDA 0x23d50).
    pub bugsense_percentage: i32,
    /// `_thisPtr + 124` (IDA 0x23d68).
    pub bugsense_log_lines: i32,
    /// `_thisPtr + 128` (IDA 0x23d80).
    pub bugsense_log_level: i32,
    /// `_thisPtr + 132` (IDA 0x23eec).
    pub search_endpoint_ipad: String,
    /// `_thisPtr + 136` (IDA 0x24024).
    pub search_endpoint_iphone: String,
    /// `_thisPtr + 140` (IDA 0x2415c).
    pub cache_ui_web_views: bool,
    /// `_thisPtr + 144` (IDA 0x24178).
    pub thumbstick_control_style: i32,
    /// `_thisPtr + 148` (IDA 0x24194).
    pub free_memory_checker_active: bool,
    /// `_thisPtr + 152` (IDA 0x241b0).
    pub free_memory_checker_rate_ms: i32,
    /// `_thisPtr + 156` (IDA 0x241cc).
    pub free_memory_checker_threshold_kb: i32,
    /// `_thisPtr + 160` (IDA 0x241e8).
    pub memory_bouncer_active: bool,
    /// `_thisPtr + 164` (IDA 0x24204).
    pub memory_bouncer_enforce_rate_ms: i32,
    /// `_thisPtr + 168` (IDA 0x24220).
    pub memory_bouncer_threshold_kb: i32,
    /// `_thisPtr + 172` (IDA 0x2423c).
    pub memory_bouncer_limit_mb: i32,
    /// `_thisPtr + 176` (IDA 0x24258).
    pub memory_bouncer_limit_lowmem_mb: i32,
}

/// Reads a `"key": <string | null>` value from a flat JSON object
/// (IDA 0x214a4 `NSJSONSerialization` host model). Outer `None` = key
/// absent or a non-string scalar (no usable value); `Some(None)` = `null`.
fn json_string_or_null(s: &str, key: &str) -> Option<Option<String>> {
    let quoted = format!("\"{key}\"");
    let mut from = 0usize;
    while let Some(rel) = s.get(from..)?.find(quoted.as_str()) {
        let mut pos = from + rel + quoted.len();
        while matches!(s.get(pos..pos + 1), Some(" " | "\t" | "\n" | "\r")) {
            pos += 1;
        }
        if s.get(pos..pos + 1) != Some(":") {
            from = pos + 1;
            continue;
        }
        pos += 1;
        while matches!(s.get(pos..pos + 1), Some(" " | "\t" | "\n" | "\r")) {
            pos += 1;
        }
        let rest = s.get(pos..)?;
        if rest.starts_with("null") {
            return Some(None);
        }
        if rest.starts_with('"') {
            let mut out = String::new();
            let mut chars = rest[1..].chars();
            loop {
                match chars.next() {
                    None => return None,
                    Some('\\') => out.push(chars.next()?),
                    Some('"') => return Some(Some(out)),
                    Some(c) => out.push(c),
                }
            }
        }
        return None;
    }
    None
}

/// Parsed `processCheckForUpdateResponse` payload (IDA 0x214a4): the
/// `data` dictionary's `UpgradeAction`/`Message` pair. `Ok(None)` = no
/// actionable upgrade (`data`/`UpgradeAction` null or absent);
/// `Err(())` = `NSJSONSerialization` failure.
fn upgrade_action_and_message(json: &str) -> Result<Option<(String, Option<String>)>, ()> {
    let text = json.trim();
    if !text.starts_with('{') || !text.ends_with('}') {
        return Err(());
    }
    match json_string_or_null(text, "UpgradeAction") {
        None | Some(None) => Ok(None),
        Some(Some(action)) => Ok(Some((action, json_string_or_null(text, "Message").flatten()))),
    }
}

// 0x19b60 — -[AppDelegate applicationWillEnterForeground:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationWillEnterForeground:]")]
pub fn stub_0x19b60(state: &mut AppForegroundState) {
    // IDA 0x19b60 `-[AppDelegate applicationWillEnterForeground:]`:
    // StandardOut begin/end markers (0x19b7e..0x19c54; `SharedPtr` release
    // folds into host ownership), `+[Appirater appEnteredForeground:1]`
    // (0x19bf0), `+[UpgradeCheckHelper checkForUpdate]` (0x19c0e),
    // `setPageViewTracking:@"RobloxApp/EnterForeGround"` (0x19c36).
    state.entered_foreground = true;
    state.appirater_foreground_notified = true;
    state.upgrade_check_requested = true;
    state.page_view = Some("RobloxApp/EnterForeGround".to_string());
}

// 0x19cdc — -[AppDelegate applicationDidBecomeActive:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationDidBecomeActive:]")]
pub fn stub_0x19cdc(state: &mut AppForegroundState, top_controller: &str) {
    // IDA 0x19cdc `-[AppDelegate applicationDidBecomeActive:]`: stores
    // `tryForeground` under `RobloxAppState` + synchronize (0x19d16..0x19d56),
    // begin marker, `enableViewBecauseGoingToForeground` (0x19de0),
    // `reportSessionFor:0` (0x19e0a), async settings block (0x19e22 ->
    // 0x19f34), pending `appPlaceID` launch (0x19e32..0x19e48), end marker,
    // then stores `inApp` + synchronize (0x19e8a..0x19eb8).
    state.app_state.insert("RobloxAppState".to_string(), "tryForeground".to_string());
    state.became_active_count += 1;
    state.place_view_enabled = true;
    state.session_reported = true;
    stub_0x19f34(state);
    if let Some(place_id) = state.pending_place_id.take() {
        stub_0x1a234(state, place_id, top_controller);
    }
    state.app_state.insert("RobloxAppState".to_string(), "inApp".to_string());
}

// 0x19f34 — ___42-[AppDelegate applicationDidBecomeActive:]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___42-[AppDelegate applicationDidBecomeActive:]_block_invoke")]
pub fn stub_0x19f34(state: &mut AppForegroundState) {
    // IDA 0x19f34 `__42-[AppDelegate applicationDidBecomeActive:]_block_invoke`:
    // `ClientAppSettings::Initialize` + singleton (0x19f38..0x19f3c),
    // `FetchClientSettingsData("iOSAppSettings",
    // "D6925E56-...")` (0x19f56),
    // `getiOSSettingsServiceWithForcedReadFromWeb:0` (0x19f78).
    state.client_settings_key = Some("iOSAppSettings".to_string());
    state.forced_settings_read = false;
}

// 0x19f7c — -[AppDelegate applicationWillTerminate:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationWillTerminate:]")]
pub fn stub_0x19f7c(state: &mut AppForegroundState) {
    // IDA 0x19f7c `-[AppDelegate applicationWillTerminate:]`: logs
    // `RobloxGameState`/`RobloxAppState` (0x19fa0..0x19ff8), stores
    // `terminated` + synchronize (0x1a002..0x1a038), `-[LoginManager
    // applicationWillTerminate]` (0x1a064),
    // `setPageViewTracking:@"RobloxApp/Exit"` (0x1a092).
    state.app_state.insert("RobloxAppState".to_string(), "terminated".to_string());
    state.terminated = true;
    state.login_terminated = true;
    state.page_view = Some("RobloxApp/Exit".to_string());
}

// 0x1a098 — __Z18_topMostControllerP16UIViewController
// type: id __fastcall(id)
#[doc(alias = "_topMostController(UIViewController *)")]
pub fn stub_0x1a098(table: &HashMap<u32, ViewControllerNode>, start: u32) -> Option<u32> {
    // IDA 0x1a098 `_topMostController`: walk `presentedViewController`
    // chain to the end (0x1a0ae..0x1a0c4), then unwrap
    // `UINavigationController` via `visibleViewController` (0x1a0e4..0x1a118);
    // returns 0 when the walk never leaves `start` (0x1a11c..0x1a11e),
    // i.e. `None` on the host.
    let mut top = start;
    if table.get(&start).and_then(|n| n.presented).is_some() {
        let mut cur = start;
        while let Some(next) = table.get(&cur).and_then(|n| n.presented) {
            cur = next;
        }
        top = cur;
    }
    if table.get(&top).is_some_and(|n| n.is_nav) {
        if let Some(visible) = table.get(&top).and_then(|n| n.visible) {
            top = visible;
        }
    }
    (top != start).then_some(top)
}

// 0x1a124 — __Z17topMostControllerv
// type: _DWORD __fastcall()
#[doc(alias = "topMostController(void)")]
pub fn stub_0x1a124(root: u32, table: &HashMap<u32, ViewControllerNode>) -> u32 {
    // IDA 0x1a124 `topMostController`: `keyWindow.rootViewController`
    // (0x1a140..0x1a160), then loop `_topMostController` (0x1a166, cf.
    // 0x1a098) until it returns 0; yields the last controller seen.
    let mut top = root;
    while let Some(next) = stub_0x1a098(table, top) {
        top = next;
    }
    top
}

// 0x1a174 — -[AppDelegate application:openURL:sourceApplication:annotation:]
// type: char __cdecl(AppDelegate *self, SEL, id, id, id, id)
#[doc(alias = "-[AppDelegate application:openURL:sourceApplication:annotation:]")]
pub fn stub_0x1a174(state: &mut AppForegroundState, url: &str) -> bool {
    // IDA 0x1a174 `-[AppDelegate application:openURL:...]`: logs
    // url/source/annotation (0x1a18a); returns 0 unless the absolute string
    // has the `robloxmobile` prefix (0x1a19c..0x1a1bc). On match, logs host
    // (0x1a1d6) and path (0x1a1f8), stores `host.intValue` into `appPlaceID`
    // (0x1a210..0x1a22e) and returns 1.
    let Some(rest) = url.strip_prefix("robloxmobile") else {
        return false;
    };
    let host = rest.strip_prefix("://").unwrap_or(rest).split('/').next().unwrap_or("");
    if let Ok(place_id) = host.parse::<i32>() {
        state.pending_place_id = Some(place_id);
    }
    true
}

// 0x1a234 — -[AppDelegate TryLaunchPlace:]
// type: void __cdecl(AppDelegate *self, SEL, int)
#[doc(alias = "-[AppDelegate TryLaunchPlace:]")]
pub fn stub_0x1a234(state: &mut AppForegroundState, place_id: i32, top_controller: &str) {
    // IDA 0x1a234 `-[AppDelegate TryLaunchPlace:]`: logs root/top view
    // names (0x1a24c..0x1a316), then dispatches on the top controller class:
    // `LoginViewController` -> sharedInstance `setLoginPlaceId:`
    // (0x1a334..0x1a372); `HomeViewController` -> `setJumpToPlaceID:` +
    // `buttonForWebDidTouchUpInside:` (0x1a386..0x1a3c0);
    // `RobloxNavBarViewController` -> `startGame:...` (0x1a3de..0x1a42a);
    // `GameViewController` -> `setJumpToPlaceIDGameInProgress:`
    // (0x1a43e..0x1a47c); otherwise logs unknown (0x1a488).
    state.launched_places.push(place_id);
    state.launch_dispatch = Some(match top_controller {
        "LoginViewController" => LaunchDispatch::LoginShared { place_id },
        "HomeViewController" => LaunchDispatch::HomeJump { place_id },
        "RobloxNavBarViewController" => LaunchDispatch::StartGame { place_id },
        "GameViewController" => LaunchDispatch::GameInProgress { place_id },
        other => LaunchDispatch::UnknownController(other.to_string()),
    });
}

// 0x1a494 — -[AppDelegate bgTask]
// type: unsigned int __cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate bgTask]")]
pub fn stub_0x1a494(state: &AppForegroundState) -> u32 {
    // IDA 0x1a494 `-[AppDelegate bgTask]`: atomic load of the ivar with a
    // full barrier (0x1a4a0..0x1a4a6); host field access is ordered.
    state.bg_task
}

// 0x1a4a8 — -[AppDelegate setBgTask:]
// type: void __cdecl(AppDelegate *self, SEL, unsigned int)
#[doc(alias = "-[AppDelegate setBgTask:]")]
pub fn stub_0x1a4a8(state: &mut AppForegroundState, task: u32) {
    // IDA 0x1a4a8 `-[AppDelegate setBgTask:]`: barrier + ivar store +
    // barrier (0x1a4b0..0x1a4ba); host field store is ordered.
    state.bg_task = task;
}

// 0x1a4c0 — -[AppDelegate window]
// type: UIWindow *__cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate window]")]
pub fn stub_0x1a4c0(state: &AppForegroundState) -> Option<u32> {
    // IDA 0x1a4c0 `-[AppDelegate window]`: returns the `_window` ivar
    // (0x1a4ce); opaque platform handle on the host.
    state.window
}

// 0x1a4d0 — -[AppDelegate setWindow:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate setWindow:]")]
pub fn stub_0x1a4d0(state: &mut AppForegroundState, window: Option<u32>) {
    // IDA 0x1a4d0 `-[AppDelegate setWindow:]`: `objc_setProperty`
    // retain/setter for slot 12 (0x1a4ec); host ownership is the `Option`.
    state.window = window;
}

// 0x1a4f4 — -[AppDelegate .cxx_destruct]
// type: void __cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate .cxx_destruct]")]
pub fn stub_0x1a4f4(state: &mut AppForegroundState) {
    // IDA 0x1a4f4 `-[AppDelegate .cxx_destruct]`:
    // `connection::disconnect(&messageOutConnection)` (0x1a552) plus the
    // weak-slot release (0x1a558..0x1a560); the latch going false is both.
    state.message_out_connected = false;
}

// 0x1a5bc — -[AppDelegate .cxx_construct]
// type: id __cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate .cxx_construct]")]
pub fn stub_0x1a5bc(state: &mut AppForegroundState) {
    // IDA 0x1a5bc `-[AppDelegate .cxx_construct]`: nulls the
    // `messageOutConnection` weak slot (0x1a5ca) and returns self.
    state.message_out_connected = false;
}

// 0x1a5d0 — __GLOBAL__I_a_1
#[doc(alias = "global constructor keyed to_a_1")]
pub fn stub_0x1a5d0() {
    // IDA 0x1a5d0 `__GLOBAL__I_a_1`: static init storing
    // `boost::system::generic_category()`/`system_category()` singletons
    // into merged globals (disasm PUSH/R4-R7 + three BL category calls;
    // cf. 0x16e4c). Host error categories need no init beyond `std::io`.
}

// 0x1a768 — _main
// type: int __fastcall(int argc, const char **argv, const char **envp)
#[doc(alias = "_main")]
pub fn stub_0x1a768() -> i32 {
    // IDA 0x1a768 `_main`: `NSAutoreleasePool` alloc/init (0x1a788..0x1a7a0),
    // `UIApplicationMain(argc, argv, @"UIApplication", @"AppDelegate")`
    // (0x1a7ba), pool release, return status (0x1a7ca..0x1a7d0). The UIKit
    // runloop owns the loop on-device; the host reports clean exit.
    0
}

// 0x1a7d4 — __GLOBAL__I_a_2
#[doc(alias = "global constructor keyed to_a_2")]
pub fn stub_0x1a7d4() {
    // IDA 0x1a7d4 `__GLOBAL__I_a_2`: same `generic_category` x2 +
    // `system_category` merged-globals init as 0x1a5d0 (cf. 0x16e4c). Host
    // error categories need no init beyond `std::io`.
}

// 0x1a970 — -[DebugSettingsViewController initWithCoder:]
// type: DebugSettingsViewController *__cdecl(DebugSettingsViewController *self, SEL, id)
#[doc(alias = "-[DebugSettingsViewController initWithCoder:]")]
pub fn stub_0x1a970(is_pad: bool, screen_bounds: [f32; 4]) -> DebugSettingsState {
    // IDA 0x1a970 `-[DebugSettingsViewController initWithCoder:]`: super
    // init (0x1a98e..0x1a99c); on iPad (`userInterfaceIdiom != 0`,
    // 0x1a9f4) the frame is 540x508 at origin (0x1a1c..0x1a1e), otherwise
    // the main-screen bounds (0x1a4e..0x1a76). Either way
    // `keyboardOffset = 114` (0x1aa7a) and `displayPickerArray =
    // [None, FPS, Summary, Physics, PhysicsAndOwner, Render]` (0x1aaa2..0x1ab12).
    DebugSettingsState {
        window_frame: if is_pad { [0.0, 0.0, 540.0, 508.0] } else { screen_bounds },
        keyboard_offset: 114,
        display_options: ["None", "FPS", "Summary", "Physics", "PhysicsAndOwner", "Render"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        view_loaded: false,
        debug_display: 0,
        display_label: None,
        picker_visible: false,
        dismissed: false,
        last_bounds_set: None,
    }
}

// 0x1ab20 — -[DebugSettingsViewController dealloc]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController dealloc]")]
pub fn stub_0x1ab20(state: &mut DebugSettingsState) {
    // IDA 0x1ab20 `-[DebugSettingsViewController dealloc]`: releases
    // `displayPickerArray` (0x1ab42), then super dealloc (0x1ab5a..0x1ab64,
    // host Drop glue).
    state.display_options.clear();
}

// 0x1ab6c — -[DebugSettingsViewController reloadOldData]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController reloadOldData]")]
pub fn stub_0x1ab6c() {
    // IDA 0x1ab6c `-[DebugSettingsViewController reloadOldData]`: empty
    // body — no-op.
}

// 0x1ab70 — -[DebugSettingsViewController viewDidLoad]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController viewDidLoad]")]
pub fn stub_0x1ab70(state: &mut DebugSettingsState) {
    // IDA 0x1ab70 `-[DebugSettingsViewController viewDidLoad]`: super
    // viewDidLoad (0x1ab8c..0x1ab96, host UIKit) then `reloadOldData`
    // (0x1aba8 -> 0x1ab6c, no-op).
    stub_0x1ab6c();
    state.view_loaded = true;
}

// 0x1abb0 — -[DebugSettingsViewController setDisplayUI]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController setDisplayUI]")]
pub fn stub_0x1abb0(state: &mut DebugSettingsState, has_label: bool) {
    // IDA 0x1abb0 `-[DebugSettingsViewController setDisplayUI]`:
    // `viewWithTag:100` (0x1abd2); when present, `getDebugDisplay` switch
    // (0x1abe6) maps 1..5 to FPS/Summary/Physics/PhysicsAndOwner/Render and
    // everything else to None (0x1ac02), written via `setText:` (0x1ac0c).
    if has_label {
        state.display_label = Some(
            match state.debug_display {
                1 => "FPS",
                2 => "Summary",
                3 => "Physics",
                4 => "PhysicsAndOwner",
                5 => "Render",
                _ => "None",
            }
            .to_string(),
        );
    }
}

// 0x1ac80 — -[DebugSettingsViewController displayPickerDoneClicked:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, id)
#[doc(alias = "-[DebugSettingsViewController displayPickerDoneClicked:]")]
pub fn stub_0x1ac80(
    state: &mut DebugSettingsState,
    picker_present: bool,
    toolbar_present: bool,
    selected_row: i32,
) {
    // IDA 0x1ac80 `-[DebugSettingsViewController displayPickerDoneClicked:]`:
    // `viewWithTag:5012` (picker) / `viewWithTag:5011` (toolbar)
    // (0x1ac9c..0x1acc6); when both exist, run the dismiss animation block
    // (0x1ad0a..0x1ad34 -> 0x1ad78) and, if `selectedRowInComponent:0 >= 0`
    // (0x1ad4e), `setDebugDisplay` (0x1ad50). Always finishes with
    // `setDisplayUI` (0x1ad62).
    if picker_present && toolbar_present {
        stub_0x1ad78(state);
        if selected_row >= 0 {
            state.debug_display = selected_row as u32;
        }
    }
    stub_0x1abb0(state, true);
}

// 0x1ad78 — ___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke")]
pub fn stub_0x1ad78(state: &mut DebugSettingsState) {
    // IDA 0x1ad78 `__56-[...displayPickerDoneClicked:]_block_invoke`:
    // slide-down animation recomputing picker/toolbar `setFrame:` values
    // from the window-height ivar (0x1ad90..0x1ae74). Host UIKit owns the
    // frames; the observable end state is the picker hidden.
    state.picker_visible = false;
}

// 0x1ae78 — ___copy_helper_block__0
// type: void __fastcall(int, const void **)
#[doc(alias = "___copy_helper_block__0")]
pub fn stub_0x1ae78(dst: &mut DisplayPickerCaptures, src: &DisplayPickerCaptures) {
    // IDA 0x1ae78 `__copy_helper_block__0`: `_Block_object_assign` retain
    // of the three captures (0x1ae88..0x1aea4; cf. 0x18c8c).
    *dst = src.clone();
}

// 0x1aea8 — ___destroy_helper_block__0
#[doc(alias = "___destroy_helper_block__0")]
pub fn stub_0x1aea8(slot: &mut DisplayPickerCaptures) {
    // IDA 0x1aea8 `__destroy_helper_block__0`: `_Block_object_dispose`
    // release of the three captures (0x1aeb2..0x1aec6; cf. 0x18c98).
    *slot = DisplayPickerCaptures::default();
}

// 0x1aed0 — -[DebugSettingsViewController displayTouchUp:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, id)
#[doc(alias = "-[DebugSettingsViewController displayTouchUp:]")]
pub fn stub_0x1aed0(state: &mut DebugSettingsState, picker_present: bool, toolbar_present: bool) {
    // IDA 0x1aed0 `-[DebugSettingsViewController displayTouchUp:]`:
    // `viewWithTag:5012` (picker) / `viewWithTag:5011` (toolbar)
    // (0x1aeec..0x1af16); when both exist, run the show animation block
    // (0x1af5c..0x1af86 -> 0x1afa0).
    if picker_present && toolbar_present {
        stub_0x1afa0(state);
    }
}

// 0x1afa0 — ___46-[DebugSettingsViewController displayTouchUp:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___46-[DebugSettingsViewController displayTouchUp:]_block_invoke")]
pub fn stub_0x1afa0(state: &mut DebugSettingsState) {
    // IDA 0x1afa0 `__46-[...displayTouchUp:]_block_invoke`: slide-up
    // animation recomputing picker/toolbar `setFrame:` values with the
    // `vsub_f32` window-height math (0x1afec..0x1b11a). Host UIKit owns the
    // frames; the observable end state is the picker shown.
    state.picker_visible = true;
}

// 0x1b11c — ___copy_helper_block_66
#[doc(alias = "___copy_helper_block_66")]
pub fn stub_0x1b11c(dst: &mut DisplayPickerCaptures, src: &DisplayPickerCaptures) {
    // IDA 0x1b11c `__copy_helper_block_66`: `_Block_object_assign` retain
    // of the three captures (0x1b12c..0x1b148; cf. 0x1ae78).
    *dst = src.clone();
}

// 0x1b14c — ___destroy_helper_block_67
#[doc(alias = "___destroy_helper_block_67")]
pub fn stub_0x1b14c(slot: &mut DisplayPickerCaptures) {
    // IDA 0x1b14c `__destroy_helper_block_67`: `_Block_object_dispose`
    // release of the three captures (0x1b156..0x1b16a; cf. 0x1aea8).
    *slot = DisplayPickerCaptures::default();
}

// 0x1b170 — -[DebugSettingsViewController didReceiveMemoryWarning]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController didReceiveMemoryWarning]")]
pub fn stub_0x1b170() {
    // IDA 0x1b170 `-[DebugSettingsViewController didReceiveMemoryWarning]`:
    // super call only (0x1b18a..0x1b194, host UIKit) — no-op.
}

// 0x1b19c — -[DebugSettingsViewController shouldAutorotateToInterfaceOrientation:]
// type: char __cdecl(DebugSettingsViewController *self, SEL, int)
#[doc(alias = "-[DebugSettingsViewController shouldAutorotateToInterfaceOrientation:]")]
pub fn stub_0x1b19c(orientation: i32, is_pad: bool) -> bool {
    // IDA 0x1b19c `-[DebugSettingsViewController
    // shouldAutorotateToInterfaceOrientation:]`: with `userInterfaceIdiom`
    // support (always true on the host), iPad allows every orientation
    // except portrait (0x1b1fa..0x1b212: `a3 != 1`), iPhone allows portrait
    // only (0x1b204..0x1b222: `a3 == 1`).
    if is_pad {
        orientation != 1
    } else {
        orientation == 1
    }
}

// 0x1b224 — -[DebugSettingsViewController viewWillAppear:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, char)
#[doc(alias = "-[DebugSettingsViewController viewWillAppear:]")]
pub fn stub_0x1b224(state: &mut DebugSettingsState, animated: bool) {
    // IDA 0x1b224 `-[DebugSettingsViewController viewWillAppear:]`: super
    // call (0x1b244..0x1b24e, host UIKit), then
    // `superview.setBounds(window.frame)` (0x1b260..0x1b29c).
    let _ = animated;
    state.last_bounds_set = Some(state.window_frame);
}

// 0x1b2a8 — -[DebugSettingsViewController doneTouchUp:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, id)
#[doc(alias = "-[DebugSettingsViewController doneTouchUp:]")]
pub fn stub_0x1b2a8(state: &mut DebugSettingsState) {
    // IDA 0x1b2a8 `-[DebugSettingsViewController doneTouchUp:]`:
    // `dismissViewControllerAnimated:1 completion:0` (0x1b2b8, host UIKit).
    state.dismissed = true;
}

// 0x1b2bc — -[DebugSettingsViewController numberOfComponentsInPickerView:]
// type: int __cdecl(DebugSettingsViewController *self, SEL, id)
#[doc(alias = "-[DebugSettingsViewController numberOfComponentsInPickerView:]")]
pub fn stub_0x1b2bc() -> i32 {
    // IDA 0x1b2bc `-[DebugSettingsViewController
    // numberOfComponentsInPickerView:]`: returns 1 (0x1b2be).
    1
}

// 0x1b2c0 — -[DebugSettingsViewController pickerView:numberOfRowsInComponent:]
// type: int __cdecl(DebugSettingsViewController *self, SEL, id, int)
#[doc(alias = "-[DebugSettingsViewController pickerView:numberOfRowsInComponent:]")]
pub fn stub_0x1b2c0(state: &DebugSettingsState) -> i32 {
    // IDA 0x1b2c0 `-[DebugSettingsViewController
    // pickerView:numberOfRowsInComponent:]`: `[displayPickerArray count]`
    // (0x1b2c4).
    state.display_options.len() as i32
}

// 0x1b2e0 — -[DebugSettingsViewController pickerView:titleForRow:forComponent:]
// type: id __cdecl(DebugSettingsViewController *self, SEL, id, int, int)
#[doc(alias = "-[DebugSettingsViewController pickerView:titleForRow:forComponent:]")]
pub fn stub_0x1b2e0(state: &DebugSettingsState, row: usize) -> Option<String> {
    // IDA 0x1b2e0 `-[DebugSettingsViewController
    // pickerView:titleForRow:forComponent:]`: `[displayPickerArray
    // objectAtIndex:]` (0x1b2e4); host returns `None` out of range instead
    // of raising.
    state.display_options.get(row).cloned()
}

// 0x1b300 — -[DebugSettingsViewController disablesAutomaticKeyboardDismissal]
// type: char __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController disablesAutomaticKeyboardDismissal]")]
pub fn stub_0x1b300() -> bool {
    // IDA 0x1b300 `-[DebugSettingsViewController
    // disablesAutomaticKeyboardDismissal]`: returns 0 (0x1b302).
    false
}

// 0x1b304 — -[DebugSettingsViewController .cxx_construct]
// type: id __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController .cxx_construct]")]
pub fn stub_0x1b304() -> DebugSettingsState {
    // IDA 0x1b304 `-[DebugSettingsViewController .cxx_construct]`:
    // returns self with no ivar init (0x1b304); the host equivalent is a
    // default state value.
    DebugSettingsState::default()
}

// 0x1b308 — __GLOBAL__I_a_3
#[doc(alias = "global constructor keyed to_a_3")]
pub fn stub_0x1b308() {
    // IDA 0x1b308 `__GLOBAL__I_a_3`: same `generic_category` x2 +
    // `system_category` merged-globals init plus `ios_base::Init` +
    // `__cxa_atexit` as 0x1a5d0 (disasm 0x1b308..0x1b35a; cf. 0x16e4c).
    // Host error categories need no init beyond `std::io`.
}

// 0x1b3d0 — -[HomeViewController initWithCoder:]
// type: HomeViewController *__cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController initWithCoder:]")]
pub fn stub_0x1b3d0(preload_ok: bool) -> HomeViewState {
    // IDA 0x1b3d0 `-[HomeViewController initWithCoder:]`: super
    // `RobloxAnimatingPageViewController` init (0x1b3ea..0x1b3f8, always
    // succeeds on the host); `preloadDesignatedWebViews` (0x1b41a..0x1b42e)
    // with the `designatedWebviewsToHomePages` fallback (0x1b442), then the
    // `handleSignupNotification:` observer (0x1b462..0x1b4a4).
    HomeViewState {
        webviews_preloaded: preload_ok,
        webviews_home_fallback: !preload_ok,
        signup_observer: true,
        ..HomeViewState::default()
    }
}

// 0x1b4b0 — -[HomeViewController dealloc]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController dealloc]")]
pub fn stub_0x1b4b0(state: &mut HomeViewState) {
    // IDA 0x1b4b0 `-[HomeViewController dealloc]`: releases the ~30 outlets
    // (`tapRecognizer`, `_placeId`/`_portId`/`_ipId`, `_imgAvatar`, labels,
    // buttons, `blueFrame`, search/logged-in views, text views, version —
    // 0x1b4d4..0x1b730) then super dealloc (0x1b748..0x1b752, host Drop
    // glue). The owned state folds back to default.
    *state = HomeViewState::default();
}

// 0x1b75c — -[HomeViewController viewDidLoad]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController viewDidLoad]")]
pub fn stub_0x1b75c(
    state: &mut HomeViewState,
    tall_screen: bool,
    bundle_version: &str,
    search_url_nonempty: bool,
    player: &PlayerInfo,
) {
    // IDA 0x1b75c `-[HomeViewController viewDidLoad]`: super (0x1b77c..0x1b786),
    // hide debug fields (0x1b7a8..0x1b800), 568h background on tall phones
    // (0x1b820..0x1b8ec), tap recognizer installed disabled (0x1b914..0x1b97c),
    // `localizeAndStyleLabels` (0x1b98e), `updateUserInfoDisplay:NO`
    // (0x1b9a2), async search block (0x1b9ac..0x1b9e4 -> 0x1bae4),
    // keyboard observers (0x1ba04..0x1ba6a), `CFBundleVersion` stamp
    // (0x1ba92..0x1bad2).
    state.debug_fields_hidden = true;
    state.tall_background = tall_screen;
    state.tap_recognizer_installed = true;
    state.tap_recognizer_enabled = false;
    stub_0x1bc10(state);
    stub_0x1bf0c(state, false, player);
    stub_0x1bae4(state, search_url_nonempty);
    state.keyboard_observers = true;
    state.version_text = Some(bundle_version.to_string());
    state.view_loaded = true;
}

// 0x1bae4 — ___33-[HomeViewController viewDidLoad]_block_invoke
#[doc(alias = "___33-[HomeViewController viewDidLoad]_block_invoke")]
pub fn stub_0x1bae4(state: &mut HomeViewState, search_url_nonempty: bool) {
    // IDA 0x1bae4 `__33-[HomeViewController viewDidLoad]_block_invoke`:
    // when `+[RobloxInfo searchUrl]` is non-empty (0x1bb04..0x1bb14), hop
    // to the main queue block (0x1bb42..0x1bb5c -> 0x1bb64). The queue hop
    // is synchronous here.
    state.search_url_pending = search_url_nonempty;
    if search_url_nonempty {
        stub_0x1bb64(state);
    }
}

// 0x1bb64 — ___33-[HomeViewController viewDidLoad]_block_invoke_2
// type: id __fastcall(int)
#[doc(alias = "___33-[HomeViewController viewDidLoad]_block_invoke_2")]
pub fn stub_0x1bb64(state: &mut HomeViewState) {
    // IDA 0x1bb64 `__33-[HomeViewController viewDidLoad]_block_invoke_2`:
    // clears the hidden flag on the `self+284` search view (0x1bb64).
    state.search_field_visible = true;
}

// 0x1bb88 — ___copy_helper_block__1
#[doc(alias = "___copy_helper_block__1")]
pub fn stub_0x1bb88(dst: &mut BlockCapture, src: &BlockCapture) {
    // IDA 0x1bb88 `__copy_helper_block__1`: single
    // `_Block_object_assign` retain (0x1bb8e; cf. 0x1ae78).
    *dst = src.clone();
}

// 0x1bb94 — ___destroy_helper_block__1
#[doc(alias = "___destroy_helper_block__1")]
pub fn stub_0x1bb94(slot: &mut BlockCapture) {
    // IDA 0x1bb94 `__destroy_helper_block__1`: single
    // `_Block_object_dispose` release (0x1bb98; cf. 0x1aea8).
    *slot = BlockCapture::default();
}

// 0x1bb9c — ___copy_helper_block_80
#[doc(alias = "___copy_helper_block_80")]
pub fn stub_0x1bb9c(dst: &mut BlockCapture, src: &BlockCapture) {
    // IDA 0x1bb9c `__copy_helper_block_80`: single
    // `_Block_object_assign` retain (0x1bba2; cf. 0x1bb88).
    *dst = src.clone();
}

// 0x1bba8 — ___destroy_helper_block_81
#[doc(alias = "___destroy_helper_block_81")]
pub fn stub_0x1bba8(slot: &mut BlockCapture) {
    // IDA 0x1bba8 `__destroy_helper_block_81`: single
    // `_Block_object_dispose` release (0x1bbac; cf. 0x1bb94).
    *slot = BlockCapture::default();
}

// 0x1bbb0 — -[HomeViewController keyboardDidShow:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController keyboardDidShow:]")]
pub fn stub_0x1bbb0(state: &mut HomeViewState) {
    // IDA 0x1bbb0 `-[HomeViewController keyboardDidShow:]`: enables the
    // tap recognizer (0x1bbcc).
    state.keyboard_visible = true;
    state.tap_recognizer_enabled = true;
}

// 0x1bbd0 — -[HomeViewController keyboardDidHide:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController keyboardDidHide:]")]
pub fn stub_0x1bbd0(state: &mut HomeViewState) {
    // IDA 0x1bbd0 `-[HomeViewController keyboardDidHide:]`: disables the
    // tap recognizer (0x1bbec).
    state.keyboard_visible = false;
    state.tap_recognizer_enabled = false;
}

// 0x1bbf0 — -[HomeViewController dismissKeyboard]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController dismissKeyboard]")]
pub fn stub_0x1bbf0(state: &mut HomeViewState) {
    // IDA 0x1bbf0 `-[HomeViewController dismissKeyboard]`:
    // `[_searchTextField resignFirstResponder]` (0x1bc0a).
    state.search_resigned = true;
}

// 0x1bc10 — -[HomeViewController localizeAndStyleLabels]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController localizeAndStyleLabels]")]
pub fn stub_0x1bc10(state: &mut HomeViewState) {
    // IDA 0x1bc10 `-[HomeViewController localizeAndStyleLabels]`:
    // `localizedStringForKey:` lookups stamped into the eleven labels/text
    // views (0x1bc48..0x1bf08: GameWord, CatalogWord, InventoryWord,
    // BuildersClubWord, ProfileWord, MessagesWord, CommunityWord,
    // WelcomeToRoblox, YouAreCurrentlyLoggedInAs, SignupButton,
    // LoginButton). Host bundle lookup is the identity table.
    for key in [
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
    ] {
        state.labels.insert(key.to_string(), key.to_string());
    }
}

// 0x1bf0c — -[HomeViewController updateUserInfoDisplay:]
// type: void __cdecl(HomeViewController *self, SEL, bool)
#[doc(alias = "-[HomeViewController updateUserInfoDisplay:]")]
pub fn stub_0x1bf0c(state: &mut HomeViewState, refresh: bool, player: &PlayerInfo) {
    // IDA 0x1bf0c `-[HomeViewController updateUserInfoDisplay:]`: with YES,
    // `UpdatePlayerInfo` first (0x1bf18..0x1bf42); `lblRobux`/`lblTix` get
    // `": "` + value (0x1bf70..0x1c000); `lblPlayerName` only when
    // `username` exists (0x1c008..0x1c044); avatar loads from
    // `userThumbNailUrl` with `highlighted = NO` (0x1c04c..0x1c0fa),
    // otherwise `highlighted = YES` (0x1c10e..0x1c130).
    if refresh {
        state.info_refreshed = true;
    }
    state.user_display = UserDisplay {
        robux_text: format!(": {}", player.robux),
        tix_text: format!(": {}", player.tix),
        player_name: player.username.clone(),
        avatar_from_url: player.thumb_url.is_some(),
        avatar_highlighted: player.thumb_url.is_none(),
    };
}

// 0x1c134 — -[HomeViewController viewDidUnload]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController viewDidUnload]")]
pub fn stub_0x1c134(state: &mut HomeViewState) {
    // IDA 0x1c134 `-[HomeViewController viewDidUnload]`: nils the seventeen
    // outlets via setters (0x1c14c..0x1c290 — note signup/login labels are
    // nilled twice, 0x1c22a/0x1c272 and 0x1c240/0x1c27c) then super
    // `viewDidUnload` (0x1c2a8..0x1c2b2, host UIKit). View-bound latches
    // fold back to unset.
    state.labels.clear();
    state.version_text = None;
    state.user_display = UserDisplay::default();
    state.search_field_visible = false;
}

// 0x1c2bc — -[HomeViewController handleSignupNotification:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController handleSignupNotification:]")]
pub fn stub_0x1c2bc(state: &mut HomeViewState, username: &str, password: &str) {
    // IDA 0x1c2bc `-[HomeViewController handleSignupNotification:]`:
    // pulls `username`/`password` from the notification `userInfo`
    // (0x1c2d8..0x1c312), retains both (0x1c324..0x1c32c), `doLoginWithUsername:`
    // (0x1c348..0x1c35c), then `showCorrectLoggedInState` (0x1c376 ->
    // 0x1c788, next batch).
    state.login_attempt = Some((username.to_string(), password.to_string()));
}

// 0x1c37c — -[HomeViewController logoutTouchUp:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController logoutTouchUp:]")]
pub fn stub_0x1c37c(state: &mut HomeViewState) {
    // IDA 0x1c37c `-[HomeViewController logoutTouchUp:]`: builds the
    // `UIAlertView` (`RobloxWord` title, `LogoutConfirmation` message,
    // delegate self, `CancelWord`/`LogoutWord` buttons — 0x1c3a4..0x1c47e),
    // shows and releases it (0x1c48e..0x1c4aa).
    state.logout_alert_shown = true;
}

// 0x1c4b0 — -[HomeViewController alertView:didDismissWithButtonIndex:]
// type: void __cdecl(HomeViewController *self, SEL, id, int)
#[doc(alias = "-[HomeViewController alertView:didDismissWithButtonIndex:]")]
pub fn stub_0x1c4b0(state: &mut HomeViewState, button: i32) {
    // IDA 0x1c4b0 `-[HomeViewController
    // alertView:didDismissWithButtonIndex:]`: button 1 (Logout) runs
    // `doLogout` + `+[UserInfo logout]` (0x1c4be..0x1c504), the fade + dismiss
    // animation pair (0x1c546..0x1c58e -> 0x1c5c8/0x1c608), and
    // `setPageViewTracking:@"Logout/Success"` (0x1c5b4). Other buttons are
    // no-ops.
    if button == 1 {
        state.logged_out = true;
        stub_0x1c5c8(state);
        stub_0x1c608(state, false, None, None);
        state.logout_page_view = Some("Logout/Success".to_string());
    }
}

// 0x1c5c8 — ___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke
#[doc(alias = "___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke")]
pub fn stub_0x1c5c8(state: &mut HomeViewState) {
    // IDA 0x1c5c8 `__58-[...alertView:didDismissWithButtonIndex:]_block_invoke`:
    // `buttonView.alpha = 0` fade step (0x1c5da).
    state.button_alpha_zero = true;
}

// 0x1c5f4 — ___copy_helper_block_224
#[doc(alias = "___copy_helper_block_224")]
pub fn stub_0x1c5f4(dst: &mut BlockCapture, src: &BlockCapture) {
    // IDA 0x1c5f4 `__copy_helper_block_224`: single
    // `_Block_object_assign` retain (0x1c5fa; cf. 0x1bb88).
    *dst = src.clone();
}

// 0x1c600 — ___destroy_helper_block_225
#[doc(alias = "___destroy_helper_block_225")]
pub fn stub_0x1c600(slot: &mut BlockCapture) {
    // IDA 0x1c600 `__destroy_helper_block_225`: single
    // `_Block_object_dispose` release (0x1c604; cf. 0x1bb94).
    *slot = BlockCapture::default();
}

// 0x1c608 — ___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke227
#[doc(alias = "___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke227")]
pub fn stub_0x1c608(
    state: &mut HomeViewState,
    has_presenter: bool,
    foreground_x: Option<f32>,
    background_x: Option<f32>,
) {
    // IDA 0x1c608 `__58-..._block_invoke227`: with a presenting controller
    // (0x1c626), snapshots the foreground/background presentation-layer X
    // (0x1c63a..0x1c712, 0 when the layer is missing), then
    // `dismissViewControllerAnimated:NO` (0x1c732).
    if has_presenter {
        state.foreground_x = Some(foreground_x.unwrap_or(0.0));
        state.background_x = Some(background_x.unwrap_or(0.0));
    }
    state.completion_dismissed = true;
}

// 0x1c734 — ___copy_helper_block_246
#[doc(alias = "___copy_helper_block_246")]
pub fn stub_0x1c734(dst: &mut BlockCapture, src: &BlockCapture) {
    // IDA 0x1c734 `__copy_helper_block_246`: single
    // `_Block_object_assign` retain (0x1c73a; cf. 0x1c5f4).
    *dst = src.clone();
}

// 0x1c740 — ___destroy_helper_block_247
#[doc(alias = "___destroy_helper_block_247")]
pub fn stub_0x1c740(slot: &mut BlockCapture) {
    // IDA 0x1c740 `__destroy_helper_block_247`: single
    // `_Block_object_dispose` release (0x1c744; cf. 0x1c600).
    *slot = BlockCapture::default();
}

// 0x1c748 — -[HomeViewController viewWillAppear:]
// type: void __cdecl(HomeViewController *self, SEL, char)
#[doc(alias = "-[HomeViewController viewWillAppear:]")]
pub fn stub_0x1c748(state: &mut HomeViewState, animated: bool) {
    // IDA 0x1c748 `-[HomeViewController viewWillAppear:]`: super
    // `RobloxPageViewController` call (0x1c764..0x1c76e, host UIKit) then
    // `showCorrectLoggedInState` (0x1c780 -> 0x1c788, next batch).
    let _ = animated;
    state.appeared = true;
}

// 0x1c788 — -[HomeViewController showCorrectLoggedInState]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController showCorrectLoggedInState]")]
pub fn stub_0x1c788(state: &mut HomeViewState, user_logged_in: bool, player: &PlayerInfo) {
    // IDA 0x1c788 `-[HomeViewController showCorrectLoggedInState]`: hides
    // `notLoggedInView` + shows `loggedInView` when `userLoggedIn == 1`
    // (0x1c7a8..0x1c7f8), the reverse otherwise (0x1c7fe..0x1c814), then the
    // async refresh block (0x1c820..0x1c858 -> 0x1c860). The queue hop is
    // synchronous here.
    state.logged_in_shown = user_logged_in;
    stub_0x1c860(state, player);
}

// 0x1c860 — ___46-[HomeViewController showCorrectLoggedInState]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___46-[HomeViewController showCorrectLoggedInState]_block_invoke")]
pub fn stub_0x1c860(state: &mut HomeViewState, player: &PlayerInfo) {
    // IDA 0x1c860 `__46-[...showCorrectLoggedInState]_block_invoke`:
    // `updateUserInfoDisplay:YES` shim (single `objc_msgSend`; cf. 0x1bb64).
    stub_0x1bf0c(state, true, player);
}

// 0x1c874 — ___copy_helper_block_261
#[doc(alias = "___copy_helper_block_261")]
pub fn stub_0x1c874(dst: &mut BlockCapture, src: &BlockCapture) {
    // IDA 0x1c874 `__copy_helper_block_261`: single
    // `_Block_object_assign` retain (0x1c87a; cf. 0x1bb88).
    *dst = src.clone();
}

// 0x1c880 — ___destroy_helper_block_262
#[doc(alias = "___destroy_helper_block_262")]
pub fn stub_0x1c880(slot: &mut BlockCapture) {
    // IDA 0x1c880 `__destroy_helper_block_262`: single
    // `_Block_object_dispose` release (0x1c884; cf. 0x1bb94).
    *slot = BlockCapture::default();
}

// 0x1c888 — -[HomeViewController viewDidAppear:]
// type: void __cdecl(HomeViewController *self, SEL, char)
#[doc(alias = "-[HomeViewController viewDidAppear:]")]
pub fn stub_0x1c888(state: &mut HomeViewState, animated: bool) {
    // IDA 0x1c888 `-[HomeViewController viewDidAppear:]`: super
    // `RobloxAnimatingPageViewController` call (0x1c8a4..0x1c8ae, host
    // UIKit); when `viewMustSegueAfterLoad == 1`, clears it and performs
    // `sequeToWeb` (0x1c8c0..0x1c8e0).
    let _ = animated;
    if state.segue_after_load {
        state.segue_after_load = false;
        state.segue_performed = Some("sequeToWeb".to_string());
    }
}

// 0x1c8e8 — -[HomeViewController handleStartGameFailure]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController handleStartGameFailure]")]
pub fn stub_0x1c8e8(state: &mut HomeViewState) {
    // IDA 0x1c8e8 `-[HomeViewController handleStartGameFailure]`:
    // `RobloxAlertWithMessage:` with the localized `GeneralGameStartError`
    // string (0x1c912..0x1c954).
    state.last_alert = Some("GeneralGameStartError".to_string());
}

// 0x1c958 — -[HomeViewController handleStartGameSuccess]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController handleStartGameSuccess]")]
pub fn stub_0x1c958() {
    // IDA 0x1c958 `-[HomeViewController handleStartGameSuccess]`: empty
    // body — no-op.
}

// 0x1c95c — -[HomeViewController placeIdClicked:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController placeIdClicked:]")]
pub fn stub_0x1c95c(state: &mut HomeViewState) {
    // IDA 0x1c95c `-[HomeViewController placeIdClicked:]`: parses
    // `placeId`/`portId` ints and `ipId` text (0x1c986..0x1c9d6), resigns
    // all three fields (0x1c9ea..0x1c9fe); with a nonzero port and nonempty
    // ip runs `startGameLocal:...` (0x1ca24..0x1ca90), otherwise
    // `startGame:...` with the place id (0x1ca3e..0x1ca5a).
    let place_id = state.place_id_text.parse::<i32>().unwrap_or(0);
    let port = state.port_id_text.parse::<i32>().unwrap_or(0);
    state.input_resigned = true;
    state.last_launch = Some(if port != 0 && !state.ip_id_text.is_empty() {
        PlaceLaunch::Local { port, ip: state.ip_id_text.clone() }
    } else {
        PlaceLaunch::Remote { place_id }
    });
}

// 0x1ca9c — -[HomeViewController searchEditingDidEnd:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController searchEditingDidEnd:]")]
pub fn stub_0x1ca9c() {
    // IDA 0x1ca9c `-[HomeViewController searchEditingDidEnd:]`: empty
    // body — no-op.
}

// 0x1caa0 — -[HomeViewController searchDidEndOnExit:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController searchDidEndOnExit:]")]
pub fn stub_0x1caa0(state: &mut HomeViewState) {
    // IDA 0x1caa0 `-[HomeViewController searchDidEndOnExit:]`: performs
    // `sequeToWeb` with the search field as sender (0x1cac4).
    state.segue_performed = Some("sequeToWeb".to_string());
}

// 0x1cac8 — -[HomeViewController signUpButtonDidTouchUpInside:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController signUpButtonDidTouchUpInside:]")]
pub fn stub_0x1cac8() {
    // IDA 0x1cac8 `-[HomeViewController signUpButtonDidTouchUpInside:]`:
    // empty body — no-op.
}

// 0x1cacc — -[HomeViewController logInButtonDidTouchUpInside:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController logInButtonDidTouchUpInside:]")]
pub fn stub_0x1cacc(state: &mut HomeViewState) {
    // IDA 0x1cacc `-[HomeViewController logInButtonDidTouchUpInside:]`:
    // `dismissViewControllerAnimated:1 completion:0` (0x1cadc, host UIKit).
    state.dismissed = true;
}

// 0x1cae0 — -[HomeViewController buttonForWebDidTouchUpInside:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController buttonForWebDidTouchUpInside:]")]
pub fn stub_0x1cae0(state: &mut HomeViewState, user_logged_in: bool) {
    // IDA 0x1cae0 `-[HomeViewController buttonForWebDidTouchUpInside:]`:
    // logged in -> `sequeToWeb` (0x1cb02..0x1cba6); otherwise
    // `RobloxAlertWithMessage:` with `YouMustLogin` (0x1cb64..0x1cb3c).
    if user_logged_in {
        state.segue_performed = Some("sequeToWeb".to_string());
    } else {
        state.last_alert = Some("YouMustLogin".to_string());
    }
}

// 0x1cbac — -[HomeViewController btnTouchPlayButtonDisabled:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController btnTouchPlayButtonDisabled:]")]
pub fn stub_0x1cbac(state: &mut HomeViewState) {
    // IDA 0x1cbac `-[HomeViewController btnTouchPlayButtonDisabled:]`:
    // `RobloxAlertWithMessage:` with the localized
    // `UnsupportedDevicePlayError` string (0x1cbd6..0x1cc18).
    state.last_alert = Some("UnsupportedDevicePlayError".to_string());
}

// 0x1cc1c — +[HomeViewController getUrlForButtonTag:recordPageView:]
// type: id __cdecl(id, SEL, int, char)
#[doc(alias = "+[HomeViewController getUrlForButtonTag:recordPageView:]")]
pub fn stub_0x1cc1c(
    base_url: &str,
    search_url: &str,
    tag: i32,
    record_page_view: bool,
    is_tablet: bool,
) -> (Option<String>, Option<String>) {
    // IDA 0x1cc1c `+[HomeViewController getUrlForButtonTag:recordPageView:]`:
    // forwards to the `query:` variant with an empty query (0x1cc50 ->
    // 0x1cc54).
    stub_0x1cc54(base_url, search_url, tag, record_page_view, "", is_tablet)
}

// 0x1cc54 — +[HomeViewController getUrlForButtonTag:recordPageView:query:]
// type: id __cdecl(id, SEL, int, char, id)
#[doc(alias = "+[HomeViewController getUrlForButtonTag:recordPageView:query:]")]
pub fn stub_0x1cc54(
    base_url: &str,
    search_url: &str,
    tag: i32,
    record_page_view: bool,
    query: &str,
    is_tablet: bool,
) -> (Option<String>, Option<String>) {
    // IDA 0x1cc54 `+[HomeViewController
    // getUrlForButtonTag:recordPageView:query:]`: tag switch over
    // `baseUrl + suffix` (0x1ccc8..0x1cf16; tablet picks the `Catalog/`,
    // `My/Character.aspx`, `User.aspx`, `My/Messages.aspx#Inbox` arms at
    // 0x1cd54/0x1cda8/0x1ce3e/0x1ce92), `setPageViewTracking:` when
    // recording (0x1cf1c..0x1cf3c), StandardOut URL log (0x1cf40..0x1cf88).
    // Returns `(url, page)`; the page is `None` unless recording.
    let (suffix, page): (String, &'static str) = match tag {
        10 => ("games/list".to_string(), "Games"),
        11 => (if is_tablet { "Catalog/" } else { "catalog/" }.to_string(), "Catalog"),
        12 => (
            if is_tablet { "My/Character.aspx" } else { "inventory" }.to_string(),
            "Inventory",
        ),
        13 => ("mobile-app-upgrades/".to_string(), "BuildersClub"),
        14 => (if is_tablet { "User.aspx" } else { "" }.to_string(), "Profile"),
        15 => (
            if is_tablet { "My/Messages.aspx#Inbox" } else { "inbox" }.to_string(),
            "Messages",
        ),
        16 => (format!("{search_url}{query}"), "Search"),
        _ => return (None, None),
    };
    let url = format!("{base_url}{suffix}");
    (Some(url), record_page_view.then(|| page.to_string()))
}

// 0x1cfe8 — -[HomeViewController prepareForSegue:sender:]
// type: void __cdecl(HomeViewController *self, SEL, id, id)
#[doc(alias = "-[HomeViewController prepareForSegue:sender:]")]
pub fn stub_0x1cfe8(
    state: &mut HomeViewState,
    dest_is_nav_bar: bool,
    sender: SegueSender,
    search_text: &str,
    base_url: &str,
    search_url: &str,
    is_tablet: bool,
) {
    // IDA 0x1cfe8 `-[HomeViewController prepareForSegue:sender:]`: only
    // `RobloxNavBarViewController` destinations are handled (0x1d02a..0x1d044).
    // A pending `jumpToPlaceID` resolves to the `----item?id=` URL plus
    // `setJumpToPlacePageAndLaunchGameWithID:` and clears the global
    // (0x1d060..0x1d0c0); otherwise a button sender resolves via
    // `getUrlForButtonTag:recordPageView:` (0x1d0d8..0x1d11a), a search
    // field via the `query:` variant (0x1d12a..0x1d190), and a Home sender
    // via tag 10 (0x1d196..0x1d1bc). Either way the destination gets
    // `setUrl:` plus the preloaded web view (0x1d1d0..0x1d232).
    if !dest_is_nav_bar {
        return;
    }
    if let Some(place_id) = state.jump_to_place_id.take() {
        state.web_url = Some(format!("http://www.roblox.com/----item?id={place_id}"));
        state.web_launch_id = Some(place_id);
        return;
    }
    let (tag, query) = match sender {
        SegueSender::Button(tag) => (tag, ""),
        SegueSender::SearchField(tag) => (tag, search_text),
        SegueSender::Home => (10, ""),
    };
    let (url, _) = stub_0x1cc54(base_url, search_url, tag, true, query, is_tablet);
    state.web_url = url;
}

// 0x1d238 — -[HomeViewController viewMustSegueAfterLoad]
// type: void __cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController viewMustSegueAfterLoad]")]
pub fn stub_0x1d238(state: &mut HomeViewState) {
    // IDA 0x1d238 `-[HomeViewController viewMustSegueAfterLoad]`:
    // sets the global (0x1d244), consumed by `viewDidAppear:` (0x1c8c0).
    state.segue_after_load = true;
}

// 0x1d248 — -[HomeViewController setJumpToPlaceID:]
// type: void __cdecl(HomeViewController *self, SEL, int)
#[doc(alias = "-[HomeViewController setJumpToPlaceID:]")]
pub fn stub_0x1d248(state: &mut HomeViewState, place_id: i32) {
    // IDA 0x1d248 `-[HomeViewController setJumpToPlaceID:]`: stores the
    // global (0x1d252), consumed by `prepareForSegue:` (0x1d060); 0 clears.
    state.jump_to_place_id = (place_id != 0).then_some(place_id);
}

// 0x1d258 — -[HomeViewController blueFrame]
// type: UIImageView *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController blueFrame]")]
pub fn stub_0x1d258(state: &HomeViewState) -> Option<u32> {
    // IDA 0x1d258 `-[HomeViewController blueFrame]`: returns the ivar
    // (0x1d266); opaque platform handle on the host.
    state.blue_frame
}

// 0x1d268 — -[HomeViewController setBlueFrame:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setBlueFrame:]")]
pub fn stub_0x1d268(state: &mut HomeViewState, view: Option<u32>) {
    // IDA 0x1d268 `-[HomeViewController setBlueFrame:]`:
    // `objc_setProperty` retain/setter for slot 196 (0x1d284); host
    // ownership is the `Option`.
    state.blue_frame = view;
}

// 0x1d28c — -[HomeViewController imgAvatar]
// type: UIImageView *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController imgAvatar]")]
pub fn stub_0x1d28c(state: &HomeViewState) -> Option<u32> {
    // IDA 0x1d28c `-[HomeViewController imgAvatar]`: returns the ivar
    // (0x1d29a); opaque platform handle on the host.
    state.avatar_view
}

// 0x1d29c — -[HomeViewController setImgAvatar:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setImgAvatar:]")]
pub fn stub_0x1d29c(state: &mut HomeViewState, view: Option<u32>) {
    // IDA 0x1d29c `-[HomeViewController setImgAvatar:]`:
    // `objc_setProperty` retain/setter (disasm prologue); host ownership
    // is the `Option` (cf. 0x1d268).
    state.avatar_view = view;
}

// 0x1d2c0 — -[HomeViewController lblPlayerName]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController lblPlayerName]")]
pub fn stub_0x1d2c0(state: &HomeViewState) -> Option<u32> {
    // IDA 0x1d2c0 `-[HomeViewController lblPlayerName]`: returns the ivar
    // (0x1d2ce); opaque platform handle on the host.
    state.player_label
}

// 0x1d2d0 — -[HomeViewController setLblPlayerName:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setLblPlayerName:]")]
pub fn stub_0x1d2d0(state: &mut HomeViewState, view: Option<u32>) {
    // IDA 0x1d2d0 `-[HomeViewController setLblPlayerName:]`:
    // `objc_setProperty` retain/setter (disasm prologue); host ownership
    // is the `Option` (cf. 0x1d29c).
    state.player_label = view;
}

// 0x1d2f4 — -[HomeViewController placeId]
// type: UITextField *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController placeId]")]
pub fn stub_0x1d2f4(state: &HomeViewState) -> String {
    // IDA 0x1d2f4 `-[HomeViewController placeId]`: returns the ivar
    // (0x1d302); the host models the debug field by its text content.
    state.place_id_text.clone()
}

// 0x1d304 — -[HomeViewController setPlaceId:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setPlaceId:]")]
pub fn stub_0x1d304(state: &mut HomeViewState, text: &str) {
    // IDA 0x1d304 `-[HomeViewController setPlaceId:]`:
    // `objc_setProperty` retain/setter for slot 208 (0x1d320); the host
    // models the debug field by its text content.
    state.place_id_text = text.to_string();
}

// 0x1d328 — -[HomeViewController portId]
// type: UITextField *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController portId]")]
pub fn stub_0x1d328(state: &HomeViewState) -> String {
    // IDA 0x1d328 `-[HomeViewController portId]`: ivar load (disasm
    // `_portId` LDR); the host models the debug field by its text content
    // (cf. 0x1d2f4).
    state.port_id_text.clone()
}

// 0x1d338 — -[HomeViewController setPortId:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setPortId:]")]
pub fn stub_0x1d338(state: &mut HomeViewState, text: &str) {
    // IDA 0x1d338 `-[HomeViewController setPortId:]`:
    // `objc_setProperty` retain/setter (disasm prologue); the host models
    // the debug field by its text content (cf. 0x1d304).
    state.port_id_text = text.to_string();
}

// 0x1d35c — -[HomeViewController ipId]
// type: UITextField *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController ipId]")]
pub fn stub_0x1d35c(state: &HomeViewState) -> String {
    // IDA 0x1d35c `-[HomeViewController ipId]`: ivar load (disasm `_ipId`
    // LDR); the host models the debug field by its text content (cf.
    // 0x1d328).
    state.ip_id_text.clone()
}

// 0x1d36c — -[HomeViewController setIpId:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setIpId:]")]
pub fn stub_0x1d36c(state: &mut HomeViewState, text: &str) {
    // IDA 0x1d36c `-[HomeViewController setIpId:]`:
    // `objc_setProperty` retain/setter (disasm prologue); the host models
    // the debug field by its text content (cf. 0x1d338).
    state.ip_id_text = text.to_string();
}

// 0x1d390 — -[HomeViewController btnPlaceLauncher]
// type: UIButton *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController btnPlaceLauncher]")]
pub fn stub_0x1d390(state: &HomeViewState) -> Option<u32> {
    // IDA 0x1d390 `-[HomeViewController btnPlaceLauncher]`: ivar load
    // (disasm `_btnPlaceLauncher` LDR); opaque platform handle on the host.
    state.outlet("btnPlaceLauncher")
}

// 0x1d3a0 — -[HomeViewController setBtnPlaceLauncher:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setBtnPlaceLauncher:]")]
pub fn stub_0x1d3a0(state: &mut HomeViewState, view: Option<u32>) {
    // IDA 0x1d3a0 `-[HomeViewController setBtnPlaceLauncher:]`:
    // `objc_setProperty` retain/setter (disasm prologue); host ownership
    // is the outlet slot.
    state.set_outlet("btnPlaceLauncher", view);
}

// 0x1d3c4 — -[HomeViewController btnGames]
// type: UIButton *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController btnGames]")]
pub fn stub_0x1d3c4(state: &HomeViewState) -> Option<u32> {
    // IDA 0x1d3c4 `-[HomeViewController btnGames]`: ivar load (disasm
    // `_btnGames` LDR); opaque platform handle on the host.
    state.outlet("btnGames")
}

// 0x1d3d4 — -[HomeViewController setBtnGames:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setBtnGames:]")]
pub fn stub_0x1d3d4(state: &mut HomeViewState, view: Option<u32>) {
    // IDA 0x1d3d4 `-[HomeViewController setBtnGames:]`:
    // `objc_setProperty` retain/setter (disasm prologue); host ownership
    // is the outlet slot.
    state.set_outlet("btnGames", view);
}

// 0x2111c — -[UpgradeCheckHelper getAlertViewButton:]
// type: id __cdecl(UpgradeCheckHelper *self, SEL, id)
#[doc(alias = "-[UpgradeCheckHelper getAlertViewButton:]")]
pub fn stub_0x2111c(state: &UpgradeCheckState, title: &str) -> Option<u32> {
    // IDA 0x2111c: fast-enumerates `upgradeAlertView.subviews`, keeps
    // the first `UIButton` whose `currentTitle isEqualToString:` the
    // argument (enumeration-mutation guard folds into the host borrow).
    state.buttons.iter().find(|button| button.title == title).map(|button| button.handle)
}

// 0x21254 — -[UpgradeCheckHelper makeUpgradeRequest:]
// type: void __cdecl(UpgradeCheckHelper *self, SEL, id)
#[doc(alias = "-[UpgradeCheckHelper makeUpgradeRequest:]")]
pub fn stub_0x21254(state: &mut UpgradeCheckState, request_url: Option<&str>) {
    // IDA 0x21254 `-[UpgradeCheckHelper makeUpgradeRequest:]`:
    // `setData:nil` clears `upgradeResponseData`; a nil request logs
    // `ItsBloodyNil...` but still connects; the new `NSURLConnection`
    // becomes `upgradeConnection`.
    state.response_data.clear();
    if request_url.is_none() {
        state.nil_request_logged = true;
    }
    let handle = state.next_connection;
    state.next_connection = state.next_connection.wrapping_add(1);
    state.connection = Some(handle);
    state.connection_request = request_url.map(str::to_string);
}

// 0x212cc — +[UpgradeCheckHelper checkForUpdate]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[UpgradeCheckHelper checkForUpdate]")]
pub fn stub_0x212cc(state: &mut UpgradeCheckState, short_version: Option<&str>, reachable: bool, base_url: &str) {
    // IDA 0x212cc `+[UpgradeCheckHelper checkForUpdate]`: no
    // `CFBundleShortVersionString` -> nothing; no reachability ->
    // `RobloxAlertWithMessage:@"ConnectionError"`; else formats
    // `AppiOSV%@`, appends it to `+[UpgradeCheckHelper getUpgradeUrl]`
    // (0x210b4), retains the request, and runs `makeUpgradeRequest:`
    // on the helper (retain/release fold into the synchronous call).
    let Some(version) = short_version else {
        return;
    };
    if !reachable {
        state.last_alert = Some("ConnectionError".to_string());
        return;
    }
    let template = crate::generated_bg_2::stub_0x210b4(base_url);
    let url = template.replace("%@", &format!("AppiOSV{version}"));
    stub_0x21254(state, Some(&url));
}

// 0x214a4 — -[UpgradeCheckHelper processCheckForUpdateResponse]
// type: void __cdecl(UpgradeCheckHelper *self, SEL)
#[doc(alias = "-[UpgradeCheckHelper processCheckForUpdateResponse]")]
pub fn stub_0x214a4(state: &mut UpgradeCheckState) {
    // IDA 0x214a4: `NSJSONSerialization` over `upgradeResponseData`
    // (options 0); failure logs `Error parsing JSON in AppDelegate: %s`
    // via `RBX::StandardOut` (here `last_parse_error`). A null/absent
    // `data`/`UpgradeAction` (NSNull) falls through with no alert;
    // `Recommended` latches the recommend title/body and enables (or
    // adds) the ignore button, `Required` latches the require
    // title/body and disables the ignore button; both `dispatch_async`
    // the main-queue `show` block (synchronous here).
    let text = match std::str::from_utf8(&state.response_data) {
        Ok(text) => text.to_owned(),
        Err(_) => {
            state.last_parse_error = Some("response is not UTF-8".to_string());
            return;
        }
    };
    match upgrade_action_and_message(&text) {
        Err(_) => {
            state.last_parse_error = Some("invalid JSON".to_string());
        }
        Ok(None) => {}
        Ok(Some((action, message))) => {
            if action == "Recommended" {
                let body = message.unwrap_or_else(|| "RecommendUpgradeBody".to_string());
                state.alert_title = Some("RecommendUpgradeTitle".to_string());
                state.alert_message = Some(body);
                if let Some(button) = state.buttons.iter_mut().find(|button| button.title == "IgnoreButtonText") {
                    button.enabled = true;
                } else {
                    let handle = state.buttons.len() as u32;
                    state.buttons.push(AlertButton {
                        title: "IgnoreButtonText".to_string(),
                        enabled: true,
                        handle,
                    });
                }
                state.show_dispatched = true;
                stub_0x21abc(state);
            } else if action == "Required" {
                let body = message.unwrap_or_else(|| "RequireUpgradeBody".to_string());
                state.alert_title = Some("RequireUpgradeTitle".to_string());
                state.alert_message = Some(body);
                if let Some(button) = state.buttons.iter_mut().find(|button| button.title == "IgnoreButtonText") {
                    button.enabled = false;
                }
                state.show_dispatched = true;
                stub_0x21af0(state);
            }
        }
    }
}

// 0x21abc — ___51-[UpgradeCheckHelper processCheckForUpdateResponse]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___51-[UpgradeCheckHelper processCheckForUpdateResponse]_block_invoke")]
pub fn stub_0x21abc(state: &mut UpgradeCheckState) {
    // IDA 0x21abc: main-queue block showing the recommended-upgrade
    // alert (`[upgradeAlertView show]` on the captured self).
    state.alert_shown = true;
}

// 0x21adc — ___copy_helper_block_132
#[doc(alias = "___copy_helper_block_132")]
pub fn stub_0x21adc(dst: &mut BlockCapture, src: &BlockCapture) {
    // IDA 0x21adc `__copy_helper_block_132`: single
    // `_Block_object_assign` retain (cf. 0x1f660).
    *dst = src.clone();
}

// 0x21ae8 — ___destroy_helper_block_133
#[doc(alias = "___destroy_helper_block_133")]
pub fn stub_0x21ae8(slot: &mut BlockCapture) {
    // IDA 0x21ae8 `__destroy_helper_block_133`: single
    // `_Block_object_dispose` release (cf. 0x1f4a0).
    *slot = BlockCapture::default();
}

// 0x21af0 — ___51-[UpgradeCheckHelper processCheckForUpdateResponse]_block_invoke141
#[doc(alias = "___51-[UpgradeCheckHelper processCheckForUpdateResponse]_block_invoke141")]
pub fn stub_0x21af0(state: &mut UpgradeCheckState) {
    // IDA 0x21af0: main-queue block showing the required-upgrade
    // alert (`[upgradeAlertView show]` on the captured self).
    state.alert_shown = true;
}

// 0x21b10 — ___copy_helper_block_142
#[doc(alias = "___copy_helper_block_142")]
pub fn stub_0x21b10(dst: &mut BlockCapture, src: &BlockCapture) {
    // IDA 0x21b10 `__copy_helper_block_142`: single
    // `_Block_object_assign` retain (cf. 0x1f660).
    *dst = src.clone();
}

// 0x21b1c — ___destroy_helper_block_143
#[doc(alias = "___destroy_helper_block_143")]
pub fn stub_0x21b1c(slot: &mut BlockCapture) {
    // IDA 0x21b1c `__destroy_helper_block_143`: single
    // `_Block_object_dispose` release (cf. 0x1f4a0).
    *slot = BlockCapture::default();
}

// 0x21b24 — -[UpgradeCheckHelper connection:didReceiveData:]
// type: void __cdecl(UpgradeCheckHelper *self, SEL, id, id)
#[doc(alias = "-[UpgradeCheckHelper connection:didReceiveData:]")]
pub fn stub_0x21b24(state: &mut UpgradeCheckState, connection: u32, data: &[u8]) {
    // IDA 0x21b24: appends only when the connection is the live
    // `upgradeConnection` (`appendData:`).
    if state.connection == Some(connection) {
        state.response_data.extend_from_slice(data);
    }
}

// 0x21b58 — -[UpgradeCheckHelper connectionDidFinishLoading:]
// type: void __cdecl(UpgradeCheckHelper *self, SEL, id)
#[doc(alias = "-[UpgradeCheckHelper connectionDidFinishLoading:]")]
pub fn stub_0x21b58(state: &mut UpgradeCheckState, connection: u32) {
    // IDA 0x21b58: on the live `upgradeConnection`, releases it, nils
    // the ivar, and runs `processCheckForUpdateResponse`.
    if state.connection == Some(connection) {
        state.connection = None;
        state.connection_request = None;
        stub_0x214a4(state);
    }
}

// 0x21ba0 — -[UpgradeCheckHelper alertView:clickedButtonAtIndex:]
// type: void __cdecl(UpgradeCheckHelper *self, SEL, id, int)
#[doc(alias = "-[UpgradeCheckHelper alertView:clickedButtonAtIndex:]")]
pub fn stub_0x21ba0(state: &mut UpgradeCheckState, is_upgrade_alert: bool, button_index: i32) {
    // IDA 0x21ba0: button 0 of the `upgradeAlertView` opens
    // `itms://itunes.com/apps/robloxmobile`.
    if is_upgrade_alert && button_index == 0 {
        state.opened_url = Some("itms://itunes.com/apps/robloxmobile".to_string());
    }
}

// 0x21c18 — __GLOBAL__I_a_6
#[doc(alias = "global constructor keyed to_a_6")]
pub fn stub_0x21c18(runtime: &mut CxxRuntimeA6) {
    // IDA 0x21c18 `__GLOBAL__I_a_6`: C++ static init — the
    // `boost::system` generic/system category statics,
    // `std::ios_base::Init`, and the `boost::exception_detail`
    // static-exception guards (disasm `BL generic_category`,
    // `__cxa_atexit` chain).
    runtime.initialized = true;
}

// 0x21ce0 — __ZN18iOSSettingsService4InitEv
// type: _DWORD __fastcall(iOSSettingsService *__hidden this)
#[doc(alias = "iOSSettingsService::Init(void)")]
pub fn stub_0x21ce0(service: &mut IosSettingsService) {
    // IDA 0x21ce0 `iOSSettingsService::Init`: sets `var3 = 1`,
    // registers the 39 name -> reader pairs in `var1`
    // (`std::map<std::string, void (*)(char const *)>::operator[]`;
    // string temporaries destroyed after each insert), and zeroes the
    // trailing word at `+44` (refcounts/`__dmb` fold into host ownership).
    service.initialized = true;
    let pairs: [(&str, SettingsHandler); 39] = [
        ("iPadMinimumVersion", SettingsHandler::IPadMinimumVersion),
        ("iPadMaximumVersion", SettingsHandler::IPadMaximumVersion),
        ("iPhoneMinimumVersion", SettingsHandler::IPhoneMinimumVersion),
        ("iPhoneMaximumVersion", SettingsHandler::IPhoneMaximumVersion),
        ("iPodMinimumVersion", SettingsHandler::IPodMinimumVersion),
        ("iPodMaximumVersion", SettingsHandler::IPodMaximumVersion),
        ("DisablePlayButtonForAll", SettingsHandler::DisablePlayButtonForAll),
        ("DisablePlayButtonForNonBC", SettingsHandler::DisablePlayButtonForNonBC),
        ("iPad1_MaximumIdealParts", SettingsHandler::IPad1MaximumIdealParts),
        ("iPad2_MaximumIdealParts", SettingsHandler::IPad2MaximumIdealParts),
        ("iPad3_MaximumIdealParts", SettingsHandler::IPad3MaximumIdealParts),
        ("iPad4_MaximumIdealParts", SettingsHandler::IPad4MaximumIdealParts),
        ("iPod4_MaximumIdealParts", SettingsHandler::IPod4MaximumIdealParts),
        ("iPod5_MaximumIdealParts", SettingsHandler::IPod5MaximumIdealParts),
        ("iPhone4s_MaximumIdealParts", SettingsHandler::IPhone4sMaximumIdealParts),
        ("iPhone5_MaximumIdealParts", SettingsHandler::IPhone5MaximumIdealParts),
        ("TimeIntervalBetweenRobuxPurchaseInMinutes", SettingsHandler::TimeIntervalBetweenRobuxPurchaseInMinutes),
        ("TimeIntervalBetweenBCPurchaseInMinutes", SettingsHandler::TimeIntervalBetweenBCPurchaseInMinutes),
        ("TimeIntervalBetweenCatalogPurchaseInMinutes", SettingsHandler::TimeIntervalBetweenCatalogPurchaseInMinutes),
        ("TimeLimitForBillingServiceRetriesBeforeGivingUp", SettingsHandler::TimeLimitForBillingServiceRetriesBeforeGivingUp),
        ("TestFlightLoggingLevel", SettingsHandler::TestFlightLoggingLevel),
        ("TestFlightPercentage", SettingsHandler::TestFlightPercentage),
        ("BugSensePercentage", SettingsHandler::BugSensePercentage),
        ("BugSenseLogLines", SettingsHandler::BugSenseLogLines),
        ("BugSenseLogLevel", SettingsHandler::BugSenseLogLevel),
        ("iOSGoogleAnalyticsAccount2", SettingsHandler::IOSGoogleAnalyticsAccount2),
        ("iOSGoogleAnalyticsSampleRate", SettingsHandler::IOSGoogleAnalyticsSampleRate),
        ("SearchEndpointIPad", SettingsHandler::SearchEndpointIPad),
        ("SearchEndpointIPhone", SettingsHandler::SearchEndpointIPhone),
        ("CacheUIWebViews", SettingsHandler::CacheUIWebViews),
        ("ThumbstickControlStyle", SettingsHandler::ThumbstickControlStyle),
        ("FreeMemoryCheckerActive", SettingsHandler::FreeMemoryCheckerActive),
        ("FreeMemoryCheckerRateMilliSeconds", SettingsHandler::FreeMemoryCheckerRateMilliSeconds),
        ("FreeMemoryCheckerThresholdKiloBytes", SettingsHandler::FreeMemoryCheckerThresholdKiloBytes),
        ("MemoryBouncerActive", SettingsHandler::MemoryBouncerActive),
        ("MemoryBouncerEnforceRateMilliSeconds", SettingsHandler::MemoryBouncerEnforceRateMilliSeconds),
        ("MemoryBouncerThresholdKiloBytes", SettingsHandler::MemoryBouncerThresholdKiloBytes),
        ("MemoryBouncerLimitMegaBytes", SettingsHandler::MemoryBouncerLimitMegaBytes),
        ("MemoryBouncerLimitMegaBytesForLowMemDevices", SettingsHandler::MemoryBouncerLimitMegaBytesForLowMemDevices),
    ];
    for (name, handler) in pairs {
        service.handlers.insert(name.to_string(), handler);
    }
    service.reserved_44 = 0;
}

/// `atoi` host model for the `ReadValue*` family (IDA 0x239ec et al.):
/// skips ASCII whitespace and one sign, accumulates decimal digits
/// with wraparound; 0 when no digits follow.
pub(crate) fn c_atoi(text: &str) -> i32 {
    let bytes = text.as_bytes();
    let mut pos = 0;
    while pos < bytes.len() && matches!(bytes[pos], b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r') {
        pos += 1;
    }
    let mut negative = false;
    if pos < bytes.len() && (bytes[pos] == b'+' || bytes[pos] == b'-') {
        negative = bytes[pos] == b'-';
        pos += 1;
    }
    let mut value: i32 = 0;
    let mut any = false;
    while pos < bytes.len() && bytes[pos].is_ascii_digit() {
        any = true;
        value = value.wrapping_mul(10).wrapping_add((bytes[pos] - b'0') as i32);
        pos += 1;
    }
    if !any {
        0
    } else if negative {
        value.wrapping_neg()
    } else {
        value
    }
}

/// `SimpleJSON::ParseBool` host model for the bool readers (IDA
/// 0x23bc8/0x23be4; cf. 0x255c8c): true only for `"true"`/`"True"`.
fn parse_bool_value(text: &str) -> bool {
    text == "true" || text == "True"
}

// 0x239ec — __ZN18iOSSettingsService27ReadValueiPadMinimumVersionEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiPadMinimumVersion(char const*)")]
pub fn stub_0x239ec(service: &mut IosSettingsService, value: &str) -> i32 {
    // IDA 0x239ec: `atoi(value)` into `_thisPtr + 28` (the `this`
    // register carries the value string; `a2` is the key, unused).
    let parsed = c_atoi(value);
    service.ipad_minimum_version = parsed;
    parsed
}

// 0x23b50 — __ZN18iOSSettingsService27ReadValueiPadMaximumVersionEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiPadMaximumVersion(char const*)")]
pub fn stub_0x23b50(service: &mut IosSettingsService, value: &str) -> i32 {
    // IDA 0x23b50: `atoi(value)` into `_thisPtr + 32`.
    let parsed = c_atoi(value);
    service.ipad_maximum_version = parsed;
    parsed
}

// 0x23b68 — __ZN18iOSSettingsService29ReadValueiPhoneMinimumVersionEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiPhoneMinimumVersion(char const*)")]
pub fn stub_0x23b68(service: &mut IosSettingsService, value: &str) -> i32 {
    // IDA 0x23b68: `atoi(value)` into `_thisPtr + 36`.
    let parsed = c_atoi(value);
    service.iphone_minimum_version = parsed;
    parsed
}

// 0x23b80 — __ZN18iOSSettingsService29ReadValueiPhoneMaximumVersionEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiPhoneMaximumVersion(char const*)")]
pub fn stub_0x23b80(service: &mut IosSettingsService, value: &str) -> i32 {
    // IDA 0x23b80: `atoi(value)` into `_thisPtr + 40`.
    let parsed = c_atoi(value);
    service.iphone_maximum_version = parsed;
    parsed
}

// 0x23b98 — __ZN18iOSSettingsService27ReadValueiPodMinimumVersionEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiPodMinimumVersion(char const*)")]
pub fn stub_0x23b98(service: &mut IosSettingsService, value: &str) -> i32 {
    // IDA 0x23b98: `atoi(value)` into `_thisPtr + 44`.
    let parsed = c_atoi(value);
    service.ipod_minimum_version = parsed;
    parsed
}

// 0x23bb0 — __ZN18iOSSettingsService27ReadValueiPodMaximumVersionEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiPodMaximumVersion(char const*)")]
pub fn stub_0x23bb0(service: &mut IosSettingsService, value: &str) -> i32 {
    // IDA 0x23bb0: `atoi(value)` into `_thisPtr + 48`.
    let parsed = c_atoi(value);
    service.ipod_maximum_version = parsed;
    parsed
}

// 0x23bc8 — __ZN18iOSSettingsService32ReadValueDisablePlayButtonForAllEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueDisablePlayButtonForAll(char const*)")]
pub fn stub_0x23bc8(service: &mut IosSettingsService, value: &str) -> bool {
    // IDA 0x23bc8: `SimpleJSON::ParseBool(value)` into `_thisPtr + 52`.
    let parsed = parse_bool_value(value);
    service.disable_play_button_for_all = parsed;
    parsed
}

// 0x23be4 — __ZN18iOSSettingsService34ReadValueDisablePlayButtonForNonBCEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueDisablePlayButtonForNonBC(char const*)")]
pub fn stub_0x23be4(service: &mut IosSettingsService, value: &str) -> bool {
    // IDA 0x23be4: `SimpleJSON::ParseBool(value)` into `_thisPtr + 53`.
    let parsed = parse_bool_value(value);
    service.disable_play_button_for_non_bc = parsed;
    parsed
}

// 0x23c00 — __ZN18iOSSettingsService32ReadValueiPad1_MaximumIdealPartsEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiPad1_MaximumIdealParts(char const*)")]
pub fn stub_0x23c00(service: &mut IosSettingsService, value: &str) -> i32 {
    // IDA 0x23c00: `atoi(value)` into `_thisPtr + 56`.
    let parsed = c_atoi(value);
    service.ipad1_maximum_ideal_parts = parsed;
    parsed
}

// 0x23c18 — __ZN18iOSSettingsService32ReadValueiPad2_MaximumIdealPartsEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiPad2_MaximumIdealParts(char const*)")]
pub fn stub_0x23c18(service: &mut IosSettingsService, value: &str) -> i32 {
    // IDA 0x23c18: `atoi(value)` into `_thisPtr + 60`.
    let parsed = c_atoi(value);
    service.ipad2_maximum_ideal_parts = parsed;
    parsed
}

// 0x23c30 — __ZN18iOSSettingsService32ReadValueiPad3_MaximumIdealPartsEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiPad3_MaximumIdealParts(char const*)")]
pub fn stub_0x23c30(service: &mut IosSettingsService, value: &str) -> i32 {
    // IDA 0x23c30: `atoi(value)` into `_thisPtr + 64`.
    let parsed = c_atoi(value);
    service.ipad3_maximum_ideal_parts = parsed;
    parsed
}

// 0x23c48 — __ZN18iOSSettingsService32ReadValueiPad4_MaximumIdealPartsEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiPad4_MaximumIdealParts(char const*)")]
pub fn stub_0x23c48(service: &mut IosSettingsService, value: &str) -> i32 {
    // IDA 0x23c48: `atoi(value)` into `_thisPtr + 68`.
    let parsed = c_atoi(value);
    service.ipad4_maximum_ideal_parts = parsed;
    parsed
}

// 0x23c60 — __ZN18iOSSettingsService32ReadValueiPod4_MaximumIdealPartsEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiPod4_MaximumIdealParts(char const*)")]
pub fn stub_0x23c60(service: &mut IosSettingsService, value: &str) -> i32 {
    // IDA 0x23c60: `atoi(value)` into `_thisPtr + 72`.
    let parsed = c_atoi(value);
    service.ipod4_maximum_ideal_parts = parsed;
    parsed
}

// 0x23c78 — __ZN18iOSSettingsService32ReadValueiPod5_MaximumIdealPartsEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiPod5_MaximumIdealParts(char const*)")]
pub fn stub_0x23c78(service: &mut IosSettingsService, value: &str) -> i32 {
    // IDA 0x23c78: `atoi(value)` into `_thisPtr + 76`.
    let parsed = c_atoi(value);
    service.ipod5_maximum_ideal_parts = parsed;
    parsed
}

// 0x23c90 — __ZN18iOSSettingsService35ReadValueiPhone4s_MaximumIdealPartsEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiPhone4s_MaximumIdealParts(char const*)")]
pub fn stub_0x23c90(service: &mut IosSettingsService, value: &str) -> i32 {
    // IDA 0x23c90: `atoi(value)` into `_thisPtr + 80`.
    let parsed = c_atoi(value);
    service.iphone4s_maximum_ideal_parts = parsed;
    parsed
}

// 0x23ca8 — __ZN18iOSSettingsService34ReadValueiPhone5_MaximumIdealPartsEPKc
// type: int __fastcall(iOSSettingsService *this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiPhone5_MaximumIdealParts(char const*)")]
pub fn stub_0x23ca8(service: &mut IosSettingsService, value: &str) -> i32 {
    // IDA 0x23ca8: `atoi(value)` into `_thisPtr + 84`.
    let parsed = c_atoi(value);
    service.iphone5_maximum_ideal_parts = parsed;
    parsed
}

// 0x23cc0 — __ZN18iOSSettingsService50ReadValueTimeIntervalBetweenRobuxPurchaseInMinutesEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueTimeIntervalBetweenRobuxPurchaseInMinutes(char const*)")]
pub fn stub_0x23cc0(service: &mut IosSettingsService, value: &str) -> i32 {
    // IDA 0x23cc0: `atoi(value)` into `_thisPtr + 96`.
    let parsed = c_atoi(value);
    service.interval_robux_purchase_minutes = parsed;
    parsed
}

// 0x23cd8 — __ZN18iOSSettingsService47ReadValueTimeIntervalBetweenBCPurchaseInMinutesEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueTimeIntervalBetweenBCPurchaseInMinutes(char const*)")]
pub fn stub_0x23cd8(service: &mut IosSettingsService, value: &str) -> i32 {
    // IDA 0x23cd8: `atoi(value)` into `_thisPtr + 100`.
    let parsed = c_atoi(value);
    service.interval_bc_purchase_minutes = parsed;
    parsed
}

// 0x23cf0 — __ZN18iOSSettingsService52ReadValueTimeIntervalBetweenCatalogPurchaseInMinutesEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueTimeIntervalBetweenCatalogPurchaseInMinutes(char const*)")]
pub fn stub_0x23cf0(service: &mut IosSettingsService, value: &str) -> i32 {
    // IDA 0x23cf0: `atoi(value)` into `_thisPtr + 104`.
    let parsed = c_atoi(value);
    service.interval_catalog_purchase_minutes = parsed;
    parsed
}

// 0x23d08 — __ZN18iOSSettingsService56ReadValueTimeLimitForBillingServiceRetriesBeforeGivingUpEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueTimeLimitForBillingServiceRetriesBeforeGivingUp(char const*)")]
pub fn stub_0x23d08(service: &mut IosSettingsService, value: &str) -> i32 {
    // IDA 0x23d08: `atoi(value)` into `_thisPtr + 108`.
    let parsed = c_atoi(value);
    service.billing_retry_limit = parsed;
    parsed
}

// 0x23d20 — __ZN18iOSSettingsService31ReadValueTestFlightLoggingLevelEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueTestFlightLoggingLevel(char const*)")]
pub fn stub_0x23d20(service: &mut IosSettingsService, value: &str) -> i32 {
    // IDA 0x23d20: `atoi(value)` into `_thisPtr + 112`.
    let parsed = c_atoi(value);
    service.testflight_logging_level = parsed;
    parsed
}

// 0x23d38 — __ZN18iOSSettingsService29ReadValueTestFlightPercentageEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueTestFlightPercentage(char const*)")]
pub fn stub_0x23d38(service: &mut IosSettingsService, value: &str) -> i32 {
    // IDA 0x23d38: `atoi(value)` into `_thisPtr + 116`.
    let parsed = c_atoi(value);
    service.testflight_percentage = parsed;
    parsed
}

// 0x23d50 — __ZN18iOSSettingsService27ReadValueBugSensePercentageEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueBugSensePercentage(char const*)")]
pub fn stub_0x23d50(service: &mut IosSettingsService, value: &str) -> i32 {
    // IDA 0x23d50: `atoi(value)` into `_thisPtr + 120`.
    let parsed = c_atoi(value);
    service.bugsense_percentage = parsed;
    parsed
}

// 0x23d68 — __ZN18iOSSettingsService25ReadValueBugSenseLogLinesEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueBugSenseLogLines(char const*)")]
pub fn stub_0x23d68(service: &mut IosSettingsService, value: &str) -> i32 {
    // IDA 0x23d68: `atoi(value)` into `_thisPtr + 124`.
    let parsed = c_atoi(value);
    service.bugsense_log_lines = parsed;
    parsed
}

// 0x23d80 — __ZN18iOSSettingsService25ReadValueBugSenseLogLevelEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueBugSenseLogLevel(char const*)")]
pub fn stub_0x23d80(service: &mut IosSettingsService, value: &str) -> i32 {
    // IDA 0x23d80: `atoi(value)` into `_thisPtr + 128`.
    let parsed = c_atoi(value);
    service.bugsense_log_level = parsed;
    parsed
}

// 0x23d9c — __ZN18iOSSettingsService35ReadValueiOSGoogleAnalyticsAccount2EPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiOSGoogleAnalyticsAccount2(char const*)")]
pub fn stub_0x23d9c(service: &mut IosSettingsService, value: &str) {
    // IDA 0x23d9c: `std::string::assign(_thisPtr + 88, value)`
    // (temporary destroyed after; refcount folds into host ownership).
    service.google_analytics_account2 = value.to_string();
}

// 0x23ed4 — __ZN18iOSSettingsService37ReadValueiOSGoogleAnalyticsSampleRateEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiOSGoogleAnalyticsSampleRate(char const*)")]
pub fn stub_0x23ed4(service: &mut IosSettingsService, value: &str) -> i32 {
    // IDA 0x23ed4: `atoi(value)` into `_thisPtr + 92`.
    let parsed = c_atoi(value);
    service.google_analytics_sample_rate = parsed;
    parsed
}

// 0x23eec — __ZN18iOSSettingsService27ReadValueSearchEndpointIPadEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueSearchEndpointIPad(char const*)")]
pub fn stub_0x23eec(service: &mut IosSettingsService, value: &str) {
    // IDA 0x23eec: `std::string::assign(_thisPtr + 132, value)`.
    service.search_endpoint_ipad = value.to_string();
}

// 0x24024 — __ZN18iOSSettingsService29ReadValueSearchEndpointIPhoneEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueSearchEndpointIPhone(char const*)")]
pub fn stub_0x24024(service: &mut IosSettingsService, value: &str) {
    // IDA 0x24024: `std::string::assign(_thisPtr + 136, value)`.
    service.search_endpoint_iphone = value.to_string();
}

// 0x2415c — __ZN18iOSSettingsService24ReadValueCacheUIWebViewsEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueCacheUIWebViews(char const*)")]
pub fn stub_0x2415c(service: &mut IosSettingsService, value: &str) -> bool {
    // IDA 0x2415c: `SimpleJSON::ParseBool(value)` into `_thisPtr + 140`.
    let parsed = parse_bool_value(value);
    service.cache_ui_web_views = parsed;
    parsed
}

// 0x24178 — __ZN18iOSSettingsService31ReadValueThumbstickControlStyleEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueThumbstickControlStyle(char const*)")]
pub fn stub_0x24178(service: &mut IosSettingsService, value: &str) -> i32 {
    // IDA 0x24178: `atoi(value)` into `_thisPtr + 144`.
    let parsed = c_atoi(value);
    service.thumbstick_control_style = parsed;
    parsed
}

// 0x24194 — __ZN18iOSSettingsService32ReadValueFreeMemoryCheckerActiveEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueFreeMemoryCheckerActive(char const*)")]
pub fn stub_0x24194(service: &mut IosSettingsService, value: &str) -> bool {
    // IDA 0x24194: `SimpleJSON::ParseBool(value)` into `_thisPtr + 148`.
    let parsed = parse_bool_value(value);
    service.free_memory_checker_active = parsed;
    parsed
}

// 0x241b0 — __ZN18iOSSettingsService42ReadValueFreeMemoryCheckerRateMilliSecondsEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueFreeMemoryCheckerRateMilliSeconds(char const*)")]
pub fn stub_0x241b0(service: &mut IosSettingsService, value: &str) -> i32 {
    // IDA 0x241b0: `atoi(value)` into `_thisPtr + 152`.
    let parsed = c_atoi(value);
    service.free_memory_checker_rate_ms = parsed;
    parsed
}

// 0x241cc — __ZN18iOSSettingsService44ReadValueFreeMemoryCheckerThresholdKiloBytesEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueFreeMemoryCheckerThresholdKiloBytes(char const*)")]
pub fn stub_0x241cc(service: &mut IosSettingsService, value: &str) -> i32 {
    // IDA 0x241cc: `atoi(value)` into `_thisPtr + 156`.
    let parsed = c_atoi(value);
    service.free_memory_checker_threshold_kb = parsed;
    parsed
}

// 0x241e8 — __ZN18iOSSettingsService28ReadValueMemoryBouncerActiveEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueMemoryBouncerActive(char const*)")]
pub fn stub_0x241e8(service: &mut IosSettingsService, value: &str) -> bool {
    // IDA 0x241e8: `SimpleJSON::ParseBool(value)` into `_thisPtr + 160`.
    let parsed = parse_bool_value(value);
    service.memory_bouncer_active = parsed;
    parsed
}
