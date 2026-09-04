// Auto-generated skeletons for rbx-script — script-bg filler EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield|lua (5401 filtered, all already stubbed) — script-bg filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x1d3f8..0x1f494 | script 15763 -> 15863 total (filler 0x1d3f8 asc, not-in-script 69782->69682)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use crate::generated_165::HomeViewState;
use crate::generated_165::{BlockCapture, DisplayPickerCaptures};
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::LazyLock;

/// Host-side `LoginViewController` state (LoginViewController.m, IDA
/// 0x1da6c..0x1eaa0). UIKit views live on the platform side; only the
/// observable latches are modeled here.
#[derive(Debug, Clone, Default)]
pub struct LoginViewState {
    /// The three notification observers installed by `initWithCoder:`
    /// (IDA 0x1dad0..0x1dbc6).
    pub login_failed_observer: bool,
    pub login_success_observer: bool,
    pub signup_observer: bool,
    /// `envs` environment URL list (IDA 0x1dd84..0x1e0d4).
    pub envs: Vec<String>,
    /// `+[RobloxInfo baseUrl]` selection (IDA 0x1e106..0x1e11a).
    pub base_url: Option<String>,
    /// Memory-bouncer start requested (IDA 0x1e13c/0x1e898).
    pub memory_bouncer_started: bool,
    /// `robloxLogo.alpha = 1.0` reset (IDA 0x1e1ca..0x1e1de).
    pub logo_alpha_reset: bool,
    /// `stopShowLoggingIn` ran (IDA 0x1e2c4).
    pub logging_in_hidden: bool,
    /// Prefilled field text (IDA 0x1e2a2/0x1e722/0x1e7ea/0x1eaa0).
    pub username_text: String,
    pub password_text: String,
    /// Field placeholders (IDA 0x1e44c/0x1e49c).
    pub placeholders: HashMap<String, String>,
    /// Localized labels (IDA 0x1e4e6..0x1e5b6).
    pub labels: HashMap<String, String>,
    /// `CFBundleVersion` stamp (IDA 0x1e5cc..0x1e606).
    pub version_text: Option<String>,
    /// Analytics `setCustomVariableWithLabel:` pairs (IDA 0x1e362..0x1e3d6).
    pub custom_vars: HashMap<String, String>,
    /// `UserAgent` defaults registered (IDA 0x1e62e..0x1e73c).
    pub user_agent_registered: bool,
    /// `btnSkip`/`EnvironmentPicker` hidden (IDA 0x1e6aa/0x1e6c2).
    pub skip_hidden: bool,
    pub env_picker_hidden: bool,
    /// `swiRememberMyPassword.on` (IDA 0x1e764..0x1e78c).
    pub remember_switch_on: bool,
    /// Keyboard observers installed (IDA 0x1e808..0x1e870).
    pub keyboard_observers: bool,
    /// `viewDidLoad` published the singleton (IDA 0x1e33e).
    pub singleton_set: bool,
    /// `viewDidUnload` cleared the singleton (IDA 0x1e9ca).
    pub singleton_cleared: bool,
    /// Remaining outlet handles by ivar name.
    pub outlets: HashMap<String, Option<u32>>,
    pub view_loaded: bool,
    /// Password field took focus (IDA 0x1f1b0..0x1f1c4).
    pub password_focused: bool,
    /// `setRememberPassword:` value (IDA 0x1f282..0x1f2ba).
    pub remember_set: Option<bool>,
    /// `userDidClickPlayNow` global (IDA 0x1f024/0x1f2d6/0x1f7fc).
    pub play_now_flag: bool,
    /// `scrollView.contentOffset` (IDA 0x1f30c/0x1f4d8).
    pub scroll_offset: Option<(f32, f32)>,
    /// Background pan running (IDA 0x1f330/0x1f674).
    pub background_pan_running: bool,
    /// Background/foreground images dimmed to alpha 0 (IDA 0x1f5f4..0x1f63c).
    pub bg_images_dimmed: bool,
    /// Stored `username`/`password` defaults (IDA 0x1f752..0x1f7dc).
    pub stored_username: Option<String>,
    pub stored_password: Option<String>,
    /// `gotLoginFailedNotification:` error text (IDA 0x1ebd0..0x1ec0e).
    pub login_error: Option<String>,
    /// `getStoreMgr` polled (IDA 0x1eca4).
    pub store_mgr_polled: bool,
    /// `doLoginTransition` requested (IDA 0x1ecb6).
    pub login_transition_requested: bool,
    /// `showLoggingIn` ran (IDA 0x1ed44).
    pub logging_in_shown: bool,
    /// `aboutButton` hidden by `showLoggingIn` (IDA 0x1ed5a..0x1ed6c).
    pub about_hidden: bool,
    /// `loginActivityIndicator` shown (IDA 0x1edd2..0x1ede6).
    pub activity_shown: bool,
    /// `loginFieldViews.alpha = 0` animation end (IDA 0x1ee6a).
    pub fields_alpha_zero: bool,
    /// `playNowDidTouchUpInside:` ran (IDA 0x1f004).
    pub play_now_clicked: bool,
    /// Guest-mode logout path taken (IDA 0x1f080..0x1f090).
    pub guest_mode: bool,
    /// `setPageViewTracking:` page for guest mode (IDA 0x1f0b6).
    pub guest_page_view: Option<String>,
    /// `segueToHomeViewController:` animated flag (IDA 0x1f064).
    pub home_segue_animated: Option<bool>,
    /// `login:` credentials (IDA 0x1f154..0x1f19a).
    pub login_attempt: Option<(String, String)>,
    /// Fields sent `endEditing:` (IDA 0x1f104/0x1f122).
    pub fields_editing_ended: bool,
}

/// `gotLoginFailedNotification:` block captures (`self`, error string;
/// IDA 0x1ebc0..0x1ebd0, helpers 0x1ec44..0x1ec7e).
#[derive(Debug, Clone, Default)]
pub struct LoginErrorCaptures {
    pub owner: Option<u32>,
    pub error: Option<String>,
}

/// `handleSignupNotification:` block captures (`self`, username, password;
/// IDA 0x1ea7e..0x1ea90, helpers 0x1eb08..0x1eb34).
#[derive(Debug, Clone, Default)]
pub struct SignupCaptures {
    pub owner: Option<u32>,
    pub username: Option<String>,
    pub password: Option<String>,
}

/// `+[LoginViewController sharedInstance]` slot (IDA 0x1da5c..0x1da68,
/// `dword_130C3F0`; set by `viewDidLoad` at 0x1e33e, cleared by
/// `viewDidUnload` at 0x1e9ca).
static LOGIN_SHARED: LazyLock<Mutex<LoginViewState>> =
    LazyLock::new(|| Mutex::new(LoginViewState::default()));

// 0x1d3f8 — -[HomeViewController btnDebugSettings]
// type: UIButton *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController btnDebugSettings]")]
pub fn stub_0x1d3f8(state: &HomeViewState) -> Option<u32> {
    // IDA 0x1d3f8 `-[HomeViewController btnDebugSettings]`: ivar load
    // (disasm `_btnDebugSettings` LDR); opaque platform handle on the host
    // (cf. 0x1d390).
    state.outlet("btnDebugSettings")
}

// 0x1d408 — -[HomeViewController setBtnDebugSettings:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setBtnDebugSettings:]")]
pub fn stub_0x1d408(state: &mut HomeViewState, view: Option<u32>) {
    // IDA 0x1d408 `-[HomeViewController setBtnDebugSettings:]`:
    // `objc_setProperty` retain/setter (disasm prologue); host ownership
    // is the outlet slot (cf. 0x1d3a0).
    state.set_outlet("btnDebugSettings", view);
}

// 0x1d42c — -[HomeViewController lblRobux]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController lblRobux]")]
pub fn stub_0x1d42c(state: &HomeViewState) -> Option<u32> {
    // IDA 0x1d42c `-[HomeViewController lblRobux]`: ivar load (disasm
    // `_lblRobux` LDR); the handle — the text lives in
    // [`crate::generated_165::UserDisplay::robux_text`] (cf. 0x1bf0c).
    state.outlet("lblRobux")
}

// 0x1d43c — -[HomeViewController setLblRobux:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setLblRobux:]")]
pub fn stub_0x1d43c(state: &mut HomeViewState, view: Option<u32>) {
    // IDA 0x1d43c `-[HomeViewController setLblRobux:]`:
    // `objc_setProperty` retain/setter (disasm prologue); host ownership
    // is the outlet slot.
    state.set_outlet("lblRobux", view);
}

// 0x1d460 — -[HomeViewController lblTix]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController lblTix]")]
pub fn stub_0x1d460(state: &HomeViewState) -> Option<u32> {
    // IDA 0x1d460 `-[HomeViewController lblTix]`: ivar load (disasm
    // `_lblTix` LDR); the handle — the text lives in
    // [`crate::generated_165::UserDisplay::tix_text`] (cf. 0x1bf0c).
    state.outlet("lblTix")
}

// 0x1d470 — -[HomeViewController setLblTix:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setLblTix:]")]
pub fn stub_0x1d470(state: &mut HomeViewState, view: Option<u32>) {
    // IDA 0x1d470 `-[HomeViewController setLblTix:]`:
    // `objc_setProperty` retain/setter (disasm prologue); host ownership
    // is the outlet slot.
    state.set_outlet("lblTix", view);
}

// 0x1d494 — -[HomeViewController btnMessages]
// type: UIButton *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController btnMessages]")]
pub fn stub_0x1d494(state: &HomeViewState) -> Option<u32> {
    // IDA 0x1d494 `-[HomeViewController btnMessages]`: ivar load (disasm
    // `_btnMessages` LDR); opaque platform handle on the host.
    state.outlet("btnMessages")
}

// 0x1d4a4 — -[HomeViewController setBtnMessages:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setBtnMessages:]")]
pub fn stub_0x1d4a4(state: &mut HomeViewState, view: Option<u32>) {
    // IDA 0x1d4a4 `-[HomeViewController setBtnMessages:]`:
    // `objc_setProperty` retain/setter (disasm prologue); host ownership
    // is the outlet slot.
    state.set_outlet("btnMessages", view);
}

// 0x1d4c8 — -[HomeViewController gameLabel]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController gameLabel]")]
pub fn stub_0x1d4c8(state: &HomeViewState) -> Option<u32> {
    // IDA 0x1d4c8 `-[HomeViewController gameLabel]`: ivar load (disasm
    // `_gameLabel` LDR); the handle — the text lives in the `labels`
    // table (cf. 0x1bc10).
    state.outlet("gameLabel")
}

// 0x1d4d8 — -[HomeViewController setGameLabel:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setGameLabel:]")]
pub fn stub_0x1d4d8(state: &mut HomeViewState, view: Option<u32>) {
    // IDA 0x1d4d8 `-[HomeViewController setGameLabel:]`:
    // `objc_setProperty` retain/setter (disasm prologue); host ownership
    // is the outlet slot.
    state.set_outlet("gameLabel", view);
}

// 0x1d4fc — -[HomeViewController catalogLabel]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController catalogLabel]")]
pub fn stub_0x1d4fc(state: &HomeViewState) -> Option<u32> {
    // IDA 0x1d4fc `-[HomeViewController catalogLabel]`: ivar load (disasm
    // `_catalogLabel` LDR); the handle — the text lives in the `labels`
    // table (cf. 0x1bc10).
    state.outlet("catalogLabel")
}

// 0x1d50c — -[HomeViewController setCatalogLabel:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setCatalogLabel:]")]
pub fn stub_0x1d50c(state: &mut HomeViewState, view: Option<u32>) {
    // IDA 0x1d50c `-[HomeViewController setCatalogLabel:]`:
    // `objc_setProperty` retain/setter (disasm prologue); host ownership
    // is the outlet slot.
    state.set_outlet("catalogLabel", view);
}

// 0x1d530 — -[HomeViewController inventoryLabel]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController inventoryLabel]")]
pub fn stub_0x1d530(state: &HomeViewState) -> Option<u32> {
    // IDA 0x1d530 `-[HomeViewController inventoryLabel]`: ivar load
    // (disasm `_inventoryLabel` LDR); the handle — the text lives in the
    // `labels` table (cf. 0x1bc10).
    state.outlet("inventoryLabel")
}

// 0x1d540 — -[HomeViewController setInventoryLabel:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setInventoryLabel:]")]
pub fn stub_0x1d540(state: &mut HomeViewState, view: Option<u32>) {
    // IDA 0x1d540 `-[HomeViewController setInventoryLabel:]`:
    // `objc_setProperty` retain/setter (disasm prologue); host ownership
    // is the outlet slot.
    state.set_outlet("inventoryLabel", view);
}

// 0x1d564 — -[HomeViewController buildersClubLabel]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController buildersClubLabel]")]
pub fn stub_0x1d564(state: &HomeViewState) -> Option<u32> {
    // IDA 0x1d564 `-[HomeViewController buildersClubLabel]`: ivar load
    // (disasm `_buildersClubLabel` LDR); the handle — the text lives in
    // the `labels` table (cf. 0x1bc10).
    state.outlet("buildersClubLabel")
}

// 0x1d574 — -[HomeViewController setBuildersClubLabel:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setBuildersClubLabel:]")]
pub fn stub_0x1d574(state: &mut HomeViewState, view: Option<u32>) {
    // IDA 0x1d574 `-[HomeViewController setBuildersClubLabel:]`: SET
    // (disasm `objc_setProperty` prologue); host ownership is the outlet
    // slot (cf. 0x1d540).
    state.set_outlet("buildersClubLabel", view);
}

// 0x1d598 — -[HomeViewController profileLabel]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController profileLabel]")]
pub fn stub_0x1d598(state: &HomeViewState) -> Option<u32> {
    // IDA 0x1d598 `-[HomeViewController profileLabel]`: GET (disasm
    // `_profileLabel` IVAR load); the handle — the text lives in the
    // `labels` table (cf. 0x1bc10).
    state.outlet("profileLabel")
}

// 0x1d5a8 — -[HomeViewController setProfileLabel:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setProfileLabel:]")]
pub fn stub_0x1d5a8(state: &mut HomeViewState, view: Option<u32>) {
    // IDA 0x1d5a8 `-[HomeViewController setProfileLabel:]`: SET (disasm
    // `objc_setProperty` prologue); host ownership is the outlet slot.
    state.set_outlet("profileLabel", view);
}

// 0x1d5cc — -[HomeViewController messagesLabel]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController messagesLabel]")]
pub fn stub_0x1d5cc(state: &HomeViewState) -> Option<u32> {
    // IDA 0x1d5cc `-[HomeViewController messagesLabel]`: GET (disasm
    // `_messagesLabel` IVAR load); the handle — the text lives in the
    // `labels` table (cf. 0x1bc10).
    state.outlet("messagesLabel")
}

// 0x1d5dc — -[HomeViewController setMessagesLabel:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setMessagesLabel:]")]
pub fn stub_0x1d5dc(state: &mut HomeViewState, view: Option<u32>) {
    // IDA 0x1d5dc `-[HomeViewController setMessagesLabel:]`: SET (disasm
    // `objc_setProperty` prologue); host ownership is the outlet slot.
    state.set_outlet("messagesLabel", view);
}

// 0x1d600 — -[HomeViewController btnPlayDisabled]
// type: UIButton *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController btnPlayDisabled]")]
pub fn stub_0x1d600(state: &HomeViewState) -> Option<u32> {
    // IDA 0x1d600 `-[HomeViewController btnPlayDisabled]`: GET (disasm
    // `_btnPlayDisabled` IVAR load); opaque platform handle on the host.
    state.outlet("btnPlayDisabled")
}

// 0x1d610 — -[HomeViewController setBtnPlayDisabled:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setBtnPlayDisabled:]")]
pub fn stub_0x1d610(state: &mut HomeViewState, view: Option<u32>) {
    // IDA 0x1d610 `-[HomeViewController setBtnPlayDisabled:]`: SET (disasm
    // `objc_setProperty` prologue); host ownership is the outlet slot.
    state.set_outlet("btnPlayDisabled", view);
}

// 0x1d634 — -[HomeViewController communityLabel]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController communityLabel]")]
pub fn stub_0x1d634(state: &HomeViewState) -> Option<u32> {
    // IDA 0x1d634 `-[HomeViewController communityLabel]`: GET (disasm
    // `_communityLabel` IVAR load); the handle — the text lives in the
    // `labels` table (cf. 0x1bc10).
    state.outlet("communityLabel")
}

// 0x1d644 — -[HomeViewController setCommunityLabel:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setCommunityLabel:]")]
pub fn stub_0x1d644(state: &mut HomeViewState, view: Option<u32>) {
    // IDA 0x1d644 `-[HomeViewController setCommunityLabel:]`: SET (disasm
    // `objc_setProperty` prologue); host ownership is the outlet slot.
    state.set_outlet("communityLabel", view);
}

// 0x1d668 — -[HomeViewController communityButton]
// type: UIButton *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController communityButton]")]
pub fn stub_0x1d668(state: &HomeViewState) -> Option<u32> {
    // IDA 0x1d668 `-[HomeViewController communityButton]`: GET (disasm
    // `_communityButton` IVAR load); opaque platform handle on the host.
    state.outlet("communityButton")
}

// 0x1d678 — -[HomeViewController setCommunityButton:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setCommunityButton:]")]
pub fn stub_0x1d678(state: &mut HomeViewState, view: Option<u32>) {
    // IDA 0x1d678 `-[HomeViewController setCommunityButton:]`: SET (disasm
    // `objc_setProperty` prologue); host ownership is the outlet slot.
    state.set_outlet("communityButton", view);
}

// 0x1d69c — -[HomeViewController buttonView]
// type: UIView *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController buttonView]")]
pub fn stub_0x1d69c(state: &HomeViewState) -> Option<u32> {
    // IDA 0x1d69c `-[HomeViewController buttonView]`: GET (disasm
    // `_buttonView` IVAR load); opaque platform handle on the host.
    state.outlet("buttonView")
}

// 0x1d6ac — -[HomeViewController setButtonView:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setButtonView:]")]
pub fn stub_0x1d6ac(state: &mut HomeViewState, view: Option<u32>) {
    // IDA 0x1d6ac `-[HomeViewController setButtonView:]`: SET (disasm
    // `objc_setProperty` prologue); host ownership is the outlet slot.
    state.set_outlet("buttonView", view);
}

// 0x1d6d0 — -[HomeViewController searchTextField]
// type: UITextField *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController searchTextField]")]
pub fn stub_0x1d6d0(state: &HomeViewState) -> Option<u32> {
    // IDA 0x1d6d0 `-[HomeViewController searchTextField]`: GET (disasm
    // `_searchTextField` IVAR load); opaque platform handle on the host.
    state.outlet("searchTextField")
}

// 0x1d6e0 — -[HomeViewController setSearchTextField:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setSearchTextField:]")]
pub fn stub_0x1d6e0(state: &mut HomeViewState, view: Option<u32>) {
    // IDA 0x1d6e0 `-[HomeViewController setSearchTextField:]`: SET (disasm
    // `objc_setProperty` prologue); host ownership is the outlet slot.
    state.set_outlet("searchTextField", view);
}

// 0x1d704 — -[HomeViewController loggedInView]
// type: UIView *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController loggedInView]")]
pub fn stub_0x1d704(state: &HomeViewState) -> Option<u32> {
    // IDA 0x1d704 `-[HomeViewController loggedInView]`: GET (disasm
    // `_loggedInView` IVAR load); visibility tracked by `logged_in_shown`
    // (cf. 0x1c788).
    state.outlet("loggedInView")
}

// 0x1d714 — -[HomeViewController setLoggedInView:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setLoggedInView:]")]
pub fn stub_0x1d714(state: &mut HomeViewState, view: Option<u32>) {
    // IDA 0x1d714 `-[HomeViewController setLoggedInView:]`: SET (disasm
    // `objc_setProperty` prologue); host ownership is the outlet slot.
    state.set_outlet("loggedInView", view);
}

// 0x1d738 — -[HomeViewController notLoggedInView]
// type: UIView *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController notLoggedInView]")]
pub fn stub_0x1d738(state: &HomeViewState) -> Option<u32> {
    // IDA 0x1d738 `-[HomeViewController notLoggedInView]`: GET (disasm
    // `_notLoggedInView` IVAR load); visibility tracked by
    // `logged_in_shown` (cf. 0x1c788).
    state.outlet("notLoggedInView")
}

// 0x1d748 — -[HomeViewController setNotLoggedInView:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setNotLoggedInView:]")]
pub fn stub_0x1d748(state: &mut HomeViewState, view: Option<u32>) {
    // IDA 0x1d748 `-[HomeViewController setNotLoggedInView:]`: SET (disasm
    // `objc_setProperty` prologue); host ownership is the outlet slot.
    state.set_outlet("notLoggedInView", view);
}

// 0x1d76c — -[HomeViewController signUpButtonLabel]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController signUpButtonLabel]")]
pub fn stub_0x1d76c(state: &HomeViewState) -> Option<u32> {
    // IDA 0x1d76c `-[HomeViewController signUpButtonLabel]`: GET (disasm
    // `_signUpButtonLabel` IVAR load); the handle — the text lives in the
    // `labels` table (cf. 0x1bc10).
    state.outlet("signUpButtonLabel")
}

// 0x1d77c — -[HomeViewController setSignUpButtonLabel:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setSignUpButtonLabel:]")]
pub fn stub_0x1d77c(state: &mut HomeViewState, view: Option<u32>) {
    // IDA 0x1d77c `-[HomeViewController setSignUpButtonLabel:]`: SET
    // (disasm `objc_setProperty` prologue); host ownership is the outlet
    // slot.
    state.set_outlet("signUpButtonLabel", view);
}

// 0x1d7a0 — -[HomeViewController loginButtonLabel]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController loginButtonLabel]")]
pub fn stub_0x1d7a0(state: &HomeViewState) -> Option<u32> {
    // IDA 0x1d7a0 `-[HomeViewController loginButtonLabel]`: GET (disasm
    // `_loginButtonLabel` IVAR load); the handle — the text lives in the
    // `labels` table (cf. 0x1bc10).
    state.outlet("loginButtonLabel")
}

// 0x1d7b0 — -[HomeViewController setLoginButtonLabel:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setLoginButtonLabel:]")]
pub fn stub_0x1d7b0(state: &mut HomeViewState, view: Option<u32>) {
    // IDA 0x1d7b0 `-[HomeViewController setLoginButtonLabel:]`: SET (disasm
    // `objc_setProperty` prologue); host ownership is the outlet slot.
    state.set_outlet("loginButtonLabel", view);
}

// 0x1d7d4 — -[HomeViewController welcomeToRobloxTextView]
// type: UITextView *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController welcomeToRobloxTextView]")]
pub fn stub_0x1d7d4(state: &HomeViewState) -> Option<u32> {
    // IDA 0x1d7d4 `-[HomeViewController welcomeToRobloxTextView]`: GET
    // (disasm `_welcomeToRobloxTextView` IVAR load); the handle — the text
    // lives in the `labels` table (cf. 0x1bc10).
    state.outlet("welcomeToRobloxTextView")
}

// 0x1d7e4 — -[HomeViewController setWelcomeToRobloxTextView:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setWelcomeToRobloxTextView:]")]
pub fn stub_0x1d7e4(state: &mut HomeViewState, view: Option<u32>) {
    // IDA 0x1d7e4 `-[HomeViewController setWelcomeToRobloxTextView:]`: SET
    // (disasm `objc_setProperty` prologue); host ownership is the outlet
    // slot.
    state.set_outlet("welcomeToRobloxTextView", view);
}

// 0x1d808 — -[HomeViewController youAreCurrentlyLoggedInAsTextView]
// type: UITextView *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController youAreCurrentlyLoggedInAsTextView]")]
pub fn stub_0x1d808(state: &HomeViewState) -> Option<u32> {
    // IDA 0x1d808 `-[HomeViewController youAreCurrentlyLoggedInAsTextView]`:
    // GET (disasm `_youAreCurrentlyLoggedInAsTextView` IVAR load); the
    // handle — the text lives in the `labels` table (cf. 0x1bc10).
    state.outlet("youAreCurrentlyLoggedInAsTextView")
}

// 0x1d818 — -[HomeViewController setYouAreCurrentlyLoggedInAsTextView:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setYouAreCurrentlyLoggedInAsTextView:]")]
pub fn stub_0x1d818(state: &mut HomeViewState, view: Option<u32>) {
    // IDA 0x1d818 `-[HomeViewController
    // setYouAreCurrentlyLoggedInAsTextView:]`: SET (disasm
    // `objc_setProperty` prologue); host ownership is the outlet slot.
    state.set_outlet("youAreCurrentlyLoggedInAsTextView", view);
}

// 0x1d83c — -[HomeViewController versionLabel]
// type: UILabel *__cdecl(HomeViewController *self, SEL)
#[doc(alias = "-[HomeViewController versionLabel]")]
pub fn stub_0x1d83c(state: &HomeViewState) -> Option<u32> {
    // IDA 0x1d83c `-[HomeViewController versionLabel]`: GET (disasm
    // `_versionLabel` IVAR load); the handle — the text is `version_text`
    // (cf. 0x1ba92).
    state.outlet("versionLabel")
}

// 0x1d84c — -[HomeViewController setVersionLabel:]
// type: void __cdecl(HomeViewController *self, SEL, id)
#[doc(alias = "-[HomeViewController setVersionLabel:]")]
pub fn stub_0x1d84c(state: &mut HomeViewState, view: Option<u32>) {
    // IDA 0x1d84c `-[HomeViewController setVersionLabel:]`: SET (disasm
    // `objc_setProperty` prologue); host ownership is the outlet slot.
    state.set_outlet("versionLabel", view);
}

// 0x1d870 — __GLOBAL__I_a_4
// was: global constructor keyed to_a_4
#[doc(alias = "global constructor keyed to_a_4")]
pub fn stub_0x1d870() {
    // IDA 0x1d870 `__GLOBAL__I_a_4`: same `generic_category` x2 +
    // `system_category` merged-globals init plus `ios_base::Init` +
    // `__cxa_atexit` as 0x1b308 (disasm 0x1d870..0x1d8b0; cf. 0x16e4c).
    // Host error categories need no init beyond `std::io`.
}

// 0x1da08 — -[NSString stringWithPercentEscape]
// type: NSString *__cdecl(NSString *self, SEL)
#[doc(alias = "-[NSString stringWithPercentEscape]")]
pub fn stub_0x1da08(text: &str) -> String {
    // IDA 0x1da08 `-[NSString stringWithPercentEscape]`:
    // `CFURLCreateStringByAddingPercentEscapes` escaping `=,!$&'()*+;@?"<># :/`
    // plus whitespace/control chars (0x1da1a..0x1da4a, encoding `0x8000100` =
    // UTF-8). Host percent-encodes per RFC 3986 with uppercase hex.
    let mut out = String::with_capacity(text.len());
    for b in text.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

// 0x1da5c — +[LoginViewController sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[LoginViewController sharedInstance]")]
pub fn stub_0x1da5c() -> parking_lot::MutexGuard<'static, LoginViewState> {
    // IDA 0x1da5c `+[LoginViewController sharedInstance]`: returns the
    // `dword_130C3F0` slot (0x1da68); host `LazyLock<Mutex<..>>` singleton.
    LOGIN_SHARED.lock()
}

// 0x1da6c — -[LoginViewController initWithCoder:]
// type: LoginViewController *__cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController initWithCoder:]")]
pub fn stub_0x1da6c() -> LoginViewState {
    // IDA 0x1da6c `-[LoginViewController initWithCoder:]`: super
    // `RobloxAnimatingPageViewController` init (0x1da8a..0x1da94, always
    // succeeds on the host), `envs = nil` (0x1dac4), then the three
    // notification observers (0x1dad0..0x1dbc6).
    LoginViewState {
        login_failed_observer: true,
        login_success_observer: true,
        signup_observer: true,
        ..LoginViewState::default()
    }
}

// 0x1dbd4 — -[LoginViewController dealloc]
// type: void __cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController dealloc]")]
pub fn stub_0x1dbd4(state: &mut LoginViewState) {
    // IDA 0x1dbd4 `-[LoginViewController dealloc]`: `removeObserver:`
    // (0x1dbf4..0x1dc06), releases the fifteen outlets plus `envs`
    // (0x1dc26..0x1dd58), then super dealloc (0x1dd70..0x1dd7a, host Drop
    // glue). The owned state folds back to default.
    *state = LoginViewState::default();
}

// 0x1dd84 — -[LoginViewController populateEnvironmentPicker]
// type: void __cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController populateEnvironmentPicker]")]
pub fn stub_0x1dd84(state: &mut LoginViewState, is_tablet: bool) {
    // IDA 0x1dd84 `-[LoginViewController populateEnvironmentPicker]`:
    // rebuilds `envs` (0x1dda8..0x1ddde); the host prefix is `www.` on
    // tablets, `m.` on phones (0x1de02..0x1de16), and the `allen..vlad`
    // sitetest entries take `m.` only on phones (0x1defc..0x1df0c).
    // Order: roblox, sitetest1..4, allen, anthony, guru, rosemary, sairam,
    // shannon, vlad, gametest5..1 (0x1de38..0x1e0d4).
    let prefix = if is_tablet { "www." } else { "m." };
    let dev_prefix = if is_tablet { "" } else { "m." };
    let mut envs = vec![
        format!("http://{prefix}roblox.com/"),
        format!("http://{prefix}sitetest1.robloxlabs.com/"),
        format!("http://{prefix}sitetest2.robloxlabs.com/"),
        format!("http://{prefix}sitetest3.robloxlabs.com/"),
        format!("http://{prefix}sitetest4.robloxlabs.com/"),
    ];
    for dev in ["allen", "anthony", "guru", "rosemary", "sairam", "shannon", "vlad"] {
        envs.push(format!("http://{dev_prefix}{dev}.sitetest3.robloxlabs.com/"));
    }
    for test in ["gametest5", "gametest4", "gametest3", "gametest2", "gametest1"] {
        envs.push(format!("http://{prefix}{test}.robloxlabs.com/"));
    }
    state.envs = envs;
}

// 0x1e0d8 — -[LoginViewController pickerView:didSelectRow:inComponent:]
// type: void __cdecl(LoginViewController *self, SEL, id, int, int)
#[doc(alias = "-[LoginViewController pickerView:didSelectRow:inComponent:]")]
pub fn stub_0x1e0d8(state: &mut LoginViewState, row: usize) {
    // IDA 0x1e0d8 `-[LoginViewController
    // pickerView:didSelectRow:inComponent:]`: `setBaseUrl:` to
    // `envs[row]` (0x1e106..0x1e11a), then the main-queue hop (0x1e138)
    // into the memory-bouncer block (cf. 0x1e13c). The hop is synchronous
    // here.
    if let Some(url) = state.envs.get(row).cloned() {
        state.base_url = Some(url);
        stub_0x1e13c(state);
    }
}

// 0x1e13c — ___59-[LoginViewController pickerView:didSelectRow:inComponent:]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___59-[LoginViewController pickerView:didSelectRow:inComponent:]_block_invoke")]
pub fn stub_0x1e13c(state: &mut LoginViewState) {
    // IDA 0x1e13c `__59-[...pickerView:didSelectRow:inComponent:]_block_invoke`:
    // `startMemoryBouncer` on the shared manager (0x1e158..0x1e16c).
    state.memory_bouncer_started = true;
}

// 0x1e170 — -[LoginViewController numberOfComponentsInPickerView:]
// type: int __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController numberOfComponentsInPickerView:]")]
pub fn stub_0x1e170() -> i32 {
    // IDA 0x1e170 `-[LoginViewController numberOfComponentsInPickerView:]`:
    // returns 1 (0x1e172; cf. 0x1b2bc).
    1
}

// 0x1e174 — -[LoginViewController pickerView:numberOfRowsInComponent:]
// type: int __cdecl(LoginViewController *self, SEL, id, int)
#[doc(alias = "-[LoginViewController pickerView:numberOfRowsInComponent:]")]
pub fn stub_0x1e174(state: &LoginViewState) -> i32 {
    // IDA 0x1e174 `-[LoginViewController
    // pickerView:numberOfRowsInComponent:]`: `[envs count]` (0x1e178; cf.
    // 0x1b2c0).
    state.envs.len() as i32
}

// 0x1e194 — -[LoginViewController pickerView:titleForRow:forComponent:]
// type: id __cdecl(LoginViewController *self, SEL, id, int, int)
#[doc(alias = "-[LoginViewController pickerView:titleForRow:forComponent:]")]
pub fn stub_0x1e194(state: &LoginViewState, row: usize) -> Option<String> {
    // IDA 0x1e194 `-[LoginViewController pickerView:titleForRow:forComponent:]`:
    // `[envs objectAtIndex:]` (0x1e198); host returns `None` out of range
    // instead of raising (cf. 0x1b2e0).
    state.envs.get(row).cloned()
}

// 0x1e1b4 — -[LoginViewController viewWillAppear:]
// type: void __cdecl(LoginViewController *self, SEL, char)
#[doc(alias = "-[LoginViewController viewWillAppear:]")]
pub fn stub_0x1e1b4(state: &mut LoginViewState, remember_password: bool, saved_password: Option<&str>) {
    // IDA 0x1e1b4 `-[LoginViewController viewWillAppear:]`:
    // `robloxLogo.alpha = 1.0` (0x1e1ca..0x1e1de), the `stopShowLoggingIn`
    // block on the main queue (0x1e210..0x1e224 -> 0x1e2c4), then the
    // password field takes the saved password when remembered (0x1e240..0x1e2a2)
    // and is cleared otherwise (0x1e2bc). The queue hop is synchronous here.
    state.logo_alpha_reset = true;
    stub_0x1e2c4(state);
    state.password_text = if remember_password {
        saved_password.unwrap_or("").to_string()
    } else {
        String::new()
    };
}

// 0x1e2c4 — ___38-[LoginViewController viewWillAppear:]_block_invoke
#[doc(alias = "___38-[LoginViewController viewWillAppear:]_block_invoke")]
pub fn stub_0x1e2c4(state: &mut LoginViewState) {
    // IDA 0x1e2c4 `__38-[LoginViewController viewWillAppear:]_block_invoke`:
    // `stopShowLoggingIn` shim (single `objc_msgSend`; cf. 0x1bb64).
    state.logging_in_hidden = true;
}

// 0x1e2d8 — ___copy_helper_block__2
#[doc(alias = "___copy_helper_block__2")]
pub fn stub_0x1e2d8(dst: &mut BlockCapture, src: &BlockCapture) {
    // IDA 0x1e2d8 `__copy_helper_block__2`: single
    // `_Block_object_assign` retain (0x1e2de; cf. 0x1bb88).
    *dst = src.clone();
}

// 0x1e2e4 — ___destroy_helper_block__2
#[doc(alias = "___destroy_helper_block__2")]
pub fn stub_0x1e2e4(slot: &mut BlockCapture) {
    // IDA 0x1e2e4 `__destroy_helper_block__2`: single
    // `_Block_object_dispose` release (0x1e2e8; cf. 0x1bb94).
    *slot = BlockCapture::default();
}

// 0x1e2ec — -[LoginViewController viewDidLoad]
// type: void __cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController viewDidLoad]")]
pub fn stub_0x1e2ec(
    state: &mut LoginViewState,
    os_version: &str,
    app_version: &str,
    device_name: &str,
    username: Option<&str>,
    password: Option<&str>,
    remember_password: bool,
    bundle_version: &str,
    user_agent: &str,
) {
    // IDA 0x1e2ec `-[LoginViewController viewDidLoad]`: super (0x1e30c..0x1e318),
    // publish singleton (0x1e33e), analytics vars for iOS/app/device
    // (0x1e342..0x1e3d6), Username/Password placeholders (0x1e406..0x1e49c),
    // Remember/Login/Signup/PlayNow labels + `CFBundleVersion` stamp
    // (0x1e4ae..0x1e606), UserAgent defaults (0x1e62e..0x1e73c, twice —
    // 0x1e734), skip/picker hidden (0x1e6aa/0x1e6c2), username prefill when
    // known (0x1e6e2..0x1e722), remember switch (0x1e764..0x1e78c),
    // password prefill when remembered and on (0x1e79e..0x1e7ea),
    // keyboard observers (0x1e808..0x1e870), memory-bouncer block
    // (0x1e88a -> 0x1e898).
    state.singleton_set = true;
    state.custom_vars.insert("iOSVersion".to_string(), os_version.to_string());
    state.custom_vars.insert("appVersion".to_string(), app_version.to_string());
    state.custom_vars.insert("deviceType".to_string(), device_name.to_string());
    state.placeholders.insert("username".to_string(), "UsernameWord".to_string());
    state.placeholders.insert("password".to_string(), "PasswordWord".to_string());
    for key in ["RememberPassword", "LoginWord", "SignupWord", "PlayNowButtonLabel"] {
        state.labels.insert(key.to_string(), key.to_string());
    }
    state.version_text = Some(bundle_version.to_string());
    state.user_agent_registered = !user_agent.is_empty();
    state.skip_hidden = true;
    state.env_picker_hidden = true;
    if let Some(name) = username {
        if !name.is_empty() {
            state.username_text = name.to_string();
        }
    }
    state.remember_switch_on = remember_password;
    if password.is_some() && remember_password {
        state.password_text = password.unwrap_or("").to_string();
    }
    state.keyboard_observers = true;
    stub_0x1e898(state);
    state.view_loaded = true;
}

// 0x1e898 — ___34-[LoginViewController viewDidLoad]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___34-[LoginViewController viewDidLoad]_block_invoke")]
pub fn stub_0x1e898(state: &mut LoginViewState) {
    // IDA 0x1e898 `__34-[LoginViewController viewDidLoad:]_block_invoke`:
    // `startMemoryBouncer` on the shared manager (0x1e8b4..0x1e8c8; cf.
    // 0x1e13c).
    state.memory_bouncer_started = true;
}

// 0x1e8cc — -[LoginViewController viewDidUnload]
// type: void __cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController viewDidUnload]")]
pub fn stub_0x1e8cc(state: &mut LoginViewState) {
    // IDA 0x1e8cc `-[LoginViewController viewDidUnload]`: nils the ten
    // outlets via setters (0x1e8e6..0x1e99a), super `viewDidUnload`
    // (0x1e9b2..0x1e9bc, host UIKit), clears the singleton slot (0x1e9ca).
    state.outlets.clear();
    state.singleton_cleared = true;
}

// 0x1e9d0 — -[LoginViewController handleSignupNotification:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController handleSignupNotification:]")]
pub fn stub_0x1e9d0(state: &mut LoginViewState, username: Option<&str>, password: Option<&str>) {
    // IDA 0x1e9d0 `-[LoginViewController handleSignupNotification:]`:
    // pulls `username`/`password` from `userInfo` (0x1e9ee..0x1ea28),
    // retains both (0x1ea3a..0x1ea42); when both exist, runs the
    // main-queue fill block (0x1ea7e..0x1ea92 -> 0x1eaa0). The hop is
    // synchronous here.
    if let (Some(name), Some(pass)) = (username, password) {
        stub_0x1eaa0(state, name, pass);
    }
}

// 0x1eaa0 — ___48-[LoginViewController handleSignupNotification:]_block_invoke
#[doc(alias = "___48-[LoginViewController handleSignupNotification:]_block_invoke")]
pub fn stub_0x1eaa0(state: &mut LoginViewState, username: &str, password: &str) {
    // IDA 0x1eaa0 `__48-[...handleSignupNotification:]_block_invoke`:
    // stamps the retained credentials into the fields (0x1eab4..0x1eae2);
    // the releases (0x1eaf6..0x1eafa) fold into host ownership.
    state.username_text = username.to_string();
    state.password_text = password.to_string();
}

// 0x1eb08 — ___copy_helper_block_226
// type: void __fastcall(int, const void **)
#[doc(alias = "___copy_helper_block_226")]
pub fn stub_0x1eb08(dst: &mut SignupCaptures, src: &SignupCaptures) {
    // IDA 0x1eb08 `__copy_helper_block_226`: `_Block_object_assign`
    // retain of the three captures (0x1eb18..0x1eb34; cf. 0x1ae78).
    *dst = src.clone();
}

// 0x1eb38 — ___destroy_helper_block_227
#[doc(alias = "___destroy_helper_block_227")]
pub fn stub_0x1eb38(slot: &mut SignupCaptures) {
    // IDA 0x1eb38 `__destroy_helper_block_227`: `_Block_object_dispose`
    // release of the three captures (0x1eb42..0x1eb56; cf. 0x1aea8).
    *slot = SignupCaptures::default();
}

// 0x1eb5c — -[LoginViewController gotLoginFailedNotification:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController gotLoginFailedNotification:]")]
pub fn stub_0x1eb5c(state: &mut LoginViewState, error: &str) {
    // IDA 0x1eb5c `-[LoginViewController gotLoginFailedNotification:]`:
    // pulls `Error` from `userInfo` (0x1eb72) and runs the main-queue
    // block (0x1ebc0..0x1ebd4 -> 0x1ebdc). The hop is synchronous here.
    stub_0x1ebdc(state, error);
}

// 0x1ebdc — ___50-[LoginViewController gotLoginFailedNotification:]_block_invoke
#[doc(alias = "___50-[LoginViewController gotLoginFailedNotification:]_block_invoke")]
pub fn stub_0x1ebdc(state: &mut LoginViewState, error: &str) {
    // IDA 0x1ebdc `__50-[...gotLoginFailedNotification:]_block_invoke`:
    // `stopShowLoggingIn` (0x1ebf0 -> 0x1eeac),
    // `RobloxAlertWithMessage:` with the error (0x1ec0e), password cleared
    // (0x1ec20..0x1ec24).
    stub_0x1eeac(state);
    state.login_error = Some(error.to_string());
    state.password_text.clear();
}

// 0x1ec44 — ___copy_helper_block_234
#[doc(alias = "___copy_helper_block_234")]
pub fn stub_0x1ec44(dst: &mut LoginErrorCaptures, src: &LoginErrorCaptures) {
    // IDA 0x1ec44 `__copy_helper_block_234`: `_Block_object_assign`
    // retain of the two captures (0x1ec54..0x1ec64; cf. 0x1eb08).
    *dst = src.clone();
}

// 0x1ec68 — ___destroy_helper_block_235
#[doc(alias = "___destroy_helper_block_235")]
pub fn stub_0x1ec68(slot: &mut LoginErrorCaptures) {
    // IDA 0x1ec68 `__destroy_helper_block_235`: `_Block_object_dispose`
    // release of the two captures (0x1ec72..0x1ec7e; cf. 0x1eb38).
    *slot = LoginErrorCaptures::default();
}

// 0x1ec84 — -[LoginViewController gotLoginSuccessfulNotification:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController gotLoginSuccessfulNotification:]")]
pub fn stub_0x1ec84(state: &mut LoginViewState) {
    // IDA 0x1ec84 `-[LoginViewController gotLoginSuccessfulNotification:]`:
    // `getStoreMgr` (0x1eca4), `doLoginTransition` (0x1ecb6), then the
    // main-queue block (0x1ece8..0x1ecfc -> 0x1ed04). The hop is
    // synchronous here.
    state.store_mgr_polled = true;
    state.login_transition_requested = true;
    stub_0x1ed04(state);
}

// 0x1ed04 — ___54-[LoginViewController gotLoginSuccessfulNotification:]_block_invoke
#[doc(alias = "___54-[LoginViewController gotLoginSuccessfulNotification:]_block_invoke")]
pub fn stub_0x1ed04(state: &mut LoginViewState) {
    // IDA 0x1ed04 `__54-[...gotLoginSuccessfulNotification:]_block_invoke`:
    // clears the `self+204` (password) field text (0x1ed04).
    state.password_text.clear();
}

// 0x1ed30 — ___copy_helper_block_242
#[doc(alias = "___copy_helper_block_242")]
pub fn stub_0x1ed30(dst: &mut BlockCapture, src: &BlockCapture) {
    // IDA 0x1ed30 `__copy_helper_block_242`: single
    // `_Block_object_assign` retain (0x1ed36; cf. 0x1e2d8).
    *dst = src.clone();
}

// 0x1ed3c — ___destroy_helper_block_243
#[doc(alias = "___destroy_helper_block_243")]
pub fn stub_0x1ed3c(slot: &mut BlockCapture) {
    // IDA 0x1ed3c `__destroy_helper_block_243`: single
    // `_Block_object_dispose` release (0x1ed40; cf. 0x1e2e4).
    *slot = BlockCapture::default();
}

// 0x1ed44 — -[LoginViewController showLoggingIn]
// type: void __cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController showLoggingIn]")]
pub fn stub_0x1ed44(state: &mut LoginViewState) {
    // IDA 0x1ed44 `-[LoginViewController showLoggingIn]`: hides
    // `aboutButton` (0x1ed5a..0x1ed6c), then the main-queue block
    // (0x1ed9e..0x1edb2 -> 0x1edbc). The hop is synchronous here.
    state.about_hidden = true;
    state.logging_in_shown = true;
    stub_0x1edbc(state);
}

// 0x1edbc — ___36-[LoginViewController showLoggingIn]_block_invoke
#[doc(alias = "___36-[LoginViewController showLoggingIn]_block_invoke")]
pub fn stub_0x1edbc(state: &mut LoginViewState) {
    // IDA 0x1edbc `__36-[LoginViewController showLoggingIn:]_block_invoke`:
    // unhides the activity indicator (0x1edd2..0x1ede6), then the 0.5s
    // fade animation block (0x1ee24..0x1ee54 -> 0x1ee58, completion nil).
    // The animation end state applies synchronously here.
    state.activity_shown = true;
    stub_0x1ee58(state);
}

// 0x1ee58 — ___36-[LoginViewController showLoggingIn]_block_invoke_2
#[doc(alias = "___36-[LoginViewController showLoggingIn]_block_invoke_2")]
pub fn stub_0x1ee58(state: &mut LoginViewState) {
    // IDA 0x1ee58 `__36-[LoginViewController showLoggingIn:]_block_invoke_2`:
    // `loginFieldViews.alpha = 0` fade step (0x1ee6a).
    state.fields_alpha_zero = true;
}

// 0x1ee84 — ___copy_helper_block_252
#[doc(alias = "___copy_helper_block_252")]
pub fn stub_0x1ee84(dst: &mut BlockCapture, src: &BlockCapture) {
    // IDA 0x1ee84 `__copy_helper_block_252`: single
    // `_Block_object_assign` retain (0x1ee8a; cf. 0x1ed30).
    *dst = src.clone();
}

// 0x1ee90 — ___destroy_helper_block_253
#[doc(alias = "___destroy_helper_block_253")]
pub fn stub_0x1ee90(slot: &mut BlockCapture) {
    // IDA 0x1ee90 `__destroy_helper_block_253`: single
    // `_Block_object_dispose` release (0x1ee94; cf. 0x1ed3c).
    *slot = BlockCapture::default();
}

// 0x1ee98 — ___copy_helper_block_257
#[doc(alias = "___copy_helper_block_257")]
pub fn stub_0x1ee98(dst: &mut BlockCapture, src: &BlockCapture) {
    // IDA 0x1ee98 `__copy_helper_block_257`: single
    // `_Block_object_assign` retain (0x1ee9e; cf. 0x1ee84).
    *dst = src.clone();
}

// 0x1eea4 — ___destroy_helper_block_258
#[doc(alias = "___destroy_helper_block_258")]
pub fn stub_0x1eea4(slot: &mut BlockCapture) {
    // IDA 0x1eea4 `__destroy_helper_block_258`: single
    // `_Block_object_dispose` release (0x1eea8; cf. 0x1ee90).
    *slot = BlockCapture::default();
}

// 0x1eeac — -[LoginViewController stopShowLoggingIn]
// type: void __cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController stopShowLoggingIn]")]
pub fn stub_0x1eeac(state: &mut LoginViewState) {
    // IDA 0x1eeac `-[LoginViewController stopShowLoggingIn]`: runs the
    // main-queue block (0x1eee2..0x1eef4 -> 0x1eefc). The hop is
    // synchronous here.
    stub_0x1eefc(state);
}

// 0x1eefc — ___40-[LoginViewController stopShowLoggingIn]_block_invoke
#[doc(alias = "___40-[LoginViewController stopShowLoggingIn]_block_invoke")]
pub fn stub_0x1eefc(state: &mut LoginViewState) {
    // IDA 0x1eefc `__40-[...stopShowLoggingIn]_block_invoke`: unhides
    // `aboutButton` (0x1ef12..0x1ef28), hides the activity indicator
    // (0x1ef3a..0x1ef42), then the 0.5s restore animation block
    // (0x1ef80..0x1efaa -> 0x1efac, completion nil). The end state applies
    // synchronously here.
    state.about_hidden = false;
    state.activity_shown = false;
    stub_0x1efac(state);
}

// 0x1efac — ___40-[LoginViewController stopShowLoggingIn]_block_invoke_2
#[doc(alias = "___40-[LoginViewController stopShowLoggingIn]_block_invoke_2")]
pub fn stub_0x1efac(state: &mut LoginViewState) {
    // IDA 0x1efac `__40-[...stopShowLoggingIn]_block_invoke_2`:
    // `loginFieldViews.alpha = 1.0` restore step (0x1efbe).
    state.fields_alpha_zero = false;
    state.logging_in_hidden = true;
}

// 0x1efdc — ___copy_helper_block_260
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_260")]
pub fn stub_0x1efdc(dst: &mut BlockCapture, src: &BlockCapture) {
    // IDA 0x1efdc `__copy_helper_block_260`: single
    // `_Block_object_assign` retain (0x1efe2; cf. 0x1ee98).
    *dst = src.clone();
}

// 0x1efe8 — ___destroy_helper_block_261
#[doc(alias = "___destroy_helper_block_261")]
pub fn stub_0x1efe8(slot: &mut BlockCapture) {
    // IDA 0x1efe8 `__destroy_helper_block_261`: single
    // `_Block_object_dispose` release (0x1efec; cf. 0x1eea4).
    *slot = BlockCapture::default();
}

// 0x1eff0 — ___copy_helper_block_263
#[doc(alias = "___copy_helper_block_263")]
pub fn stub_0x1eff0(dst: &mut BlockCapture, src: &BlockCapture) {
    // IDA 0x1eff0 `__copy_helper_block_263`: single
    // `_Block_object_assign` retain (0x1eff6; cf. 0x1efdc).
    *dst = src.clone();
}

// 0x1effc — ___destroy_helper_block_264
#[doc(alias = "___destroy_helper_block_264")]
pub fn stub_0x1effc(slot: &mut BlockCapture) {
    // IDA 0x1effc `__destroy_helper_block_264`: single
    // `_Block_object_dispose` release (0x1f000; cf. 0x1efe8).
    *slot = BlockCapture::default();
}

// 0x1f004 — -[LoginViewController playNowDidTouchUpInside:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController playNowDidTouchUpInside:]")]
pub fn stub_0x1f004(state: &mut LoginViewState) {
    // IDA 0x1f004 `-[LoginViewController playNowDidTouchUpInside:]`:
    // sets `userDidClickPlayNow` (0x1f024); with a nonempty password field
    // runs `login:` (0x1f028..0x1f0ce -> 0x1f0d4), otherwise `doLogout` +
    // `setPageViewTracking:@"Login/GuestMode"` + `segueToHomeViewController:1`
    // (0x1f080..0x1f064).
    state.play_now_clicked = true;
    if !state.password_text.is_empty() {
        stub_0x1f0d4(state);
    } else {
        state.guest_mode = true;
        state.guest_page_view = Some("Login/GuestMode".to_string());
        state.home_segue_animated = Some(true);
    }
}

// 0x1f0d4 — -[LoginViewController login:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController login:]")]
pub fn stub_0x1f0d4(state: &mut LoginViewState) {
    // IDA 0x1f0d4 `-[LoginViewController login:]`: `endEditing:` on both
    // fields (0x1f0f0..0x1f122), `showLoggingIn` (0x1f134 -> 0x1ed44),
    // `doLoginWithUsername:password:` with the field texts (0x1f154..0x1f19a).
    state.fields_editing_ended = true;
    stub_0x1ed44(state);
    state.login_attempt = Some((state.username_text.clone(), state.password_text.clone()));
}

// 0x1f1a0 — -[LoginViewController usernameDidEndOnExit:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController usernameDidEndOnExit:]")]
pub fn stub_0x1f1a0(state: &mut LoginViewState) {
    // IDA 0x1f1a0 `-[LoginViewController usernameDidEndOnExit:]`:
    // `password.becomeFirstResponder` (0x1f1b0..0x1f1c4).
    state.password_focused = true;
}

// 0x1f1c8 — -[LoginViewController passwordDidEndOnExit:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController passwordDidEndOnExit:]")]
pub fn stub_0x1f1c8(state: &mut LoginViewState) {
    // IDA 0x1f1c8 `-[LoginViewController passwordDidEndOnExit:]`:
    // `showLoggingIn` (0x1f1e0 -> 0x1ed44), then
    // `doLoginWithUsername:password:` with the field texts (0x1f200..0x1f25a;
    // cf. 0x1f0d4).
    stub_0x1ed44(state);
    state.login_attempt = Some((state.username_text.clone(), state.password_text.clone()));
}

// 0x1f260 — -[LoginViewController swiToggleRememberMyPassword:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController swiToggleRememberMyPassword:]")]
pub fn stub_0x1f260(state: &mut LoginViewState, switch_on: bool) {
    // IDA 0x1f260 `-[LoginViewController swiToggleRememberMyPassword:]`:
    // `setRememberPassword:` with the switch state (0x1f282..0x1f2ba).
    state.remember_switch_on = switch_on;
    state.remember_set = Some(switch_on);
}

// 0x1f2c0 — -[LoginViewController loginButtonDidTouchUpInside:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController loginButtonDidTouchUpInside:]")]
pub fn stub_0x1f2c0(state: &mut LoginViewState) {
    // IDA 0x1f2c0 `-[LoginViewController loginButtonDidTouchUpInside:]`:
    // `userDidClickPlayNow = 0` (0x1f2d6), then `login:` (0x1f2dc ->
    // 0x1f0d4).
    state.play_now_flag = false;
    stub_0x1f0d4(state);
}

// 0x1f2e0 — -[LoginViewController onKeyboardHide:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController onKeyboardHide:]")]
pub fn stub_0x1f2e0(state: &mut LoginViewState, has_memory_warning: bool) {
    // IDA 0x1f2e0 `-[LoginViewController onKeyboardHide:]`:
    // `scrollView.setContentOffset:(0,0)` (0x1f30c); unless a memory
    // warning was received, `startBackgroundPan` plus the main-queue
    // block (0x1f31e..0x1f376 -> 0x1f380). The hop is synchronous here.
    state.scroll_offset = Some((0.0, 0.0));
    if !has_memory_warning {
        state.background_pan_running = true;
        stub_0x1f380(state);
    }
}

// 0x1f380 — ___38-[LoginViewController onKeyboardHide:]_block_invoke
#[doc(alias = "___38-[LoginViewController onKeyboardHide:]_block_invoke")]
pub fn stub_0x1f380(state: &mut LoginViewState) {
    // IDA 0x1f380 `__38-[...onKeyboardHide:]_block_invoke`: wraps the
    // restore block in a 0.25s animation (0x1f3c4..0x1f3ec -> 0x1f3f8).
    // The end state applies synchronously here.
    stub_0x1f3f8(state);
}

// 0x1f3f8 — ___38-[LoginViewController onKeyboardHide:]_block_invoke_2
#[doc(alias = "___38-[LoginViewController onKeyboardHide:]_block_invoke_2")]
pub fn stub_0x1f3f8(state: &mut LoginViewState) {
    // IDA 0x1f3f8 `__38-[...onKeyboardHide:]_block_invoke_2`: restores
    // alpha 1.0 on the background/foreground images and the two
    // `self+160/164` views (0x1f40c..0x1f45a).
    state.bg_images_dimmed = false;
}

// 0x1f480 — ___copy_helper_block_300
#[doc(alias = "___copy_helper_block_300")]
pub fn stub_0x1f480(dst: &mut BlockCapture, src: &BlockCapture) {
    // IDA 0x1f480 `__copy_helper_block_300`: single
    // `_Block_object_assign` retain (0x1f486; cf. 0x1efdc).
    *dst = src.clone();
}

// 0x1f48c — ___destroy_helper_block_301
#[doc(alias = "___destroy_helper_block_301")]
pub fn stub_0x1f48c(slot: &mut BlockCapture) {
    // IDA 0x1f48c `__destroy_helper_block_301`: single
    // `_Block_object_dispose` release (0x1f490; cf. 0x1efe8).
    *slot = BlockCapture::default();
}

// 0x1f494 — ___copy_helper_block_305
#[doc(alias = "___copy_helper_block_305")]
pub fn stub_0x1f494(dst: &mut BlockCapture, src: &BlockCapture) {
    // IDA 0x1f494 `__copy_helper_block_305`: single
    // `_Block_object_assign` retain (0x1f49a; cf. 0x1f480).
    *dst = src.clone();
}
