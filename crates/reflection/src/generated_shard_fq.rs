// Auto-generated shard FQ — 150 stubs EA-sorted asc 0x51e54..0x56894 (global gap filler not yet in reflection, 22415->22565 distinct)
// Source: ida/export.json (85545 funcs) EA asc not in crates/reflection/src/*.rs, next 150
// Format: // 0xADDR — mangled + doc alias + stub using rbx_core::SharedPtr not boost

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;
/// `MainViewController` outlet presence (IDA 0x51e68-0x51fd0): hosted
/// view plus the ogre window/view, roblox view, ogre controller and
/// last nongame controller. Views live out of slice.
pub(crate) static MAINVIEW_SET: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static MAINVIEW_OGRE_WINDOW: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static MAINVIEW_OGRE_VIEW: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static MAINVIEW_ROBLOX_VIEW: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static MAINVIEW_OGRE_VC: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static MAINVIEW_LAST_NONGAME: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
/// `RobloxAnimatingPageViewController` state (IDA 0x52178-0x52614):
/// background flag plus appear count. Animations live out of slice.
pub(crate) static ANIM_IN_BACKGROUND: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static ANIM_APPEARS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
/// Page-animation counters (IDA 0x52aec-0x53a6c): zero-position and
/// layer animations, pan-loop flag, layer copies, foreground image x
/// and pan runs. Frames and tweens live out of slice.
pub(crate) static ZERO_ANIMS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static LAST_TWEEN: std::sync::LazyLock<parking_lot::Mutex<f32>> =
    std::sync::LazyLock::new(|| parking_lot::Mutex::new(0.0));
pub(crate) static LAYER_ANIMS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static ANIMATION_LOOPING: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static ANIM_PANS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static FG_COPY: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static BG_COPY: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static FG_IMAGE_X: std::sync::LazyLock<parking_lot::Mutex<f32>> =
    std::sync::LazyLock::new(|| parking_lot::Mutex::new(0.0));
/// Page layer + navbar state (IDA 0x53a98-0x543dc): background image
/// x, animation/background/foreground views, navbar back-button
/// visibility, URL and leave signals plus appear count. Views live
/// out of slice.
pub(crate) static BG_IMAGE_X: std::sync::LazyLock<parking_lot::Mutex<f32>> =
    std::sync::LazyLock::new(|| parking_lot::Mutex::new(0.0));
pub(crate) static ANIM_VIEW: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static IMG_BG: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static IMG_FG: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static NAV_BACK_VISIBLE: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static NAV_URL: std::sync::LazyLock<
    parking_lot::Mutex<String>,
> = std::sync::LazyLock::new(|| parking_lot::Mutex::new(String::new()));
pub(crate) static NAV_LEAVE_SIGNALS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static NAV_APPEARS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
/// Fullscreen-text overlay state (IDA 0x5449c-0x545f8): text,
/// visibility and show count. Spinner + overlay views live out of
/// slice.
pub(crate) static FULLSCREEN_TEXT: std::sync::LazyLock<
    parking_lot::Mutex<String>,
> = std::sync::LazyLock::new(|| parking_lot::Mutex::new(String::new()));
pub(crate) static FULLSCREEN_VISIBLE: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static FULLSCREEN_SHOWS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
/// Place-launch flow (IDA 0x5479c-0x549e4): launch count plus async
/// join-block runs. Defaults/state writes and analytics live out of
/// slice.
pub(crate) static PLACE_LAUNCHES: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static PLACE_LAUNCH_RUNS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
/// Player-info display refreshes (IDA 0x54e40 `UpdatePlayerInfo`
/// path).
pub(crate) static PLAYERINFO_REFRESHES: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
/// Web-view load callbacks (IDA 0x54d2c/0x54d58/0x54db4): fail, start
/// and finish counts.
pub(crate) static WEB_LOAD_FAILS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static WEB_LOAD_STARTS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub(crate) static WEB_LOAD_FINISHES: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);

// 0x51e54 — ___copy_helper_block__13
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__13")]
pub fn stub_51e54() {
    // IDA 0x51e54: `__copy_helper_block__13` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x51e60 — ___destroy_helper_block__13
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__13")]
pub fn stub_51e60() {
    // IDA 0x51e60: `__destroy_helper_block__13` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x51e68 — -[MainViewController switchView:]
// type: void __cdecl(MainViewController *self, SEL, id)
#[doc(alias = "-[MainViewController switchView:]")]
pub fn stub_51e68(view_present: bool) {
    // IDA 0x51e68: `switchView:` sets the hosted view (0x51e74). It
    // records here.
    MAINVIEW_SET.store(view_present, std::sync::atomic::Ordering::SeqCst);
}

// 0x51e78 — -[MainViewController addSubview:]
// type: void __cdecl(MainViewController *self, SEL, id)
#[doc(alias = "-[MainViewController addSubview:]")]
pub fn stub_51e78(view_present: bool) {
    // IDA 0x51e78: `addSubview:` adds the view (same hosted-view shape
    // as 0x51e68). It records here.
    MAINVIEW_SET.store(view_present, std::sync::atomic::Ordering::SeqCst);
}

// 0x51eb8 — -[MainViewController initWithNibName:bundle:]
// type: MainViewController *__cdecl(MainViewController *self, SEL, id, id)
#[doc(alias = "-[MainViewController initWithNibName:bundle:]")]
pub fn stub_51eb8() {
    // IDA 0x51eb8: `initWithNibName:bundle:` supers. Super-init glue;
    // no explicit body.
}

// 0x51ee8 — -[MainViewController viewDidLoad]
// type: void __cdecl(MainViewController *self, SEL)
#[doc(alias = "-[MainViewController viewDidLoad]")]
pub fn stub_51ee8() {
    // IDA 0x51ee8: `viewDidLoad` supers. Super glue; no explicit body.
}

// 0x51f14 — -[MainViewController viewDidUnload]
// type: void __cdecl(MainViewController *self, SEL)
#[doc(alias = "-[MainViewController viewDidUnload]")]
pub fn stub_51f14() {
    // IDA 0x51f14: `viewDidUnload` releases the outlets. Release is
    // drop glue; the outlet flags reset here.
    MAINVIEW_SET.store(false, std::sync::atomic::Ordering::SeqCst);
    MAINVIEW_OGRE_WINDOW.store(false, std::sync::atomic::Ordering::SeqCst);
    MAINVIEW_OGRE_VIEW.store(false, std::sync::atomic::Ordering::SeqCst);
    MAINVIEW_ROBLOX_VIEW.store(false, std::sync::atomic::Ordering::SeqCst);
    MAINVIEW_OGRE_VC.store(false, std::sync::atomic::Ordering::SeqCst);
    MAINVIEW_LAST_NONGAME.store(false, std::sync::atomic::Ordering::SeqCst);
}

// 0x51f40 — -[MainViewController getOgreWindow]
// type: id __cdecl(MainViewController *self, SEL)
#[doc(alias = "-[MainViewController getOgreWindow]")]
pub fn stub_51f40() -> bool {
    // IDA 0x51f40: `getOgreWindow` returns the ivar. Presence reports
    // here.
    MAINVIEW_OGRE_WINDOW.load(std::sync::atomic::Ordering::SeqCst)
}

// 0x51f50 — -[MainViewController setOgreWindow:]
// type: void __cdecl(MainViewController *self, SEL, id)
#[doc(alias = "-[MainViewController setOgreWindow:]")]
pub fn stub_51f50(present: bool) {
    // IDA 0x51f50: `setOgreWindow:` stores the ivar. It records here.
    MAINVIEW_OGRE_WINDOW.store(present, std::sync::atomic::Ordering::SeqCst);
}

// 0x51f60 — -[MainViewController getOgreView]
// type: id __cdecl(MainViewController *self, SEL)
#[doc(alias = "-[MainViewController getOgreView]")]
pub fn stub_51f60() -> bool {
    // IDA 0x51f60: `getOgreView` returns the ivar. Presence reports
    // here.
    MAINVIEW_OGRE_VIEW.load(std::sync::atomic::Ordering::SeqCst)
}

// 0x51f70 — -[MainViewController setOgreView:]
// type: void __cdecl(MainViewController *self, SEL, id)
#[doc(alias = "-[MainViewController setOgreView:]")]
pub fn stub_51f70(present: bool) {
    // IDA 0x51f70: `setOgreView:` stores the ivar. It records here.
    MAINVIEW_OGRE_VIEW.store(present, std::sync::atomic::Ordering::SeqCst);
}

// 0x51f80 — -[MainViewController setRobloxView:]
// type: void __cdecl(MainViewController *self, SEL, RobloxView *)
#[doc(alias = "-[MainViewController setRobloxView:]")]
pub fn stub_51f80(present: bool) {
    // IDA 0x51f80: `setRobloxView:` stores the ivar. It records here.
    MAINVIEW_ROBLOX_VIEW.store(present, std::sync::atomic::Ordering::SeqCst);
}

// 0x51f90 — -[MainViewController getRobloxView]
// type: RobloxView *__cdecl(MainViewController *self, SEL)
#[doc(alias = "-[MainViewController getRobloxView]")]
pub fn stub_51f90() -> bool {
    // IDA 0x51f90: `getRobloxView` returns the ivar. Presence reports
    // here.
    MAINVIEW_ROBLOX_VIEW.load(std::sync::atomic::Ordering::SeqCst)
}

// 0x51fa0 — -[MainViewController getOgreViewController]
// type: id __cdecl(MainViewController *self, SEL)
#[doc(alias = "-[MainViewController getOgreViewController]")]
pub fn stub_51fa0() -> bool {
    // IDA 0x51fa0: `getOgreViewController` returns the ivar. Presence
    // reports here.
    MAINVIEW_OGRE_VC.load(std::sync::atomic::Ordering::SeqCst)
}

// 0x51fb0 — -[MainViewController setOgreViewController:]
// type: void __cdecl(MainViewController *self, SEL, id)
#[doc(alias = "-[MainViewController setOgreViewController:]")]
pub fn stub_51fb0(present: bool) {
    // IDA 0x51fb0: `setOgreViewController:` stores the ivar. It
    // records here.
    MAINVIEW_OGRE_VC.store(present, std::sync::atomic::Ordering::SeqCst);
}

// 0x51fc0 — -[MainViewController setLastNonGameController:]
// type: void __cdecl(MainViewController *self, SEL, id)
#[doc(alias = "-[MainViewController setLastNonGameController:]")]
pub fn stub_51fc0(present: bool) {
    // IDA 0x51fc0: `setLastNonGameController:` stores the ivar. It
    // records here.
    MAINVIEW_LAST_NONGAME.store(present, std::sync::atomic::Ordering::SeqCst);
}

// 0x51fd0 — -[MainViewController getLastNonGameController]
// type: id __cdecl(MainViewController *self, SEL)
#[doc(alias = "-[MainViewController getLastNonGameController]")]
pub fn stub_51fd0() -> bool {
    // IDA 0x51fd0: `getLastNonGameController` returns the ivar.
    // Presence reports here.
    MAINVIEW_LAST_NONGAME.load(std::sync::atomic::Ordering::SeqCst)
}

// 0x51fe0 — __GLOBAL__I_a_27
#[doc(alias = "__GLOBAL__I_a_27")]
pub fn stub_51fe0() {
    // IDA 0x51fe0: `__GLOBAL__I_a_27` runs the `a_27`
    // translation-unit static initializers. Static-init glue; no
    // explicit body.
}

// 0x52178 — -[RobloxAnimatingPageViewController initWithCoder:]
// type: RobloxAnimatingPageViewController *__cdecl(RobloxAnimatingPageViewController *self, SEL, id)
#[doc(alias = "-[RobloxAnimatingPageViewController initWithCoder:]")]
pub fn stub_52178() {
    // IDA 0x52178: `RobloxAnimatingPageViewController::initWithCoder:`
    // supers. Super-init glue; no explicit body.
}

// 0x52280 — -[RobloxAnimatingPageViewController dealloc]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL)
#[doc(alias = "-[RobloxAnimatingPageViewController dealloc]")]
pub fn stub_52280() {
    // IDA 0x52280: `dealloc` drops the page views. Release is drop
    // glue; the background flag resets here.
    ANIM_IN_BACKGROUND.store(false, std::sync::atomic::Ordering::SeqCst);
}

// 0x5233c — -[RobloxAnimatingPageViewController appInBackground:]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL, id)
#[doc(alias = "-[RobloxAnimatingPageViewController appInBackground:]")]
pub fn stub_5233c(backgrounded: bool) {
    // IDA 0x5233c: `appInBackground:` records the backgrounding. It
    // records here.
    ANIM_IN_BACKGROUND.store(backgrounded, std::sync::atomic::Ordering::SeqCst);
}

// 0x5234c — -[RobloxAnimatingPageViewController appInForeground:]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL, id)
#[doc(alias = "-[RobloxAnimatingPageViewController appInForeground:]")]
pub fn stub_5234c() {
    // IDA 0x5234c: `appInForeground:` clears the backgrounding. It
    // records here.
    ANIM_IN_BACKGROUND.store(false, std::sync::atomic::Ordering::SeqCst);
}

// 0x52384 — -[RobloxAnimatingPageViewController removeViewAndAnimation:]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL, id)
#[doc(alias = "-[RobloxAnimatingPageViewController removeViewAndAnimation:]")]
pub fn stub_52384() {
    // IDA 0x52384: `removeViewAndAnimation:` detaches the page.
    // Removal is drop glue; no explicit body.
}

// 0x523d4 — -[RobloxAnimatingPageViewController didReceiveMemoryWarning]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL)
#[doc(alias = "-[RobloxAnimatingPageViewController didReceiveMemoryWarning]")]
pub fn stub_523d4() {
    // IDA 0x523d4: `didReceiveMemoryWarning` supers. Super glue; no
    // explicit body.
}

// 0x52400 — -[RobloxAnimatingPageViewController viewDidLoad]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL)
#[doc(alias = "-[RobloxAnimatingPageViewController viewDidLoad]")]
pub fn stub_52400() {
    // IDA 0x52400: `viewDidLoad` supers. Super glue; no explicit body.
    // [INFERENCE] Body unexamined; standard lifecycle passthrough per
    // surrounding family (0x523d4/0x52614 sibs).
}

// 0x52580 — -[RobloxAnimatingPageViewController getInitialXPosition:]
// type: float __cdecl(RobloxAnimatingPageViewController *self, SEL, id)
#[doc(alias = "-[RobloxAnimatingPageViewController getInitialXPosition:]")]
pub fn stub_52580(view: Option<(f32, f32, f32)>) -> f32 {
    // IDA 0x52580: `getInitialXPosition:` returns 0 for a nil view
    // (0x5258c-0x52602); for a non-positive width it returns the
    // origin y (0x525be-0x525fe), else x minus width (0x525c6-0x52610).
    let Some((x, y, w)) = view else {
        return 0.0;
    };
    if w <= 0.0 { y } else { x - w }
}

// 0x52614 — -[RobloxAnimatingPageViewController viewDidAppear:]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL, char)
#[doc(alias = "-[RobloxAnimatingPageViewController viewDidAppear:]")]
pub fn stub_52614() {
    // IDA 0x52614: `viewDidAppear:` records the appearance. It
    // records here.
    ANIM_APPEARS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x52a50 — -[RobloxAnimatingPageViewController viewDidDisappear:]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL, char)
#[doc(alias = "-[RobloxAnimatingPageViewController viewDidDisappear:]")]
pub fn stub_52a50() {
    // IDA 0x52a50: `viewDidDisappear:` tears the page animations
    // down. The teardown records here.
    ANIM_IN_BACKGROUND.store(false, std::sync::atomic::Ordering::SeqCst);
    ANIMATION_LOOPING.store(false, std::sync::atomic::Ordering::SeqCst);
}

// 0x52aa0 — -[RobloxAnimatingPageViewController hasNaNValue:]
// type: char __cdecl(RobloxAnimatingPageViewController *self, SEL, CGRect)
#[doc(alias = "-[RobloxAnimatingPageViewController hasNaNValue:]")]
pub fn stub_52aa0() -> bool {
    // IDA 0x52aa0: `hasNaNValue:` is stubbed to return 0 in the
    // binary (decompiled 0x52aa0).
    false
}

// 0x52aec — -[RobloxAnimatingPageViewController animateToZeroPosition:copyLayer:defaultTweenTime:]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL, id, id, float)
#[doc(alias = "-[RobloxAnimatingPageViewController animateToZeroPosition:copyLayer:defaultTweenTime:]")]
pub fn stub_52aec(tween: f32) {
    // IDA 0x52aec: `animateToZeroPosition:` guards NaN frames
    // (0x52b12-0x52baa, always clear per 0x52aa0) and animates the
    // layer to zero over the tween (0x52cea-0x52d8e). The run records
    // here.
    *LAST_TWEEN.lock() = tween;
    ZERO_ANIMS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x52dac — ___86-[RobloxAnimatingPageViewController animateToZeroPosition:copyLayer:defaultTweenTime:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___86-[RobloxAnimatingPageViewController animateToZeroPosition:copyLayer:defaultTweenTime:]_block_invoke")]
pub fn stub_52dac() {
    // IDA 0x52dac: the zero-position animation block (continuation of
    // 0x52aec). Animation glue; no explicit body.
}

// 0x52ed4 — ___copy_helper_block__14
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__14")]
pub fn stub_52ed4() {
    // IDA 0x52ed4: `__copy_helper_block__14` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x52ef8 — ___destroy_helper_block__14
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__14")]
pub fn stub_52ef8() {
    // IDA 0x52ef8: `__destroy_helper_block__14` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x52f14 — ___86-[RobloxAnimatingPageViewController animateToZeroPosition:copyLayer:defaultTweenTime:]_block_invoke73
// type: id __fastcall(int)
#[doc(alias = "___86-[RobloxAnimatingPageViewController animateToZeroPosition:copyLayer:defaultTweenTime:]_block_invoke73")]
pub fn stub_52f14() {
    // IDA 0x52f14: the zero-position completion block (continuation of
    // 0x52aec). Animation glue; no explicit body.
}

// 0x52f44 — ___copy_helper_block_76
// type: void __fastcall(int, const void **)
#[doc(alias = "___copy_helper_block_76")]
pub fn stub_52f44() {
    // IDA 0x52f44: `__copy_helper_block_76` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x52f74 — ___destroy_helper_block_77
// type: void __fastcall(const void **)
#[doc(alias = "___destroy_helper_block_77")]
pub fn stub_52f74() {
    // IDA 0x52f74: `__destroy_helper_block_77` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x52f98 — -[RobloxAnimatingPageViewController animateBackground]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL)
#[doc(alias = "-[RobloxAnimatingPageViewController animateBackground]")]
pub fn stub_52f98(frame_nonzero: bool) {
    // IDA 0x52f98: `animateBackground` routes to zero-position when
    // the background frame is nonzero (0x53008), else to the layer
    // path (0x5301e-0x5302c). The branch records here.
    if frame_nonzero {
        ZERO_ANIMS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    } else {
        LAYER_ANIMS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x53034 — -[RobloxAnimatingPageViewController animateForeground]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL)
#[doc(alias = "-[RobloxAnimatingPageViewController animateForeground]")]
pub fn stub_53034(frame_nonzero: bool) {
    // IDA 0x53034: `animateForeground` routes like `animateBackground`
    // (same branch shape as 0x52f98). The branch records here.
    if frame_nonzero {
        ZERO_ANIMS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    } else {
        LAYER_ANIMS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x530d0 — -[RobloxAnimatingPageViewController animateLayer:copyLayer:animationDuration:]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL, id, id, float)
#[doc(alias = "-[RobloxAnimatingPageViewController animateLayer:copyLayer:animationDuration:]")]
pub fn stub_530d0(duration: f32) {
    // IDA 0x530d0: `animateLayer:copyLayer:animationDuration:`
    // animates the layer over the duration. The run records here.
    *LAST_TWEEN.lock() = duration;
    LAYER_ANIMS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x5340c — ___78-[RobloxAnimatingPageViewController animateLayer:copyLayer:animationDuration:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___78-[RobloxAnimatingPageViewController animateLayer:copyLayer:animationDuration:]_block_invoke")]
pub fn stub_5340c() {
    // IDA 0x5340c: the layer animation block (continuation of 0x530d0).
    // Animation glue; no explicit body.
}

// 0x535ac — ___copy_helper_block_84
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_84")]
pub fn stub_535ac() {
    // IDA 0x535ac: `__copy_helper_block_84` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x535d0 — ___destroy_helper_block_85
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_85")]
pub fn stub_535d0() {
    // IDA 0x535d0: `__destroy_helper_block_85` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x535ec — ___78-[RobloxAnimatingPageViewController animateLayer:copyLayer:animationDuration:]_block_invoke87
// type: _BYTE *__fastcall(_DWORD *, char)
#[doc(alias = "___78-[RobloxAnimatingPageViewController animateLayer:copyLayer:animationDuration:]_block_invoke87")]
pub fn stub_535ec() {
    // IDA 0x535ec: the layer completion block (continuation of
    // 0x530d0). Animation glue; no explicit body.
}

// 0x53634 — ___copy_helper_block_88
// type: void __fastcall(int, const void **)
#[doc(alias = "___copy_helper_block_88")]
pub fn stub_53634() {
    // IDA 0x53634: `__copy_helper_block_88` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x53664 — ___destroy_helper_block_89
// type: void __fastcall(const void **)
#[doc(alias = "___destroy_helper_block_89")]
pub fn stub_53664() {
    // IDA 0x53664: `__destroy_helper_block_89` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x53688 — -[RobloxAnimatingPageViewController startBackgroundPan]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL)
#[doc(alias = "-[RobloxAnimatingPageViewController startBackgroundPan]")]
pub fn stub_53688(no_warning: bool, looping: bool) {
    // IDA 0x53688: `startBackgroundPan` loops the foreground +
    // background animations when no warning fired and no loop runs
    // (0x536ae-0x536dc). The start records here.
    if no_warning && !looping {
        ANIMATION_LOOPING.store(true, std::sync::atomic::Ordering::SeqCst);
        ANIM_PANS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

// 0x536e0 — -[RobloxAnimatingPageViewController stopBackgroundPan]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL)
#[doc(alias = "-[RobloxAnimatingPageViewController stopBackgroundPan]")]
pub fn stub_536e0() {
    // IDA 0x536e0: `stopBackgroundPan` clears the loop. It records
    // here.
    ANIMATION_LOOPING.store(false, std::sync::atomic::Ordering::SeqCst);
}

// 0x53750 — ___54-[RobloxAnimatingPageViewController stopBackgroundPan]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___54-[RobloxAnimatingPageViewController stopBackgroundPan]_block_invoke")]
pub fn stub_53750() {
    // IDA 0x53750: the stop-pan block (continuation of 0x536e0).
    // Animation glue; no explicit body.
}

// 0x539f0 — ___copy_helper_block_97
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_97")]
pub fn stub_539f0() {
    // IDA 0x539f0: `__copy_helper_block_97` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x539fc — ___destroy_helper_block_98
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_98")]
pub fn stub_539fc() {
    // IDA 0x539fc: `__destroy_helper_block_98` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x53a04 — -[RobloxAnimatingPageViewController foregroundCopy]
// type: UIImageView *__cdecl(RobloxAnimatingPageViewController *self, SEL)
#[doc(alias = "-[RobloxAnimatingPageViewController foregroundCopy]")]
pub fn stub_53a04() -> bool {
    // IDA 0x53a04: `foregroundCopy` returns the ivar. Presence
    // reports here.
    FG_COPY.load(std::sync::atomic::Ordering::SeqCst)
}

// 0x53a14 — -[RobloxAnimatingPageViewController setForegroundCopy:]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL, id)
#[doc(alias = "-[RobloxAnimatingPageViewController setForegroundCopy:]")]
pub fn stub_53a14(present: bool) {
    // IDA 0x53a14: `setForegroundCopy:` stores the ivar. It records
    // here.
    FG_COPY.store(present, std::sync::atomic::Ordering::SeqCst);
}

// 0x53a38 — -[RobloxAnimatingPageViewController backgroundCopy]
// type: UIImageView *__cdecl(RobloxAnimatingPageViewController *self, SEL)
#[doc(alias = "-[RobloxAnimatingPageViewController backgroundCopy]")]
pub fn stub_53a38() -> bool {
    // IDA 0x53a38: `backgroundCopy` returns the ivar. Presence
    // reports here.
    BG_COPY.load(std::sync::atomic::Ordering::SeqCst)
}

// 0x53a48 — -[RobloxAnimatingPageViewController setBackgroundCopy:]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL, id)
#[doc(alias = "-[RobloxAnimatingPageViewController setBackgroundCopy:]")]
pub fn stub_53a48(present: bool) {
    // IDA 0x53a48: `setBackgroundCopy:` stores the ivar. It records
    // here.
    BG_COPY.store(present, std::sync::atomic::Ordering::SeqCst);
}

// 0x53a6c — -[RobloxAnimatingPageViewController foregroundImageInitialX]
// type: float __cdecl(RobloxAnimatingPageViewController *self, SEL)
#[doc(alias = "-[RobloxAnimatingPageViewController foregroundImageInitialX]")]
pub fn stub_53a6c() -> f32 {
    // IDA 0x53a6c: `foregroundImageInitialX` returns the ivar
    // (decompiled 0x53a6c).
    *FG_IMAGE_X.lock()
}

// 0x53a80 — -[RobloxAnimatingPageViewController setForegroundImageInitialX:]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL, float)
#[doc(alias = "-[RobloxAnimatingPageViewController setForegroundImageInitialX:]")]
pub fn stub_53a80(x: f32) {
    // IDA 0x53a80: `setForegroundImageInitialX:` stores the ivar.
    *FG_IMAGE_X.lock() = x;
}

// 0x53a98 — -[RobloxAnimatingPageViewController backgroundImageInitialX]
// type: float __cdecl(RobloxAnimatingPageViewController *self, SEL)
#[doc(alias = "-[RobloxAnimatingPageViewController backgroundImageInitialX]")]
pub fn stub_53a98() -> f32 {
    // IDA 0x53a98: `backgroundImageInitialX` returns the ivar (same
    // shape as 0x53a6c).
    *BG_IMAGE_X.lock()
}

// 0x53aac — -[RobloxAnimatingPageViewController setBackgroundImageInitialX:]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL, float)
#[doc(alias = "-[RobloxAnimatingPageViewController setBackgroundImageInitialX:]")]
pub fn stub_53aac(x: f32) {
    // IDA 0x53aac: `setBackgroundImageInitialX:` stores the ivar
    // (same shape as 0x53a80).
    *BG_IMAGE_X.lock() = x;
}

// 0x53ac4 — -[RobloxAnimatingPageViewController animationView]
// type: UIView *__cdecl(RobloxAnimatingPageViewController *self, SEL)
#[doc(alias = "-[RobloxAnimatingPageViewController animationView]")]
pub fn stub_53ac4() -> bool {
    // IDA 0x53ac4: `animationView` returns the ivar. Presence reports
    // here.
    ANIM_VIEW.load(std::sync::atomic::Ordering::SeqCst)
}

// 0x53ad4 — -[RobloxAnimatingPageViewController setAnimationView:]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL, id)
#[doc(alias = "-[RobloxAnimatingPageViewController setAnimationView:]")]
pub fn stub_53ad4(present: bool) {
    // IDA 0x53ad4: `setAnimationView:` stores the ivar. It records
    // here.
    ANIM_VIEW.store(present, std::sync::atomic::Ordering::SeqCst);
}

// 0x53af8 — -[RobloxAnimatingPageViewController imgBackground]
// type: UIImageView *__cdecl(RobloxAnimatingPageViewController *self, SEL)
#[doc(alias = "-[RobloxAnimatingPageViewController imgBackground]")]
pub fn stub_53af8() -> bool {
    // IDA 0x53af8: `imgBackground` returns the ivar. Presence reports
    // here.
    IMG_BG.load(std::sync::atomic::Ordering::SeqCst)
}

// 0x53b08 — -[RobloxAnimatingPageViewController setImgBackground:]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL, id)
#[doc(alias = "-[RobloxAnimatingPageViewController setImgBackground:]")]
pub fn stub_53b08(present: bool) {
    // IDA 0x53b08: `setImgBackground:` stores the ivar. It records
    // here.
    IMG_BG.store(present, std::sync::atomic::Ordering::SeqCst);
}

// 0x53b2c — -[RobloxAnimatingPageViewController imgForeground]
// type: UIImageView *__cdecl(RobloxAnimatingPageViewController *self, SEL)
#[doc(alias = "-[RobloxAnimatingPageViewController imgForeground]")]
pub fn stub_53b2c() -> bool {
    // IDA 0x53b2c: `imgForeground` returns the ivar. Presence reports
    // here.
    IMG_FG.load(std::sync::atomic::Ordering::SeqCst)
}

// 0x53b3c — -[RobloxAnimatingPageViewController setImgForeground:]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL, id)
#[doc(alias = "-[RobloxAnimatingPageViewController setImgForeground:]")]
pub fn stub_53b3c(present: bool) {
    // IDA 0x53b3c: `setImgForeground:` stores the ivar (same shape as
    // 0x53b08).
    IMG_FG.store(present, std::sync::atomic::Ordering::SeqCst);
}

// 0x53b60 — -[RobloxNavBarViewController initWithCoder:]
// type: RobloxNavBarViewController *__cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController initWithCoder:]")]
pub fn stub_53b60() {
    // IDA 0x53b60: `RobloxNavBarViewController::initWithCoder:`
    // supers. Super-init glue; no explicit body.
}

// 0x53cbc — -[RobloxNavBarViewController dealloc]
// type: void __cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController dealloc]")]
pub fn stub_53cbc() {
    // IDA 0x53cbc: `dealloc` drops the bar views. Release is drop
    // glue; the navbar state resets here.
    NAV_BACK_VISIBLE.store(false, std::sync::atomic::Ordering::SeqCst);
    *NAV_URL.lock() = String::new();
}

// 0x53e6c — -[RobloxNavBarViewController setUrl:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setUrl:]")]
pub fn stub_53e6c(url: &str) {
    // IDA 0x53e6c: `setUrl:` stores the URL. It records here.
    *NAV_URL.lock() = url.to_owned();
}

// 0x53e8c — -[RobloxNavBarViewController getUrl]
// type: id __cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController getUrl]")]
pub fn stub_53e8c() -> String {
    // IDA 0x53e8c: `getUrl` returns the URL.
    NAV_URL.lock().clone()
}

// 0x53e9c — -[RobloxNavBarViewController gotStartLeaveGameNotification:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController gotStartLeaveGameNotification:]")]
pub fn stub_53e9c() {
    // IDA 0x53e9c: `gotStartLeaveGameNotification:` handles the leave
    // start. The signal records here.
    NAV_LEAVE_SIGNALS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x53f38 — -[RobloxNavBarViewController gotDidLeaveGameNotification:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController gotDidLeaveGameNotification:]")]
pub fn stub_53f38() {
    // IDA 0x53f38: `gotDidLeaveGameNotification:` handles the leave
    // finish (same shape as 0x53e9c). The signal records here.
    NAV_LEAVE_SIGNALS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x53fac — -[RobloxNavBarViewController viewWillAppear:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, char)
#[doc(alias = "-[RobloxNavBarViewController viewWillAppear:]")]
pub fn stub_53fac() {
    // IDA 0x53fac: `viewWillAppear:` supers. Super glue; no explicit
    // body.
}

// 0x53ffc — -[RobloxNavBarViewController viewDidAppear:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, char)
#[doc(alias = "-[RobloxNavBarViewController viewDidAppear:]")]
pub fn stub_53ffc() {
    // IDA 0x53ffc: `viewDidAppear:` records the appearance. It
    // records here.
    NAV_APPEARS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x540c4 — ___44-[RobloxNavBarViewController viewDidAppear:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___44-[RobloxNavBarViewController viewDidAppear:]_block_invoke")]
pub fn stub_540c4() {
    // IDA 0x540c4: the appear block runs post-appear work on main
    // (continuation of 0x53ffc, counted there). View glue; no explicit
    // body.
}

// 0x540f0 — ___copy_helper_block__15
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__15")]
pub fn stub_540f0() {
    // IDA 0x540f0: `__copy_helper_block__15` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x540fc — ___destroy_helper_block__15
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__15")]
pub fn stub_540fc() {
    // IDA 0x540fc: `__destroy_helper_block__15` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x54104 — -[RobloxNavBarViewController viewDidLoad]
// type: void __cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController viewDidLoad]")]
pub fn stub_54104() {
    // IDA 0x54104: `viewDidLoad` supers. Super glue; no explicit body.
}

// 0x543dc — -[RobloxNavBarViewController hideBackButton]
// type: void __cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController hideBackButton]")]
pub fn stub_543dc() {
    // IDA 0x543dc: `hideBackButton` hides the button. It records
    // here.
    NAV_BACK_VISIBLE.store(false, std::sync::atomic::Ordering::SeqCst);
}

// 0x543fc — -[RobloxNavBarViewController showBackButton]
// type: void __cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController showBackButton]")]
pub fn stub_543fc() {
    // IDA 0x543fc: `showBackButton` shows the button (also the
    // no-launch branch at 0x54a90). It records here.
    NAV_BACK_VISIBLE.store(true, std::sync::atomic::Ordering::SeqCst);
}

// 0x5441c — -[RobloxNavBarViewController viewDidUnload]
// type: void __cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController viewDidUnload]")]
pub fn stub_5441c() {
    // IDA 0x5441c: `viewDidUnload` releases the bar views. Release is
    // drop glue; the navbar state resets here.
    NAV_BACK_VISIBLE.store(false, std::sync::atomic::Ordering::SeqCst);
    *NAV_URL.lock() = String::new();
}

// 0x5449c — -[RobloxNavBarViewController showFullscreenText:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController showFullscreenText:]")]
pub fn stub_5449c(text: &str) {
    // IDA 0x5449c: `showFullscreenText:` shows the spinner overlay
    // with the text (block at 0x54514). It sequences the block here.
    *FULLSCREEN_TEXT.lock() = text.to_owned();
    stub_54514();
}

// 0x54514 — ___49-[RobloxNavBarViewController showFullscreenText:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___49-[RobloxNavBarViewController showFullscreenText:]_block_invoke")]
pub fn stub_54514() {
    // IDA 0x54514: the show block starts the spinner and unhides the
    // overlay (0x54536-0x5457a). It records here.
    FULLSCREEN_VISIBLE.store(true, std::sync::atomic::Ordering::SeqCst);
    FULLSCREEN_SHOWS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x54594 — ___copy_helper_block_134
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_134")]
pub fn stub_54594() {
    // IDA 0x54594: `__copy_helper_block_134` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x545a0 — ___destroy_helper_block_135
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_135")]
pub fn stub_545a0() {
    // IDA 0x545a0: `__destroy_helper_block_135` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x545a8 — -[RobloxNavBarViewController hideFullscreenText]
// type: void __cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController hideFullscreenText]")]
pub fn stub_545a8() {
    // IDA 0x545a8: `hideFullscreenText` hides the overlay. It records
    // here.
    FULLSCREEN_VISIBLE.store(false, std::sync::atomic::Ordering::SeqCst);
}

// 0x545f8 — ___48-[RobloxNavBarViewController hideFullscreenText]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___48-[RobloxNavBarViewController hideFullscreenText]_block_invoke")]
pub fn stub_545f8() {
    // IDA 0x545f8: the hide block (continuation of 0x545a8). View
    // glue; no explicit body.
}

// 0x54648 — ___copy_helper_block_139
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_139")]
pub fn stub_54648() {
    // IDA 0x54648: `__copy_helper_block_139` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x54654 — ___destroy_helper_block_140
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_140")]
pub fn stub_54654() {
    // IDA 0x54654: `__destroy_helper_block_140` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x5465c — +[RobloxNavBarViewController checkForInAppPurchases:navigationType:]
// type: char __cdecl(id, SEL, id, int)
#[doc(alias = "+[RobloxNavBarViewController checkForInAppPurchases:navigationType:]")]
pub fn stub_5465c(is_store_url: bool) -> i32 {
    // IDA 0x5465c: `checkForInAppPurchases:navigationType:` reports
    // nonzero when the navigation is consumed in-app (same shape as
    // the platform `in_app_check_result`).
    i32::from(is_store_url)
}

// 0x5479c — -[RobloxNavBarViewController doPlaceLaunch:request:]
// type: char __cdecl(RobloxNavBarViewController *self, SEL, int, int)
#[doc(alias = "-[RobloxNavBarViewController doPlaceLaunch:request:]")]
pub fn stub_5479c(type_id: i32, _kind: i32) -> bool {
    // IDA 0x5479c: `doPlaceLaunch:request:` warns and reports 0 for a
    // non-positive id (0x5495e-0x549a4); else it marks tryGameJoin,
    // shows "LaunchGame", dispatches the join block, tracks the page
    // and flushes (0x54810-0x54952). The launch reports here.
    if type_id < 1 {
        return false;
    }
    stub_5449c("LaunchGame");
    PLACE_LAUNCHES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    stub_549e4();
    true
}

// 0x549e4 — ___52-[RobloxNavBarViewController doPlaceLaunch:request:]_block_invoke
// type: id __fastcall(_DWORD *)
#[doc(alias = "___52-[RobloxNavBarViewController doPlaceLaunch:request:]_block_invoke")]
pub fn stub_549e4() {
    // IDA 0x549e4: the join block runs the place join on main
    // (continuation of 0x5479c). It records here.
    PLACE_LAUNCH_RUNS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x54a28 — ___copy_helper_block_180
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_180")]
pub fn stub_54a28() {
    // IDA 0x54a28: `__copy_helper_block_180` retains the captures.
    // Retain is drop glue; no explicit body.
}

// 0x54a34 — ___destroy_helper_block_181
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_181")]
pub fn stub_54a34() {
    // IDA 0x54a34: `__destroy_helper_block_181` releases the captures.
    // Release is drop glue; no explicit body.
}

// 0x54a3c — -[RobloxNavBarViewController checkForGameLaunch:]
// type: char __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController checkForGameLaunch:]")]
pub fn stub_54a3c(url: Option<&str>, playing: bool) -> Option<(i32, i32)> {
    // IDA 0x54a3c: `checkForGameLaunch:` bails when already playing
    // (0x54a7e); else it shows the back button (0x54a90) and parses
    // "/games/start?" placeid/userid/appid into a launch id + kind
    // (0x54aa2-0x54c56). The parse reports here.
    if playing {
        return None;
    }
    stub_543fc();
    let u = url?;
    let after = u.split("/games/start?").nth(1)?;
    let parts: Vec<&str> = after.split(['=', '&', '?', '/']).collect();
    let mut it = parts.iter();
    while let Some(p) = it.next() {
        match p.to_lowercase().as_str() {
            "placeid" => return it.next().and_then(|v| v.parse().ok()).map(|id| (id, 0)),
            "userid" => return it.next().and_then(|v| v.parse().ok()).map(|id| (id, 1)),
            "appid" => return it.next().and_then(|v| v.parse().ok()).map(|id| (id, 2)),
            _ => {}
        }
    }
    None
}

// 0x54c64 — -[RobloxNavBarViewController webView:shouldStartLoadWithRequest:navigationType:]
// type: char __cdecl(RobloxNavBarViewController *self, SEL, id, id, int)
#[doc(alias = "-[RobloxNavBarViewController webView:shouldStartLoadWithRequest:navigationType:]")]
pub fn stub_54c64(in_app: i32, launched: Option<(i32, i32)>) -> bool {
    // IDA 0x54c64: the web view loads unless the in-app check consumes
    // the navigation (0x54cb6-0x54cd0) or a game launch handles it
    // (0x54cea-0x54d08); the home button re-enables on every exit.
    // The load reports here.
    if in_app != 0 {
        return false;
    }
    launched.is_none()
}

// 0x54d0c — -[RobloxNavBarViewController handleStartGameFailure]
// type: void __cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController handleStartGameFailure]")]
pub fn stub_54d0c() {
    // IDA 0x54d0c: `handleStartGameFailure` hides the fullscreen text
    // (0x54d18). It sequences the hide here.
    stub_545a8();
}

// 0x54d1c — -[RobloxNavBarViewController handleStartGameSuccess]
// type: void __cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController handleStartGameSuccess]")]
pub fn stub_54d1c() {
    // IDA 0x54d1c: `handleStartGameSuccess` hides the fullscreen text
    // (0x54d28, same shape as 0x54d0c). It sequences the hide here.
    stub_545a8();
}

// 0x54d2c — -[RobloxNavBarViewController webView:didFailLoadWithError:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id, id)
#[doc(alias = "-[RobloxNavBarViewController webView:didFailLoadWithError:]")]
pub fn stub_54d2c() {
    // IDA 0x54d2c: `webView:didFailLoadWithError:` records the
    // failure. It records here.
    WEB_LOAD_FAILS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x54d58 — -[RobloxNavBarViewController webViewDidStartLoad:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController webViewDidStartLoad:]")]
pub fn stub_54d58() {
    // IDA 0x54d58: `webViewDidStartLoad:` records the start. It
    // records here.
    WEB_LOAD_STARTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x54db4 — -[RobloxNavBarViewController webViewDidFinishLoad:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController webViewDidFinishLoad:]")]
pub fn stub_54db4() {
    // IDA 0x54db4: `webViewDidFinishLoad:` records the finish. It
    // records here.
    WEB_LOAD_FINISHES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

// 0x54e40 — -[RobloxNavBarViewController updateUserInfoDisplay:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, bool)
#[doc(alias = "-[RobloxNavBarViewController updateUserInfoDisplay:]")]
pub fn stub_54e40(update: bool, logged_in: bool) -> bool {
    // IDA 0x54e40: `updateUserInfoDisplay:` refreshes the player info
    // on demand (0x54e74-0x54e8c), then shows the balances when logged
    // in and hides them otherwise (0x54ec0-0x54fec). The display
    // reports here.
    if update {
        PLAYERINFO_REFRESHES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    logged_in
}

// 0x54ff0 — -[RobloxNavBarViewController MenuClick:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController MenuClick:]")]
pub fn stub_54ff0() -> ! {
    todo!("0x54ff0 -[RobloxNavBarViewController MenuClick:]")
}

// 0x55074 — ___40-[RobloxNavBarViewController MenuClick:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___40-[RobloxNavBarViewController MenuClick:]_block_invoke")]
pub fn stub_55074() -> ! {
    todo!("0x55074 ___40-[RobloxNavBarViewController MenuClick:]_block_invoke")
}

// 0x5508c — ___copy_helper_block_240
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_240")]
pub fn stub_5508c() -> ! {
    todo!("0x5508c ___copy_helper_block_240")
}

// 0x55098 — ___destroy_helper_block_241
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_241")]
pub fn stub_55098() -> ! {
    todo!("0x55098 ___destroy_helper_block_241")
}

// 0x550a0 — +[RobloxNavBarViewController mostRecentViewController]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxNavBarViewController mostRecentViewController]")]
pub fn stub_550a0() -> ! {
    todo!("0x550a0 +[RobloxNavBarViewController mostRecentViewController]")
}

// 0x550b0 — -[RobloxNavBarViewController setMainWebView:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setMainWebView:]")]
pub fn stub_550b0() -> ! {
    todo!("0x550b0 -[RobloxNavBarViewController setMainWebView:]")
}

// 0x551d8 — -[RobloxNavBarViewController backButtonClick:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController backButtonClick:]")]
pub fn stub_551d8() -> ! {
    todo!("0x551d8 -[RobloxNavBarViewController backButtonClick:]")
}

// 0x5523c — -[RobloxNavBarViewController setJumpToPlacePageAndLaunchGameWithID:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, int)
#[doc(alias = "-[RobloxNavBarViewController setJumpToPlacePageAndLaunchGameWithID:]")]
pub fn stub_5523c() -> ! {
    todo!("0x5523c -[RobloxNavBarViewController setJumpToPlacePageAndLaunchGameWithID:]")
}

// 0x5524c — -[RobloxNavBarViewController setJumpToPlaceIDGameInProgress:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, int)
#[doc(alias = "-[RobloxNavBarViewController setJumpToPlaceIDGameInProgress:]")]
pub fn stub_5524c() -> ! {
    todo!("0x5524c -[RobloxNavBarViewController setJumpToPlaceIDGameInProgress:]")
}

// 0x5525c — -[RobloxNavBarViewController activityIndicator]
// type: UIActivityIndicatorView *__cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController activityIndicator]")]
pub fn stub_5525c() -> ! {
    todo!("0x5525c -[RobloxNavBarViewController activityIndicator]")
}

// 0x5526c — -[RobloxNavBarViewController setActivityIndicator:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setActivityIndicator:]")]
pub fn stub_5526c() -> ! {
    todo!("0x5526c -[RobloxNavBarViewController setActivityIndicator:]")
}

// 0x55290 — -[RobloxNavBarViewController btnBack]
// type: UIBarButtonItem *__cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController btnBack]")]
pub fn stub_55290() -> ! {
    todo!("0x55290 -[RobloxNavBarViewController btnBack]")
}

// 0x552a0 — -[RobloxNavBarViewController setBtnBack:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setBtnBack:]")]
pub fn stub_552a0() -> ! {
    todo!("0x552a0 -[RobloxNavBarViewController setBtnBack:]")
}

// 0x552c4 — -[RobloxNavBarViewController barTopToolbar]
// type: UIToolbar *__cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController barTopToolbar]")]
pub fn stub_552c4() -> ! {
    todo!("0x552c4 -[RobloxNavBarViewController barTopToolbar]")
}

// 0x552d4 — -[RobloxNavBarViewController setBarTopToolbar:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setBarTopToolbar:]")]
pub fn stub_552d4() -> ! {
    todo!("0x552d4 -[RobloxNavBarViewController setBarTopToolbar:]")
}

// 0x552f8 — -[RobloxNavBarViewController lblRobux]
// type: UILabel *__cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController lblRobux]")]
pub fn stub_552f8() -> ! {
    todo!("0x552f8 -[RobloxNavBarViewController lblRobux]")
}

// 0x55308 — -[RobloxNavBarViewController setLblRobux:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setLblRobux:]")]
pub fn stub_55308() -> ! {
    todo!("0x55308 -[RobloxNavBarViewController setLblRobux:]")
}

// 0x5532c — -[RobloxNavBarViewController lblTix]
// type: UILabel *__cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController lblTix]")]
pub fn stub_5532c() -> ! {
    todo!("0x5532c -[RobloxNavBarViewController lblTix]")
}

// 0x5533c — -[RobloxNavBarViewController setLblTix:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setLblTix:]")]
pub fn stub_5533c() -> ! {
    todo!("0x5533c -[RobloxNavBarViewController setLblTix:]")
}

// 0x55360 — -[RobloxNavBarViewController toolbar]
// type: UIToolbar *__cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController toolbar]")]
pub fn stub_55360() -> ! {
    todo!("0x55360 -[RobloxNavBarViewController toolbar]")
}

// 0x55370 — -[RobloxNavBarViewController setToolbar:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setToolbar:]")]
pub fn stub_55370() -> ! {
    todo!("0x55370 -[RobloxNavBarViewController setToolbar:]")
}

// 0x55394 — -[RobloxNavBarViewController pageLoadActivityIndicator]
// type: UIActivityIndicatorView *__cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController pageLoadActivityIndicator]")]
pub fn stub_55394() -> ! {
    todo!("0x55394 -[RobloxNavBarViewController pageLoadActivityIndicator]")
}

// 0x553a4 — -[RobloxNavBarViewController setPageLoadActivityIndicator:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setPageLoadActivityIndicator:]")]
pub fn stub_553a4() -> ! {
    todo!("0x553a4 -[RobloxNavBarViewController setPageLoadActivityIndicator:]")
}

// 0x553c8 — -[RobloxNavBarViewController loadingOverlay]
// type: UIView *__cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController loadingOverlay]")]
pub fn stub_553c8() -> ! {
    todo!("0x553c8 -[RobloxNavBarViewController loadingOverlay]")
}

// 0x553d8 — -[RobloxNavBarViewController setLoadingOverlay:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setLoadingOverlay:]")]
pub fn stub_553d8() -> ! {
    todo!("0x553d8 -[RobloxNavBarViewController setLoadingOverlay:]")
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
#[doc(alias = "__GLOBAL__I_a_28")]
pub fn stub_554cc() -> ! {
    todo!("0x554cc global constructor keyed to_a_28")
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
