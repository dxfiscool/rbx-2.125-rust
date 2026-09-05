// Auto-generated skeletons for rbx-script — Lua|Script|Yield batch (gap filler)
// Filter: Lua|Script|Yield (4818 filtered, 0 remaining) -> global gap filler EA-sorted asc next 150 not yet in script crate
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +150 stubs | range 0x5479c..0x5bf68 EA-sorted asc next 150 global not yet in script crate (script 16912 -> 17062 distinct)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicU32, Ordering};
use crate::generated_172::NavBarVC;

/// Pending place-jump globals (IDA 0x5523c/0x5524c): `viewDidAppear`
/// (0x54036) and `gotDidLeaveGameNotification` (0x53f8e) consume them;
/// the global glue folds into the host.
static JUMP_NAVIGATE: AtomicU32 = AtomicU32::new(0);
static JUMP_PROGRESS: AtomicU32 = AtomicU32::new(0);
/// `__GLOBAL__I_a_28` one-shot latch (IDA 0x554cc).
static GLOBAL_A28_INIT: LazyLock<u32> = LazyLock::new(|| 1);

/// `StoreManager` purchase-throttle courts (IDA 0x55664..0x55754): the
/// minute gaps between robux/BC/catalog purchases and the billing retry
/// limit. The payment-queue glue folds into the host.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StoreMgr {
    pub robux_min: u32,
    pub bc_min: u32,
    pub catalog_min: u32,
    pub retry_limit: u32,
}

// 0x5479c — -[RobloxNavBarViewController doPlaceLaunch:request:]
// type: char __cdecl(RobloxNavBarViewController *self, SEL, int, int)
#[doc(alias = "-[RobloxNavBarViewController doPlaceLaunch:request:]")]
pub fn stub_0x5479c(vc: &mut NavBarVC) {
    // IDA 0x5479c: `doPlaceLaunch:request:` clears the web cache,
    // captures the launch in a block, and dispatches it (see
    // `stub_0x549e4`); the queue hop folds into the caller.
    stub_0x549e4(vc);
}

// 0x549e4 — ___52-[RobloxNavBarViewController doPlaceLaunch:request:]_block_invoke
// type: id __fastcall(_DWORD *)
#[doc(alias = "___52-[RobloxNavBarViewController doPlaceLaunch:request:]_block_invoke")]
pub fn stub_0x549e4(vc: &mut NavBarVC) {
    // IDA 0x549e4: the launch block starts the game through the shared
    // place launcher (0x54a04..0x54a26); the launcher send folds into
    // the host.
    vc.launches += 1;
}

// 0x54a28 — ___copy_helper_block_180
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_180")]
pub fn stub_0x54a28() {
    // IDA 0x54a28: `__copy_helper_block_180` retains captures; `Arc`
    // glue covers it — no-op.
}

// 0x54a34 — ___destroy_helper_block_181
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_181")]
pub fn stub_0x54a34() {
    // IDA 0x54a34: `__destroy_helper_block_181` releases captures (pair
    // of 0x54a28); `Arc` glue covers it — no-op.
}

// 0x54a3c — -[RobloxNavBarViewController checkForGameLaunch:]
// type: char __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController checkForGameLaunch:]")]
pub fn stub_0x54a3c(vc: &mut NavBarVC, playing: bool) -> bool {
    // IDA 0x54a3c: `checkForGameLaunch:` bails when already playing
    // (0x54a76..0x54a7e), else shows the back button and parses the
    // launch URL; the parse folds into the host.
    if !playing {
        vc.back_enabled = true;
    }
    !playing
}

// 0x54c64 — -[RobloxNavBarViewController webView:shouldStartLoadWithRequest:navigationType:]
// type: char __cdecl(RobloxNavBarViewController *self, SEL, id, id, int)
#[doc(alias = "-[RobloxNavBarViewController webView:shouldStartLoadWithRequest:navigationType:]")]
pub fn stub_0x54c64(vc: &mut NavBarVC, is_purchase: bool) -> bool {
    // IDA 0x54c64: `webView:shouldStartLoadWithRequest:` parks the home
    // button (0x54c80..0x54c96), then routes purchases natively and
    // re-arms home (0x54cb6..0x54cc4); the IAP check folds into the
    // host — see `stub_0x5465c`.
    if is_purchase {
        vc.home_enabled = true;
        false
    } else {
        vc.home_enabled = false;
        true
    }
}

// 0x54d0c — -[RobloxNavBarViewController handleStartGameFailure]
// type: void __cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController handleStartGameFailure]")]
pub fn stub_0x54d0c(vc: &mut NavBarVC) {
    // IDA 0x54d0c: `handleStartGameFailure` hides the loader (0x54d18).
    vc.fullscreen = None;
}

// 0x54d1c — -[RobloxNavBarViewController handleStartGameSuccess]
// type: void __cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController handleStartGameSuccess]")]
pub fn stub_0x54d1c(vc: &mut NavBarVC) {
    // IDA 0x54d1c: `handleStartGameSuccess` hides the loader (0x54d28).
    vc.fullscreen = None;
}

// 0x54d2c — -[RobloxNavBarViewController webView:didFailLoadWithError:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id, id)
#[doc(alias = "-[RobloxNavBarViewController webView:didFailLoadWithError:]")]
pub fn stub_0x54d2c(vc: &mut NavBarVC) {
    // IDA 0x54d2c: `webView:didFailLoadWithError:` hides the spinner
    // (0x54d3c..0x54d52).
    vc.loading = false;
}

// 0x54d58 — -[RobloxNavBarViewController webViewDidStartLoad:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController webViewDidStartLoad:]")]
pub fn stub_0x54d58(vc: &mut NavBarVC, is_loading: bool) {
    // IDA 0x54d58: `webViewDidStartLoad:` shows the spinner while the
    // main view loads (0x54d6e..0x54dae).
    if is_loading {
        vc.loading = true;
    }
}

// 0x54db4 — -[RobloxNavBarViewController webViewDidFinishLoad:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController webViewDidFinishLoad:]")]
pub fn stub_0x54db4(vc: &mut NavBarVC, can_go_back: bool) {
    // IDA 0x54db4: `webViewDidFinishLoad:` refreshes robux info when
    // flagged (0x54dc6..0x54de0), hides the back button at depth zero
    // (0x54e00..0x54e14), and hides the spinner (0x54e26..0x54e3c).
    if vc.refresh_robux {
        vc.info_refresh += 1;
        vc.refresh_robux = false;
    }
    vc.web_depth = if can_go_back { vc.web_depth.max(1) } else { 0 };
    if !can_go_back {
        vc.back_enabled = false;
    }
    vc.loading = false;
}

// 0x54e40 — -[RobloxNavBarViewController updateUserInfoDisplay:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, bool)
#[doc(alias = "-[RobloxNavBarViewController updateUserInfoDisplay:]")]
pub fn stub_0x54e40(vc: &mut NavBarVC, force: bool) {
    // IDA 0x54e40: `updateUserInfoDisplay:` refreshes the player info
    // when forced (0x54e74..0x54e8c) and repaints the robux/tix labels;
    // the label glue folds into the host.
    let _ = force;
    vc.info_refresh += 1;
}

// 0x54ff0 — -[RobloxNavBarViewController MenuClick:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController MenuClick:]")]
pub fn stub_0x54ff0(vc: &mut NavBarVC, playing: bool) {
    // IDA 0x54ff0: `MenuClick:` `dispatch_async`s the dismiss block when
    // no game runs (0x55024..0x5506c); the queue hop folds into the
    // caller — see `stub_0x55074`.
    if !playing {
        stub_0x55074(vc);
    }
}

// 0x55074 — ___40-[RobloxNavBarViewController MenuClick:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___40-[RobloxNavBarViewController MenuClick:]_block_invoke")]
pub fn stub_0x55074(vc: &mut NavBarVC) {
    // IDA 0x55074: the menu block dismisses the controller (0x5506a).
    vc.dismissed = true;
}

// 0x5508c — ___copy_helper_block_240
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_240")]
pub fn stub_0x5508c() {
    // IDA 0x5508c: `__copy_helper_block_240` retains captures; `Arc`
    // glue covers it — no-op.
}

// 0x55098 — ___destroy_helper_block_241
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_241")]
pub fn stub_0x55098() {
    // IDA 0x55098: `__destroy_helper_block_241` releases captures (pair
    // of 0x5508c); `Arc` glue covers it — no-op.
}

// 0x550a0 — +[RobloxNavBarViewController mostRecentViewController]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxNavBarViewController mostRecentViewController]")]
pub fn stub_0x550a0(vc: &NavBarVC) -> bool {
    // IDA 0x550a0: `mostRecentViewController` answers the registered
    // controller (0x550ac, set in 0x53fec); the registry folds into the
    // host.
    vc.recent
}

// 0x550b0 — -[RobloxNavBarViewController setMainWebView:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setMainWebView:]")]
pub fn stub_0x550b0(vc: &mut NavBarVC, view: Option<u32>) {
    // IDA 0x550b0: `setMainWebView:` retains and seats a given view
    // (0x550b8..0x550f2), else ensures the web view and loads the URL
    // (0x55120..); the WebKit glue folds into the host.
    if view.is_some() {
        vc.web_view = view;
    }
}

// 0x551d8 — -[RobloxNavBarViewController backButtonClick:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController backButtonClick:]")]
pub fn stub_0x551d8(vc: &mut NavBarVC) {
    // IDA 0x551d8: `backButtonClick:` goes back when history remains
    // (0x551fa..0x55212) and hides the back button at depth zero
    // (0x5521e..0x55238).
    if vc.web_depth > 0 {
        vc.web_depth -= 1;
    }
    if vc.web_depth == 0 {
        vc.back_enabled = false;
    }
}

// 0x5523c — -[RobloxNavBarViewController setJumpToPlacePageAndLaunchGameWithID:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, int)
#[doc(alias = "-[RobloxNavBarViewController setJumpToPlacePageAndLaunchGameWithID:]")]
pub fn stub_0x5523c(id: u32) {
    // IDA 0x5523c: `setJumpToPlacePageAndLaunchGameWithID:` stores the
    // navigate target (0x55246).
    JUMP_NAVIGATE.store(id, Ordering::SeqCst);
}

// 0x5524c — -[RobloxNavBarViewController setJumpToPlaceIDGameInProgress:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, int)
#[doc(alias = "-[RobloxNavBarViewController setJumpToPlaceIDGameInProgress:]")]
pub fn stub_0x5524c(id: u32) {
    // IDA 0x5524c: `setJumpToPlaceIDGameInProgress:` stores the pending
    // place (0x55256).
    JUMP_PROGRESS.store(id, Ordering::SeqCst);
}

// 0x5525c — -[RobloxNavBarViewController activityIndicator]
// type: UIActivityIndicatorView *__cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController activityIndicator]")]
pub fn stub_0x5525c(vc: &NavBarVC) -> Option<u32> {
    // IDA 0x5525c: `activityIndicator` answers the spinner (0x5526a).
    vc.spinner
}

// 0x5526c — -[RobloxNavBarViewController setActivityIndicator:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setActivityIndicator:]")]
pub fn stub_0x5526c(vc: &mut NavBarVC, spinner: u32) {
    // IDA 0x5526c: `setActivityIndicator:` stores the spinner (0x55288).
    vc.spinner = Some(spinner);
}

// 0x55290 — -[RobloxNavBarViewController btnBack]
// type: UIBarButtonItem *__cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController btnBack]")]
pub fn stub_0x55290(vc: &NavBarVC) -> Option<u32> {
    // IDA 0x55290: `btnBack` answers the button (0x5529e).
    vc.btn_back
}

// 0x552a0 — -[RobloxNavBarViewController setBtnBack:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setBtnBack:]")]
pub fn stub_0x552a0(vc: &mut NavBarVC, btn: u32) {
    // IDA 0x552a0: `setBtnBack:` stores the button (0x552bc).
    vc.btn_back = Some(btn);
}

// 0x552c4 — -[RobloxNavBarViewController barTopToolbar]
// type: UIToolbar *__cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController barTopToolbar]")]
pub fn stub_0x552c4(vc: &NavBarVC) -> Option<u32> {
    // IDA 0x552c4: `barTopToolbar` answers the toolbar handle.
    vc.top_toolbar
}

// 0x552d4 — -[RobloxNavBarViewController setBarTopToolbar:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setBarTopToolbar:]")]
pub fn stub_0x552d4(vc: &mut NavBarVC, toolbar: u32) {
    // IDA 0x552d4: `setBarTopToolbar:` stores the toolbar handle.
    vc.top_toolbar = Some(toolbar);
}

// 0x552f8 — -[RobloxNavBarViewController lblRobux]
// type: UILabel *__cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController lblRobux]")]
pub fn stub_0x552f8(vc: &NavBarVC) -> Option<u32> {
    // IDA 0x552f8: `lblRobux` answers the label handle.
    vc.lbl_robux
}

// 0x55308 — -[RobloxNavBarViewController setLblRobux:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setLblRobux:]")]
pub fn stub_0x55308(vc: &mut NavBarVC, label: u32) {
    // IDA 0x55308: `setLblRobux:` stores the label handle.
    vc.lbl_robux = Some(label);
}

// 0x5532c — -[RobloxNavBarViewController lblTix]
// type: UILabel *__cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController lblTix]")]
pub fn stub_0x5532c(vc: &NavBarVC) -> Option<u32> {
    // IDA 0x5532c: `lblTix` answers the label handle.
    vc.lbl_tix
}

// 0x5533c — -[RobloxNavBarViewController setLblTix:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setLblTix:]")]
pub fn stub_0x5533c(vc: &mut NavBarVC, label: u32) {
    // IDA 0x5533c: `setLblTix:` stores the label handle.
    vc.lbl_tix = Some(label);
}

// 0x55360 — -[RobloxNavBarViewController toolbar]
// type: UIToolbar *__cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController toolbar]")]
pub fn stub_0x55360(vc: &NavBarVC) -> Option<u32> {
    // IDA 0x55360: `toolbar` answers the toolbar handle.
    vc.toolbar
}

// 0x55370 — -[RobloxNavBarViewController setToolbar:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setToolbar:]")]
pub fn stub_0x55370(vc: &mut NavBarVC, toolbar: u32) {
    // IDA 0x55370: `setToolbar:` stores the toolbar handle.
    vc.toolbar = Some(toolbar);
}

// 0x55394 — -[RobloxNavBarViewController pageLoadActivityIndicator]
// type: UIActivityIndicatorView *__cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController pageLoadActivityIndicator]")]
pub fn stub_0x55394(vc: &NavBarVC) -> Option<u32> {
    // IDA 0x55394: `pageLoadActivityIndicator` answers the spinner.
    vc.page_spinner
}

// 0x553a4 — -[RobloxNavBarViewController setPageLoadActivityIndicator:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setPageLoadActivityIndicator:]")]
pub fn stub_0x553a4(vc: &mut NavBarVC, spinner: u32) {
    // IDA 0x553a4: `setPageLoadActivityIndicator:` stores the spinner.
    vc.page_spinner = Some(spinner);
}

// 0x553c8 — -[RobloxNavBarViewController loadingOverlay]
// type: UIView *__cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController loadingOverlay]")]
pub fn stub_0x553c8(vc: &NavBarVC) -> Option<u32> {
    // IDA 0x553c8: `loadingOverlay` answers the overlay handle.
    vc.overlay
}

// 0x553d8 — -[RobloxNavBarViewController setLoadingOverlay:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setLoadingOverlay:]")]
pub fn stub_0x553d8(vc: &mut NavBarVC, overlay: u32) {
    // IDA 0x553d8: `setLoadingOverlay:` stores the overlay handle.
    vc.overlay = Some(overlay);
}

// 0x553fc — -[RobloxNavBarViewController loadingLabel]
// type: UILabel *__cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController loadingLabel]")]
pub fn stub_0x553fc(vc: &NavBarVC) -> Option<u32> {
    // IDA 0x553fc: `loadingLabel` answers the label handle.
    vc.load_label
}

// 0x5540c — -[RobloxNavBarViewController setLoadingLabel:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setLoadingLabel:]")]
pub fn stub_0x5540c(vc: &mut NavBarVC, label: u32) {
    // IDA 0x5540c: `setLoadingLabel:` stores the label handle.
    vc.load_label = Some(label);
}

// 0x55430 — -[RobloxNavBarViewController btnHome]
// type: UIBarButtonItem *__cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController btnHome]")]
pub fn stub_0x55430(vc: &NavBarVC) -> Option<u32> {
    // IDA 0x55430: `btnHome` answers the button handle.
    vc.btn_home
}

// 0x55440 — -[RobloxNavBarViewController setBtnHome:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setBtnHome:]")]
pub fn stub_0x55440(vc: &mut NavBarVC, btn: u32) {
    // IDA 0x55440: `setBtnHome:` stores the button handle.
    vc.btn_home = Some(btn);
}

// 0x55464 — -[RobloxNavBarViewController robuxImageView]
// type: UIImageView *__cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController robuxImageView]")]
pub fn stub_0x55464(vc: &NavBarVC) -> Option<u32> {
    // IDA 0x55464: `robuxImageView` answers the image handle.
    vc.robux_img
}

// 0x55474 — -[RobloxNavBarViewController setRobuxImageView:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setRobuxImageView:]")]
pub fn stub_0x55474(vc: &mut NavBarVC, img: u32) {
    // IDA 0x55474: `setRobuxImageView:` stores the image handle.
    vc.robux_img = Some(img);
}

// 0x55498 — -[RobloxNavBarViewController tixImageView]
// type: UIImageView *__cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController tixImageView]")]
pub fn stub_0x55498(vc: &NavBarVC) -> Option<u32> {
    // IDA 0x55498: `tixImageView` answers the image handle.
    vc.tix_img
}

// 0x554a8 — -[RobloxNavBarViewController setTixImageView:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setTixImageView:]")]
pub fn stub_0x554a8(vc: &mut NavBarVC, img: u32) {
    // IDA 0x554a8: `setTixImageView:` stores the image handle.
    vc.tix_img = Some(img);
}

// 0x554cc — __GLOBAL__I_a_28
#[doc(alias = "global constructor keyed to_a_28")]
pub fn stub_0x554cc() -> u32 {
    // IDA 0x554cc: `__GLOBAL__I_a_28` — see `GLOBAL_A28_INIT`.
    *GLOBAL_A28_INIT
}

// 0x55664 — -[StoreManager init]
// type: StoreManager *__cdecl(StoreManager *self, SEL)
#[doc(alias = "-[StoreManager init]")]
pub fn stub_0x55664() -> StoreMgr {
    // IDA 0x55664: `StoreManager init` chains to super (0x5567e..0x55688)
    // and seeds the throttle gaps 5/5/5 with retry limit 20
    // (0x556ac..0x556ce); the queue/block glue folds into the host —
    // see `stub_0x55754`.
    StoreMgr { robux_min: 5, bc_min: 5, catalog_min: 5, retry_limit: 20 }
}

// 0x55754 — ___20-[StoreManager init]_block_invoke
// type: int __fastcall(int)
#[doc(alias = "___20-[StoreManager init]_block_invoke")]
pub fn stub_0x55754(mgr: &mut StoreMgr, robux: u32, bc: u32, catalog: u32, retry: u32) {
    // IDA 0x55754: the init block overwrites the gaps from the settings
    // service (0x55774..0x557c4).
    mgr.robux_min = robux;
    mgr.bc_min = bc;
    mgr.catalog_min = catalog;
    mgr.retry_limit = retry;
}

// 0x557c8 — ___copy_helper_block__16
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__16")]
pub fn stub_0x557c8() {
    // IDA 0x557c8: `__copy_helper_block__16` retains captures; `Arc`
    // glue covers it — no-op.
}

// 0x557d4 — ___destroy_helper_block__16
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__16")]
pub fn stub_0x557d4() {
    // IDA 0x557d4: `__destroy_helper_block__16` releases captures (pair
    // of 0x557c8); `Arc` glue covers it — no-op.
}

// 0x557dc — +[StoreManager getStoreMgr]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[StoreManager getStoreMgr]")]
pub fn stub_0x557dc() -> ! {
    todo!("0x557dc +[StoreManager getStoreMgr]")
}

// 0x55838 — ___27+[StoreManager getStoreMgr]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___27+[StoreManager getStoreMgr]_block_invoke")]
pub fn stub_0x55838() -> ! {
    todo!("0x55838 ___27+[StoreManager getStoreMgr]_block_invoke")
}

// 0x5586c — ___copy_helper_block_23
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_23")]
pub fn stub_0x5586c() -> ! {
    todo!("0x5586c ___copy_helper_block_23")
}

// 0x55878 — ___destroy_helper_block_24
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_24")]
pub fn stub_0x55878() -> ! {
    todo!("0x55878 ___destroy_helper_block_24")
}

// 0x55880 — -[StoreManager canMakePurchase]
// type: char __cdecl(StoreManager *self, SEL)
#[doc(alias = "-[StoreManager canMakePurchase]")]
pub fn stub_0x55880() -> ! {
    todo!("0x55880 -[StoreManager canMakePurchase]")
}

// 0x558d0 — -[StoreManager request:didFailWithError:]
// type: void __cdecl(StoreManager *self, SEL, id, id)
#[doc(alias = "-[StoreManager request:didFailWithError:]")]
pub fn stub_0x558d0() -> ! {
    todo!("0x558d0 -[StoreManager request:didFailWithError:]")
}

// 0x559d0 — -[StoreManager requestDidFinish:]
// type: void __cdecl(StoreManager *self, SEL, id)
#[doc(alias = "-[StoreManager requestDidFinish:]")]
pub fn stub_0x559d0() -> ! {
    todo!("0x559d0 -[StoreManager requestDidFinish:]")
}

// 0x55a9c — -[StoreManager restrictTimeBoundPurchase:]
// type: char __cdecl(StoreManager *self, SEL, id)
#[doc(alias = "-[StoreManager restrictTimeBoundPurchase:]")]
pub fn stub_0x55a9c() -> ! {
    todo!("0x55a9c -[StoreManager restrictTimeBoundPurchase:]")
}

// 0x55c68 — -[StoreManager reset]
// type: void __cdecl(StoreManager *self, SEL)
#[doc(alias = "-[StoreManager reset]")]
pub fn stub_0x55c68() -> ! {
    todo!("0x55c68 -[StoreManager reset]")
}

// 0x55d04 — -[StoreManager recordPurchaseTime:]
// type: void __cdecl(StoreManager *self, SEL, id)
#[doc(alias = "-[StoreManager recordPurchaseTime:]")]
pub fn stub_0x55d04() -> ! {
    todo!("0x55d04 -[StoreManager recordPurchaseTime:]")
}

// 0x55e94 — -[StoreManager productsRequest:didReceiveResponse:]
// type: void __cdecl(StoreManager *self, SEL, id, id)
#[doc(alias = "-[StoreManager productsRequest:didReceiveResponse:]")]
pub fn stub_0x55e94() -> ! {
    todo!("0x55e94 -[StoreManager productsRequest:didReceiveResponse:]")
}

// 0x56894 — -[StoreManager requestProductData:]
// type: void __cdecl(StoreManager *self, SEL, id)
#[doc(alias = "-[StoreManager requestProductData:]")]
pub fn stub_0x56894() -> ! {
    todo!("0x56894 -[StoreManager requestProductData:]")
}

// 0x56914 — -[StoreManager purchaseProduct:]
// type: void __cdecl(StoreManager *self, SEL, id)
#[doc(alias = "-[StoreManager purchaseProduct:]")]
pub fn stub_0x56914() -> ! {
    todo!("0x56914 -[StoreManager purchaseProduct:]")
}

// 0x569b4 — -[StoreManager verifyIfCorrectUser]
// type: int __cdecl(StoreManager *self, SEL)
#[doc(alias = "-[StoreManager verifyIfCorrectUser]")]
pub fn stub_0x569b4() -> ! {
    todo!("0x569b4 -[StoreManager verifyIfCorrectUser]")
}

// 0x56ad0 — -[StoreManager completeTransaction:]
// type: void __cdecl(StoreManager *self, SEL, id)
#[doc(alias = "-[StoreManager completeTransaction:]")]
pub fn stub_0x56ad0() -> ! {
    todo!("0x56ad0 -[StoreManager completeTransaction:]")
}

// 0x56d80 — -[StoreManager endTransaction:paymentTransaction:paymentQueue:]
// type: void __cdecl(StoreManager *self, SEL, char, id, id)
#[doc(alias = "-[StoreManager endTransaction:paymentTransaction:paymentQueue:]")]
pub fn stub_0x56d80() -> ! {
    todo!("0x56d80 -[StoreManager endTransaction:paymentTransaction:paymentQueue:]")
}

// 0x572e4 — ___63-[StoreManager endTransaction:paymentTransaction:paymentQueue:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___63-[StoreManager endTransaction:paymentTransaction:paymentQueue:]_block_invoke")]
pub fn stub_0x572e4() -> ! {
    todo!("0x572e4 ___63-[StoreManager endTransaction:paymentTransaction:paymentQueue:]_block_invoke")
}

// 0x573b0 — ___copy_helper_block_212
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_212")]
pub fn stub_0x573b0() -> ! {
    todo!("0x573b0 ___copy_helper_block_212")
}

// 0x573bc — ___destroy_helper_block_213
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_213")]
pub fn stub_0x573bc() -> ! {
    todo!("0x573bc ___destroy_helper_block_213")
}

// 0x573c4 — ___63-[StoreManager endTransaction:paymentTransaction:paymentQueue:]_block_invoke215
// type: id __fastcall(int)
#[doc(alias = "___63-[StoreManager endTransaction:paymentTransaction:paymentQueue:]_block_invoke215")]
pub fn stub_0x573c4() -> ! {
    todo!("0x573c4 ___63-[StoreManager endTransaction:paymentTransaction:paymentQueue:]_block_invoke215")
}

// 0x57410 — ___copy_helper_block_216
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_216")]
pub fn stub_0x57410() -> ! {
    todo!("0x57410 ___copy_helper_block_216")
}

// 0x57434 — ___destroy_helper_block_217
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_217")]
pub fn stub_0x57434() -> ! {
    todo!("0x57434 ___destroy_helper_block_217")
}

// 0x57450 — ___63-[StoreManager endTransaction:paymentTransaction:paymentQueue:]_block_invoke219
// type: id __fastcall(int)
#[doc(alias = "___63-[StoreManager endTransaction:paymentTransaction:paymentQueue:]_block_invoke219")]
pub fn stub_0x57450() -> ! {
    todo!("0x57450 ___63-[StoreManager endTransaction:paymentTransaction:paymentQueue:]_block_invoke219")
}

// 0x5751c — ___copy_helper_block_222
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_222")]
pub fn stub_0x5751c() -> ! {
    todo!("0x5751c ___copy_helper_block_222")
}

// 0x57528 — ___destroy_helper_block_223
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_223")]
pub fn stub_0x57528() -> ! {
    todo!("0x57528 ___destroy_helper_block_223")
}

// 0x57530 — -[StoreManager failedTransaction:]
// type: void __cdecl(StoreManager *self, SEL, id)
#[doc(alias = "-[StoreManager failedTransaction:]")]
pub fn stub_0x57530() -> ! {
    todo!("0x57530 -[StoreManager failedTransaction:]")
}

// 0x5763c — -[StoreManager restoreTransaction:]
// type: void __cdecl(StoreManager *self, SEL, id)
#[doc(alias = "-[StoreManager restoreTransaction:]")]
pub fn stub_0x5763c() -> ! {
    todo!("0x5763c -[StoreManager restoreTransaction:]")
}

// 0x57740 — -[StoreManager paymentQueue:updatedTransactions:]
// type: void __cdecl(StoreManager *self, SEL, id, id)
#[doc(alias = "-[StoreManager paymentQueue:updatedTransactions:]")]
pub fn stub_0x57740() -> ! {
    todo!("0x57740 -[StoreManager paymentQueue:updatedTransactions:]")
}

// 0x5784c — -[StoreManager encode:length:]
// type: id __cdecl(StoreManager *self, SEL, const char *, int)
#[doc(alias = "-[StoreManager encode:length:]")]
pub fn stub_0x5784c() -> ! {
    todo!("0x5784c -[StoreManager encode:length:]")
}

// 0x5796c — -[StoreManager verifyReceipt:forProductId:paymentTransaction:paymentQueue:]
// type: void __cdecl(StoreManager *self, SEL, id, id, id, id)
#[doc(alias = "-[StoreManager verifyReceipt:forProductId:paymentTransaction:paymentQueue:]")]
pub fn stub_0x5796c() -> ! {
    todo!("0x5796c -[StoreManager verifyReceipt:forProductId:paymentTransaction:paymentQueue:]")
}

// 0x57da0 — ___75-[StoreManager verifyReceipt:forProductId:paymentTransaction:paymentQueue:]_block_invoke
// type: void __fastcall(int, void *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "___75-[StoreManager verifyReceipt:forProductId:paymentTransaction:paymentQueue:]_block_invoke")]
pub fn stub_0x57da0() -> ! {
    todo!("0x57da0 ___75-[StoreManager verifyReceipt:forProductId:paymentTransaction:paymentQueue:]_block_invoke")
}

// 0x57f28 — ___75-[StoreManager verifyReceipt:forProductId:paymentTransaction:paymentQueue:]_block_invoke_2
// type: void __cdecl(id)
#[doc(alias = "___75-[StoreManager verifyReceipt:forProductId:paymentTransaction:paymentQueue:]_block_invoke_2")]
pub fn stub_0x57f28() -> ! {
    todo!("0x57f28 ___75-[StoreManager verifyReceipt:forProductId:paymentTransaction:paymentQueue:]_block_invoke_2")
}

// 0x57f98 — ___copy_helper_block_319
// type: void __fastcall(int, const void **)
#[doc(alias = "___copy_helper_block_319")]
pub fn stub_0x57f98() -> ! {
    todo!("0x57f98 ___copy_helper_block_319")
}

// 0x57fc8 — ___destroy_helper_block_320
// type: void __fastcall(const void **)
#[doc(alias = "___destroy_helper_block_320")]
pub fn stub_0x57fc8() -> ! {
    todo!("0x57fc8 ___destroy_helper_block_320")
}

// 0x57fec — __GLOBAL__I_a_29
#[doc(alias = "global constructor keyed to_a_29")]
pub fn stub_0x57fec() -> ! {
    todo!("0x57fec global constructor keyed to_a_29")
}

// 0x58184 — -[UIWebViewCacheManager init]
// type: UIWebViewCacheManager *__cdecl(UIWebViewCacheManager *self, SEL)
#[doc(alias = "-[UIWebViewCacheManager init]")]
pub fn stub_0x58184() -> ! {
    todo!("0x58184 -[UIWebViewCacheManager init]")
}

// 0x582f8 — ___29-[UIWebViewCacheManager init]_block_invoke
// type: int __fastcall(int)
#[doc(alias = "___29-[UIWebViewCacheManager init]_block_invoke")]
pub fn stub_0x582f8() -> ! {
    todo!("0x582f8 ___29-[UIWebViewCacheManager init]_block_invoke")
}

// 0x58334 — ___copy_helper_block__17
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__17")]
pub fn stub_0x58334() -> ! {
    todo!("0x58334 ___copy_helper_block__17")
}

// 0x58340 — ___destroy_helper_block__17
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__17")]
pub fn stub_0x58340() -> ! {
    todo!("0x58340 ___destroy_helper_block__17")
}

// 0x58348 — -[UIWebViewCacheManager dealloc]
// type: void __cdecl(UIWebViewCacheManager *self, SEL)
#[doc(alias = "-[UIWebViewCacheManager dealloc]")]
pub fn stub_0x58348() -> ! {
    todo!("0x58348 -[UIWebViewCacheManager dealloc]")
}

// 0x583a8 — -[UIWebViewCacheManager baseUrlDidChange:]
// type: void __cdecl(UIWebViewCacheManager *self, SEL, id)
#[doc(alias = "-[UIWebViewCacheManager baseUrlDidChange:]")]
pub fn stub_0x583a8() -> ! {
    todo!("0x583a8 -[UIWebViewCacheManager baseUrlDidChange:]")
}

// 0x583b8 — -[UIWebViewCacheManager gotDidLeaveGameNotification:]
// type: void __cdecl(UIWebViewCacheManager *self, SEL, id)
#[doc(alias = "-[UIWebViewCacheManager gotDidLeaveGameNotification:]")]
pub fn stub_0x583b8() -> ! {
    todo!("0x583b8 -[UIWebViewCacheManager gotDidLeaveGameNotification:]")
}

// 0x583f0 — -[UIWebViewCacheManager setPagesToPreload]
// type: void __cdecl(UIWebViewCacheManager *self, SEL)
#[doc(alias = "-[UIWebViewCacheManager setPagesToPreload]")]
pub fn stub_0x583f0() -> ! {
    todo!("0x583f0 -[UIWebViewCacheManager setPagesToPreload]")
}

// 0x58574 — ___copy_helper_block_55
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_55")]
pub fn stub_0x58574() -> ! {
    todo!("0x58574 ___copy_helper_block_55")
}

// 0x58588 — -[UIWebViewCacheManager flush]
// type: void __cdecl(UIWebViewCacheManager *self, SEL)
#[doc(alias = "-[UIWebViewCacheManager flush]")]
pub fn stub_0x58588() -> ! {
    todo!("0x58588 -[UIWebViewCacheManager flush]")
}

// 0x585dc — -[UIWebViewCacheManager preloadDesignatedWebViews]
// type: char __cdecl(UIWebViewCacheManager *self, SEL)
#[doc(alias = "-[UIWebViewCacheManager preloadDesignatedWebViews]")]
pub fn stub_0x585dc() -> ! {
    todo!("0x585dc -[UIWebViewCacheManager preloadDesignatedWebViews]")
}

// 0x58658 — ___50-[UIWebViewCacheManager preloadDesignatedWebViews]_block_invoke
// type: int __fastcall(int)
#[doc(alias = "___50-[UIWebViewCacheManager preloadDesignatedWebViews]_block_invoke")]
pub fn stub_0x58658() -> ! {
    todo!("0x58658 ___50-[UIWebViewCacheManager preloadDesignatedWebViews]_block_invoke")
}

// 0x58858 — -[UIWebViewCacheManager designatedWebviewsToHomePages]
// type: void __cdecl(UIWebViewCacheManager *self, SEL)
#[doc(alias = "-[UIWebViewCacheManager designatedWebviewsToHomePages]")]
pub fn stub_0x58858() -> ! {
    todo!("0x58858 -[UIWebViewCacheManager designatedWebviewsToHomePages]")
}

// 0x588b8 — ___54-[UIWebViewCacheManager designatedWebviewsToHomePages]_block_invoke
// type: int __fastcall(int)
#[doc(alias = "___54-[UIWebViewCacheManager designatedWebviewsToHomePages]_block_invoke")]
pub fn stub_0x588b8() -> ! {
    todo!("0x588b8 ___54-[UIWebViewCacheManager designatedWebviewsToHomePages]_block_invoke")
}

// 0x58a08 — -[UIWebViewCacheManager getPreloadedWebViewForUrl:]
// type: id __cdecl(UIWebViewCacheManager *self, SEL, id)
#[doc(alias = "-[UIWebViewCacheManager getPreloadedWebViewForUrl:]")]
pub fn stub_0x58a08() -> ! {
    todo!("0x58a08 -[UIWebViewCacheManager getPreloadedWebViewForUrl:]")
}

// 0x58d48 — -[RobloxPageViewController handleStartGameFailure]
// type: void __cdecl(RobloxPageViewController *self, SEL)
#[doc(alias = "-[RobloxPageViewController handleStartGameFailure]")]
pub fn stub_0x58d48() -> ! {
    todo!("0x58d48 -[RobloxPageViewController handleStartGameFailure]")
}

// 0x58d4c — -[RobloxPageViewController handleStartGameSuccess]
// type: void __cdecl(RobloxPageViewController *self, SEL)
#[doc(alias = "-[RobloxPageViewController handleStartGameSuccess]")]
pub fn stub_0x58d4c() -> ! {
    todo!("0x58d4c -[RobloxPageViewController handleStartGameSuccess]")
}

// 0x58d50 — -[RobloxPageViewController initWithCoder:]
// type: RobloxPageViewController *__cdecl(RobloxPageViewController *self, SEL, id)
#[doc(alias = "-[RobloxPageViewController initWithCoder:]")]
pub fn stub_0x58d50() -> ! {
    todo!("0x58d50 -[RobloxPageViewController initWithCoder:]")
}

// 0x58d7c — -[RobloxPageViewController viewDidLoad]
// type: void __cdecl(RobloxPageViewController *self, SEL)
#[doc(alias = "-[RobloxPageViewController viewDidLoad]")]
pub fn stub_0x58d7c() -> ! {
    todo!("0x58d7c -[RobloxPageViewController viewDidLoad]")
}

// 0x58e20 — -[RobloxPageViewController viewWillAppear:]
// type: void __cdecl(RobloxPageViewController *self, SEL, char)
#[doc(alias = "-[RobloxPageViewController viewWillAppear:]")]
pub fn stub_0x58e20() -> ! {
    todo!("0x58e20 -[RobloxPageViewController viewWillAppear:]")
}

// 0x58e4c — -[RobloxPageViewController shouldAutorotate]
// type: char __cdecl(RobloxPageViewController *self, SEL)
#[doc(alias = "-[RobloxPageViewController shouldAutorotate]")]
pub fn stub_0x58e4c() -> ! {
    todo!("0x58e4c -[RobloxPageViewController shouldAutorotate]")
}

// 0x58e50 — -[RobloxPageViewController supportedInterfaceOrientations]
// type: unsigned int __cdecl(RobloxPageViewController *self, SEL)
#[doc(alias = "-[RobloxPageViewController supportedInterfaceOrientations]")]
pub fn stub_0x58e50() -> ! {
    todo!("0x58e50 -[RobloxPageViewController supportedInterfaceOrientations]")
}

// 0x58eb8 — -[RobloxPageViewController shouldAutorotateToInterfaceOrientation:]
// type: char __cdecl(RobloxPageViewController *self, SEL, int)
#[doc(alias = "-[RobloxPageViewController shouldAutorotateToInterfaceOrientation:]")]
pub fn stub_0x58eb8() -> ! {
    todo!("0x58eb8 -[RobloxPageViewController shouldAutorotateToInterfaceOrientation:]")
}

// 0x58f40 — -[NSString(Escaping) stringWithPercentEscape]_0
// type: NSString *__cdecl(NSString *self, SEL)
#[doc(alias = "-[NSString(Escaping) stringWithPercentEscape]_0")]
pub fn stub_0x58f40() -> ! {
    todo!("0x58f40 -[NSString(Escaping) stringWithPercentEscape]_0")
}

// 0x59038 — -[LoginManager init]
// type: LoginManager *__cdecl(LoginManager *self, SEL)
#[doc(alias = "-[LoginManager init]")]
pub fn stub_0x59038() -> ! {
    todo!("0x59038 -[LoginManager init]")
}

// 0x5913c — -[LoginManager dealloc]
// type: void __cdecl(LoginManager *self, SEL)
#[doc(alias = "-[LoginManager dealloc]")]
pub fn stub_0x5913c() -> ! {
    todo!("0x5913c -[LoginManager dealloc]")
}

// 0x591a0 — -[LoginManager applicationWillTerminate]
// type: void __cdecl(LoginManager *self, SEL)
#[doc(alias = "-[LoginManager applicationWillTerminate]")]
pub fn stub_0x591a0() -> ! {
    todo!("0x591a0 -[LoginManager applicationWillTerminate]")
}

// 0x592a0 — -[LoginManager getRememberPassword]
// type: char __cdecl(LoginManager *self, SEL)
#[doc(alias = "-[LoginManager getRememberPassword]")]
pub fn stub_0x592a0() -> ! {
    todo!("0x592a0 -[LoginManager getRememberPassword]")
}

// 0x592b0 — -[LoginManager setRememberPassword:]
// type: void __cdecl(LoginManager *self, SEL, char)
#[doc(alias = "-[LoginManager setRememberPassword:]")]
pub fn stub_0x592b0() -> ! {
    todo!("0x592b0 -[LoginManager setRememberPassword:]")
}

// 0x594e4 — -[LoginManager getLoginFailedNotification]
// type: id __cdecl(LoginManager *self, SEL)
#[doc(alias = "-[LoginManager getLoginFailedNotification]")]
pub fn stub_0x594e4() -> ! {
    todo!("0x594e4 -[LoginManager getLoginFailedNotification]")
}

// 0x594f4 — -[LoginManager getLoginSuccessfulNotification]
// type: id __cdecl(LoginManager *self, SEL)
#[doc(alias = "-[LoginManager getLoginSuccessfulNotification]")]
pub fn stub_0x594f4() -> ! {
    todo!("0x594f4 -[LoginManager getLoginSuccessfulNotification]")
}

// 0x59504 — -[LoginManager updateUserInfo:password:]
// type: void __cdecl(LoginManager *self, SEL, id, id)
#[doc(alias = "-[LoginManager updateUserInfo:password:]")]
pub fn stub_0x59504() -> ! {
    todo!("0x59504 -[LoginManager updateUserInfo:password:]")
}

// 0x59690 — -[LoginManager isConnectedToInternet]
// type: char __cdecl(LoginManager *self, SEL)
#[doc(alias = "-[LoginManager isConnectedToInternet]")]
pub fn stub_0x59690() -> ! {
    todo!("0x59690 -[LoginManager isConnectedToInternet]")
}

// 0x598e4 — -[LoginManager doLogout]
// type: void __cdecl(LoginManager *self, SEL)
#[doc(alias = "-[LoginManager doLogout]")]
pub fn stub_0x598e4() -> ! {
    todo!("0x598e4 -[LoginManager doLogout]")
}

// 0x59a6c — ___24-[LoginManager doLogout]_block_invoke
// type: id __fastcall(int, int, int, int)
#[doc(alias = "___24-[LoginManager doLogout]_block_invoke")]
pub fn stub_0x59a6c() -> ! {
    todo!("0x59a6c ___24-[LoginManager doLogout]_block_invoke")
}

// 0x59ae8 — -[LoginManager doLoginWithUsername:password:]
// type: void __cdecl(LoginManager *self, SEL, id, id)
#[doc(alias = "-[LoginManager doLoginWithUsername:password:]")]
pub fn stub_0x59ae8() -> ! {
    todo!("0x59ae8 -[LoginManager doLoginWithUsername:password:]")
}

// 0x59ecc — ___45-[LoginManager doLoginWithUsername:password:]_block_invoke
// type: id __fastcall(int, int, int, int)
#[doc(alias = "___45-[LoginManager doLoginWithUsername:password:]_block_invoke")]
pub fn stub_0x59ecc() -> ! {
    todo!("0x59ecc ___45-[LoginManager doLoginWithUsername:password:]_block_invoke")
}

// 0x5a0e4 — -[LoginManager processLoginResponse:loginData:loginError:userLoginInfo:]
// type: id __cdecl(LoginManager *self, SEL, id, id, id, id)
#[doc(alias = "-[LoginManager processLoginResponse:loginData:loginError:userLoginInfo:]")]
pub fn stub_0x5a0e4() -> ! {
    todo!("0x5a0e4 -[LoginManager processLoginResponse:loginData:loginError:userLoginInfo:]")
}

// 0x5a42c — -[LoginManager processLogOutResponse:logoutData:logoutError:]
// type: id __cdecl(LoginManager *self, SEL, id, id, id)
#[doc(alias = "-[LoginManager processLogOutResponse:logoutData:logoutError:]")]
pub fn stub_0x5a42c() -> ! {
    todo!("0x5a42c -[LoginManager processLogOutResponse:logoutData:logoutError:]")
}

// 0x5a6a8 — -[LoginManager processSuccessfulLoginResponse:httpResponse:userLoginInfo:]
// type: id __cdecl(LoginManager *self, SEL, id, id, id)
#[doc(alias = "-[LoginManager processSuccessfulLoginResponse:httpResponse:userLoginInfo:]")]
pub fn stub_0x5a6a8() -> ! {
    todo!("0x5a6a8 -[LoginManager processSuccessfulLoginResponse:httpResponse:userLoginInfo:]")
}

// 0x5ac78 — -[LoginManager processSuccessfulLogoutResponse:httpResponse:]
// type: id __cdecl(LoginManager *self, SEL, id, id)
#[doc(alias = "-[LoginManager processSuccessfulLogoutResponse:httpResponse:]")]
pub fn stub_0x5ac78() -> ! {
    todo!("0x5ac78 -[LoginManager processSuccessfulLogoutResponse:httpResponse:]")
}

// 0x5ae50 — -[LoginManager processFailureLoginResponse:]
// type: id __cdecl(LoginManager *self, SEL, id)
#[doc(alias = "-[LoginManager processFailureLoginResponse:]")]
pub fn stub_0x5ae50() -> ! {
    todo!("0x5ae50 -[LoginManager processFailureLoginResponse:]")
}

// 0x5b150 — -[LoginManager processFailureLogoutResponse:]
// type: id __cdecl(LoginManager *self, SEL, id)
#[doc(alias = "-[LoginManager processFailureLogoutResponse:]")]
pub fn stub_0x5b150() -> ! {
    todo!("0x5b150 -[LoginManager processFailureLogoutResponse:]")
}

// 0x5b4a0 — -[AgreementController initWithCoder:]
// type: AgreementController *__cdecl(AgreementController *self, SEL, id)
#[doc(alias = "-[AgreementController initWithCoder:]")]
pub fn stub_0x5b4a0() -> ! {
    todo!("0x5b4a0 -[AgreementController initWithCoder:]")
}

// 0x5b4e0 — -[AgreementController init:]
// type: id __cdecl(AgreementController *self, SEL, id)
#[doc(alias = "-[AgreementController init:]")]
pub fn stub_0x5b4e0() -> ! {
    todo!("0x5b4e0 -[AgreementController init:]")
}

// 0x5b550 — -[AgreementController init:newFrame:]
// type: id __cdecl(AgreementController *self, SEL, id, CGRect)
#[doc(alias = "-[AgreementController init:newFrame:]")]
pub fn stub_0x5b550() -> ! {
    todo!("0x5b550 -[AgreementController init:newFrame:]")
}

// 0x5b5fc — -[AgreementController dealloc]
// type: void __cdecl(AgreementController *self, SEL)
#[doc(alias = "-[AgreementController dealloc]")]
pub fn stub_0x5b5fc() -> ! {
    todo!("0x5b5fc -[AgreementController dealloc]")
}

// 0x5b680 — -[AgreementController setUrl:]
// type: void __cdecl(AgreementController *self, SEL, id)
#[doc(alias = "-[AgreementController setUrl:]")]
pub fn stub_0x5b680() -> ! {
    todo!("0x5b680 -[AgreementController setUrl:]")
}

// 0x5b690 — -[AgreementController cancelTouch:]
// type: void __cdecl(AgreementController *self, SEL, id)
#[doc(alias = "-[AgreementController cancelTouch:]")]
pub fn stub_0x5b690() -> ! {
    todo!("0x5b690 -[AgreementController cancelTouch:]")
}

// 0x5b6a4 — -[AgreementController viewDidLoad]
// type: void __cdecl(AgreementController *self, SEL)
#[doc(alias = "-[AgreementController viewDidLoad]")]
pub fn stub_0x5b6a4() -> ! {
    todo!("0x5b6a4 -[AgreementController viewDidLoad]")
}

// 0x5ba90 — -[AgreementController toolBar]
// type: UIToolbar *__cdecl(AgreementController *self, SEL)
#[doc(alias = "-[AgreementController toolBar]")]
pub fn stub_0x5ba90() -> ! {
    todo!("0x5ba90 -[AgreementController toolBar]")
}

// 0x5baa0 — -[AgreementController setToolBar:]
// type: void __cdecl(AgreementController *self, SEL, id)
#[doc(alias = "-[AgreementController setToolBar:]")]
pub fn stub_0x5baa0() -> ! {
    todo!("0x5baa0 -[AgreementController setToolBar:]")
}

// 0x5bac4 — -[AgreementController closeButton]
// type: UIBarButtonItem *__cdecl(AgreementController *self, SEL)
#[doc(alias = "-[AgreementController closeButton]")]
pub fn stub_0x5bac4() -> ! {
    todo!("0x5bac4 -[AgreementController closeButton]")
}

// 0x5bad4 — -[AgreementController setCloseButton:]
// type: void __cdecl(AgreementController *self, SEL, id)
#[doc(alias = "-[AgreementController setCloseButton:]")]
pub fn stub_0x5bad4() -> ! {
    todo!("0x5bad4 -[AgreementController setCloseButton:]")
}

// 0x5baf8 — -[SignUpErrorViewController initWithCoder:]
// type: SignUpErrorViewController *__cdecl(SignUpErrorViewController *self, SEL, id)
#[doc(alias = "-[SignUpErrorViewController initWithCoder:]")]
pub fn stub_0x5baf8() -> ! {
    todo!("0x5baf8 -[SignUpErrorViewController initWithCoder:]")
}

// 0x5bb44 — -[SignUpErrorViewController dealloc]
// type: void __cdecl(SignUpErrorViewController *self, SEL)
#[doc(alias = "-[SignUpErrorViewController dealloc]")]
pub fn stub_0x5bb44() -> ! {
    todo!("0x5bb44 -[SignUpErrorViewController dealloc]")
}

// 0x5bc00 — -[SignUpErrorViewController viewDidLoad]
// type: void __cdecl(SignUpErrorViewController *self, SEL)
#[doc(alias = "-[SignUpErrorViewController viewDidLoad]")]
pub fn stub_0x5bc00() -> ! {
    todo!("0x5bc00 -[SignUpErrorViewController viewDidLoad]")
}

// 0x5bcb8 — -[SignUpErrorViewController observeValueForKeyPath:ofObject:change:context:]
// type: void __cdecl(SignUpErrorViewController *self, SEL, id, id, id, void *)
#[doc(alias = "-[SignUpErrorViewController observeValueForKeyPath:ofObject:change:context:]")]
pub fn stub_0x5bcb8() -> ! {
    todo!("0x5bcb8 -[SignUpErrorViewController observeValueForKeyPath:ofObject:change:context:]")
}

// 0x5bd70 — -[SignUpErrorViewController didReceiveMemoryWarning]
// type: void __cdecl(SignUpErrorViewController *self, SEL)
#[doc(alias = "-[SignUpErrorViewController didReceiveMemoryWarning]")]
pub fn stub_0x5bd70() -> ! {
    todo!("0x5bd70 -[SignUpErrorViewController didReceiveMemoryWarning]")
}

// 0x5bd9c — -[SignUpErrorViewController setSuggestedUsername:]
// type: void __cdecl(SignUpErrorViewController *self, SEL, id)
#[doc(alias = "-[SignUpErrorViewController setSuggestedUsername:]")]
pub fn stub_0x5bd9c() -> ! {
    todo!("0x5bd9c -[SignUpErrorViewController setSuggestedUsername:]")
}

// 0x5bdbc — -[SignUpErrorViewController setMessage:]
// type: void __cdecl(SignUpErrorViewController *self, SEL, id)
#[doc(alias = "-[SignUpErrorViewController setMessage:]")]
pub fn stub_0x5bdbc() -> ! {
    todo!("0x5bdbc -[SignUpErrorViewController setMessage:]")
}

// 0x5be1c — -[SignUpErrorViewController setSignupController:]
// type: void __cdecl(SignUpErrorViewController *self, SEL, id)
#[doc(alias = "-[SignUpErrorViewController setSignupController:]")]
pub fn stub_0x5be1c() -> ! {
    todo!("0x5be1c -[SignUpErrorViewController setSignupController:]")
}

// 0x5be2c — -[SignUpErrorViewController touchesBegan:withEvent:]
// type: void __cdecl(SignUpErrorViewController *self, SEL, id, id)
#[doc(alias = "-[SignUpErrorViewController touchesBegan:withEvent:]")]
pub fn stub_0x5be2c() -> ! {
    todo!("0x5be2c -[SignUpErrorViewController touchesBegan:withEvent:]")
}

// 0x5be5c — -[SignUpErrorViewController touchesEnded:withEvent:]
// type: void __cdecl(SignUpErrorViewController *self, SEL, id, id)
#[doc(alias = "-[SignUpErrorViewController touchesEnded:withEvent:]")]
pub fn stub_0x5be5c() -> ! {
    todo!("0x5be5c -[SignUpErrorViewController touchesEnded:withEvent:]")
}

// 0x5bf68 — -[SignUpErrorViewController messageTextView]
// type: UITextView *__cdecl(SignUpErrorViewController *self, SEL)
#[doc(alias = "-[SignUpErrorViewController messageTextView]")]
pub fn stub_0x5bf68() -> ! {
    todo!("0x5bf68 -[SignUpErrorViewController messageTextView]")
}

#[cfg(test)]
mod navbar_web_batch_tests {
    use super::*;
    use crate::generated_172::NavBarVC;
    use std::sync::atomic::Ordering;

    #[test]
    fn launch_and_check() {
        let mut vc = NavBarVC::default();
        stub_0x5479c(&mut vc);
        assert_eq!(vc.launches, 1);
        assert!(!stub_0x54a3c(&mut vc, true));
        assert!(stub_0x54a3c(&mut vc, false));
        assert!(vc.back_enabled);
        assert!(stub_0x54c64(&mut vc, false));
        assert!(!vc.home_enabled);
        assert!(!stub_0x54c64(&mut vc, true));
        assert!(vc.home_enabled);
        stub_0x54a28();
        stub_0x54a34();
        stub_0x5508c();
        stub_0x55098();
    }

    #[test]
    fn page_lifecycle() {
        let mut vc = NavBarVC::default();
        stub_0x54d58(&mut vc, false);
        assert!(!vc.loading);
        stub_0x54d58(&mut vc, true);
        assert!(vc.loading);
        stub_0x54d2c(&mut vc);
        assert!(!vc.loading);
        stub_0x54d58(&mut vc, true);
        vc.back_enabled = true;
        stub_0x54db4(&mut vc, true);
        assert!(!vc.loading);
        assert!(vc.back_enabled);
        assert_eq!(vc.web_depth, 1);
        stub_0x551d8(&mut vc);
        assert_eq!(vc.web_depth, 0);
        assert!(!vc.back_enabled);
        vc.refresh_robux = true;
        stub_0x54db4(&mut vc, false);
        assert_eq!(vc.info_refresh, 1);
        assert!(!vc.refresh_robux);
        stub_0x54e40(&mut vc, true);
        assert_eq!(vc.info_refresh, 2);
        vc.fullscreen = Some("x".to_string());
        stub_0x54d0c(&mut vc);
        assert_eq!(vc.fullscreen, None);
        vc.fullscreen = Some("x".to_string());
        stub_0x54d1c(&mut vc);
        assert_eq!(vc.fullscreen, None);
    }

    #[test]
    fn menu_and_jumps() {
        let mut vc = NavBarVC::default();
        stub_0x54ff0(&mut vc, true);
        assert!(!vc.dismissed);
        stub_0x54ff0(&mut vc, false);
        assert!(vc.dismissed);
        stub_0x5523c(42);
        assert_eq!(super::JUMP_NAVIGATE.load(Ordering::SeqCst), 42);
        stub_0x5524c(77);
        assert_eq!(super::JUMP_PROGRESS.load(Ordering::SeqCst), 77);
        assert_eq!(stub_0x550a0(&vc), false);
        vc.recent = true;
        assert!(stub_0x550a0(&vc));
        assert_eq!(stub_0x5525c(&vc), None);
        stub_0x5526c(&mut vc, 3);
        assert_eq!(stub_0x5525c(&vc), Some(3));
        stub_0x552a0(&mut vc, 4);
        assert_eq!(stub_0x55290(&vc), Some(4));
        assert_eq!(vc.web_view, None);
        stub_0x550b0(&mut vc, None);
        assert_eq!(vc.web_view, None);
        stub_0x550b0(&mut vc, Some(9));
        assert_eq!(vc.web_view, Some(9));
    }
}

#[cfg(test)]
mod outlet_store_batch_tests {
    use super::*;
    use crate::generated_172::NavBarVC;

    #[test]
    fn outlets() {
        let mut vc = NavBarVC::default();
        stub_0x552d4(&mut vc, 1);
        assert_eq!(stub_0x552c4(&vc), Some(1));
        stub_0x55308(&mut vc, 2);
        assert_eq!(stub_0x552f8(&vc), Some(2));
        stub_0x5533c(&mut vc, 3);
        assert_eq!(stub_0x5532c(&vc), Some(3));
        stub_0x55370(&mut vc, 4);
        assert_eq!(stub_0x55360(&vc), Some(4));
        stub_0x553a4(&mut vc, 5);
        assert_eq!(stub_0x55394(&vc), Some(5));
        stub_0x553d8(&mut vc, 6);
        assert_eq!(stub_0x553c8(&vc), Some(6));
        stub_0x5540c(&mut vc, 7);
        assert_eq!(stub_0x553fc(&vc), Some(7));
        stub_0x55440(&mut vc, 8);
        assert_eq!(stub_0x55430(&vc), Some(8));
        stub_0x55474(&mut vc, 9);
        assert_eq!(stub_0x55464(&vc), Some(9));
        stub_0x554a8(&mut vc, 10);
        assert_eq!(stub_0x55498(&vc), Some(10));
        assert_eq!(stub_0x554cc(), 1);
        stub_0x557c8();
        stub_0x557d4();
    }

    #[test]
    fn store_mgr() {
        let mut mgr = stub_0x55664();
        assert_eq!(mgr, StoreMgr { robux_min: 5, bc_min: 5, catalog_min: 5, retry_limit: 20 });
        stub_0x55754(&mut mgr, 10, 15, 30, 60);
        assert_eq!(mgr, StoreMgr { robux_min: 10, bc_min: 15, catalog_min: 30, retry_limit: 60 });
    }
}
