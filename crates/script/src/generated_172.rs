// Auto-generated skeletons for rbx-script — Lua|Script|Yield batch (filler)
// Filter: Lua|Script|Yield (4818 filtered, 0 remaining) -> global gap filler EA-sorted asc next 150 not yet in script crate
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +150 stubs | range 0x4e8b8..0x5465c EA-sorted asc next 150 global not yet in script crate (script 16762 -> 16912 distinct)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use std::sync::LazyLock;
use crate::generated_171::GameVC;

/// `__GLOBAL__I_a` one-shot latches (IDA 0x4ef74/0x4f7bc).
static GLOBAL_A22_INIT: LazyLock<u32> = LazyLock::new(|| 1);
static GLOBAL_A23_INIT: LazyLock<u32> = LazyLock::new(|| 1);

/// `LoginManager` login courts (IDA 0x4e9a0..0x4eac8): the attempt count,
/// last username, and successful logins. The credential send folds into
/// the host (the password is never stored).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LoginAttempt {
    pub attempts: u32,
    pub user: String,
    pub ok: u32,
}

/// `JumpButton` observable state (IDA 0x4f188..0x4f43c): the frame, the
/// component seating, and the jump latch. Images fold into the host.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct JumpBtn {
    pub frame: [f32; 4],
    pub seated: bool,
    pub jumping: bool,
}
/// `ThumbStickControl` observable state (IDA 0x4f9d0..0x4fe88): the frame,
/// style, claimed touch, and move vector. Stick visuals fold into the
/// host.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ThumbStick {
    pub frame: [f32; 4],
    pub style: u32,
    pub touch: Option<u32>,
    pub move_vec: [f32; 2],
}
/// `__GLOBAL__I_a_24` one-shot latch (IDA 0x50c98).
static GLOBAL_A24_INIT: LazyLock<u32> = LazyLock::new(|| 1);

/// `GameMenu` observable state (IDA 0x50eb0..0x515f0): visibility plus the
/// leave-game requests. Buttons/labels fold into the host.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GameMenu {
    pub shown: bool,
    pub leave_count: u32,
}

/// `__GLOBAL__I_a` one-shot latches (IDA 0x517f0/0x51bb0).
static GLOBAL_A25_INIT: LazyLock<u32> = LazyLock::new(|| 1);
static GLOBAL_A26_INIT: LazyLock<u32> = LazyLock::new(|| 1);

/// `MenuButton` observable state (IDA 0x51a04..0x51b44): the frame, the
/// owned menu visibility, and the enabled latch. Images fold into the
/// host.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MenuBtn {
    pub frame: [f32; 4],
    pub menu_open: bool,
    pub enabled: bool,
}

/// `MainViewController` observable state (IDA 0x51e68..0x51f90): the
/// current view, Ogre window/view handles, the Roblox view, the load
/// latch, and the subview count. UIKit peers fold into the host.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MainVC {
    pub view: Option<u32>,
    pub ogre_window: Option<u32>,
    pub ogre_view: Option<u32>,
    pub ogre_vc: Option<u32>,
    pub last_non_game: Option<u32>,
    pub rbx_view: Option<u32>,
    pub loaded: bool,
    pub subviews: u32,
}
/// `__GLOBAL__I_a_27` one-shot latch (IDA 0x51fe0).
static GLOBAL_A27_INIT: LazyLock<u32> = LazyLock::new(|| 1);

/// `RobloxAnimatingPageViewController` observable state (IDA
/// 0x52178..0x52aa0): load/appear latches, the memory-warning latch, the
/// pan latch, and the pan start count. Image views fold into the host.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AnimVC {
    pub loaded: bool,
    pub appeared: bool,
    pub mem_warning: bool,
    pub panning: bool,
    pub pan_count: u32,
    pub anims: u32,
}
// 0x4e8b8 — ___46-[GameViewController handlePromptSignupSignal]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___46-[GameViewController handlePromptSignupSignal]_block_invoke")]
pub fn stub_0x4e8b8(vc: &mut GameVC) {
    // IDA 0x4e8b8: the signup block instantiates the signup controller
    // from the main storyboard and presents it (twin of 0x4e780); the
    // storyboard glue folds into the host — see `stub_0x4e868`.
    vc.signup_shown = true;
}

// 0x4e98c — ___copy_helper_block_179
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_179")]
pub fn stub_0x4e98c() {
    // IDA 0x4e98c: `__copy_helper_block_179` retains captures; `Arc`
    // glue covers it — no-op.
}

// 0x4e998 — ___destroy_helper_block_180
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_180")]
pub fn stub_0x4e998() {
    // IDA 0x4e998: `__destroy_helper_block_180` releases captures (pair
    // of 0x4e98c); `Arc` glue covers it — no-op.
}

// 0x4e9a0 — -[GameViewController handleSignupNotification:]
// type: void __cdecl(GameViewController *self, SEL, id)
#[doc(alias = "-[GameViewController handleSignupNotification:]")]
pub fn stub_0x4e9a0(login: &mut LoginAttempt, user: &str) {
    // IDA 0x4e9a0: `handleSignupNotification:` reads the username and
    // password from the notification (0x4e9d8..0x4ea12) and logs in
    // through the shared manager (0x4e9c6..0x4ea2c); the manager send
    // folds into the host.
    login.attempts += 1;
    login.user = user.to_string();
}

// 0x4ea30 — -[GameViewController handleLoginNotification:]
// type: void __cdecl(GameViewController *self, SEL, id)
#[doc(alias = "-[GameViewController handleLoginNotification:]")]
pub fn stub_0x4ea30(login: &mut LoginAttempt, success: bool) {
    // IDA 0x4ea30: `handleLoginNotification:` reads the success flag
    // (0x4ea48..0x4eab6) and `dispatch_async`s the handler block
    // (0x4eaa6..0x4eabe); the queue hop folds into the caller — see
    // `stub_0x4eac8`.
    stub_0x4eac8(login, success);
}

// 0x4eac8 — ___46-[GameViewController handleLoginNotification:]_block_invoke
// type: void __fastcall(id *)
#[doc(alias = "___46-[GameViewController handleLoginNotification:]_block_invoke")]
pub fn stub_0x4eac8(login: &mut LoginAttempt, success: bool) {
    // IDA 0x4eac8: the login block applies the success/failure outcome;
    // the alert/dismiss glue folds into the host.
    if success {
        login.ok += 1;
    }
}

// 0x4edcc — ___copy_helper_block_203
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_203")]
pub fn stub_0x4edcc() {
    // IDA 0x4edcc: `__copy_helper_block_203` retains captures; `Arc`
    // glue covers it — no-op.
}

// 0x4edf0 — ___destroy_helper_block_204
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_204")]
pub fn stub_0x4edf0() {
    // IDA 0x4edf0: `__destroy_helper_block_204` releases captures (pair
    // of 0x4edcc); `Arc` glue covers it — no-op.
}

// 0x4ef74 — __GLOBAL__I_a_22
#[doc(alias = "global constructor keyed to_a_22")]
pub fn stub_0x4ef74() -> u32 {
    // IDA 0x4ef74: `__GLOBAL__I_a_22` — see `GLOBAL_A22_INIT`.
    *GLOBAL_A22_INIT
}

// 0x4f188 — -[JumpButton initWithFrame:]
// type: JumpButton *__cdecl(JumpButton *self, SEL, CGRect)
#[doc(alias = "-[JumpButton initWithFrame:]")]
pub fn stub_0x4f188(frame: [f32; 4]) -> JumpBtn {
    // IDA 0x4f188: `JumpButton initWithFrame:` chains to super
    // (0x4f1aa..0x4f1be), builds the control component (0x4f1dc..0x4f210),
    // and seats the button images (0x4f224..); the UIKit glue folds into
    // the host.
    JumpBtn { frame, seated: false, jumping: false }
}

// 0x4f2b0 — -[JumpButton dealloc]
// type: void __cdecl(JumpButton *self, SEL)
#[doc(alias = "-[JumpButton dealloc]")]
pub fn stub_0x4f2b0(btn: &mut JumpBtn) {
    // IDA 0x4f2b0: `dealloc` releases the control component (0x4f2d2)
    // and chains to super (0x4f2ea..); drop glue covers it and the
    // record resets.
    *btn = JumpBtn::default();
}

// 0x4f2fc — -[JumpButton setControlComponentSuperview:]
// type: void __cdecl(JumpButton *self, SEL, id)
#[doc(alias = "-[JumpButton setControlComponentSuperview:]")]
pub fn stub_0x4f2fc(btn: &mut JumpBtn) {
    // IDA 0x4f2fc: `setControlComponentSuperview:` seats the component
    // and wires the jump connections through the input service; the
    // service glue folds into the host.
    btn.seated = true;
}

// 0x4f404 — -[JumpButton jumpEnabledChanged:]
// type: void __cdecl(JumpButton *self, SEL, const PropertyDescriptor *)
#[doc(alias = "-[JumpButton jumpEnabledChanged:]")]
pub fn stub_0x4f404() {
    // IDA 0x4f404: `jumpEnabledChanged:` — empty body; no-op.
}

// 0x4f408 — -[JumpButton touchDown]
// type: void __cdecl(JumpButton *self, SEL)
#[doc(alias = "-[JumpButton touchDown]")]
pub fn stub_0x4f408(btn: &mut JumpBtn) {
    // IDA 0x4f408: `touchDown` jumps the local character (1) through the
    // input service (0x4f426..0x4f436); the service send folds into the
    // host.
    btn.jumping = true;
}

// 0x4f43c — -[JumpButton touchUp]
// type: void __cdecl(JumpButton *self, SEL)
#[doc(alias = "-[JumpButton touchUp]")]
pub fn stub_0x4f43c(btn: &mut JumpBtn) {
    // IDA 0x4f43c: `touchUp` releases the jump (0) through the input
    // service (0x4f45a..0x4f46a).
    btn.jumping = false;
}

// 0x4f7bc — __GLOBAL__I_a_23
#[doc(alias = "global constructor keyed to_a_23")]
pub fn stub_0x4f7bc() -> u32 {
    // IDA 0x4f7bc: `__GLOBAL__I_a_23` — see `GLOBAL_A23_INIT`.
    *GLOBAL_A23_INIT
}

// 0x4f9d0 — -[ThumbStickControl init:]
// type: id __cdecl(ThumbStickControl *self, SEL, CGRect)
#[doc(alias = "-[ThumbStickControl init:]")]
pub fn stub_0x4f9d0(frame: [f32; 4]) -> ThumbStick {
    // IDA 0x4f9d0: `ThumbStickControl init:` chains to super (0x4f9fc..),
    // seats the stick visuals, and queues the style block (see
    // `stub_0x4fcf4`); the UIKit glue folds into the host.
    ThumbStick { frame, style: 0, touch: None, move_vec: [0.0, 0.0] }
}

// 0x4fcf4 — ___26-[ThumbStickControl init:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___26-[ThumbStickControl init:]_block_invoke")]
pub fn stub_0x4fcf4(stick: &mut ThumbStick, setting: u32) {
    // IDA 0x4fcf4: the init block reads the thumbstick-style setting and
    // applies it (0x4fd14..0x4fd3c).
    stick.style = stub_0x4fdb8(setting);
}

// 0x4fd40 — ___copy_helper_block__11
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__11")]
pub fn stub_0x4fd40() {
    // IDA 0x4fd40: `__copy_helper_block__11` retains captures; `Arc`
    // glue covers it — no-op.
}

// 0x4fd4c — ___destroy_helper_block__11
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__11")]
pub fn stub_0x4fd4c() {
    // IDA 0x4fd4c: `__destroy_helper_block__11` releases captures (pair
    // of 0x4fd40); `Arc` glue covers it — no-op.
}

// 0x4fd54 — -[ThumbStickControl dealloc]
// type: void __cdecl(ThumbStickControl *self, SEL)
#[doc(alias = "-[ThumbStickControl dealloc]")]
pub fn stub_0x4fd54(stick: &mut ThumbStick) {
    // IDA 0x4fd54: `dealloc` releases the stick visuals (0x4fd78..0x4fd8c)
    // and chains to super (0x4fda4..); drop glue covers it and the
    // record resets.
    *stick = ThumbStick::default();
}

// 0x4fdb8 — -[ThumbStickControl intToThumbstickStyle:]
// type: int __cdecl(ThumbStickControl *self, SEL, int)
#[doc(alias = "-[ThumbStickControl intToThumbstickStyle:]")]
pub fn stub_0x4fdb8(setting: u32) -> u32 {
    // IDA 0x4fdb8: `intToThumbstickStyle:` clamps out-of-range settings
    // to 0 (0x4fdba..0x4fdc0).
    if setting >= 2 { 0 } else { setting }
}

// 0x4fdc4 — -[ThumbStickControl DistanceBetweenTwoPoints:withPoint2:]
// type: float __cdecl(ThumbStickControl *self, SEL, CGPoint, CGPoint)
#[doc(alias = "-[ThumbStickControl DistanceBetweenTwoPoints:withPoint2:]")]
pub fn stub_0x4fdc4(p1: [f32; 2], p2: [f32; 2]) -> f32 {
    // IDA 0x4fdc4: `DistanceBetweenTwoPoints` answers the Euclidean
    // distance (0x4fdd4..0x4fdf0).
    let dx = p2[0] - p1[0];
    let dy = p2[1] - p1[1];
    (dx * dx + dy * dy).sqrt()
}

// 0x4fdf4 — -[ThumbStickControl rotatePointAboutLocation:withPointToRotateAbout:withRadians:]
// type: CGPoint *__cdecl(CGPoint *__return_ptr __struct_ptr retstr, ThumbStickControl *self, SEL, CGPoint, CGPoint, float)
#[doc(alias = "-[ThumbStickControl rotatePointAboutLocation:withPointToRotateAbout:withRadians:]")]
pub fn stub_0x4fdf4(point: [f32; 2], center: [f32; 2], radians: f32) -> [f32; 2] {
    // IDA 0x4fdf4: `rotatePointAboutLocation` rotates by `radians` about
    // the center (0x4fe0c..0x4fe3e: delta, sin, cos).
    let dx = point[0] - center[0];
    let dy = point[1] - center[1];
    let (s, c) = radians.sin_cos();
    [center[0] + dx * c - dy * s, center[1] + dx * s + dy * c]
}

// 0x4fe88 — -[ThumbStickControl touchesBegan:withEvent:]
// type: void __cdecl(ThumbStickControl *self, SEL, id, id)
#[doc(alias = "-[ThumbStickControl touchesBegan:withEvent:]")]
pub fn stub_0x4fe88(stick: &mut ThumbStick, touch: u32, inside: bool) {
    // IDA 0x4fe88: `touchesBegan` claims the touch landing inside the
    // stick radius; the hit test folds into the host.
    if inside && stick.touch.is_none() {
        stick.touch = Some(touch);
    }
}

// 0x50108 — -[ThumbStickControl stationaryThumbstickTouchMove]
// type: void __cdecl(ThumbStickControl *self, SEL)
#[doc(alias = "-[ThumbStickControl stationaryThumbstickTouchMove]")]
pub fn stub_0x50108(stick: &mut ThumbStick, dx: f32, dy: f32) {
    // IDA 0x50108: `stationaryThumbstickTouchMove` re-seats the inner
    // stick around the anchor and drives the character; the layout math
    // folds into the host.
    stick.move_vec = [dx, dy];
}

// 0x50338 — -[ThumbStickControl followThumbstickTouchMove]
// type: void __cdecl(ThumbStickControl *self, SEL)
#[doc(alias = "-[ThumbStickControl followThumbstickTouchMove]")]
pub fn stub_0x50338(stick: &mut ThumbStick, dx: f32, dy: f32) {
    // IDA 0x50338: `followThumbstickTouchMove` re-seats the whole stick
    // under the drag and drives the character (follow-style twin of
    // 0x50108).
    stick.move_vec = [dx, dy];
}

// 0x506cc — -[ThumbStickControl touchesMoved:withEvent:]
// type: void __cdecl(ThumbStickControl *self, SEL, id, id)
#[doc(alias = "-[ThumbStickControl touchesMoved:withEvent:]")]
pub fn stub_0x506cc(stick: &mut ThumbStick, dx: f32, dy: f32) {
    // IDA 0x506cc: `touchesMoved` enumerates the touches and dispatches
    // to the stationary/follow mover by style; the enumeration folds
    // into the host.
    stick.move_vec = [dx, dy];
}

// 0x508b0 — -[ThumbStickControl cancelMovement]
// type: void __cdecl(ThumbStickControl *self, SEL)
#[doc(alias = "-[ThumbStickControl cancelMovement]")]
pub fn stub_0x508b0(stick: &mut ThumbStick) {
    // IDA 0x508b0: `cancelMovement` clears the thumbstick touch (0x508f0)
    // and fades the stick out via the animation blocks (0x50900..0x5094a,
    // see `stub_0x50960`/`stub_0x50c18`/`stub_0x50c80`).
    stick.touch = None;
    stick.move_vec = [0.0, 0.0];
}

// 0x50960 — ___35-[ThumbStickControl cancelMovement]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___35-[ThumbStickControl cancelMovement]_block_invoke")]
pub fn stub_0x50960() {
    // IDA 0x50960: the cancel animation block fades the stick visuals
    // out (0x50986); pure presentation folds into the host — no-op.
}

// 0x509a8 — ___copy_helper_block_77
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_77")]
pub fn stub_0x509a8() {
    // IDA 0x509a8: `__copy_helper_block_77` retains captures; `Arc` glue
    // covers it — no-op.
}

// 0x509b4 — ___destroy_helper_block_78
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_78")]
pub fn stub_0x509b4() {
    // IDA 0x509b4: `__destroy_helper_block_78` releases captures (pair
    // of 0x509a8); `Arc` glue covers it — no-op.
}

// 0x50c18 — ___35-[ThumbStickControl cancelMovement]_block_invoke_2
// type: id __fastcall(int)
#[doc(alias = "___35-[ThumbStickControl cancelMovement]_block_invoke_2")]
pub fn stub_0x50c18() {
    // IDA 0x50c18: the cancel completion block restores the stick alpha
    // (0x50c4a); pure presentation folds into the host — no-op.
}

// 0x50c6c — ___copy_helper_block_81
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_81")]
pub fn stub_0x50c6c() {
    // IDA 0x50c6c: `__copy_helper_block_81` retains captures; `Arc` glue
    // covers it — no-op.
}

// 0x50c78 — ___destroy_helper_block_82
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_82")]
pub fn stub_0x50c78() {
    // IDA 0x50c78: `__destroy_helper_block_82` releases captures (pair
    // of 0x50c6c); `Arc` glue covers it — no-op.
}

// 0x50c80 — ___35-[ThumbStickControl cancelMovement]_block_invoke84
// type: void __cdecl(id, char)
#[doc(alias = "___35-[ThumbStickControl cancelMovement]_block_invoke84")]
pub fn stub_0x50c80() {
    // IDA 0x50c80: the cancel completion sentinel (block 84) — empty
    // body; no-op.
}

// 0x50c84 — ___copy_helper_block_89
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_89")]
pub fn stub_0x50c84() {
    // IDA 0x50c84: `__copy_helper_block_89` retains captures; `Arc` glue
    // covers it — no-op.
}

// 0x50c90 — ___destroy_helper_block_90
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_90")]
pub fn stub_0x50c90() {
    // IDA 0x50c90: `__destroy_helper_block_90` releases captures (pair
    // of 0x50c84); `Arc` glue covers it — no-op.
}

// 0x50c98 — __GLOBAL__I_a_24
#[doc(alias = "global constructor keyed to_a_24")]
pub fn stub_0x50c98() -> u32 {
    // IDA 0x50c98: `__GLOBAL__I_a_24` — see `GLOBAL_A24_INIT`.
    *GLOBAL_A24_INIT
}

// 0x50eb0 — -[GameMenu init:]
// type: id __cdecl(GameMenu *self, SEL, id)
#[doc(alias = "-[GameMenu init:]")]
pub fn stub_0x50eb0() -> GameMenu {
    // IDA 0x50eb0: `GameMenu init:` chains to super and builds the
    // buttons/labels; the UIKit glue folds into the host.
    GameMenu::default()
}

// 0x512f8 — -[GameMenu dealloc]
// type: void __cdecl(GameMenu *self, SEL)
#[doc(alias = "-[GameMenu dealloc]")]
pub fn stub_0x512f8(menu: &mut GameMenu) {
    // IDA 0x512f8: `dealloc` releases the buttons/label
    // (0x5131c..0x51344) and chains to super (0x5135c..); drop glue
    // covers it and the record resets.
    *menu = GameMenu::default();
}

// 0x51370 — -[GameMenu isShown]
// type: char __cdecl(GameMenu *self, SEL)
#[doc(alias = "-[GameMenu isShown]")]
pub fn stub_0x51370(menu: &GameMenu) -> bool {
    // IDA 0x51370: `isShown` answers the shown latch (0x5137e).
    menu.shown
}

// 0x51380 — -[GameMenu acceptButtonPressed:]
// type: void __cdecl(GameMenu *self, SEL, id)
#[doc(alias = "-[GameMenu acceptButtonPressed:]")]
pub fn stub_0x51380(menu: &mut GameMenu) {
    // IDA 0x51380: `acceptButtonPressed:` leaves the game through the
    // shared place launcher (0x5139c..0x513b0); the launcher send folds
    // into the host.
    menu.leave_count += 1;
}

// 0x513b4 — -[GameMenu declineButtonPressed:]
// type: void __cdecl(GameMenu *self, SEL, id)
#[doc(alias = "-[GameMenu declineButtonPressed:]")]
pub fn stub_0x513b4(menu: &mut GameMenu) {
    // IDA 0x513b4: `declineButtonPressed:` hides via `hideMenu`
    // (0x513c0).
    stub_0x515f0(menu);
}

// 0x513c4 — -[GameMenu inverseMenuState:]
// type: void __cdecl(GameMenu *self, SEL, id)
#[doc(alias = "-[GameMenu inverseMenuState:]")]
pub fn stub_0x513c4(menu: &mut GameMenu) {
    // IDA 0x513c4: `inverseMenuState:` hides when shown (0x513d0..0x513f0)
    // and shows otherwise (0x513e0).
    menu.shown = !menu.shown;
}

// 0x513f8 — -[GameMenu showMenu:]
// type: void __cdecl(GameMenu *self, SEL, id)
#[doc(alias = "-[GameMenu showMenu:]")]
pub fn stub_0x513f8(menu: &mut GameMenu) {
    // IDA 0x513f8: `showMenu:` latches shown (0x51428) and animates the
    // menu in (see `stub_0x51570`); the animation folds into the host.
    menu.shown = true;
}

// 0x51570 — ___21-[GameMenu showMenu:]_block_invoke
// type: id __fastcall(_DWORD *)
#[doc(alias = "___21-[GameMenu showMenu:]_block_invoke")]
pub fn stub_0x51570() {
    // IDA 0x51570: the show animation block seats the menu frame; pure
    // presentation folds into the host — no-op.
}

// 0x515dc — ___copy_helper_block__12
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__12")]
pub fn stub_0x515dc() {
    // IDA 0x515dc: `__copy_helper_block__12` retains captures; `Arc`
    // glue covers it — no-op.
}

// 0x515e8 — ___destroy_helper_block__12
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__12")]
pub fn stub_0x515e8() {
    // IDA 0x515e8: `__destroy_helper_block__12` releases captures (pair
    // of 0x515dc); `Arc` glue covers it — no-op.
}

// 0x515f0 — -[GameMenu hideMenu]
// type: void __cdecl(GameMenu *self, SEL)
#[doc(alias = "-[GameMenu hideMenu]")]
pub fn stub_0x515f0(menu: &mut GameMenu) {
    // IDA 0x515f0: `hideMenu` unlatches shown (twin of 0x513f8, cf.
    // 0x513b4/0x513c4 call sites).
    menu.shown = false;
}

// 0x51738 — ___20-[GameMenu hideMenu]_block_invoke
// type: id __fastcall(_DWORD *)
#[doc(alias = "___20-[GameMenu hideMenu]_block_invoke")]
pub fn stub_0x51738() {
    // IDA 0x51738: the hide animation block re-seats the menu frame
    // (0x51748..0x51792); pure presentation folds into the host — no-op.
}

// 0x51794 — ___copy_helper_block_96
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_96")]
pub fn stub_0x51794() {
    // IDA 0x51794: `__copy_helper_block_96` retains captures; `Arc` glue
    // covers it — no-op.
}

// 0x517a0 — ___destroy_helper_block_97
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_97")]
pub fn stub_0x517a0() {
    // IDA 0x517a0: `__destroy_helper_block_97` releases captures (pair
    // of 0x51794); `Arc` glue covers it — no-op.
}

// 0x517a8 — ___20-[GameMenu hideMenu]_block_invoke99
// type: id __fastcall(int)
#[doc(alias = "___20-[GameMenu hideMenu]_block_invoke99")]
pub fn stub_0x517a8() {
    // IDA 0x517a8: the hide completion block hides the view and removes
    // it from its superview (0x517be); the shown latch in `stub_0x515f0`
    // already records the hide — no-op.
}

// 0x517d8 — ___copy_helper_block_102
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_102")]
pub fn stub_0x517d8() {
    // IDA 0x517d8: `__copy_helper_block_102` retains captures; `Arc`
    // glue covers it — no-op.
}

// 0x517e4 — ___destroy_helper_block_103
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_103")]
pub fn stub_0x517e4() {
    // IDA 0x517e4: `__destroy_helper_block_103` releases captures (pair
    // of 0x517d8); `Arc` glue covers it — no-op.
}

// 0x517ec — -[GameMenu .cxx_construct]
// type: id __cdecl(GameMenu *self, SEL)
#[doc(alias = "-[GameMenu .cxx_construct]")]
pub fn stub_0x517ec() {
    // IDA 0x517ec: `GameMenu .cxx_construct` runs no ivar inits and
    // answers self (0x517ec); folds into `Default` — no-op.
}

// 0x517f0 — __GLOBAL__I_a_25
#[doc(alias = "global constructor keyed to_a_25")]
pub fn stub_0x517f0() -> u32 {
    // IDA 0x517f0: `__GLOBAL__I_a_25` — see `GLOBAL_A25_INIT`.
    *GLOBAL_A25_INIT
}

// 0x51a04 — -[MenuButton init:]
// type: id __cdecl(MenuButton *self, SEL, CGRect)
#[doc(alias = "-[MenuButton init:]")]
pub fn stub_0x51a04(frame: [f32; 4]) -> MenuBtn {
    // IDA 0x51a04: `MenuButton init:` chains to super (0x51a20..0x51a2c),
    // seats the frame (0x51a50), installs the images (0x51a76..0x51a8c),
    // and builds the owned menu; the UIKit glue folds into the host.
    MenuBtn { frame, menu_open: false, enabled: true }
}

// 0x51af8 — -[MenuButton dealloc]
// type: void __cdecl(MenuButton *self, SEL)
#[doc(alias = "-[MenuButton dealloc]")]
pub fn stub_0x51af8(btn: &mut MenuBtn) {
    // IDA 0x51af8: `dealloc` releases the owned menu (0x51b1a) and chains
    // to super (0x51b32..); drop glue covers it and the record resets.
    *btn = MenuBtn::default();
}

// 0x51b44 — -[MenuButton doMenuSwitch:]
// type: void __cdecl(MenuButton *self, SEL, id)
#[doc(alias = "-[MenuButton doMenuSwitch:]")]
pub fn stub_0x51b44(btn: &mut MenuBtn, menu: &mut GameMenu) {
    // IDA 0x51b44: `doMenuSwitch:` inverts the menu (0x51b64..0x51b7a)
    // and enables the button exactly when the menu ends hidden
    // (0x51ba2..0x51bac).
    stub_0x513c4(menu);
    btn.menu_open = menu.shown;
    btn.enabled = !menu.shown;
}

// 0x51bb0 — __GLOBAL__I_a_26
#[doc(alias = "global constructor keyed to_a_26")]
pub fn stub_0x51bb0() -> u32 {
    // IDA 0x51bb0: `__GLOBAL__I_a_26` — see `GLOBAL_A26_INIT`.
    *GLOBAL_A26_INIT
}

// 0x51e54 — ___copy_helper_block__13
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__13")]
pub fn stub_0x51e54() {
    // IDA 0x51e54: `__copy_helper_block__13` retains captures; `Arc`
    // glue covers it — no-op.
}

// 0x51e60 — ___destroy_helper_block__13
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__13")]
pub fn stub_0x51e60() {
    // IDA 0x51e60: `__destroy_helper_block__13` releases captures (pair
    // of 0x51e54); `Arc` glue covers it — no-op.
}

// 0x51e68 — -[MainViewController switchView:]
// type: void __cdecl(MainViewController *self, SEL, id)
#[doc(alias = "-[MainViewController switchView:]")]
pub fn stub_0x51e68(vc: &mut MainVC, view: u32) {
    // IDA 0x51e68: `switchView:` seats the new view (0x51e74).
    vc.view = Some(view);
}

// 0x51e78 — -[MainViewController addSubview:]
// type: void __cdecl(MainViewController *self, SEL, id)
#[doc(alias = "-[MainViewController addSubview:]")]
pub fn stub_0x51e78(vc: &mut MainVC) {
    // IDA 0x51e78: `addSubview:` appends to the current view when one is
    // seated (0x51e90..0x51eb4); the hierarchy folds into the host.
    if vc.view.is_some() {
        vc.subviews += 1;
    }
}

// 0x51eb8 — -[MainViewController initWithNibName:bundle:]
// type: MainViewController *__cdecl(MainViewController *self, SEL, id, id)
#[doc(alias = "-[MainViewController initWithNibName:bundle:]")]
pub fn stub_0x51eb8() -> MainVC {
    // IDA 0x51eb8: `initWithNibName:bundle:` chains to super
    // (0x51ed2..0x51ee4); the nib glue folds into the host.
    MainVC::default()
}

// 0x51ee8 — -[MainViewController viewDidLoad]
// type: void __cdecl(MainViewController *self, SEL)
#[doc(alias = "-[MainViewController viewDidLoad]")]
pub fn stub_0x51ee8(vc: &mut MainVC) {
    // IDA 0x51ee8: `viewDidLoad` chains to super (0x51f02..0x51f0c).
    vc.loaded = true;
}

// 0x51f14 — -[MainViewController viewDidUnload]
// type: void __cdecl(MainViewController *self, SEL)
#[doc(alias = "-[MainViewController viewDidUnload]")]
pub fn stub_0x51f14(vc: &mut MainVC) {
    // IDA 0x51f14: `viewDidUnload` chains to super (0x51f2e..0x51f38)
    // and drops the view; the hierarchy glue folds into the host.
    vc.loaded = false;
}

// 0x51f40 — -[MainViewController getOgreWindow]
// type: id __cdecl(MainViewController *self, SEL)
#[doc(alias = "-[MainViewController getOgreWindow]")]
pub fn stub_0x51f40(vc: &MainVC) -> Option<u32> {
    // IDA 0x51f40: `getOgreWindow` answers the window (0x51f4e).
    vc.ogre_window
}

// 0x51f50 — -[MainViewController setOgreWindow:]
// type: void __cdecl(MainViewController *self, SEL, id)
#[doc(alias = "-[MainViewController setOgreWindow:]")]
pub fn stub_0x51f50(vc: &mut MainVC, window: u32) {
    // IDA 0x51f50: `setOgreWindow:` stores the window (0x51f5c).
    vc.ogre_window = Some(window);
}

// 0x51f60 — -[MainViewController getOgreView]
// type: id __cdecl(MainViewController *self, SEL)
#[doc(alias = "-[MainViewController getOgreView]")]
pub fn stub_0x51f60(vc: &MainVC) -> Option<u32> {
    // IDA 0x51f60: `getOgreView` answers the view (0x51f6e).
    vc.ogre_view
}

// 0x51f70 — -[MainViewController setOgreView:]
// type: void __cdecl(MainViewController *self, SEL, id)
#[doc(alias = "-[MainViewController setOgreView:]")]
pub fn stub_0x51f70(vc: &mut MainVC, view: u32) {
    // IDA 0x51f70: `setOgreView:` stores the view (0x51f7c).
    vc.ogre_view = Some(view);
}

// 0x51f80 — -[MainViewController setRobloxView:]
// type: void __cdecl(MainViewController *self, SEL, RobloxView *)
#[doc(alias = "-[MainViewController setRobloxView:]")]
pub fn stub_0x51f80(vc: &mut MainVC, view: u32) {
    // IDA 0x51f80: `setRobloxView:` stores the Roblox view (0x51f8c).
    vc.rbx_view = Some(view);
}

// 0x51f90 — -[MainViewController getRobloxView]
// type: RobloxView *__cdecl(MainViewController *self, SEL)
#[doc(alias = "-[MainViewController getRobloxView]")]
pub fn stub_0x51f90(vc: &MainVC) -> Option<u32> {
    // IDA 0x51f90: `getRobloxView` answers the Roblox view (0x51f9e).
    vc.rbx_view
}

// 0x51fa0 — -[MainViewController getOgreViewController]
// type: id __cdecl(MainViewController *self, SEL)
pub fn stub_0x51fa0(vc: &MainVC) -> Option<u32> {
    // IDA 0x51fa0: `getOgreViewController` answers the controller
    // (0x51fae).
    vc.ogre_vc
}

// 0x51fb0 — -[MainViewController setOgreViewController:]
// type: void __cdecl(MainViewController *self, SEL, id)
pub fn stub_0x51fb0(vc: &mut MainVC, controller: u32) {
    // IDA 0x51fb0: `setOgreViewController:` stores the controller
    // (0x51fbc).
    vc.ogre_vc = Some(controller);
}

// 0x51fc0 — -[MainViewController setLastNonGameController:]
// type: void __cdecl(MainViewController *self, SEL, id)
pub fn stub_0x51fc0(vc: &mut MainVC, controller: u32) {
    // IDA 0x51fc0: `setLastNonGameController:` stores the controller
    // (0x51fcc).
    vc.last_non_game = Some(controller);
}

// 0x51fd0 — -[MainViewController getLastNonGameController]
// type: id __cdecl(MainViewController *self, SEL)
pub fn stub_0x51fd0(vc: &MainVC) -> Option<u32> {
    // IDA 0x51fd0: `getLastNonGameController` answers the controller
    // (0x51fde).
    vc.last_non_game
}

// 0x51fe0 — __GLOBAL__I_a_27
pub fn stub_0x51fe0() -> u32 {
    // IDA 0x51fe0: `__GLOBAL__I_a_27` — see `GLOBAL_A27_INIT`.
    *GLOBAL_A27_INIT
}

// 0x52178 — -[RobloxAnimatingPageViewController initWithCoder:]
// type: RobloxAnimatingPageViewController *__cdecl(RobloxAnimatingPageViewController *self, SEL, id)
pub fn stub_0x52178() -> AnimVC {
    // IDA 0x52178: `initWithCoder:` chains to super (0x52196..0x521a0)
    // and clears the warning/loop latches (0x521c4..0x521d4); the nib
    // glue folds into the host.
    AnimVC::default()
}

// 0x52280 — -[RobloxAnimatingPageViewController dealloc]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL)
pub fn stub_0x52280(vc: &mut AnimVC) {
    // IDA 0x52280: `dealloc` releases the animation views unless a
    // memory warning fired (0x52294..0x522e8) and chains to super; drop
    // glue covers it and the record resets.
    *vc = AnimVC::default();
}

// 0x5233c — -[RobloxAnimatingPageViewController appInBackground:]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL, id)
pub fn stub_0x5233c(vc: &mut AnimVC) {
    // IDA 0x5233c: `appInBackground:` stops the background pan (0x52348).
    vc.panning = false;
}

// 0x5234c — -[RobloxAnimatingPageViewController appInForeground:]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL, id)
pub fn stub_0x5234c(vc: &mut AnimVC, loaded: bool) {
    // IDA 0x5234c: `appInForeground:` restarts the pan for a loaded view
    // (0x52364..0x5237e); the loaded check folds into the host.
    if loaded {
        vc.panning = true;
        vc.pan_count += 1;
    }
}

// 0x52384 — -[RobloxAnimatingPageViewController removeViewAndAnimation:]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL, id)
pub fn stub_0x52384() {
    // IDA 0x52384: `removeViewAndAnimation:` strips the layer animations
    // (0x523a8), drops the view (0x523ba), and releases it (0x523d0);
    // the layer/view glue folds into the host — no-op.
}

// 0x523d4 — -[RobloxAnimatingPageViewController didReceiveMemoryWarning]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL)
pub fn stub_0x523d4(vc: &mut AnimVC) {
    // IDA 0x523d4: `didReceiveMemoryWarning` chains to super
    // (0x523ee..0x523f8); the warning is observed.
    vc.mem_warning = true;
}

// 0x52400 — -[RobloxAnimatingPageViewController viewDidLoad]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL)
pub fn stub_0x52400(vc: &mut AnimVC) {
    // IDA 0x52400: `viewDidLoad` chains to super (0x52420..0x5242a) and
    // builds the animation views (0x5243a..); the UIKit glue folds into
    // the host.
    vc.loaded = true;
}

// 0x52580 — -[RobloxAnimatingPageViewController getInitialXPosition:]
// type: float __cdecl(RobloxAnimatingPageViewController *self, SEL, id)
pub fn stub_0x52580(x: Option<f32>) -> f32 {
    // IDA 0x52580: `getInitialXPosition:` answers 0 for a null view
    // (0x5258c..0x52602) and derives the position from the view bounds
    // otherwise; the geometry folds into the host.
    x.unwrap_or(0.0)
}

// 0x52614 — -[RobloxAnimatingPageViewController viewDidAppear:]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL, char)
pub fn stub_0x52614(vc: &mut AnimVC) {
    // IDA 0x52614: `viewDidAppear:` starts the foreground/background pan
    // animations; the animation glue folds into the host.
    vc.appeared = true;
    vc.panning = true;
    vc.pan_count += 1;
}

// 0x52a50 — -[RobloxAnimatingPageViewController viewDidDisappear:]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL, char)
pub fn stub_0x52a50(vc: &mut AnimVC) {
    // IDA 0x52a50: `viewDidDisappear:` chains to super (0x52a6c..) and
    // stops the pan short of a memory warning (0x52a86..0x52a98).
    vc.appeared = false;
    vc.panning = false;
}

// 0x52aa0 — -[RobloxAnimatingPageViewController hasNaNValue:]
// type: char __cdecl(RobloxAnimatingPageViewController *self, SEL, CGRect)
pub fn stub_0x52aa0() -> bool {
    // IDA 0x52aa0: `hasNaNValue:` answers 0 unconditionally (0x52ad2).
    false
}

// 0x52aec — -[RobloxAnimatingPageViewController animateToZeroPosition:copyLayer:defaultTweenTime:]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL, id, id, float)
pub fn stub_0x52aec(vc: &mut AnimVC) {
    // IDA 0x52aec: `animateToZeroPosition` builds the layer animation
    // blocks (see `stub_0x52dac`/`stub_0x52f14`) and runs them; the
    // CoreAnimation glue folds into the host.
    vc.anims += 1;
}

// 0x52dac — ___86-[RobloxAnimatingPageViewController animateToZeroPosition:copyLayer:defaultTweenTime:]_block_invoke
// type: id __fastcall(int)
pub fn stub_0x52dac() {
    // IDA 0x52dac: the zero-position block composes the layer animation;
    // folds into `stub_0x52aec` — no-op.
}

// 0x52ed4 — ___copy_helper_block__14
// type: void __fastcall(int, int)
pub fn stub_0x52ed4() {
    // IDA 0x52ed4: `__copy_helper_block__14` retains captures; `Arc`
    // glue covers it — no-op.
}

// 0x52ef8 — ___destroy_helper_block__14
// type: void __fastcall(int)
pub fn stub_0x52ef8() {
    // IDA 0x52ef8: `__destroy_helper_block__14` releases captures (pair
    // of 0x52ed4); `Arc` glue covers it — no-op.
}

// 0x52f14 — ___86-[RobloxAnimatingPageViewController animateToZeroPosition:copyLayer:defaultTweenTime:]_block_invoke73
// type: id __fastcall(int)
pub fn stub_0x52f14() {
    // IDA 0x52f14: the animation block forwards to `animateLayer:...`
    // (0x52f42); folds into `stub_0x52aec` — no-op.
}

// 0x52f44 — ___copy_helper_block_76
// type: void __fastcall(int, const void **)
pub fn stub_0x52f44() {
    // IDA 0x52f44: `__copy_helper_block_76` retains captures; `Arc` glue
    // covers it — no-op.
}

// 0x52f74 — ___destroy_helper_block_77
// type: void __fastcall(const void **)
pub fn stub_0x52f74() {
    // IDA 0x52f74: `__destroy_helper_block_77` releases captures (pair
    // of 0x52f44); `Arc` glue covers it — no-op.
}

// 0x52f98 — -[RobloxAnimatingPageViewController animateBackground]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL)
pub fn stub_0x52f98(vc: &mut AnimVC) {
    // IDA 0x52f98: `animateBackground` animates to zero when the frame
    // is seated (0x52fde..0x53012), else runs the plain layer animation
    // (0x5301e..); either way one animation starts.
    vc.anims += 1;
}

// 0x53034 — -[RobloxAnimatingPageViewController animateForeground]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL)
pub fn stub_0x53034(vc: &mut AnimVC) {
    // IDA 0x53034: `animateForeground` — same seated-or-plain branch as
    // 0x52f98 (0x5307a..0x530ba); one animation starts.
    vc.anims += 1;
}

// 0x530d0 — -[RobloxAnimatingPageViewController animateLayer:copyLayer:animationDuration:]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL, id, id, float)
#[doc(alias = "-[RobloxAnimatingPageViewController animateLayer:copyLayer:animationDuration:]")]
pub fn stub_0x530d0() -> ! {
    todo!("0x530d0 -[RobloxAnimatingPageViewController animateLayer:copyLayer:animationDuration:]")
}

// 0x5340c — ___78-[RobloxAnimatingPageViewController animateLayer:copyLayer:animationDuration:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___78-[RobloxAnimatingPageViewController animateLayer:copyLayer:animationDuration:]_block_invoke")]
pub fn stub_0x5340c() -> ! {
    todo!("0x5340c ___78-[RobloxAnimatingPageViewController animateLayer:copyLayer:animationDuration:]_block_invoke")
}

// 0x535ac — ___copy_helper_block_84
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_84")]
pub fn stub_0x535ac() -> ! {
    todo!("0x535ac ___copy_helper_block_84")
}

// 0x535d0 — ___destroy_helper_block_85
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_85")]
pub fn stub_0x535d0() -> ! {
    todo!("0x535d0 ___destroy_helper_block_85")
}

// 0x535ec — ___78-[RobloxAnimatingPageViewController animateLayer:copyLayer:animationDuration:]_block_invoke87
// type: _BYTE *__fastcall(_DWORD *, char)
#[doc(alias = "___78-[RobloxAnimatingPageViewController animateLayer:copyLayer:animationDuration:]_block_invoke87")]
pub fn stub_0x535ec() -> ! {
    todo!("0x535ec ___78-[RobloxAnimatingPageViewController animateLayer:copyLayer:animationDuration:]_block_invoke87")
}

// 0x53634 — ___copy_helper_block_88
// type: void __fastcall(int, const void **)
#[doc(alias = "___copy_helper_block_88")]
pub fn stub_0x53634() -> ! {
    todo!("0x53634 ___copy_helper_block_88")
}

// 0x53664 — ___destroy_helper_block_89
// type: void __fastcall(const void **)
#[doc(alias = "___destroy_helper_block_89")]
pub fn stub_0x53664() -> ! {
    todo!("0x53664 ___destroy_helper_block_89")
}

// 0x53688 — -[RobloxAnimatingPageViewController startBackgroundPan]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL)
#[doc(alias = "-[RobloxAnimatingPageViewController startBackgroundPan]")]
pub fn stub_0x53688() -> ! {
    todo!("0x53688 -[RobloxAnimatingPageViewController startBackgroundPan]")
}

// 0x536e0 — -[RobloxAnimatingPageViewController stopBackgroundPan]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL)
#[doc(alias = "-[RobloxAnimatingPageViewController stopBackgroundPan]")]
pub fn stub_0x536e0() -> ! {
    todo!("0x536e0 -[RobloxAnimatingPageViewController stopBackgroundPan]")
}

// 0x53750 — ___54-[RobloxAnimatingPageViewController stopBackgroundPan]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___54-[RobloxAnimatingPageViewController stopBackgroundPan]_block_invoke")]
pub fn stub_0x53750() -> ! {
    todo!("0x53750 ___54-[RobloxAnimatingPageViewController stopBackgroundPan]_block_invoke")
}

// 0x539f0 — ___copy_helper_block_97
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_97")]
pub fn stub_0x539f0() -> ! {
    todo!("0x539f0 ___copy_helper_block_97")
}

// 0x539fc — ___destroy_helper_block_98
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_98")]
pub fn stub_0x539fc() -> ! {
    todo!("0x539fc ___destroy_helper_block_98")
}

// 0x53a04 — -[RobloxAnimatingPageViewController foregroundCopy]
// type: UIImageView *__cdecl(RobloxAnimatingPageViewController *self, SEL)
#[doc(alias = "-[RobloxAnimatingPageViewController foregroundCopy]")]
pub fn stub_0x53a04() -> ! {
    todo!("0x53a04 -[RobloxAnimatingPageViewController foregroundCopy]")
}

// 0x53a14 — -[RobloxAnimatingPageViewController setForegroundCopy:]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL, id)
#[doc(alias = "-[RobloxAnimatingPageViewController setForegroundCopy:]")]
pub fn stub_0x53a14() -> ! {
    todo!("0x53a14 -[RobloxAnimatingPageViewController setForegroundCopy:]")
}

// 0x53a38 — -[RobloxAnimatingPageViewController backgroundCopy]
// type: UIImageView *__cdecl(RobloxAnimatingPageViewController *self, SEL)
#[doc(alias = "-[RobloxAnimatingPageViewController backgroundCopy]")]
pub fn stub_0x53a38() -> ! {
    todo!("0x53a38 -[RobloxAnimatingPageViewController backgroundCopy]")
}

// 0x53a48 — -[RobloxAnimatingPageViewController setBackgroundCopy:]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL, id)
#[doc(alias = "-[RobloxAnimatingPageViewController setBackgroundCopy:]")]
pub fn stub_0x53a48() -> ! {
    todo!("0x53a48 -[RobloxAnimatingPageViewController setBackgroundCopy:]")
}

// 0x53a6c — -[RobloxAnimatingPageViewController foregroundImageInitialX]
// type: float __cdecl(RobloxAnimatingPageViewController *self, SEL)
#[doc(alias = "-[RobloxAnimatingPageViewController foregroundImageInitialX]")]
pub fn stub_0x53a6c() -> ! {
    todo!("0x53a6c -[RobloxAnimatingPageViewController foregroundImageInitialX]")
}

// 0x53a80 — -[RobloxAnimatingPageViewController setForegroundImageInitialX:]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL, float)
#[doc(alias = "-[RobloxAnimatingPageViewController setForegroundImageInitialX:]")]
pub fn stub_0x53a80() -> ! {
    todo!("0x53a80 -[RobloxAnimatingPageViewController setForegroundImageInitialX:]")
}

// 0x53a98 — -[RobloxAnimatingPageViewController backgroundImageInitialX]
// type: float __cdecl(RobloxAnimatingPageViewController *self, SEL)
#[doc(alias = "-[RobloxAnimatingPageViewController backgroundImageInitialX]")]
pub fn stub_0x53a98() -> ! {
    todo!("0x53a98 -[RobloxAnimatingPageViewController backgroundImageInitialX]")
}

// 0x53aac — -[RobloxAnimatingPageViewController setBackgroundImageInitialX:]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL, float)
#[doc(alias = "-[RobloxAnimatingPageViewController setBackgroundImageInitialX:]")]
pub fn stub_0x53aac() -> ! {
    todo!("0x53aac -[RobloxAnimatingPageViewController setBackgroundImageInitialX:]")
}

// 0x53ac4 — -[RobloxAnimatingPageViewController animationView]
// type: UIView *__cdecl(RobloxAnimatingPageViewController *self, SEL)
#[doc(alias = "-[RobloxAnimatingPageViewController animationView]")]
pub fn stub_0x53ac4() -> ! {
    todo!("0x53ac4 -[RobloxAnimatingPageViewController animationView]")
}

// 0x53ad4 — -[RobloxAnimatingPageViewController setAnimationView:]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL, id)
#[doc(alias = "-[RobloxAnimatingPageViewController setAnimationView:]")]
pub fn stub_0x53ad4() -> ! {
    todo!("0x53ad4 -[RobloxAnimatingPageViewController setAnimationView:]")
}

// 0x53af8 — -[RobloxAnimatingPageViewController imgBackground]
// type: UIImageView *__cdecl(RobloxAnimatingPageViewController *self, SEL)
#[doc(alias = "-[RobloxAnimatingPageViewController imgBackground]")]
pub fn stub_0x53af8() -> ! {
    todo!("0x53af8 -[RobloxAnimatingPageViewController imgBackground]")
}

// 0x53b08 — -[RobloxAnimatingPageViewController setImgBackground:]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL, id)
#[doc(alias = "-[RobloxAnimatingPageViewController setImgBackground:]")]
pub fn stub_0x53b08() -> ! {
    todo!("0x53b08 -[RobloxAnimatingPageViewController setImgBackground:]")
}

// 0x53b2c — -[RobloxAnimatingPageViewController imgForeground]
// type: UIImageView *__cdecl(RobloxAnimatingPageViewController *self, SEL)
#[doc(alias = "-[RobloxAnimatingPageViewController imgForeground]")]
pub fn stub_0x53b2c() -> ! {
    todo!("0x53b2c -[RobloxAnimatingPageViewController imgForeground]")
}

// 0x53b3c — -[RobloxAnimatingPageViewController setImgForeground:]
// type: void __cdecl(RobloxAnimatingPageViewController *self, SEL, id)
#[doc(alias = "-[RobloxAnimatingPageViewController setImgForeground:]")]
pub fn stub_0x53b3c() -> ! {
    todo!("0x53b3c -[RobloxAnimatingPageViewController setImgForeground:]")
}

// 0x53b60 — -[RobloxNavBarViewController initWithCoder:]
// type: RobloxNavBarViewController *__cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController initWithCoder:]")]
pub fn stub_0x53b60() -> ! {
    todo!("0x53b60 -[RobloxNavBarViewController initWithCoder:]")
}

// 0x53cbc — -[RobloxNavBarViewController dealloc]
// type: void __cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController dealloc]")]
pub fn stub_0x53cbc() -> ! {
    todo!("0x53cbc -[RobloxNavBarViewController dealloc]")
}

// 0x53e6c — -[RobloxNavBarViewController setUrl:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController setUrl:]")]
pub fn stub_0x53e6c() -> ! {
    todo!("0x53e6c -[RobloxNavBarViewController setUrl:]")
}

// 0x53e8c — -[RobloxNavBarViewController getUrl]
// type: id __cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController getUrl]")]
pub fn stub_0x53e8c() -> ! {
    todo!("0x53e8c -[RobloxNavBarViewController getUrl]")
}

// 0x53e9c — -[RobloxNavBarViewController gotStartLeaveGameNotification:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController gotStartLeaveGameNotification:]")]
pub fn stub_0x53e9c() -> ! {
    todo!("0x53e9c -[RobloxNavBarViewController gotStartLeaveGameNotification:]")
}

// 0x53f38 — -[RobloxNavBarViewController gotDidLeaveGameNotification:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController gotDidLeaveGameNotification:]")]
pub fn stub_0x53f38() -> ! {
    todo!("0x53f38 -[RobloxNavBarViewController gotDidLeaveGameNotification:]")
}

// 0x53fac — -[RobloxNavBarViewController viewWillAppear:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, char)
#[doc(alias = "-[RobloxNavBarViewController viewWillAppear:]")]
pub fn stub_0x53fac() -> ! {
    todo!("0x53fac -[RobloxNavBarViewController viewWillAppear:]")
}

// 0x53ffc — -[RobloxNavBarViewController viewDidAppear:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, char)
#[doc(alias = "-[RobloxNavBarViewController viewDidAppear:]")]
pub fn stub_0x53ffc() -> ! {
    todo!("0x53ffc -[RobloxNavBarViewController viewDidAppear:]")
}

// 0x540c4 — ___44-[RobloxNavBarViewController viewDidAppear:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___44-[RobloxNavBarViewController viewDidAppear:]_block_invoke")]
pub fn stub_0x540c4() -> ! {
    todo!("0x540c4 ___44-[RobloxNavBarViewController viewDidAppear:]_block_invoke")
}

// 0x540f0 — ___copy_helper_block__15
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__15")]
pub fn stub_0x540f0() -> ! {
    todo!("0x540f0 ___copy_helper_block__15")
}

// 0x540fc — ___destroy_helper_block__15
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__15")]
pub fn stub_0x540fc() -> ! {
    todo!("0x540fc ___destroy_helper_block__15")
}

// 0x54104 — -[RobloxNavBarViewController viewDidLoad]
// type: void __cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController viewDidLoad]")]
pub fn stub_0x54104() -> ! {
    todo!("0x54104 -[RobloxNavBarViewController viewDidLoad]")
}

// 0x543dc — -[RobloxNavBarViewController hideBackButton]
// type: void __cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController hideBackButton]")]
pub fn stub_0x543dc() -> ! {
    todo!("0x543dc -[RobloxNavBarViewController hideBackButton]")
}

// 0x543fc — -[RobloxNavBarViewController showBackButton]
// type: void __cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController showBackButton]")]
pub fn stub_0x543fc() -> ! {
    todo!("0x543fc -[RobloxNavBarViewController showBackButton]")
}

// 0x5441c — -[RobloxNavBarViewController viewDidUnload]
// type: void __cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController viewDidUnload]")]
pub fn stub_0x5441c() -> ! {
    todo!("0x5441c -[RobloxNavBarViewController viewDidUnload]")
}

// 0x5449c — -[RobloxNavBarViewController showFullscreenText:]
// type: void __cdecl(RobloxNavBarViewController *self, SEL, id)
#[doc(alias = "-[RobloxNavBarViewController showFullscreenText:]")]
pub fn stub_0x5449c() -> ! {
    todo!("0x5449c -[RobloxNavBarViewController showFullscreenText:]")
}

// 0x54514 — ___49-[RobloxNavBarViewController showFullscreenText:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___49-[RobloxNavBarViewController showFullscreenText:]_block_invoke")]
pub fn stub_0x54514() -> ! {
    todo!("0x54514 ___49-[RobloxNavBarViewController showFullscreenText:]_block_invoke")
}

// 0x54594 — ___copy_helper_block_134
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_134")]
pub fn stub_0x54594() -> ! {
    todo!("0x54594 ___copy_helper_block_134")
}

// 0x545a0 — ___destroy_helper_block_135
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_135")]
pub fn stub_0x545a0() -> ! {
    todo!("0x545a0 ___destroy_helper_block_135")
}

// 0x545a8 — -[RobloxNavBarViewController hideFullscreenText]
// type: void __cdecl(RobloxNavBarViewController *self, SEL)
#[doc(alias = "-[RobloxNavBarViewController hideFullscreenText]")]
pub fn stub_0x545a8() -> ! {
    todo!("0x545a8 -[RobloxNavBarViewController hideFullscreenText]")
}

// 0x545f8 — ___48-[RobloxNavBarViewController hideFullscreenText]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___48-[RobloxNavBarViewController hideFullscreenText]_block_invoke")]
pub fn stub_0x545f8() -> ! {
    todo!("0x545f8 ___48-[RobloxNavBarViewController hideFullscreenText]_block_invoke")
}

// 0x54648 — ___copy_helper_block_139
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_139")]
pub fn stub_0x54648() -> ! {
    todo!("0x54648 ___copy_helper_block_139")
}

// 0x54654 — ___destroy_helper_block_140
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_140")]
pub fn stub_0x54654() -> ! {
    todo!("0x54654 ___destroy_helper_block_140")
}

// 0x5465c — +[RobloxNavBarViewController checkForInAppPurchases:navigationType:]
// type: char __cdecl(id, SEL, id, int)
#[doc(alias = "+[RobloxNavBarViewController checkForInAppPurchases:navigationType:]")]
pub fn stub_0x5465c() -> ! {
    todo!("0x5465c +[RobloxNavBarViewController checkForInAppPurchases:navigationType:]")
}

#[cfg(test)]
mod jump_stick_batch_tests {
    use super::*;
    use crate::generated_171::GameVC;

    #[test]
    fn login_flow() {
        let mut vc = GameVC::default();
        stub_0x4e8b8(&mut vc);
        assert!(vc.signup_shown);
        let mut login = LoginAttempt::default();
        stub_0x4e9a0(&mut login, "builderman");
        assert_eq!(login.attempts, 1);
        assert_eq!(login.user, "builderman");
        stub_0x4ea30(&mut login, false);
        assert_eq!(login.ok, 0);
        stub_0x4eac8(&mut login, true);
        assert_eq!(login.ok, 1);
        stub_0x4e98c();
        stub_0x4e998();
        stub_0x4edcc();
        stub_0x4edf0();
        assert_eq!(stub_0x4ef74(), 1);
        assert_eq!(stub_0x4f7bc(), 1);
    }

    #[test]
    fn jump_button() {
        let mut btn = stub_0x4f188([0.0, 0.0, 64.0, 64.0]);
        assert_eq!(btn.frame, [0.0, 0.0, 64.0, 64.0]);
        stub_0x4f404();
        assert!(!btn.seated);
        stub_0x4f2fc(&mut btn);
        assert!(btn.seated);
        stub_0x4f408(&mut btn);
        assert!(btn.jumping);
        stub_0x4f43c(&mut btn);
        assert!(!btn.jumping);
        stub_0x4f2b0(&mut btn);
        assert_eq!(btn, JumpBtn::default());
    }

    #[test]
    fn thumbstick() {
        let mut stick = stub_0x4f9d0([0.0; 4]);
        stub_0x4fcf4(&mut stick, 1);
        assert_eq!(stick.style, 1);
        stub_0x4fcf4(&mut stick, 7);
        assert_eq!(stick.style, 0);
        assert_eq!(stub_0x4fdb8(0), 0);
        assert_eq!(stub_0x4fdb8(1), 1);
        assert_eq!(stub_0x4fdb8(2), 0);
        assert_eq!(stub_0x4fdc4([0.0, 0.0], [3.0, 4.0]), 5.0);
        let r = stub_0x4fdf4([1.0, 0.0], [0.0, 0.0], std::f32::consts::FRAC_PI_2);
        assert!((r[0] - 0.0).abs() < 1e-6);
        assert!((r[1] - 1.0).abs() < 1e-6);
        stub_0x4fe88(&mut stick, 5, false);
        assert_eq!(stick.touch, None);
        stub_0x4fe88(&mut stick, 5, true);
        assert_eq!(stick.touch, Some(5));
        stub_0x4fe88(&mut stick, 6, true);
        assert_eq!(stick.touch, Some(5));
        stub_0x4fd40();
        stub_0x4fd4c();
        stub_0x4fd54(&mut stick);
        assert_eq!(stick, ThumbStick::default());
    }
}

#[cfg(test)]
mod stick_menu_batch_tests {
    use super::*;

    #[test]
    fn stick_moves() {
        let mut stick = stub_0x4f9d0([0.0; 4]);
        stub_0x4fe88(&mut stick, 5, true);
        stub_0x50108(&mut stick, 0.2, 0.1);
        assert_eq!(stick.move_vec, [0.2, 0.1]);
        stub_0x50338(&mut stick, -0.3, 0.4);
        assert_eq!(stick.move_vec, [-0.3, 0.4]);
        stub_0x506cc(&mut stick, 0.0, -1.0);
        assert_eq!(stick.move_vec, [0.0, -1.0]);
        stub_0x508b0(&mut stick);
        assert_eq!(stick.touch, None);
        assert_eq!(stick.move_vec, [0.0, 0.0]);
        stub_0x50960();
        stub_0x50c18();
        stub_0x50c80();
        assert_eq!(stub_0x50c98(), 1);
        stub_0x509a8();
        stub_0x509b4();
        stub_0x50c6c();
        stub_0x50c78();
        stub_0x50c84();
        stub_0x50c90();
    }

    #[test]
    fn game_menu() {
        let mut menu = stub_0x50eb0();
        assert!(!stub_0x51370(&menu));
        stub_0x513f8(&mut menu);
        assert!(stub_0x51370(&menu));
        stub_0x513c4(&mut menu);
        assert!(!stub_0x51370(&menu));
        stub_0x513c4(&mut menu);
        assert!(stub_0x51370(&menu));
        stub_0x513b4(&mut menu);
        assert!(!stub_0x51370(&menu));
        stub_0x51380(&mut menu);
        assert_eq!(menu.leave_count, 1);
        stub_0x51570();
        stub_0x515dc();
        stub_0x515e8();
        stub_0x512f8(&mut menu);
        assert_eq!(menu, GameMenu::default());
    }
}

#[cfg(test)]
mod menu_mainvc_batch_tests {
    use super::*;

    #[test]
    fn hide_blocks() {
        stub_0x51738();
        stub_0x517a8();
        stub_0x517ec();
        assert_eq!(stub_0x517f0(), 1);
        assert_eq!(stub_0x51bb0(), 1);
        stub_0x51794();
        stub_0x517a0();
        stub_0x517d8();
        stub_0x517e4();
        stub_0x51e54();
        stub_0x51e60();
    }

    #[test]
    fn menu_button() {
        let mut btn = stub_0x51a04([0.0, 0.0, 44.0, 44.0]);
        assert!(btn.enabled);
        let mut menu = stub_0x50eb0();
        stub_0x51b44(&mut btn, &mut menu);
        assert!(menu.shown);
        assert!(btn.menu_open);
        assert!(!btn.enabled);
        stub_0x51b44(&mut btn, &mut menu);
        assert!(!menu.shown);
        assert!(!btn.menu_open);
        assert!(btn.enabled);
        stub_0x51af8(&mut btn);
        assert_eq!(btn, MenuBtn::default());
    }

    #[test]
    fn main_vc() {
        let mut vc = stub_0x51eb8();
        stub_0x51ee8(&mut vc);
        assert!(vc.loaded);
        stub_0x51e68(&mut vc, 3);
        assert_eq!(vc.view, Some(3));
        stub_0x51e78(&mut vc);
        assert_eq!(vc.subviews, 1);
        stub_0x51f50(&mut vc, 11);
        assert_eq!(stub_0x51f40(&vc), Some(11));
        stub_0x51f70(&mut vc, 12);
        assert_eq!(stub_0x51f60(&vc), Some(12));
        stub_0x51f80(&mut vc, 13);
        assert_eq!(stub_0x51f90(&vc), Some(13));
        stub_0x51f14(&mut vc);
        assert!(!vc.loaded);
        let mut bare = MainVC::default();
        stub_0x51e78(&mut bare);
        assert_eq!(bare.subviews, 0);
    }
}

#[cfg(test)]
mod anim_vc_batch_tests {
    use super::*;

    #[test]
    fn main_vc_controllers() {
        let mut vc = stub_0x51eb8();
        assert_eq!(stub_0x51fa0(&vc), None);
        stub_0x51fb0(&mut vc, 21);
        assert_eq!(stub_0x51fa0(&vc), Some(21));
        assert_eq!(stub_0x51fd0(&vc), None);
        stub_0x51fc0(&mut vc, 22);
        assert_eq!(stub_0x51fd0(&vc), Some(22));
        assert_eq!(stub_0x51fe0(), 1);
    }

    #[test]
    fn anim_lifecycle() {
        let mut vc = stub_0x52178();
        assert!(!vc.loaded);
        stub_0x52400(&mut vc);
        assert!(vc.loaded);
        stub_0x5234c(&mut vc, false);
        assert!(!vc.panning);
        stub_0x5234c(&mut vc, true);
        assert!(vc.panning);
        assert_eq!(vc.pan_count, 1);
        stub_0x5233c(&mut vc);
        assert!(!vc.panning);
        stub_0x52614(&mut vc);
        assert!(vc.appeared);
        assert!(vc.panning);
        stub_0x52a50(&mut vc);
        assert!(!vc.appeared);
        assert!(!vc.panning);
        stub_0x523d4(&mut vc);
        assert!(vc.mem_warning);
        stub_0x52384();
        assert!(!stub_0x52aa0());
        assert_eq!(stub_0x52580(None), 0.0);
        assert_eq!(stub_0x52580(Some(3.5)), 3.5);
        stub_0x52280(&mut vc);
        assert_eq!(vc, AnimVC::default());
    }

    #[test]
    fn animations() {
        let mut vc = stub_0x52178();
        stub_0x52aec(&mut vc);
        stub_0x52f98(&mut vc);
        stub_0x53034(&mut vc);
        assert_eq!(vc.anims, 3);
        stub_0x52dac();
        stub_0x52f14();
        stub_0x52ed4();
        stub_0x52ef8();
        stub_0x52f44();
        stub_0x52f74();
    }
}
