//! reflection — generated_bg_4 — 100 stubs EA-sorted asc global gap filler 0x1d36c..0x1f2e0 not yet in crates/reflection (global 85545 funcs, 64301 gaps reflection; 21244 distinct before, 21344 after)
//! Source: ida/export.json (85545 funcs) global EA asc not in crates/reflection/src — next 100 uncovered for reflection-bg sorted asc
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

/// Gap-filler LoginViewController observable state (IDA 0x1da5c-0x1ec84).
/// The canonical controller, `LoginManager`/`UserInfo`/`StoreManager`
/// models and UIKit views live in `rbx_platform`/UIKit, so their effects
/// record here with matching shapes: the shared instance + field text +
/// base URL become plain cells, the `envs` array becomes a `Vec<String>`,
/// alerts/transitions/observers/bouncer starts become counters +
/// last-value cells.
pub(crate) static LOGIN_SHARED_HANDLE: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);
pub(crate) static LOGIN_ENVS: parking_lot::Mutex<Vec<String>> =
    parking_lot::Mutex::new(Vec::new());
pub(crate) static LOGIN_BASE_URL: parking_lot::Mutex<String> = parking_lot::Mutex::new(String::new());
pub(crate) static LOGIN_LABEL_TEXTS: std::sync::LazyLock<
    parking_lot::Mutex<std::collections::HashMap<String, String>>,
> = std::sync::LazyLock::new(|| parking_lot::Mutex::new(std::collections::HashMap::new()));
pub(crate) static LOGIN_USERNAME_TEXT: parking_lot::Mutex<String> =
    parking_lot::Mutex::new(String::new());
pub(crate) static LOGIN_PASSWORD_TEXT: parking_lot::Mutex<String> =
    parking_lot::Mutex::new(String::new());
pub(crate) static LOGIN_USER_AGENT: parking_lot::Mutex<String> = parking_lot::Mutex::new(String::new());
pub(crate) static LOGIN_REMEMBER_ON: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static LOGIN_SHOWING: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static LOGIN_SKIP_HIDDEN: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static LOGIN_PICKER_HIDDEN: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static LOGIN_ABOUT_HIDDEN: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static LOGIN_INDICATOR_HIDDEN: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(true);
pub(crate) static LOGIN_LOGO_ALPHA_BITS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0x3f800000);
pub(crate) static LOGIN_OBSERVERS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static LOGIN_TRANSITIONS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static MEMORY_BOUNCER_STARTS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static LOGIN_ANALYTICS_VARS: parking_lot::Mutex<Vec<(String, String)>> =
    parking_lot::Mutex::new(Vec::new());
pub(crate) static LAST_LOGIN_ALERT: parking_lot::Mutex<String> =
    parking_lot::Mutex::new(String::new());
pub(crate) static PENDING_LOGIN_SIGNUP: parking_lot::Mutex<(String, String)> =
    parking_lot::Mutex::new((String::new(), String::new()));
pub(crate) static LOGIN_SIGNUP_DISPATCHES: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static LOGIN_FIELD_ALPHA_BITS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0x3f800000);
pub(crate) static PENDING_LOGIN: parking_lot::Mutex<(String, String)> =
    parking_lot::Mutex::new((String::new(), String::new()));
pub(crate) static LOGIN_ATTEMPTS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
pub(crate) static LOGIN_LOGOUTS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
pub(crate) static LAST_PAGE_TRACKING: parking_lot::Mutex<String> = parking_lot::Mutex::new(String::new());
pub(crate) static LOGIN_HOME_SEGUES: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
pub(crate) static LAST_HOME_SEGUE_ANIMATED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static USER_DID_CLICK_PLAY_NOW: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static LOGIN_FIRST_RESPONDER: parking_lot::Mutex<String> =
    parking_lot::Mutex::new(String::new());
pub(crate) static LOGIN_SCROLL_OFFSET: parking_lot::Mutex<(f32, f32)> =
    parking_lot::Mutex::new((0.0, 0.0));
pub(crate) static LOGIN_KEYBOARD_HIDES: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static LOGIN_BACKGROUND_PANS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
/// `-[LoginViewController segueToHomeViewController:]` delivery (IDA
/// 0x1f854, `generated_bg_5`). The animated flag + segue count record;
/// the block dispatch records at the call site.
pub(crate) fn record_home_segue(animated: bool) {
    LOGIN_HOME_SEGUES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    LAST_HOME_SEGUE_ANIMATED.store(animated, std::sync::atomic::Ordering::SeqCst);
}
/// `-[LoginManager doLoginWithUsername:password:]` delivery behind
/// `login:` / `passwordDidEndOnExit:` (IDA 0x1f0d4/0x1f1c8). The manager
/// has no target here; the credentials + attempt count record.
pub(crate) fn record_login_attempt(username: &str, password: &str) {
    *PENDING_LOGIN.lock() = (username.to_owned(), password.to_owned());
    LOGIN_ATTEMPTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}
pub(crate) static LOGIN_OUTLETS: std::sync::LazyLock<
    parking_lot::Mutex<std::collections::HashMap<String, usize>>,
> = std::sync::LazyLock::new(|| parking_lot::Mutex::new(std::collections::HashMap::new()));
pub(crate) fn login_outlet(name: &str) -> usize {
    LOGIN_OUTLETS.lock().get(name).copied().unwrap_or(0)
}
pub(crate) fn set_login_outlet(name: &str, handle: usize) {
    LOGIN_OUTLETS.lock().insert(name.to_owned(), handle);
}
/// `CFURLCreateStringByAddingPercentEscapes` mapping behind
/// `-[NSString stringWithPercentEscape]` (IDA 0x1da08).
pub(crate) fn percent_escape(input: &str) -> String {
    // IDA 0x1da08: escapes `\u{FFFC}=,!$&'()*+;@?\n"<>#\t :/` (0x1da4a).
    // `%` itself is not in the escape set, so pre-escaped input
    // double-encodes — preserved as-is. The non-ASCII member is replaced
    // first so every remaining escape byte is ASCII (all slice indices
    // stay on char boundaries).
    const ESCAPED: &[u8] = b"=,!$&'()*+;@?\n\"<>#\t :/";
    let pre = input.replace('\u{FFFC}', "%EF%BF%BC");
    let mut out = String::with_capacity(pre.len());
    let mut run = 0;
    for (i, b) in pre.as_bytes().iter().enumerate() {
        if ESCAPED.contains(b) {
            out.push_str(&pre[run..i]);
            out.push_str(&format!("%{b:02X}"));
            run = i + 1;
        }
    }
    out.push_str(&pre[run..]);
    out
}

// 0x1d36c — -[HomeViewController setIpId:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setIpId:]")]
pub fn stub_0x1d36c(handle: usize) {
    // IDA 0x1d36c: `setIpId:` retains via `objc_setProperty` (offset 216,
    // 0x1d388). Retain is drop glue; the handle records in the bg_3 outlet
    // registry.
    crate::generated_bg_3::set_home_outlet("ipId", handle);
}

// 0x1d390 — -[HomeViewController btnPlaceLauncher]
// type: UIButton *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController btnPlaceLauncher]")]
pub fn stub_0x1d390() -> usize {
    // IDA 0x1d390: `btnPlaceLauncher` returns the `_btnPlaceLauncher` ivar
    // (0x1d39e). Opaque `id` handle; 0 when unset.
    crate::generated_bg_3::home_outlet("btnPlaceLauncher")
}

// 0x1d3a0 — -[HomeViewController setBtnPlaceLauncher:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setBtnPlaceLauncher:]")]
pub fn stub_0x1d3a0(handle: usize) {
    // IDA 0x1d3a0: `setBtnPlaceLauncher:` retains via `objc_setProperty`
    // (offset 220, 0x1d3bc). Retain is drop glue; the handle records.
    crate::generated_bg_3::set_home_outlet("btnPlaceLauncher", handle);
}

// 0x1d3c4 — -[HomeViewController btnGames]
// type: UIButton *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController btnGames]")]
pub fn stub_0x1d3c4() -> usize {
    // IDA 0x1d3c4: `btnGames` returns the `_btnGames` ivar (0x1d3d2).
    // Opaque `id` handle; 0 when unset.
    crate::generated_bg_3::home_outlet("btnGames")
}

// 0x1d3d4 — -[HomeViewController setBtnGames:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setBtnGames:]")]
pub fn stub_0x1d3d4(handle: usize) {
    // IDA 0x1d3d4: `setBtnGames:` retains via `objc_setProperty` (offset
    // 224, 0x1d3f0). Retain is drop glue; the handle records.
    crate::generated_bg_3::set_home_outlet("btnGames", handle);
}

// 0x1d3f8 — -[HomeViewController btnDebugSettings]
// type: UIButton *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController btnDebugSettings]")]
pub fn stub_0x1d3f8() -> usize {
    // IDA 0x1d3f8: `btnDebugSettings` returns the `_btnDebugSettings` ivar
    // (0x1d406). Opaque `id` handle; 0 when unset.
    crate::generated_bg_3::home_outlet("btnDebugSettings")
}

// 0x1d408 — -[HomeViewController setBtnDebugSettings:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setBtnDebugSettings:]")]
pub fn stub_0x1d408(handle: usize) {
    // IDA 0x1d408: `setBtnDebugSettings:` retains via `objc_setProperty`
    // (offset 228, 0x1d424). Retain is drop glue; the handle records.
    crate::generated_bg_3::set_home_outlet("btnDebugSettings", handle);
}

// 0x1d42c — -[HomeViewController lblRobux]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController lblRobux]")]
pub fn stub_0x1d42c() -> usize {
    // IDA 0x1d42c: `lblRobux` returns the `_lblRobux` ivar (0x1d43a).
    // Opaque `id` handle; 0 when unset.
    crate::generated_bg_3::home_outlet("lblRobux")
}

// 0x1d43c — -[HomeViewController setLblRobux:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setLblRobux:]")]
pub fn stub_0x1d43c(handle: usize) {
    // IDA 0x1d43c: `setLblRobux:` retains via `objc_setProperty` (offset
    // 232, 0x1d458). Retain is drop glue; the handle records.
    crate::generated_bg_3::set_home_outlet("lblRobux", handle);
}

// 0x1d460 — -[HomeViewController lblTix]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController lblTix]")]
pub fn stub_0x1d460() -> usize {
    // IDA 0x1d460: `lblTix` returns the `_lblTix` ivar (0x1d46e).
    // Opaque `id` handle; 0 when unset.
    crate::generated_bg_3::home_outlet("lblTix")
}

// 0x1d470 — -[HomeViewController setLblTix:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setLblTix:]")]
pub fn stub_0x1d470(handle: usize) {
    // IDA 0x1d470: `setLblTix:` retains via `objc_setProperty` (offset
    // 236, 0x1d48c). Retain is drop glue; the handle records.
    crate::generated_bg_3::set_home_outlet("lblTix", handle);
}

// 0x1d494 — -[HomeViewController btnMessages]
// type: UIButton *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController btnMessages]")]
pub fn stub_0x1d494() -> usize {
    // IDA 0x1d494: `btnMessages` returns the `_btnMessages` ivar (0x1d4a2).
    // Opaque `id` handle; 0 when unset.
    crate::generated_bg_3::home_outlet("btnMessages")
}

// 0x1d4a4 — -[HomeViewController setBtnMessages:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setBtnMessages:]")]
pub fn stub_0x1d4a4(handle: usize) {
    // IDA 0x1d4a4: `setBtnMessages:` retains via `objc_setProperty` (offset
    // 240, 0x1d4c0). Retain is drop glue; the handle records.
    crate::generated_bg_3::set_home_outlet("btnMessages", handle);
}

// 0x1d4c8 — -[HomeViewController gameLabel]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController gameLabel]")]
pub fn stub_0x1d4c8() -> usize {
    // IDA 0x1d4c8: `gameLabel` returns the `_gameLabel` ivar (0x1d4d6).
    // Opaque `id` handle; 0 when unset.
    crate::generated_bg_3::home_outlet("gameLabel")
}

// 0x1d4d8 — -[HomeViewController setGameLabel:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setGameLabel:]")]
pub fn stub_0x1d4d8(handle: usize) {
    // IDA 0x1d4d8: `setGameLabel:` retains via `objc_setProperty` (offset
    // 244, 0x1d4f4). Retain is drop glue; the handle records.
    crate::generated_bg_3::set_home_outlet("gameLabel", handle);
}

// 0x1d4fc — -[HomeViewController catalogLabel]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController catalogLabel]")]
pub fn stub_0x1d4fc() -> usize {
    // IDA 0x1d4fc: `catalogLabel` returns the `_catalogLabel` ivar
    // (0x1d50a). Opaque `id` handle; 0 when unset.
    crate::generated_bg_3::home_outlet("catalogLabel")
}

// 0x1d50c — -[HomeViewController setCatalogLabel:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setCatalogLabel:]")]
pub fn stub_0x1d50c(handle: usize) {
    // IDA 0x1d50c: `setCatalogLabel:` retains via `objc_setProperty`
    // (offset 248, 0x1d528). Retain is drop glue; the handle records.
    crate::generated_bg_3::set_home_outlet("catalogLabel", handle);
}

// 0x1d530 — -[HomeViewController inventoryLabel]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController inventoryLabel]")]
pub fn stub_0x1d530() -> usize {
    // IDA 0x1d530: `inventoryLabel` returns the `_inventoryLabel` ivar
    // (0x1d53e). Opaque `id` handle; 0 when unset.
    crate::generated_bg_3::home_outlet("inventoryLabel")
}

// 0x1d540 — -[HomeViewController setInventoryLabel:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setInventoryLabel:]")]
pub fn stub_0x1d540(handle: usize) {
    // IDA 0x1d540: `setInventoryLabel:` retains via `objc_setProperty`
    // (offset 252, 0x1d55c). Retain is drop glue; the handle records.
    crate::generated_bg_3::set_home_outlet("inventoryLabel", handle);
}

// 0x1d564 — -[HomeViewController buildersClubLabel]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController buildersClubLabel]")]
pub fn stub_0x1d564() -> usize {
    // IDA 0x1d564: `buildersClubLabel` returns the `_buildersClubLabel`
    // ivar (0x1d572). Opaque `id` handle; 0 when unset.
    crate::generated_bg_3::home_outlet("buildersClubLabel")
}

// 0x1d574 — -[HomeViewController setBuildersClubLabel:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setBuildersClubLabel:]")]
pub fn stub_0x1d574(handle: usize) {
    // IDA 0x1d574: `setBuildersClubLabel:` retains via `objc_setProperty`
    // (offset 256, 0x1d590). Retain is drop glue; the handle records.
    crate::generated_bg_3::set_home_outlet("buildersClubLabel", handle);
}

// 0x1d598 — -[HomeViewController profileLabel]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController profileLabel]")]
pub fn stub_0x1d598() -> usize {
    // IDA 0x1d598: `profileLabel` returns the `_profileLabel` ivar
    // (0x1d5a6). Opaque `id` handle; 0 when unset.
    crate::generated_bg_3::home_outlet("profileLabel")
}

// 0x1d5a8 — -[HomeViewController setProfileLabel:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setProfileLabel:]")]
pub fn stub_0x1d5a8(handle: usize) {
    // IDA 0x1d5a8: `setProfileLabel:` retains via `objc_setProperty`
    // (offset 260, 0x1d5c4). Retain is drop glue; the handle records.
    crate::generated_bg_3::set_home_outlet("profileLabel", handle);
}

// 0x1d5cc — -[HomeViewController messagesLabel]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController messagesLabel]")]
pub fn stub_0x1d5cc() -> usize {
    // IDA 0x1d5cc: `messagesLabel` returns the `_messagesLabel` ivar
    // (0x1d5da). Opaque `id` handle; 0 when unset.
    crate::generated_bg_3::home_outlet("messagesLabel")
}

// 0x1d5dc — -[HomeViewController setMessagesLabel:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setMessagesLabel:]")]
pub fn stub_0x1d5dc(handle: usize) {
    // IDA 0x1d5dc: `setMessagesLabel:` retains via `objc_setProperty`
    // (offset 264, 0x1d5f8). Retain is drop glue; the handle records.
    crate::generated_bg_3::set_home_outlet("messagesLabel", handle);
}

// 0x1d600 — -[HomeViewController btnPlayDisabled]
// type: UIButton *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController btnPlayDisabled]")]
pub fn stub_0x1d600() -> usize {
    // IDA 0x1d600: `btnPlayDisabled` returns the `_btnPlayDisabled` ivar
    // (0x1d60e). Opaque `id` handle; 0 when unset.
    crate::generated_bg_3::home_outlet("btnPlayDisabled")
}

// 0x1d610 — -[HomeViewController setBtnPlayDisabled:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setBtnPlayDisabled:]")]
pub fn stub_0x1d610(handle: usize) {
    // IDA 0x1d610: `setBtnPlayDisabled:` retains via `objc_setProperty`
    // (offset 268, 0x1d62c). Retain is drop glue; the handle records.
    crate::generated_bg_3::set_home_outlet("btnPlayDisabled", handle);
}

// 0x1d634 — -[HomeViewController communityLabel]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController communityLabel]")]
pub fn stub_0x1d634() -> usize {
    // IDA 0x1d634: `communityLabel` returns the `_communityLabel` ivar
    // (0x1d642). Opaque `id` handle; 0 when unset.
    crate::generated_bg_3::home_outlet("communityLabel")
}

// 0x1d644 — -[HomeViewController setCommunityLabel:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setCommunityLabel:]")]
pub fn stub_0x1d644(handle: usize) {
    // IDA 0x1d644: `setCommunityLabel:` retains via `objc_setProperty`
    // (offset 272, 0x1d660). Retain is drop glue; the handle records.
    crate::generated_bg_3::set_home_outlet("communityLabel", handle);
}

// 0x1d668 — -[HomeViewController communityButton]
// type: UIButton *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController communityButton]")]
pub fn stub_0x1d668() -> usize {
    // IDA 0x1d668: `communityButton` returns the `_communityButton` ivar
    // (0x1d676). Opaque `id` handle; 0 when unset.
    crate::generated_bg_3::home_outlet("communityButton")
}

// 0x1d678 — -[HomeViewController setCommunityButton:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setCommunityButton:]")]
pub fn stub_0x1d678(handle: usize) {
    // IDA 0x1d678: `setCommunityButton:` retains via `objc_setProperty`
    // (offset 276, 0x1d694). Retain is drop glue; the handle records.
    crate::generated_bg_3::set_home_outlet("communityButton", handle);
}

// 0x1d69c — -[HomeViewController buttonView]
// type: UIView *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController buttonView]")]
pub fn stub_0x1d69c() -> usize {
    // IDA 0x1d69c: `buttonView` returns the `_buttonView` ivar (0x1d6aa).
    // Opaque `id` handle; 0 when unset.
    crate::generated_bg_3::home_outlet("buttonView")
}

// 0x1d6ac — -[HomeViewController setButtonView:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setButtonView:]")]
pub fn stub_0x1d6ac(handle: usize) {
    // IDA 0x1d6ac: `setButtonView:` retains via `objc_setProperty` (offset
    // 280, 0x1d6c8). Retain is drop glue; the handle records.
    crate::generated_bg_3::set_home_outlet("buttonView", handle);
}

// 0x1d6d0 — -[HomeViewController searchTextField]
// type: UITextField *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController searchTextField]")]
pub fn stub_0x1d6d0() -> usize {
    // IDA 0x1d6d0: `searchTextField` returns the `_searchTextField` ivar
    // (0x1d6de). Opaque `id` handle; 0 when unset.
    crate::generated_bg_3::home_outlet("searchTextField")
}

// 0x1d6e0 — -[HomeViewController setSearchTextField:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setSearchTextField:]")]
pub fn stub_0x1d6e0(handle: usize) {
    // IDA 0x1d6e0: `setSearchTextField:` retains via `objc_setProperty`
    // (offset 284, 0x1d6fc). Retain is drop glue; the handle records.
    crate::generated_bg_3::set_home_outlet("searchTextField", handle);
}

// 0x1d704 — -[HomeViewController loggedInView]
// type: UIView *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController loggedInView]")]
pub fn stub_0x1d704() -> usize {
    // IDA 0x1d704: `loggedInView` returns the `_loggedInView` ivar
    // (0x1d712). Opaque `id` handle; 0 when unset.
    crate::generated_bg_3::home_outlet("loggedInView")
}

// 0x1d714 — -[HomeViewController setLoggedInView:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setLoggedInView:]")]
pub fn stub_0x1d714(handle: usize) {
    // IDA 0x1d714: `setLoggedInView:` retains via `objc_setProperty`
    // (offset 288, 0x1d730). Retain is drop glue; the handle records.
    crate::generated_bg_3::set_home_outlet("loggedInView", handle);
}

// 0x1d738 — -[HomeViewController notLoggedInView]
// type: UIView *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController notLoggedInView]")]
pub fn stub_0x1d738() -> usize {
    // IDA 0x1d738: `notLoggedInView` returns the `_notLoggedInView` ivar
    // (0x1d746). Opaque `id` handle; 0 when unset.
    crate::generated_bg_3::home_outlet("notLoggedInView")
}

// 0x1d748 — -[HomeViewController setNotLoggedInView:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setNotLoggedInView:]")]
pub fn stub_0x1d748(handle: usize) {
    // IDA 0x1d748: `setNotLoggedInView:` retains via `objc_setProperty`
    // (offset 292, 0x1d764). Retain is drop glue; the handle records.
    crate::generated_bg_3::set_home_outlet("notLoggedInView", handle);
}

// 0x1d76c — -[HomeViewController signUpButtonLabel]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController signUpButtonLabel]")]
pub fn stub_0x1d76c() -> usize {
    // IDA 0x1d76c: `signUpButtonLabel` returns the `_signUpButtonLabel`
    // ivar (0x1d77a). Opaque `id` handle; 0 when unset.
    crate::generated_bg_3::home_outlet("signUpButtonLabel")
}

// 0x1d77c — -[HomeViewController setSignUpButtonLabel:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setSignUpButtonLabel:]")]
pub fn stub_0x1d77c(handle: usize) {
    // IDA 0x1d77c: `setSignUpButtonLabel:` retains via `objc_setProperty`
    // (offset 296, 0x1d798). Retain is drop glue; the handle records.
    crate::generated_bg_3::set_home_outlet("signUpButtonLabel", handle);
}

// 0x1d7a0 — -[HomeViewController loginButtonLabel]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController loginButtonLabel]")]
pub fn stub_0x1d7a0() -> usize {
    // IDA 0x1d7a0: `loginButtonLabel` returns the `_loginButtonLabel` ivar
    // (0x1d7ae). Opaque `id` handle; 0 when unset.
    crate::generated_bg_3::home_outlet("loginButtonLabel")
}

// 0x1d7b0 — -[HomeViewController setLoginButtonLabel:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setLoginButtonLabel:]")]
pub fn stub_0x1d7b0(handle: usize) {
    // IDA 0x1d7b0: `setLoginButtonLabel:` retains via `objc_setProperty`
    // (offset 300, 0x1d7cc). Retain is drop glue; the handle records.
    crate::generated_bg_3::set_home_outlet("loginButtonLabel", handle);
}

// 0x1d7d4 — -[HomeViewController welcomeToRobloxTextView]
// type: UITextView *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController welcomeToRobloxTextView]")]
pub fn stub_0x1d7d4() -> usize {
    // IDA 0x1d7d4: `welcomeToRobloxTextView` returns the
    // `_welcomeToRobloxTextView` ivar (0x1d7e2). Opaque `id` handle; 0 when
    // unset.
    crate::generated_bg_3::home_outlet("welcomeToRobloxTextView")
}

// 0x1d7e4 — -[HomeViewController setWelcomeToRobloxTextView:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setWelcomeToRobloxTextView:]")]
pub fn stub_0x1d7e4(handle: usize) {
    // IDA 0x1d7e4: `setWelcomeToRobloxTextView:` retains via
    // `objc_setProperty` (offset 304, 0x1d800). Retain is drop glue; the
    // handle records.
    crate::generated_bg_3::set_home_outlet("welcomeToRobloxTextView", handle);
}

// 0x1d808 — -[HomeViewController youAreCurrentlyLoggedInAsTextView]
// type: UITextView *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController youAreCurrentlyLoggedInAsTextView]")]
pub fn stub_0x1d808() -> usize {
    // IDA 0x1d808: `youAreCurrentlyLoggedInAsTextView` returns the
    // `_youAreCurrentlyLoggedInAsTextView` ivar (0x1d816). Opaque `id`
    // handle; 0 when unset.
    crate::generated_bg_3::home_outlet("youAreCurrentlyLoggedInAsTextView")
}

// 0x1d818 — -[HomeViewController setYouAreCurrentlyLoggedInAsTextView:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setYouAreCurrentlyLoggedInAsTextView:]")]
pub fn stub_0x1d818(handle: usize) {
    // IDA 0x1d818: `setYouAreCurrentlyLoggedInAsTextView:` retains via
    // `objc_setProperty` (offset 308, 0x1d834). Retain is drop glue; the
    // handle records in the bg_3 outlet registry.
    crate::generated_bg_3::set_home_outlet("youAreCurrentlyLoggedInAsTextView", handle);
}

// 0x1d83c — -[HomeViewController versionLabel]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController versionLabel]")]
pub fn stub_0x1d83c() -> usize {
    // IDA 0x1d83c: `versionLabel` returns the `_versionLabel` ivar
    // (0x1d84a). Opaque `id` handle; 0 when unset.
    crate::generated_bg_3::home_outlet("versionLabel")
}

// 0x1d84c — -[HomeViewController setVersionLabel:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setVersionLabel:]")]
pub fn stub_0x1d84c(handle: usize) {
    // IDA 0x1d84c: `setVersionLabel:` retains via `objc_setProperty`
    // (offset 312, 0x1d868). Retain is drop glue; the handle records.
    crate::generated_bg_3::set_home_outlet("versionLabel", handle);
}

// 0x1d870 — __GLOBAL__I_a_4
#[doc(alias = "global constructor keyed to_a_4")]
#[doc(alias = "__GLOBAL__I_a_4")]
pub fn stub_0x1d870() {
    // IDA 0x1d870: `__GLOBAL__I_a_4` — stores
    // `boost::system::generic_category()` (x2) / `system_category()`
    // singletons into `__MergedGlobals_38` (disasm 0x1d874-0x1d88e;
    // decompile unavailable, init thunk). Same cutover as stub_0x16e4c; no
    // body.
}

// 0x1da08 — -[NSString stringWithPercentEscape]
// type: NSString *__cdecl(NSString *self, SEL)
#[doc(alias = "-[NSString stringWithPercentEscape]")]
pub fn stub_0x1da08(input: &str) -> String {
    // IDA 0x1da08: `stringWithPercentEscape` mutable-copies self
    // (0x1da1a-0x1da2c), escapes via `CFURLCreateStringByAddingPercentEscapes`
    // (0x1da4a) and autoreleases (0x1da5a). Retain traffic is drop glue;
    // the escape mapping (percent_escape) is the observable.
    percent_escape(input)
}

// 0x1da5c — +[LoginViewController sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[LoginViewController sharedInstance]")]
pub fn stub_0x1da5c() -> usize {
    // IDA 0x1da5c: `+sharedInstance` returns `dword_130C3F0` (0x1da68),
    // published by `viewDidLoad` (stub_0x1e2ec) and cleared by
    // `viewDidUnload` (stub_0x1e8cc). Opaque `id` handle; 0 when unset.
    LOGIN_SHARED_HANDLE.load(std::sync::atomic::Ordering::SeqCst)
}

// 0x1da6c — -[LoginViewController initWithCoder:]
// type: LoginViewController *__cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController initWithCoder:]")]
pub fn stub_0x1da6c() {
    // IDA 0x1da6c: `initWithCoder:` — super
    // `RobloxAnimatingPageViewController initWithCoder:` (0x1da8a-0x1da94,
    // no target here), zeroes `envs` (0x1dac4), and registers the
    // login-failed / login-successful / signup-finished observers
    // (0x1dad0-0x1dbc6). Observer delivery has no target here; the env
    // reset + registration count record.
    LOGIN_ENVS.lock().clear();
    LOGIN_OBSERVERS.fetch_add(3, std::sync::atomic::Ordering::SeqCst);
}

// 0x1dbd4 — -[LoginViewController dealloc]
// type: void __cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController dealloc]")]
pub fn stub_0x1dbd4() {
    // IDA 0x1dbd4: `dealloc` removes the notification observer
    // (0x1dbf4-0x1dc06), releases ~16 retained outlets plus `envs` when set
    // (0x1dc26-0x1dd58), then super dealloc (0x1dd70-0x1dd7a). Release is
    // drop glue; the env/outlet cells clear.
    LOGIN_ENVS.lock().clear();
    LOGIN_OUTLETS.lock().clear();
}

// 0x1dd84 — -[LoginViewController populateEnvironmentPicker]
// type: void __cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController populateEnvironmentPicker]")]
pub fn stub_0x1dd84(is_tablet: bool) {
    // IDA 0x1dd84: rebuilds `envs` (0x1dda8-0x1ddde): `www.` on tablet,
    // `m.` on phone (0x1dde6-0x1de16) prefixes roblox.com + sitetest1-4
    // (0x1de38-0x1dee6); an empty/`m.` prefix (0x1defc-0x1df0c) covers the
    // seven named sitetest3 hosts; gametest5-1 reuse the first prefix
    // (0x1e02a-0x1e0d4). 17 entries in binary order. The device query
    // crosses as a parameter.
    let sub = if is_tablet { "www." } else { "m." };
    let named = if is_tablet { "" } else { "m." };
    let mut envs = LOGIN_ENVS.lock();
    envs.clear();
    for host in [
        "roblox.com/",
        "sitetest1.robloxlabs.com/",
        "sitetest2.robloxlabs.com/",
        "sitetest3.robloxlabs.com/",
        "sitetest4.robloxlabs.com/",
    ] {
        envs.push(format!("http://{sub}{host}"));
    }
    for user in ["allen", "anthony", "guru", "rosemary", "sairam", "shannon", "vlad"] {
        envs.push(format!("http://{named}{user}.sitetest3.robloxlabs.com/"));
    }
    for n in (1..=5).rev() {
        envs.push(format!("http://{sub}gametest{n}.robloxlabs.com/"));
    }
}

// 0x1e0d8 — -[LoginViewController pickerView:didSelectRow:inComponent:]
// type: void __cdecl(LoginViewController *self, SEL, id, int, int)
#[doc(alias = "-[LoginViewController pickerView:didSelectRow:inComponent:]")]
pub fn stub_0x1e0d8(row: usize) {
    // IDA 0x1e0d8: `pickerView:didSelectRow:` pushes `envs[row]` as the
    // base URL (0x1e106-0x1e11a; `objectAtIndex:` raises on an out-of-range
    // row, indexing panics the same way) and bounces memory on main
    // (0x1e138, stub_0x1e13c). The queue hop collapses to the direct call.
    *LOGIN_BASE_URL.lock() = LOGIN_ENVS.lock()[row].clone();
    stub_0x1e13c();
}

// 0x1e13c — ___59-[LoginViewController pickerView:didSelectRow:inComponent:]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___59-[LoginViewController pickerView:didSelectRow:inComponent:]_block_invoke")]
pub fn stub_0x1e13c() {
    // IDA 0x1e13c: the did-select block runs `RobloxMemoryManager
    // startMemoryBouncer` (0x1e158-0x1e16c). The manager has no target
    // here; the start records.
    MEMORY_BOUNCER_STARTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x1e170 — -[LoginViewController numberOfComponentsInPickerView:]
// type: int __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController numberOfComponentsInPickerView:]")]
pub fn stub_0x1e170() -> i32 {
    // IDA 0x1e170: `numberOfComponentsInPickerView:` returns 1 (0x1e172).
    1
}

// 0x1e174 — -[LoginViewController pickerView:numberOfRowsInComponent:]
// type: int __cdecl(LoginViewController *self, SEL, id, int)
#[doc(alias = "-[LoginViewController pickerView:numberOfRowsInComponent:]")]
pub fn stub_0x1e174() -> usize {
    // IDA 0x1e174: `pickerView:numberOfRowsInComponent:` returns the
    // `envs` count. Reads the shared environment table.
    LOGIN_ENVS.lock().len()
}

// 0x1e194 — -[LoginViewController pickerView:titleForRow:forComponent:]
// type: id __cdecl(LoginViewController *self, SEL, id, int, int)
#[doc(alias = "-[LoginViewController pickerView:titleForRow:forComponent:]")]
pub fn stub_0x1e194(row: usize) -> String {
    // IDA 0x1e194: `pickerView:titleForRow:` returns
    // `[envs objectAtIndex:row]`. `NSArray` raises on an out-of-range row;
    // indexing panics the same way.
    LOGIN_ENVS.lock()[row].clone()
}

// 0x1e1b4 — -[LoginViewController viewWillAppear:]
// type: void __cdecl(LoginViewController *self, SEL, char)
#[doc(alias = "-[LoginViewController viewWillAppear:]")]
pub fn stub_0x1e1b4(remember_password: bool, saved_password: &str) {
    // IDA 0x1e1b4: `viewWillAppear:` sets the logo alpha to 1.0
    // (0x1e1ca-0x1e1de), runs `stopShowLoggingIn` on main (0x1e210-0x1e224,
    // stub_0x1e2c4), then fills the password field from the saved password
    // when `LoginManager getRememberPassword` is set, else empty
    // (0x1e240-0x1e2bc). The manager query crosses as a parameter; the
    // queue hop collapses to the direct call.
    LOGIN_LOGO_ALPHA_BITS.store(0x3f800000, std::sync::atomic::Ordering::SeqCst);
    stub_0x1e2c4();
    *LOGIN_PASSWORD_TEXT.lock() = if remember_password {
        saved_password.to_owned()
    } else {
        String::new()
    };
}

// 0x1e2c4 — ___38-[LoginViewController viewWillAppear:]_block_invoke
#[doc(alias = "___38-[LoginViewController viewWillAppear:]_block_invoke")]
pub fn stub_0x1e2c4() {
    // IDA 0x1e2c4: the will-appear block calls `stopShowLoggingIn`
    // (0x1e21c, stub_0x1eeac). The queue hop collapses to the direct
    // call; the spinner-hidden outcome records through it.
    stub_0x1eeac();
}

// 0x1e2d8 — ___copy_helper_block__2
#[doc(alias = "___copy_helper_block__2")]
pub fn stub_0x1e2d8(_dst: usize, _src: usize) {
    // IDA 0x1e2d8: `__copy_helper_block__2` — `_Block_object_assign`
    // retain (same shape as stub_0x18094). No explicit body.
}

// 0x1e2e4 — ___destroy_helper_block__2
#[doc(alias = "___destroy_helper_block__2")]
pub fn stub_0x1e2e4(_block: usize) {
    // IDA 0x1e2e4: `__destroy_helper_block__2` — `_Block_object_dispose`
    // release (same shape as stub_0x180a0). No explicit body.
}

// 0x1e2ec — -[LoginViewController viewDidLoad]
// type: void __cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController viewDidLoad]")]
pub fn stub_0x1e2ec(
    handle: usize,
    os_version: &str,
    app_version: &str,
    device_name: &str,
    user_agent: &str,
    username: Option<&str>,
    remember_password: bool,
    saved_password: Option<&str>,
    bundle_version: &str,
) {
    // IDA 0x1e2ec: `viewDidLoad` — super (0x1e30c-0x1e318, no target here);
    // publishes self as the shared instance (0x1e33e, stub_0x1da5c);
    // analytics vars iOSVersion/appVersion/deviceType (0x1e362-0x1e3d6);
    // localized placeholders + labels (0x1e406-0x1e5b6); version label from
    // `CFBundleVersion` (0x1e5cc-0x1e606); the UserAgent defaults register
    // twice (0x1e62e-0x1e686, 0x1e734-0x1e73c — the second is a no-op
    // duplicate); skip + picker hidden (0x1e6aa-0x1e6c2); username field
    // when set (0x1e6e2-0x1e722); remember switch (0x1e764-0x1e78c);
    // password field when set and the switch is on (0x1e79e-0x1e7ea);
    // keyboard observers (0x1e808-0x1e870); memory-bouncer block on main
    // (0x1e88a, stub_0x1e898). Device/defaults queries collapse into
    // parameters; the queue hop collapses to the direct call.
    LOGIN_SHARED_HANDLE.store(handle, std::sync::atomic::Ordering::SeqCst);
    *LOGIN_ANALYTICS_VARS.lock() = vec![
        ("iOSVersion".to_owned(), os_version.to_owned()),
        ("appVersion".to_owned(), app_version.to_owned()),
        ("deviceType".to_owned(), device_name.to_owned()),
    ];
    let mut texts = LOGIN_LABEL_TEXTS.lock();
    for (slot, key) in [
        ("usernamePlaceholder", "UsernameWord"),
        ("passwordPlaceholder", "PasswordWord"),
        ("rememberPwLabel", "RememberPassword"),
        ("loginLabel", "LoginWord"),
        ("signupLabel", "SignupWord"),
        ("playNowLabel", "PlayNowButtonLabel"),
    ] {
        texts.insert(slot.to_owned(), key.to_owned());
    }
    texts.insert("versionLabel".to_owned(), bundle_version.to_owned());
    drop(texts);
    *LOGIN_USER_AGENT.lock() = user_agent.to_owned();
    LOGIN_SKIP_HIDDEN.store(true, std::sync::atomic::Ordering::SeqCst);
    LOGIN_PICKER_HIDDEN.store(true, std::sync::atomic::Ordering::SeqCst);
    if let Some(name) = username {
        *LOGIN_USERNAME_TEXT.lock() = name.to_owned();
    }
    LOGIN_REMEMBER_ON.store(remember_password, std::sync::atomic::Ordering::SeqCst);
    if remember_password {
        if let Some(password) = saved_password {
            *LOGIN_PASSWORD_TEXT.lock() = password.to_owned();
        }
    }
    LOGIN_OBSERVERS.fetch_add(2, std::sync::atomic::Ordering::SeqCst);
    stub_0x1e898();
}

// 0x1e898 — ___34-[LoginViewController viewDidLoad]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___34-[LoginViewController viewDidLoad]_block_invoke")]
pub fn stub_0x1e898() {
    // IDA 0x1e898: the did-load block runs `RobloxMemoryManager
    // startMemoryBouncer` (0x1e8b4-0x1e8c8, same shape as stub_0x1e13c).
    // Sequences the call.
    stub_0x1e13c();
}

// 0x1e8cc — -[LoginViewController viewDidUnload]
// type: void __cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController viewDidUnload]")]
pub fn stub_0x1e8cc() {
    // IDA 0x1e8cc: `viewDidUnload` nils 10 login outlets (0x1e8e6-0x1e99a),
    // super `viewDidUnload` (0x1e9b2-0x1e9bc), then clears the shared
    // instance (0x1e9ca, stub_0x1da5c). Outlet release is drop glue; the
    // registry + shared cell clear.
    LOGIN_OUTLETS.lock().clear();
    LOGIN_SHARED_HANDLE.store(0, std::sync::atomic::Ordering::SeqCst);
}

// 0x1e9d0 — -[LoginViewController handleSignupNotification:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController handleSignupNotification:]")]
pub fn stub_0x1e9d0(username: Option<&str>, password: Option<&str>) {
    // IDA 0x1e9d0: `handleSignupNotification:` pulls `username`/`password`
    // from the notification `userInfo` (0x1e9ee-0x1ea28), retains both
    // (0x1ea3a-0x1ea42, drop glue), and only with both non-nil
    // (0x1ea46-0x1ea4e) dispatches the fill-in block on main
    // (0x1ea7e-0x1ea92, stub_0x1eaa0). The queue hop collapses to the
    // direct call.
    if let (Some(username), Some(password)) = (username, password) {
        *PENDING_LOGIN_SIGNUP.lock() = (username.to_owned(), password.to_owned());
        LOGIN_SIGNUP_DISPATCHES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        stub_0x1eaa0(username, password);
    }
}

// 0x1eaa0 — ___48-[LoginViewController handleSignupNotification:]_block_invoke
#[doc(alias = "___48-[LoginViewController handleSignupNotification:]_block_invoke")]
pub fn stub_0x1eaa0(username: &str, password: &str) {
    // IDA 0x1eaa0: the signup block fills the username/password fields
    // (0x1eab4-0x1eae2) and releases the retained pair (0x1eaf6-0x1eb02,
    // drop glue).
    *LOGIN_USERNAME_TEXT.lock() = username.to_owned();
    *LOGIN_PASSWORD_TEXT.lock() = password.to_owned();
}

// 0x1eb08 — ___copy_helper_block_226
// type: void __fastcall(int, const void **)
#[doc(alias = "___copy_helper_block_226")]
pub fn stub_0x1eb08(_dst: usize, _src: usize) {
    // IDA 0x1eb08: `__copy_helper_block_226` — three `_Block_object_assign`
    // retains (0x1eb18-0x1eb34, same shape as stub_0x18094). No explicit
    // body.
}

// 0x1eb38 — ___destroy_helper_block_227
#[doc(alias = "___destroy_helper_block_227")]
pub fn stub_0x1eb38(_block: usize) {
    // IDA 0x1eb38: `__destroy_helper_block_227` — three
    // `_Block_object_dispose` releases (0x1eb42-0x1eb56, same shape as
    // stub_0x180a0). No explicit body.
}

// 0x1eb5c — -[LoginViewController gotLoginFailedNotification:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController gotLoginFailedNotification:]")]
pub fn stub_0x1eb5c(error: &str) {
    // IDA 0x1eb5c: `gotLoginFailedNotification:` pulls the `Error` value
    // from the notification `userInfo` (0x1eb72-0x1ebd0) and dispatches the
    // failure block on main (0x1ebc0-0x1ebd4, stub_0x1ebdc). The queue hop
    // collapses to the direct call.
    stub_0x1ebdc(error);
}

// 0x1ebdc — ___50-[LoginViewController gotLoginFailedNotification:]_block_invoke
#[doc(alias = "___50-[LoginViewController gotLoginFailedNotification:]_block_invoke")]
pub fn stub_0x1ebdc(error: &str) {
    // IDA 0x1ebdc: the failure block stops the spinner (0x1ebf0), shows a
    // `RobloxAlert` with the error (0x1ec0e), and clears the password field
    // (0x1ec20-0x1ec2c).
    LOGIN_SHOWING.store(false, std::sync::atomic::Ordering::SeqCst);
    *LAST_LOGIN_ALERT.lock() = error.to_owned();
    LOGIN_PASSWORD_TEXT.lock().clear();
}

// 0x1ec44 — ___copy_helper_block_234
#[doc(alias = "___copy_helper_block_234")]
pub fn stub_0x1ec44(_dst: usize, _src: usize) {
    // IDA 0x1ec44: `__copy_helper_block_234` — two `_Block_object_assign`
    // retains (0x1ec54-0x1ec64, same shape as stub_0x18094). No explicit
    // body.
}

// 0x1ec68 — ___destroy_helper_block_235
#[doc(alias = "___destroy_helper_block_235")]
pub fn stub_0x1ec68(_block: usize) {
    // IDA 0x1ec68: `__destroy_helper_block_235` — two
    // `_Block_object_dispose` releases (0x1ec72-0x1ec7e, same shape as
    // stub_0x180a0). No explicit body.
}

// 0x1ec84 — -[LoginViewController gotLoginSuccessfulNotification:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController gotLoginSuccessfulNotification:]")]
pub fn stub_0x1ec84(remember_password: bool, username: &str, password: &str) {
    // IDA 0x1ec84: `gotLoginSuccessfulNotification:` warms the store
    // manager (0x1eca4, no target here), runs `doLoginTransition`
    // (0x1ecb6, stub_0x1f6b0), and dispatches the completion block on
    // main (0x1ece8-0x1ecfc, stub_0x1ed04). The manager/player queries
    // behind the transition cross as parameters; both hops collapse to
    // direct calls.
    crate::generated_bg_5::stub_0x1f6b0(remember_password, username, password);
    stub_0x1ed04();
}

// 0x1ed04 — ___54-[LoginViewController gotLoginSuccessfulNotification:]_block_invoke
#[doc(alias = "___54-[LoginViewController gotLoginSuccessfulNotification:]_block_invoke")]
pub fn stub_0x1ed04() {
    // IDA 0x1ed04: the success block clears the password field via
    // `setText:` with the empty `CFString` (0x1ed04-0x1ed2a, stru_12CB0D8
    // is the null constant). The ivar load is drop glue; the clear
    // records.
    LOGIN_PASSWORD_TEXT.lock().clear();
}

// 0x1ed30 — ___copy_helper_block_242
#[doc(alias = "___copy_helper_block_242")]
pub fn stub_0x1ed30(_dst: usize, _src: usize) {
    // IDA 0x1ed30: `__copy_helper_block_242` — one `_Block_object_assign`
    // retain (0x1ed30-0x1ed36, same shape as stub_0x18094). No explicit
    // body.
}

// 0x1ed3c — ___destroy_helper_block_243
#[doc(alias = "___destroy_helper_block_243")]
pub fn stub_0x1ed3c(_block: usize) {
    // IDA 0x1ed3c: `__destroy_helper_block_243` — one
    // `_Block_object_dispose` release (0x1ed3c-0x1ed40, same shape as
    // stub_0x180a0). No explicit body.
}

// 0x1ed44 — -[LoginViewController showLoggingIn]
// type: void __cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController showLoggingIn]")]
pub fn stub_0x1ed44() {
    // IDA 0x1ed44: `showLoggingIn` hides the about button (0x1ed4c-0x1ed6c)
    // and dispatches the spinner block on main (0x1ed70-0x1edb2,
    // stub_0x1edbc). The spinner shows; the queue hop collapses to the
    // direct call.
    LOGIN_ABOUT_HIDDEN.store(true, std::sync::atomic::Ordering::SeqCst);
    LOGIN_SHOWING.store(true, std::sync::atomic::Ordering::SeqCst);
    stub_0x1edbc();
}

// 0x1edbc — ___36-[LoginViewController showLoggingIn]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___36-[LoginViewController showLoggingIn]_block_invoke")]
pub fn stub_0x1edbc() {
    // IDA 0x1edbc: the spinner block unhides the activity indicator
    // (0x1edc2-0x1ede6) and runs the 0.5s fade-out animation with no
    // delay/options/completion (0x1edea-0x1ee4e, stub_0x1ee58). The
    // animation hop collapses to the direct call.
    LOGIN_INDICATOR_HIDDEN.store(false, std::sync::atomic::Ordering::SeqCst);
    stub_0x1ee58();
}

// 0x1ee58 — ___36-[LoginViewController showLoggingIn]_block_invoke_2
// type: id __fastcall(int)
#[doc(alias = "___36-[LoginViewController showLoggingIn]_block_invoke_2")]
pub fn stub_0x1ee58() {
    // IDA 0x1ee58: the fade-out animation sets the login field views
    // alpha to 0 (0x1ee58-0x1ee80).
    LOGIN_FIELD_ALPHA_BITS.store(0, std::sync::atomic::Ordering::SeqCst);
}

// 0x1ee84 — ___copy_helper_block_252
#[doc(alias = "___copy_helper_block_252")]
pub fn stub_0x1ee84(_dst: usize, _src: usize) {
    // IDA 0x1ee84: `__copy_helper_block_252` — one `_Block_object_assign`
    // retain (0x1ee84-0x1ee8a, same shape as stub_0x18094). No explicit
    // body.
}

// 0x1ee90 — ___destroy_helper_block_253
#[doc(alias = "___destroy_helper_block_253")]
pub fn stub_0x1ee90(_block: usize) {
    // IDA 0x1ee90: `__destroy_helper_block_253` — one
    // `_Block_object_dispose` release (0x1ee90-0x1ee94, same shape as
    // stub_0x180a0). No explicit body.
}

// 0x1ee98 — ___copy_helper_block_257
#[doc(alias = "___copy_helper_block_257")]
pub fn stub_0x1ee98(_dst: usize, _src: usize) {
    // IDA 0x1ee98: `__copy_helper_block_257` — one `_Block_object_assign`
    // retain (0x1ee98-0x1ee9e, same shape as stub_0x18094). No explicit
    // body.
}

// 0x1eea4 — ___destroy_helper_block_258
#[doc(alias = "___destroy_helper_block_258")]
pub fn stub_0x1eea4(_block: usize) {
    // IDA 0x1eea4: `__destroy_helper_block_258` — one
    // `_Block_object_dispose` release (0x1eea4-0x1eea8, same shape as
    // stub_0x180a0). No explicit body.
}

// 0x1eeac — -[LoginViewController stopShowLoggingIn]
// type: void __cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController stopShowLoggingIn]")]
pub fn stub_0x1eeac() {
    // IDA 0x1eeac: `stopShowLoggingIn` dispatches the restore block on
    // main (0x1eeb2-0x1eef4, stub_0x1eefc). The spinner hides; the queue
    // hop collapses to the direct call.
    LOGIN_SHOWING.store(false, std::sync::atomic::Ordering::SeqCst);
    stub_0x1eefc();
}

// 0x1eefc — ___40-[LoginViewController stopShowLoggingIn]_block_invoke
#[doc(alias = "___40-[LoginViewController stopShowLoggingIn]_block_invoke")]
pub fn stub_0x1eefc() {
    // IDA 0x1eefc: the restore block unhides the about button
    // (0x1ef02-0x1ef28), hides the activity indicator (0x1ef2c-0x1ef42),
    // and runs the 0.5s fade-in animation with no delay/options/completion
    // (0x1ef46-0x1efa4, stub_0x1efac). The animation hop collapses to the
    // direct call.
    LOGIN_ABOUT_HIDDEN.store(false, std::sync::atomic::Ordering::SeqCst);
    LOGIN_INDICATOR_HIDDEN.store(true, std::sync::atomic::Ordering::SeqCst);
    stub_0x1efac();
}

// 0x1efac — ___40-[LoginViewController stopShowLoggingIn]_block_invoke_2
#[doc(alias = "___40-[LoginViewController stopShowLoggingIn]_block_invoke_2")]
pub fn stub_0x1efac() {
    // IDA 0x1efac: the fade-in animation restores the login field views
    // alpha to 1.0 (0x1efac-0x1efd6, 0x3f800000).
    LOGIN_FIELD_ALPHA_BITS.store(0x3f800000, std::sync::atomic::Ordering::SeqCst);
}

// 0x1efdc — ___copy_helper_block_260
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_260")]
pub fn stub_0x1efdc(_dst: usize, _src: usize) {
    // IDA 0x1efdc: `__copy_helper_block_260` — one `_Block_object_assign`
    // retain (0x1efdc-0x1efe2, same shape as stub_0x18094). No explicit
    // body.
}

// 0x1efe8 — ___destroy_helper_block_261
#[doc(alias = "___destroy_helper_block_261")]
pub fn stub_0x1efe8(_block: usize) {
    // IDA 0x1efe8: `__destroy_helper_block_261` — one
    // `_Block_object_dispose` release (0x1efe8-0x1efec, same shape as
    // stub_0x180a0). No explicit body.
}

// 0x1eff0 — ___copy_helper_block_263
#[doc(alias = "___copy_helper_block_263")]
pub fn stub_0x1eff0(_dst: usize, _src: usize) {
    // IDA 0x1eff0: `__copy_helper_block_263` — one `_Block_object_assign`
    // retain (0x1eff0-0x1eff6, same shape as stub_0x18094). No explicit
    // body.
}

// 0x1effc — ___destroy_helper_block_264
#[doc(alias = "___destroy_helper_block_264")]
pub fn stub_0x1effc(_block: usize) {
    // IDA 0x1effc: `__destroy_helper_block_264` — one
    // `_Block_object_dispose` release (0x1effc-0x1f000, same shape as
    // stub_0x180a0). No explicit body.
}

// 0x1f004 — -[LoginViewController playNowDidTouchUpInside:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController playNowDidTouchUpInside:]")]
pub fn stub_0x1f004() {
    // IDA 0x1f004: `playNowDidTouchUpInside:` flags the guest tap
    // (`_userDidClickPlayNow = 1`, 0x1f008-0x1f024) and, with a non-empty
    // password field (0x1f028-0x1f04e), falls through to `login:`
    // (0x1f050-0x1f064, stub_0x1f0d4); with an empty one it logs out
    // (0x1f068-0x1f090), tracks the `Login/GuestMode` page view
    // (0x1f094-0x1f0b6), and segues home animated (0x1f0ba-0x1f0ce,
    // stub_0x1f854). The manager/analytics hops after the branch record;
    // both selector hops collapse to direct calls.
    USER_DID_CLICK_PLAY_NOW.store(true, std::sync::atomic::Ordering::SeqCst);
    if !LOGIN_PASSWORD_TEXT.lock().is_empty() {
        stub_0x1f0d4();
    } else {
        LOGIN_LOGOUTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        *LAST_PAGE_TRACKING.lock() = "Login/GuestMode".to_owned();
        crate::generated_bg_5::stub_0x1f854(true);
    }
}

// 0x1f0d4 — -[LoginViewController login:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController login:]")]
pub fn stub_0x1f0d4() {
    // IDA 0x1f0d4: `login:` ends editing on both fields (0x1f0de-0x1f122,
    // keyboard dismissal, no target here), shows the spinner
    // (0x1f126-0x1f134, stub_0x1ed44), then delivers the field texts to
    // `LoginManager doLoginWithUsername:password:` (0x1f138-0x1f192,
    // record_login_attempt).
    stub_0x1ed44();
    let username = LOGIN_USERNAME_TEXT.lock().clone();
    let password = LOGIN_PASSWORD_TEXT.lock().clone();
    record_login_attempt(&username, &password);
}

// 0x1f1a0 — -[LoginViewController usernameDidEndOnExit:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController usernameDidEndOnExit:]")]
pub fn stub_0x1f1a0() {
    // IDA 0x1f1a0: `usernameDidEndOnExit:` moves focus to the password
    // field (0x1f1a0-0x1f1c4).
    *LOGIN_FIRST_RESPONDER.lock() = "password".to_owned();
}

// 0x1f1c8 — -[LoginViewController passwordDidEndOnExit:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController passwordDidEndOnExit:]")]
pub fn stub_0x1f1c8() {
    // IDA 0x1f1c8: `passwordDidEndOnExit:` shows the spinner
    // (0x1f1d2-0x1f1e0, stub_0x1ed44) and delivers the field texts to
    // `LoginManager doLoginWithUsername:password:` (0x1f1e4-0x1f25a,
    // record_login_attempt) — the same tail as `login:` (stub_0x1f0d4)
    // without the end-editing prologue.
    stub_0x1ed44();
    let username = LOGIN_USERNAME_TEXT.lock().clone();
    let password = LOGIN_PASSWORD_TEXT.lock().clone();
    record_login_attempt(&username, &password);
}

// 0x1f260 — -[LoginViewController swiToggleRememberMyPassword:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController swiToggleRememberMyPassword:]")]
pub fn stub_0x1f260(is_on: bool) {
    // IDA 0x1f260: `swiToggleRememberMyPassword:` forwards the switch
    // `isOn` to `LoginManager setRememberPassword:` (0x1f260-0x1f2ba).
    // The manager has no target here; the switch query crosses as a
    // parameter and the value records. Matches the `is_tablet` shape of
    // stub_0x1dd84.
    LOGIN_REMEMBER_ON.store(is_on, std::sync::atomic::Ordering::SeqCst);
}

// 0x1f2c0 — -[LoginViewController loginButtonDidTouchUpInside:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController loginButtonDidTouchUpInside:]")]
pub fn stub_0x1f2c0() {
    // IDA 0x1f2c0: `loginButtonDidTouchUpInside:` clears the guest-tap
    // flag (`_userDidClickPlayNow = 0`, 0x1f2c0-0x1f2d6) and falls through
    // to `login:` (0x1f2da-0x1f2dc, stub_0x1f0d4).
    USER_DID_CLICK_PLAY_NOW.store(false, std::sync::atomic::Ordering::SeqCst);
    stub_0x1f0d4();
}

// 0x1f2e0 — -[LoginViewController onKeyboardHide:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController onKeyboardHide:]")]
pub fn stub_0x1f2e0(has_received_memory_warning: bool) {
    // IDA 0x1f2e0: `onKeyboardHide:` resets the scroll offset to (0, 0)
    // animated (0x1f2e8-0x1f30c) and, unless the controller has received a
    // memory warning (0x1f310-0x1f320), restarts the background pan
    // (0x1f322-0x1f330) and dispatches the hide block on main
    // (0x1f334-0x1f376, stub_0x1f380). The warning flag crosses as a
    // parameter; the queue hop collapses to the direct call.
    *LOGIN_SCROLL_OFFSET.lock() = (0.0, 0.0);
    LOGIN_KEYBOARD_HIDES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    if !has_received_memory_warning {
        LOGIN_BACKGROUND_PANS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        crate::generated_bg_5::stub_0x1f380();
    }
}
