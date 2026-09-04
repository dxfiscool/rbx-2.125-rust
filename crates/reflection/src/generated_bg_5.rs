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
pub fn stub_0x1f8b0() -> ! {
    todo!("0x1f8b0 ___49-[LoginViewController segueToHomeViewController:]_block_invoke")
}

// 0x1fa18 — ___49-[LoginViewController segueToHomeViewController:]_block_invoke_2
#[doc(alias = "___49-[LoginViewController segueToHomeViewController:]_block_invoke_2")]
pub fn stub_0x1fa18() -> ! {
    todo!("0x1fa18 ___49-[LoginViewController segueToHomeViewController:]_block_invoke_2")
}

// 0x1fa44 — ___copy_helper_block_339
#[doc(alias = "___copy_helper_block_339")]
pub fn stub_0x1fa44() -> ! {
    todo!("0x1fa44 ___copy_helper_block_339")
}

// 0x1fa50 — ___destroy_helper_block_340
#[doc(alias = "___destroy_helper_block_340")]
pub fn stub_0x1fa50() -> ! {
    todo!("0x1fa50 ___destroy_helper_block_340")
}

// 0x1fa58 — ___49-[LoginViewController segueToHomeViewController:]_block_invoke342
#[doc(alias = "___49-[LoginViewController segueToHomeViewController:]_block_invoke342")]
pub fn stub_0x1fa58() -> ! {
    todo!("0x1fa58 ___49-[LoginViewController segueToHomeViewController:]_block_invoke342")
}

// 0x1fbd8 — ___49-[LoginViewController segueToHomeViewController:]_block_invoke_2353
#[doc(alias = "___49-[LoginViewController segueToHomeViewController:]_block_invoke_2353")]
pub fn stub_0x1fbd8() -> ! {
    todo!("0x1fbd8 ___49-[LoginViewController segueToHomeViewController:]_block_invoke_2353")
}

// 0x1fc60 — ___49-[LoginViewController segueToHomeViewController:]_block_invoke_3
#[doc(alias = "___49-[LoginViewController segueToHomeViewController:]_block_invoke_3")]
pub fn stub_0x1fc60() -> ! {
    todo!("0x1fc60 ___49-[LoginViewController segueToHomeViewController:]_block_invoke_3")
}

// 0x1fc90 — ___copy_helper_block_356
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_356")]
pub fn stub_0x1fc90() -> ! {
    todo!("0x1fc90 ___copy_helper_block_356")
}

// 0x1fc9c — ___destroy_helper_block_357
#[doc(alias = "___destroy_helper_block_357")]
pub fn stub_0x1fc9c() -> ! {
    todo!("0x1fc9c ___destroy_helper_block_357")
}

// 0x1fca4 — ___copy_helper_block_359
#[doc(alias = "___copy_helper_block_359")]
pub fn stub_0x1fca4() -> ! {
    todo!("0x1fca4 ___copy_helper_block_359")
}

// 0x1fcc8 — ___destroy_helper_block_360
#[doc(alias = "___destroy_helper_block_360")]
pub fn stub_0x1fcc8() -> ! {
    todo!("0x1fcc8 ___destroy_helper_block_360")
}

// 0x1fce4 — ___copy_helper_block_364
#[doc(alias = "___copy_helper_block_364")]
pub fn stub_0x1fce4() -> ! {
    todo!("0x1fce4 ___copy_helper_block_364")
}

// 0x1fd08 — ___destroy_helper_block_365
#[doc(alias = "___destroy_helper_block_365")]
pub fn stub_0x1fd08() -> ! {
    todo!("0x1fd08 ___destroy_helper_block_365")
}

// 0x1fd24 — ___copy_helper_block_367
#[doc(alias = "___copy_helper_block_367")]
pub fn stub_0x1fd24() -> ! {
    todo!("0x1fd24 ___copy_helper_block_367")
}

// 0x1fd30 — ___destroy_helper_block_368
#[doc(alias = "___destroy_helper_block_368")]
pub fn stub_0x1fd30() -> ! {
    todo!("0x1fd30 ___destroy_helper_block_368")
}

// 0x1fd38 — -[LoginViewController prepareForSegue:sender:]
// type: void __cdecl(LoginViewController *self, SEL, id, id)
#[doc(alias = "-[LoginViewController prepareForSegue:sender:]")]
pub fn stub_0x1fd38() -> ! {
    todo!("0x1fd38 -[LoginViewController prepareForSegue:sender:]")
}

// 0x1fe70 — -[LoginViewController setLoginPlaceId:]
// type: void __cdecl(LoginViewController *self, SEL, int)
#[doc(alias = "-[LoginViewController setLoginPlaceId:]")]
pub fn stub_0x1fe70() -> ! {
    todo!("0x1fe70 -[LoginViewController setLoginPlaceId:]")
}

// 0x1ff5c — -[LoginViewController username]
// type: UITextField *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController username]")]
pub fn stub_0x1ff5c() -> ! {
    todo!("0x1ff5c -[LoginViewController username]")
}

// 0x1ff6c — -[LoginViewController setUsername:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setUsername:]")]
pub fn stub_0x1ff6c() -> ! {
    todo!("0x1ff6c -[LoginViewController setUsername:]")
}

// 0x1ff90 — -[LoginViewController password]
// type: UITextField *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController password]")]
pub fn stub_0x1ff90() -> ! {
    todo!("0x1ff90 -[LoginViewController password]")
}

// 0x1ffa0 — -[LoginViewController setPassword:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setPassword:]")]
pub fn stub_0x1ffa0() -> ! {
    todo!("0x1ffa0 -[LoginViewController setPassword:]")
}

// 0x1ffc4 — -[LoginViewController btnSkip]
// type: UIButton *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController btnSkip]")]
pub fn stub_0x1ffc4() -> ! {
    todo!("0x1ffc4 -[LoginViewController btnSkip]")
}

// 0x1ffd4 — -[LoginViewController setBtnSkip:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setBtnSkip:]")]
pub fn stub_0x1ffd4() -> ! {
    todo!("0x1ffd4 -[LoginViewController setBtnSkip:]")
}

// 0x1fff8 — -[LoginViewController mainView]
// type: UIView *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController mainView]")]
pub fn stub_0x1fff8() -> ! {
    todo!("0x1fff8 -[LoginViewController mainView]")
}

// 0x20008 — -[LoginViewController setMainView:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setMainView:]")]
pub fn stub_0x20008() -> ! {
    todo!("0x20008 -[LoginViewController setMainView:]")
}

// 0x2002c — -[LoginViewController EnvironmentPicker]
// type: UIPickerView *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController EnvironmentPicker]")]
pub fn stub_0x2002c() -> ! {
    todo!("0x2002c -[LoginViewController EnvironmentPicker]")
}

// 0x2003c — -[LoginViewController setEnvironmentPicker:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setEnvironmentPicker:]")]
pub fn stub_0x2003c() -> ! {
    todo!("0x2003c -[LoginViewController setEnvironmentPicker:]")
}

// 0x20060 — -[LoginViewController rememberPwLabel]
// type: UILabel *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController rememberPwLabel]")]
pub fn stub_0x20060() -> ! {
    todo!("0x20060 -[LoginViewController rememberPwLabel]")
}

// 0x20070 — -[LoginViewController setRememberPwLabel:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setRememberPwLabel:]")]
pub fn stub_0x20070() -> ! {
    todo!("0x20070 -[LoginViewController setRememberPwLabel:]")
}

// 0x20094 — -[LoginViewController loginLabel]
// type: UILabel *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController loginLabel]")]
pub fn stub_0x20094() -> ! {
    todo!("0x20094 -[LoginViewController loginLabel]")
}

// 0x200a4 — -[LoginViewController setLoginLabel:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setLoginLabel:]")]
pub fn stub_0x200a4() -> ! {
    todo!("0x200a4 -[LoginViewController setLoginLabel:]")
}

// 0x200c8 — -[LoginViewController signupLabel]
// type: UILabel *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController signupLabel]")]
pub fn stub_0x200c8() -> ! {
    todo!("0x200c8 -[LoginViewController signupLabel]")
}

// 0x200d8 — -[LoginViewController setSignupLabel:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setSignupLabel:]")]
pub fn stub_0x200d8() -> ! {
    todo!("0x200d8 -[LoginViewController setSignupLabel:]")
}

// 0x200fc — -[LoginViewController swiRememberMyPassword]
// type: UISwitch *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController swiRememberMyPassword]")]
pub fn stub_0x200fc() -> ! {
    todo!("0x200fc -[LoginViewController swiRememberMyPassword]")
}

// 0x2010c — -[LoginViewController setSwiRememberMyPassword:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setSwiRememberMyPassword:]")]
pub fn stub_0x2010c() -> ! {
    todo!("0x2010c -[LoginViewController setSwiRememberMyPassword:]")
}

// 0x20130 — -[LoginViewController scrollView]
// type: UIScrollView *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController scrollView]")]
pub fn stub_0x20130() -> ! {
    todo!("0x20130 -[LoginViewController scrollView]")
}

// 0x20140 — -[LoginViewController setScrollView:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setScrollView:]")]
pub fn stub_0x20140() -> ! {
    todo!("0x20140 -[LoginViewController setScrollView:]")
}

// 0x20164 — -[LoginViewController imgUsernamePasswordBackground]
// type: UIImageView *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController imgUsernamePasswordBackground]")]
pub fn stub_0x20164() -> ! {
    todo!("0x20164 -[LoginViewController imgUsernamePasswordBackground]")
}

// 0x20174 — -[LoginViewController setImgUsernamePasswordBackground:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setImgUsernamePasswordBackground:]")]
pub fn stub_0x20174() -> ! {
    todo!("0x20174 -[LoginViewController setImgUsernamePasswordBackground:]")
}

// 0x20198 — -[LoginViewController robloxLogo]
// type: UIImageView *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController robloxLogo]")]
pub fn stub_0x20198() -> ! {
    todo!("0x20198 -[LoginViewController robloxLogo]")
}

// 0x201a8 — -[LoginViewController setRobloxLogo:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setRobloxLogo:]")]
pub fn stub_0x201a8() -> ! {
    todo!("0x201a8 -[LoginViewController setRobloxLogo:]")
}

// 0x201cc — -[LoginViewController loginFieldViews]
// type: UIView *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController loginFieldViews]")]
pub fn stub_0x201cc() -> ! {
    todo!("0x201cc -[LoginViewController loginFieldViews]")
}

// 0x201dc — -[LoginViewController setLoginFieldViews:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setLoginFieldViews:]")]
pub fn stub_0x201dc() -> ! {
    todo!("0x201dc -[LoginViewController setLoginFieldViews:]")
}

// 0x20200 — -[LoginViewController loginActivityIndicator]
// type: UIActivityIndicatorView *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController loginActivityIndicator]")]
pub fn stub_0x20200() -> ! {
    todo!("0x20200 -[LoginViewController loginActivityIndicator]")
}

// 0x20210 — -[LoginViewController setLoginActivityIndicator:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setLoginActivityIndicator:]")]
pub fn stub_0x20210() -> ! {
    todo!("0x20210 -[LoginViewController setLoginActivityIndicator:]")
}

// 0x20234 — -[LoginViewController aboutButton]
// type: UIButton *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController aboutButton]")]
pub fn stub_0x20234() -> ! {
    todo!("0x20234 -[LoginViewController aboutButton]")
}

// 0x20244 — -[LoginViewController setAboutButton:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setAboutButton:]")]
pub fn stub_0x20244() -> ! {
    todo!("0x20244 -[LoginViewController setAboutButton:]")
}

// 0x20268 — -[LoginViewController playNowLabel]
// type: UILabel *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController playNowLabel]")]
pub fn stub_0x20268() -> ! {
    todo!("0x20268 -[LoginViewController playNowLabel]")
}

// 0x20278 — -[LoginViewController setPlayNowLabel:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setPlayNowLabel:]")]
pub fn stub_0x20278() -> ! {
    todo!("0x20278 -[LoginViewController setPlayNowLabel:]")
}

// 0x2029c — -[LoginViewController versionLabel]
// type: UILabel *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController versionLabel]")]
pub fn stub_0x2029c() -> ! {
    todo!("0x2029c -[LoginViewController versionLabel]")
}

// 0x202ac — -[LoginViewController setVersionLabel:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setVersionLabel:]")]
pub fn stub_0x202ac() -> ! {
    todo!("0x202ac -[LoginViewController setVersionLabel:]")
}

// 0x202d0 — __GLOBAL__I_a_5
#[doc(alias = "global constructor keyed to_a_5")]
#[doc(alias = "__GLOBAL__I_a_5")]
pub fn stub_0x202d0() -> ! {
    todo!("0x202d0 global constructor keyed to_a_5")
}

// 0x20468 — -[AboutController initWithCoder:]
// type: AboutController *__cdecl(AboutController *self, SEL, id)
#[doc(alias = "-[AboutController initWithCoder:]")]
pub fn stub_0x20468() -> ! {
    todo!("0x20468 -[AboutController initWithCoder:]")
}

// 0x2057c — -[AboutController dealloc]
// type: void __cdecl(AboutController *self, SEL)
#[doc(alias = "-[AboutController dealloc]")]
pub fn stub_0x2057c() -> ! {
    todo!("0x2057c -[AboutController dealloc]")
}

// 0x20644 — -[AboutController viewDidLoad]
// type: void __cdecl(AboutController *self, SEL)
#[doc(alias = "-[AboutController viewDidLoad]")]
pub fn stub_0x20644() -> ! {
    todo!("0x20644 -[AboutController viewDidLoad]")
}

// 0x20a7c — -[AboutController viewWillAppear:]
// type: void __cdecl(AboutController *self, SEL, char)
#[doc(alias = "-[AboutController viewWillAppear:]")]
pub fn stub_0x20a7c() -> ! {
    todo!("0x20a7c -[AboutController viewWillAppear:]")
}

// 0x20b00 — -[AboutController webViewDidFinishLoad:]
// type: void __cdecl(AboutController *self, SEL, id)
#[doc(alias = "-[AboutController webViewDidFinishLoad:]")]
pub fn stub_0x20b00() -> ! {
    todo!("0x20b00 -[AboutController webViewDidFinishLoad:]")
}

// 0x20b28 — -[AboutController webView:shouldStartLoadWithRequest:navigationType:]
// type: char __cdecl(AboutController *self, SEL, id, id, int)
#[doc(alias = "-[AboutController webView:shouldStartLoadWithRequest:navigationType:]")]
pub fn stub_0x20b28() -> ! {
    todo!("0x20b28 -[AboutController webView:shouldStartLoadWithRequest:navigationType:]")
}

// 0x20bb0 — -[AboutController prepareForSegue:sender:]
// type: void __cdecl(AboutController *self, SEL, id, id)
#[doc(alias = "-[AboutController prepareForSegue:sender:]")]
pub fn stub_0x20bb0() -> ! {
    todo!("0x20bb0 -[AboutController prepareForSegue:sender:]")
}

// 0x20c14 — -[AboutController closeButtonPressed:]
// type: void __cdecl(AboutController *self, SEL, id)
#[doc(alias = "-[AboutController closeButtonPressed:]")]
pub fn stub_0x20c14() -> ! {
    todo!("0x20c14 -[AboutController closeButtonPressed:]")
}

// 0x20c28 — -[AboutController clearCookiesButtonPressed:]
// type: void __cdecl(AboutController *self, SEL, id)
#[doc(alias = "-[AboutController clearCookiesButtonPressed:]")]
pub fn stub_0x20c28() -> ! {
    todo!("0x20c28 -[AboutController clearCookiesButtonPressed:]")
}

// 0x20cb4 — -[AboutController viewDidUnload]
// type: void __cdecl(AboutController *self, SEL)
#[doc(alias = "-[AboutController viewDidUnload]")]
pub fn stub_0x20cb4() -> ! {
    todo!("0x20cb4 -[AboutController viewDidUnload]")
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
