//! Auto-generated skeletons for rbx-network — global EA-sorted filler (RakNet|Network|Replicat|Socket filtered exhausted)
//! Filter: RakNet|Network|Replicat|Socket -> 5198 funcs (cs), 5282 (ci), 1 remaining before batch (next 0xecd6e8 _TFCreateCrashSocket); filler global ascending
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +100 stubs | range 0x23d50..0x2ba14 | existing 17309 -> 17409 total (filler global ascending EA-sorted, rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use crate::generated_154::{IosSettingKey, IosSettingsValues, ios_atoi, ios_parse_bool};
use std::collections::HashMap;
// 0x23d50 — __ZN18iOSSettingsService27ReadValueBugSensePercentageEPKc
// demangled: iOSSettingsService::ReadValueBugSensePercentage(char const*)
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueBugSensePercentage(char const*)")]
pub fn stub_23d50(state: &mut IosSettingsValues, value: &str) -> i32 {
    // IDA 0x23d50: atoi(value) stored to _thisPtr+120 (0x23d54..0x23d64).
    let parsed = ios_atoi(value);
    state.ints.insert(IosSettingKey::BugSensePercentage, parsed as i64);
    parsed
}

// 0x23d68 — __ZN18iOSSettingsService25ReadValueBugSenseLogLinesEPKc
// demangled: iOSSettingsService::ReadValueBugSenseLogLines(char const*)
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueBugSenseLogLines(char const*)")]
pub fn stub_23d68(state: &mut IosSettingsValues, value: &str) -> i32 {
    // IDA 0x23d68: atoi(value) stored to _thisPtr+124 (0x23d6c..0x23d7c).
    let parsed = ios_atoi(value);
    state.ints.insert(IosSettingKey::BugSenseLogLines, parsed as i64);
    parsed
}

// 0x23d80 — __ZN18iOSSettingsService25ReadValueBugSenseLogLevelEPKc
// demangled: iOSSettingsService::ReadValueBugSenseLogLevel(char const*)
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueBugSenseLogLevel(char const*)")]
pub fn stub_23d80(state: &mut IosSettingsValues, value: &str) -> i32 {
    // IDA 0x23d80: atoi(value) stored to _thisPtr+128 (0x23d84..0x23d94).
    let parsed = ios_atoi(value);
    state.ints.insert(IosSettingKey::BugSenseLogLevel, parsed as i64);
    parsed
}

// 0x23d9c — __ZN18iOSSettingsService35ReadValueiOSGoogleAnalyticsAccount2EPKc
// demangled: iOSSettingsService::ReadValueiOSGoogleAnalyticsAccount2(char const*)
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiOSGoogleAnalyticsAccount2(char const*)")]
pub fn stub_23d9c(state: &mut IosSettingsValues, value: &str) {
    // IDA 0x23d9c: std::string assign of value to _thisPtr+88 (0x23dfc..0x23e0a).
    state.strings.insert(IosSettingKey::IosGoogleAnalyticsAccount2, value.to_owned());
}

// 0x23ed4 — __ZN18iOSSettingsService37ReadValueiOSGoogleAnalyticsSampleRateEPKc
// demangled: iOSSettingsService::ReadValueiOSGoogleAnalyticsSampleRate(char const*)
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiOSGoogleAnalyticsSampleRate(char const*)")]
pub fn stub_23ed4(state: &mut IosSettingsValues, value: &str) -> i32 {
    // IDA 0x23ed4: atoi(value) stored to _thisPtr+92 (0x23ed8..0x23ee8).
    let parsed = ios_atoi(value);
    state.ints.insert(IosSettingKey::IosGoogleAnalyticsSampleRate, parsed as i64);
    parsed
}

// 0x23eec — __ZN18iOSSettingsService27ReadValueSearchEndpointIPadEPKc
// demangled: iOSSettingsService::ReadValueSearchEndpointIPad(char const*)
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueSearchEndpointIPad(char const*)")]
pub fn stub_23eec(state: &mut IosSettingsValues, value: &str) {
    // IDA 0x23eec: std::string assign of value to _thisPtr+132 (0x23f5a).
    state.strings.insert(IosSettingKey::SearchEndpointIpad, value.to_owned());
}

// 0x24024 — __ZN18iOSSettingsService29ReadValueSearchEndpointIPhoneEPKc
// demangled: iOSSettingsService::ReadValueSearchEndpointIPhone(char const*)
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueSearchEndpointIPhone(char const*)")]
pub fn stub_24024(state: &mut IosSettingsValues, value: &str) {
    // IDA 0x24024: std::string assign of value to _thisPtr+136 (0x24092).
    state.strings.insert(IosSettingKey::SearchEndpointIphone, value.to_owned());
}

// 0x2415c — __ZN18iOSSettingsService24ReadValueCacheUIWebViewsEPKc
// demangled: iOSSettingsService::ReadValueCacheUIWebViews(char const*)
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueCacheUIWebViews(char const*)")]
pub fn stub_2415c(state: &mut IosSettingsValues, value: &str) -> bool {
    // IDA 0x2415c: SimpleJSON::ParseBool(value) ("true"/"True" only, 0x24160) stored to _thisPtr+140 (0x24170).
    let parsed = ios_parse_bool(value);
    state.bools.insert(IosSettingKey::CacheUiWebViews, parsed);
    parsed
}

// 0x24178 — __ZN18iOSSettingsService31ReadValueThumbstickControlStyleEPKc
// demangled: iOSSettingsService::ReadValueThumbstickControlStyle(char const*)
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueThumbstickControlStyle(char const*)")]
pub fn stub_24178(state: &mut IosSettingsValues, value: &str) -> i32 {
    // IDA 0x24178: atoi(value) stored to _thisPtr+144 (0x2417c..0x2418c).
    let parsed = ios_atoi(value);
    state.ints.insert(IosSettingKey::ThumbstickControlStyle, parsed as i64);
    parsed
}

// 0x24194 — __ZN18iOSSettingsService32ReadValueFreeMemoryCheckerActiveEPKc
// demangled: iOSSettingsService::ReadValueFreeMemoryCheckerActive(char const*)
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueFreeMemoryCheckerActive(char const*)")]
pub fn stub_24194(state: &mut IosSettingsValues, value: &str) -> bool {
    // IDA 0x24194: SimpleJSON::ParseBool(value) ("true"/"True" only, 0x24198) stored to _thisPtr+148 (0x241a8).
    let parsed = ios_parse_bool(value);
    state.bools.insert(IosSettingKey::FreeMemoryCheckerActive, parsed);
    parsed
}

// 0x241b0 — __ZN18iOSSettingsService42ReadValueFreeMemoryCheckerRateMilliSecondsEPKc
// demangled: iOSSettingsService::ReadValueFreeMemoryCheckerRateMilliSeconds(char const*)
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueFreeMemoryCheckerRateMilliSeconds(char const*)")]
pub fn stub_241b0(state: &mut IosSettingsValues, value: &str) -> i32 {
    // IDA 0x241b0: atoi(value) stored to _thisPtr+152 (0x241b4..0x241c4).
    let parsed = ios_atoi(value);
    state.ints.insert(IosSettingKey::FreeMemoryCheckerRateMilliSeconds, parsed as i64);
    parsed
}

// 0x241cc — __ZN18iOSSettingsService44ReadValueFreeMemoryCheckerThresholdKiloBytesEPKc
// demangled: iOSSettingsService::ReadValueFreeMemoryCheckerThresholdKiloBytes(char const*)
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueFreeMemoryCheckerThresholdKiloBytes(char const*)")]
pub fn stub_241cc(state: &mut IosSettingsValues, value: &str) -> i32 {
    // IDA 0x241cc: atoi(value) stored to _thisPtr+156 (0x241d0..0x241e0).
    let parsed = ios_atoi(value);
    state.ints.insert(IosSettingKey::FreeMemoryCheckerThresholdKiloBytes, parsed as i64);
    parsed
}

// 0x241e8 — __ZN18iOSSettingsService28ReadValueMemoryBouncerActiveEPKc
// demangled: iOSSettingsService::ReadValueMemoryBouncerActive(char const*)
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueMemoryBouncerActive(char const*)")]
pub fn stub_241e8(state: &mut IosSettingsValues, value: &str) -> bool {
    // IDA 0x241e8: SimpleJSON::ParseBool(value) ("true"/"True" only, 0x241ec) stored to _thisPtr+160 (0x241fc).
    let parsed = ios_parse_bool(value);
    state.bools.insert(IosSettingKey::MemoryBouncerActive, parsed);
    parsed
}

// 0x24204 — __ZN18iOSSettingsService45ReadValueMemoryBouncerEnforceRateMilliSecondsEPKc
// demangled: iOSSettingsService::ReadValueMemoryBouncerEnforceRateMilliSeconds(char const*)
// type: int __fastcall(iOSSettingsService *this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueMemoryBouncerEnforceRateMilliSeconds(char const*)")]
pub fn stub_24204(state: &mut IosSettingsValues, value: &str) -> i32 {
    // IDA 0x24204: atoi(value) stored to _thisPtr+164 (0x24208..0x24218).
    let parsed = ios_atoi(value);
    state.ints.insert(IosSettingKey::MemoryBouncerEnforceRateMilliSeconds, parsed as i64);
    parsed
}

// 0x24220 — __ZN18iOSSettingsService40ReadValueMemoryBouncerThresholdKiloBytesEPKc
// demangled: iOSSettingsService::ReadValueMemoryBouncerThresholdKiloBytes(char const*)
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueMemoryBouncerThresholdKiloBytes(char const*)")]
pub fn stub_24220(state: &mut IosSettingsValues, value: &str) -> i32 {
    // IDA 0x24220: atoi(value) stored to _thisPtr+168 (0x24224..0x24234).
    let parsed = ios_atoi(value);
    state.ints.insert(IosSettingKey::MemoryBouncerThresholdKiloBytes, parsed as i64);
    parsed
}

// 0x2423c — __ZN18iOSSettingsService36ReadValueMemoryBouncerLimitMegaBytesEPKc
// demangled: iOSSettingsService::ReadValueMemoryBouncerLimitMegaBytes(char const*)
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueMemoryBouncerLimitMegaBytes(char const*)")]
pub fn stub_2423c(state: &mut IosSettingsValues, value: &str) -> i32 {
    // IDA 0x2423c: atoi(value) stored to _thisPtr+172 (0x24240..0x24250).
    let parsed = ios_atoi(value);
    state.ints.insert(IosSettingKey::MemoryBouncerLimitMegaBytes, parsed as i64);
    parsed
}

// 0x24258 — __ZN18iOSSettingsService52ReadValueMemoryBouncerLimitMegaBytesForLowMemDevicesEPKc
// demangled: iOSSettingsService::ReadValueMemoryBouncerLimitMegaBytesForLowMemDevices(char const*)
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueMemoryBouncerLimitMegaBytesForLowMemDevices(char const*)")]
pub fn stub_24258(state: &mut IosSettingsValues, value: &str) -> i32 {
    // IDA 0x24258: atoi(value) stored to _thisPtr+176 (0x2425c..0x2426c).
    let parsed = ios_atoi(value);
    state.ints.insert(IosSettingKey::MemoryBouncerLimitMegaBytesForLowMemDevices, parsed as i64);
    parsed
}

// 0x24274 — __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_
// demangled: std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,void (*)(char const*)>>,std::pair<std::string const,void (*)(char const*)> const&)
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,void (*)(char const*)>>,std::pair<std::string const,void (*)(char const*)> const&)")]
pub fn stub_24274(map: &mut HashMap<String, IosSettingKey>, key: &str, value: IosSettingKey) -> bool {
    // IDA 0x24274: _Rb_tree _M_insert_unique (position-hint overload) on the string->ReadValue map; host unique-insert — true when the key was inserted.
    if map.contains_key(key) {
        false
    } else {
        map.insert(key.to_owned(), value);
        true
    }
}

// 0x24360 — __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE9_M_insertEPSt18_Rb_tree_node_baseSE_RKS6_
// demangled: std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,void (*)(char const*)> const&)
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,void (*)(char const*)> const&)")]
pub fn stub_24360(map: &mut HashMap<String, IosSettingKey>, key: &str, value: IosSettingKey) -> bool {
    // IDA 0x24360: _Rb_tree _M_insert (hinted) on the string->ReadValue map; host unique-insert — true when the key was inserted.
    if map.contains_key(key) {
        false
    } else {
        map.insert(key.to_owned(), value);
        true
    }
}

// 0x243b0 — __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE16_M_insert_uniqueERKS6_
// demangled: std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_insert_unique(std::pair<std::string const,void (*)(char const*)> const&)
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_insert_unique(std::pair<std::string const,void (*)(char const*)> const&)")]
pub fn stub_243b0(map: &mut HashMap<String, IosSettingKey>, key: &str, value: IosSettingKey) -> bool {
    // IDA 0x243b0: _Rb_tree _M_insert_unique (reference overload) on the string->ReadValue map; host unique-insert — true when the key was inserted.
    if map.contains_key(key) {
        false
    } else {
        map.insert(key.to_owned(), value);
        true
    }
}

// 0x24434 — __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE14_M_create_nodeERKS6_
// demangled: std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_create_node(std::pair<std::string const,void (*)(char const*)> const&)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_create_node(std::pair<std::string const,void (*)(char const*)> const&)")]
pub fn stub_24434(handle: u32) {
    // IDA 0x24434: _Rb_tree _M_create_node — node copy-alloc engine-side; no host carrier.
    let _ = handle;
}

// 0x24510 — __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE11lower_boundERS1_
// demangled: std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::lower_bound(std::string const&)
// type: int __fastcall(int, std::string *)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::lower_bound(std::string const&)")]
pub fn stub_24510<'a>(map: &'a HashMap<String, IosSettingKey>, key: &str) -> Option<&'a IosSettingKey> {
    // IDA 0x24510: _Rb_tree lower_bound on the reader map; host folds to an exact lookup (tree ordering has no HashMap carrier) — None covers the miss path whose slot operator[] would then default-insert.
    map.get(key)
}

// 0x24540 — __GLOBAL__I_a_7
// demangled: global constructor keyed to_a_7
// type: 
#[doc(alias = "global constructor keyed to_a_7")]
pub fn stub_24540() {
    // IDA 0x24540: __GLOBAL__I_a_7 — static init storing boost::system generic_category + system_category into merged globals (was: boost::system::error_category singletons; host maps to std::io error kinds — faithful no-op shell).
}

/// Host state built by `-[PlaceLauncher init]` (IDA 0x246d8).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PlaceLauncherState {
    pub has_received_memory_warning: bool,
    pub is_currently_playing_game: bool,
    pub is_leaving_game: bool,
    pub last_place_id: i32,
    pub teleporter_window: u32,
    pub did_leave_game_notification: String,
    pub start_leave_game_notification: String,
    pub game_finished_loading_notification: String,
}

/// Host reachability (`Reachability currentReachabilityStatus`, cf. IDA 0x24b6a).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReachabilityStatus {
    None,
    Wifi,
    Cellular,
}

/// Host outcome of `-[PlaceLauncher prepareGame]` (IDA 0x24ab0).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrepareGame {
    Ready { asset_folder: String, datamodel_hash: String },
    Alert(&'static str),
}

/// Host warning raised by the `checkPlacePartCount` block (IDA 0x2512c).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartCountWarning {
    pub title_key: &'static str,
    pub body_format_key: &'static str,
    pub event_category: &'static str,
    pub event_action: &'static str,
    pub place_id: i32,
}

// 0x246d8 — -[PlaceLauncher init]
// demangled: -[PlaceLauncher init]
// type: PlaceLauncher *__cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher init]")]
pub fn stub_246d8(window: u32) -> PlaceLauncherState {
    // IDA 0x246d8: super init (0x2471a); zeroed view/flags/placeId (0x24760..0x24780); Teleporter(window) installed + SetCallback (0x2478e..0x247dc); RBXDidLeaveGame/RBXStartLeaveGame/RBXGameFinishedLoading notification names (0x24800..0x24890).
    PlaceLauncherState { has_received_memory_warning: false, is_currently_playing_game: false, is_leaving_game: false, last_place_id: 0, teleporter_window: window, did_leave_game_notification: "RBXDidLeaveGameNotification".to_owned(), start_leave_game_notification: "RBXStartLeaveGameNotification".to_owned(), game_finished_loading_notification: "RBXGameFinishedLoadingNotification".to_owned() }
}

// 0x248dc — -[PlaceLauncher dealloc]
// demangled: -[PlaceLauncher dealloc]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher dealloc]")]
pub fn stub_248dc(handle: u32) {
    // IDA 0x248dc: dealloc — SetCallback(0), teleporter released + zeroed (0x248e8..0x24902), three notification strings released (0x24920..0x24948), super dealloc (0x24960); ref traffic stays engine-side.
    let _ = handle;
}

// 0x24974 — +[PlaceLauncher sharedInstance]
// demangled: +[PlaceLauncher sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[PlaceLauncher sharedInstance]")]
pub fn stub_24974() -> Option<u32> {
    // IDA 0x24974: dispatch_once singleton (block 0x249d0 alloc+init); returns the cached handle.
    Some(0)
}

// 0x249d0 — ___31+[PlaceLauncher sharedInstance]_block_invoke
// demangled: ___31+[PlaceLauncher sharedInstance]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___31+[PlaceLauncher sharedInstance]_block_invoke")]
pub fn stub_249d0() -> Option<u32> {
    // IDA 0x249d0: sharedInstance block — alloc+init stored to the singleton slot; returns the fresh handle.
    Some(0)
}

// 0x24a04 — ___copy_helper_block__4
// demangled: ___copy_helper_block__4
// type: 
#[doc(alias = "___copy_helper_block__4")]
pub fn stub_24a04(dst: u32, src: u32) {
    // IDA 0x24a04: __copy_helper_block — single _Block_object_assign slot; block retain has no host carrier.
    let _ = (dst, src);
}

// 0x24a10 — ___destroy_helper_block__4
// demangled: ___destroy_helper_block__4
// type: 
#[doc(alias = "___destroy_helper_block__4")]
pub fn stub_24a10(handle: u32) {
    // IDA 0x24a10: __destroy_helper_block — single _Block_object_dispose slot; block release has no host carrier.
    let _ = handle;
}

// 0x24a18 — -[PlaceLauncher getIsCurrentlyPlayingGame]
// demangled: -[PlaceLauncher getIsCurrentlyPlayingGame]
// type: char __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher getIsCurrentlyPlayingGame]")]
pub fn stub_24a18(state: &PlaceLauncherState) -> bool {
    // IDA 0x24a18: returns isCurrentlyPlayingGame (0x24a26).
    state.is_currently_playing_game
}

// 0x24a28 — -[PlaceLauncher getDidLeaveGameNotification]
// demangled: -[PlaceLauncher getDidLeaveGameNotification]
// type: id __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher getDidLeaveGameNotification]")]
pub fn stub_24a28(state: &PlaceLauncherState) -> &str {
    // IDA 0x24a28: returns didLeaveGameNotification (0x24a36).
    &state.did_leave_game_notification
}

// 0x24a38 — -[PlaceLauncher getStartLeaveGameNotification]
// demangled: -[PlaceLauncher getStartLeaveGameNotification]
// type: id __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher getStartLeaveGameNotification]")]
pub fn stub_24a38(state: &PlaceLauncherState) -> &str {
    // IDA 0x24a38: returns startLeaveGameNotification (0x24a46).
    &state.start_leave_game_notification
}

// 0x24a48 — -[PlaceLauncher getGameFinishedLoadingNotification]
// demangled: -[PlaceLauncher getGameFinishedLoadingNotification]
// type: id __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher getGameFinishedLoadingNotification]")]
pub fn stub_24a48(state: &PlaceLauncherState) -> &str {
    // IDA 0x24a48: returns gameFinishedLoadingNotification (0x24a56).
    &state.game_finished_loading_notification
}

// 0x24a58 — -[PlaceLauncher handleStartGameFailure]
// demangled: -[PlaceLauncher handleStartGameFailure]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher handleStartGameFailure]")]
pub fn stub_24a58(state: &mut PlaceLauncherState, has_fallback_controller: bool) -> bool {
    // IDA 0x24a58: forwards handleStartGameFailure to the last non-game controller when present (0x24a76..0x24a98), then clears isCurrentlyPlayingGame (0x24aaa) — returns whether the failure was forwarded.
    state.is_currently_playing_game = false;
    has_fallback_controller
}

// 0x24ab0 — -[PlaceLauncher prepareGame]
// demangled: -[PlaceLauncher prepareGame]
// type: bool __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher prepareGame]")]
pub fn stub_24ab0(resource_path: &str, status: ReachabilityStatus, wifi_only: bool) -> PrepareGame {
    // IDA 0x24ab0: asset folder resourcePath+"/content" + globalInit + TeleportService base URL (0x24aea..0x24b36); WWAN + wifi-only pref alerts WiFiOnlyError (0x24b6a..0x24cb8), unreachable logs + alerts ConnectionError (0x24c2e..0x24c8a); DataModel hash "ios,ios" (0x24ccc..0x24d6e), settings loadState, TaskScheduler thread-count setup (0x24d78..0x24ddc) — Ready carries the folder + hash.
    match status {
        ReachabilityStatus::Cellular if wifi_only => PrepareGame::Alert("WiFiOnlyError"),
        ReachabilityStatus::None => PrepareGame::Alert("ConnectionError"),
        _ => PrepareGame::Ready { asset_folder: format!("{resource_path}/content"), datamodel_hash: "ios,ios".to_owned() },
    }
}

// 0x25080 — -[PlaceLauncher setLastPlaceId:]
// demangled: -[PlaceLauncher setLastPlaceId:]
// type: void __cdecl(PlaceLauncher *self, SEL, int)
#[doc(alias = "-[PlaceLauncher setLastPlaceId:]")]
pub fn stub_25080(state: &mut PlaceLauncherState, place_id: i32) {
    // IDA 0x25080: lastPlaceId = a3 (0x2508c).
    state.last_place_id = place_id;
}

// 0x25090 — -[PlaceLauncher checkPlacePartCount]
// demangled: -[PlaceLauncher checkPlacePartCount]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher checkPlacePartCount]")]
pub fn stub_25090(warnings_enabled: bool) -> bool {
    // IDA 0x25090: warnings_preference boolValue gates dispatch_async of the part-count block on the global queue (0x250da..0x25124) — returns whether the check was dispatched.
    warnings_enabled
}

// 0x2512c — ___36-[PlaceLauncher checkPlacePartCount]_block_invoke
// demangled: ___36-[PlaceLauncher checkPlacePartCount]_block_invoke
// type: 
#[doc(alias = "___36-[PlaceLauncher checkPlacePartCount]_block_invoke")]
pub fn stub_2512c(threshold: i32, part_count: Option<i32>, place_id: i32) -> Option<PartCountWarning> {
    // IDA 0x2512c: settings threshold (0x25178); skipped below 1 or when the datamodel chain is nil (0x25198..0x25222); threshold < part count alerts WarnPlaceIsNotIdeal + WarnTooManyParts format and tracks PlayErrors/TooManyParts labeled with the place id (0x2522e..0x25384).
    match part_count {
        Some(count) if threshold >= 1 && threshold < count => Some(PartCountWarning { title_key: "WarnPlaceIsNotIdeal", body_format_key: "WarnTooManyParts", event_category: "PlayErrors", event_action: "TooManyParts", place_id }),
        _ => None,
    }
}

/// Host wiring established by `-[PlaceLauncher setupDatamodelConnections:]` (IDA 0x25e00).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DatamodelConnections {
    pub open_url_window: bool,
    pub players_child_added: bool,
    pub prompt_login: bool,
}

/// Host work items of `-[PlaceLauncher finishGameSetup:gameViewController:]` (IDA 0x25498).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameViewSetup {
    pub game_view_id: String,
    pub overlay_view_id: String,
    pub screen_size: (u32, u32),
    pub finish_now: bool,
    pub has_overlay: bool,
}

/// Host outcome of `-[PlaceLauncher setLastNonGameController:]` (IDA 0x26170).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NonGameControllerSet {
    Cleared,
    Prepared,
    PrepareFailed,
}

/// Host game object selected by `-[PlaceLauncher setupGame:unsecuredGame:isApp:]` (IDA 0x26558).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameKind {
    UnsecuredStudio,
    SecurePlayer,
}

// 0x253cc — ___copy_helper_block_98
// demangled: ___copy_helper_block_98
// type: 
#[doc(alias = "___copy_helper_block_98")]
pub fn stub_253cc(dst: u32, src: u32) {
    // IDA 0x253cc: __copy_helper_block — single _Block_object_assign slot; block retain has no host carrier.
    let _ = (dst, src);
}

// 0x253d8 — ___destroy_helper_block_99
// demangled: ___destroy_helper_block_99
// type: 
#[doc(alias = "___destroy_helper_block_99")]
pub fn stub_253d8(handle: u32) {
    // IDA 0x253d8: __destroy_helper_block — single _Block_object_dispose slot; block release has no host carrier.
    let _ = handle;
}

// 0x253e0 — -[PlaceLauncher placeDidFinishLoading]
// demangled: -[PlaceLauncher placeDidFinishLoading]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher placeDidFinishLoading]")]
pub fn stub_253e0(state: &PlaceLauncherState) -> &str {
    // IDA 0x253e0: posts gameFinishedLoadingNotification (0x25400..0x25424), then checkPlacePartCount (0x2543c) — returns the posted name; caller runs stub_25090 next.
    &state.game_finished_loading_notification
}

// 0x25440 — -[PlaceLauncher deleteRobloxView]
// demangled: -[PlaceLauncher deleteRobloxView]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher deleteRobloxView]")]
pub fn stub_25440(has_view: bool) -> bool {
    // IDA 0x25440: non-nil rbxView zeroed + destroyed (0x2545a..0x25464), free-memory checker stopped (0x25480..0x25494) — returns whether a view was torn down.
    has_view
}

// 0x25498 — -[PlaceLauncher finishGameSetup:gameViewController:]
// demangled: -[PlaceLauncher finishGameSetup:gameViewController:]
// type: void __cdecl(PlaceLauncher *self, SEL, shared_ptr<RBX::Game>, id)
#[doc(alias = "-[PlaceLauncher finishGameSetup:gameViewController:]")]
pub fn stub_25498(game_window: u64, overlay_window: u64, screen: Option<(f32, f32)>, datamodel_loaded: bool, overlay_present: bool) -> GameViewSetup {
    // IDA 0x25498: stringstream window-handle ids (0x254e8..0x255f8), screen bounds or zero (0x25628..0x25670), RobloxView::create_view (0x256d2); loaded datamodel calls placeDidFinishLoading directly, else connects it as the loaded slot (0x25730..0x257e8); setupDatamodelConnections for datamodel + overlay when present (0x257f2..0x25898).
    let (w, h) = screen.unwrap_or((0.0, 0.0));
    GameViewSetup { game_view_id: game_window.to_string(), overlay_view_id: overlay_window.to_string(), screen_size: (w as u32, h as u32), finish_now: datamodel_loaded, has_overlay: overlay_present }
}

// 0x25e00 — -[PlaceLauncher setupDatamodelConnections:]
// demangled: -[PlaceLauncher setupDatamodelConnections:]
// type: void __cdecl(PlaceLauncher *self, SEL, shared_ptr<RBX::DataModel>)
#[doc(alias = "-[PlaceLauncher setupDatamodelConnections:]")]
pub fn stub_25e00(has_gui_service: bool, login_service_created: bool) -> DatamodelConnections {
    // IDA 0x25e00: GuiService present connects Ogre openUrlWindow: (0x25e2a..0x25eea); main-queue block starts the free-memory checker (0x25f04); Players childAdded: connected (0x25f18..0x25fcc); created LoginService connects handlePromptLoginSignal (0x25fd2..0x2606c).
    DatamodelConnections { open_url_window: has_gui_service, players_child_added: true, prompt_login: login_service_created }
}

// 0x2613c — ___43-[PlaceLauncher setupDatamodelConnections:]_block_invoke
// demangled: ___43-[PlaceLauncher setupDatamodelConnections:]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___43-[PlaceLauncher setupDatamodelConnections:]_block_invoke")]
pub fn stub_2613c() -> bool {
    // IDA 0x2613c: main-queue block starts the free-memory checker (0x26158..0x2616c) — returns whether the checker was started.
    true
}

// 0x26170 — -[PlaceLauncher setLastNonGameController:]
// demangled: -[PlaceLauncher setLastNonGameController:]
// type: void __cdecl(PlaceLauncher *self, SEL, id)
#[doc(alias = "-[PlaceLauncher setLastNonGameController:]")]
pub fn stub_26170(has_controller: bool, game_ready: bool) -> NonGameControllerSet {
    // IDA 0x26170: controller forwarded to MainViewController (0x26190..0x261a2); non-nil runs prepareGame, whose failure runs handleStartGameFailure (0x261a8..0x261d4).
    if !has_controller {
        NonGameControllerSet::Cleared
    } else if game_ready {
        NonGameControllerSet::Prepared
    } else {
        NonGameControllerSet::PrepareFailed
    }
}

// 0x261d8 — -[PlaceLauncher createGame:presentGameAutomatically:]
// demangled: -[PlaceLauncher createGame:presentGameAutomatically:]
// type: void __cdecl(PlaceLauncher *self, SEL, shared_ptr<RBX::Game>, char)
#[doc(alias = "-[PlaceLauncher createGame:presentGameAutomatically:]")]
pub fn stub_261d8(state: &mut PlaceLauncherState, has_host_controller: bool) -> bool {
    // IDA 0x261d8: clears hasReceivedMemoryWarning + deletes the old view (0x26212..0x26216); with a host controller the caller allocs the game VC, runs finishGameSetup and submits initControlView (0x26246..0x2638a) — returns whether creation proceeds.
    state.has_received_memory_warning = false;
    has_host_controller
}

// 0x2643c — __ZL15initControlViewP10RobloxViewaPN3RBX18FunctionMarshallerE
// demangled: initControlView(RobloxView *,signed char,RBX::FunctionMarshaller *)
// type: _DWORD __fastcall(RobloxView *, signed __int8, RBX::FunctionMarshaller *)
#[doc(alias = "initControlView(RobloxView *,signed char,RBX::FunctionMarshaller *)")]
pub fn stub_2643c(is_app: bool) -> bool {
    // IDA 0x2643c: binds initControlViewHelper(view, isApp) and executes it on the marshaller (0x26478..0x264b8) — returns the captured flag.
    is_app
}

// 0x26520 — -[PlaceLauncher setupGame:isApp:]
// demangled: -[PlaceLauncher setupGame:isApp:]
// type: shared_ptr<RBX::Game> *__cdecl(shared_ptr<RBX::Game> *__return_ptr __struct_ptr retstr, PlaceLauncher *self, SEL, id, char)
#[doc(alias = "-[PlaceLauncher setupGame:isApp:]")]
pub fn stub_26520(has_launcher: bool) -> bool {
    // IDA 0x26520: forwards to setupGame:unsecuredGame:0 isApp: (0x26544); nil self yields a nil game (0x2654c) — returns whether setup proceeds.
    has_launcher
}

// 0x26558 — -[PlaceLauncher setupGame:unsecuredGame:isApp:]
// demangled: -[PlaceLauncher setupGame:unsecuredGame:isApp:]
// type: shared_ptr<RBX::Game> *__cdecl(shared_ptr<RBX::Game> *__return_ptr __struct_ptr retstr, PlaceLauncher *self, SEL, id, char, char)
#[doc(alias = "-[PlaceLauncher setupGame:unsecuredGame:isApp:]")]
pub fn stub_26558(already_playing: bool, unsecured: bool, is_app: bool) -> Option<(GameKind, bool)> {
    // IDA 0x26558: already playing yields a nil game (0x26594..0x265bc); else ClientAppSettings init + iOSAppSettings fetch (0x265ca..0x265ec), forced settings read (0x26610), idle timer disabled (0x26642), playing=1 + setLastNonGameController (0x26650..0x2665c); UnsecuredStudioGame vs SecurePlayerGame on a5 (0x2666e..0x266ec).
    if already_playing {
        None
    } else {
        Some((if unsecured { GameKind::UnsecuredStudio } else { GameKind::SecurePlayer }, is_app))
    }
}

/// Host spawn request of `-[PlaceLauncher injectJoinScript:]` (IDA 0x267ec).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JoinScriptSpawn {
    pub script: String,
    pub thread_name: &'static str,
}

/// Host forward of `-[PlaceLauncher setupPreloadedGameWithNonGameController:unsecuredGame:isApp:]` (IDA 0x26784).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PreloadedGameRequest {
    pub unsecured: bool,
    pub is_app: bool,
}

/// Host join captured by `-[PlaceLauncher startGameLocal:...]` (IDA 0x26bb8).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalGameJoin {
    pub port: i32,
    pub ip: String,
}

// 0x26768 — -[PlaceLauncher presentGameViewController]
// demangled: -[PlaceLauncher presentGameViewController]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher presentGameViewController]")]
pub fn stub_26768() {
    // IDA 0x26768: dispatch_async of the presentation block on the main queue (0x2677e); dispatch engine-side, faithful no-op shell.
}

// 0x26784 — -[PlaceLauncher setupPreloadedGameWithNonGameController:unsecuredGame:isApp:]
// demangled: -[PlaceLauncher setupPreloadedGameWithNonGameController:unsecuredGame:isApp:]
// type: shared_ptr<RBX::Game> *__cdecl(shared_ptr<RBX::Game> *__return_ptr __struct_ptr retstr, PlaceLauncher *self, SEL, id, char, char)
#[doc(alias = "-[PlaceLauncher setupPreloadedGameWithNonGameController:unsecuredGame:isApp:]")]
pub fn stub_26784(has_launcher: bool, unsecured: bool, is_app: bool) -> Option<PreloadedGameRequest> {
    // IDA 0x26784: forwards to setupGame:unsecuredGame:isApp: (0x267a8); nil self yields a nil game (0x267b0) — caller feeds the request into stub_26558.
    has_launcher.then_some(PreloadedGameRequest { unsecured, is_app })
}

// 0x267bc — -[PlaceLauncher setupPreloadedGameWithNonGameController:isApp:]
// demangled: -[PlaceLauncher setupPreloadedGameWithNonGameController:isApp:]
// type: shared_ptr<RBX::Game> *__cdecl(shared_ptr<RBX::Game> *__return_ptr __struct_ptr retstr, PlaceLauncher *self, SEL, id, char)
#[doc(alias = "-[PlaceLauncher setupPreloadedGameWithNonGameController:isApp:]")]
pub fn stub_267bc(has_launcher: bool, is_app: bool) -> Option<bool> {
    // IDA 0x267bc: forwards to setupGame:isApp: (0x267d8); nil self yields a nil game (0x267e2) — returns the isApp flag to forward.
    has_launcher.then_some(is_app)
}

// 0x267ec — -[PlaceLauncher injectJoinScript:]
// demangled: -[PlaceLauncher injectJoinScript:]
// type: void __cdecl(PlaceLauncher *self, SEL, id)
#[doc(alias = "-[PlaceLauncher injectJoinScript:]")]
pub fn stub_267ec(script: &str) -> JoinScriptSpawn {
    // IDA 0x267ec: UTF8String of the join script (0x2681c) bound with the game into joinGameWithJoinScript and run on a detached "InjectStartScript" thread (0x2687e..0x268b2).
    JoinScriptSpawn { script: script.to_owned(), thread_name: "InjectStartScript" }
}

// 0x26990 — __ZL22joinGameWithJoinScriptRKSsN5boost10shared_ptrIN3RBX4GameEEE // was: boost::shared_ptr
// demangled: joinGameWithJoinScript(std::string const&,boost::shared_ptr<RBX::Game>)
// type: 
#[doc(alias = "joinGameWithJoinScript(std::string const&,rbx_core::SharedPtr<RBX::Game>)")]
pub fn stub_26990(script: &str) -> &str {
    // IDA 0x26990: joinGameWithJoinScript runs executeUrlScript(game, script) (0x269fa..0x26a06) — returns the script the engine executes.
    script
}

// 0x26bb8 — -[PlaceLauncher startGameLocal:ipAddress:controller:presentGameAutomatically:]
// demangled: -[PlaceLauncher startGameLocal:ipAddress:controller:presentGameAutomatically:]
// type: char __cdecl(PlaceLauncher *self, SEL, int, id, id, char)
#[doc(alias = "-[PlaceLauncher startGameLocal:ipAddress:controller:presentGameAutomatically:]")]
pub fn stub_26bb8(has_launcher: bool, game_ready: bool, started: bool, port: i32, ip: &str) -> Option<(LocalGameJoin, bool)> {
    // IDA 0x26bb8: nil self yields false (0x26cfc..0x26d02); preloaded unsecured game setup (0x26c34) gates binding joinLocalGame(port, ip) (0x26c84) and startGame:controller:preloadedGame:presentGameAutomatically: whose result returns (0x26cc8..0x26d2c).
    if !has_launcher || !game_ready {
        None
    } else {
        Some((LocalGameJoin { port, ip: ip.to_owned() }, started))
    }
}

// 0x26dd4 — __ZL13joinLocalGameiRKSsN5boost10shared_ptrIN3RBX4GameEEE // was: boost::shared_ptr
// demangled: joinLocalGame(int,std::string const&,boost::shared_ptr<RBX::Game>)
// type: 
#[doc(alias = "joinLocalGame(int,std::string const&,rbx_core::SharedPtr<RBX::Game>)")]
pub fn stub_26dd4(base_url: &str, port: i32, server: &str) -> String {
    // IDA 0x26dd4: RBX::format "%sGame/Join.ashx?userID=0&serverPort=%i&server=%s" (0x26e76), executeUrlScript engine-side (0x26e98) — returns the join URL.
    format!("{base_url}Game/Join.ashx?userID=0&serverPort={port}&server={server}")
}

/// Host load captured by `-[PlaceLauncher startAppWithFile:...]` (IDA 0x27054).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalAppLoad {
    pub file: String,
}

/// Host join captured by `-[PlaceLauncher startAppWithId:...]` (IDA 0x276b0).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceIdJoin {
    pub place_id: i32,
    pub request: JoinGameRequest,
}

// 0x27054 — -[PlaceLauncher startAppWithFile:controller:presentGameAutomatically:]
// demangled: -[PlaceLauncher startAppWithFile:controller:presentGameAutomatically:]
// type: char __cdecl(PlaceLauncher *self, SEL, id, id, char)
#[doc(alias = "-[PlaceLauncher startAppWithFile:controller:presentGameAutomatically:]")]
pub fn stub_27054(has_launcher: bool, game_ready: bool, started: bool, file: &str) -> Option<(LocalAppLoad, bool)> {
    // IDA 0x27054: nil self yields false (0x2718e..0x27194); preloaded game setup (0x270cc) gates binding loadLocalApp(file) (0x27116) and startGame:controller:preloadedGame:presentGameAutomatically: whose result returns (0x2715a..0x271be).
    if !has_launcher || !game_ready {
        None
    } else {
        Some((LocalAppLoad { file: file.to_owned() }, started))
    }
}

// 0x27268 — __ZL12loadLocalAppRKSsN5boost10shared_ptrIN3RBX4GameEEE // was: boost::shared_ptr
// demangled: loadLocalApp(std::string const&,boost::shared_ptr<RBX::Game>)
// type: 
#[doc(alias = "loadLocalApp(std::string const&,rbx_core::SharedPtr<RBX::Game>)")]
pub fn stub_27268(file: &str, has_game: bool) -> Option<String> {
    // IDA 0x27268: RBX::format "Game:Load('rbxasset://%s')" (0x272c8), executeScript on the non-nil game (0x27306..0x27344) — returns the load script.
    has_game.then(|| format!("Game:Load('rbxasset://{file}')"))
}

// 0x276b0 — -[PlaceLauncher startAppWithId:controller:presentGameAutomatically:]
// demangled: -[PlaceLauncher startAppWithId:controller:presentGameAutomatically:]
// type: char __cdecl(PlaceLauncher *self, SEL, int, id, char)
#[doc(alias = "-[PlaceLauncher startAppWithId:controller:presentGameAutomatically:]")]
pub fn stub_276b0(has_launcher: bool, game_ready: bool, started: bool, place_id: i32) -> Option<(PlaceIdJoin, bool)> {
    // IDA 0x276b0: preloaded game setup via setupPreloadedGameWithNonGameController:isApp: (0x27726) gates binding joinGamePlaceId(placeId, JoinGameRequest 2) (0x2775e) and startGame:controller:preloadedGame:presentGameAutomatically: whose result returns (0x277a2).
    if !has_launcher || !game_ready {
        None
    } else {
        Some((PlaceIdJoin { place_id, request: JoinGameRequest::AppStart }, started))
    }
}

/// `JoinGameRequest` selector of `joinGamePlaceId` (IDA 0x278a8, cf. 0x27a02..0x27a6e).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinGameRequest {
    Place = 0,
    FollowUser = 1,
    AppStart = 2,
}

/// Poll decision inside the PlaceLauncher.ashx retry loop (IDA 0x27c4a..0x27d32, 5 tries).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LauncherPoll {
    Success,
    RetryCountdown,
    RetryWait,
}

/// Failure alert of the exhausted join loop (IDA 0x28052..0x28158).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinFailureAlert {
    ConnectionError,
    GameFull,
    GameEnded,
}

/// Outcome of `joinGamePlaceId` (IDA 0x278a8); the success tail applies
/// setLastPlaceId + SessionReporter(3, id) + Visit/Success/Join (0x27b64..0x27bd2),
/// the failure tail runs leaveGame + handleStartGameFailure (0x2821c..0x2825a).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaceJoinOutcome {
    Joined { script_url: String },
    Signed { body: String },
    Failed { alert: JoinFailureAlert },
}

/// Request kind selector (IDA 0x27a02..0x27a6e): 1 follows a user, anything else joins a place.
pub fn join_request_parts(request: JoinGameRequest) -> (&'static str, &'static str) {
    match request {
        JoinGameRequest::FollowUser => ("userId", "RequestFollowUser"),
        _ => ("placeId", "RequestGame"),
    }
}

/// PlaceLauncher.ashx join URL (IDA 0x27c42).
pub fn join_launcher_url(base_url: &str, request: JoinGameRequest, id: i32) -> String {
    let (param, name) = join_request_parts(request);
    format!("{base_url}Game/PlaceLauncher.ashx?request={name}&{param}={id}&isPartyLeader=false&gender=&isTeleport=false")
}

/// AppStart URL for the overlay path (IDA 0x27a96).
pub fn join_app_start_url(base_url: &str, id: i32) -> String {
    format!("{base_url}Game/AppStart.ashx?appid={id}")
}

/// Retry-loop decision (IDA 0x27cca..0x27d32): "status":2 succeeds; "status":0/1
/// retries with countdown; anything else retries without consuming a try.
pub fn join_poll_decision(response: &str) -> LauncherPoll {
    if response.contains("\"status\":2") {
        LauncherPoll::Success
    } else if response.contains("\"status\":0") || response.contains("\"status\":1") {
        LauncherPoll::RetryCountdown
    } else {
        LauncherPoll::RetryWait
    }
}

/// Failure alert for an exhausted loop (IDA 0x28052..0x280e4).
pub fn join_failure_alert(response: &str) -> JoinFailureAlert {
    if response.contains("\"status\":5") {
        JoinFailureAlert::GameEnded
    } else if response.contains("\"status\":6") {
        JoinFailureAlert::GameFull
    } else {
        JoinFailureAlert::ConnectionError
    }
}

/// joinScriptUrl value through the next comma with "\/" unescaped (IDA 0x27e24..0x27ef6); empty when absent.
pub fn extract_join_script_url(response: &str) -> String {
    let key = "joinScriptUrl";
    let Some(pos) = response.find(key) else {
        return String::new();
    };
    let rest = &response[pos + key.len()..];
    let Some(start) = rest.find(|c: char| c != '"' && c != ':' && c != ' ' && c != '=') else {
        return String::new();
    };
    let rest = &rest[start..];
    let end = rest.find(',').unwrap_or(rest.len());
    rest[..end].trim_matches('"').replace("\\/", "/")
}

// 0x278a8 — __ZL15joinGamePlaceIdiN5boost10shared_ptrIN3RBX4GameEEE15JoinGameRequest // was: boost::shared_ptr
// demangled: joinGamePlaceId(int,boost::shared_ptr<RBX::Game>,JoinGameRequest)
// type: 
#[doc(alias = "joinGamePlaceId(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest)")]
pub fn stub_278a8<F>(base_url: &str, place_id: i32, request: JoinGameRequest, overlay_app_start: bool, mut get: F) -> PlaceJoinOutcome
where
    F: FnMut(&str) -> String,
{
    // IDA 0x278a8: logs + UserAgent default (0x278fe..0x279e4); AppStart GET when overlay-enabled and request==2 (0x27a6e..0x27b2a, executeSignedScript); else the PlaceLauncher.ashx poll loop (0x27c42..0x27d32), joinScriptUrl extraction + executeUrlScript (0x27e24..0x27ef6), failure alerts + leaveGame/handleStartGameFailure (0x27dce..0x2825a); HTTP stays with the caller-provided fetch.
    if overlay_app_start && request == JoinGameRequest::AppStart {
        return PlaceJoinOutcome::Signed { body: get(&join_app_start_url(base_url, place_id)) };
    }
    let url = join_launcher_url(base_url, request, place_id);
    let mut body = String::new();
    let mut tries = 5;
    loop {
        body = get(&url);
        match join_poll_decision(&body) {
            LauncherPoll::Success => break,
            LauncherPoll::RetryCountdown => {
                tries -= 1;
                if tries < 0 {
                    return PlaceJoinOutcome::Failed { alert: join_failure_alert(&body) };
                }
            }
            LauncherPoll::RetryWait => {}
        }
    }
    PlaceJoinOutcome::Joined { script_url: extract_join_script_url(&body) }
}

/// Host solo script selected by `joinGamePlaceIdSolo` (IDA 0x28d98).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SoloJoinScript {
    Workshop { base_url: String },
    Visit { base_url: String, place_id: i32 },
}

impl SoloJoinScript {
    pub fn source(&self) -> String {
        match self {
            SoloJoinScript::Workshop { base_url } => format!("game:Load('rbxasset://places/workshop/workshopStartPlace.rbxl') loadfile('{base_url}game/visit.ashx')()"),
            SoloJoinScript::Visit { base_url, place_id } => format!("loadfile('{base_url}game/visit.ashx?placeid={place_id}')()"),
        }
    }
}

// 0x289a8 — -[PlaceLauncher startGame:controller:request:presentGameAutomatically:]
// demangled: -[PlaceLauncher startGame:controller:request:presentGameAutomatically:]
// type: char __cdecl(PlaceLauncher *self, SEL, int, id, int, char)
#[doc(alias = "-[PlaceLauncher startGame:controller:request:presentGameAutomatically:]")]
pub fn stub_289a8(has_launcher: bool, game_ready: bool, started: bool, place_id: i32, request: JoinGameRequest) -> Option<(PlaceIdJoin, bool)> {
    // IDA 0x289a8: preloaded setup with isApp = (request == 2) (0x28a2a) gates binding joinGamePlaceId(placeId, request) (0x28a60) and startGame:controller:preloadedGame:presentGameAutomatically: whose result returns (0x28aa4..0x28b08) — the bool is the setup isApp flag.
    if !has_launcher || !game_ready {
        None
    } else {
        Some((PlaceIdJoin { place_id, request }, request == JoinGameRequest::AppStart))
    }
}

// 0x28ba8 — -[PlaceLauncher startGameSolo:controller:presentGameAutomatically:]
// demangled: -[PlaceLauncher startGameSolo:controller:presentGameAutomatically:]
// type: char __cdecl(PlaceLauncher *self, SEL, int, id, char)
#[doc(alias = "-[PlaceLauncher startGameSolo:controller:presentGameAutomatically:]")]
pub fn stub_28ba8(has_launcher: bool, game_ready: bool, started: bool, place_id: i32) -> Option<(i32, bool)> {
    // IDA 0x28ba8: preloaded setup (0x28c1e) gates binding joinGamePlaceIdSolo(placeId) (0x28c50) and startGame:controller:preloadedGame:presentGameAutomatically: whose result returns (0x28c94) — returns the bound place id + start result.
    if !has_launcher || !game_ready {
        None
    } else {
        Some((place_id, started))
    }
}

// 0x28d98 — __ZL19joinGamePlaceIdSoloiN5boost10shared_ptrIN3RBX4GameEEE // was: boost::shared_ptr
// demangled: joinGamePlaceIdSolo(int,boost::shared_ptr<RBX::Game>)
// type: 
#[doc(alias = "joinGamePlaceIdSolo(int,rbx_core::SharedPtr<RBX::Game>)")]
pub fn stub_28d98(base_url: &str, place_id: i32) -> SoloJoinScript {
    // IDA 0x28d98: registers the UserAgent default (0x28e16..0x28e8a); place < 1 loads the workshop start place + base visit.ashx (0x28ebc..0x28f68), else visit.ashx?placeid=N (0x28ec2..0x28efa); executeScript engine-side (0x28f96).
    if place_id < 1 {
        SoloJoinScript::Workshop { base_url: base_url.to_owned() }
    } else {
        SoloJoinScript::Visit { base_url: base_url.to_owned(), place_id }
    }
}
/// Host join captured by `-[PlaceLauncher startGameWithJoinScript:...]` (IDA 0x29280).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JoinScriptJoin {
    pub script: String,
}

/// Host spawn of `-[PlaceLauncher startGame:controller:preloadedGame:...]` (IDA 0x29490).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameStartSpawn {
    pub thread_name: &'static str,
    pub present_automatically: bool,
}

/// Host path of `-[PlaceLauncher leaveGame]` (IDA 0x298e0).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeaveGameAction {
    ShutdownDirect,
    ShutdownDispatched,
}

// 0x29280 — -[PlaceLauncher startGameWithJoinScript:controller:presentGameAutomatically:]
// demangled: -[PlaceLauncher startGameWithJoinScript:controller:presentGameAutomatically:]
// type: char __cdecl(PlaceLauncher *self, SEL, id, id, char)
#[doc(alias = "-[PlaceLauncher startGameWithJoinScript:controller:presentGameAutomatically:]")]
pub fn stub_29280(has_launcher: bool, game_ready: bool, started: bool, script: &str) -> Option<(JoinScriptJoin, bool)> {
    // IDA 0x29280: nil self yields false (0x293b8..0x293be); setupPreloadedGameWithNonGameController:isApp: (0x292f4) gates binding joinGameWithJoinScript(script UTF8) (0x29314..0x2934c) and startGame:controller:preloadedGame:presentGameAutomatically: whose result returns (0x29352..0x293e8) — boost::bind/function0 map to the captured script closure.
    if !has_launcher || !game_ready {
        None
    } else {
        Some((JoinScriptJoin { script: script.to_owned() }, started))
    }
}

// 0x29490 — -[PlaceLauncher startGame:controller:preloadedGame:presentGameAutomatically:]
// demangled: -[PlaceLauncher startGame:controller:preloadedGame:presentGameAutomatically:]
// type: char __cdecl(PlaceLauncher *self, SEL, function0<void>, id, shared_ptr<RBX::Game>, char)
#[doc(alias = "-[PlaceLauncher startGame:controller:preloadedGame:presentGameAutomatically:]")]
pub fn stub_29490(present_automatically: bool) -> GameStartSpawn {
    // IDA 0x29490: RBX::thread_wrapper + boost::thread run the join closure as "GameStartScript" (0x294c0..0x294fc), then createGame:presentGameAutomatically: (0x29510..0x29534); always returns 1 (0x29560) — boost::thread maps to std::thread, caller runs stub_261d8 next.
    GameStartSpawn { thread_name: "GameStartScript", present_automatically }
}

// 0x295c0 — -[PlaceLauncher leaveGameShutdown]
// demangled: -[PlaceLauncher leaveGameShutdown]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher leaveGameShutdown]")]
pub fn stub_295c0(state: &PlaceLauncherState) -> &str {
    // IDA 0x295c0: posts startLeaveGameNotification (0x295fe..0x29622), then dismissViewControllerAnimated:completion: of the ogre controller with the 0x29684 block (0x29634..0x2967c) — returns the posted name; caller runs stub_29684 next.
    &state.start_leave_game_notification
}

// 0x29684 — ___34-[PlaceLauncher leaveGameShutdown]_block_invoke
// demangled: ___34-[PlaceLauncher leaveGameShutdown]_block_invoke
// type:
#[doc(alias = "___34-[PlaceLauncher leaveGameShutdown]_block_invoke")]
pub fn stub_29684(state: &mut PlaceLauncherState) -> &str {
    // IDA 0x29684: releases + nils the ogre controller/view/window (0x2969e..0x296ee), deleteRobloxView (0x29700), clears playing/leaving/warning flags (0x2971c..0x297e8), posts didLeaveGameNotification (0x29740..0x29764), drops RobloxGameState defaults (0x29790..0x297c2), ends the bg task (0x297f4..0x29872) — returns the posted name.
    state.is_currently_playing_game = false;
    state.is_leaving_game = false;
    state.has_received_memory_warning = false;
    &state.did_leave_game_notification
}

// 0x298a0 — ___copy_helper_block_191
// demangled: ___copy_helper_block_191
// type:
#[doc(alias = "___copy_helper_block_191")]
pub fn stub_298a0(dst: u32, src: u32) {
    // IDA 0x298a0: __copy_helper_block_191 — two _Block_object_assign slots (0x298b0..0x298c0); block retain has no host carrier.
    let _ = (dst, src);
}

// 0x298c4 — ___destroy_helper_block_192
// demangled: ___destroy_helper_block_192
// type:
#[doc(alias = "___destroy_helper_block_192")]
pub fn stub_298c4(handle: u32) {
    // IDA 0x298c4: __destroy_helper_block_192 — two _Block_object_dispose slots (0x298ce..0x298da); block release has no host carrier.
    let _ = handle;
}

// 0x298e0 — -[PlaceLauncher leaveGame]
// demangled: -[PlaceLauncher leaveGame]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher leaveGame]")]
pub fn stub_298e0(state: &mut PlaceLauncherState, has_ogre_controller: bool, system_version: f32) -> Option<LeaveGameAction> {
    // IDA 0x298e0: no-ops unless currently playing, not already leaving, and the ogre controller exists (0x2996e..0x2998e); else sets isLeavingGame + idle timer off + RobloxGameState=leaveGame (0x299a2..0x29a36), closeChildConnections + SessionReporter(4) + Visit/Success/LeaveGame (0x29a48..0x29a92), bg task begin (0x29aec..0x29b12); iOS >= 6.0 dispatches leaveGameShutdown async (0x29b62..0x29ba8), older runs it inline (0x29b72).
    if !state.is_currently_playing_game || state.is_leaving_game || !has_ogre_controller {
        return None;
    }
    state.is_leaving_game = true;
    if system_version >= 6.0 {
        Some(LeaveGameAction::ShutdownDispatched)
    } else {
        Some(LeaveGameAction::ShutdownDirect)
    }
}

// 0x29bb4 — ___26-[PlaceLauncher leaveGame]_block_invoke
// demangled: ___26-[PlaceLauncher leaveGame]_block_invoke
// type: 
#[doc(alias = "___26-[PlaceLauncher leaveGame]_block_invoke")]
pub fn stub_29bb4(state: &mut PlaceLauncherState) -> bool {
    // IDA 0x29bb4: expiration handler clears isLeavingGame (0x29bde) and ends the bg task (0x29be8..0x29c0c) — returns whether the task was ended.
    state.is_leaving_game = false;
    true
}

// 0x29c34 — ___copy_helper_block_217
// demangled: ___copy_helper_block_217
// type: 
#[doc(alias = "___copy_helper_block_217")]
pub fn stub_29c34(dst: u32, src: u32) {
    // IDA 0x29c34: __copy_helper_block_217 — two _Block_object_assign slots (0x29c44..0x29c54); block retain has no host carrier.
    let _ = (dst, src);
}

// 0x29c58 — ___destroy_helper_block_218
// demangled: ___destroy_helper_block_218
// type: 
#[doc(alias = "___destroy_helper_block_218")]
pub fn stub_29c58(handle: u32) {
    // IDA 0x29c58: __destroy_helper_block_218 — two _Block_object_dispose slots (0x29c62..0x29c6e); block release has no host carrier.
    let _ = handle;
}

// 0x29c74 — ___26-[PlaceLauncher leaveGame]_block_invoke231
// demangled: ___26-[PlaceLauncher leaveGame]_block_invoke231
// type: 
#[doc(alias = "___26-[PlaceLauncher leaveGame]_block_invoke231")]
pub fn stub_29c74(state: &PlaceLauncherState) -> &str {
    // IDA 0x29c74: main-queue block forwards to leaveGameShutdown (0x29ba2) — returns the notification stub_295c0 posts.
    &state.start_leave_game_notification
}

// 0x29c88 — ___copy_helper_block_232
// demangled: ___copy_helper_block_232
// type: 
#[doc(alias = "___copy_helper_block_232")]
pub fn stub_29c88(dst: u32, src: u32) {
    // IDA 0x29c88: __copy_helper_block_232 — single _Block_object_assign slot (0x29c8e); block retain has no host carrier.
    let _ = (dst, src);
}

// 0x29c94 — ___destroy_helper_block_233
// demangled: ___destroy_helper_block_233
// type: 
#[doc(alias = "___destroy_helper_block_233")]
pub fn stub_29c94(handle: u32) {
    // IDA 0x29c94: __destroy_helper_block_233 — single _Block_object_dispose slot (0x29c98); block release has no host carrier.
    let _ = handle;
}

// 0x29c9c — -[PlaceLauncher disableViewBecauseGoingToBackground]
// demangled: -[PlaceLauncher disableViewBecauseGoingToBackground]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher disableViewBecauseGoingToBackground]")]
pub fn stub_29c9c(has_view: bool) -> bool {
    // IDA 0x29c9c: non-nil rbxView runs RobloxView::requestStopRenderingForBackgroundMode (0x29ca8..0x29cae) — returns whether rendering was stopped.
    has_view
}

// 0x29cb4 — -[PlaceLauncher enableViewBecauseGoingToForeground]
// demangled: -[PlaceLauncher enableViewBecauseGoingToForeground]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher enableViewBecauseGoingToForeground]")]
pub fn stub_29cb4(has_view: bool) -> bool {
    // IDA 0x29cb4: non-nil rbxView runs RobloxView::requestResumeRendering (0x29cc0..0x29cc6) — returns whether rendering was resumed.
    has_view
}

// 0x29ccc — -[PlaceLauncher teleport:withAuthentication:withScript:]
// demangled: -[PlaceLauncher teleport:withAuthentication:withScript:]
// type: void __cdecl(PlaceLauncher *self, SEL, id, id, id)
#[doc(alias = "-[PlaceLauncher teleport:withAuthentication:withScript:]")]
pub fn stub_29ccc(place: &str, auth: &str, script: &str) -> TeleportRequest {
    // IDA 0x29ccc: stashes the last non-game controller (0x29d0a..0x29d42), builds a SecurePlayerGame + binds joinGameTeleport(place, auth, script) on a detached thread (0x29d58..0x29e40, boost::thread maps to std::thread), deleteRobloxView (0x29ec6), then a 0.5s UIView animation running the 0x2a8c8 frame block with 0x2a99c completion (0x29f04..0x29fca) — returns the bound teleport triple; caller runs stub_2a350 for the fetch leg and stub_2a8c8/stub_2a99c for the animation legs.
    TeleportRequest { place: place.to_owned(), auth: auth.to_owned(), script: script.to_owned(), animation_secs: TELEPORT_ANIM_SECS }
}

/// Teleport animation length of `-[PlaceLauncher teleport:...]` and `finishTeleportHelper` (IDA 0x29fca, 0x2b90e).
pub const TELEPORT_ANIM_SECS: f64 = 0.5;

/// Host triple bound by `-[PlaceLauncher teleport:withAuthentication:withScript:]` (IDA 0x29ccc).
#[derive(Debug, Clone, PartialEq)]
pub struct TeleportRequest {
    pub place: String,
    pub auth: String,
    pub script: String,
    pub animation_secs: f64,
}

/// Host outcome of `-[PlaceLauncher applicationDidReceiveMemoryWarning]` (IDA 0x2ae54).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryWarningOutcome {
    Ignored,
    Shutdown { early_exit: bool, alert: bool },
}

/// Host route of `-[PlaceLauncher childAdded:]` (IDA 0x2b1bc).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChildAddedRoute {
    NoView,
    NoDatamodel,
    NoPlayers,
    NoPlayer,
    PlayerIsChild,
    PlayerOther,
}

// 0x2a350 — __ZL16joinGameTeleportSsSsSsP8NSObjectN5boost10shared_ptrIN3RBX4GameEEE // was: boost::shared_ptr
// demangled: joinGameTeleport(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>)
// type: 
#[doc(alias = "joinGameTeleport(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>)")]
pub fn stub_2a350(base_url: &str, place: &str, suggest: &str, has_controller: bool) -> (String, bool) {
    // IDA 0x2a350: copies place, appends "?suggest="+auth when non-empty (0x2a3b8..0x2a3dc), RBX::Http GET of base+url (0x2a3f2..0x2a438), executeUrlScript with the game (0x2a46e..0x2a48a), then handleStartGameSuccess on the controller when non-nil (0x2a49c..0x2a4b0) — returns the fetched URL + whether success was notified; HTTP stays with the caller.
    let mut url = format!("{base_url}{place}");
    if !suggest.is_empty() {
        url.push_str("?suggest=");
        url.push_str(suggest);
    }
    (url, has_controller)
}

// 0x2a8c8 — ___56-[PlaceLauncher teleport:withAuthentication:withScript:]_block_invoke
// demangled: ___56-[PlaceLauncher teleport:withAuthentication:withScript:]_block_invoke
// type: 
#[doc(alias = "___56-[PlaceLauncher teleport:withAuthentication:withScript:]_block_invoke")]
pub fn stub_2a8c8(has_view: bool, width: f32, height: f32) -> (f32, f32) {
    // IDA 0x2a8c8: animation block halves the ogre view frame origin (vmul 0.5, 0x2a908..0x2a914) and re-sets the frame (0x2a940..0x2a984); nil view centers at zero (0x2a920..0x2a922).
    if has_view { (width * 0.5, height * 0.5) } else { (0.0, 0.0) }
}

// 0x2a988 — ___copy_helper_block_243
// demangled: ___copy_helper_block_243
// type: 
#[doc(alias = "___copy_helper_block_243")]
pub fn stub_2a988(dst: u32, src: u32) {
    // IDA 0x2a988: __copy_helper_block_243 — single _Block_object_assign slot (0x2a98e); block retain has no host carrier.
    let _ = (dst, src);
}

// 0x2a994 — ___destroy_helper_block_244
// demangled: ___destroy_helper_block_244
// type: 
#[doc(alias = "___destroy_helper_block_244")]
pub fn stub_2a994(handle: u32) {
    // IDA 0x2a994: __destroy_helper_block_244 — single _Block_object_dispose slot (0x2a998); block release has no host carrier.
    let _ = handle;
}

// 0x2a99c — ___56-[PlaceLauncher teleport:withAuthentication:withScript:]_block_invoke246
// demangled: ___56-[PlaceLauncher teleport:withAuthentication:withScript:]_block_invoke246
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "___56-[PlaceLauncher teleport:withAuthentication:withScript:]_block_invoke246")]
pub fn stub_2a99c(has_launcher: bool) -> bool {
    // IDA 0x2a99c: completion runs finishGameSetup:gameViewController: (0x2aa18), then binds finishTeleport(view, game, marshaller) and submits it via RBX::DataModel::submitTask (0x2aa3c..0x2aaaa) — boost::bind/function map to a host closure; returns whether the task was submitted.
    has_launcher
}

// 0x2aba4 — __ZL14finishTeleportP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEEPNS3_18FunctionMarshallerE // was: boost::shared_ptr
// demangled: finishTeleport(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *)
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "finishTeleport(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *)")]
pub fn stub_2aba4(has_view: bool, has_game: bool) -> bool {
    // IDA 0x2aba4: binds finishTeleportHelper(view, game) and executes it on the marshaller (0x2abd8..0x2ac32, RBX::FunctionMarshaller::Execute) — boost::bind/function map to a host closure; returns whether dispatch proceeds.
    has_view && has_game
}

// 0x2acec — ___copy_helper_block_247
// demangled: ___copy_helper_block_247
// type: void __fastcall(_DWORD *, const shared_count *)
#[doc(alias = "___copy_helper_block_247")]
pub fn stub_2acec(dst: u32, src: u32) {
    // IDA 0x2acec: __copy_helper_block_247 — two _Block_object_assign slots + a shared_count copy (0x2ad18..0x2ad64); block retain has no host carrier.
    let _ = (dst, src);
}

// 0x2ada4 — ___destroy_helper_block_248
// demangled: ___destroy_helper_block_248
// type: 
#[doc(alias = "___destroy_helper_block_248")]
pub fn stub_2ada4(handle: u32) {
    // IDA 0x2ada4: __destroy_helper_block_248 — two _Block_object_dispose slots + a shared_count release (0x2adc6..0x2ae06); block release has no host carrier.
    let _ = handle;
}

// 0x2ae44 — -[PlaceLauncher isCurrentlyPlayingGame]
// demangled: -[PlaceLauncher isCurrentlyPlayingGame]
// type: char __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher isCurrentlyPlayingGame]")]
pub fn stub_2ae44(state: &PlaceLauncherState) -> bool {
    // IDA 0x2ae44: returns self->isCurrentlyPlayingGame (0x2ae52).
    state.is_currently_playing_game
}

// 0x2ae54 — -[PlaceLauncher applicationDidReceiveMemoryWarning]
// demangled: -[PlaceLauncher applicationDidReceiveMemoryWarning]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher applicationDidReceiveMemoryWarning]")]
pub fn stub_2ae54(state: &PlaceLauncherState, child_connected: bool, player_connected: bool, warnings_enabled: bool) -> MemoryWarningOutcome {
    // IDA 0x2ae54: out of game logs and ignores (0x2afc2..0x2afe8); in game logs free memory (0x2aebe..0x2aed6), connected child/player selects OutOfMemory_EarlyExit + SessionReporter(5) vs OutOfMemory + SessionReporter(6) (0x2aeee..0x2b03c), closeChildConnections (0x2b056), warnings_preference gates the MemoryError alert (0x2b074..0x2b100), then leaveGame (0x2b142) — caller runs stub_298e0 next.
    if !state.is_currently_playing_game {
        return MemoryWarningOutcome::Ignored;
    }
    MemoryWarningOutcome::Shutdown { early_exit: child_connected || player_connected, alert: warnings_enabled }
}

// 0x2b1bc — -[PlaceLauncher childAdded:]
// demangled: -[PlaceLauncher childAdded:]
// type: void __cdecl(PlaceLauncher *self, SEL, shared_ptr<RBX::Instance>)
#[doc(alias = "-[PlaceLauncher childAdded:]")]
pub fn stub_2b1bc(has_view: bool, has_datamodel: bool, has_players: bool, has_player: bool, player_is_child: bool) -> ChildAddedRoute {
    // IDA 0x2b1bc: nil view/datamodel/players/player each log + closeChildConnections (0x2b248..0x2b3be); a live player connects playerLoaded: onto the child-added signal — same-slot rewire when the player is the added child, cross-slot otherwise (0x2b264..0x2b472, rbx::signals maps to rbx_core::signal) — returns the route taken; caller runs stub_2b5e0 on the close legs.
    if !has_view {
        ChildAddedRoute::NoView
    } else if !has_datamodel {
        ChildAddedRoute::NoDatamodel
    } else if !has_players {
        ChildAddedRoute::NoPlayers
    } else if !has_player {
        ChildAddedRoute::NoPlayer
    } else if player_is_child {
        ChildAddedRoute::PlayerIsChild
    } else {
        ChildAddedRoute::PlayerOther
    }
}

// 0x2b548 — -[PlaceLauncher playerLoaded:]
// demangled: -[PlaceLauncher playerLoaded:]
// type: void __cdecl(PlaceLauncher *self, SEL, shared_ptr<RBX::Instance>)
#[doc(alias = "-[PlaceLauncher playerLoaded:]")]
pub fn stub_2b548() -> &'static str {
    // IDA 0x2b548: disconnects playerConnection (0x2b56a), closeChildConnections (0x2b57c), stores RobloxGameState=inGame (0x2b59a..0x2b5da) — returns the stored state value.
    "inGame"
}

// 0x2b5e0 — -[PlaceLauncher closeChildConnections]
// demangled: -[PlaceLauncher closeChildConnections]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher closeChildConnections]")]
pub fn stub_2b5e0(child_connected: bool, player_connected: bool) -> (bool, bool) {
    // IDA 0x2b5e0: disconnects childConnection/playerConnection when connected (0x2b5f2..0x2b61e, rbx::signals maps to rbx_core::signal), then stops the free-memory checker (0x2b63a..0x2b64e) — returns which slots were disconnected.
    (child_connected, player_connected)
}

// 0x2b654 — -[PlaceLauncher .cxx_destruct]
// demangled: -[PlaceLauncher .cxx_destruct]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher .cxx_destruct]")]
pub fn stub_2b654(has_teleporter: bool, has_child_slot: bool, has_player_slot: bool) {
    // IDA 0x2b654: weak-releases the player/child connection slots when set (0x2b68e..0x2b6cc, boost::intrusive_ptr maps to rbx_core::SharedPtr drop) and deletes the teleporter (0x2b6de..0x2b6e6) — host drops carry the same lifetimes; faithful no-op shell.
    let _ = (has_teleporter, has_child_slot, has_player_slot);
}

// 0x2b724 — -[PlaceLauncher .cxx_construct]
// demangled: -[PlaceLauncher .cxx_construct]
// type: id __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher .cxx_construct]")]
pub fn stub_2b724() {
    // IDA 0x2b724: zeroes teleporter.px + child/player connection slots (0x2b73c..0x2b74e) — host PlaceLauncherState::default carries the same zeroed flags; faithful no-op shell.
}

// 0x2b754 — __ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE // was: boost::shared_ptr
// demangled: finishTeleportHelper(RobloxView *,boost::shared_ptr<RBX::Game>)
// type: 
#[doc(alias = "finishTeleportHelper(RobloxView *,rbx_core::SharedPtr<RBX::Game>)")]
pub fn stub_2b754(has_controller: bool, has_subview: bool) -> bool {
    // IDA 0x2b754: with a MainViewController sets the teleported game on the first ogre subview (0x2b7a4..0x2b878), then a 0.5s UIView animation running the 0x2b980 frame block with 0x2ba14 completion (0x2b8be..0x2b90e) — returns whether the game was attached.
    has_controller && has_subview
}

// 0x2b980 — ____ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE_block_invoke // was: boost
// demangled: ____ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE_block_invoke
// type: 
#[doc(alias = "____ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE_block_invoke")]
pub fn stub_2b980(has_screen: bool, x: i32, y: i32, w: i32, h: i32) -> (i32, i32, i32, i32) {
    // IDA 0x2b980: animation block re-sets the view frame to the main-screen bounds (0x2b9c8..0x2b9fe); nil screen zeroes the frame (0x2b9b8..0x2b9f8).
    if has_screen { (x, y, w, h) } else { (0, 0, 0, 0) }
}

// 0x2ba00 — ___copy_helper_block_425
// demangled: ___copy_helper_block_425
// type: 
#[doc(alias = "___copy_helper_block_425")]
pub fn stub_2ba00(dst: u32, src: u32) {
    // IDA 0x2ba00: __copy_helper_block_425 — single _Block_object_assign slot (0x2ba06); block retain has no host carrier.
    let _ = (dst, src);
}

// 0x2ba0c — ___destroy_helper_block_426
// demangled: ___destroy_helper_block_426
// type: 
#[doc(alias = "___destroy_helper_block_426")]
pub fn stub_2ba0c(handle: u32) {
    // IDA 0x2ba0c: __destroy_helper_block_426 — single _Block_object_dispose slot (0x2ba10); block release has no host carrier.
    let _ = handle;
}

// 0x2ba14 — ____ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE_block_invoke428 // was: boost
// demangled: ____ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE_block_invoke428
// type: 
#[doc(alias = "____ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE_block_invoke428")]
pub fn stub_2ba14() -> bool {
    // IDA 0x2ba14: completion clears clipsToBounds on the view (0x2ba26) — returns the resulting flag.
    false
}