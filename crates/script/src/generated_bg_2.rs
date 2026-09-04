// Auto-generated skeletons for rbx-script — script-bg filler EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield|lua (5401 filtered, all already stubbed) — script-bg filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x1f4a0..0x210b4 | script 15763 -> 15863 total (filler 0x1f4a0 asc, not-in-script 69782->69682)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use crate::generated_165::BlockCapture;
use crate::generated_165::stub_0x1cc1c as home_url_for_button_tag;
use crate::generated_bg_1::LoginViewState;

// 0x1f4a0 — ___destroy_helper_block_306
#[doc(alias = "___destroy_helper_block_306")]
pub fn stub_0x1f4a0(slot: &mut BlockCapture) {
    // IDA 0x1f4a0 `__destroy_helper_block_306`: single
    // `_Block_object_dispose` release (0x1f4a4; cf. 0x1f48c).
    *slot = BlockCapture::default();
}

// 0x1f4a8 — -[LoginViewController onKeyboardShow:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController onKeyboardShow:]")]
pub fn stub_0x1f4a8(state: &mut LoginViewState, has_memory_warning: bool) {
    // IDA 0x1f4a8 `-[LoginViewController onKeyboardShow:]`:
    // `scrollView.setContentOffset:(0,112)` (0x1f4d8; 112.0f =
    // `0x42E00000`); unless a memory warning was received, the main-queue
    // block (0x1f51e..0x1f530 -> 0x1f538). The hop is synchronous here.
    state.scroll_offset = Some((0.0, 112.0));
    if !has_memory_warning {
        stub_0x1f538(state);
    }
}

// 0x1f538 — ___38-[LoginViewController onKeyboardShow:]_block_invoke
#[doc(alias = "___38-[LoginViewController onKeyboardShow:]_block_invoke")]
pub fn stub_0x1f538(state: &mut LoginViewState) {
    // IDA 0x1f538 `__38-[...onKeyboardShow:]_block_invoke`: wraps the dim
    // block and the pan-stop block in one animation with completion
    // (0x1f580..0x1f5d6 -> 0x1f5e0/0x1f674). End states apply
    // synchronously here.
    stub_0x1f5e0(state);
    stub_0x1f674(state);
}

// 0x1f5e0 — ___38-[LoginViewController onKeyboardShow:]_block_invoke_2
#[doc(alias = "___38-[LoginViewController onKeyboardShow:]_block_invoke_2")]
pub fn stub_0x1f5e0(state: &mut LoginViewState) {
    // IDA 0x1f5e0 `__38-[...onKeyboardShow:]_block_invoke_2`: dims alpha 0
    // on the background/foreground images and the two `self+160/164`
    // views (0x1f5f4..0x1f63c; cf. 0x1f3f8).
    state.bg_images_dimmed = true;
}

// 0x1f660 — ___copy_helper_block_308
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_308")]
pub fn stub_0x1f660(dst: &mut BlockCapture, src: &BlockCapture) {
    // IDA 0x1f660 `__copy_helper_block_308`: single
    // `_Block_object_assign` retain (0x1f666; cf. 0x1f494).
    *dst = src.clone();
}

// 0x1f66c — ___destroy_helper_block_309
#[doc(alias = "___destroy_helper_block_309")]
pub fn stub_0x1f66c(slot: &mut BlockCapture) {
    // IDA 0x1f66c `__destroy_helper_block_309`: single
    // `_Block_object_dispose` release (0x1f670; cf. 0x1f4a0).
    *slot = BlockCapture::default();
}

// 0x1f674 — ___38-[LoginViewController onKeyboardShow:]_block_invoke311
// type: id __fastcall(int)
#[doc(alias = "___38-[LoginViewController onKeyboardShow:]_block_invoke311")]
pub fn stub_0x1f674(state: &mut LoginViewState) {
    // IDA 0x1f674 `__38-[...onKeyboardShow:]_block_invoke311`:
    // `stopBackgroundPan` completion shim (single `objc_msgSend`).
    state.background_pan_running = false;
}

// 0x1f688 — ___copy_helper_block_314
#[doc(alias = "___copy_helper_block_314")]
pub fn stub_0x1f688(dst: &mut BlockCapture, src: &BlockCapture) {
    // IDA 0x1f688 `__copy_helper_block_314`: single
    // `_Block_object_assign` retain (0x1f68e; cf. 0x1f660).
    *dst = src.clone();
}

// 0x1f694 — ___destroy_helper_block_315
#[doc(alias = "___destroy_helper_block_315")]
pub fn stub_0x1f694(slot: &mut BlockCapture) {
    // IDA 0x1f694 `__destroy_helper_block_315`: single
    // `_Block_object_dispose` release (0x1f698; cf. 0x1f66c).
    *slot = BlockCapture::default();
}

// 0x1f69c — ___copy_helper_block_320
#[doc(alias = "___copy_helper_block_320")]
pub fn stub_0x1f69c(dst: &mut BlockCapture, src: &BlockCapture) {
    // IDA 0x1f69c `__copy_helper_block_320`: single
    // `_Block_object_assign` retain (0x1f6a2; cf. 0x1f688).
    *dst = src.clone();
}

// 0x1f6a8 — ___destroy_helper_block_321
#[doc(alias = "___destroy_helper_block_321")]
pub fn stub_0x1f6a8(slot: &mut BlockCapture) {
    // IDA 0x1f6a8 `__destroy_helper_block_321`: single
    // `_Block_object_dispose` release (0x1f6ac; cf. 0x1f694).
    *slot = BlockCapture::default();
}

// 0x1f6b0 — -[LoginViewController doLoginTransition]
// type: void __cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController doLoginTransition]")]
pub fn stub_0x1f6b0(state: &mut LoginViewState, remember_password: bool, play_now: bool) {
    // IDA 0x1f6b0 `-[LoginViewController doLoginTransition]`: when the
    // password is not remembered, clears the field on the main queue
    // (0x1f6e8..0x1f72e -> 0x1f808); stores `username`/`password` defaults
    // (0x1f752..0x1f7dc); segues home with `userDidClickPlayNow`
    // (0x1f7fc). The hop is synchronous here.
    if !remember_password {
        stub_0x1f808(state);
    }
    state.stored_username = Some(state.username_text.clone());
    state.stored_password = Some(state.password_text.clone());
    state.home_segue_animated = Some(play_now);
}

// 0x1f808 — ___40-[LoginViewController doLoginTransition]_block_invoke
#[doc(alias = "___40-[LoginViewController doLoginTransition]_block_invoke")]
pub fn stub_0x1f808(state: &mut LoginViewState) {
    // IDA 0x1f808 `__40-[...doLoginTransition]_block_invoke`: clears the
    // `self+204` (password) field text (0x1f808; cf. 0x1ed04).
    state.password_text.clear();
}

// 0x1f82c — ___copy_helper_block_323
#[doc(alias = "___copy_helper_block_323")]
pub fn stub_0x1f82c(dst: &mut BlockCapture, src: &BlockCapture) {
    // IDA 0x1f82c `__copy_helper_block_323`: single
    // `_Block_object_assign` retain (0x1f832; cf. 0x1f69c).
    *dst = src.clone();
}

// 0x1f838 — ___destroy_helper_block_324
#[doc(alias = "___destroy_helper_block_324")]
pub fn stub_0x1f838(slot: &mut BlockCapture) {
    // IDA 0x1f838 `__destroy_helper_block_324`: single
    // `_Block_object_dispose` release (0x1f83c; cf. 0x1f6a8).
    *slot = BlockCapture::default();
}

// 0x1f840 — -[LoginViewController externalSegueToHomeViewController:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController externalSegueToHomeViewController:]")]
pub fn stub_0x1f840(state: &mut crate::generated_bg_1::LoginViewState) {
    // IDA 0x1f840 `-[LoginViewController externalSegueToHomeViewController:]`:
    // forwards to `segueToHomeViewController:0` (0x1f84e -> 0x1f854).
    stub_0x1f854(state, false);
}

// 0x1f854 — -[LoginViewController segueToHomeViewController:]
// type: void __cdecl(LoginViewController *self, SEL, char)
#[doc(alias = "-[LoginViewController segueToHomeViewController:]")]
pub fn stub_0x1f854(state: &mut LoginViewState, animated: bool) {
    // IDA 0x1f854 `-[LoginViewController segueToHomeViewController:]`:
    // runs the main-queue block (0x1f888..0x1f8a4 -> 0x1f8b0), capturing
    // the animated flag (0x1f89c). The hop is synchronous here.
    state.home_segue_requested = true;
    state.home_segue_animated = Some(animated);
    stub_0x1f8b0(state, animated);
}

// 0x1f8b0 — ___49-[LoginViewController segueToHomeViewController:]_block_invoke
#[doc(alias = "___49-[LoginViewController segueToHomeViewController:]_block_invoke")]
pub fn stub_0x1f8b0(state: &mut LoginViewState, animated: bool) {
    // IDA 0x1f8b0 `__49-[...segueToHomeViewController:]_block_invoke`:
    // instantiates `HomeViewController` from the main storyboard
    // (0x1f8e6..0x1f954); with animated == 1 the home VC takes
    // `viewMustSegueAfterLoad` (0x1f958..0x1f96c); then the fade +
    // completion animation pair (0x1f9b4..0x1fa0e -> 0x1fa18/0x1fa58).
    // End states apply synchronously here.
    state.home_instantiated = true;
    if animated {
        state.home_segue_after_load = true;
    }
    stub_0x1fa18(state);
    stub_0x1fa58(state, animated, None, None);
}

// 0x1fa18 — ___49-[LoginViewController segueToHomeViewController:]_block_invoke_2
#[doc(alias = "___49-[LoginViewController segueToHomeViewController:]_block_invoke_2")]
pub fn stub_0x1fa18(state: &mut LoginViewState) {
    // IDA 0x1fa18 `__49-[...segueToHomeViewController:]_block_invoke_2`:
    // `robloxLogo.alpha = 0` fade step (0x1fa2a).
    state.logo_dimmed = true;
}

// 0x1fa44 — ___copy_helper_block_339
#[doc(alias = "___copy_helper_block_339")]
pub fn stub_0x1fa44(dst: &mut BlockCapture, src: &BlockCapture) {
    // IDA 0x1fa44 `__copy_helper_block_339`: single
    // `_Block_object_assign` retain (0x1fa4a; cf. 0x1f660).
    *dst = src.clone();
}

// 0x1fa50 — ___destroy_helper_block_340
#[doc(alias = "___destroy_helper_block_340")]
pub fn stub_0x1fa50(slot: &mut BlockCapture) {
    // IDA 0x1fa50 `__destroy_helper_block_340`: single
    // `_Block_object_dispose` release (0x1fa54; cf. 0x1f66c).
    *slot = BlockCapture::default();
}

// 0x1fa58 — ___49-[LoginViewController segueToHomeViewController:]_block_invoke342
#[doc(alias = "___49-[LoginViewController segueToHomeViewController:]_block_invoke342")]
pub fn stub_0x1fa58(state: &mut LoginViewState, animated: bool, fg_x: Option<f32>, bg_x: Option<f32>) {
    // IDA 0x1fa58 `__49-..._block_invoke342`: `stopBackgroundPan`
    // (0x1fa72); unless the pan flag is set (0x1fa86), snapshots the
    // foreground/background presentation-layer X (0x1fa9c..0x1fb62, 0 when
    // the layer is missing); with animated == 1 the home VC takes
    // `viewMustSegueAfterLoad` (0x1fb6a..0x1fb7a); finally presents the
    // home VC unanimated with the follow-up block (0x1fb98..0x1fbd6 ->
    // 0x1fbd8).
    state.background_pan_running = false;
    state.home_fg_x = Some(fg_x.unwrap_or(0.0));
    state.home_bg_x = Some(bg_x.unwrap_or(0.0));
    if animated {
        state.home_segue_after_load = true;
    }
    state.home_presented = true;
    stub_0x1fbd8(state);
}

// 0x1fbd8 — ___49-[LoginViewController segueToHomeViewController:]_block_invoke_2353
#[doc(alias = "___49-[LoginViewController segueToHomeViewController:]_block_invoke_2353")]
pub fn stub_0x1fbd8(state: &mut LoginViewState) {
    // IDA 0x1fbd8 `__49-..._block_invoke_2353`: `stopShowLoggingIn`
    // (0x1fbee -> 0x1eeac), then the 0.3s restore animation block
    // (0x1fc2a..0x1fc56 -> 0x1fc60). The end state applies synchronously
    // here.
    crate::generated_bg_1::stub_0x1eeac(state);
    stub_0x1fc60(state);
}

// 0x1fc60 — ___49-[LoginViewController segueToHomeViewController:]_block_invoke_3
#[doc(alias = "___49-[LoginViewController segueToHomeViewController:]_block_invoke_3")]
pub fn stub_0x1fc60(state: &mut LoginViewState) {
    // IDA 0x1fc60 `__49-..._block_invoke_3`: `buttonView.alpha = 1.0`
    // restore step (0x1fc72; cf. 0x1c5c8).
    state.button_alpha_one = true;
}

// 0x1fc90 — ___copy_helper_block_356
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_356")]
pub fn stub_0x1fc90(dst: &mut BlockCapture, src: &BlockCapture) {
    // IDA 0x1fc90 `__copy_helper_block_356`: single
    // `_Block_object_assign` retain (0x1fc96; cf. 0x1fa44).
    *dst = src.clone();
}

// 0x1fc9c — ___destroy_helper_block_357
#[doc(alias = "___destroy_helper_block_357")]
pub fn stub_0x1fc9c(slot: &mut BlockCapture) {
    // IDA 0x1fc9c `__destroy_helper_block_357`: single
    // `_Block_object_dispose` release (0x1fca0; cf. 0x1fa50).
    *slot = BlockCapture::default();
}

// 0x1fca4 — ___copy_helper_block_359
#[doc(alias = "___copy_helper_block_359")]
pub fn stub_0x1fca4(dst: &mut crate::generated_bg_1::SegueCaptures, src: &crate::generated_bg_1::SegueCaptures) {
    // IDA 0x1fca4 `__copy_helper_block_359`: `_Block_object_assign`
    // retain of the two captures (0x1fcb4..0x1fcc4; cf. 0x1ec44).
    *dst = src.clone();
}

// 0x1fcc8 — ___destroy_helper_block_360
#[doc(alias = "___destroy_helper_block_360")]
pub fn stub_0x1fcc8(slot: &mut crate::generated_bg_1::SegueCaptures) {
    // IDA 0x1fcc8 `__destroy_helper_block_360`: `_Block_object_dispose`
    // release of the two captures (0x1fcd2..0x1fcde; cf. 0x1ec68).
    *slot = crate::generated_bg_1::SegueCaptures::default();
}

// 0x1fce4 — ___copy_helper_block_364
#[doc(alias = "___copy_helper_block_364")]
pub fn stub_0x1fce4(dst: &mut crate::generated_bg_1::SegueCaptures, src: &crate::generated_bg_1::SegueCaptures) {
    // IDA 0x1fce4 `__copy_helper_block_364`: `_Block_object_assign`
    // retain of the two captures (0x1fcf4..0x1fd04; cf. 0x1fca4).
    *dst = src.clone();
}

// 0x1fd08 — ___destroy_helper_block_365
#[doc(alias = "___destroy_helper_block_365")]
pub fn stub_0x1fd08(slot: &mut crate::generated_bg_1::SegueCaptures) {
    // IDA 0x1fd08 `__destroy_helper_block_365`: `_Block_object_dispose`
    // release of the two captures (0x1fd12..0x1fd1e; cf. 0x1fcc8).
    *slot = crate::generated_bg_1::SegueCaptures::default();
}

// 0x1fd24 — ___copy_helper_block_367
#[doc(alias = "___copy_helper_block_367")]
pub fn stub_0x1fd24(dst: &mut BlockCapture, src: &BlockCapture) {
    // IDA 0x1fd24 `__copy_helper_block_367`: single
    // `_Block_object_assign` retain (0x1fd2a; cf. 0x1fc90).
    *dst = src.clone();
}

// 0x1fd30 — ___destroy_helper_block_368
#[doc(alias = "___destroy_helper_block_368")]
pub fn stub_0x1fd30(slot: &mut BlockCapture) {
    // IDA 0x1fd30 `__destroy_helper_block_368`: single
    // `_Block_object_dispose` release (0x1fd34; cf. 0x1fc9c).
    *slot = BlockCapture::default();
}

// 0x1fd38 — -[LoginViewController prepareForSegue:sender:]
// type: void __cdecl(LoginViewController *self, SEL, id, id)
#[doc(alias = "-[LoginViewController prepareForSegue:sender:]")]
pub fn stub_0x1fd38(
    state: &mut LoginViewState,
    dest_is_nav_bar: bool,
    sender_tag: Option<i32>,
    base_url: &str,
    search_url: &str,
    is_tablet: bool,
) {
    // IDA 0x1fd38 `-[LoginViewController prepareForSegue:sender:]`: only
    // `RobloxNavBarViewController` destinations are handled (0x1fd74..0x1fd8e).
    // A button sender resolves via `+[HomeViewController
    // getUrlForButtonTag:recordPageView:]` (0x1fdac..0x1fe04); other
    // senders leave the inherited URL. Either way the destination gets
    // the preloaded web view (0x1fe24..0x1fe64; cf. 0x1cfe8).
    if !dest_is_nav_bar {
        return;
    }
    if let Some(tag) = sender_tag {
        let (url, _) = home_url_for_button_tag(base_url, search_url, tag, true, is_tablet);
        state.nav_web_url = url;
    }
    state.preloaded_webview = true;
}

// 0x1fe70 — -[LoginViewController setLoginPlaceId:]
// type: void __cdecl(LoginViewController *self, SEL, int)
#[doc(alias = "-[LoginViewController setLoginPlaceId:]")]
pub fn stub_0x1fe70(state: &mut LoginViewState, place_id: i32) {
    // IDA 0x1fe70 `-[LoginViewController setLoginPlaceId:]`: logs the id
    // (0x1fe88), instantiates the Home VC from the storyboard and sets
    // its `jumpToPlaceID:` (0x1feb4..0x1ff2c), sets `userDidClickPlayNow`
    // (0x1ff46), then `playNowDidTouchUpInside:` (0x1ff56 -> 0x1f004).
    state.login_place_id = Some(place_id);
    state.play_now_flag = true;
    crate::generated_bg_1::stub_0x1f004(state);
}

// 0x1ff5c — -[LoginViewController username]
// type: UITextField *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController username]")]
pub fn stub_0x1ff5c(state: &LoginViewState) -> String {
    // IDA 0x1ff5c `-[LoginViewController username]`: GET (disasm
    // `_username` IVAR load); the host models the field by its text
    // content (cf. 0x1d2f4).
    state.username_text.clone()
}

// 0x1ff6c — -[LoginViewController setUsername:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setUsername:]")]
pub fn stub_0x1ff6c(state: &mut LoginViewState, text: &str) {
    // IDA 0x1ff6c `-[LoginViewController setUsername:]`: SET (disasm
    // `objc_setProperty` prologue); the host models the field by its text
    // content (cf. 0x1d304).
    state.username_text = text.to_string();
}

// 0x1ff90 — -[LoginViewController password]
// type: UITextField *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController password]")]
pub fn stub_0x1ff90(state: &LoginViewState) -> String {
    // IDA 0x1ff90 `-[LoginViewController password]`: GET (disasm
    // `_password` IVAR load); the host models the field by its text
    // content (cf. 0x1ff5c).
    state.password_text.clone()
}

// 0x1ffa0 — -[LoginViewController setPassword:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setPassword:]")]
pub fn stub_0x1ffa0(state: &mut LoginViewState, text: &str) {
    // IDA 0x1ffa0 `-[LoginViewController setPassword:]`: SET (disasm
    // `objc_setProperty` prologue); the host models the field by its text
    // content (cf. 0x1ff6c).
    state.password_text = text.to_string();
}

// 0x1ffc4 — -[LoginViewController btnSkip]
// type: UIButton *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController btnSkip]")]
pub fn stub_0x1ffc4(state: &LoginViewState) -> Option<u32> {
    // IDA 0x1ffc4 `-[LoginViewController btnSkip]`: GET (disasm
    // `_btnSkip` IVAR load); opaque platform handle on the host.
    state.outlets.get("btnSkip").copied().flatten()
}

// 0x1ffd4 — -[LoginViewController setBtnSkip:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setBtnSkip:]")]
pub fn stub_0x1ffd4(state: &mut LoginViewState, view: Option<u32>) {
    // IDA 0x1ffd4 `-[LoginViewController setBtnSkip:]`: SET (disasm
    // `objc_setProperty` prologue); host ownership is the outlet slot.
    state.outlets.insert("btnSkip".to_string(), view);
}

// 0x1fff8 — -[LoginViewController mainView]
// type: UIView *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController mainView]")]
pub fn stub_0x1fff8(state: &LoginViewState) -> Option<u32> {
    // IDA 0x1fff8 `-[LoginViewController mainView]`: GET (disasm
    // `_mainView` IVAR load); opaque platform handle on the host.
    state.outlets.get("mainView").copied().flatten()
}

// 0x20008 — -[LoginViewController setMainView:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setMainView:]")]
pub fn stub_0x20008(state: &mut LoginViewState, view: Option<u32>) {
    // IDA 0x20008 `-[LoginViewController setMainView:]`: SET (disasm
    // `objc_setProperty` prologue); host ownership is the outlet slot.
    state.outlets.insert("mainView".to_string(), view);
}

// 0x2002c — -[LoginViewController EnvironmentPicker]
// type: UIPickerView *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController EnvironmentPicker]")]
pub fn stub_0x2002c(state: &LoginViewState) -> Option<u32> {
    // IDA 0x2002c `-[LoginViewController EnvironmentPicker]`: GET (disasm
    // `_EnvironmentPicker` IVAR load); the handle — rows live in `envs`
    // (cf. 0x1dd84).
    state.outlets.get("EnvironmentPicker").copied().flatten()
}

// 0x2003c — -[LoginViewController setEnvironmentPicker:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setEnvironmentPicker:]")]
pub fn stub_0x2003c(state: &mut LoginViewState, view: Option<u32>) {
    // IDA 0x2003c `-[LoginViewController setEnvironmentPicker:]`: SET
    // (disasm `objc_setProperty` prologue); host ownership is the outlet
    // slot.
    state.outlets.insert("EnvironmentPicker".to_string(), view);
}

// 0x20060 — -[LoginViewController rememberPwLabel]
// type: UILabel *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController rememberPwLabel]")]
pub fn stub_0x20060(state: &LoginViewState) -> Option<u32> {
    // IDA 0x20060 `-[LoginViewController rememberPwLabel]`: GET (disasm
    // `_rememberPwLabel` IVAR load); the handle — the text lives in the
    // `labels` table (cf. 0x1e2ec).
    state.outlets.get("rememberPwLabel").copied().flatten()
}

// 0x20070 — -[LoginViewController setRememberPwLabel:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setRememberPwLabel:]")]
pub fn stub_0x20070(state: &mut LoginViewState, view: Option<u32>) {
    // IDA 0x20070 `-[LoginViewController setRememberPwLabel:]`: SET
    // (disasm `objc_setProperty` prologue); host ownership is the outlet
    // slot.
    state.outlets.insert("rememberPwLabel".to_string(), view);
}

// 0x20094 — -[LoginViewController loginLabel]
// type: UILabel *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController loginLabel]")]
pub fn stub_0x20094(state: &LoginViewState) -> Option<u32> {
    // IDA 0x20094 `-[LoginViewController loginLabel]`: GET (disasm
    // `_loginLabel` IVAR load); the handle — the text lives in the
    // `labels` table (cf. 0x1e2ec).
    state.outlets.get("loginLabel").copied().flatten()
}

// 0x200a4 — -[LoginViewController setLoginLabel:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setLoginLabel:]")]
pub fn stub_0x200a4(state: &mut LoginViewState, view: Option<u32>) {
    // IDA 0x200a4 `-[LoginViewController setLoginLabel:]`: SET (disasm
    // `objc_setProperty` prologue); host ownership is the outlet slot.
    state.outlets.insert("loginLabel".to_string(), view);
}

// 0x200c8 — -[LoginViewController signupLabel]
// type: UILabel *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController signupLabel]")]
pub fn stub_0x200c8(state: &LoginViewState) -> Option<u32> {
    // IDA 0x200c8 `-[LoginViewController signupLabel]`: GET (disasm
    // `_signupLabel` IVAR load); the handle — the text lives in the
    // `labels` table (cf. 0x1e2ec).
    state.outlets.get("signupLabel").copied().flatten()
}

// 0x200d8 — -[LoginViewController setSignupLabel:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setSignupLabel:]")]
pub fn stub_0x200d8(state: &mut LoginViewState, view: Option<u32>) {
    // IDA 0x200d8 `-[LoginViewController setSignupLabel:]`: SET (disasm
    // `objc_setProperty` prologue); host ownership is the outlet slot.
    state.outlets.insert("signupLabel".to_string(), view);
}

// 0x200fc — -[LoginViewController swiRememberMyPassword]
// type: UISwitch *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController swiRememberMyPassword]")]
pub fn stub_0x200fc(state: &LoginViewState) -> Option<u32> {
    // IDA 0x200fc `-[LoginViewController swiRememberMyPassword]`: GET
    // (disasm `_swiRememberMyPassword` IVAR load); the on/off state is
    // `remember_switch_on` (cf. 0x1e764).
    state.outlets.get("swiRememberMyPassword").copied().flatten()
}

// 0x2010c — -[LoginViewController setSwiRememberMyPassword:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setSwiRememberMyPassword:]")]
pub fn stub_0x2010c(state: &mut LoginViewState, view: Option<u32>) {
    // IDA 0x2010c `-[LoginViewController setSwiRememberMyPassword:]`: SET
    // (disasm `objc_setProperty` prologue); host ownership is the outlet
    // slot.
    state.outlets.insert("swiRememberMyPassword".to_string(), view);
}

// 0x20130 — -[LoginViewController scrollView]
// type: UIScrollView *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController scrollView]")]
pub fn stub_0x20130(state: &LoginViewState) -> Option<u32> {
    // IDA 0x20130 `-[LoginViewController scrollView]`: GET (disasm
    // `_scrollView` IVAR load); the offset is `scroll_offset` (cf.
    // 0x1f2e0).
    state.outlets.get("scrollView").copied().flatten()
}

// 0x20140 — -[LoginViewController setScrollView:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setScrollView:]")]
pub fn stub_0x20140(state: &mut LoginViewState, view: Option<u32>) {
    // IDA 0x20140 `-[LoginViewController setScrollView:]`: SET (disasm
    // `objc_setProperty` prologue); host ownership is the outlet slot.
    state.outlets.insert("scrollView".to_string(), view);
}

// 0x20164 — -[LoginViewController imgUsernamePasswordBackground]
// type: UIImageView *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController imgUsernamePasswordBackground]")]
pub fn stub_0x20164(state: &LoginViewState) -> Option<u32> {
    // IDA 0x20164 `-[LoginViewController imgUsernamePasswordBackground]`:
    // GET (disasm `_imgUsernamePasswordBackground` IVAR load); opaque
    // platform handle on the host.
    state.outlets.get("imgUsernamePasswordBackground").copied().flatten()
}

// 0x20174 — -[LoginViewController setImgUsernamePasswordBackground:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setImgUsernamePasswordBackground:]")]
pub fn stub_0x20174(state: &mut LoginViewState, view: Option<u32>) {
    // IDA 0x20174 `-[LoginViewController
    // setImgUsernamePasswordBackground:]`: SET (disasm `objc_setProperty`
    // prologue); host ownership is the outlet slot.
    state.outlets.insert("imgUsernamePasswordBackground".to_string(), view);
}

// 0x20198 — -[LoginViewController robloxLogo]
// type: UIImageView *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController robloxLogo]")]
pub fn stub_0x20198(state: &LoginViewState) -> Option<u32> {
    // IDA 0x20198 `-[LoginViewController robloxLogo]`: GET (disasm
    // `_robloxLogo` IVAR load); alpha tracked by `logo_alpha_reset` /
    // `logo_dimmed` (cf. 0x1e1b4/0x1fa18).
    state.outlets.get("robloxLogo").copied().flatten()
}

// 0x201a8 — -[LoginViewController setRobloxLogo:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setRobloxLogo:]")]
pub fn stub_0x201a8(state: &mut LoginViewState, view: Option<u32>) {
    // IDA 0x201a8 `-[LoginViewController setRobloxLogo:]`: SET (disasm
    // `objc_setProperty` prologue); host ownership is the outlet slot.
    state.outlets.insert("robloxLogo".to_string(), view);
}

// 0x201cc — -[LoginViewController loginFieldViews]
// type: UIView *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController loginFieldViews]")]
pub fn stub_0x201cc(state: &LoginViewState) -> Option<u32> {
    // IDA 0x201cc `-[LoginViewController loginFieldViews]`: GET (disasm
    // `_loginFieldViews` IVAR load); alpha tracked by `fields_alpha_zero`
    // (cf. 0x1ee58).
    state.outlets.get("loginFieldViews").copied().flatten()
}

// 0x201dc — -[LoginViewController setLoginFieldViews:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setLoginFieldViews:]")]
pub fn stub_0x201dc(state: &mut LoginViewState, view: Option<u32>) {
    // IDA 0x201dc `-[LoginViewController setLoginFieldViews:]`: SET
    // (disasm `objc_setProperty` prologue); host ownership is the outlet
    // slot.
    state.outlets.insert("loginFieldViews".to_string(), view);
}

// 0x20200 — -[LoginViewController loginActivityIndicator]
// type: UIActivityIndicatorView *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController loginActivityIndicator]")]
pub fn stub_0x20200(state: &LoginViewState) -> Option<u32> {
    // IDA 0x20200 `-[LoginViewController loginActivityIndicator]`: GET
    // (disasm `_loginActivityIndicator` IVAR load); visibility tracked by
    // `activity_shown` (cf. 0x1edbc).
    state.outlets.get("loginActivityIndicator").copied().flatten()
}

// 0x20210 — -[LoginViewController setLoginActivityIndicator:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setLoginActivityIndicator:]")]
pub fn stub_0x20210(state: &mut LoginViewState, view: Option<u32>) {
    // IDA 0x20210 `-[LoginViewController setLoginActivityIndicator:]`:
    // SET (disasm `objc_setProperty` prologue); host ownership is the
    // outlet slot.
    state.outlets.insert("loginActivityIndicator".to_string(), view);
}

// 0x20234 — -[LoginViewController aboutButton]
// type: UIButton *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController aboutButton]")]
pub fn stub_0x20234(state: &LoginViewState) -> Option<u32> {
    // IDA 0x20234 `-[LoginViewController aboutButton]`: GET (disasm
    // `_aboutButton` IVAR load); visibility tracked by `about_hidden`
    // (cf. 0x1ed44).
    state.outlets.get("aboutButton").copied().flatten()
}

// 0x20244 — -[LoginViewController setAboutButton:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setAboutButton:]")]
pub fn stub_0x20244(state: &mut LoginViewState, view: Option<u32>) {
    // IDA 0x20244 `-[LoginViewController setAboutButton:]`: SET (disasm
    // `objc_setProperty` prologue); host ownership is the outlet slot.
    state.outlets.insert("aboutButton".to_string(), view);
}

// 0x20268 — -[LoginViewController playNowLabel]
// type: UILabel *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController playNowLabel]")]
pub fn stub_0x20268(state: &LoginViewState) -> Option<u32> {
    // IDA 0x20268 `-[LoginViewController playNowLabel]`: GET (disasm
    // `_playNowLabel` IVAR load); the handle — the text lives in the
    // `labels` table (cf. 0x1e2ec).
    state.outlets.get("playNowLabel").copied().flatten()
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
