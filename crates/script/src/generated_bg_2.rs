// Auto-generated skeletons for rbx-script — script-bg filler EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield|lua (5401 filtered, all already stubbed) — script-bg filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x1f4a0..0x210b4 | script 15763 -> 15863 total (filler 0x1f4a0 asc, not-in-script 69782->69682)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use crate::generated_165::{AlertButton, BlockCapture, UpgradeCheckRegistry, UpgradeCheckState};
use crate::generated_165::stub_0x1cc1c as home_url_for_button_tag;
use crate::generated_bg_1::LoginViewState;
use std::collections::HashMap;

/// Host-side `AboutController` state (AboutController.m, IDA 0x20468..0x20cb4).
/// UIKit views live on the platform side; only the observable latches are
/// modeled here.
#[derive(Debug, Clone, Default)]
pub struct AboutState {
    /// Settings window frame x/y/w/h (IDA 0x20512/0x2056c; cf. 0x1a970).
    pub window_frame: [f32; 4],
    /// `agreementWebView.hidden = YES` at load (IDA 0x2068e).
    pub agreement_hidden: bool,
    /// Agreement scroll view disabled (IDA 0x206bc..0x206ce).
    pub agreement_scroll_disabled: bool,
    /// Localized `Agreements.html` loaded into the web view (IDA
    /// 0x2071e..0x208a2).
    pub agreement_html_loaded: bool,
    /// Terms/Licensing/Privacy/And replacements (IDA 0x2077a..0x2083c).
    pub replacements: HashMap<String, String>,
    /// `CFBundleVersion` stamp (IDA 0x208bc..0x208ee).
    pub version_text: Option<String>,
    /// `RbxBaseUrl`/`RbxBaseMobileUrl` domain text (IDA 0x20924..0x20954).
    pub domain_text: Option<String>,
    /// `AboutWord` nav title (IDA 0x2096c..0x209b2).
    pub nav_title: Option<String>,
    /// `CloseWord` button title (IDA 0x209c8..0x209f8).
    pub close_title: Option<String>,
    /// `ClearCookiesWord` button title (IDA 0x20a0e..0x20a34).
    pub clear_cookies_title: Option<String>,
    /// `LegalText` view text (IDA 0x20a48..0x20a6e).
    pub legal_text: Option<String>,
    /// Last `setBounds:` pushed to the superview (IDA 0x20ab8..0x20af4).
    pub last_bounds_set: Option<[f32; 4]>,
    /// Agreement web view unhidden on finish (IDA 0x20b10..0x20b24).
    pub agreement_visible: bool,
    /// `AboutToAgreementSegue` segue + URL (IDA 0x20b9c/0x20bfa..0x20c10).
    pub segue: Option<(String, String)>,
    /// `dismissViewControllerAnimated:` ran (IDA 0x20c24).
    pub dismissed: bool,
    /// `clearAllRobloxCookie` ran (IDA 0x20c46).
    pub cookies_cleared: bool,
    /// Last `RobloxAlertWithMessage:` key (IDA 0x20cb0).
    pub last_alert: Option<String>,
    /// Remaining outlet handles by ivar name.
    pub outlets: HashMap<String, Option<u32>>,
    pub view_loaded: bool,
}

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
pub fn stub_0x20278(state: &mut LoginViewState, view: Option<u32>) {
    // IDA 0x20278 `-[LoginViewController setPlayNowLabel:]`: SET (disasm
    // `objc_setProperty` prologue); host ownership is the outlet slot.
    state.outlets.insert("playNowLabel".to_string(), view);
}

// 0x2029c — -[LoginViewController versionLabel]
// type: UILabel *__cdecl(LoginViewController *self, SEL)
#[doc(alias = "-[LoginViewController versionLabel]")]
pub fn stub_0x2029c(state: &LoginViewState) -> Option<u32> {
    // IDA 0x2029c `-[LoginViewController versionLabel]`: GET (disasm
    // `_versionLabel` IVAR load); the handle — the text is `version_text`
    // (cf. 0x1e2ec).
    state.outlets.get("versionLabel").copied().flatten()
}

// 0x202ac — -[LoginViewController setVersionLabel:]
// type: void __cdecl(LoginViewController *self, SEL, id)
#[doc(alias = "-[LoginViewController setVersionLabel:]")]
pub fn stub_0x202ac(state: &mut LoginViewState, view: Option<u32>) {
    // IDA 0x202ac `-[LoginViewController setVersionLabel:]`: SET (disasm
    // `objc_setProperty` prologue); host ownership is the outlet slot.
    state.outlets.insert("versionLabel".to_string(), view);
}

// 0x202d0 — __GLOBAL__I_a_5
#[doc(alias = "global constructor keyed to_a_5")]
pub fn stub_0x202d0() {
    // IDA 0x202d0 `__GLOBAL__I_a_5`: same `generic_category` x2 +
    // `system_category` merged-globals init as 0x1d870 (disasm GLOBAL;
    // cf. 0x16e4c). Host error categories need no init beyond `std::io`.
}

// 0x20468 — -[AboutController initWithCoder:]
// type: AboutController *__cdecl(AboutController *self, SEL, id)
#[doc(alias = "-[AboutController initWithCoder:]")]
pub fn stub_0x20468(is_pad: bool, screen_bounds: [f32; 4]) -> AboutState {
    // IDA 0x20468 `-[AboutController initWithCoder:]`: super
    // `RobloxPageViewController` init (0x20486..0x20494); on iPad the
    // frame is 540x508 at origin (0x204fc..0x20518), otherwise the
    // main-screen bounds (0x20544..0x2056c; cf. 0x1a970).
    AboutState {
        window_frame: if is_pad { [0.0, 0.0, 540.0, 508.0] } else { screen_bounds },
        ..AboutState::default()
    }
}

// 0x2057c — -[AboutController dealloc]
// type: void __cdecl(AboutController *self, SEL)
#[doc(alias = "-[AboutController dealloc]")]
pub fn stub_0x2057c(state: &mut AboutState) {
    // IDA 0x2057c `-[AboutController dealloc]`: releases the seven
    // outlets (`versionLabel`, `agreementWebView`, `navigationTitle`,
    // `closeButton`, `legalTextView`, `domainName`, `clearCookies` —
    // 0x205a0..0x20618) then super dealloc (0x20630..0x2063a, host Drop
    // glue). The owned state folds back to default.
    *state = AboutState::default();
}

// 0x20644 — -[AboutController viewDidLoad]
// type: void __cdecl(AboutController *self, SEL)
#[doc(alias = "-[AboutController viewDidLoad]")]
pub fn stub_0x20644(state: &mut AboutState, has_agreements: bool, is_tablet: bool, bundle_version: &str, domain: &str) {
    // IDA 0x20644 `-[AboutController viewDidLoad]`: super (0x20664..0x2066e),
    // hide agreement view + disable its scroll (0x2068e..0x206ce), load
    // and localize `Agreements.html` (0x206f2..0x208a2),
    // `CFBundleVersion` stamp (0x208bc..0x208ee), `RbxBaseUrl` on tablets
    // else `RbxBaseMobileUrl` (0x2090e..0x20954), About/Close/ClearCookies
    // titles + legal text (0x2096c..0x20a6e).
    state.agreement_hidden = true;
    state.agreement_scroll_disabled = true;
    if has_agreements {
        state.agreement_html_loaded = true;
        for (from, to) in [
            ("Terms of Service", "TermsOfService"),
            ("Licensing Agreement", "LicensingAgreement"),
            ("Privacy Policy", "PrivacyPolicy"),
            ("and", "AndWord"),
        ] {
            state.replacements.insert(from.to_string(), to.to_string());
        }
    }
    state.version_text = Some(bundle_version.to_string());
    state.domain_text = Some(domain.to_string());
    let _ = is_tablet;
    state.nav_title = Some("AboutWord".to_string());
    state.close_title = Some("CloseWord".to_string());
    state.clear_cookies_title = Some("ClearCookiesWord".to_string());
    state.legal_text = Some("LegalText".to_string());
    state.view_loaded = true;
}

// 0x20a7c — -[AboutController viewWillAppear:]
// type: void __cdecl(AboutController *self, SEL, char)
#[doc(alias = "-[AboutController viewWillAppear:]")]
pub fn stub_0x20a7c(state: &mut AboutState, animated: bool) {
    // IDA 0x20a7c `-[AboutController viewWillAppear:]`: super
    // `RobloxPageViewController` call (0x20a9c..0x20aa6, host UIKit), then
    // `superview.setBounds(window.frame)` (0x20ab8..0x20af4; cf. 0x1b224).
    let _ = animated;
    state.last_bounds_set = Some(state.window_frame);
}

// 0x20b00 — -[AboutController webViewDidFinishLoad:]
// type: void __cdecl(AboutController *self, SEL, id)
#[doc(alias = "-[AboutController webViewDidFinishLoad:]")]
pub fn stub_0x20b00(state: &mut AboutState, is_agreement: bool) {
    // IDA 0x20b00 `-[AboutController webViewDidFinishLoad:]`: when the
    // finished view is the agreement view, unhides it (0x20b10..0x20b24).
    if is_agreement {
        state.agreement_hidden = false;
        state.agreement_visible = true;
    }
}

// 0x20b28 — -[AboutController webView:shouldStartLoadWithRequest:navigationType:]
// type: char __cdecl(AboutController *self, SEL, id, id, int)
#[doc(alias = "-[AboutController webView:shouldStartLoadWithRequest:navigationType:]")]
pub fn stub_0x20b28(state: &mut AboutState, url: Option<&str>) -> bool {
    // IDA 0x20b28 `-[AboutController
    // webView:shouldStartLoadWithRequest:navigationType:]`: missing URL
    // allows the load (0x20b56..0x20baa); a `file` URL allows it
    // (0x20b72..0x20bae); anything else segues to the agreement viewer
    // and cancels the load (0x20b80..0x20ba0).
    match url {
        None => true,
        Some(u) if u.contains("file") => true,
        Some(u) => {
            state.segue = Some(("AboutToAgreementSegue".to_string(), u.to_string()));
            false
        }
    }
}

// 0x20bb0 — -[AboutController prepareForSegue:sender:]
// type: void __cdecl(AboutController *self, SEL, id, id)
#[doc(alias = "-[AboutController prepareForSegue:sender:]")]
pub fn stub_0x20bb0(state: &mut AboutState, identifier: &str, sender_url: &str) {
    // IDA 0x20bb0 `-[AboutController prepareForSegue:sender:]`: the
    // `AboutToAgreementSegue` destination takes the sender as its URL
    // (0x20bc6..0x20c10); other identifiers are no-ops.
    if identifier == "AboutToAgreementSegue" {
        state.segue = Some((identifier.to_string(), sender_url.to_string()));
    }
}

// 0x20c14 — -[AboutController closeButtonPressed:]
// type: void __cdecl(AboutController *self, SEL, id)
#[doc(alias = "-[AboutController closeButtonPressed:]")]
pub fn stub_0x20c14(state: &mut AboutState) {
    // IDA 0x20c14 `-[AboutController closeButtonPressed:]`:
    // `dismissViewControllerAnimated:1 completion:0` (0x20c24, host UIKit).
    state.dismissed = true;
}

// 0x20c28 — -[AboutController clearCookiesButtonPressed:]
// type: void __cdecl(AboutController *self, SEL, id)
#[doc(alias = "-[AboutController clearCookiesButtonPressed:]")]
pub fn stub_0x20c28(state: &mut AboutState) {
    // IDA 0x20c28 `-[AboutController clearCookiesButtonPressed:]`:
    // `clearAllRobloxCookie` (0x20c46), then `RobloxAlertWithMessage:`
    // with the localized `CookiesClearedMessage` string (0x20c6e..0x20cb0).
    state.cookies_cleared = true;
    state.last_alert = Some("CookiesClearedMessage".to_string());
}

// 0x20cb4 — -[AboutController viewDidUnload]
// type: void __cdecl(AboutController *self, SEL)
#[doc(alias = "-[AboutController viewDidUnload]")]
pub fn stub_0x20cb4(state: &mut AboutState) {
    // IDA 0x20cb4 `-[AboutController viewDidUnload]`: nils `domainName`
    // and `clearCookies` via setters (0x20ccc..0x20ce0), then super
    // `viewDidUnload` (0x20cf8..0x20d02, host UIKit).
    state.outlets.remove("domainName");
    state.outlets.remove("clearCookies");
}

// 0x20d0c — -[AboutController navigationTitle]
// type: UINavigationItem *__cdecl(AboutController *self, SEL)
#[doc(alias = "-[AboutController navigationTitle]")]
pub fn stub_0x20d0c(state: &AboutState) -> Option<u32> {
    // IDA 0x20d0c `-[AboutController navigationTitle]`: GET (disasm
    // `_navigationTitle` IVAR load); the title is `nav_title` (cf.
    // 0x2096c).
    state.outlets.get("navigationTitle").copied().flatten()
}

// 0x20d1c — -[AboutController setNavigationTitle:]
// type: void __cdecl(AboutController *self, SEL, id)
#[doc(alias = "-[AboutController setNavigationTitle:]")]
pub fn stub_0x20d1c(state: &mut AboutState, view: Option<u32>) {
    // IDA 0x20d1c `-[AboutController setNavigationTitle:]`: SET (disasm
    // `objc_setProperty` prologue); host ownership is the outlet slot.
    state.outlets.insert("navigationTitle".to_string(), view);
}

// 0x20d40 — -[AboutController closeButton]
// type: UIBarButtonItem *__cdecl(AboutController *self, SEL)
#[doc(alias = "-[AboutController closeButton]")]
pub fn stub_0x20d40(state: &AboutState) -> Option<u32> {
    // IDA 0x20d40 `-[AboutController closeButton]`: GET (disasm
    // `_closeButton` IVAR load); the title is `close_title` (cf. 0x209c8).
    state.outlets.get("closeButton").copied().flatten()
}

// 0x20d50 — -[AboutController setCloseButton:]
// type: void __cdecl(AboutController *self, SEL, id)
#[doc(alias = "-[AboutController setCloseButton:]")]
pub fn stub_0x20d50(state: &mut AboutState, view: Option<u32>) {
    // IDA 0x20d50 `-[AboutController setCloseButton:]`: SET (disasm
    // `objc_setProperty` prologue); host ownership is the outlet slot.
    state.outlets.insert("closeButton".to_string(), view);
}

// 0x20d74 — -[AboutController clearCookies]
// type: UIBarButtonItem *__cdecl(AboutController *self, SEL)
#[doc(alias = "-[AboutController clearCookies]")]
pub fn stub_0x20d74(state: &AboutState) -> Option<u32> {
    // IDA 0x20d74 `-[AboutController clearCookies]`: GET (disasm
    // `_clearCookies` IVAR load); the title is `clear_cookies_title`
    // (cf. 0x20a0e).
    state.outlets.get("clearCookies").copied().flatten()
}

// 0x20d84 — -[AboutController setClearCookies:]
// type: void __cdecl(AboutController *self, SEL, id)
#[doc(alias = "-[AboutController setClearCookies:]")]
pub fn stub_0x20d84(state: &mut AboutState, view: Option<u32>) {
    // IDA 0x20d84 `-[AboutController setClearCookies:]`: SET (disasm
    // `objc_setProperty` prologue); host ownership is the outlet slot.
    state.outlets.insert("clearCookies".to_string(), view);
}

// 0x20da8 — -[AboutController legalTextView]
// type: UITextView *__cdecl(AboutController *self, SEL)
#[doc(alias = "-[AboutController legalTextView]")]
pub fn stub_0x20da8(state: &AboutState) -> Option<u32> {
    // IDA 0x20da8 `-[AboutController legalTextView]`: GET (disasm
    // `_legalTextView` IVAR load); the text is `legal_text` (cf. 0x20a48).
    state.outlets.get("legalTextView").copied().flatten()
}

// 0x20db8 — -[AboutController setLegalTextView:]
// type: void __cdecl(AboutController *self, SEL, id)
#[doc(alias = "-[AboutController setLegalTextView:]")]
pub fn stub_0x20db8(state: &mut AboutState, view: Option<u32>) {
    // IDA 0x20db8 `-[AboutController setLegalTextView:]`: SET (disasm
    // `objc_setProperty` prologue); host ownership is the outlet slot.
    state.outlets.insert("legalTextView".to_string(), view);
}

// 0x20ddc — -[AboutController versionLabel]
// type: UILabel *__cdecl(AboutController *self, SEL)
#[doc(alias = "-[AboutController versionLabel]")]
pub fn stub_0x20ddc(state: &AboutState) -> Option<u32> {
    // IDA 0x20ddc `-[AboutController versionLabel]`: GET (disasm
    // `_versionLabel` IVAR load); the text is `version_text` (cf.
    // 0x208bc).
    state.outlets.get("versionLabel").copied().flatten()
}

// 0x20dec — -[AboutController setVersionLabel:]
// type: void __cdecl(AboutController *self, SEL, id)
#[doc(alias = "-[AboutController setVersionLabel:]")]
pub fn stub_0x20dec(state: &mut AboutState, view: Option<u32>) {
    // IDA 0x20dec `-[AboutController setVersionLabel:]`: SET (disasm
    // `objc_setProperty` prologue); host ownership is the outlet slot.
    state.outlets.insert("versionLabel".to_string(), view);
}

// 0x20e10 — -[AboutController agreementWebView]
// type: UIWebView *__cdecl(AboutController *self, SEL)
#[doc(alias = "-[AboutController agreementWebView]")]
pub fn stub_0x20e10(state: &AboutState) -> Option<u32> {
    // IDA 0x20e10 `-[AboutController agreementWebView]`: GET (disasm
    // `_agreementWebView` IVAR load); visibility tracked by
    // `agreement_hidden`/`agreement_visible` (cf. 0x20644/0x20b00).
    state.outlets.get("agreementWebView").copied().flatten()
}

// 0x20e20 — -[AboutController setAgreementWebView:]
// type: void __cdecl(AboutController *self, SEL, id)
#[doc(alias = "-[AboutController setAgreementWebView:]")]
pub fn stub_0x20e20(state: &mut AboutState, view: Option<u32>) {
    // IDA 0x20e20 `-[AboutController setAgreementWebView:]`: SET
    // (`objc_setProperty` prologue, cf. 0x20dec); host ownership is the
    // outlet slot.
    state.outlets.insert("agreementWebView".to_string(), view);
}

// 0x20e44 — -[AboutController domainName]
// type: UILabel *__cdecl(AboutController *self, SEL)
#[doc(alias = "-[AboutController domainName]")]
pub fn stub_0x20e44(state: &AboutState) -> Option<u32> {
    // IDA 0x20e44 `-[AboutController domainName]`: GET (`_domainName`
    // IVAR load); the text is `domain_text` (cf. 0x20924).
    state.outlets.get("domainName").copied().flatten()
}

// 0x20e54 — -[AboutController setDomainName:]
// type: void __cdecl(AboutController *self, SEL, id)
#[doc(alias = "-[AboutController setDomainName:]")]
pub fn stub_0x20e54(state: &mut AboutState, view: Option<u32>) {
    // IDA 0x20e54 `-[AboutController setDomainName:]`: SET
    // (`objc_setProperty` prologue, cf. 0x20dec); host ownership is the
    // outlet slot.
    state.outlets.insert("domainName".to_string(), view);
}

// 0x20e78 — +[UpgradeCheckHelper getUpgradeCheckHelper]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[UpgradeCheckHelper getUpgradeCheckHelper]")]
pub fn stub_0x20e78(reg: &mut UpgradeCheckRegistry) -> &mut UpgradeCheckState {
    // IDA 0x20e78 `+[UpgradeCheckHelper getUpgradeCheckHelper]`:
    // `dispatch_once` singleton (0x20e9a..0x20ea6 -> 0x20ed4);
    // the predicate folds into `once_token`.
    if !reg.once_token {
        stub_0x20ed4(reg);
        reg.once_token = true;
    }
    &mut reg.helper
}

// 0x20ed4 — ___43+[UpgradeCheckHelper getUpgradeCheckHelper]_block_invoke
#[doc(alias = "___43+[UpgradeCheckHelper getUpgradeCheckHelper]_block_invoke")]
pub fn stub_0x20ed4(reg: &mut UpgradeCheckRegistry) {
    // IDA 0x20ed4: `alloc` + `init` into `dword_130C414`.
    reg.helper = UpgradeCheckState::default();
    stub_0x20f1c(&mut reg.helper);
}

// 0x20f08 — ___copy_helper_block__3
#[doc(alias = "___copy_helper_block__3")]
pub fn stub_0x20f08(dst: &mut BlockCapture, src: &BlockCapture) {
    // IDA 0x20f08 `__copy_helper_block__3`: single
    // `_Block_object_assign` retain (cf. 0x1f660).
    *dst = src.clone();
}

// 0x20f14 — ___destroy_helper_block__3
#[doc(alias = "___destroy_helper_block__3")]
pub fn stub_0x20f14(slot: &mut BlockCapture) {
    // IDA 0x20f14 `__destroy_helper_block__3`: single
    // `_Block_object_dispose` release (cf. 0x1f4a0).
    *slot = BlockCapture::default();
}

// 0x20f1c — -[UpgradeCheckHelper init]
// type: UpgradeCheckHelper *__cdecl(UpgradeCheckHelper *self, SEL)
#[doc(alias = "-[UpgradeCheckHelper init]")]
pub fn stub_0x20f1c(state: &mut UpgradeCheckState) {
    // IDA 0x20f1c `-[UpgradeCheckHelper init]`: super `init`, fresh
    // `UIAlertView` (delegate self) + `UpgradeButtonText` button
    // (localized on the platform side; key kept here), empty
    // `upgradeResponseData`, nil `upgradeConnection`.
    state.initialized = true;
    state.released = false;
    state.buttons = vec![AlertButton {
        title: "UpgradeButtonText".to_string(),
        enabled: true,
        handle: 0,
    }];
    state.response_data.clear();
    state.connection = None;
    state.connection_request = None;
}

// 0x21038 — -[UpgradeCheckHelper dealloc]
// type: void __cdecl(UpgradeCheckHelper *self, SEL)
#[doc(alias = "-[UpgradeCheckHelper dealloc]")]
pub fn stub_0x21038(state: &mut UpgradeCheckState) {
    // IDA 0x21038 `-[UpgradeCheckHelper dealloc]`: releases the alert
    // view, the live connection (if any), and the response data, then
    // super `dealloc` (releases fold into host ownership).
    state.buttons.clear();
    state.connection = None;
    state.connection_request = None;
    state.response_data.clear();
    state.released = true;
}

// 0x210b4 — +[UpgradeCheckHelper getUpgradeUrl]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[UpgradeCheckHelper getUpgradeUrl]")]
pub fn stub_0x210b4(base_url: &str) -> String {
    // IDA 0x210b4 `+[UpgradeCheckHelper getUpgradeUrl]`:
    // `infoDictionary[RbxBaseUrl] stringByAppendingString:
    // @"mobileapi/check-app-version?appVersion=%@"`.
    format!("{base_url}mobileapi/check-app-version?appVersion=%@")
}
