//! reflection — generated_bg_5 — 120 stubs EA-sorted asc global gap filler 0x1f380.. not yet in crates/reflection (global 85545 funcs, 64201 gaps reflection; 21344 distinct before, 21464 after)
//! Source: ida/export.json (85545 funcs) global EA asc not in crates/reflection/src — next 120 uncovered for reflection-bg sorted asc after 0x1f2e0
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

/// Gap-filler LoginViewController keyboard/transition observable state
/// (IDA 0x1f380-0x1f854). The scrolled offset, password text, guest-tap
/// flag and home-segue counts live in `generated_bg_4`; the background
/// scene alphas, keyboard-show counts, pan stops, saved-defaults pair
/// and segue dispatches record here with matching shapes.
pub(crate) static LOGIN_BG_ALPHA_BITS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0x3f800000);
pub(crate) static LOGIN_FG_ALPHA_BITS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0x3f800000);
pub(crate) static LOGIN_BG_COPY_ALPHA_BITS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0x3f800000);
pub(crate) static LOGIN_FG_COPY_ALPHA_BITS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0x3f800000);
pub(crate) static LOGIN_KEYBOARD_SHOWS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static LOGIN_KEYBOARD_SHOW_DISPATCHES: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static LOGIN_BACKGROUND_PAN_STOPS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static LOGIN_DEFAULTS: parking_lot::Mutex<(String, String)> =
    parking_lot::Mutex::new((String::new(), String::new()));
pub(crate) static LOGIN_HOME_SEGUE_DISPATCHES: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
/// Fade the four background-scene views together (IDA 0x1f3f8/0x1f5e0).
/// `imgBackground`/`imgForeground` plus the `foregroundCopy` /
/// `backgroundCopy` superclass ivars always move as one.
pub(crate) fn set_login_scene_alpha(bits: u32) {
    LOGIN_BG_ALPHA_BITS.store(bits, std::sync::atomic::Ordering::SeqCst);
    LOGIN_FG_ALPHA_BITS.store(bits, std::sync::atomic::Ordering::SeqCst);
    LOGIN_BG_COPY_ALPHA_BITS.store(bits, std::sync::atomic::Ordering::SeqCst);
    LOGIN_FG_COPY_ALPHA_BITS.store(bits, std::sync::atomic::Ordering::SeqCst);
}
pub(crate) static LOGIN_HOME_INSTANTIATIONS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static LOGIN_HOME_PRESENTATIONS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static LOGIN_BUTTON_ALPHA_BITS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0x3f800000);
pub(crate) static LOGIN_SEGUE_URL: parking_lot::Mutex<String> = parking_lot::Mutex::new(String::new());
pub(crate) static LOGIN_SEGUE_PRELOADED: parking_lot::Mutex<String> =
    parking_lot::Mutex::new(String::new());

/// Gap-filler AboutController observable state (IDA 0x20468-0x20cb4).
/// The window rect, agreement HTML, localized label texts, web-view
/// visibility, segue/dismiss/cookie counts and alert text record here;
/// outlets keep opaque `id` handles (0 when unset) behind
/// `objc_setProperty` retain glue.
pub(crate) static ABOUT_WINDOW: parking_lot::Mutex<(f32, f32, f32, f32)> =
    parking_lot::Mutex::new((0.0, 0.0, 0.0, 0.0));
pub(crate) static ABOUT_BOUNDS: parking_lot::Mutex<(f32, f32, f32, f32)> =
    parking_lot::Mutex::new((0.0, 0.0, 0.0, 0.0));
pub(crate) static ABOUT_AGREEMENT_HIDDEN: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static ABOUT_WEB_SCROLL_ENABLED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(true);
pub(crate) static ABOUT_AGREEMENT_HTML: parking_lot::Mutex<String> = parking_lot::Mutex::new(String::new());
pub(crate) static ABOUT_LABEL_TEXTS: std::sync::LazyLock<
    parking_lot::Mutex<std::collections::HashMap<String, String>>,
> = std::sync::LazyLock::new(|| parking_lot::Mutex::new(std::collections::HashMap::new()));
pub(crate) static ABOUT_AGREEMENT_URL: parking_lot::Mutex<String> = parking_lot::Mutex::new(String::new());
pub(crate) static ABOUT_TO_AGREEMENT_SEGUES: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static ABOUT_DISMISSALS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
pub(crate) static ABOUT_COOKIES_CLEARED: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static LAST_ABOUT_ALERT: parking_lot::Mutex<String> = parking_lot::Mutex::new(String::new());
pub(crate) static ABOUT_OUTLETS: std::sync::LazyLock<
    parking_lot::Mutex<std::collections::HashMap<String, usize>>,
> = std::sync::LazyLock::new(|| parking_lot::Mutex::new(std::collections::HashMap::new()));
pub(crate) fn about_outlet(name: &str) -> usize {
    ABOUT_OUTLETS.lock().get(name).copied().unwrap_or(0)
}
pub(crate) fn set_about_outlet(name: &str, handle: usize) {
    ABOUT_OUTLETS.lock().insert(name.to_owned(), handle);
}

// 0x1f380 — ___38-[LoginViewController onKeyboardHide:]_block_invoke
#[doc(alias = "___38-[LoginViewController onKeyboardHide:]_block_invoke")]
pub fn stub_0x1f380() {
    // IDA 0x1f380: the hide block runs the 0.3s fade-in animation
    // (0x1f380-0x1f3f4, stub_0x1f3f8). The animation hop collapses to the
    // direct call.
    stub_0x1f3f8();
}

// 0x1f3f8 — ___38-[LoginViewController onKeyboardHide:]_block_invoke_2
#[doc(alias = "___38-[LoginViewController onKeyboardHide:]_block_invoke_2")]
pub fn stub_0x1f3f8() {
    // IDA 0x1f3f8: the hide fade-in restores the background-scene alpha
    // to 1.0 (0x1f3f8-0x1f47c, 0x3f800000): `imgBackground`,
    // `imgForeground` and the `foregroundCopy`/`backgroundCopy`
    // superclass ivars (disasm 0x1f442-0x1f46e).
    set_login_scene_alpha(0x3f800000);
}

// 0x1f480 — ___copy_helper_block_300
#[doc(alias = "___copy_helper_block_300")]
pub fn stub_0x1f480(_dst: usize, _src: usize) {
    // IDA 0x1f480: `__copy_helper_block_300` — one `_Block_object_assign`
    // retain (0x1f480-0x1f486, same shape as stub_0x18094). No explicit
    // body.
}

// 0x1f48c — ___destroy_helper_block_301
#[doc(alias = "___destroy_helper_block_301")]
pub fn stub_0x1f48c(_block: usize) {
    // IDA 0x1f48c: `__destroy_helper_block_301` — one
    // `_Block_object_dispose` release (0x1f48c-0x1f490, same shape as
    // stub_0x180a0). No explicit body.
}

// 0x1f494 — ___copy_helper_block_305
#[doc(alias = "___copy_helper_block_305")]
pub fn stub_0x1f494(_dst: usize, _src: usize) {
    // IDA 0x1f494: `__copy_helper_block_305` — one `_Block_object_assign`
    // retain (0x1f494-0x1f49a, same shape as stub_0x18094). No explicit
    // body.
}

// 0x1f4a0 — ___destroy_helper_block_306
#[doc(alias = "___destroy_helper_block_306")]
pub fn stub_0x1f4a0(_block: usize) {
    // IDA 0x1f4a0: `__destroy_helper_block_306` — one
    // `_Block_object_dispose` release (0x1f4a0-0x1f4a4, same shape as
    // stub_0x180a0). No explicit body.
}

// 0x1f4a8 — -[LoginViewController onKeyboardShow:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController onKeyboardShow:]")]
pub fn stub_0x1f4a8(has_received_memory_warning: bool) {
    // IDA 0x1f4a8: `onKeyboardShow:` shifts the scroll offset to (0, 115)
    // animated (0x1f4a8-0x1f4f6, 115.0 = 0x42e60000) and, unless the
    // controller has received a memory warning (0x1f4f8-0x1f508),
    // dispatches the show block on main (0x1f50c-0x1f534, stub_0x1f538).
    // The warning flag crosses as a parameter, mirroring stub_0x1f2e0;
    // the queue hop collapses to the direct call.
    *crate::generated_bg_4::LOGIN_SCROLL_OFFSET.lock() = (0.0, 115.0);
    LOGIN_KEYBOARD_SHOWS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    if !has_received_memory_warning {
        LOGIN_KEYBOARD_SHOW_DISPATCHES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        stub_0x1f538();
    }
}

// 0x1f538 — ___38-[LoginViewController onKeyboardShow:]_block_invoke
#[doc(alias = "___38-[LoginViewController onKeyboardShow:]_block_invoke")]
pub fn stub_0x1f538() {
    // IDA 0x1f538: the show block runs the 0.3s fade-out animation
    // (stub_0x1f5e0) with the pan-stop completion (stub_0x1f674)
    // (0x1f538-0x1f5da). Both hops collapse to direct calls.
    stub_0x1f5e0();
    stub_0x1f674();
}

// 0x1f5e0 — ___38-[LoginViewController onKeyboardShow:]_block_invoke_2
#[doc(alias = "___38-[LoginViewController onKeyboardShow:]_block_invoke_2")]
pub fn stub_0x1f5e0() {
    // IDA 0x1f5e0: the show fade-out zeroes the background-scene alpha
    // (0x1f5e0-0x1f656): `imgBackground`, `imgForeground` and the
    // `foregroundCopy`/`backgroundCopy` superclass ivars, mirroring
    // stub_0x1f3f8.
    set_login_scene_alpha(0);
}

// 0x1f660 — ___copy_helper_block_308
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_308")]
pub fn stub_0x1f660(_dst: usize, _src: usize) {
    // IDA 0x1f660: `__copy_helper_block_308` — one `_Block_object_assign`
    // retain (0x1f660-0x1f666, same shape as stub_0x18094). No explicit
    // body.
}

// 0x1f66c — ___destroy_helper_block_309
#[doc(alias = "___destroy_helper_block_309")]
pub fn stub_0x1f66c(_block: usize) {
    // IDA 0x1f66c: `__destroy_helper_block_309` — one
    // `_Block_object_dispose` release (0x1f66c-0x1f670, same shape as
    // stub_0x180a0). No explicit body.
}

// 0x1f674 — ___38-[LoginViewController onKeyboardShow:]_block_invoke311
// type: id __fastcall(int)
#[doc(alias = "___38-[LoginViewController onKeyboardShow:]_block_invoke311")]
pub fn stub_0x1f674() {
    // IDA 0x1f674: the show-animation completion stops the background
    // pan (0x1f674-0x1f67c).
    LOGIN_BACKGROUND_PAN_STOPS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x1f688 — ___copy_helper_block_314
#[doc(alias = "___copy_helper_block_314")]
pub fn stub_0x1f688(_dst: usize, _src: usize) {
    // IDA 0x1f688: `__copy_helper_block_314` — one `_Block_object_assign`
    // retain (0x1f688-0x1f68e, same shape as stub_0x18094). No explicit
    // body.
}

// 0x1f694 — ___destroy_helper_block_315
#[doc(alias = "___destroy_helper_block_315")]
pub fn stub_0x1f694(_block: usize) {
    // IDA 0x1f694: `__destroy_helper_block_315` — one
    // `_Block_object_dispose` release (0x1f694-0x1f698, same shape as
    // stub_0x180a0). No explicit body.
}

// 0x1f69c — ___copy_helper_block_320
#[doc(alias = "___copy_helper_block_320")]
pub fn stub_0x1f69c(_dst: usize, _src: usize) {
    // IDA 0x1f69c: `__copy_helper_block_320` — one `_Block_object_assign`
    // retain (0x1f69c-0x1f6a2, same shape as stub_0x18094). No explicit
    // body.
}

// 0x1f6a8 — ___destroy_helper_block_321
#[doc(alias = "___destroy_helper_block_321")]
pub fn stub_0x1f6a8(_block: usize) {
    // IDA 0x1f6a8: `__destroy_helper_block_321` — one
    // `_Block_object_dispose` release (0x1f6a8-0x1f6ac, same shape as
    // stub_0x180a0). No explicit body.
}

// 0x1f6b0 — -[LoginViewController doLoginTransition]
// type: void __cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController doLoginTransition]")]
pub fn stub_0x1f6b0(remember_password: bool, username: &str, password: &str) {
    // IDA 0x1f6b0: `doLoginTransition` clears the password field on main
    // when `LoginManager getRememberPassword` is unset (0x1f6c6-0x1f706,
    // stub_0x1f808), saves the `UserInfo CurrentPlayer` username/password
    // into `NSUserDefaults` (0x1f708-0x1f78c), and segues home with the
    // guest-tap flag (0x1f78e-0x1f806, stub_0x1f854). The manager/player
    // queries cross as parameters, mirroring stub_0x1e2ec; the queue hop
    // collapses to the direct call.
    if !remember_password {
        stub_0x1f808();
    }
    crate::generated_bg_4::LOGIN_TRANSITIONS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    *LOGIN_DEFAULTS.lock() = (username.to_owned(), password.to_owned());
    let guest = crate::generated_bg_4::USER_DID_CLICK_PLAY_NOW.load(std::sync::atomic::Ordering::SeqCst);
    stub_0x1f854(guest);
}

// 0x1f808 — ___40-[LoginViewController doLoginTransition]_block_invoke
#[doc(alias = "___40-[LoginViewController doLoginTransition]_block_invoke")]
pub fn stub_0x1f808() {
    // IDA 0x1f808: the transition block clears the password field via
    // `setText:` with nil (0x1f808-0x1f828) — the same observable as
    // stub_0x1ed04.
    crate::generated_bg_4::LOGIN_PASSWORD_TEXT.lock().clear();
}

// 0x1f82c — ___copy_helper_block_323
#[doc(alias = "___copy_helper_block_323")]
pub fn stub_0x1f82c(_dst: usize, _src: usize) {
    // IDA 0x1f82c: `__copy_helper_block_323` — one `_Block_object_assign`
    // retain (0x1f82c-0x1f832, same shape as stub_0x18094). No explicit
    // body.
}

// 0x1f838 — ___destroy_helper_block_324
#[doc(alias = "___destroy_helper_block_324")]
pub fn stub_0x1f838(_block: usize) {
    // IDA 0x1f838: `__destroy_helper_block_324` — one
    // `_Block_object_dispose` release (0x1f838-0x1f83c, same shape as
    // stub_0x180a0). No explicit body.
}

// 0x1f840 — -[LoginViewController externalSegueToHomeViewController:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController externalSegueToHomeViewController:]")]
pub fn stub_0x1f840() {
    // IDA 0x1f840: `externalSegueToHomeViewController:` segues home
    // unanimated (0x1f840-0x1f852, stub_0x1f854).
    stub_0x1f854(false);
}

// 0x1f854 — -[LoginViewController segueToHomeViewController:]
// type: void __cdecl(LoginViewController *self, SEL, char)
#[doc(alias = "-[LoginViewController segueToHomeViewController:]")]
pub fn stub_0x1f854(animated: bool) {
    // IDA 0x1f854: `segueToHomeViewController:` records the animated home
    // segue and dispatches the segue block on main (0x1f854-0x1f8ac,
    // stub_0x1f8b0). The queue hop collapses when the block lands.
    crate::generated_bg_4::record_home_segue(animated);
    LOGIN_HOME_SEGUE_DISPATCHES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x1f8b0 — ___49-[LoginViewController segueToHomeViewController:]_block_invoke
#[doc(alias = "___49-[LoginViewController segueToHomeViewController:]_block_invoke")]
pub fn stub_0x1f8b0(animated: bool, has_memory_warning: bool, fg_x: Option<f32>, bg_x: Option<f32>) {
    // IDA 0x1f8b0: the segue block instantiates the `HomeViewController`
    // from the main storyboard (0x1f8b0-0x1f922, no target here), flags
    // it to segue after load when animated (0x1f924-0x1f93c,
    // stub_0x1d238), and runs the 0.3s animation with the logo fade
    // (0x1f93e-0x1f9d2, stub_0x1fa18) and the present-home completion
    // (stub_0x1fa58). The animated byte crosses as a parameter with the
    // completion's layer queries; both hops collapse to direct calls.
    LOGIN_HOME_INSTANTIATIONS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    if animated {
        crate::generated_bg_3::stub_0x1d238();
    }
    stub_0x1fa18();
    stub_0x1fa58(animated, has_memory_warning, fg_x, bg_x);
}

// 0x1fa18 — ___49-[LoginViewController segueToHomeViewController:]_block_invoke_2
#[doc(alias = "___49-[LoginViewController segueToHomeViewController:]_block_invoke_2")]
pub fn stub_0x1fa18() {
    // IDA 0x1fa18: the segue fade-out zeroes the logo alpha
    // (0x1fa18-0x1fa3e).
    crate::generated_bg_4::LOGIN_LOGO_ALPHA_BITS.store(0, std::sync::atomic::Ordering::SeqCst);
}

// 0x1fa44 — ___copy_helper_block_339
#[doc(alias = "___copy_helper_block_339")]
pub fn stub_0x1fa44(_dst: usize, _src: usize) {
    // IDA 0x1fa44: `__copy_helper_block_339` — one `_Block_object_assign`
    // retain (0x1fa44-0x1fa4a, same shape as stub_0x18094). No explicit
    // body.
}

// 0x1fa50 — ___destroy_helper_block_340
#[doc(alias = "___destroy_helper_block_340")]
pub fn stub_0x1fa50(_block: usize) {
    // IDA 0x1fa50: `__destroy_helper_block_340` — one
    // `_Block_object_dispose` release (0x1fa50-0x1fa54, same shape as
    // stub_0x180a0). No explicit body.
}

// 0x1fa58 — ___49-[LoginViewController segueToHomeViewController:]_block_invoke342
#[doc(alias = "___49-[LoginViewController segueToHomeViewController:]_block_invoke342")]
pub fn stub_0x1fa58(animated: bool, has_memory_warning: bool, fg_x: Option<f32>, bg_x: Option<f32>) {
    // IDA 0x1fa58: the present-home completion stops the background pan
    // (0x1fa58-0x1fa66), captures the foreground/background
    // presentation-layer x into the home controller unless a memory
    // warning was received (0x1fa68-0x1fb2c, defaulting each to 0 on a
    // nil layer), flags the home segue-after-load when animated
    // (0x1fb2e-0x1fb50, stub_0x1d238), and presents the home controller
    // unanimated with the spinner-restore completion (0x1fb52-0x1fbd4,
    // stub_0x1fbd8). Layer queries collapse into parameters.
    LOGIN_BACKGROUND_PAN_STOPS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    if !has_memory_warning {
        crate::generated_bg_3::set_home_image_initial_x(fg_x.unwrap_or(0.0), bg_x.unwrap_or(0.0));
    }
    if animated {
        crate::generated_bg_3::stub_0x1d238();
    }
    LOGIN_HOME_PRESENTATIONS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    stub_0x1fbd8();
}

// 0x1fbd8 — ___49-[LoginViewController segueToHomeViewController:]_block_invoke_2353
#[doc(alias = "___49-[LoginViewController segueToHomeViewController:]_block_invoke_2353")]
pub fn stub_0x1fbd8() {
    // IDA 0x1fbd8: the presented-home completion stops the spinner
    // (0x1fbd8-0x1fbe6, stub_0x1eeac) and runs the button fade-in
    // animation (0x1fbe8-0x1fc5c, stub_0x1fc60). Both hops collapse to
    // direct calls.
    crate::generated_bg_4::stub_0x1eeac();
    stub_0x1fc60();
}
// 0x1fc60 — ___49-[LoginViewController segueToHomeViewController:]_block_invoke_3
#[doc(alias = "___49-[LoginViewController segueToHomeViewController:]_block_invoke_3")]
pub fn stub_0x1fc60() {
    // IDA 0x1fc60: the button fade-in restores the button-view alpha to
    // 1.0 (0x1fc60-0x1fc8c, 0x3f800000).
    LOGIN_BUTTON_ALPHA_BITS.store(0x3f800000, std::sync::atomic::Ordering::SeqCst);
}
// 0x1fc90 — ___copy_helper_block_356
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_356")]
pub fn stub_0x1fc90(_dst: usize, _src: usize) {
    // IDA 0x1fc90: `__copy_helper_block_356` — one `_Block_object_assign`
    // retain (0x1fc90-0x1fc96, same shape as stub_0x18094). No explicit
    // body.
}
// 0x1fc9c — ___destroy_helper_block_357
#[doc(alias = "___destroy_helper_block_357")]
pub fn stub_0x1fc9c(_block: usize) {
    // IDA 0x1fc9c: `__destroy_helper_block_357` — one
    // `_Block_object_dispose` release (0x1fc9c-0x1fca0, same shape as
    // stub_0x180a0). No explicit body.
}
// 0x1fca4 — ___copy_helper_block_359
#[doc(alias = "___copy_helper_block_359")]
pub fn stub_0x1fca4(_dst: usize, _src: usize) {
    // IDA 0x1fca4: `__copy_helper_block_359` — two `_Block_object_assign`
    // retains for the self + home-controller captures (0x1fca4-0x1fcbc,
    // same shape as stub_0x1eb08). No explicit body.
}
// 0x1fcc8 — ___destroy_helper_block_360
#[doc(alias = "___destroy_helper_block_360")]
pub fn stub_0x1fcc8(_block: usize) {
    // IDA 0x1fcc8: `__destroy_helper_block_360` — two
    // `_Block_object_dispose` releases (0x1fcc8-0x1fcda, same shape as
    // stub_0x1eb38). No explicit body.
}
// 0x1fce4 — ___copy_helper_block_364
#[doc(alias = "___copy_helper_block_364")]
pub fn stub_0x1fce4(_dst: usize, _src: usize) {
    // IDA 0x1fce4: `__copy_helper_block_364` — two `_Block_object_assign`
    // retains (0x1fce4-0x1fcfc, same shape as stub_0x1eb08). No explicit
    // body.
}
// 0x1fd08 — ___destroy_helper_block_365
#[doc(alias = "___destroy_helper_block_365")]
pub fn stub_0x1fd08(_block: usize) {
    // IDA 0x1fd08: `__destroy_helper_block_365` — two
    // `_Block_object_dispose` releases (0x1fd08-0x1fd1a, same shape as
    // stub_0x1eb38). No explicit body.
}
// 0x1fd24 — ___copy_helper_block_367
#[doc(alias = "___copy_helper_block_367")]
pub fn stub_0x1fd24(_dst: usize, _src: usize) {
    // IDA 0x1fd24: `__copy_helper_block_367` — one `_Block_object_assign`
    // retain (0x1fd24-0x1fd2a, same shape as stub_0x18094). No explicit
    // body.
}
// 0x1fd30 — ___destroy_helper_block_368
#[doc(alias = "___destroy_helper_block_368")]
pub fn stub_0x1fd30(_block: usize) {
    // IDA 0x1fd30: `__destroy_helper_block_368` — one
    // `_Block_object_dispose` release (0x1fd30-0x1fd34, same shape as
    // stub_0x180a0). No explicit body.
}
// 0x1fd38 — -[LoginViewController prepareForSegue:sender:]
// type: void __cdecl(LoginViewController *self, SEL, id, id)
#[doc(alias = "-[LoginViewController prepareForSegue:sender:]")]
pub fn stub_0x1fd38(
    dest_is_navbar: bool,
    button_tag: Option<i32>,
    is_tablet: bool,
    base_url: &str,
    search_url: &str,
) {
    // IDA 0x1fd38: `prepareForSegue:sender:` ignores non-navbar
    // destinations (0x1fd88 fallthrough). A button sender resolves its
    // tag via `getUrlForButtonTag:recordPageView:` and points the
    // destination at it (0x1fda0-0x1fdf8); other senders skip `setUrl:`
    // (0x1fe04 goto). Every navbar path ends attaching the preloaded web
    // view for the destination URL (0x1fe20-0x1fe62). Class queries
    // collapse into parameters; the cache manager has no target here, so
    // the destination URL + attached URL record. Mirrors the
    // `SegueSender` shape of stub_0x1cfe8.
    if !dest_is_navbar {
        return;
    }
    if let Some(tag) = button_tag {
        let url = crate::generated_bg_3::stub_0x1cc1c(tag, true, is_tablet, base_url, search_url);
        *LOGIN_SEGUE_URL.lock() = url.clone();
        *LOGIN_SEGUE_PRELOADED.lock() = url;
    } else {
        let url = LOGIN_SEGUE_URL.lock().clone();
        *LOGIN_SEGUE_PRELOADED.lock() = url;
    }
}

// 0x1fe70 — -[LoginViewController setLoginPlaceId:]
// type: void __cdecl(LoginViewController *self, SEL, int)
#[doc(alias = "-[LoginViewController setLoginPlaceId:]")]
pub fn stub_0x1fe70(place_id: i32) {
    // IDA 0x1fe70: `setLoginPlaceId:` logs the place id (0x1fe70-0x1fe84,
    // `NSLog`, unmodeled), instantiates the `HomeViewController` and
    // points it at the place (0x1fe86-0x1fefc, stub_0x1d248), flags the
    // guest tap and falls through to `playNowDidTouchUpInside:`
    // (0x1fefe-0x1ff14, stub_0x1f004).
    crate::generated_bg_3::stub_0x1d248(place_id);
    crate::generated_bg_4::USER_DID_CLICK_PLAY_NOW.store(true, std::sync::atomic::Ordering::SeqCst);
    crate::generated_bg_4::stub_0x1f004();
}

// 0x1ff5c — -[LoginViewController username]
// type: UITextField *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController username]")]
pub fn stub_0x1ff5c() -> usize {
    // IDA 0x1ff5c: `username` returns the `username_` ivar
    // (0x1ff5c-0x1ff68). Opaque `id` handle; 0 when unset.
    crate::generated_bg_4::login_outlet("username")
}

// 0x1ff6c — -[LoginViewController setUsername:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setUsername:]")]
pub fn stub_0x1ff6c(handle: usize) {
    // IDA 0x1ff6c: `setUsername:` retains via `objc_setProperty`
    // (0x1ff6c-0x1ff8a). Retain is drop glue; the handle records.
    crate::generated_bg_4::set_login_outlet("username", handle);
}

// 0x1ff90 — -[LoginViewController password]
// type: UITextField *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController password]")]
pub fn stub_0x1ff90() -> usize {
    // IDA 0x1ff90: `password` returns the `password_` ivar (same shape
    // as stub_0x1ff5c). Opaque `id` handle; 0 when unset.
    crate::generated_bg_4::login_outlet("password")
}

// 0x1ffa0 — -[LoginViewController setPassword:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setPassword:]")]
pub fn stub_0x1ffa0(handle: usize) {
    // IDA 0x1ffa0: `setPassword:` retains via `objc_setProperty` (same
    // shape as stub_0x1ff6c). Retain is drop glue; the handle records.
    crate::generated_bg_4::set_login_outlet("password", handle);
}

// 0x1ffc4 — -[LoginViewController btnSkip]
// type: UIButton *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController btnSkip]")]
pub fn stub_0x1ffc4() -> usize {
    // IDA 0x1ffc4: `btnSkip` returns the `_btnSkip` ivar (same shape as
    // stub_0x1ff5c). Opaque `id` handle; 0 when unset.
    crate::generated_bg_4::login_outlet("btnSkip")
}

// 0x1ffd4 — -[LoginViewController setBtnSkip:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setBtnSkip:]")]
pub fn stub_0x1ffd4(handle: usize) {
    // IDA 0x1ffd4: `setBtnSkip:` retains via `objc_setProperty` (same
    // shape as stub_0x1ff6c). Retain is drop glue; the handle records.
    crate::generated_bg_4::set_login_outlet("btnSkip", handle);
}

// 0x1fff8 — -[LoginViewController mainView]
// type: UIView *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController mainView]")]
pub fn stub_0x1fff8() -> usize {
    // IDA 0x1fff8: `mainView` returns the `_mainView` ivar (same shape
    // as stub_0x1ff5c). Opaque `id` handle; 0 when unset.
    crate::generated_bg_4::login_outlet("mainView")
}

// 0x20008 — -[LoginViewController setMainView:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setMainView:]")]
pub fn stub_0x20008(handle: usize) {
    // IDA 0x20008: `setMainView:` retains via `objc_setProperty` (same
    // shape as stub_0x1ff6c). Retain is drop glue; the handle records.
    crate::generated_bg_4::set_login_outlet("mainView", handle);
}

// 0x2002c — -[LoginViewController EnvironmentPicker]
// type: UIPickerView *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController EnvironmentPicker]")]
pub fn stub_0x2002c() -> usize {
    // IDA 0x2002c: `EnvironmentPicker` returns the `_EnvironmentPicker`
    // ivar (same shape as stub_0x1ff5c). Opaque `id` handle; 0 when
    // unset.
    crate::generated_bg_4::login_outlet("EnvironmentPicker")
}

// 0x2003c — -[LoginViewController setEnvironmentPicker:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setEnvironmentPicker:]")]
pub fn stub_0x2003c(handle: usize) {
    // IDA 0x2003c: `setEnvironmentPicker:` retains via `objc_setProperty`
    // (same shape as stub_0x1ff6c). Retain is drop glue; the handle
    // records.
    crate::generated_bg_4::set_login_outlet("EnvironmentPicker", handle);
}

// 0x20060 — -[LoginViewController rememberPwLabel]
// type: UILabel *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController rememberPwLabel]")]
pub fn stub_0x20060() -> usize {
    // IDA 0x20060: `rememberPwLabel` returns the `_rememberPwLabel` ivar
    // (same shape as stub_0x1ff5c). Opaque `id` handle; 0 when unset.
    crate::generated_bg_4::login_outlet("rememberPwLabel")
}

// 0x20070 — -[LoginViewController setRememberPwLabel:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setRememberPwLabel:]")]
pub fn stub_0x20070(handle: usize) {
    // IDA 0x20070: `setRememberPwLabel:` retains via `objc_setProperty`
    // (same shape as stub_0x1ff6c). Retain is drop glue; the handle
    // records.
    crate::generated_bg_4::set_login_outlet("rememberPwLabel", handle);
}

// 0x20094 — -[LoginViewController loginLabel]
// type: UILabel *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController loginLabel]")]
pub fn stub_0x20094() -> usize {
    // IDA 0x20094: `loginLabel` returns the `_loginLabel` ivar (same
    // shape as stub_0x1ff5c). Opaque `id` handle; 0 when unset.
    crate::generated_bg_4::login_outlet("loginLabel")
}

// 0x200a4 — -[LoginViewController setLoginLabel:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setLoginLabel:]")]
pub fn stub_0x200a4(handle: usize) {
    // IDA 0x200a4: `setLoginLabel:` retains via `objc_setProperty` (same
    // shape as stub_0x1ff6c). Retain is drop glue; the handle records.
    crate::generated_bg_4::set_login_outlet("loginLabel", handle);
}

// 0x200c8 — -[LoginViewController signupLabel]
// type: UILabel *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController signupLabel]")]
pub fn stub_0x200c8() -> usize {
    // IDA 0x200c8: `signupLabel` returns the `_signupLabel` ivar (same
    // shape as stub_0x1ff5c). Opaque `id` handle; 0 when unset.
    crate::generated_bg_4::login_outlet("signupLabel")
}

// 0x200d8 — -[LoginViewController setSignupLabel:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setSignupLabel:]")]
pub fn stub_0x200d8(handle: usize) {
    // IDA 0x200d8: `setSignupLabel:` retains via `objc_setProperty`
    // (same shape as stub_0x1ff6c). Retain is drop glue; the handle
    // records.
    crate::generated_bg_4::set_login_outlet("signupLabel", handle);
}

// 0x200fc — -[LoginViewController swiRememberMyPassword]
// type: UISwitch *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController swiRememberMyPassword]")]
pub fn stub_0x200fc() -> usize {
    // IDA 0x200fc: `swiRememberMyPassword` returns the
    // `_swiRememberMyPassword` ivar (same shape as stub_0x1ff5c).
    // Opaque `id` handle; 0 when unset.
    crate::generated_bg_4::login_outlet("swiRememberMyPassword")
}

// 0x2010c — -[LoginViewController setSwiRememberMyPassword:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setSwiRememberMyPassword:]")]
pub fn stub_0x2010c(handle: usize) {
    // IDA 0x2010c: `setSwiRememberMyPassword:` retains via
    // `objc_setProperty` (same shape as stub_0x1ff6c). Retain is drop
    // glue; the handle records.
    crate::generated_bg_4::set_login_outlet("swiRememberMyPassword", handle);
}

// 0x20130 — -[LoginViewController scrollView]
// type: UIScrollView *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController scrollView]")]
pub fn stub_0x20130() -> usize {
    // IDA 0x20130: `scrollView` returns the `_scrollView` ivar (same
    // shape as stub_0x1ff5c). Opaque `id` handle; 0 when unset.
    crate::generated_bg_4::login_outlet("scrollView")
}

// 0x20140 — -[LoginViewController setScrollView:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setScrollView:]")]
pub fn stub_0x20140(handle: usize) {
    // IDA 0x20140: `setScrollView:` retains via `objc_setProperty`
    // (same shape as stub_0x1ff6c). Retain is drop glue; the handle
    // records.
    crate::generated_bg_4::set_login_outlet("scrollView", handle);
}

// 0x20164 — -[LoginViewController imgUsernamePasswordBackground]
// type: UIImageView *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController imgUsernamePasswordBackground]")]
pub fn stub_0x20164() -> usize {
    // IDA 0x20164: `imgUsernamePasswordBackground` returns the
    // `_imgUsernamePasswordBackground` ivar (same shape as stub_0x1ff5c).
    // Opaque `id` handle; 0 when unset.
    crate::generated_bg_4::login_outlet("imgUsernamePasswordBackground")
}

// 0x20174 — -[LoginViewController setImgUsernamePasswordBackground:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setImgUsernamePasswordBackground:]")]
pub fn stub_0x20174(handle: usize) {
    // IDA 0x20174: `setImgUsernamePasswordBackground:` retains via
    // `objc_setProperty` (same shape as stub_0x1ff6c). Retain is drop
    // glue; the handle records.
    crate::generated_bg_4::set_login_outlet("imgUsernamePasswordBackground", handle);
}

// 0x20198 — -[LoginViewController robloxLogo]
// type: UIImageView *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController robloxLogo]")]
pub fn stub_0x20198() -> usize {
    // IDA 0x20198: `robloxLogo` returns the `_robloxLogo` ivar (same
    // shape as stub_0x1ff5c). Opaque `id` handle; 0 when unset.
    crate::generated_bg_4::login_outlet("robloxLogo")
}

// 0x201a8 — -[LoginViewController setRobloxLogo:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setRobloxLogo:]")]
pub fn stub_0x201a8(handle: usize) {
    // IDA 0x201a8: `setRobloxLogo:` retains via `objc_setProperty`
    // (same shape as stub_0x1ff6c). Retain is drop glue; the handle
    // records.
    crate::generated_bg_4::set_login_outlet("robloxLogo", handle);
}

// 0x201cc — -[LoginViewController loginFieldViews]
// type: UIView *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController loginFieldViews]")]
pub fn stub_0x201cc() -> usize {
    // IDA 0x201cc: `loginFieldViews` returns the `_loginFieldViews`
    // ivar (same shape as stub_0x1ff5c). Opaque `id` handle; 0 when
    // unset.
    crate::generated_bg_4::login_outlet("loginFieldViews")
}

// 0x201dc — -[LoginViewController setLoginFieldViews:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setLoginFieldViews:]")]
pub fn stub_0x201dc(handle: usize) {
    // IDA 0x201dc: `setLoginFieldViews:` retains via `objc_setProperty`
    // (same shape as stub_0x1ff6c). Retain is drop glue; the handle
    // records.
    crate::generated_bg_4::set_login_outlet("loginFieldViews", handle);
}

// 0x20200 — -[LoginViewController loginActivityIndicator]
// type: UIActivityIndicatorView *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController loginActivityIndicator]")]
pub fn stub_0x20200() -> usize {
    // IDA 0x20200: `loginActivityIndicator` returns the
    // `_loginActivityIndicator` ivar (same shape as stub_0x1ff5c).
    // Opaque `id` handle; 0 when unset.
    crate::generated_bg_4::login_outlet("loginActivityIndicator")
}

// 0x20210 — -[LoginViewController setLoginActivityIndicator:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setLoginActivityIndicator:]")]
pub fn stub_0x20210(handle: usize) {
    // IDA 0x20210: `setLoginActivityIndicator:` retains via
    // `objc_setProperty` (same shape as stub_0x1ff6c). Retain is drop
    // glue; the handle records.
    crate::generated_bg_4::set_login_outlet("loginActivityIndicator", handle);
}

// 0x20234 — -[LoginViewController aboutButton]
// type: UIButton *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController aboutButton]")]
pub fn stub_0x20234() -> usize {
    // IDA 0x20234: `aboutButton` returns the `_aboutButton` ivar (same
    // shape as stub_0x1ff5c). Opaque `id` handle; 0 when unset.
    crate::generated_bg_4::login_outlet("aboutButton")
}

// 0x20244 — -[LoginViewController setAboutButton:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setAboutButton:]")]
pub fn stub_0x20244(handle: usize) {
    // IDA 0x20244: `setAboutButton:` retains via `objc_setProperty`
    // (same shape as stub_0x1ff6c). Retain is drop glue; the handle
    // records.
    crate::generated_bg_4::set_login_outlet("aboutButton", handle);
}

// 0x20268 — -[LoginViewController playNowLabel]
// type: UILabel *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController playNowLabel]")]
pub fn stub_0x20268() -> usize {
    // IDA 0x20268: `playNowLabel` returns the `_playNowLabel` ivar
    // (0x20268-0x20274). Opaque `id` handle; 0 when unset.
    crate::generated_bg_4::login_outlet("playNowLabel")
}

// 0x20278 — -[LoginViewController setPlayNowLabel:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setPlayNowLabel:]")]
pub fn stub_0x20278(handle: usize) {
    // IDA 0x20278: `setPlayNowLabel:` retains via `objc_setProperty`
    // (0x20278-0x20292). Retain is drop glue; the handle records.
    crate::generated_bg_4::set_login_outlet("playNowLabel", handle);
}

// 0x2029c — -[LoginViewController versionLabel]
// type: UILabel *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController versionLabel]")]
pub fn stub_0x2029c() -> usize {
    // IDA 0x2029c: `versionLabel` returns the `_versionLabel` ivar
    // (same shape as stub_0x1ff5c). Opaque `id` handle; 0 when unset.
    crate::generated_bg_4::login_outlet("versionLabel")
}

// 0x202ac — -[LoginViewController setVersionLabel:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setVersionLabel:]")]
pub fn stub_0x202ac(handle: usize) {
    // IDA 0x202ac: `setVersionLabel:` retains via `objc_setProperty`
    // (same shape as stub_0x1ff6c). Retain is drop glue; the handle
    // records.
    crate::generated_bg_4::set_login_outlet("versionLabel", handle);
}

// 0x202d0 — __GLOBAL__I_a_5
#[doc(alias = "global constructor keyed to_a_5")]
#[doc(alias = "__GLOBAL__I_a_5")]
pub fn stub_0x202d0() {
    // IDA 0x202d0: `__GLOBAL__I_a_5` — stores
    // `boost::system::generic_category()` (x2) / `system_category()`
    // singletons, runs `std::ios_base::Init`, and guards the
    // `exception_ptr` static objects + `singleton_pool` storages
    // (disasm 0x202d4-0x2041e; decompile unavailable, init thunk). Same
    // cutover as stub_0x1d870; no body.
}

// 0x20468 — -[AboutController initWithCoder:]
// type: AboutController *__cdecl(AboutController *self, SEL, id)
#[doc(alias = "-[AboutController initWithCoder:]")]
pub fn stub_0x20468(is_tablet: bool, screen_bounds: Option<(f32, f32, f32, f32)>) {
    // IDA 0x20468: `initWithCoder:` supers `RobloxPageViewController
    // initWithCoder:` (0x20486-0x20490, no target here) and sizes the
    // window: (0, 0, 540, 508) on tablet (0x204c6-0x2050c), else the
    // main-screen bounds, or zero when the screen is nil (0x20510-0x20560).
    // Device/screen queries collapse into parameters, mirroring
    // stub_0x1dd84.
    *ABOUT_WINDOW.lock() = if is_tablet {
        (0.0, 0.0, 540.0, 508.0)
    } else {
        screen_bounds.unwrap_or((0.0, 0.0, 0.0, 0.0))
    };
}

// 0x2057c — -[AboutController dealloc]
// type: void __cdecl(AboutController *self, SEL)
#[doc(alias = "-[AboutController dealloc]")]
pub fn stub_0x2057c() {
    // IDA 0x2057c: `dealloc` releases the seven retained outlets
    // (0x2057c-0x205f8) then super dealloc (0x205fa-0x20604). Release is
    // drop glue; the outlet registry clears.
    ABOUT_OUTLETS.lock().clear();
}

// 0x20644 — -[AboutController viewDidLoad]
// type: void __cdecl(AboutController *self, SEL)
#[doc(alias = "-[AboutController viewDidLoad]")]
pub fn stub_0x20644(
    agreements_html: Option<&str>,
    terms: &str,
    licensing: &str,
    privacy: &str,
    and_word: &str,
    bundle_version: &str,
    is_tablet: bool,
    base_url: &str,
    base_mobile_url: &str,
    about_title: &str,
    close_title: &str,
    clear_cookies_title: &str,
    legal_text: &str,
) {
    // IDA 0x20644: `viewDidLoad` supers (0x2065c-0x20666, no target
    // here), hides the agreement web view (0x2066e-0x2067e), disables
    // its scroll view when present (0x20680-0x206b0), and with
    // `Agreements.html` in the bundle (0x206b2-0x206e2) localizes the
    // template — `TermsOfService`/`LicensingAgreement`/`PrivacyPolicy`/
    // `AndWord` over "Terms of Service"/"Licensing Agreement"/
    // "Privacy Policy"/"and" — and loads it as text/html UTF-8
    // (0x206e4-0x20844). Then the version label from `CFBundleVersion`
    // (0x20850-0x2086c), the domain label from `RbxBaseUrl` on tablet
    // else `RbxBaseMobileUrl` (0x2086e-0x208c4), the `AboutWord` nav
    // title (0x208d2-0x20978), `CloseWord`/`ClearCookiesWord` buttons
    // (0x20986-0x209de) and the `LegalText` body (0x209ea-0x20a36).
    // Bundle/device queries collapse into parameters.
    ABOUT_AGREEMENT_HIDDEN.store(true, std::sync::atomic::Ordering::SeqCst);
    ABOUT_WEB_SCROLL_ENABLED.store(false, std::sync::atomic::Ordering::SeqCst);
    if let Some(html) = agreements_html {
        let page = html
            .replace("Terms of Service", terms)
            .replace("Licensing Agreement", licensing)
            .replace("Privacy Policy", privacy)
            .replace("and", and_word);
        *ABOUT_AGREEMENT_HTML.lock() = page;
    }
    let mut texts = ABOUT_LABEL_TEXTS.lock();
    texts.insert("versionLabel".to_owned(), bundle_version.to_owned());
    texts.insert(
        "domainName".to_owned(),
        if is_tablet { base_url } else { base_mobile_url }.to_owned(),
    );
    texts.insert("navigationTitle".to_owned(), about_title.to_owned());
    texts.insert("closeButton".to_owned(), close_title.to_owned());
    texts.insert("clearCookies".to_owned(), clear_cookies_title.to_owned());
    texts.insert("legalTextView".to_owned(), legal_text.to_owned());
}

// 0x20a7c — -[AboutController viewWillAppear:]
// type: void __cdecl(AboutController *self, SEL, char)
#[doc(alias = "-[AboutController viewWillAppear:]")]
pub fn stub_0x20a7c() {
    // IDA 0x20a7c: `viewWillAppear:` supers (0x20a90-0x20a9c, no target
    // here) and sets the superview bounds from the window rect
    // (0x20aa4-0x20ad8).
    *ABOUT_BOUNDS.lock() = *ABOUT_WINDOW.lock();
}

// 0x20b00 — -[AboutController webViewDidFinishLoad:]
// type: void __cdecl(AboutController *self, SEL, id)
pub fn stub_0x20b00(agreement_handle: usize, loaded_handle: usize) {
    // IDA 0x20b00: `webViewDidFinishLoad:` unhides the agreement web
    // view when it is the loaded view (0x20b00-0x20b1e). Handles cross
    // as parameters for the pointer comparison.
    if agreement_handle == loaded_handle {
        ABOUT_AGREEMENT_HIDDEN.store(false, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x20b28 — -[AboutController webView:shouldStartLoadWithRequest:navigationType:]
// type: char __cdecl(AboutController *self, SEL, id, id, int)
#[doc(alias = "-[AboutController webView:shouldStartLoadWithRequest:navigationType:]")]
pub fn stub_0x20b28(url: Option<&str>) -> bool {
    // IDA 0x20b28: `webView:shouldStartLoadWithRequest:` lets `file`
    // URLs load inline (0x20b28-0x20b7c); anything else segues
    // `AboutToAgreementSegue` with the URL (0x20b7e-0x20ba4) and
    // cancels the load. A missing URL loads (0x20b58).
    match url {
        None => true,
        Some(url) if url.contains("file") => true,
        Some(url) => {
            *ABOUT_AGREEMENT_URL.lock() = url.to_owned();
            ABOUT_TO_AGREEMENT_SEGUES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            false
        }
    }
}

// 0x20bb0 — -[AboutController prepareForSegue:sender:]
// type: void __cdecl(AboutController *self, SEL, id, id)
#[doc(alias = "-[AboutController prepareForSegue:sender:]")]
pub fn stub_0x20bb0(is_about_to_agreement: bool, sender_url: &str) {
    // IDA 0x20bb0: `prepareForSegue:sender:` forwards the sender URL to
    // the destination on the `AboutToAgreementSegue` identifier
    // (0x20bb0-0x20bfa). The identifier query collapses into a
    // parameter.
    if is_about_to_agreement {
        *ABOUT_AGREEMENT_URL.lock() = sender_url.to_owned();
    }
}

// 0x20c14 — -[AboutController closeButtonPressed:]
// type: void __cdecl(AboutController *self, SEL, id)
#[doc(alias = "-[AboutController closeButtonPressed:]")]
pub fn stub_0x20c14() {
    // IDA 0x20c14: `closeButtonPressed:` dismisses animated with no
    // completion (0x20c14-0x20c26).
    ABOUT_DISMISSALS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x20c28 — -[AboutController clearCookiesButtonPressed:]
// type: void __cdecl(AboutController *self, SEL, id)
#[doc(alias = "-[AboutController clearCookiesButtonPressed:]")]
pub fn stub_0x20c28(cookies_cleared_message: &str) {
    // IDA 0x20c28: `clearCookiesButtonPressed:` clears the Roblox
    // cookies (0x20c28-0x20c38, no target here) and shows a
    // `RobloxAlert` with the `CookiesClearedMessage` text
    // (0x20c3a-0x20cb0). The localized message crosses as a parameter.
    ABOUT_COOKIES_CLEARED.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    *LAST_ABOUT_ALERT.lock() = cookies_cleared_message.to_owned();
}

// 0x20cb4 — -[AboutController viewDidUnload]
// type: void __cdecl(AboutController *self, SEL)
#[doc(alias = "-[AboutController viewDidUnload]")]
pub fn stub_0x20cb4() {
    // IDA 0x20cb4: `viewDidUnload` nils the domain-name + clear-cookies
    // outlets (0x20cb4-0x20cd8) then super `viewDidUnload`
    // (0x20cda-0x20ce4). Outlet release is drop glue; the two slots
    // clear.
    ABOUT_OUTLETS.lock().remove("domainName");
    ABOUT_OUTLETS.lock().remove("clearCookies");
}

// 0x20d0c — -[AboutController navigationTitle]
// type: UINavigationItem *__cdecl(AboutController *self, SEL)
#[doc(alias = "-[AboutController navigationTitle]")]
pub fn stub_0x20d0c() -> ! {
    todo!("0x20d0c -[AboutController navigationTitle]")
}

// 0x20d1c — -[AboutController setNavigationTitle:]
// type: void __cdecl(AboutController *self, SEL, id)
#[doc(alias = "-[AboutController setNavigationTitle:]")]
pub fn stub_0x20d1c() -> ! {
    todo!("0x20d1c -[AboutController setNavigationTitle:]")
}

// 0x20d40 — -[AboutController closeButton]
// type: UIBarButtonItem *__cdecl(AboutController *self, SEL)
#[doc(alias = "-[AboutController closeButton]")]
pub fn stub_0x20d40() -> ! {
    todo!("0x20d40 -[AboutController closeButton]")
}

// 0x20d50 — -[AboutController setCloseButton:]
// type: void __cdecl(AboutController *self, SEL, id)
#[doc(alias = "-[AboutController setCloseButton:]")]
pub fn stub_0x20d50() -> ! {
    todo!("0x20d50 -[AboutController setCloseButton:]")
}

// 0x20d74 — -[AboutController clearCookies]
// type: UIBarButtonItem *__cdecl(AboutController *self, SEL)
#[doc(alias = "-[AboutController clearCookies]")]
pub fn stub_0x20d74() -> ! {
    todo!("0x20d74 -[AboutController clearCookies]")
}

// 0x20d84 — -[AboutController setClearCookies:]
// type: void __cdecl(AboutController *self, SEL, id)
#[doc(alias = "-[AboutController setClearCookies:]")]
pub fn stub_0x20d84() -> ! {
    todo!("0x20d84 -[AboutController setClearCookies:]")
}

// 0x20da8 — -[AboutController legalTextView]
// type: UITextView *__cdecl(AboutController *self, SEL)
#[doc(alias = "-[AboutController legalTextView]")]
pub fn stub_0x20da8() -> ! {
    todo!("0x20da8 -[AboutController legalTextView]")
}

// 0x20db8 — -[AboutController setLegalTextView:]
// type: void __cdecl(AboutController *self, SEL, id)
#[doc(alias = "-[AboutController setLegalTextView:]")]
pub fn stub_0x20db8() -> ! {
    todo!("0x20db8 -[AboutController setLegalTextView:]")
}

// 0x20ddc — -[AboutController versionLabel]
// type: UILabel *__cdecl(AboutController *self, SEL)
#[doc(alias = "-[AboutController versionLabel]")]
pub fn stub_0x20ddc() -> ! {
    todo!("0x20ddc -[AboutController versionLabel]")
}

// 0x20dec — -[AboutController setVersionLabel:]
// type: void __cdecl(AboutController *self, SEL, id)
#[doc(alias = "-[AboutController setVersionLabel:]")]
pub fn stub_0x20dec() -> ! {
    todo!("0x20dec -[AboutController setVersionLabel:]")
}

// 0x20e10 — -[AboutController agreementWebView]
// type: UIWebView *__cdecl(AboutController *self, SEL)
#[doc(alias = "-[AboutController agreementWebView]")]
pub fn stub_0x20e10() -> ! {
    todo!("0x20e10 -[AboutController agreementWebView]")
}

// 0x20e20 — -[AboutController setAgreementWebView:]
// type: void __cdecl(AboutController *self, SEL, id)
#[doc(alias = "-[AboutController setAgreementWebView:]")]
pub fn stub_0x20e20() -> ! {
    todo!("0x20e20 -[AboutController setAgreementWebView:]")
}

// 0x20e44 — -[AboutController domainName]
// type: UILabel *__cdecl(AboutController *self, SEL)
#[doc(alias = "-[AboutController domainName]")]
pub fn stub_0x20e44() -> ! {
    todo!("0x20e44 -[AboutController domainName]")
}

// 0x20e54 — -[AboutController setDomainName:]
// type: void __cdecl(AboutController *self, SEL, id)
#[doc(alias = "-[AboutController setDomainName:]")]
pub fn stub_0x20e54() -> ! {
    todo!("0x20e54 -[AboutController setDomainName:]")
}

// 0x20e78 — +[UpgradeCheckHelper getUpgradeCheckHelper]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[UpgradeCheckHelper getUpgradeCheckHelper]")]
pub fn stub_0x20e78() -> ! {
    todo!("0x20e78 +[UpgradeCheckHelper getUpgradeCheckHelper]")
}

// 0x20ed4 — ___43+[UpgradeCheckHelper getUpgradeCheckHelper]_block_invoke
#[doc(alias = "___43+[UpgradeCheckHelper getUpgradeCheckHelper]_block_invoke")]
pub fn stub_0x20ed4() -> ! {
    todo!("0x20ed4 ___43+[UpgradeCheckHelper getUpgradeCheckHelper]_block_invoke")
}

// 0x20f08 — ___copy_helper_block__3
#[doc(alias = "___copy_helper_block__3")]
pub fn stub_0x20f08() -> ! {
    todo!("0x20f08 ___copy_helper_block__3")
}

// 0x20f14 — ___destroy_helper_block__3
#[doc(alias = "___destroy_helper_block__3")]
pub fn stub_0x20f14() -> ! {
    todo!("0x20f14 ___destroy_helper_block__3")
}

// 0x20f1c — -[UpgradeCheckHelper init]
// type: UpgradeCheckHelper *__cdecl(UpgradeCheckHelper *self, SEL)
#[doc(alias = "-[UpgradeCheckHelper init]")]
pub fn stub_0x20f1c() -> ! {
    todo!("0x20f1c -[UpgradeCheckHelper init]")
}

// 0x21038 — -[UpgradeCheckHelper dealloc]
// type: void __cdecl(UpgradeCheckHelper *self, SEL)
#[doc(alias = "-[UpgradeCheckHelper dealloc]")]
pub fn stub_0x21038() -> ! {
    todo!("0x21038 -[UpgradeCheckHelper dealloc]")
}

// 0x210b4 — +[UpgradeCheckHelper getUpgradeUrl]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[UpgradeCheckHelper getUpgradeUrl]")]
pub fn stub_0x210b4() -> ! {
    todo!("0x210b4 +[UpgradeCheckHelper getUpgradeUrl]")
}

// 0x2111c — -[UpgradeCheckHelper getAlertViewButton:]
// type: id __cdecl(UpgradeCheckHelper *self, SEL, id)
#[doc(alias = "-[UpgradeCheckHelper getAlertViewButton:]")]
pub fn stub_0x2111c() -> ! {
    todo!("0x2111c -[UpgradeCheckHelper getAlertViewButton:]")
}

// 0x21254 — -[UpgradeCheckHelper makeUpgradeRequest:]
// type: void __cdecl(UpgradeCheckHelper *self, SEL, id)
#[doc(alias = "-[UpgradeCheckHelper makeUpgradeRequest:]")]
pub fn stub_0x21254() -> ! {
    todo!("0x21254 -[UpgradeCheckHelper makeUpgradeRequest:]")
}

// 0x212cc — +[UpgradeCheckHelper checkForUpdate]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[UpgradeCheckHelper checkForUpdate]")]
pub fn stub_0x212cc() -> ! {
    todo!("0x212cc +[UpgradeCheckHelper checkForUpdate]")
}

// 0x214a4 — -[UpgradeCheckHelper processCheckForUpdateResponse]
// type: void __cdecl(UpgradeCheckHelper *self, SEL)
#[doc(alias = "-[UpgradeCheckHelper processCheckForUpdateResponse]")]
pub fn stub_0x214a4() -> ! {
    todo!("0x214a4 -[UpgradeCheckHelper processCheckForUpdateResponse]")
}

// 0x21abc — ___51-[UpgradeCheckHelper processCheckForUpdateResponse]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___51-[UpgradeCheckHelper processCheckForUpdateResponse]_block_invoke")]
pub fn stub_0x21abc() -> ! {
    todo!("0x21abc ___51-[UpgradeCheckHelper processCheckForUpdateResponse]_block_invoke")
}

// 0x21adc — ___copy_helper_block_132
#[doc(alias = "___copy_helper_block_132")]
pub fn stub_0x21adc() -> ! {
    todo!("0x21adc ___copy_helper_block_132")
}

// 0x21ae8 — ___destroy_helper_block_133
#[doc(alias = "___destroy_helper_block_133")]
pub fn stub_0x21ae8() -> ! {
    todo!("0x21ae8 ___destroy_helper_block_133")
}

// 0x21af0 — ___51-[UpgradeCheckHelper processCheckForUpdateResponse]_block_invoke141
#[doc(alias = "___51-[UpgradeCheckHelper processCheckForUpdateResponse]_block_invoke141")]
pub fn stub_0x21af0() -> ! {
    todo!("0x21af0 ___51-[UpgradeCheckHelper processCheckForUpdateResponse]_block_invoke141")
}

// 0x21b10 — ___copy_helper_block_142
#[doc(alias = "___copy_helper_block_142")]
pub fn stub_0x21b10() -> ! {
    todo!("0x21b10 ___copy_helper_block_142")
}

// 0x21b1c — ___destroy_helper_block_143
#[doc(alias = "___destroy_helper_block_143")]
pub fn stub_0x21b1c() -> ! {
    todo!("0x21b1c ___destroy_helper_block_143")
}

// 0x21b24 — -[UpgradeCheckHelper connection:didReceiveData:]
// type: void __cdecl(UpgradeCheckHelper *self, SEL, id, id)
#[doc(alias = "-[UpgradeCheckHelper connection:didReceiveData:]")]
pub fn stub_0x21b24() -> ! {
    todo!("0x21b24 -[UpgradeCheckHelper connection:didReceiveData:]")
}

// 0x21b58 — -[UpgradeCheckHelper connectionDidFinishLoading:]
// type: void __cdecl(UpgradeCheckHelper *self, SEL, id)
#[doc(alias = "-[UpgradeCheckHelper connectionDidFinishLoading:]")]
pub fn stub_0x21b58() -> ! {
    todo!("0x21b58 -[UpgradeCheckHelper connectionDidFinishLoading:]")
}

// 0x21ba0 — -[UpgradeCheckHelper alertView:clickedButtonAtIndex:]
// type: void __cdecl(UpgradeCheckHelper *self, SEL, id, int)
#[doc(alias = "-[UpgradeCheckHelper alertView:clickedButtonAtIndex:]")]
pub fn stub_0x21ba0() -> ! {
    todo!("0x21ba0 -[UpgradeCheckHelper alertView:clickedButtonAtIndex:]")
}

// 0x21c18 — __GLOBAL__I_a_6
#[doc(alias = "global constructor keyed to_a_6")]
#[doc(alias = "__GLOBAL__I_a_6")]
pub fn stub_0x21c18() -> ! {
    todo!("0x21c18 global constructor keyed to_a_6")
}

// 0x21ce0 — __ZN18iOSSettingsService4InitEv
// type: _DWORD __fastcall(iOSSettingsService *__hidden this)
#[doc(alias = "iOSSettingsService::Init(void)")]
#[doc(alias = "__ZN18iOSSettingsService4InitEv")]
pub fn stub_0x21ce0() -> ! {
    todo!("0x21ce0 iOSSettingsService::Init(void)")
}
