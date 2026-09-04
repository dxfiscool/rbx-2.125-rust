//! reflection — generated_bg_6 — 150 stubs EA-sorted asc global gap filler 0x239ec..0x2d768 not yet in crates/reflection (global 85545 funcs, 64081 gaps reflection before; 21465->21615 distinct)
//! Source: ida/export.json (85545 funcs) global EA asc not in crates/reflection/src — next 150 uncovered for reflection-bg sorted asc after 0x21ce0
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

/// Gap-filler iOSSettingsService value state (IDA 0x239ec-0x24024).
/// Each `ReadValue*` callback parses its `char const*` into a service
/// member; the raw value records under its key and the parsed value
/// returns. `std::string`/`map` traffic is drop glue.
pub(crate) static IOS_SETTING_VALUES: std::sync::LazyLock<
    parking_lot::Mutex<std::collections::HashMap<String, String>>,
> = std::sync::LazyLock::new(|| parking_lot::Mutex::new(std::collections::HashMap::new()));
pub(crate) fn record_setting_value(key: &str, value: &str) {
    IOS_SETTING_VALUES.lock().insert(key.to_owned(), value.to_owned());
}
/// C `atoi` (IDA `atoi` behind the int readers): skips `isspace`,
/// takes an optional sign + digit run, wraps on overflow, yields 0
/// with no digits.
pub(crate) fn c_atoi(input: &str) -> i32 {
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() && matches!(bytes[i], b' ' | b'\t' | b'\n' | b'\x0b' | b'\x0c' | b'\r') {
        i += 1;
    }
    let mut negative = false;
    if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
        negative = bytes[i] == b'-';
        i += 1;
    }
    let mut acc: i32 = 0;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        acc = acc.wrapping_mul(10).wrapping_add((bytes[i] - b'0') as i32);
        i += 1;
    }
    if negative { acc.wrapping_neg() } else { acc }
}
/// `SimpleJSON::ParseBool` (IDA 0x255c8c): only `"true"`/`"True"`.
pub(crate) fn parse_bool_value(input: &str) -> bool {
    input == "true" || input == "True"
}
/// Gap-filler PlaceLauncher observable state (IDA 0x246d8-0x24a58).
/// The singleton handle, teleport-callback count, playing flag, last
/// place id and failure-forward count record here; the three
/// notification names are binary constants returned directly.
pub(crate) static PLACE_LAUNCHER_HANDLE: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);
pub(crate) static PLACE_TELEPORT_CALLBACKS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static PLACE_IS_PLAYING: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static PLACE_LAST_ID: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
pub(crate) static PLACE_FAILURE_FORWARDS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
/// Gap-filler PlaceLauncher game-lifecycle flags (IDA 0x24ab0-0x27268).
/// `prepareGame`/setup/start/join/load flow records here; UIKit views,
/// GCD queues, `boost::bind`/`thread` and `std::string` traffic is drop
/// glue. `SharedPtr<RBX::Game>` presence collapses to `bool`.
pub(crate) static PLACE_CURRENTLY_PLAYING: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static PLACE_ROBX_VIEW: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static PLACE_MEM_CHECKER: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static PLACE_MEM_WARNING: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static PLACE_IDLE_TIMER_DISABLED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static PLACE_UNSECURED_GAME: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static PLACE_LAST_NON_GAME: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static PLACE_DEFERRED_FINISH: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static PLACE_LOGIN_CONNECTED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static PLACE_CHILD_ADDED_CONNECTED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static PLACE_LOCAL_PLAYER_CREATED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static PLACE_CONTROL_FLAG: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static PLACE_PART_CHECK_QUEUED: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static PLACE_PART_WARNINGS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static PLACE_FINISHED_POSTS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static PLACE_DATAMODEL_CONNS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static PLACE_ANALYTICS_EVENTS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static PLACE_GAME_VC_CREATED: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static PLACE_CONTROL_TASKS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static PLACE_CONTROL_EXECS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static PLACE_PRESENT_QUEUED: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static PLACE_JOIN_THREADS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static PLACE_EXECUTED_SCRIPTS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
/// Last alert key (`ConnectionError`, `WiFiOnlyError`, `WarnTooManyParts`),
/// join/load scripts, join URL/local endpoint, app file and analytics label.
pub(crate) static PLACE_LAST_ALERT: std::sync::LazyLock<parking_lot::Mutex<String>> =
    std::sync::LazyLock::new(|| parking_lot::Mutex::new(String::new()));
pub(crate) static PLACE_LAST_JOIN_SCRIPT: std::sync::LazyLock<parking_lot::Mutex<String>> =
    std::sync::LazyLock::new(|| parking_lot::Mutex::new(String::new()));
pub(crate) static PLACE_LAST_JOIN_URL: std::sync::LazyLock<parking_lot::Mutex<String>> =
    std::sync::LazyLock::new(|| parking_lot::Mutex::new(String::new()));
pub(crate) static PLACE_LAST_APP_FILE: std::sync::LazyLock<parking_lot::Mutex<String>> =
    std::sync::LazyLock::new(|| parking_lot::Mutex::new(String::new()));
pub(crate) static PLACE_LAST_LOAD_SCRIPT: std::sync::LazyLock<parking_lot::Mutex<String>> =
    std::sync::LazyLock::new(|| parking_lot::Mutex::new(String::new()));
pub(crate) static PLACE_LAST_ANALYTICS_LABEL: std::sync::LazyLock<parking_lot::Mutex<String>> =
    std::sync::LazyLock::new(|| parking_lot::Mutex::new(String::new()));
pub(crate) static PLACE_LAST_JOIN_LOCAL: std::sync::LazyLock<parking_lot::Mutex<(i32, String)>> =
    std::sync::LazyLock::new(|| parking_lot::Mutex::new((0, String::new())));
/// Gap-filler PlaceLauncher join/leave/teleport state (IDA 0x276b0-0x2a99c).
/// Join-request queue, session/page-view reports, leave/shutdown posts and
/// the background-task + backgrounded-view flags record here.
pub(crate) static PLACE_SIGNED_SCRIPTS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static PLACE_SESSION_REPORTS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static PLACE_PAGE_VIEWS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static PLACE_LEAVE_POSTS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static PLACE_TELEPORT_ANIMS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static PLACE_START_SUCCESSES: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static PLACE_IS_LEAVING: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static PLACE_BG_TASK: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static PLACE_VIEW_BACKGROUNDED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static PLACE_LAST_SOLO_JOIN: std::sync::atomic::AtomicI32 =
    std::sync::atomic::AtomicI32::new(0);
pub(crate) static PLACE_LAST_JOIN_REQUEST: std::sync::LazyLock<parking_lot::Mutex<(i32, u32)>> =
    std::sync::LazyLock::new(|| parking_lot::Mutex::new((0, 0)));
pub(crate) static PLACE_LAST_TELEPORT: std::sync::LazyLock<
    parking_lot::Mutex<(String, String, String)>,
> = std::sync::LazyLock::new(|| {
    parking_lot::Mutex::new((String::new(), String::new(), String::new()))
});
/// Gap-filler PlaceLauncher connection/teleport tail state (IDA 0x2aba4-0x2bf74).
/// Child/player signal connections, the teleporter handle, the
/// `RobloxGameState` default and the subview game-set count record here.
pub(crate) static PLACE_PLAYER_CONNECTED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static PLACE_TELEPORTER: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static PLACE_SUBVIEW_GAME_SET: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static PLACE_GAME_STATE: std::sync::LazyLock<parking_lot::Mutex<String>> =
    std::sync::LazyLock::new(|| parking_lot::Mutex::new(String::new()));

// 0x239ec — __ZN18iOSSettingsService27ReadValueiPadMinimumVersionEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiPadMinimumVersion(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService27ReadValueiPadMinimumVersionEPKc")]
pub fn stub_0x239ec(value: &str) -> i32 {
    // IDA 0x239ec: `ReadValueiPadMinimumVersion` parses the value with
    // `atoi` into the service member. The raw value records under its
    // key; the parsed value returns.
    let parsed = c_atoi(value);
    record_setting_value("iPadMinimumVersion", value);
    parsed
}

// 0x23a04 — __ZNSt3mapISsPFvPKcESt4lessISsESaISt4pairIKSsS3_EEEixERS7_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<std::string,void (*)(char const*),std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::operator[](std::string const&)")]
#[doc(alias = "__ZNSt3mapISsPFvPKcESt4lessISsESaISt4pairIKSsS3_EEEixERS7_")]
pub fn stub_0x23a04(_map: usize, _key: &str) {
    // IDA 0x23a04: `std::map::operator[]` — red-black `lower_bound` +
    // unique insert of the key (disasm 0x23a04-0x23b4e). STL glue;
    // the service map itself records via `IOS_SETTINGS_KEYS`. No
    // explicit body.
}

// 0x23b50 — __ZN18iOSSettingsService27ReadValueiPadMaximumVersionEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiPadMaximumVersion(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService27ReadValueiPadMaximumVersionEPKc")]
pub fn stub_0x23b50(value: &str) -> i32 {
    // IDA 0x23b50: `ReadValueiPadMaximumVersion` parses the value with
    // `atoi` into the service member. Same shape as stub_0x239ec.
    let parsed = c_atoi(value);
    record_setting_value("iPadMaximumVersion", value);
    parsed
}

// 0x23b68 — __ZN18iOSSettingsService29ReadValueiPhoneMinimumVersionEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiPhoneMinimumVersion(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService29ReadValueiPhoneMinimumVersionEPKc")]
pub fn stub_0x23b68(value: &str) -> i32 {
    // IDA 0x23b68: `ReadValueiPhoneMinimumVersion` parses the value
    // with `atoi` into the service member. Same shape as stub_0x239ec.
    let parsed = c_atoi(value);
    record_setting_value("iPhoneMinimumVersion", value);
    parsed
}

// 0x23b80 — __ZN18iOSSettingsService29ReadValueiPhoneMaximumVersionEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiPhoneMaximumVersion(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService29ReadValueiPhoneMaximumVersionEPKc")]
pub fn stub_0x23b80(value: &str) -> i32 {
    // IDA 0x23b80: `ReadValueiPhoneMaximumVersion` parses the value
    // with `atoi` into the service member. Same shape as stub_0x239ec.
    let parsed = c_atoi(value);
    record_setting_value("iPhoneMaximumVersion", value);
    parsed
}

// 0x23b98 — __ZN18iOSSettingsService27ReadValueiPodMinimumVersionEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiPodMinimumVersion(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService27ReadValueiPodMinimumVersionEPKc")]
pub fn stub_0x23b98(value: &str) -> i32 {
    // IDA 0x23b98: `ReadValueiPodMinimumVersion` parses the value with
    // `atoi` into the service member. Same shape as stub_0x239ec.
    let parsed = c_atoi(value);
    record_setting_value("iPodMinimumVersion", value);
    parsed
}

// 0x23bb0 — __ZN18iOSSettingsService27ReadValueiPodMaximumVersionEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiPodMaximumVersion(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService27ReadValueiPodMaximumVersionEPKc")]
pub fn stub_0x23bb0(value: &str) -> i32 {
    // IDA 0x23bb0: `ReadValueiPodMaximumVersion` parses the value with
    // `atoi` into the service member. Same shape as stub_0x239ec.
    let parsed = c_atoi(value);
    record_setting_value("iPodMaximumVersion", value);
    parsed
}

// 0x23bc8 — __ZN18iOSSettingsService32ReadValueDisablePlayButtonForAllEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueDisablePlayButtonForAll(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService32ReadValueDisablePlayButtonForAllEPKc")]
pub fn stub_0x23bc8(value: &str) -> bool {
    // IDA 0x23bc8: `ReadValueDisablePlayButtonForAll` parses the value
    // with `SimpleJSON::ParseBool` (`"true"`/`"True"`, 0x255c8c) into
    // the service member. The raw value records under its key; the
    // parsed value returns.
    let parsed = parse_bool_value(value);
    record_setting_value("DisablePlayButtonForAll", value);
    parsed
}

// 0x23be4 — __ZN18iOSSettingsService34ReadValueDisablePlayButtonForNonBCEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueDisablePlayButtonForNonBC(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService34ReadValueDisablePlayButtonForNonBCEPKc")]
pub fn stub_0x23be4(value: &str) -> bool {
    // IDA 0x23be4: `ReadValueDisablePlayButtonForNonBC` parses the
    // value with `SimpleJSON::ParseBool` into the service member. Same
    // shape as stub_0x23bc8.
    let parsed = parse_bool_value(value);
    record_setting_value("DisablePlayButtonForNonBC", value);
    parsed
}

// 0x23c00 — __ZN18iOSSettingsService32ReadValueiPad1_MaximumIdealPartsEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiPad1_MaximumIdealParts(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService32ReadValueiPad1_MaximumIdealPartsEPKc")]
pub fn stub_0x23c00(value: &str) -> i32 {
    // IDA 0x23c00: `ReadValueiPad1_MaximumIdealParts` parses the value
    // with `atoi` into the service member. Same shape as stub_0x239ec.
    let parsed = c_atoi(value);
    record_setting_value("iPad1_MaximumIdealParts", value);
    parsed
}

// 0x23c18 — __ZN18iOSSettingsService32ReadValueiPad2_MaximumIdealPartsEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiPad2_MaximumIdealParts(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService32ReadValueiPad2_MaximumIdealPartsEPKc")]
pub fn stub_0x23c18(value: &str) -> i32 {
    // IDA 0x23c18: `ReadValueiPad2_MaximumIdealParts` parses the value
    // with `atoi` into the service member. Same shape as stub_0x239ec.
    let parsed = c_atoi(value);
    record_setting_value("iPad2_MaximumIdealParts", value);
    parsed
}

// 0x23c30 — __ZN18iOSSettingsService32ReadValueiPad3_MaximumIdealPartsEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiPad3_MaximumIdealParts(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService32ReadValueiPad3_MaximumIdealPartsEPKc")]
pub fn stub_0x23c30(value: &str) -> i32 {
    // IDA 0x23c30: `ReadValueiPad3_MaximumIdealParts` parses the value
    // with `atoi` into the service member. Same shape as stub_0x239ec.
    let parsed = c_atoi(value);
    record_setting_value("iPad3_MaximumIdealParts", value);
    parsed
}

// 0x23c48 — __ZN18iOSSettingsService32ReadValueiPad4_MaximumIdealPartsEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiPad4_MaximumIdealParts(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService32ReadValueiPad4_MaximumIdealPartsEPKc")]
pub fn stub_0x23c48(value: &str) -> i32 {
    // IDA 0x23c48: `ReadValueiPad4_MaximumIdealParts` parses the value
    // with `atoi` into the service member. Same shape as stub_0x239ec.
    let parsed = c_atoi(value);
    record_setting_value("iPad4_MaximumIdealParts", value);
    parsed
}

// 0x23c60 — __ZN18iOSSettingsService32ReadValueiPod4_MaximumIdealPartsEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiPod4_MaximumIdealParts(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService32ReadValueiPod4_MaximumIdealPartsEPKc")]
pub fn stub_0x23c60(value: &str) -> i32 {
    // IDA 0x23c60: `ReadValueiPod4_MaximumIdealParts` parses the value
    // with `atoi` into the service member. Same shape as stub_0x239ec.
    let parsed = c_atoi(value);
    record_setting_value("iPod4_MaximumIdealParts", value);
    parsed
}

// 0x23c78 — __ZN18iOSSettingsService32ReadValueiPod5_MaximumIdealPartsEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiPod5_MaximumIdealParts(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService32ReadValueiPod5_MaximumIdealPartsEPKc")]
pub fn stub_0x23c78(value: &str) -> i32 {
    // IDA 0x23c78: `ReadValueiPod5_MaximumIdealParts` parses the value
    // with `atoi` into the service member. Same shape as stub_0x239ec.
    let parsed = c_atoi(value);
    record_setting_value("iPod5_MaximumIdealParts", value);
    parsed
}

// 0x23c90 — __ZN18iOSSettingsService35ReadValueiPhone4s_MaximumIdealPartsEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiPhone4s_MaximumIdealParts(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService35ReadValueiPhone4s_MaximumIdealPartsEPKc")]
pub fn stub_0x23c90(value: &str) -> i32 {
    // IDA 0x23c90: `ReadValueiPhone4s_MaximumIdealParts` parses the
    // value with `atoi` into the service member. Same shape as
    // stub_0x239ec.
    let parsed = c_atoi(value);
    record_setting_value("iPhone4s_MaximumIdealParts", value);
    parsed
}

// 0x23ca8 — __ZN18iOSSettingsService34ReadValueiPhone5_MaximumIdealPartsEPKc
// type: int __fastcall(iOSSettingsService *this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiPhone5_MaximumIdealParts(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService34ReadValueiPhone5_MaximumIdealPartsEPKc")]
pub fn stub_0x23ca8(value: &str) -> i32 {
    // IDA 0x23ca8: `ReadValueiPhone5_MaximumIdealParts` parses the
    // value with `atoi` into the service member. Same shape as
    // stub_0x239ec.
    let parsed = c_atoi(value);
    record_setting_value("iPhone5_MaximumIdealParts", value);
    parsed
}

// 0x23cc0 — __ZN18iOSSettingsService50ReadValueTimeIntervalBetweenRobuxPurchaseInMinutesEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueTimeIntervalBetweenRobuxPurchaseInMinutes(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService50ReadValueTimeIntervalBetweenRobuxPurchaseInMinutesEPKc")]
pub fn stub_0x23cc0(value: &str) -> i32 {
    // IDA 0x23cc0: `ReadValueTimeIntervalBetweenRobuxPurchaseInMinutes`
    // parses the value with `atoi` into the service member. Same shape
    // as stub_0x239ec.
    let parsed = c_atoi(value);
    record_setting_value("TimeIntervalBetweenRobuxPurchaseInMinutes", value);
    parsed
}

// 0x23cd8 — __ZN18iOSSettingsService47ReadValueTimeIntervalBetweenBCPurchaseInMinutesEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueTimeIntervalBetweenBCPurchaseInMinutes(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService47ReadValueTimeIntervalBetweenBCPurchaseInMinutesEPKc")]
pub fn stub_0x23cd8(value: &str) -> i32 {
    // IDA 0x23cd8: `ReadValueTimeIntervalBetweenBCPurchaseInMinutes`
    // parses the value with `atoi` into the service member. Same shape
    // as stub_0x239ec.
    let parsed = c_atoi(value);
    record_setting_value("TimeIntervalBetweenBCPurchaseInMinutes", value);
    parsed
}

// 0x23cf0 — __ZN18iOSSettingsService52ReadValueTimeIntervalBetweenCatalogPurchaseInMinutesEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueTimeIntervalBetweenCatalogPurchaseInMinutes(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService52ReadValueTimeIntervalBetweenCatalogPurchaseInMinutesEPKc")]
pub fn stub_0x23cf0(value: &str) -> i32 {
    // IDA 0x23cf0:
    // `ReadValueTimeIntervalBetweenCatalogPurchaseInMinutes` parses the
    // value with `atoi` into the service member. Same shape as
    // stub_0x239ec.
    let parsed = c_atoi(value);
    record_setting_value("TimeIntervalBetweenCatalogPurchaseInMinutes", value);
    parsed
}

// 0x23d08 — __ZN18iOSSettingsService56ReadValueTimeLimitForBillingServiceRetriesBeforeGivingUpEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueTimeLimitForBillingServiceRetriesBeforeGivingUp(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService56ReadValueTimeLimitForBillingServiceRetriesBeforeGivingUpEPKc")]
pub fn stub_0x23d08(value: &str) -> i32 {
    // IDA 0x23d08:
    // `ReadValueTimeLimitForBillingServiceRetriesBeforeGivingUp`
    // parses the value with `atoi` into the service member. Same shape
    // as stub_0x239ec.
    let parsed = c_atoi(value);
    record_setting_value("TimeLimitForBillingServiceRetriesBeforeGivingUp", value);
    parsed
}

// 0x23d20 — __ZN18iOSSettingsService31ReadValueTestFlightLoggingLevelEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueTestFlightLoggingLevel(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService31ReadValueTestFlightLoggingLevelEPKc")]
pub fn stub_0x23d20(value: &str) -> i32 {
    // IDA 0x23d20: `ReadValueTestFlightLoggingLevel` parses the value
    // with `atoi` into the service member. Same shape as stub_0x239ec.
    let parsed = c_atoi(value);
    record_setting_value("TestFlightLoggingLevel", value);
    parsed
}

// 0x23d38 — __ZN18iOSSettingsService29ReadValueTestFlightPercentageEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueTestFlightPercentage(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService29ReadValueTestFlightPercentageEPKc")]
pub fn stub_0x23d38(value: &str) -> i32 {
    // IDA 0x23d38: `ReadValueTestFlightPercentage` parses the value
    // with `atoi` into the service member. Same shape as stub_0x239ec.
    let parsed = c_atoi(value);
    record_setting_value("TestFlightPercentage", value);
    parsed
}

// 0x23d50 — __ZN18iOSSettingsService27ReadValueBugSensePercentageEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueBugSensePercentage(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService27ReadValueBugSensePercentageEPKc")]
pub fn stub_0x23d50(value: &str) -> i32 {
    // IDA 0x23d50: `ReadValueBugSensePercentage` parses the value with
    // `atoi` into the service member. Same shape as stub_0x239ec.
    let parsed = c_atoi(value);
    record_setting_value("BugSensePercentage", value);
    parsed
}

// 0x23d68 — __ZN18iOSSettingsService25ReadValueBugSenseLogLinesEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueBugSenseLogLines(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService25ReadValueBugSenseLogLinesEPKc")]
pub fn stub_0x23d68(value: &str) -> i32 {
    // IDA 0x23d68: `ReadValueBugSenseLogLines` parses the value with
    // `atoi` into the service member. Same shape as stub_0x239ec.
    let parsed = c_atoi(value);
    record_setting_value("BugSenseLogLines", value);
    parsed
}

// 0x23d80 — __ZN18iOSSettingsService25ReadValueBugSenseLogLevelEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueBugSenseLogLevel(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService25ReadValueBugSenseLogLevelEPKc")]
pub fn stub_0x23d80(value: &str) -> i32 {
    // IDA 0x23d80: `ReadValueBugSenseLogLevel` parses the value with
    // `atoi` into the service member. Same shape as stub_0x239ec.
    let parsed = c_atoi(value);
    record_setting_value("BugSenseLogLevel", value);
    parsed
}

// 0x23d9c — __ZN18iOSSettingsService35ReadValueiOSGoogleAnalyticsAccount2EPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiOSGoogleAnalyticsAccount2(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService35ReadValueiOSGoogleAnalyticsAccount2EPKc")]
pub fn stub_0x23d9c(value: &str) {
    // IDA 0x23d9c: `ReadValueiOSGoogleAnalyticsAccount2` assigns the
    // value into the service member (`std::string` copy/assign
    // chain). Assignment is drop glue; the raw value records.
    record_setting_value("iOSGoogleAnalyticsAccount2", value);
}

// 0x23ed4 — __ZN18iOSSettingsService37ReadValueiOSGoogleAnalyticsSampleRateEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueiOSGoogleAnalyticsSampleRate(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService37ReadValueiOSGoogleAnalyticsSampleRateEPKc")]
pub fn stub_0x23ed4(value: &str) -> i32 {
    // IDA 0x23ed4: `ReadValueiOSGoogleAnalyticsSampleRate` parses the
    // value with `atoi` into the service member. Same shape as
    // stub_0x239ec.
    let parsed = c_atoi(value);
    record_setting_value("iOSGoogleAnalyticsSampleRate", value);
    parsed
}

// 0x23eec — __ZN18iOSSettingsService27ReadValueSearchEndpointIPadEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueSearchEndpointIPad(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService27ReadValueSearchEndpointIPadEPKc")]
pub fn stub_0x23eec(value: &str) {
    // IDA 0x23eec: `ReadValueSearchEndpointIPad` assigns the value into
    // the service member (`std::string` copy/assign chain). Assignment
    // is drop glue; the raw value records.
    record_setting_value("SearchEndpointIPad", value);
}

// 0x24024 — __ZN18iOSSettingsService29ReadValueSearchEndpointIPhoneEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueSearchEndpointIPhone(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService29ReadValueSearchEndpointIPhoneEPKc")]
pub fn stub_0x24024(value: &str) {
    // IDA 0x24024: `ReadValueSearchEndpointIPhone` assigns the value
    // into the service member (`std::string` copy/assign chain).
    // Assignment is drop glue; the raw value records.
    record_setting_value("SearchEndpointIPhone", value);
}

// 0x2415c — __ZN18iOSSettingsService24ReadValueCacheUIWebViewsEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueCacheUIWebViews(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService24ReadValueCacheUIWebViewsEPKc")]
pub fn stub_0x2415c(value: &str) -> bool {
    // IDA 0x2415c: `ReadValueCacheUIWebViews` parses the value with
    // `SimpleJSON::ParseBool` into the service member. Same shape as
    // stub_0x23bc8.
    let parsed = parse_bool_value(value);
    record_setting_value("CacheUIWebViews", value);
    parsed
}

// 0x24178 — __ZN18iOSSettingsService31ReadValueThumbstickControlStyleEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueThumbstickControlStyle(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService31ReadValueThumbstickControlStyleEPKc")]
pub fn stub_0x24178(value: &str) -> i32 {
    // IDA 0x24178: `ReadValueThumbstickControlStyle` parses the value
    // with `atoi` into the service member. Same shape as stub_0x239ec.
    let parsed = c_atoi(value);
    record_setting_value("ThumbstickControlStyle", value);
    parsed
}

// 0x24194 — __ZN18iOSSettingsService32ReadValueFreeMemoryCheckerActiveEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueFreeMemoryCheckerActive(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService32ReadValueFreeMemoryCheckerActiveEPKc")]
pub fn stub_0x24194(value: &str) -> bool {
    // IDA 0x24194: `ReadValueFreeMemoryCheckerActive` parses the value
    // with `SimpleJSON::ParseBool` into the service member. Same shape
    // as stub_0x23bc8.
    let parsed = parse_bool_value(value);
    record_setting_value("FreeMemoryCheckerActive", value);
    parsed
}

// 0x241b0 — __ZN18iOSSettingsService42ReadValueFreeMemoryCheckerRateMilliSecondsEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueFreeMemoryCheckerRateMilliSeconds(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService42ReadValueFreeMemoryCheckerRateMilliSecondsEPKc")]
pub fn stub_0x241b0(value: &str) -> i32 {
    // IDA 0x241b0: `ReadValueFreeMemoryCheckerRateMilliSeconds`
    // parses the value with `atoi` into the service member. Same shape
    // as stub_0x239ec.
    let parsed = c_atoi(value);
    record_setting_value("FreeMemoryCheckerRateMilliSeconds", value);
    parsed
}

// 0x241cc — __ZN18iOSSettingsService44ReadValueFreeMemoryCheckerThresholdKiloBytesEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueFreeMemoryCheckerThresholdKiloBytes(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService44ReadValueFreeMemoryCheckerThresholdKiloBytesEPKc")]
pub fn stub_0x241cc(value: &str) -> i32 {
    // IDA 0x241cc: `ReadValueFreeMemoryCheckerThresholdKiloBytes`
    // parses the value with `atoi` into the service member. Same shape
    // as stub_0x239ec.
    let parsed = c_atoi(value);
    record_setting_value("FreeMemoryCheckerThresholdKiloBytes", value);
    parsed
}

// 0x241e8 — __ZN18iOSSettingsService28ReadValueMemoryBouncerActiveEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueMemoryBouncerActive(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService28ReadValueMemoryBouncerActiveEPKc")]
pub fn stub_0x241e8(value: &str) -> bool {
    // IDA 0x241e8: `ReadValueMemoryBouncerActive` parses the value with
    // `SimpleJSON::ParseBool` into the service member. Same shape as
    // stub_0x23bc8.
    let parsed = parse_bool_value(value);
    record_setting_value("MemoryBouncerActive", value);
    parsed
}

// 0x24204 — __ZN18iOSSettingsService45ReadValueMemoryBouncerEnforceRateMilliSecondsEPKc
// type: int __fastcall(iOSSettingsService *this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueMemoryBouncerEnforceRateMilliSeconds(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService45ReadValueMemoryBouncerEnforceRateMilliSecondsEPKc")]
pub fn stub_0x24204(value: &str) -> i32 {
    // IDA 0x24204: `ReadValueMemoryBouncerEnforceRateMilliSeconds`
    // parses the value with `atoi` into the service member. Same shape
    // as stub_0x239ec.
    let parsed = c_atoi(value);
    record_setting_value("MemoryBouncerEnforceRateMilliSeconds", value);
    parsed
}

// 0x24220 — __ZN18iOSSettingsService40ReadValueMemoryBouncerThresholdKiloBytesEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueMemoryBouncerThresholdKiloBytes(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService40ReadValueMemoryBouncerThresholdKiloBytesEPKc")]
pub fn stub_0x24220(value: &str) -> i32 {
    // IDA 0x24220: `ReadValueMemoryBouncerThresholdKiloBytes` parses
    // the value with `atoi` into the service member. Same shape as
    // stub_0x239ec.
    let parsed = c_atoi(value);
    record_setting_value("MemoryBouncerThresholdKiloBytes", value);
    parsed
}

// 0x2423c — __ZN18iOSSettingsService36ReadValueMemoryBouncerLimitMegaBytesEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueMemoryBouncerLimitMegaBytes(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService36ReadValueMemoryBouncerLimitMegaBytesEPKc")]
pub fn stub_0x2423c(value: &str) -> i32 {
    // IDA 0x2423c: `ReadValueMemoryBouncerLimitMegaBytes` parses the
    // value with `atoi` into the service member. Same shape as
    // stub_0x239ec.
    let parsed = c_atoi(value);
    record_setting_value("MemoryBouncerLimitMegaBytes", value);
    parsed
}

// 0x24258 — __ZN18iOSSettingsService52ReadValueMemoryBouncerLimitMegaBytesForLowMemDevicesEPKc
// type: _DWORD __fastcall(iOSSettingsService *__hidden this, const char *)
#[doc(alias = "iOSSettingsService::ReadValueMemoryBouncerLimitMegaBytesForLowMemDevices(char const*)")]
#[doc(alias = "__ZN18iOSSettingsService52ReadValueMemoryBouncerLimitMegaBytesForLowMemDevicesEPKc")]
pub fn stub_0x24258(value: &str) -> i32 {
    // IDA 0x24258:
    // `ReadValueMemoryBouncerLimitMegaBytesForLowMemDevices` parses the
    // value with `atoi` into the service member. Same shape as
    // stub_0x239ec.
    let parsed = c_atoi(value);
    record_setting_value("MemoryBouncerLimitMegaBytesForLowMemDevices", value);
    parsed
}

// 0x24274 — __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,void (*)(char const*)>>,std::pair<std::string const,void (*)(char const*)> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_")]
pub fn stub_0x24274() {
    // IDA 0x24274: `std::_Rb_tree::_M_insert_unique` (hinted) — tree
    // node insertion for the settings map. STL glue; the map itself
    // records via `IOS_SETTINGS_KEYS`. No explicit body.
}

// 0x24360 — __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE9_M_insertEPSt18_Rb_tree_node_baseSE_RKS6_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,void (*)(char const*)> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE9_M_insertEPSt18_Rb_tree_node_baseSE_RKS6_")]
pub fn stub_0x24360() {
    // IDA 0x24360: `std::_Rb_tree::_M_insert` — tree node splice for
    // the settings map. STL glue; no explicit body.
}

// 0x243b0 — __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE16_M_insert_uniqueERKS6_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_insert_unique(std::pair<std::string const,void (*)(char const*)> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE16_M_insert_uniqueERKS6_")]
pub fn stub_0x243b0() {
    // IDA 0x243b0: `std::_Rb_tree::_M_insert_unique` (unhinted) — tree
    // node insertion for the settings map. STL glue; no explicit body.
}

// 0x24434 — __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE14_M_create_nodeERKS6_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_create_node(std::pair<std::string const,void (*)(char const*)> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE14_M_create_nodeERKS6_")]
pub fn stub_0x24434() {
    // IDA 0x24434: `std::_Rb_tree::_M_create_node` — node allocation
    // for the settings map. STL glue; no explicit body.
}

// 0x24510 — __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE11lower_boundERS1_
// type: int __fastcall(int, std::string *)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::lower_bound(std::string const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE11lower_boundERS1_")]
pub fn stub_0x24510() {
    // IDA 0x24510: `std::_Rb_tree::lower_bound` — key search for the
    // settings map. STL glue; no explicit body.
}

// 0x24540 — __GLOBAL__I_a_7
#[doc(alias = "global constructor keyed to_a_7")]
#[doc(alias = "__GLOBAL__I_a_7")]
pub fn stub_0x24540() {
    // IDA 0x24540: `__GLOBAL__I_a_7` — boost category singletons,
    // `std::ios_base::Init` and `exception_ptr`/pool guards (same
    // cutover as stub_0x1d870; decompile unavailable, init thunk). No
    // body.
}

// 0x246d8 — -[PlaceLauncher init]
// type: PlaceLauncher *__cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher init]")]
pub fn stub_0x246d8() {
    // IDA 0x246d8: `init` supers (0x2473c-0x24746, no target here),
    // zeroes the view/warning/playing/place fields (0x24748-0x24762),
    // installs a `Teleporter` (0x24764-0x247b0,
    // `TeleportService::SetCallback`, counted) and publishes the three
    // `RBX*Notification` names (0x247b2-0x2482c). Allocation + service
    // wiring have no target here; the callback + field reset record.
    PLACE_IS_PLAYING.store(false, std::sync::atomic::Ordering::SeqCst);
    PLACE_LAST_ID.store(0, std::sync::atomic::Ordering::SeqCst);
    PLACE_TELEPORT_CALLBACKS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x248dc — -[PlaceLauncher dealloc]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher dealloc]")]
pub fn stub_0x248dc() {
    // IDA 0x248dc: `dealloc` clears the teleport callback, drops the
    // teleporter, releases the three notification names
    // (0x248dc-0x2494e), then super dealloc (0x24950-0x2495a). Release
    // is drop glue; the callback + playing cells reset.
    PLACE_TELEPORT_CALLBACKS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    PLACE_IS_PLAYING.store(false, std::sync::atomic::Ordering::SeqCst);
    PLACE_LAUNCHER_HANDLE.store(0, std::sync::atomic::Ordering::SeqCst);
}

// 0x24974 — +[PlaceLauncher sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[PlaceLauncher sharedInstance]")]
pub fn stub_0x24974() -> usize {
    // IDA 0x24974: `sharedInstance` runs the alloc/init block once
    // (0x24974-0x249cc, stub_0x249d0) and returns the singleton
    // (0x249cc). `dispatch_once` collapses to get-or-init on the
    // handle cell.
    let mut handle = PLACE_LAUNCHER_HANDLE.load(std::sync::atomic::Ordering::SeqCst);
    if handle == 0 {
        handle = stub_0x249d0();
        PLACE_LAUNCHER_HANDLE.store(handle, std::sync::atomic::Ordering::SeqCst);
    }
    handle
}

// 0x249d0 — ___31+[PlaceLauncher sharedInstance]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___31+[PlaceLauncher sharedInstance]_block_invoke")]
pub fn stub_0x249d0() -> usize {
    // IDA 0x249d0: the once block allocs + inits the launcher
    // (0x249d0-0x249fc) and publishes it (0x249f8). Allocation is drop
    // glue; init runs for its field/callback observables and the cell
    // takes a nonzero handle.
    stub_0x246d8();
    1
}

// 0x24a04 — ___copy_helper_block__4
#[doc(alias = "___copy_helper_block__4")]
pub fn stub_0x24a04(_dst: usize, _src: usize) {
    // IDA 0x24a04: `__copy_helper_block__4` — one `_Block_object_assign`
    // retain (same shape as stub_0x18094). No explicit body.
}

// 0x24a10 — ___destroy_helper_block__4
#[doc(alias = "___destroy_helper_block__4")]
pub fn stub_0x24a10(_block: usize) {
    // IDA 0x24a10: `__destroy_helper_block__4` — one
    // `_Block_object_dispose` release (same shape as stub_0x180a0). No
    // explicit body.
}

// 0x24a18 — -[PlaceLauncher getIsCurrentlyPlayingGame]
// type: char __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher getIsCurrentlyPlayingGame]")]
pub fn stub_0x24a18() -> bool {
    // IDA 0x24a18: `getIsCurrentlyPlayingGame` returns the flag
    // (0x24a18-0x24a24).
    PLACE_IS_PLAYING.load(std::sync::atomic::Ordering::SeqCst)
}

// 0x24a28 — -[PlaceLauncher getDidLeaveGameNotification]
// type: id __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher getDidLeaveGameNotification]")]
pub fn stub_0x24a28() -> &'static str {
    // IDA 0x24a28: `getDidLeaveGameNotification` returns the
    // `RBXDidLeaveGameNotification` name (0x24a28-0x24a34, constant).
    "RBXDidLeaveGameNotification"
}

// 0x24a38 — -[PlaceLauncher getStartLeaveGameNotification]
// type: id __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher getStartLeaveGameNotification]")]
pub fn stub_0x24a38() -> &'static str {
    // IDA 0x24a38: `getStartLeaveGameNotification` returns the
    // `RBXStartLeaveGameNotification` name (0x24a38-0x24a44, constant).
    "RBXStartLeaveGameNotification"
}

// 0x24a48 — -[PlaceLauncher getGameFinishedLoadingNotification]
// type: id __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher getGameFinishedLoadingNotification]")]
pub fn stub_0x24a48() -> &'static str {
    // IDA 0x24a48: `getGameFinishedLoadingNotification` returns the
    // `RBXGameFinishedLoadingNotification` name (0x24a48-0x24a58,
    // constant).
    "RBXGameFinishedLoadingNotification"
}

// 0x24a58 — -[PlaceLauncher handleStartGameFailure]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher handleStartGameFailure]")]
pub fn stub_0x24a58(has_fallback_controller: bool) {
    // IDA 0x24a58: `handleStartGameFailure` forwards to the last
    // non-game controller when present (0x24a58-0x24a84, counted) and
    // clears the playing flag (0x24a86-0x24a8e). The controller query
    // collapses into a parameter.
    if has_fallback_controller {
        PLACE_FAILURE_FORWARDS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    PLACE_IS_PLAYING.store(false, std::sync::atomic::Ordering::SeqCst);
}

// 0x24ab0 — -[PlaceLauncher prepareGame]
// type: bool __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher prepareGame]")]
pub fn stub_0x24ab0(reachability: u32, wifi_only: bool) -> bool {
    // IDA 0x24ab0: `prepareGame` sets the `<resourcePath>/content` asset
    // folder, `Game::globalInit`, the teleport base URL, then gates on
    // reachability: none → `ConnectionError` alert, false; cellular (2)
    // with the wifionly preference → `WiFiOnlyError` alert, false.
    // Otherwise `DataModel::hash` becomes `"ios,ios"`, settings load, the
    // scheduler thread count applies, and it returns true. UIKit and
    // `std::string` traffic is drop glue.
    if reachability == 0 {
        *PLACE_LAST_ALERT.lock() = "ConnectionError".to_owned();
        return false;
    }
    if reachability == 2 && wifi_only {
        *PLACE_LAST_ALERT.lock() = "WiFiOnlyError".to_owned();
        return false;
    }
    true
}

// 0x25080 — -[PlaceLauncher setLastPlaceId:]
// type: void __cdecl(PlaceLauncher *self, SEL, int)
#[doc(alias = "-[PlaceLauncher setLastPlaceId:]")]
pub fn stub_0x25080(place_id: i32) {
    // IDA 0x25080: `setLastPlaceId:` stores `self->lastPlaceId`.
    PLACE_LAST_ID.store(place_id, std::sync::atomic::Ordering::SeqCst);
}

// 0x25090 — -[PlaceLauncher checkPlacePartCount]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher checkPlacePartCount]")]
pub fn stub_0x25090(warnings_enabled: bool) {
    // IDA 0x25090: `checkPlacePartCount` reads the warnings preference;
    // when set it `dispatch_async`s the 0x2512c block on a global queue.
    // The GCD hop is drop glue; the block body lives in stub_0x2512c.
    if warnings_enabled {
        PLACE_PART_CHECK_QUEUED.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x2512c — ___36-[PlaceLauncher checkPlacePartCount]_block_invoke
#[doc(alias = "___36-[PlaceLauncher checkPlacePartCount]_block_invoke")]
pub fn stub_0x2512c(max_parts: i32, part_count: i32, place_id: i32) {
    // IDA 0x2512c: block reads the max-parts setting; with a live game,
    // workspace and part count above the max it shows the
    // `WarnPlaceIsNotIdeal`/`WarnTooManyParts` text and files a
    // `PlayErrors`/`TooManyParts` analytics event tagged with the place id.
    if max_parts < 1 || part_count <= max_parts {
        return;
    }
    PLACE_PART_WARNINGS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    PLACE_ANALYTICS_EVENTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    *PLACE_LAST_ALERT.lock() = "WarnTooManyParts".to_owned();
    *PLACE_LAST_ANALYTICS_LABEL.lock() = format!("TooManyParts:{place_id}");
}

// 0x253cc — ___copy_helper_block_98
#[doc(alias = "___copy_helper_block_98")]
pub fn stub_0x253cc() {
    // IDA 0x253cc: `__copy_helper_block_98` retains the captured object at
    // +20 (`_Block_object_assign`, Block_byref). `Arc` clone glue covers
    // it; no explicit body.
}

// 0x253d8 — ___destroy_helper_block_99
#[doc(alias = "___destroy_helper_block_99")]
pub fn stub_0x253d8() {
    // IDA 0x253d8: `__destroy_helper_block_99` releases the captured object
    // at +20 (`_Block_object_dispose`). `Arc` drop glue covers it; no
    // explicit body.
}

// 0x253e0 — -[PlaceLauncher placeDidFinishLoading]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher placeDidFinishLoading]")]
pub fn stub_0x253e0(warnings_enabled: bool) {
    // IDA 0x253e0: `placeDidFinishLoading` posts the
    // `gameFinishedLoadingNotification` then runs `checkPlacePartCount`
    // (0x25090).
    PLACE_FINISHED_POSTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    stub_0x25090(warnings_enabled);
}

// 0x25440 — -[PlaceLauncher deleteRobloxView]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher deleteRobloxView]")]
pub fn stub_0x25440() {
    // IDA 0x25440: `deleteRobloxView` destroys the `RobloxView`, clears the
    // ivar and stops the free-memory checker when a view is attached.
    if PLACE_ROBX_VIEW.swap(false, std::sync::atomic::Ordering::SeqCst) {
        PLACE_MEM_CHECKER.store(false, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x25498 — -[PlaceLauncher finishGameSetup:gameViewController:]
// type: void __cdecl(PlaceLauncher *self, SEL, shared_ptr<RBX::Game>, id)
#[doc(alias = "-[PlaceLauncher finishGameSetup:gameViewController:]")]
pub fn stub_0x25498(datamodel_ready: bool, has_overlay: bool, warnings_enabled: bool) {
    // IDA 0x25498: `finishGameSetup:gameViewController:` builds the
    // `RobloxView` from the game + screen bounds (stringstream ids are
    // drop glue). A ready datamodel fires `placeDidFinishLoading` directly
    // (0x253e0), otherwise a deferred `placeDidFinishLoading` slot connects;
    // then `setupDatamodelConnections:` runs for the datamodel (0x25e00)
    // and again for the overlay game when present.
    PLACE_ROBX_VIEW.store(true, std::sync::atomic::Ordering::SeqCst);
    if datamodel_ready {
        stub_0x253e0(warnings_enabled);
    } else {
        PLACE_DEFERRED_FINISH.store(true, std::sync::atomic::Ordering::SeqCst);
    }
    stub_0x25e00(true, true);
    if has_overlay {
        stub_0x25e00(true, true);
    }
}

// 0x25e00 — -[PlaceLauncher setupDatamodelConnections:]
// type: void __cdecl(PlaceLauncher *self, SEL, shared_ptr<RBX::DataModel>)
#[doc(alias = "-[PlaceLauncher setupDatamodelConnections:]")]
pub fn stub_0x25e00(has_gui_service: bool, has_login_service: bool) {
    // IDA 0x25e00: `setupDatamodelConnections:` wires `GuiService`'s
    // `openUrlWindow:` to the ogre controller, `Players` `childAdded:` to
    // `childAdded:`, and `LoginService`'s prompt-login signal; a main-queue
    // block also dispatches (drop glue, cf. 0x2613c). `boost::bind` slots
    // collapse into flags.
    PLACE_DATAMODEL_CONNS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let _ = has_gui_service;
    PLACE_CHILD_ADDED_CONNECTED.store(true, std::sync::atomic::Ordering::SeqCst);
    if has_login_service {
        PLACE_LOGIN_CONNECTED.store(true, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x2613c — ___43-[PlaceLauncher setupDatamodelConnections:]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___43-[PlaceLauncher setupDatamodelConnections:]_block_invoke")]
pub fn stub_0x2613c() {
    // IDA 0x2613c: block starts the `RobloxMemoryManager` free-memory
    // checker on the main queue.
    PLACE_MEM_CHECKER.store(true, std::sync::atomic::Ordering::SeqCst);
}

// 0x26170 — -[PlaceLauncher setLastNonGameController:]
// type: void __cdecl(PlaceLauncher *self, SEL, id)
#[doc(alias = "-[PlaceLauncher setLastNonGameController:]")]
pub fn stub_0x26170(has_controller: bool, game_ready: bool) {
    // IDA 0x26170: `setLastNonGameController:` forwards to
    // `MainViewController`; with a controller attached a failed
    // `prepareGame` falls into `handleStartGameFailure` (same shape as
    // 0x24a58).
    PLACE_LAST_NON_GAME.store(has_controller, std::sync::atomic::Ordering::SeqCst);
    if has_controller && !game_ready {
        PLACE_FAILURE_FORWARDS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        PLACE_IS_PLAYING.store(false, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x261d8 — -[PlaceLauncher createGame:presentGameAutomatically:]
// type: void __cdecl(PlaceLauncher *self, SEL, shared_ptr<RBX::Game>, char)
#[doc(alias = "-[PlaceLauncher createGame:presentGameAutomatically:]")]
pub fn stub_0x261d8(
    has_last_non_game: bool,
    datamodel_ready: bool,
    has_overlay: bool,
    warnings_enabled: bool,
    present_automatically: bool,
) {
    // IDA 0x261d8: `createGame:presentGameAutomatically:` clears the memory
    // warning, drops the old view (0x25440), and with a last-non-game
    // controller allocates the `GameViewController`, finishes setup
    // (0x25498) and submits `initControlView` (0x2643c) as a datamodel task.
    PLACE_MEM_WARNING.store(false, std::sync::atomic::Ordering::SeqCst);
    stub_0x25440();
    if has_last_non_game {
        PLACE_GAME_VC_CREATED.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        stub_0x25498(datamodel_ready, has_overlay, warnings_enabled);
        PLACE_CONTROL_TASKS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    let _ = present_automatically;
}

// 0x2643c — __ZL15initControlViewP10RobloxViewaPN3RBX18FunctionMarshallerE
// type: _DWORD __fastcall(RobloxView *, signed __int8, RBX::FunctionMarshaller *)
#[doc(alias = "initControlView(RobloxView *,signed char,RBX::FunctionMarshaller *)")]
#[doc(alias = "__ZL15initControlViewP10RobloxViewaPN3RBX18FunctionMarshallerE")]
pub fn stub_0x2643c(flag: bool) {
    // IDA 0x2643c: `initControlView` wraps `initControlViewHelper` in a
    // `function0` and runs it through `FunctionMarshaller::Execute`.
    // The marshaller hop is drop glue; the executed flag records.
    PLACE_CONTROL_FLAG.store(flag, std::sync::atomic::Ordering::SeqCst);
    PLACE_CONTROL_EXECS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x26520 — -[PlaceLauncher setupGame:isApp:]
// type: shared_ptr<RBX::Game> *__cdecl(shared_ptr<RBX::Game> *__return_ptr __struct_ptr retstr, PlaceLauncher *self, SEL, id, char)
#[doc(alias = "-[PlaceLauncher setupGame:isApp:]")]
pub fn stub_0x26520(has_self: bool, is_app: bool) -> bool {
    // IDA 0x26520: `setupGame:isApp:` forwards to
    // `setupGame:unsecuredGame:isApp:` (0x26558) with `unsecuredGame` 0;
    // a null launcher yields a null game.
    if !has_self {
        return false;
    }
    stub_0x26558(true, false, is_app)
}

// 0x26558 — -[PlaceLauncher setupGame:unsecuredGame:isApp:]
// type: shared_ptr<RBX::Game> *__cdecl(shared_ptr<RBX::Game> *__return_ptr __struct_ptr retstr, PlaceLauncher *self, SEL, id, char, char)
#[doc(alias = "-[PlaceLauncher setupGame:unsecuredGame:isApp:]")]
pub fn stub_0x26558(has_self: bool, unsecured: bool, is_app: bool) -> bool {
    // IDA 0x26558: `setupGame:unsecuredGame:isApp:` bails with a null game
    // while one plays; otherwise it fetches client/iOS settings, disables
    // the idle timer, records the non-game controller (cf. 0x26170) and
    // builds a `SecurePlayerGame` or `UnsecuredStudioGame` off the base URL.
    if !has_self {
        return false;
    }
    if PLACE_CURRENTLY_PLAYING.swap(true, std::sync::atomic::Ordering::SeqCst) {
        return false;
    }
    PLACE_IDLE_TIMER_DISABLED.store(true, std::sync::atomic::Ordering::SeqCst);
    PLACE_LAST_NON_GAME.store(true, std::sync::atomic::Ordering::SeqCst);
    PLACE_UNSECURED_GAME.store(unsecured, std::sync::atomic::Ordering::SeqCst);
    let _ = is_app;
    true
}

// 0x26768 — -[PlaceLauncher presentGameViewController]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher presentGameViewController]")]
pub fn stub_0x26768() {
    // IDA 0x26768: `presentGameViewController` hops to the main queue
    // (`__block_literal_global505`, drop glue) to present the game view.
    PLACE_PRESENT_QUEUED.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x26784 — -[PlaceLauncher setupPreloadedGameWithNonGameController:unsecuredGame:isApp:]
// type: shared_ptr<RBX::Game> *__cdecl(shared_ptr<RBX::Game> *__return_ptr __struct_ptr retstr, PlaceLauncher *self, SEL, id, char, char)
#[doc(alias = "-[PlaceLauncher setupPreloadedGameWithNonGameController:unsecuredGame:isApp:]")]
pub fn stub_0x26784(has_self: bool, unsecured: bool, is_app: bool) -> bool {
    // IDA 0x26784: `setupPreloadedGameWithNonGameController:unsecuredGame:`
    // `isApp:` forwards to `setupGame:unsecuredGame:isApp:` (0x26558).
    if !has_self {
        return false;
    }
    stub_0x26558(true, unsecured, is_app)
}

// 0x267bc — -[PlaceLauncher setupPreloadedGameWithNonGameController:isApp:]
// type: shared_ptr<RBX::Game> *__cdecl(shared_ptr<RBX::Game> *__return_ptr __struct_ptr retstr, PlaceLauncher *self, SEL, id, char)
#[doc(alias = "-[PlaceLauncher setupPreloadedGameWithNonGameController:isApp:]")]
pub fn stub_0x267bc(has_self: bool, is_app: bool) -> bool {
    // IDA 0x267bc: `setupPreloadedGameWithNonGameController:isApp:`
    // forwards to `setupGame:isApp:` (0x26520).
    if !has_self {
        return false;
    }
    stub_0x26520(true, is_app)
}

// 0x267ec — -[PlaceLauncher injectJoinScript:]
// type: void __cdecl(PlaceLauncher *self, SEL, id)
#[doc(alias = "-[PlaceLauncher injectJoinScript:]")]
pub fn stub_0x267ec(script: &str) {
    // IDA 0x267ec: `injectJoinScript:` binds `joinGameWithJoinScript`
    // (0x26990) over the script UTF-8 + game and runs it on a detached
    // `boost::thread` named `InjectStartScript`. The bind/thread hop is
    // drop glue; the script and spawn record.
    *PLACE_LAST_JOIN_SCRIPT.lock() = script.to_owned();
    PLACE_JOIN_THREADS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x26990 — __ZL22joinGameWithJoinScriptRKSsN5boost10shared_ptrIN3RBX4GameEEE
#[doc(alias = "joinGameWithJoinScript(std::string const&,rbx_core::SharedPtr<RBX::Game>)")]
#[doc(alias = "__ZL22joinGameWithJoinScriptRKSsN5boost10shared_ptrIN3RBX4GameEEE")]
pub fn stub_0x26990(script: &str) {
    // IDA 0x26990: `joinGameWithJoinScript` copies the script string and
    // runs `executeUrlScript` on the game (`std::string`/`SharedPtr`
    // traffic is drop glue).
    *PLACE_LAST_JOIN_SCRIPT.lock() = script.to_owned();
    PLACE_EXECUTED_SCRIPTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x26bb8 — -[PlaceLauncher startGameLocal:ipAddress:controller:presentGameAutomatically:]
// type: char __cdecl(PlaceLauncher *self, SEL, int, id, id, char)
#[doc(alias = "-[PlaceLauncher startGameLocal:ipAddress:controller:presentGameAutomatically:]")]
pub fn stub_0x26bb8(port: i32, ip: &str, present_automatically: bool) -> bool {
    // IDA 0x26bb8: `startGameLocal:...` sets up the preloaded unsecured
    // game (0x26784), binds `joinLocalGame` (0x26dd4) over port + ip, and
    // starts it via `startGame:controller:preloadedGame:` (0x29490).
    if !stub_0x26784(true, true, false) {
        return false;
    }
    *PLACE_LAST_JOIN_LOCAL.lock() = (port, ip.to_owned());
    stub_0x29490(present_automatically)
}

// 0x26dd4 — __ZL13joinLocalGameiRKSsN5boost10shared_ptrIN3RBX4GameEEE
#[doc(alias = "joinLocalGame(int,std::string const&,rbx_core::SharedPtr<RBX::Game>)")]
#[doc(alias = "__ZL13joinLocalGameiRKSsN5boost10shared_ptrIN3RBX4GameEEE")]
pub fn stub_0x26dd4(port: i32, server: &str) {
    // IDA 0x26dd4: `joinLocalGame` formats
    // `"%sGame/Join.ashx?userID=0&serverPort=%i&server=%s"` off the
    // `RobloxInfo` base URL and runs `executeUrlScript` on the game.
    // The base URL prefixes on device; the formatted tail records.
    *PLACE_LAST_JOIN_URL.lock() =
        format!("Game/Join.ashx?userID=0&serverPort={port}&server={server}");
    PLACE_EXECUTED_SCRIPTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x27054 — -[PlaceLauncher startAppWithFile:controller:presentGameAutomatically:]
// type: char __cdecl(PlaceLauncher *self, SEL, id, id, char)
#[doc(alias = "-[PlaceLauncher startAppWithFile:controller:presentGameAutomatically:]")]
pub fn stub_0x27054(file: &str, present_automatically: bool) -> bool {
    // IDA 0x27054: `startAppWithFile:...` sets up the preloaded unsecured
    // game (0x26784), binds `loadLocalApp` (0x27268) over the file path,
    // and starts it via `startGame:controller:preloadedGame:` (0x29490).
    if !stub_0x26784(true, true, false) {
        return false;
    }
    *PLACE_LAST_APP_FILE.lock() = file.to_owned();
    stub_0x29490(present_automatically)
}

// 0x27268 — __ZL12loadLocalAppRKSsN5boost10shared_ptrIN3RBX4GameEEE
#[doc(alias = "loadLocalApp(std::string const&,rbx_core::SharedPtr<RBX::Game>)")]
#[doc(alias = "__ZL12loadLocalAppRKSsN5boost10shared_ptrIN3RBX4GameEEE")]
pub fn stub_0x27268(file: &str) {
    // IDA 0x27268: `loadLocalApp` formats `Game:Load('rbxasset://%s')`,
    // executes it on the game datamodel, then creates the `Players`
    // service and the local player from the current user id.
    *PLACE_LAST_LOAD_SCRIPT.lock() = format!("Game:Load('rbxasset://{file}')");
    PLACE_EXECUTED_SCRIPTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    PLACE_LOCAL_PLAYER_CREATED.store(true, std::sync::atomic::Ordering::SeqCst);
}

// 0x276b0 — -[PlaceLauncher startAppWithId:controller:presentGameAutomatically:]
// type: char __cdecl(PlaceLauncher *self, SEL, int, id, char)
#[doc(alias = "-[PlaceLauncher startAppWithId:controller:presentGameAutomatically:]")]
pub fn stub_0x276b0(place_id: i32, present_automatically: bool) -> bool {
    // IDA 0x276b0: `startAppWithId:...` sets up the preloaded app game
    // (0x267bc, `isApp` 1), binds `joinGamePlaceId` (0x278a8) over the
    // place id + request 2, and starts it via
    // `startGame:controller:preloadedGame:` (0x29490).
    if !stub_0x267bc(true, true) {
        return false;
    }
    *PLACE_LAST_JOIN_REQUEST.lock() = (place_id, 2);
    stub_0x29490(present_automatically)
}

// 0x278a8 — __ZL15joinGamePlaceIdiN5boost10shared_ptrIN3RBX4GameEEE15JoinGameRequest
#[doc(alias = "joinGamePlaceId(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest)")]
#[doc(alias = "__ZL15joinGamePlaceIdiN5boost10shared_ptrIN3RBX4GameEEE15JoinGameRequest")]
pub fn stub_0x278a8(place_id: i32, request: u32, join_response: &str) {
    // IDA 0x278a8: `joinGamePlaceId` polls `Game/PlaceLauncher.ashx`
    // (`request=RequestGame&placeId=` / `RequestFollowUser&userId=`, or the
    // `Game/AppStart.ashx` overlay path for request 2) until `"status":2`
    // (retry on 0/1, `usleep` backoff — poll loop is drop glue, the final
    // response drives the outcome). Request 2 runs `executeSignedScript`;
    // otherwise the `joinScriptUrl` value unescapes (`\/` → `/`) and runs
    // `executeUrlScript`. Success records the place id (0x25080), a session
    // report and a `Visit/Success/Join` page view; failure alerts
    // (`ConnectionError`, `ConnectionErrorGameFull` on 6,
    // `ConnectionErrorGameEnded` on 5) then `leaveGame` + failure forward.
    if join_response.contains("\"status\":2") {
        if request == 2 {
            PLACE_SIGNED_SCRIPTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        } else {
            let url = join_response.find("joinScriptUrl").map(|at| {
                let tail = &join_response[at..];
                let end = tail.find([',', '"', '}']).unwrap_or(tail.len());
                tail[..end].replace("\\/", "/")
            });
            *PLACE_LAST_JOIN_SCRIPT.lock() = url.unwrap_or_else(|| join_response.to_owned());
            PLACE_EXECUTED_SCRIPTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
        stub_0x25080(place_id);
        PLACE_SESSION_REPORTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        PLACE_PAGE_VIEWS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    } else {
        let alert = if join_response.contains("\"status\":6") {
            "ConnectionErrorGameFull"
        } else if join_response.contains("\"status\":5") {
            "ConnectionErrorGameEnded"
        } else {
            "ConnectionError"
        };
        *PLACE_LAST_ALERT.lock() = alert.to_owned();
        stub_0x298e0(true, true);
        PLACE_FAILURE_FORWARDS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        PLACE_IS_PLAYING.store(false, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x289a8 — -[PlaceLauncher startGame:controller:request:presentGameAutomatically:]
// type: char __cdecl(PlaceLauncher *self, SEL, int, id, int, char)
#[doc(alias = "-[PlaceLauncher startGame:controller:request:presentGameAutomatically:]")]
pub fn stub_0x289a8(place_id: i32, request: u32, present_automatically: bool) -> bool {
    // IDA 0x289a8: `startGame:controller:request:...` sets up the preloaded
    // game (0x267bc, `isApp` = request 2), binds `joinGamePlaceId` (0x278a8)
    // over place id + request, and starts it via
    // `startGame:controller:preloadedGame:` (0x29490).
    if !stub_0x267bc(true, request == 2) {
        return false;
    }
    *PLACE_LAST_JOIN_REQUEST.lock() = (place_id, request);
    stub_0x29490(present_automatically)
}

// 0x28ba8 — -[PlaceLauncher startGameSolo:controller:presentGameAutomatically:]
// type: char __cdecl(PlaceLauncher *self, SEL, int, id, char)
#[doc(alias = "-[PlaceLauncher startGameSolo:controller:presentGameAutomatically:]")]
pub fn stub_0x28ba8(place_id: i32, present_automatically: bool) -> bool {
    // IDA 0x28ba8: `startGameSolo:...` sets up the preloaded game
    // (0x267bc), binds `joinGamePlaceIdSolo` (0x28d98) over the place id,
    // and starts it via `startGame:controller:preloadedGame:` (0x29490).
    if !stub_0x267bc(true, false) {
        return false;
    }
    PLACE_LAST_SOLO_JOIN.store(place_id, std::sync::atomic::Ordering::SeqCst);
    stub_0x29490(present_automatically)
}

// 0x28d98 — __ZL19joinGamePlaceIdSoloiN5boost10shared_ptrIN3RBX4GameEEE
#[doc(alias = "joinGamePlaceIdSolo(int,rbx_core::SharedPtr<RBX::Game>)")]
#[doc(alias = "__ZL19joinGamePlaceIdSoloiN5boost10shared_ptrIN3RBX4GameEEE")]
pub fn stub_0x28d98(place_id: i32) {
    // IDA 0x28d98: `joinGamePlaceIdSolo` formats
    // `loadfile('<base>game/visit.ashx?placeid=%d')()` (or the workshop
    // start place for id < 1), executes it on the game, then records the
    // place id (0x25080) and a `VisitSolo/Success/Join` page view. The
    // `RobloxInfo` base URL prefixes on device; the formatted tail records.
    let script = if place_id < 1 {
        "game:Load('rbxasset://places/workshop/workshopStartPlace.rbxl') loadfile('game/visit.ashx')()"
            .to_owned()
    } else {
        format!("loadfile('game/visit.ashx?placeid={place_id}')()")
    };
    *PLACE_LAST_LOAD_SCRIPT.lock() = script;
    PLACE_EXECUTED_SCRIPTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    stub_0x25080(place_id);
    PLACE_PAGE_VIEWS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x29280 — -[PlaceLauncher startGameWithJoinScript:controller:presentGameAutomatically:]
// type: char __cdecl(PlaceLauncher *self, SEL, id, id, char)
#[doc(alias = "-[PlaceLauncher startGameWithJoinScript:controller:presentGameAutomatically:]")]
pub fn stub_0x29280(script: &str, present_automatically: bool) -> bool {
    // IDA 0x29280: `startGameWithJoinScript:...` sets up the preloaded game
    // (0x267bc), binds `joinGameWithJoinScript` (0x26990) over the script,
    // and starts it via `startGame:controller:preloadedGame:` (0x29490).
    if !stub_0x267bc(true, false) {
        return false;
    }
    *PLACE_LAST_JOIN_SCRIPT.lock() = script.to_owned();
    stub_0x29490(present_automatically)
}

// 0x29490 — -[PlaceLauncher startGame:controller:preloadedGame:presentGameAutomatically:]
// type: char __cdecl(PlaceLauncher *self, SEL, function0<void>, id, shared_ptr<RBX::Game>, char)
#[doc(alias = "-[PlaceLauncher startGame:controller:preloadedGame:presentGameAutomatically:]")]
pub fn stub_0x29490(present_automatically: bool) -> bool {
    // IDA 0x29490: `startGame:controller:preloadedGame:...` runs the bound
    // join closure on a detached `GameStartScript` thread (`boost::thread`
    // hop is drop glue), then `createGame:presentGameAutomatically:`
    // (0x261d8), always reporting started.
    PLACE_JOIN_THREADS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    stub_0x261d8(true, false, false, false, present_automatically);
    true
}

// 0x295c0 — -[PlaceLauncher leaveGameShutdown]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher leaveGameShutdown]")]
pub fn stub_0x295c0() {
    // IDA 0x295c0: `leaveGameShutdown` posts the
    // `startLeaveGameNotification`, dismisses the ogre view controller
    // unanimated, and its completion block (0x29684) tears the game down.
    PLACE_LEAVE_POSTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    stub_0x29684();
}

// 0x29684 — ___34-[PlaceLauncher leaveGameShutdown]_block_invoke
#[doc(alias = "___34-[PlaceLauncher leaveGameShutdown]_block_invoke")]
pub fn stub_0x29684() {
    // IDA 0x29684: `leaveGameShutdown` completion releases the ogre view
    // controller/view/window, runs `deleteRobloxView` (0x25440), clears the
    // playing/leaving flags, posts the leave notification, drops the
    // `RobloxGameState` default and ends the background task (`NSLog`
    // traffic is drop glue).
    stub_0x25440();
    PLACE_CURRENTLY_PLAYING.store(false, std::sync::atomic::Ordering::SeqCst);
    PLACE_IS_LEAVING.store(false, std::sync::atomic::Ordering::SeqCst);
    PLACE_BG_TASK.store(false, std::sync::atomic::Ordering::SeqCst);
    PLACE_GAME_STATE.lock().clear();
    PLACE_LEAVE_POSTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x298a0 — ___copy_helper_block_191
#[doc(alias = "___copy_helper_block_191")]
pub fn stub_0x298a0() {
    // IDA 0x298a0: `__copy_helper_block_191` retains the captured objects
    // (`_Block_object_assign`). `Arc` clone glue covers it; no explicit body.
}

// 0x298c4 — ___destroy_helper_block_192
#[doc(alias = "___destroy_helper_block_192")]
pub fn stub_0x298c4() {
    // IDA 0x298c4: `__destroy_helper_block_192` releases the captured
    // objects (`_Block_object_dispose`). `Arc` drop glue covers it; no
    // explicit body.
}

// 0x298e0 — -[PlaceLauncher leaveGame]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher leaveGame]")]
pub fn stub_0x298e0(currently_playing: bool, has_ogre_view: bool) {
    // IDA 0x298e0: `leaveGame` proceeds only while playing and not already
    // leaving with an ogre view up: it re-enables the idle timer, records
    // the `leaveGame` state, closes child connections (0x2b5e0), files a
    // session report + `Visit/Success/LeaveGame` page view, opens a
    // background task (expiration block 0x29bb4), and on iOS 6+ dispatches
    // `leaveGameShutdown` (0x29c74 → 0x295c0) on the main queue, else shuts
    // down inline. UIKit/GCD hops are drop glue.
    if !currently_playing || PLACE_IS_LEAVING.swap(true, std::sync::atomic::Ordering::SeqCst) {
        return;
    }
    if !has_ogre_view {
        return;
    }
    PLACE_IDLE_TIMER_DISABLED.store(false, std::sync::atomic::Ordering::SeqCst);
    *PLACE_GAME_STATE.lock() = "leaveGame".to_owned();
    PLACE_CHILD_ADDED_CONNECTED.store(false, std::sync::atomic::Ordering::SeqCst);
    PLACE_SESSION_REPORTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    PLACE_BG_TASK.store(true, std::sync::atomic::Ordering::SeqCst);
    stub_0x29bb4();
    stub_0x29c74();
}

// 0x29bb4 — ___26-[PlaceLauncher leaveGame]_block_invoke
#[doc(alias = "___26-[PlaceLauncher leaveGame]_block_invoke")]
pub fn stub_0x29bb4() {
    // IDA 0x29bb4: `leaveGame` expiration block ends the background task
    // and invalidates the handle.
    PLACE_BG_TASK.store(false, std::sync::atomic::Ordering::SeqCst);
}

// 0x29c34 — ___copy_helper_block_217
#[doc(alias = "___copy_helper_block_217")]
pub fn stub_0x29c34() {
    // IDA 0x29c34: `__copy_helper_block_217` retains the captured objects
    // (`_Block_object_assign`). `Arc` clone glue covers it; no explicit body.
}

// 0x29c58 — ___destroy_helper_block_218
#[doc(alias = "___destroy_helper_block_218")]
pub fn stub_0x29c58() {
    // IDA 0x29c58: `__destroy_helper_block_218` releases the captured
    // objects (`_Block_object_dispose`). `Arc` drop glue covers it; no
    // explicit body.
}

// 0x29c74 — ___26-[PlaceLauncher leaveGame]_block_invoke231
#[doc(alias = "___26-[PlaceLauncher leaveGame]_block_invoke231")]
pub fn stub_0x29c74() {
    // IDA 0x29c74: `leaveGame` main-queue block runs `leaveGameShutdown`
    // (0x295c0) on iOS 6+; older releases shut down inline (same
    // 0x295c0 path, cf. 0x298e0).
    stub_0x295c0();
}

// 0x29c88 — ___copy_helper_block_232
#[doc(alias = "___copy_helper_block_232")]
pub fn stub_0x29c88() {
    // IDA 0x29c88: `__copy_helper_block_232` retains the captured objects
    // (`_Block_object_assign`). `Arc` clone glue covers it; no explicit body.
}

// 0x29c94 — ___destroy_helper_block_233
#[doc(alias = "___destroy_helper_block_233")]
pub fn stub_0x29c94() {
    // IDA 0x29c94: `__destroy_helper_block_233` releases the captured
    // objects (`_Block_object_dispose`). `Arc` drop glue covers it; no
    // explicit body.
}

// 0x29c9c — -[PlaceLauncher disableViewBecauseGoingToBackground]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher disableViewBecauseGoingToBackground]")]
pub fn stub_0x29c9c() {
    // IDA 0x29c9c: `disableViewBecauseGoingToBackground` stops rendering
    // on the attached `RobloxView`.
    if PLACE_ROBX_VIEW.load(std::sync::atomic::Ordering::SeqCst) {
        PLACE_VIEW_BACKGROUNDED.store(true, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x29cb4 — -[PlaceLauncher enableViewBecauseGoingToForeground]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher enableViewBecauseGoingToForeground]")]
pub fn stub_0x29cb4() {
    // IDA 0x29cb4: `enableViewBecauseGoingToForeground` resumes rendering
    // on the attached `RobloxView` (cf. 0x29c9c).
    if PLACE_ROBX_VIEW.load(std::sync::atomic::Ordering::SeqCst) {
        PLACE_VIEW_BACKGROUNDED.store(false, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x29ccc — -[PlaceLauncher teleport:withAuthentication:withScript:]
// type: void __cdecl(PlaceLauncher *self, SEL, id, id, id)
#[doc(alias = "-[PlaceLauncher teleport:withAuthentication:withScript:]")]
pub fn stub_0x29ccc(place_url: &str, auth: &str, script: &str) {
    // IDA 0x29ccc: `teleport:...` re-records the last-non-game controller,
    // builds a fresh `SecurePlayerGame`, spawns the `joinGameTeleport`
    // (0x2a350) thread over url/auth/script, drops the current view
    // (0x25440), clips the ogre view and runs the 0.5s teleport animation
    // (blocks 0x2a8c8/0x2a99c, drop glue).
    PLACE_LAST_NON_GAME.store(true, std::sync::atomic::Ordering::SeqCst);
    PLACE_UNSECURED_GAME.store(false, std::sync::atomic::Ordering::SeqCst);
    *PLACE_LAST_TELEPORT.lock() = (place_url.to_owned(), auth.to_owned(), script.to_owned());
    PLACE_JOIN_THREADS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    stub_0x25440();
    PLACE_TELEPORT_ANIMS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x2a350 — __ZL16joinGameTeleportSsSsSsP8NSObjectN5boost10shared_ptrIN3RBX4GameEEE
#[doc(alias = "joinGameTeleport(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>)")]
#[doc(alias = "__ZL16joinGameTeleportSsSsSsP8NSObjectN5boost10shared_ptrIN3RBX4GameEEE")]
pub fn stub_0x2a350(place_url: &str, suggest: &str, script: &str, has_controller: bool) {
    // IDA 0x2a350: `joinGameTeleport` appends `?suggest=` when non-empty,
    // GETs the teleport URL, runs `executeUrlScript` on the game and pings
    // `handleStartGameSuccess` on the controller when attached.
    let full = if suggest.is_empty() {
        place_url.to_owned()
    } else {
        format!("{place_url}?suggest={suggest}")
    };
    *PLACE_LAST_JOIN_URL.lock() = full;
    *PLACE_LAST_JOIN_SCRIPT.lock() = script.to_owned();
    PLACE_EXECUTED_SCRIPTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    if has_controller {
        PLACE_START_SUCCESSES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x2a8c8 — ___56-[PlaceLauncher teleport:withAuthentication:withScript:]_block_invoke
#[doc(alias = "___56-[PlaceLauncher teleport:withAuthentication:withScript:]_block_invoke")]
pub fn stub_0x2a8c8(has_view: bool) {
    // IDA 0x2a8c8: teleport animation block centers a 1x1 frame on the ogre
    // view (pure UIKit geometry, drop glue); without a view it applies the
    // fallback frame instead.
    let _ = has_view;
}

// 0x2a988 — ___copy_helper_block_243
#[doc(alias = "___copy_helper_block_243")]
pub fn stub_0x2a988() {
    // IDA 0x2a988: `__copy_helper_block_243` retains the captured objects
    // (`_Block_object_assign`). `Arc` clone glue covers it; no explicit body.
}

// 0x2a994 — ___destroy_helper_block_244
#[doc(alias = "___destroy_helper_block_244")]
pub fn stub_0x2a994() {
    // IDA 0x2a994: `__destroy_helper_block_244` releases the captured
    // objects (`_Block_object_dispose`). `Arc` drop glue covers it; no
    // explicit body.
}

// 0x2a99c — ___56-[PlaceLauncher teleport:withAuthentication:withScript:]_block_invoke246
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "___56-[PlaceLauncher teleport:withAuthentication:withScript:]_block_invoke246")]
pub fn stub_0x2a99c(datamodel_ready: bool, has_overlay: bool, warnings_enabled: bool) {
    // IDA 0x2a99c: teleport completion runs
    // `finishGameSetup:gameViewController:` (0x25498), then submits the
    // `finishTeleport` (0x2aba4) datamodel task (`boost::bind` is drop glue).
    stub_0x25498(datamodel_ready, has_overlay, warnings_enabled);
    PLACE_CONTROL_TASKS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x2aba4 — __ZL14finishTeleportP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEEPNS3_18FunctionMarshallerE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "finishTeleport(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *)")]
#[doc(alias = "__ZL14finishTeleportP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEEPNS3_18FunctionMarshallerE")]
pub fn stub_0x2aba4() {
    // IDA 0x2aba4: `finishTeleport` binds `finishTeleportHelper` (0x2b754)
    // over view + game and runs it through `FunctionMarshaller::Execute`
    // (same shape as 0x2643c).
    PLACE_CONTROL_EXECS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x2acec — ___copy_helper_block_247
// type: void __fastcall(_DWORD *, const shared_count *)
#[doc(alias = "___copy_helper_block_247")]
pub fn stub_0x2acec() {
    // IDA 0x2acec: `__copy_helper_block_247` retains the captured objects
    // (`_Block_object_assign`). `Arc` clone glue covers it; no explicit body.
}

// 0x2ada4 — ___destroy_helper_block_248
#[doc(alias = "___destroy_helper_block_248")]
pub fn stub_0x2ada4() {
    // IDA 0x2ada4: `__destroy_helper_block_248` releases the captured
    // objects (`_Block_object_dispose`). `Arc` drop glue covers it; no
    // explicit body.
}

// 0x2ae44 — -[PlaceLauncher isCurrentlyPlayingGame]
// type: char __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher isCurrentlyPlayingGame]")]
pub fn stub_0x2ae44() -> bool {
    // IDA 0x2ae44: `isCurrentlyPlayingGame` returns the playing flag.
    PLACE_CURRENTLY_PLAYING.load(std::sync::atomic::Ordering::SeqCst)
}

// 0x2ae54 — -[PlaceLauncher applicationDidReceiveMemoryWarning]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher applicationDidReceiveMemoryWarning]")]
pub fn stub_0x2ae54(child_connected: bool, player_connected: bool, warnings_enabled: bool) {
    // IDA 0x2ae54: `applicationDidReceiveMemoryWarning` ignores the warning
    // out of game; in game it files a `PlayErrors` analytics event
    // (`OutOfMemory_EarlyExit` + session 5 while a child/player connection
    // is live, else `OutOfMemory` + session 6), closes child connections
    // (0x2b5e0), shows the `MemoryError` alert when warnings are on, and
    // leaves the game (0x298e0).
    if !stub_0x2ae44() {
        return;
    }
    PLACE_MEM_WARNING.store(true, std::sync::atomic::Ordering::SeqCst);
    let early_exit = child_connected || player_connected;
    let action = if early_exit { "OutOfMemory_EarlyExit" } else { "OutOfMemory" };
    PLACE_ANALYTICS_EVENTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    *PLACE_LAST_ANALYTICS_LABEL.lock() = format!(
        "{action}:{}",
        PLACE_LAST_ID.load(std::sync::atomic::Ordering::SeqCst)
    );
    PLACE_SESSION_REPORTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    stub_0x2b5e0();
    if warnings_enabled {
        *PLACE_LAST_ALERT.lock() = "MemoryError".to_owned();
    }
    stub_0x298e0(true, true);
}

// 0x2b1bc — -[PlaceLauncher childAdded:]
// type: void __cdecl(PlaceLauncher *self, SEL, shared_ptr<RBX::Instance>)
#[doc(alias = "-[PlaceLauncher childAdded:]")]
pub fn stub_0x2b1bc(has_view: bool, has_datamodel: bool, players_present: bool, is_player_child: bool) {
    // IDA 0x2b1bc: `childAdded:` with no view, no datamodel, no `Players`
    // service or no player closes the child connections (0x2b5e0,
    // `NSLog` traffic is drop glue). Otherwise it binds `playerLoaded:`
    // (0x2b548) onto the player-added signal, stores the player connection
    // and disconnects the child connection.
    if !(has_view && has_datamodel && players_present && is_player_child) {
        stub_0x2b5e0();
        return;
    }
    PLACE_PLAYER_CONNECTED.store(true, std::sync::atomic::Ordering::SeqCst);
    PLACE_CHILD_ADDED_CONNECTED.store(false, std::sync::atomic::Ordering::SeqCst);
}

// 0x2b548 — -[PlaceLauncher playerLoaded:]
// type: void __cdecl(PlaceLauncher *self, SEL, shared_ptr<RBX::Instance>)
#[doc(alias = "-[PlaceLauncher playerLoaded:]")]
pub fn stub_0x2b548() {
    // IDA 0x2b548: `playerLoaded:` disconnects the player connection,
    // closes the child connections (0x2b5e0) and records the `inGame`
    // state.
    PLACE_PLAYER_CONNECTED.store(false, std::sync::atomic::Ordering::SeqCst);
    stub_0x2b5e0();
    *PLACE_GAME_STATE.lock() = "inGame".to_owned();
}

// 0x2b5e0 — -[PlaceLauncher closeChildConnections]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher closeChildConnections]")]
pub fn stub_0x2b5e0() {
    // IDA 0x2b5e0: `closeChildConnections` disconnects the child + player
    // connections and stops the free-memory checker.
    PLACE_CHILD_ADDED_CONNECTED.store(false, std::sync::atomic::Ordering::SeqCst);
    PLACE_PLAYER_CONNECTED.store(false, std::sync::atomic::Ordering::SeqCst);
    PLACE_MEM_CHECKER.store(false, std::sync::atomic::Ordering::SeqCst);
}

// 0x2b654 — -[PlaceLauncher .cxx_destruct]
// type: void __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher .cxx_destruct]")]
pub fn stub_0x2b654() {
    // IDA 0x2b654: `PlaceLauncher .cxx_destruct` releases the player/child
    // connection slots and the teleporter. `Arc`/slot drop glue covers it;
    // the observable flags clear.
    PLACE_CHILD_ADDED_CONNECTED.store(false, std::sync::atomic::Ordering::SeqCst);
    PLACE_PLAYER_CONNECTED.store(false, std::sync::atomic::Ordering::SeqCst);
    PLACE_TELEPORTER.store(false, std::sync::atomic::Ordering::SeqCst);
}

// 0x2b724 — -[PlaceLauncher .cxx_construct]
// type: id __cdecl(PlaceLauncher *self, SEL)
#[doc(alias = "-[PlaceLauncher .cxx_construct]")]
pub fn stub_0x2b724() {
    // IDA 0x2b724: `PlaceLauncher .cxx_construct` zeroes the teleporter +
    // child/player connection slots. Default-init glue; no explicit body.
}

// 0x2b754 — __ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE
#[doc(alias = "finishTeleportHelper(RobloxView *,rbx_core::SharedPtr<RBX::Game>)")]
#[doc(alias = "__ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE")]
pub fn stub_0x2b754(has_main_vc: bool, has_subview: bool) {
    // IDA 0x2b754: `finishTeleportHelper` sets the game on the ogre view's
    // first subview, then runs the 0.5s finish animation (blocks 0x2b980 /
    // 0x2ba14, drop glue).
    if has_main_vc && has_subview {
        PLACE_SUBVIEW_GAME_SET.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    if has_main_vc {
        PLACE_TELEPORT_ANIMS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x2b980 — ____ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE_block_invoke
#[doc(alias = "____ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE_block_invoke")]
pub fn stub_0x2b980(has_screen: bool) {
    // IDA 0x2b980: finish animation block stretches the ogre view over the
    // main-screen bounds (pure UIKit geometry, drop glue); without a screen
    // it applies the zero frame instead.
    let _ = has_screen;
}

// 0x2ba00 — ___copy_helper_block_425
#[doc(alias = "___copy_helper_block_425")]
pub fn stub_0x2ba00() {
    // IDA 0x2ba00: `__copy_helper_block_425` retains the captured objects
    // (`_Block_object_assign`). `Arc` clone glue covers it; no explicit body.
}

// 0x2ba0c — ___destroy_helper_block_426
#[doc(alias = "___destroy_helper_block_426")]
pub fn stub_0x2ba0c() {
    // IDA 0x2ba0c: `__destroy_helper_block_426` releases the captured
    // objects (`_Block_object_dispose`). `Arc` drop glue covers it; no
    // explicit body.
}

// 0x2ba14 — ____ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE_block_invoke428
#[doc(alias = "____ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE_block_invoke428")]
pub fn stub_0x2ba14() {
    // IDA 0x2ba14: finish-animation completion clears `clipsToBounds` on the
    // ogre view (pure UIKit state, drop glue); no explicit body.
}

// 0x2ba40 — ___copy_helper_block_429
#[doc(alias = "___copy_helper_block_429")]
pub fn stub_0x2ba40() {
    // IDA 0x2ba40: `__copy_helper_block_429` retains the captured objects
    // (`_Block_object_assign`). `Arc` clone glue covers it; no explicit body.
}

// 0x2ba4c — ___destroy_helper_block_430
#[doc(alias = "___destroy_helper_block_430")]
pub fn stub_0x2ba4c() {
    // IDA 0x2ba4c: `__destroy_helper_block_430` releases the captured
    // objects (`_Block_object_dispose`). `Arc` drop glue covers it; no
    // explicit body.
}

// 0x2ba54 — __ZL16executeUrlScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs
#[doc(alias = "executeUrlScript(rbx_core::SharedPtr<RBX::DataModel>,std::string const&)")]
#[doc(alias = "__ZL16executeUrlScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs")]
pub fn stub_0x2ba54(script: &str, is_url: bool) {
    // IDA 0x2ba54: `executeUrlScript` impersonates level 7, fetches the URL
    // content under the datamodel legacy lock and runs it via
    // `executeSignedScript` (0x2bdb0), then resets the security context.
    // Non-URL input skips the fetch. Content fetch/networking is drop glue;
    // the executed script records.
    *PLACE_LAST_JOIN_SCRIPT.lock() = script.to_owned();
    if is_url {
        stub_0x2bdb0(script, true);
    }
}

// 0x2bdb0 — __ZL19executeSignedScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs
#[doc(alias = "executeSignedScript(rbx_core::SharedPtr<RBX::DataModel>,std::string const&)")]
#[doc(alias = "__ZL19executeSignedScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs")]
pub fn stub_0x2bdb0(script: &str, verified: bool) {
    // IDA 0x2bdb0: `executeSignedScript` verifies the script signature and
    // runs the verified source via `executeScript` (0x2bf74).
    if verified {
        PLACE_SIGNED_SCRIPTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    stub_0x2bf74(script, true);
}

// 0x2bf74 — __ZL13executeScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs
#[doc(alias = "executeScript(rbx_core::SharedPtr<RBX::DataModel>,std::string const&)")]
#[doc(alias = "__ZL13executeScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs")]
pub fn stub_0x2bf74(script: &str, scripts_enabled: bool) {
    // IDA 0x2bf74: `executeScript` takes the datamodel legacy lock and,
    // with scripts enabled, runs the trusted source in a new
    // `ScriptContext` thread. Lock/thread hops are drop glue; the executed
    // script records.
    if scripts_enabled {
        *PLACE_LAST_LOAD_SCRIPT.lock() = script.to_owned();
        PLACE_EXECUTED_SCRIPTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x2c138 — ____ZL15presentGameViewv_block_invoke
// type: void __cdecl(id)
#[doc(alias = "____ZL15presentGameViewv_block_invoke")]
pub fn stub_0x2c138() -> ! {
    todo!("0x2c138 ____ZL15presentGameViewv_block_invoke")
}

// 0x2c1f8 — ____ZL15presentGameViewv_block_invoke_2
// type: id __fastcall(int)
#[doc(alias = "____ZL15presentGameViewv_block_invoke_2")]
pub fn stub_0x2c1f8() -> ! {
    todo!("0x2c1f8 ____ZL15presentGameViewv_block_invoke_2")
}

// 0x2c210 — ___copy_helper_block_499
#[doc(alias = "___copy_helper_block_499")]
pub fn stub_0x2c210() -> ! {
    todo!("0x2c210 ___copy_helper_block_499")
}

// 0x2c21c — ___destroy_helper_block_500
#[doc(alias = "___destroy_helper_block_500")]
pub fn stub_0x2c21c() -> ! {
    todo!("0x2c21c ___destroy_helper_block_500")
}

// 0x2c224 — __ZL21initControlViewHelperP10RobloxViewa
// type: _DWORD __fastcall(RobloxView *, signed __int8)
#[doc(alias = "initControlViewHelper(RobloxView *,signed char)")]
#[doc(alias = "__ZL21initControlViewHelperP10RobloxViewa")]
pub fn stub_0x2c224() -> ! {
    todo!("0x2c224 initControlViewHelper(RobloxView *,signed char)")
}

// 0x2c5b0 — __ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEE9singletonEv
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, boost::mutex *, char, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEE9singletonEv")]
pub fn stub_0x2c5b0() -> ! {
    todo!("0x2c5b0 __ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEE9singletonEv")
}

// 0x2c764 — __ZNK3RBX15ServiceProvider4findINS_10GuiServiceEEEPT_v
// type: int __fastcall(pthread_mutex_t *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::GuiService * RBX::ServiceProvider::find<RBX::GuiService>(void)const")]
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_10GuiServiceEEEPT_v")]
pub fn stub_0x2c764() -> ! {
    todo!("0x2c764 RBX::GuiService * RBX::ServiceProvider::find<RBX::GuiService>(void)const")
}

// 0x2c8c0 — __ZN3rbx7signals6signalIFvSsEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string)>::connect<boost::function<void ()(std::string)>>(boost::function<void ()(std::string)> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_")]
pub fn stub_0x2c8c0() -> ! {
    todo!("0x2c8c0 rbx::signals::connection rbx::signals::signal<void ()(std::string)>::connect<boost::function<void ()(std::string)>>(boost::function<void ()(std::string)> const&)")
}

// 0x2c9a8 — __ZN5boost10shared_ptrIN3RBX4GameEEC1INS1_16SecurePlayerGameEEEPT_
#[doc(alias = "rbx_core::SharedPtr<RBX::Game>::shared_ptr<RBX::SecurePlayerGame>(RBX::SecurePlayerGame *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX4GameEEC1INS1_16SecurePlayerGameEEEPT_")]
pub fn stub_0x2c9a8() -> ! {
    todo!("0x2c9a8 boost::shared_ptr<RBX::Game>::shared_ptr<RBX::SecurePlayerGame>(RBX::SecurePlayerGame *)")
}

// 0x2ca7c — __ZN5boost4bindIvRKSsNS_10shared_ptrIN3RBX4GameEEEPKcS6_EENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list_av_2<char const*,rbx_core::SharedPtr<RBX::Game>>::type> boost::bind<void,std::string const&,rbx_core::SharedPtr<RBX::Game>,char const*,rbx_core::SharedPtr<RBX::Game>>(void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),char const*,rbx_core::SharedPtr<RBX::Game>)")]
#[doc(alias = "__ZN5boost4bindIvRKSsNS_10shared_ptrIN3RBX4GameEEEPKcS6_EENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_")]
pub fn stub_0x2ca7c() -> ! {
    todo!("0x2ca7c boost::_bi::bind_t<void,void (*)(std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list_av_2<char const*,boost::shared_ptr<RBX::Game>>::type> boost::bind<void,std::string const&,boost::shared_ptr<RBX::Game>,char const*,boost::shared_ptr<RBX::Game>>(void (*)(std::string const&,boost::shared_ptr<RBX::Game>),char const*,boost::shared_ptr<RBX::Game>)")
}

// 0x2cb64 — __ZN5boost4bindIviRKSsNS_10shared_ptrIN3RBX4GameEEEiPKcS6_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_ENS9_9list_av_3IT3_T4_T5_E4typeEEESG_SI_SJ_SK_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list_av_3<int,char const*,rbx_core::SharedPtr<RBX::Game>>::type> boost::bind<void,int,std::string const&,rbx_core::SharedPtr<RBX::Game>,int,char const*,rbx_core::SharedPtr<RBX::Game>>(void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),int,char const*,rbx_core::SharedPtr<RBX::Game>)")]
#[doc(alias = "__ZN5boost4bindIviRKSsNS_10shared_ptrIN3RBX4GameEEEiPKcS6_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_ENS9_9list_av_3IT3_T4_T5_E4typeEEESG_SI_SJ_SK_")]
pub fn stub_0x2cb64() -> ! {
    todo!("0x2cb64 boost::_bi::bind_t<void,void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),boost::_bi::list_av_3<int,char const*,boost::shared_ptr<RBX::Game>>::type> boost::bind<void,int,std::string const&,boost::shared_ptr<RBX::Game>,int,char const*,boost::shared_ptr<RBX::Game>>(void (*)(int,std::string const&,boost::shared_ptr<RBX::Game>),int,char const*,boost::shared_ptr<RBX::Game>)")
}

// 0x2cc54 — __ZN5boost4bindIviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestiS4_S5_EENS_3_bi6bind_tIT_PFS8_T0_T1_T2_ENS6_9list_av_3IT3_T4_T5_E4typeEEESD_SF_SG_SH_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list_av_3<int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest>::type> boost::bind<void,int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest,int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest>(void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest)")]
#[doc(alias = "__ZN5boost4bindIviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestiS4_S5_EENS_3_bi6bind_tIT_PFS8_T0_T1_T2_ENS6_9list_av_3IT3_T4_T5_E4typeEEESD_SF_SG_SH_")]
pub fn stub_0x2cc54() -> ! {
    todo!("0x2cc54 boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),boost::_bi::list_av_3<int,boost::shared_ptr<RBX::Game>,JoinGameRequest>::type> boost::bind<void,int,boost::shared_ptr<RBX::Game>,JoinGameRequest,int,boost::shared_ptr<RBX::Game>,JoinGameRequest>(void (*)(int,boost::shared_ptr<RBX::Game>,JoinGameRequest),int,boost::shared_ptr<RBX::Game>,JoinGameRequest)")
}

// 0x2cd44 — __ZN5boost4bindIviNS_10shared_ptrIN3RBX4GameEEEiS4_EENS_3_bi6bind_tIT_PFS7_T0_T1_ENS5_9list_av_2IT2_T3_E4typeEEESB_SD_SE_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list_av_2<int,rbx_core::SharedPtr<RBX::Game>>::type> boost::bind<void,int,rbx_core::SharedPtr<RBX::Game>,int,rbx_core::SharedPtr<RBX::Game>>(void (*)(int,rbx_core::SharedPtr<RBX::Game>),int,rbx_core::SharedPtr<RBX::Game>)")]
#[doc(alias = "__ZN5boost4bindIviNS_10shared_ptrIN3RBX4GameEEEiS4_EENS_3_bi6bind_tIT_PFS7_T0_T1_ENS5_9list_av_2IT2_T3_E4typeEEESB_SD_SE_")]
pub fn stub_0x2cd44() -> ! {
    todo!("0x2cd44 boost::_bi::bind_t<void,void (*)(int,boost::shared_ptr<RBX::Game>),boost::_bi::list_av_2<int,boost::shared_ptr<RBX::Game>>::type> boost::bind<void,int,boost::shared_ptr<RBX::Game>,int,boost::shared_ptr<RBX::Game>>(void (*)(int,boost::shared_ptr<RBX::Game>),int,boost::shared_ptr<RBX::Game>)")
}

// 0x2ce2c — __ZN5boost4bindIvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEESsSsSsP24RobloxPageViewControllerS6_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_T3_T4_ENS9_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESI_SK_SL_SM_SN_SO_
// type: int __fastcall(int, int, std::string *, int, std::string *, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list_av_5<std::string,std::string,std::string,RobloxPageViewController *,rbx_core::SharedPtr<RBX::Game>>::type> boost::bind<void,std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>,std::string,std::string,std::string,RobloxPageViewController *,rbx_core::SharedPtr<RBX::Game>>(void (*)(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>),std::string,std::string,std::string,RobloxPageViewController *,rbx_core::SharedPtr<RBX::Game>)")]
#[doc(alias = "__ZN5boost4bindIvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEESsSsSsP24RobloxPageViewControllerS6_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_T3_T4_ENS9_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESI_SK_SL_SM_SN_SO_")]
pub fn stub_0x2ce2c() -> ! {
    todo!("0x2ce2c boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list_av_5<std::string,std::string,std::string,RobloxPageViewController *,boost::shared_ptr<RBX::Game>>::type> boost::bind<void,std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>,std::string,std::string,std::string,RobloxPageViewController *,boost::shared_ptr<RBX::Game>>(void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),std::string,std::string,std::string,RobloxPageViewController *,boost::shared_ptr<RBX::Game>)")
}

// 0x2d280 — __ZN5boost4bindIvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS4_18FunctionMarshallerES2_S6_S8_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_ENS9_9list_av_3IT3_T4_T5_E4typeEEESG_SI_SJ_SK_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list_av_3<RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *>::type> boost::bind<void,RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *,RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *>(void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *)")]
#[doc(alias = "__ZN5boost4bindIvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS4_18FunctionMarshallerES2_S6_S8_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_ENS9_9list_av_3IT3_T4_T5_E4typeEEESG_SI_SJ_SK_")]
pub fn stub_0x2d280() -> ! {
    todo!("0x2d280 boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list_av_3<RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *>::type> boost::bind<void,RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *,RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *>(void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *)")
}

// 0x2d370 — __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS7_5list3INS7_5valueISA_EENSJ_ISD_EENSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS7_5list3INS7_5valueISA_EENSJ_ISD_EENSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x2d370() -> ! {
    todo!("0x2d370 __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS7_5list3INS7_5valueISA_EENSJ_ISD_EENSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")
}

// 0x2d458 — __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, struct _Unwind_Exception *lpuexcpt, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x2d458() -> ! {
    todo!("0x2d458 __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")
}

// 0x2d544 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEEEEEvT_
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>)")]
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEEEEEvT_")]
pub fn stub_0x2d544() -> ! {
    todo!("0x2d544 void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>)")
}

// 0x2d644 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x2d644() -> ! {
    todo!("0x2d644 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x2d660 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEvPNS8_9DataModelEE6invokeERNS1_15function_bufferESN_
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEvPNS8_9DataModelEE6invokeERNS1_15function_bufferESN_")]
pub fn stub_0x2d660() -> ! {
    todo!("0x2d660 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")
}

// 0x2d67c — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS3_4GameEEEPNS3_18FunctionMarshallerEENS8_5list3INS8_5valueISB_EENSK_ISE_EENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS3_4GameEEEPNS3_18FunctionMarshallerEENS8_5list3INS8_5valueISB_EENSK_ISE_EENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_0x2d67c() -> ! {
    todo!("0x2d67c bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &)const")
}

// 0x2d768 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS3_4GameEEEPNS3_18FunctionMarshallerEENS8_5list3INS8_5valueISB_EENSK_ISE_EENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS3_4GameEEEPNS3_18FunctionMarshallerEENS8_5list3INS8_5valueISB_EENSK_ISE_EENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_0x2d768() -> ! {
    todo!("0x2d768 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}
