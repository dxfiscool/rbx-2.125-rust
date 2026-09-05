//! network generated_bg_1 — RakNet + RBX::Network + Replicator (auto-generated, do not edit manually)
//! Global gap filler bg_1 100 stubs 0x54104..0x582f8 EA-sorted asc next 100 after 0x540fc (RakNet|Network|Replicat|Socket|Upnp|HTTP 6232/6232 complete, 18869->18969 network distinct, rbx_core::SharedPtr not boost) [skeleton batch]

#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use rbx_core::SharedPtr;

/// Nav-bar state (IDA 0x543dc et al.).
#[derive(Clone, Debug, Default)]
pub struct NavBar {
 pub back_enabled: bool,
 pub loading: bool,
 pub need_robux_refresh: bool,
 pub web_view: Option<usize>,
 pub activity_indicator: Option<usize>,
 pub btn_back: Option<usize>,
 pub bar_top_toolbar: Option<usize>,
 pub lbl_robux: Option<String>,
 pub lbl_tix: Option<String>,
 pub toolbar: Option<usize>,
 pub page_indicator: Option<usize>,
 pub loading_overlay: Option<usize>,
 pub jump_place_id: i32,
 pub jump_in_progress: bool,
}

// 0x54104 — -[RobloxNavBarViewController viewDidLoad]
// type: void __cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController viewDidLoad]")]
pub fn stub_54104(setup: &mut dyn FnMut()) {
    // IDA 0x54104: viewDidLoad — nav/webview setup (below truncation).
    setup();
}

// 0x543dc — -[RobloxNavBarViewController hideBackButton]
// type: void __cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController hideBackButton]")]
pub fn stub_543dc(nav: &mut NavBar, set: &mut dyn FnMut(bool)) {
    // IDA 0x543dc: btnBack enabled = NO.
    nav.back_enabled = false;
    set(false);
}

// 0x543fc — -[RobloxNavBarViewController showBackButton]
// type: void __cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController showBackButton]")]
pub fn stub_543fc(nav: &mut NavBar, set: &mut dyn FnMut(bool)) {
    // IDA 0x543fc: btnBack enabled = YES.
    nav.back_enabled = true;
    set(true);
}

// 0x5441c — -[RobloxNavBarViewController viewDidUnload]
// type: void __cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController viewDidUnload]")]
pub fn stub_5441c(nav: &mut NavBar, teardown: &mut dyn FnMut()) {
    // IDA 0x5441c: nil outlets; super viewDidUnload.
    nav.back_enabled = false;
    nav.loading = false;
    teardown();
}

// 0x5449c — -[RobloxNavBarViewController showFullscreenText:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController showFullscreenText:]")]
pub fn stub_5449c(text: &str, set: &mut dyn FnMut(&str), dispatch: &mut dyn FnMut()) {
    // IDA 0x5449c: loading label text + dispatch show block.
    set(text);
    dispatch();
}

// 0x54514 — ___49-[RobloxNavBarViewController showFullscreenText:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___49-[RobloxNavBarViewController showFullscreenText:]_block_invoke")]
pub fn stub_54514(show: &mut dyn FnMut()) {
    // IDA 0x54514: startAnimating + overlay addSubview + unhide.
    show();
}

// 0x54594 — ___copy_helper_block_134
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_134")]
pub fn stub_54594(dst20: &mut usize, src20: usize, retain: &mut dyn FnMut(usize) -> usize) {
    // IDA 0x54594: _Block_object_assign(dst+20, src+20, 3).
    *dst20 = retain(src20);
}

// 0x545a0 — ___destroy_helper_block_135
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_135")]
pub fn stub_545a0(slot20: &mut usize, release: &mut dyn FnMut(usize)) {
    // IDA 0x545a0: _Block_object_dispose(slot+20, 3).
    release(*slot20);
}

// 0x545a8 — -[RobloxNavBarViewController hideFullscreenText]
// type: void __cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController hideFullscreenText]")]
pub fn stub_545a8(dispatch: &mut dyn FnMut()) {
    // IDA 0x545a8: dispatch_async(main, hide block).
    dispatch();
}

// 0x545f8 — ___48-[RobloxNavBarViewController hideFullscreenText]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___48-[RobloxNavBarViewController hideFullscreenText]_block_invoke")]
pub fn stub_545f8(hide: &mut dyn FnMut()) {
    // IDA 0x545f8: overlay hidden + stopAnimating.
    hide();
}

// 0x54648 — ___copy_helper_block_139
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_139")]
pub fn stub_54648(dst20: &mut usize, src20: usize, retain: &mut dyn FnMut(usize) -> usize) {
    // IDA 0x54648: _Block_object_assign(dst+20, src+20, 3).
    *dst20 = retain(src20);
}

// 0x54654 — ___destroy_helper_block_140
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_140")]
pub fn stub_54654(slot20: &mut usize, release: &mut dyn FnMut(usize)) {
    // IDA 0x54654: _Block_object_dispose(slot+20, 3).
    release(*slot20);
}

// 0x5465c — +[RobloxNavBarViewController checkForInAppPurchases:navigationType:]
// type: char __cdecl(id, SEL, id, int)
#[doc(alias = "+[RobloxNavBarViewController checkForInAppPurchases:navigationType:]")]
pub fn stub_5465c(refresh: &mut bool, nav_type: i32, url: &str, check: &mut dyn FnMut(&str) -> bool) -> bool {
    // IDA 0x5465c: link-click type -> refresh flag + NO; else URL purchase check.
    if nav_type == 1 {
        *refresh = true;
        false
    } else {
        check(url)
    }
}

// 0x5479c — -[RobloxNavBarViewController doPlaceLaunch:request:]
// type: char __cdecl(RobloxNavBarViewController *self, SEL, int, int)
#[doc(alias = "-[RobloxNavBarViewController doPlaceLaunch:request:]")]
pub fn stub_5479c(launch: &mut dyn FnMut()) {
    // IDA 0x5479c: doPlaceLaunch:request: (below truncation).
    launch();
}

// 0x549e4 — ___52-[RobloxNavBarViewController doPlaceLaunch:request:]_block_invoke
// type: id __fastcall(_DWORD *)
#[doc(alias = "___52-[RobloxNavBarViewController doPlaceLaunch:request:]_block_invoke")]
pub fn stub_549e4(start: &mut dyn FnMut()) {
    // IDA 0x549e4: PlaceLauncher startGame block.
    start();
}

// 0x54a28 — ___copy_helper_block_180
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_180")]
pub fn stub_54a28(dst20: &mut usize, src20: usize, retain: &mut dyn FnMut(usize) -> usize) {
    // IDA 0x54a28: _Block_object_assign(dst+20, src+20, 3).
    *dst20 = retain(src20);
}

// 0x54a34 — ___destroy_helper_block_181
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_181")]
pub fn stub_54a34(slot20: &mut usize, release: &mut dyn FnMut(usize)) {
    // IDA 0x54a34: _Block_object_dispose(slot+20, 3).
    release(*slot20);
}

// 0x54a3c — -[RobloxNavBarViewController checkForGameLaunch:]
// type: char __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController checkForGameLaunch:]")]
pub fn stub_54a3c(url: &str, check: &mut dyn FnMut(&str) -> bool) -> bool {
    // IDA 0x54a3c: game-launch URL check (below truncation).
    check(url)
}

// 0x54c64 — -[RobloxNavBarViewController webView:shouldStartLoadWithRequest:navigationType:]
// type: char __cdecl(RobloxNavBarViewController *self, SEL, id, id, int)
#[doc(alias = "-[RobloxNavBarViewController webView:shouldStartLoadWithRequest:navigationType:]")]
pub fn stub_54c64(disable_home: &mut dyn FnMut(), allow: &mut dyn FnMut() -> bool) -> bool {
    // IDA 0x54c64: btnHome disabled; purchase/launch checks gate load (below truncation).
    disable_home();
    allow()
}

// 0x54d0c — -[RobloxNavBarViewController handleStartGameFailure]
// type: void __cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController handleStartGameFailure]")]
pub fn stub_54d0c(hide: &mut dyn FnMut()) {
    // IDA 0x54d0c: handleStartGameFailure -> hideFullscreenText.
    hide();
}

// 0x54d1c — -[RobloxNavBarViewController handleStartGameSuccess]
// type: void __cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController handleStartGameSuccess]")]
pub fn stub_54d1c(hide: &mut dyn FnMut()) {
    // IDA 0x54d1c: handleStartGameSuccess -> hideFullscreenText.
    hide();
}

// 0x54d2c — -[RobloxNavBarViewController webView:didFailLoadWithError:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id, id)
#[doc(alias = "-[RobloxNavBarViewController webView:didFailLoadWithError:]")]
pub fn stub_54d2c(hide: &mut dyn FnMut()) {
    // IDA 0x54d2c: page-load indicator hidden.
    hide();
}

// 0x54d58 — -[RobloxNavBarViewController webViewDidStartLoad:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController webViewDidStartLoad:]")]
pub fn stub_54d58(is_main: bool, loading: bool, show: &mut dyn FnMut()) {
    // IDA 0x54d58: main webview loading -> show indicator.
    if is_main && loading {
        show();
    }
}

// 0x54db4 — -[RobloxNavBarViewController webViewDidFinishLoad:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController webViewDidFinishLoad:]")]
pub fn stub_54db4(nav: &mut NavBar, can_go_back: bool, refresh: &mut dyn FnMut(), hide_back: &mut dyn FnMut(), hide_spin: &mut dyn FnMut()) {
    // IDA 0x54db4: robux refresh; no-back -> hide back; indicator hidden.
    if nav.need_robux_refresh {
        refresh();
        nav.need_robux_refresh = false;
    }
    if !can_go_back {
        hide_back();
    }
    hide_spin();
}

// 0x54e40 — -[RobloxNavBarViewController updateUserInfoDisplay:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, bool)
#[doc(alias = "-[RobloxNavBarViewController updateUserInfoDisplay:]")]
pub fn stub_54e40(update: &mut dyn FnMut(bool), animate: bool) {
    // IDA 0x54e40: updateUserInfoDisplay — robux/tix labels (below truncation).
    update(animate);
}

// 0x54ff0 — -[RobloxNavBarViewController MenuClick:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController MenuClick:]")]
pub fn stub_54ff0(playing: bool, dismiss: &mut dyn FnMut()) {
    // IDA 0x54ff0: not playing ? dispatch dismiss block.
    if !playing {
        dismiss();
    }
}

// 0x55074 — ___40-[RobloxNavBarViewController MenuClick:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___40-[RobloxNavBarViewController MenuClick:]_block_invoke")]
pub fn stub_55074(dismiss: &mut dyn FnMut(bool)) {
    // IDA 0x55074: dismissViewControllerAnimated:YES.
    dismiss(true);
}

// 0x5508c — ___copy_helper_block_240
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_240")]
pub fn stub_5508c(dst20: &mut usize, src20: usize, retain: &mut dyn FnMut(usize) -> usize) {
    // IDA 0x5508c: _Block_object_assign(dst+20, src+20, 3).
    *dst20 = retain(src20);
}

// 0x55098 — ___destroy_helper_block_241
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_241")]
pub fn stub_55098(slot20: &mut usize, release: &mut dyn FnMut(usize)) {
    // IDA 0x55098: _Block_object_dispose(slot+20, 3).
    release(*slot20);
}

// 0x550a0 — +[RobloxNavBarViewController mostRecentViewController]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxNavBarViewController mostRecentViewController]")]
pub fn stub_550a0(recent: usize) -> usize {
    // IDA 0x550a0: return mostRecentViewController.
    recent
}

// 0x550b0 — -[RobloxNavBarViewController setMainWebView:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setMainWebView:]")]
pub fn stub_550b0(nav: &mut NavBar, view: Option<usize>, create: &mut dyn FnMut() -> usize) {
    // IDA 0x550b0: set webView or lazy-create + load.
    match view {
        Some(v) => nav.web_view = Some(v),
        None => {
            if nav.web_view.is_none() {
                nav.web_view = Some(create());
            }
        }
    }
}

// 0x551d8 — -[RobloxNavBarViewController backButtonClick:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController backButtonClick:]")]
pub fn stub_551d8(before: bool, after: bool, go_back: &mut dyn FnMut(), hide: &mut dyn FnMut()) {
    // IDA 0x551d8: canGoBack ? goBack; !canGoBack -> hideBackButton.
    if before {
        go_back();
    }
    if !after {
        hide();
    }
}

// 0x5523c — -[RobloxNavBarViewController setJumpToPlacePageAndLaunchGameWithID:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, int)
#[doc(alias = "-[RobloxNavBarViewController setJumpToPlacePageAndLaunchGameWithID:]")]
pub fn stub_5523c(nav: &mut NavBar, place_id: i32) {
    // IDA 0x5523c: jumpToPlaceIDNavigate = id.
    nav.jump_place_id = place_id;
}

// 0x5524c — -[RobloxNavBarViewController setJumpToPlaceIDGameInProgress:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, int)
#[doc(alias = "-[RobloxNavBarViewController setJumpToPlaceIDGameInProgress:]")]
pub fn stub_5524c(nav: &mut NavBar, in_progress: bool) {
    // IDA 0x5524c: jumpToPlaceIDGameInProgress = flag.
    nav.jump_in_progress = in_progress;
}

// 0x5525c — -[RobloxNavBarViewController activityIndicator]
// type: UIActivityIndicatorView *__cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController activityIndicator]")]
pub fn stub_5525c(nav: &NavBar) -> Option<usize> {
    // IDA 0x5525c: return activityIndicator.
    nav.activity_indicator
}

// 0x5526c — -[RobloxNavBarViewController setActivityIndicator:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setActivityIndicator:]")]
pub fn stub_5526c(nav: &mut NavBar, value: Option<usize>) {
    // IDA 0x5526c: objc_setProperty activityIndicator.
    nav.activity_indicator = value;
}

// 0x55290 — -[RobloxNavBarViewController btnBack]
// type: UIBarButtonItem *__cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController btnBack]")]
pub fn stub_55290(nav: &NavBar) -> Option<usize> {
    // IDA 0x55290: return btnBack.
    nav.btn_back
}

// 0x552a0 — -[RobloxNavBarViewController setBtnBack:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setBtnBack:]")]
pub fn stub_552a0(nav: &mut NavBar, value: Option<usize>) {
    // IDA 0x552a0: objc_setProperty btnBack.
    nav.btn_back = value;
}

// 0x552c4 — -[RobloxNavBarViewController barTopToolbar]
// type: UIToolbar *__cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController barTopToolbar]")]
pub fn stub_552c4(nav: &NavBar) -> Option<usize> {
    // IDA 0x552c4: return barTopToolbar.
    nav.bar_top_toolbar
}

// 0x552d4 — -[RobloxNavBarViewController setBarTopToolbar:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setBarTopToolbar:]")]
pub fn stub_552d4(nav: &mut NavBar, value: Option<usize>) {
    // IDA 0x552d4: objc_setProperty barTopToolbar.
    nav.bar_top_toolbar = value;
}

// 0x552f8 — -[RobloxNavBarViewController lblRobux]
// type: UILabel *__cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController lblRobux]")]
pub fn stub_552f8(nav: &NavBar) -> Option<String> {
    // IDA 0x552f8: return _lblRobux.
    nav.lbl_robux.clone()
}

// 0x55308 — -[RobloxNavBarViewController setLblRobux:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setLblRobux:]")]
pub fn stub_55308(nav: &mut NavBar, value: Option<String>) {
    // IDA 0x55308: objc_setProperty lblRobux.
    nav.lbl_robux = value;
}

// 0x5532c — -[RobloxNavBarViewController lblTix]
// type: UILabel *__cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController lblTix]")]
pub fn stub_5532c(nav: &NavBar) -> Option<String> {
    // IDA 0x5532c: return _lblTix.
    nav.lbl_tix.clone()
}

// 0x5533c — -[RobloxNavBarViewController setLblTix:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setLblTix:]")]
pub fn stub_5533c(nav: &mut NavBar, value: Option<String>) {
    // IDA 0x5533c: objc_setProperty lblTix.
    nav.lbl_tix = value;
}

// 0x55360 — -[RobloxNavBarViewController toolbar]
// type: UIToolbar *__cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController toolbar]")]
pub fn stub_55360(nav: &NavBar) -> Option<usize> {
    // IDA 0x55360: return _toolbar.
    nav.toolbar
}

// 0x55370 — -[RobloxNavBarViewController setToolbar:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setToolbar:]")]
pub fn stub_55370(nav: &mut NavBar, value: Option<usize>) {
    // IDA 0x55370: objc_setProperty toolbar.
    nav.toolbar = value;
}

// 0x55394 — -[RobloxNavBarViewController pageLoadActivityIndicator]
// type: UIActivityIndicatorView *__cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController pageLoadActivityIndicator]")]
pub fn stub_55394(nav: &NavBar) -> Option<usize> {
    // IDA 0x55394: return _pageLoadActivityIndicator.
    nav.page_indicator
}

// 0x553a4 — -[RobloxNavBarViewController setPageLoadActivityIndicator:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setPageLoadActivityIndicator:]")]
pub fn stub_553a4(nav: &mut NavBar, value: Option<usize>) {
    // IDA 0x553a4: objc_setProperty pageLoadActivityIndicator.
    nav.page_indicator = value;
}

// 0x553c8 — -[RobloxNavBarViewController loadingOverlay]
// type: UIView *__cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController loadingOverlay]")]
pub fn stub_553c8(nav: &NavBar) -> Option<usize> {
    // IDA 0x553c8: return _loadingOverlay.
    nav.loading_overlay
}

// 0x553d8 — -[RobloxNavBarViewController setLoadingOverlay:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setLoadingOverlay:]")]
pub fn stub_553d8(nav: &mut NavBar, value: Option<usize>) {
    // IDA 0x553d8: objc_setProperty loadingOverlay.
    nav.loading_overlay = value;
}

// 0x553fc — -[RobloxNavBarViewController loadingLabel]
// type: UILabel *__cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController loadingLabel]")]
pub fn stub_553fc() -> ! {
    todo!("0x553fc -[RobloxNavBarViewController loadingLabel]")
}

// 0x5540c — -[RobloxNavBarViewController setLoadingLabel:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setLoadingLabel:]")]
pub fn stub_5540c() -> ! {
    todo!("0x5540c -[RobloxNavBarViewController setLoadingLabel:]")
}

// 0x55430 — -[RobloxNavBarViewController btnHome]
// type: UIBarButtonItem *__cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController btnHome]")]
pub fn stub_55430() -> ! {
    todo!("0x55430 -[RobloxNavBarViewController btnHome]")
}

// 0x55440 — -[RobloxNavBarViewController setBtnHome:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setBtnHome:]")]
pub fn stub_55440() -> ! {
    todo!("0x55440 -[RobloxNavBarViewController setBtnHome:]")
}

// 0x55464 — -[RobloxNavBarViewController robuxImageView]
// type: UIImageView *__cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController robuxImageView]")]
pub fn stub_55464() -> ! {
    todo!("0x55464 -[RobloxNavBarViewController robuxImageView]")
}

// 0x55474 — -[RobloxNavBarViewController setRobuxImageView:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setRobuxImageView:]")]
pub fn stub_55474() -> ! {
    todo!("0x55474 -[RobloxNavBarViewController setRobuxImageView:]")
}

// 0x55498 — -[RobloxNavBarViewController tixImageView]
// type: UIImageView *__cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController tixImageView]")]
pub fn stub_55498() -> ! {
    todo!("0x55498 -[RobloxNavBarViewController tixImageView]")
}

// 0x554a8 — -[RobloxNavBarViewController setTixImageView:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setTixImageView:]")]
pub fn stub_554a8() -> ! {
    todo!("0x554a8 -[RobloxNavBarViewController setTixImageView:]")
}

// 0x554cc — __GLOBAL__I_a_28
// type: void __fastcall()
#[doc(alias = "global constructor keyed to_a_28")]
pub fn stub_554cc() -> ! {
    todo!("0x554cc __GLOBAL__I_a_28")
}

// 0x55664 — -[StoreManager init]
// type: StoreManager *__cdecl(StoreManager *self, SEL)
#[doc(alias = "-[StoreManager init]")]
pub fn stub_55664() -> ! {
    todo!("0x55664 -[StoreManager init]")
}

// 0x55754 — ___20-[StoreManager init]_block_invoke
// type: int __fastcall(int)
#[doc(alias = "___20-[StoreManager init]_block_invoke")]
pub fn stub_55754() -> ! {
    todo!("0x55754 ___20-[StoreManager init]_block_invoke")
}

// 0x557c8 — ___copy_helper_block__16
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__16")]
pub fn stub_557c8() -> ! {
    todo!("0x557c8 ___copy_helper_block__16")
}

// 0x557d4 — ___destroy_helper_block__16
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__16")]
pub fn stub_557d4() -> ! {
    todo!("0x557d4 ___destroy_helper_block__16")
}

// 0x557dc — +[StoreManager getStoreMgr]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[StoreManager getStoreMgr]")]
pub fn stub_557dc() -> ! {
    todo!("0x557dc +[StoreManager getStoreMgr]")
}

// 0x55838 — ___27+[StoreManager getStoreMgr]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___27+[StoreManager getStoreMgr]_block_invoke")]
pub fn stub_55838() -> ! {
    todo!("0x55838 ___27+[StoreManager getStoreMgr]_block_invoke")
}

// 0x5586c — ___copy_helper_block_23
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_23")]
pub fn stub_5586c() -> ! {
    todo!("0x5586c ___copy_helper_block_23")
}

// 0x55878 — ___destroy_helper_block_24
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_24")]
pub fn stub_55878() -> ! {
    todo!("0x55878 ___destroy_helper_block_24")
}

// 0x55880 — -[StoreManager canMakePurchase]
// type: char __cdecl(StoreManager *self, SEL)
#[doc(alias = "-[StoreManager canMakePurchase]")]
pub fn stub_55880() -> ! {
    todo!("0x55880 -[StoreManager canMakePurchase]")
}

// 0x558d0 — -[StoreManager request:didFailWithError:]
// type: void __cdecl(StoreManager *self, SEL, id, id)
#[doc(alias = "-[StoreManager request:didFailWithError:]")]
pub fn stub_558d0() -> ! {
    todo!("0x558d0 -[StoreManager request:didFailWithError:]")
}

// 0x559d0 — -[StoreManager requestDidFinish:]
// type: void __cdecl(StoreManager *self, SEL, id)
#[doc(alias = "-[StoreManager requestDidFinish:]")]
pub fn stub_559d0() -> ! {
    todo!("0x559d0 -[StoreManager requestDidFinish:]")
}

// 0x55a9c — -[StoreManager restrictTimeBoundPurchase:]
// type: char __cdecl(StoreManager *self, SEL, id)
#[doc(alias = "-[StoreManager restrictTimeBoundPurchase:]")]
pub fn stub_55a9c() -> ! {
    todo!("0x55a9c -[StoreManager restrictTimeBoundPurchase:]")
}

// 0x55c68 — -[StoreManager reset]
// type: void __cdecl(StoreManager *self, SEL)
#[doc(alias = "-[StoreManager reset]")]
pub fn stub_55c68() -> ! {
    todo!("0x55c68 -[StoreManager reset]")
}

// 0x55d04 — -[StoreManager recordPurchaseTime:]
// type: void __cdecl(StoreManager *self, SEL, id)
#[doc(alias = "-[StoreManager recordPurchaseTime:]")]
pub fn stub_55d04() -> ! {
    todo!("0x55d04 -[StoreManager recordPurchaseTime:]")
}

// 0x55e94 — -[StoreManager productsRequest:didReceiveResponse:]
// type: void __cdecl(StoreManager *self, SEL, id, id)
#[doc(alias = "-[StoreManager productsRequest:didReceiveResponse:]")]
pub fn stub_55e94() -> ! {
    todo!("0x55e94 -[StoreManager productsRequest:didReceiveResponse:]")
}

// 0x56894 — -[StoreManager requestProductData:]
// type: void __cdecl(StoreManager *self, SEL, id)
#[doc(alias = "-[StoreManager requestProductData:]")]
pub fn stub_56894() -> ! {
    todo!("0x56894 -[StoreManager requestProductData:]")
}

// 0x56914 — -[StoreManager purchaseProduct:]
// type: void __cdecl(StoreManager *self, SEL, id)
#[doc(alias = "-[StoreManager purchaseProduct:]")]
pub fn stub_56914() -> ! {
    todo!("0x56914 -[StoreManager purchaseProduct:]")
}

// 0x569b4 — -[StoreManager verifyIfCorrectUser]
// type: int __cdecl(StoreManager *self, SEL)
#[doc(alias = "-[StoreManager verifyIfCorrectUser]")]
pub fn stub_569b4() -> ! {
    todo!("0x569b4 -[StoreManager verifyIfCorrectUser]")
}

// 0x56ad0 — -[StoreManager completeTransaction:]
// type: void __cdecl(StoreManager *self, SEL, id)
#[doc(alias = "-[StoreManager completeTransaction:]")]
pub fn stub_56ad0() -> ! {
    todo!("0x56ad0 -[StoreManager completeTransaction:]")
}

// 0x56d80 — -[StoreManager endTransaction:paymentTransaction:paymentQueue:]
// type: void __cdecl(StoreManager *self, SEL, char, id, id)
#[doc(alias = "-[StoreManager endTransaction:paymentTransaction:paymentQueue:]")]
pub fn stub_56d80() -> ! {
    todo!("0x56d80 -[StoreManager endTransaction:paymentTransaction:paymentQueue:]")
}

// 0x572e4 — ___63-[StoreManager endTransaction:paymentTransaction:paymentQueue:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___63-[StoreManager endTransaction:paymentTransaction:paymentQueue:]_block_invoke")]
pub fn stub_572e4() -> ! {
    todo!("0x572e4 ___63-[StoreManager endTransaction:paymentTransaction:paymentQueue:]_block_invoke")
}

// 0x573b0 — ___copy_helper_block_212
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_212")]
pub fn stub_573b0() -> ! {
    todo!("0x573b0 ___copy_helper_block_212")
}

// 0x573bc — ___destroy_helper_block_213
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_213")]
pub fn stub_573bc() -> ! {
    todo!("0x573bc ___destroy_helper_block_213")
}

// 0x573c4 — ___63-[StoreManager endTransaction:paymentTransaction:paymentQueue:]_block_invoke215
// type: id __fastcall(int)
#[doc(alias = "___63-[StoreManager endTransaction:paymentTransaction:paymentQueue:]_block_invoke215")]
pub fn stub_573c4() -> ! {
    todo!("0x573c4 ___63-[StoreManager endTransaction:paymentTransaction:paymentQueue:]_block_invoke215")
}

// 0x57410 — ___copy_helper_block_216
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_216")]
pub fn stub_57410() -> ! {
    todo!("0x57410 ___copy_helper_block_216")
}

// 0x57434 — ___destroy_helper_block_217
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_217")]
pub fn stub_57434() -> ! {
    todo!("0x57434 ___destroy_helper_block_217")
}

// 0x57450 — ___63-[StoreManager endTransaction:paymentTransaction:paymentQueue:]_block_invoke219
// type: id __fastcall(int)
#[doc(alias = "___63-[StoreManager endTransaction:paymentTransaction:paymentQueue:]_block_invoke219")]
pub fn stub_57450() -> ! {
    todo!("0x57450 ___63-[StoreManager endTransaction:paymentTransaction:paymentQueue:]_block_invoke219")
}

// 0x5751c — ___copy_helper_block_222
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_222")]
pub fn stub_5751c() -> ! {
    todo!("0x5751c ___copy_helper_block_222")
}

// 0x57528 — ___destroy_helper_block_223
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_223")]
pub fn stub_57528() -> ! {
    todo!("0x57528 ___destroy_helper_block_223")
}

// 0x57530 — -[StoreManager failedTransaction:]
// type: void __cdecl(StoreManager *self, SEL, id)
#[doc(alias = "-[StoreManager failedTransaction:]")]
pub fn stub_57530() -> ! {
    todo!("0x57530 -[StoreManager failedTransaction:]")
}

// 0x5763c — -[StoreManager restoreTransaction:]
// type: void __cdecl(StoreManager *self, SEL, id)
#[doc(alias = "-[StoreManager restoreTransaction:]")]
pub fn stub_5763c() -> ! {
    todo!("0x5763c -[StoreManager restoreTransaction:]")
}

// 0x57740 — -[StoreManager paymentQueue:updatedTransactions:]
// type: void __cdecl(StoreManager *self, SEL, id, id)
#[doc(alias = "-[StoreManager paymentQueue:updatedTransactions:]")]
pub fn stub_57740() -> ! {
    todo!("0x57740 -[StoreManager paymentQueue:updatedTransactions:]")
}

// 0x5784c — -[StoreManager encode:length:]
// type: id __cdecl(StoreManager *self, SEL, const char *, int)
#[doc(alias = "-[StoreManager encode:length:]")]
pub fn stub_5784c() -> ! {
    todo!("0x5784c -[StoreManager encode:length:]")
}

// 0x5796c — -[StoreManager verifyReceipt:forProductId:paymentTransaction:paymentQueue:]
// type: void __cdecl(StoreManager *self, SEL, id, id, id, id)
#[doc(alias = "-[StoreManager verifyReceipt:forProductId:paymentTransaction:paymentQueue:]")]
pub fn stub_5796c() -> ! {
    todo!("0x5796c -[StoreManager verifyReceipt:forProductId:paymentTransaction:paymentQueue:]")
}

// 0x57da0 — ___75-[StoreManager verifyReceipt:forProductId:paymentTransaction:paymentQueue:]_block_invoke
// type: void __fastcall(int, void *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "___75-[StoreManager verifyReceipt:forProductId:paymentTransaction:paymentQueue:]_block_invoke")]
pub fn stub_57da0() -> ! {
    todo!("0x57da0 ___75-[StoreManager verifyReceipt:forProductId:paymentTransaction:paymentQueue:]_block_invoke")
}

// 0x57f28 — ___75-[StoreManager verifyReceipt:forProductId:paymentTransaction:paymentQueue:]_block_invoke_2
// type: void __cdecl(id)
#[doc(alias = "___75-[StoreManager verifyReceipt:forProductId:paymentTransaction:paymentQueue:]_block_invoke_2")]
pub fn stub_57f28() -> ! {
    todo!("0x57f28 ___75-[StoreManager verifyReceipt:forProductId:paymentTransaction:paymentQueue:]_block_invoke_2")
}

// 0x57f98 — ___copy_helper_block_319
// type: void __fastcall(int, const void **)
#[doc(alias = "___copy_helper_block_319")]
pub fn stub_57f98() -> ! {
    todo!("0x57f98 ___copy_helper_block_319")
}

// 0x57fc8 — ___destroy_helper_block_320
// type: void __fastcall(const void **)
#[doc(alias = "___destroy_helper_block_320")]
pub fn stub_57fc8() -> ! {
    todo!("0x57fc8 ___destroy_helper_block_320")
}

// 0x57fec — __GLOBAL__I_a_29
// type: void __fastcall()
#[doc(alias = "global constructor keyed to_a_29")]
pub fn stub_57fec() -> ! {
    todo!("0x57fec __GLOBAL__I_a_29")
}

// 0x58184 — -[UIWebViewCacheManager init]
// type: UIWebViewCacheManager *__cdecl(UIWebViewCacheManager *self, SEL)
#[doc(alias = "-[UIWebViewCacheManager init]")]
pub fn stub_58184() -> ! {
    todo!("0x58184 -[UIWebViewCacheManager init]")
}

// 0x582f8 — ___29-[UIWebViewCacheManager init]_block_invoke
// type: int __fastcall(int)
#[doc(alias = "___29-[UIWebViewCacheManager init]_block_invoke")]
pub fn stub_582f8() -> ! {
    todo!("0x582f8 ___29-[UIWebViewCacheManager init]_block_invoke")
}
