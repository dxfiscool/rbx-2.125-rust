// Auto-generated skeletons for rbx-script — Lua|Script|Yield|CodeGen batch (filler)
// Filter: Lua|Script|Yield|CodeGen (4818 filtered, 0 remaining) -> global gap filler EA-sorted asc next 150 not yet in script crate
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +150 stubs | range 0x463cc..0x4e868 EA-sorted asc next 150 global not yet in script crate (script 16612 -> 16762 distinct)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use std::sync::LazyLock;
use crate::generated_112::{SlotConn, SlotFn, SlotList};

/// `DataModel*`-flavor slot static mutex (IDA 0x4b4bc..0x4b4c0, same shape
/// as 0x45fa0).
static SLOT_DATAMODEL_MUTEX: LazyLock<u32> = LazyLock::new(|| 1);
/// `__GLOBAL__I_a` one-shot latches (IDA 0x4c034/0x4c498).
static GLOBAL_A18_INIT: LazyLock<u32> = LazyLock::new(|| 1);
static GLOBAL_A19_INIT: LazyLock<u32> = LazyLock::new(|| 1);

/// `GameInputViewController` observable state (IDA 0x4c3f4..0x4c46c):
/// the load latch. The owned `ControlView` folds into the host.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GameInputVC {
    pub loaded: bool,
}

/// `GameKeyboard` observable state (IDA 0x4c71c..0x4d07c): visibility,
/// field text, placeholder, bound text box, and submitted edits. The
/// `UITextField`/notification-center peers fold into the host.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GameKeys {
    pub shown: bool,
    pub text: String,
    pub placeholder: String,
    pub current_box: Option<u32>,
    pub submitted: u32,
}

/// Bound ObjC-forwarding functor for the UIEvent signal (IDA 0x463cc,
/// 0x4642c): the stored target/selector pair plus the armed latch and
/// dispatch count. The `bind_t` argument shuffling folds into the host.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BindSlot {
    pub bound: bool,
    pub calls: u32,
}
/// `__GLOBAL__I_a` one-shot latches (IDA 0x46490/0x46f64/0x47424).
static GLOBAL_A15_INIT: LazyLock<u32> = LazyLock::new(|| 1);
static GLOBAL_A16_INIT: LazyLock<u32> = LazyLock::new(|| 1);
static GLOBAL_A17_INIT: LazyLock<u32> = LazyLock::new(|| 1);

/// `CharacterMove` observable state (IDA 0x466cc..0x469e8): the frame, the
/// input-service connection, movement latch, and move vector.
/// `ThumbStickControl` peers fold into the host.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CharMove {
    pub frame: [f32; 4],
    pub connected: bool,
    pub moving: bool,
    pub move_vec: [f32; 2],
}

/// `ControlComponent` observable state (IDA 0x47178..0x47274): user
/// interaction plus the resolved control view and game handles.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CtrlComp {
    pub interaction: bool,
    pub view: Option<u32>,
    pub game: Option<u32>,
}
/// `ControlView` observable state (IDA 0x47638..0x49acc): the frame, bound
/// game, control visibility, event wiring, tap/pinch tracking, input
/// binding, built controls, focused box, and delivered mouse events.
/// UIKit peers (menu, jump, camera, pinch) fold into the host.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CtrlView {
    pub frame: [f32; 4],
    pub game: Option<u32>,
    pub controls_visible: bool,
    pub events_up: bool,
    pub tap_touch: Option<u32>,
    pub pinch_scale: f32,
    pub pinch_time: f32,
    pub input_bound: bool,
    pub controls_built: bool,
    pub menu_built: bool,
    pub focused_textbox: Option<u32>,
    pub mouse_events: u32,
}

// 0x463cc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorbPvN3RBX7UIEventEENS3_5list5INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,bool,void *,RBX::UIEvent),boost::_bi::list5<boost::_bi::value<objc_object *>,boost::_bi::list5<objc_selector>,boost::arg<1>,boost::_bi::list5<objc_selector><2>,boost::_bi::list5<objc_selector><3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,bool,void *,RBX::UIEvent),boost::_bi::list5<boost::_bi::value<objc_object *>,boost::_bi::list5<objc_selector>,boost::arg<1>,boost::_bi::list5<objc_selector><2>,boost::_bi::list5<objc_selector><3>>>>&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x463cc(slot: &mut BindSlot, op: u32) {
    // IDA 0x463cc: `functor_manager<bind_t...>::manage` — clone (op 0)
    // copies the buffer (0x463de..0x463e6), destroy (op 1) drops it, and
    // the type queries (ops 3/4, 0x463f4/0x463d6) answer metadata; the
    // buffer/type glue folds into the host.
    match op {
        0 => slot.bound = true,
        1 => slot.bound = false,
        _ => {}
    }
}

// 0x4642c — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorbPvN3RBX7UIEventEENS3_5list5INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEvbS8_SA_E6invokeERNS1_15function_bufferEbS8_SA_
// type: int __fastcall(int, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,bool,void *,RBX::UIEvent),boost::_bi::list5<boost::_bi::value<objc_object *>,boost::_bi::list5<objc_selector>,boost::arg<1>,boost::_bi::list5<objc_selector><2>,boost::_bi::list5<objc_selector><3>>>,void,bool,objc_selector *,RBX>::invoke(boost::detail::function::function_buffer &,bool,objc_selector *,RBX)")]
pub fn stub_0x4642c(slot: &mut BindSlot) {
    // IDA 0x4642c: `void_function_obj_invoker3::invoke` calls through to
    // the stored target/selector with the bound args (0x46462); the
    // ObjC send folds into the host.
    if slot.bound {
        slot.calls += 1;
    }
}

// 0x46490 — __GLOBAL__I_a_15
#[doc(alias = "global constructor keyed to_a_15")]
pub fn stub_0x46490() -> u32 {
    // IDA 0x46490: `__GLOBAL__I_a_15` — see `GLOBAL_A15_INIT`.
    *GLOBAL_A15_INIT
}

// 0x466cc — -[CharacterMove init:]
// type: id __cdecl(CharacterMove *self, SEL, CGRect)
#[doc(alias = "-[CharacterMove init:]")]
pub fn stub_0x466cc(frame: [f32; 4]) -> CharMove {
    // IDA 0x466cc: `CharacterMove init:` chains to `ThumbStickControl
    // init:` with the frame (0x466e6..0x46702); the superclass glue
    // folds into the host.
    CharMove { frame, ..CharMove::default() }
}

// 0x46704 — -[CharacterMove setupCharacterMoveConnection]
// type: void __cdecl(CharacterMove *self, SEL)
#[doc(alias = "-[CharacterMove setupCharacterMoveConnection]")]
pub fn stub_0x46704(move_: &mut CharMove) {
    // IDA 0x46704: `setupCharacterMoveConnection` resolves the input
    // service (0x46738) and wires the movement connection when present
    // (0x46760); the service lookup folds into the host.
    move_.connected = true;
}

// 0x467e8 — -[CharacterMove localCharacterMovementEnabledChange:]
// type: void __cdecl(CharacterMove *self, SEL, const PropertyDescriptor *)
#[doc(alias = "-[CharacterMove localCharacterMovementEnabledChange:]")]
pub fn stub_0x467e8() {
    // IDA 0x467e8: `localCharacterMovementEnabledChange:` — empty body;
    // no-op.
}

// 0x467ec — -[CharacterMove touchesEnded:withEvent:]
// type: void __cdecl(CharacterMove *self, SEL, id, id)
#[doc(alias = "-[CharacterMove touchesEnded:withEvent:]")]
pub fn stub_0x467ec(move_: &mut CharMove, thumb: bool) {
    // IDA 0x467ec: `touchesEnded` enumerates the touches (0x46838..) and
    // cancels when one is the thumbstick touch (0x46870..0x468a0).
    if thumb {
        stub_0x4698c(move_);
    }
}

// 0x468bc — -[CharacterMove touchesCancelled:withEvent:]
// type: void __cdecl(CharacterMove *self, SEL, id, id)
#[doc(alias = "-[CharacterMove touchesCancelled:withEvent:]")]
pub fn stub_0x468bc(move_: &mut CharMove, thumb: bool) {
    // IDA 0x468bc: `touchesCancelled` — same cancel-on-thumbstick-touch
    // shape as 0x467ec.
    if thumb {
        stub_0x4698c(move_);
    }
}

// 0x4698c — -[CharacterMove cancelMovement]
// type: void __cdecl(CharacterMove *self, SEL)
#[doc(alias = "-[CharacterMove cancelMovement]")]
pub fn stub_0x4698c(move_: &mut CharMove) {
    // IDA 0x4698c: `cancelMovement` chains to super (0x469a8..0x469b2),
    // then zeroes the local-character move via the input service
    // (0x469c4..0x469de); the service send folds into the host.
    move_.moving = false;
    move_.move_vec = [0.0, 0.0];
}

// 0x469e8 — -[CharacterMove touchesMoved:withEvent:]
// type: void __cdecl(CharacterMove *self, SEL, id, id)
#[doc(alias = "-[CharacterMove touchesMoved:withEvent:]")]
pub fn stub_0x469e8(move_: &mut CharMove, dx: f32, dy: f32) {
    // IDA 0x469e8: `touchesMoved` tracks the thumbstick drag, re-seats
    // the stick visuals, and drives the local character (the projection
    // math folds into the host).
    move_.moving = true;
    move_.move_vec = [dx, dy];
}

// 0x46f64 — __GLOBAL__I_a_16
#[doc(alias = "global constructor keyed to_a_16")]
pub fn stub_0x46f64() -> u32 {
    // IDA 0x46f64: `__GLOBAL__I_a_16` — see `GLOBAL_A16_INIT`.
    *GLOBAL_A16_INIT
}

// 0x47178 — -[ControlComponent init]
// type: ControlComponent *__cdecl(ControlComponent *self, SEL)
#[doc(alias = "-[ControlComponent init]")]
pub fn stub_0x47178() -> CtrlComp {
    // IDA 0x47178: `ControlComponent init` chains to super (0x47192..)
    // and enables user interaction (0x471b4).
    CtrlComp { interaction: true, ..CtrlComp::default() }
}

// 0x471c0 — -[ControlComponent findControlView]
// type: id __cdecl(ControlComponent *self, SEL)
#[doc(alias = "-[ControlComponent findControlView]")]
pub fn stub_0x471c0(comp: &CtrlComp) -> Option<u32> {
    // IDA 0x471c0: `findControlView` answers self when it is a
    // `ControlView`, else walks superviews for one (0x471e8..0x47268)
    // and answers null past a non-`UIView` ancestor (0x47264).
    comp.view
}

// 0x47274 — -[ControlComponent getGameFromControlView]
// type: Game *__cdecl(ControlComponent *self, SEL)
#[doc(alias = "-[ControlComponent getGameFromControlView]")]
pub fn stub_0x47274(comp: &CtrlComp) -> Option<u32> {
    // IDA 0x47274: `getGameFromControlView` finds the view (0x472a4)
    // and answers its game, defaulting to null (0x472ae..0x472ce).
    if comp.view.is_some() { comp.game } else { None }
}

// 0x47424 — __GLOBAL__I_a_17
#[doc(alias = "global constructor keyed to_a_17")]
pub fn stub_0x47424() -> u32 {
    // IDA 0x47424: `__GLOBAL__I_a_17` — see `GLOBAL_A17_INIT`.
    *GLOBAL_A17_INIT
}

// 0x47638 — -[ControlView init:withGame:]
// type: id __cdecl(ControlView *self, SEL, CGRect, shared_ptr<RBX::Game>)
#[doc(alias = "-[ControlView init:withGame:]")]
pub fn stub_0x47638(frame: [f32; 4], game: Option<u32>) -> CtrlView {
    // IDA 0x47638: `ControlView init:withGame:` chains to super, registers
    // notifications, installs the pinch recognizer, and binds the game;
    // the UIKit/service glue folds into the host.
    CtrlView { frame, game, controls_visible: true, ..CtrlView::default() }
}

// 0x47904 — -[ControlView dealloc]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView dealloc]")]
pub fn stub_0x47904(view: &mut CtrlView) {
    // IDA 0x47904: `dealloc` removes the notification observer (0x47924..)
    // and releases the menu/jump/camera/pinch peers
    // (0x47946..0x479cc); drop glue covers it and the record resets.
    *view = CtrlView::default();
}

// 0x479f8 — -[ControlView setGame:]
// type: void __cdecl(ControlView *self, SEL, shared_ptr<RBX::Game>)
#[doc(alias = "-[ControlView setGame:]")]
pub fn stub_0x479f8(view: &mut CtrlView, game: Option<u32>) {
    // IDA 0x479f8: `setGame:` rebinds the shared game (0x47a28) and
    // refreshes from the data model; the `shared_ptr` glue folds into
    // the host.
    view.game = game;
}

// 0x47aec — -[ControlView gotStartLeaveGameNotification:]
// type: void __cdecl(ControlView *self, SEL, id)
#[doc(alias = "-[ControlView gotStartLeaveGameNotification:]")]
pub fn stub_0x47aec(view: &mut CtrlView) {
    // IDA 0x47aec: `gotStartLeaveGameNotification:` disconnects events
    // (0x47af8).
    view.events_up = false;
}

// 0x47afc — -[ControlView dataModelChanged:]
// type: void __cdecl(ControlView *self, SEL, DataModel *)
#[doc(alias = "-[ControlView dataModelChanged:]")]
pub fn stub_0x47afc(view: &mut CtrlView, model: Option<u32>) {
    // IDA 0x47afc: `dataModelChanged:` sets up events plus input controls
    // for a live model (0x47b12..0x47b1e) and disconnects for null
    // (0x47b2a..0x47b34).
    view.events_up = model.is_some();
}

// 0x47b38 — -[ControlView setControlVisibility:]
// type: void __cdecl(ControlView *self, SEL, char)
#[doc(alias = "-[ControlView setControlVisibility:]")]
pub fn stub_0x47b38(view: &mut CtrlView, visible: bool) {
    // IDA 0x47b38: `setControlVisibility:` captures the flag in a block
    // (0x47b6c..0x47b84) and `dispatch_async`s it to main (0x47b88); the
    // queue hop folds into the caller — see `stub_0x47b90`.
    stub_0x47b90(view, visible);
}

// 0x47b90 — ___36-[ControlView setControlVisibility:]_block_invoke
#[doc(alias = "___36-[ControlView setControlVisibility:]_block_invoke")]
pub fn stub_0x47b90(view: &mut CtrlView, visible: bool) {
    // IDA 0x47b90: the visibility block hides the menu (0x47bc0) and the
    // camera/jump peer (0x47bd4..0x47bf6) exactly when `visible` is
    // false.
    view.controls_visible = visible;
}

// 0x47c04 — ___copy_helper_block__8
#[doc(alias = "___copy_helper_block__8")]
pub fn stub_0x47c04() {
    // IDA 0x47c04: `__copy_helper_block__8` retains the captured view
    // (0x47c0a); `Arc` glue covers it — no-op.
}

// 0x47c10 — ___destroy_helper_block__8
#[doc(alias = "___destroy_helper_block__8")]
pub fn stub_0x47c10() {
    // IDA 0x47c10: `__destroy_helper_block__8` releases the captured
    // view (pair of 0x47c04); `Arc` glue covers it — no-op.
}

// 0x47c18 — -[ControlView showControls]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView showControls]")]
pub fn stub_0x47c18(view: &mut CtrlView) {
    // IDA 0x47c18: `showControls` shows via `setControlVisibility:`
    // (0x47c26).
    stub_0x47b38(view, true);
}

// 0x47c2c — -[ControlView hideControls]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView hideControls]")]
pub fn stub_0x47c2c(view: &mut CtrlView) {
    // IDA 0x47c2c: `hideControls` hides via `setControlVisibility:`
    // (0x47c3a).
    stub_0x47b38(view, false);
}

// 0x47c40 — -[ControlView postMouseEventProcessedFromOverlay:inputObject:event:]
// type: void __cdecl(ControlView *self, SEL, bool, void *, UIEvent)
#[doc(alias = "-[ControlView postMouseEventProcessedFromOverlay:inputObject:event:]")]
pub fn stub_0x47c40(view: &mut CtrlView) {
    // IDA 0x47c40: `postMouseEventProcessedFromOverlay` forwards the
    // overlay mouse event to the game through the data model; the
    // routing folds into the host and delivery is observed.
    view.mouse_events += 1;
}

// 0x47d48 — -[ControlView postMouseEventProcessed:inputObject:event:]
// type: void __cdecl(ControlView *self, SEL, bool, void *, UIEvent)
#[doc(alias = "-[ControlView postMouseEventProcessed:inputObject:event:]")]
pub fn stub_0x47d48(view: &mut CtrlView, touch: Option<u32>, processed: bool) {
    // IDA 0x47d48: `postMouseEventProcessed` invalidates the tap gesture
    // (0x47d74) when the processed touch is the tap touch (0x47d62).
    if processed && touch.is_some() && touch == view.tap_touch {
        stub_0x48ff8(view, touch);
    }
}

// 0x47d78 — -[ControlView setupLocalPlayerConnections]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView setupLocalPlayerConnections]")]
pub fn stub_0x47d78() {
    // IDA 0x47d78: `setupLocalPlayerConnections` — empty body; no-op.
}

// 0x47d7c — -[ControlView textBoxFocusGained:]
// type: void __cdecl(ControlView *self, SEL, shared_ptr<RBX::TextBox>)
#[doc(alias = "-[ControlView textBoxFocusGained:]")]
pub fn stub_0x47d7c(view: &mut CtrlView, textbox: u32) {
    // IDA 0x47d7c: `textBoxFocusGained:` routes the focused box to the
    // game keyboard (0x47d9c..); the keyboard presentation folds into
    // the host and the focus target is observed.
    view.focused_textbox = Some(textbox);
}

// 0x47ea4 — -[ControlView getGame]
// type: shared_ptr<RBX::Game> *__cdecl(shared_ptr<RBX::Game> *__return_ptr __struct_ptr retstr, ControlView *self, SEL)
#[doc(alias = "-[ControlView getGame]")]
pub fn stub_0x47ea4(view: &CtrlView) -> Option<u32> {
    // IDA 0x47ea4: `getGame` copies the bound shared game out
    // (0x47ee4..0x47f0a); the `shared_ptr` glue folds into the host.
    view.game
}

// 0x47f48 — -[ControlView setupEvents]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView setupEvents]")]
pub fn stub_0x47f48(view: &mut CtrlView) {
    // IDA 0x47f48: `setupEvents` wires the game-loaded and input-service
    // connections; the signal glue folds into the host.
    view.events_up = true;
}

// 0x4818c — -[ControlView disconnectEvents]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView disconnectEvents]")]
pub fn stub_0x4818c(view: &mut CtrlView) {
    // IDA 0x4818c: `disconnectEvents` disconnects the game-loaded and
    // both input-property connections (0x481a0..0x481c8).
    view.events_up = false;
}

// 0x481cc — -[ControlView bindToUserInputService:]
// type: void __cdecl(ControlView *self, SEL, shared_ptr<RBX::DataModel>)
#[doc(alias = "-[ControlView bindToUserInputService:]")]
pub fn stub_0x481cc(view: &mut CtrlView) {
    // IDA 0x481cc: `bindToUserInputService:` resolves the input service
    // from the model and binds the overlay properties; the service glue
    // folds into the host.
    view.input_bound = true;
}

// 0x48604 — -[ControlView bindUserInputService]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView bindUserInputService]")]
pub fn stub_0x48604(view: &mut CtrlView) {
    // IDA 0x48604: `bindUserInputService` resolves the game/overlay
    // models itself, then binds like 0x481cc.
    view.input_bound = true;
}

// 0x487d4 — -[ControlView isValidUserInputProperty:]
// type: char __cdecl(ControlView *self, SEL, const PropertyDescriptor *)
#[doc(alias = "-[ControlView isValidUserInputProperty:]")]
pub fn stub_0x487d4(view: &CtrlView, name: Option<&str>) -> bool {
    // IDA 0x487d4: `isValidUserInputProperty:` needs a live game
    // (0x487e4..0x487ea), a descriptor (0x487ec), and a name other than
    // `Parent` (0x487ee..0x48804).
    view.game.is_some() && matches!(name, Some(n) if n != "Parent")
}

// 0x48918 — -[ControlView userInputPropertyChangedOnOverlay:]
// type: void __cdecl(ControlView *self, SEL, const PropertyDescriptor *)
#[doc(alias = "-[ControlView userInputPropertyChangedOnOverlay:]")]
pub fn stub_0x48918() {
    // IDA 0x48918: `userInputPropertyChangedOnOverlay:` applies the
    // overlay property change through to the input service; neither the
    // overlay lookup nor the application touches `ControlView` state —
    // no-op.
}

// 0x48a50 — -[ControlView setupInputControls]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView setupInputControls]")]
pub fn stub_0x48a50(view: &mut CtrlView) {
    // IDA 0x48a50: `setupInputControls` builds and seats the camera,
    // move, jump, menu, and keyboard controls; the UIKit glue folds into
    // the host.
    view.controls_built = true;
}

// 0x48fe8 — -[ControlView gameLoaded]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView gameLoaded]")]
pub fn stub_0x48fe8(view: &mut CtrlView) {
    // IDA 0x48fe8: `gameLoaded` shows via `showControls` (0x48ff4).
    stub_0x47c18(view);
}

// 0x48ff8 — -[ControlView invalidateTapGesture:]
// type: void __cdecl(ControlView *self, SEL, id)
#[doc(alias = "-[ControlView invalidateTapGesture:]")]
pub fn stub_0x48ff8(view: &mut CtrlView, touch: Option<u32>) {
    // IDA 0x48ff8: `invalidateTapGesture:` clears the tap touch for a
    // null gesture (0x48ffc) or a matching touch (0x49006..0x49012).
    if touch.is_none() || touch == view.tap_touch {
        view.tap_touch = None;
    }
}

// 0x49018 — -[ControlView createNativeMenu]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView createNativeMenu]")]
pub fn stub_0x49018(view: &mut CtrlView) {
    // IDA 0x49018: `createNativeMenu` allocs/inits the menu button
    // (0x49038..0x4907c) and adds it as a subview (0x49088); the UIKit
    // glue folds into the host.
    view.menu_built = true;
}

// 0x4908c — -[ControlView checkTouchesForTap:withEvent:]
// type: id __cdecl(ControlView *self, SEL, id, id)
#[doc(alias = "-[ControlView checkTouchesForTap:withEvent:]")]
pub fn stub_0x4908c(view: &CtrlView, touches: &[u32]) -> Option<u32> {
    // IDA 0x4908c: `checkTouchesForTap` answers the tap touch when it is
    // set (0x490ba..0x490c2) and present in the set (0x490da..), else
    // null.
    match view.tap_touch {
        Some(t) if touches.contains(&t) => Some(t),
        _ => None,
    }
}

// 0x4918c — -[ControlView sendMouseEventToGame:withTouch:]
// type: void __cdecl(ControlView *self, SEL, UIEvent, id)
#[doc(alias = "-[ControlView sendMouseEventToGame:withTouch:]")]
pub fn stub_0x4918c(view: &mut CtrlView) {
    // IDA 0x4918c: `sendMouseEventToGame:withTouch:` routes the mouse
    // event to the game via the data model; delivery is observed.
    view.mouse_events += 1;
}

// 0x49314 — -[ControlView touchesBegan:withEvent:]
// type: void __cdecl(ControlView *self, SEL, id, id)
#[doc(alias = "-[ControlView touchesBegan:withEvent:]")]
pub fn stub_0x49314(view: &mut CtrlView, touches: &[u32]) {
    // IDA 0x49314: `touchesBegan` claims the tap touch when none is held
    // and exactly one touch lands (0x4935e..0x4937c), then tracks it.
    if view.tap_touch.is_none() && touches.len() == 1 {
        view.tap_touch = Some(touches[0]);
    }
}

// 0x4951c — -[ControlView touchesEnded:withEvent:]
// type: void __cdecl(ControlView *self, SEL, id, id)
#[doc(alias = "-[ControlView touchesEnded:withEvent:]")]
pub fn stub_0x4951c(view: &mut CtrlView, touch: Option<u32>) {
    // IDA 0x4951c: `touchesEnded` resets the pinch clock (0x4955c),
    // resolves the tap (0x4956c), and releases a matching tap touch
    // (same shape as 0x49920).
    view.pinch_time = -1.0;
    if touch.is_some() && touch == view.tap_touch {
        view.tap_touch = None;
    }
}

// 0x49684 — -[ControlView touchesMoved:withEvent:]
// type: void __cdecl(ControlView *self, SEL, id, id)
#[doc(alias = "-[ControlView touchesMoved:withEvent:]")]
pub fn stub_0x49684(view: &mut CtrlView, tap_moved: bool) {
    // IDA 0x49684: `touchesMoved` re-checks the tap drag (0x496b8) and
    // forwards the moves; the mouse routing folds into the host — see
    // `stub_0x497d0`.
    stub_0x497d0(view, tap_moved);
}

// 0x497d0 — -[ControlView checkTapTouchMove:]
// type: void __cdecl(ControlView *self, SEL, id)
#[doc(alias = "-[ControlView checkTapTouchMove:]")]
pub fn stub_0x497d0(view: &mut CtrlView, tap_moved: bool) {
    // IDA 0x497d0: `checkTapTouchMove:` enumerates the touches
    // (0x4981c..) and invalidates the tap once it drags past the tap
    // slop; the distance threshold folds into the host.
    if tap_moved {
        view.tap_touch = None;
    }
}

// 0x49920 — -[ControlView touchesCancelled:withEvent:]
// type: void __cdecl(ControlView *self, SEL, id, id)
#[doc(alias = "-[ControlView touchesCancelled:withEvent:]")]
pub fn stub_0x49920(view: &mut CtrlView, touch: Option<u32>) {
    // IDA 0x49920: `touchesCancelled` enumerates (0x4996c..) and releases
    // the tap touch on a match (0x49998, same shape as 0x467ec).
    if touch.is_some() && touch == view.tap_touch {
        view.tap_touch = None;
    }
}

// 0x499e0 — -[ControlView twoFingerPinch:]
// type: void __cdecl(ControlView *self, SEL, id)
#[doc(alias = "-[ControlView twoFingerPinch:]")]
pub fn stub_0x499e0(view: &mut CtrlView, began: bool, scale: f32) {
    // IDA 0x499e0: `twoFingerPinch:` resets the pinch baseline on begin
    // (0x49a0e..0x49a20), ends the camera pan (0x49a3c), clears the tap
    // (0x49a50), and zooms by the scale delta through the input service
    // (0x49a6c..0x49a9e); the service send folds into the host.
    if began {
        view.pinch_scale = 1.0;
    }
    view.tap_touch = None;
    view.pinch_scale = scale;
}

// 0x49acc — -[ControlView oneFingerSingleTap]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView oneFingerSingleTap]")]
pub fn stub_0x49acc(view: &mut CtrlView) {
    // IDA 0x49acc: `oneFingerSingleTap` resolves the input service
    // (0x49aea..0x49af0) and clicks the tap point (0x49afe..); delivery
    // is observed.
    view.mouse_events += 1;
}

// 0x49bb4 — -[ControlView gestureRecognizer:shouldReceiveTouch:]
// type: char __cdecl(ControlView *self, SEL, id, id)
#[doc(alias = "-[ControlView gestureRecognizer:shouldReceiveTouch:]")]
pub fn stub_0x49bb4(_view: &CtrlView, is_pinch: bool, hit_self: bool) -> bool {
    // IDA 0x49bb4: `gestureRecognizer:shouldReceiveTouch:` answers true
    // outright (0x49bd2) except for the pinch recognizer (0x49bd6),
    // which takes the touch only when the hit test lands on the view or
    // its camera peer (0x49bf6..); the hit test folds into the host.
    if !is_pinch {
        true
    } else {
        hit_self
    }
}

// 0x49ca0 — -[ControlView .cxx_destruct]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView .cxx_destruct]")]
pub fn stub_0x49ca0(view: &mut CtrlView) {
    // IDA 0x49ca0: `.cxx_destruct` disconnects the three connections
    // (0x49d00..0x49d54) and releases the C++ ivars (game included);
    // the release glue folds into the host.
    view.events_up = false;
    view.game = None;
    view.input_bound = false;
}

// 0x49e18 — -[ControlView .cxx_construct]
// type: id __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView .cxx_construct]")]
pub fn stub_0x49e18() -> CtrlView {
    // IDA 0x49e18: `.cxx_construct` zeroes the frame size, tap origin,
    // game, and connections (0x49e30..0x49e78); folds into `Default`.
    CtrlView::default()
}

// 0x49e7c — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::DataModel *)>::connect<boost::function<void ()(RBX::DataModel *)>>(boost::function<void ()(RBX::DataModel *)> const&)")]
pub fn stub_0x49e7c(list: &mut SlotList) -> u32 {
    // IDA 0x49e7c: `DataModel*` `connect` — same new/construct/insert
    // shape as 0x49f64.
    list.slots += 1;
    list.slots
}

// 0x4b010 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX7TextBoxEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x4b010(slot: &mut BindSlot, op: u32) {
    // IDA 0x4b010: `functor_manager<TextBox-bind>::manage` — identical
    // clone/destroy/type op shape to 0x463cc (0x4b01a..0x4b038).
    stub_0x463cc(slot, op);
}

// 0x4b070 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX7TextBoxEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::TextBox>::invoke(boost::detail::function::function_buffer &,RBX::TextBox)")]
pub fn stub_0x4b070(slot: &mut BindSlot) {
    // IDA 0x4b070: `void_function_obj_invoker1::invoke` for the TextBox
    // bind — same call-through shape as 0x4642c.
    stub_0x4642c(slot);
}

// 0x4b088 — __ZN5boost3_bi5list3INS0_5valueIP11objc_objectEENS2_IP13objc_selectorEENS_3argILi1EEEEclIPFvS4_S6_NS_10shared_ptrIN3RBX7TextBoxEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, void (__fastcall **)(int, int, sp_counted_base **), const shared_count **, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,rbx_core::SharedPtr<RBX::TextBox>),boost::_bi::list1<RBX::TextBox&>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,rbx_core::SharedPtr<RBX::TextBox>) &,boost::_bi::list1<RBX::TextBox&> &,int)")]
pub fn stub_0x4b088(slot: &mut BindSlot) {
    // IDA 0x4b088: `list3<TextBox-bind>::operator()` applies the stored
    // target/selector to the box argument; the ObjC send folds into the
    // host.
    stub_0x4642c(slot);
}

// 0x4b164 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE6insertEPNS6_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::insert(rbx::signals::signal<void ()(RBX::DataModel *)>::slot *)")]
pub fn stub_0x4b164(list: &mut SlotList) {
    // IDA 0x4b164: `DataModel*` `insert` — same lock-and-append shape as
    // 0x4a28c.
    list.slots += 1;
}

// 0x4b374 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotEEaSEPS9_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::DataModel *)>::slot*)")]
pub fn stub_0x4b374() {
    // IDA 0x4b374: `intrusive_ptr<DataModel-slot>::operator=` from a raw
    // slot pointer (same shape as 0x4a49c); `Arc` glue covers it — no-op.
}

// 0x4b418 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotEEaSERKSA_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot> const&)")]
pub fn stub_0x4b418() {
    // IDA 0x4b418: `intrusive_ptr<DataModel-slot>::operator=` from a
    // const ref (same shape as 0x45808); `Arc` glue covers it — no-op.
}

// 0x4b4bc — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::safe_static_init_mutex(void)")]
pub fn stub_0x4b4bc() -> u32 {
    // IDA 0x4b4bc: `DataModel*` signal `safe_static_init_mutex` — see
    // `SLOT_DATAMODEL_MUTEX`.
    *SLOT_DATAMODEL_MUTEX
}

// 0x4b4c0 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::safe_static_do_get_mutex(void)")]
pub fn stub_0x4b4c0() -> u32 {
    // IDA 0x4b4c0: `DataModel*` signal `safe_static_do_get_mutex` — same
    // guarded once-init shape as 0x45fa4.
    *SLOT_DATAMODEL_MUTEX
}

// 0x4b5b8 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_EC2IPS7_EERKSB_T_
// type: _DWORD *__fastcall(_DWORD *, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::callable<rbx::signals::signal<void ()(RBX::DataModel *)>*>(boost::function<void ()(RBX::DataModel *)> const&,rbx::signals::signal<void ()(RBX::DataModel *)>*)")]
pub fn stub_0x4b5b8() {
    // IDA 0x4b5b8: `DataModel*` `callable<...,1>` ctor — same
    // vtable-install/function-copy shape as 0x4a544; construction glue
    // covers it — no-op.
}

// 0x4b6b4 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE13callable_slotIN5boost8functionIS5_EEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::callable_slot<boost::function<void ()(RBX::DataModel *)>>::~callable_slot()")]
pub fn stub_0x4b6b4() {
    // IDA 0x4b6b4: `DataModel*` `callable_slot` D1 dtor; drop glue covers
    // it — no-op.
}

// 0x4b788 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE13callable_slotIN5boost8functionIS5_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::callable_slot<boost::function<void ()(RBX::DataModel *)>>::~callable_slot() [0x4b788]")]
pub fn stub_0x4b788() {
    // IDA 0x4b788: `DataModel*` `callable_slot` D0 deleting dtor; drop
    // glue covers it — no-op.
}

// 0x4b860 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot10disconnectEv
// type: void __fastcall(int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::disconnect(void)")]
pub fn stub_0x4b860(slot: &mut SlotConn) {
    // IDA 0x4b860: `DataModel*` `slot::disconnect` — same guarded
    // mutex-lock-and-remove shape as 0x45c4c.
    slot.connected = false;
}

// 0x4b970 — __ZNK3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot9connectedEv
// type: bool __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::connected(void)const")]
pub fn stub_0x4b970(slot: &SlotConn) -> bool {
    // IDA 0x4b970: `DataModel*` `slot::connected` — same `a1+12 != 0`
    // shape as 0x45d5c.
    slot.connected
}

// 0x4b97c — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::call(RBX::DataModel *)")]
pub fn stub_0x4b97c(f: &mut SlotFn) {
    // IDA 0x4b97c: `DataModel*` `callable<...,1>::call` — same
    // throw-or-dispatch shape as 0x45d68.
    stub_0x4b98c(f);
}

// 0x4b984 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::call(RBX::DataModel *)")]
pub fn stub_0x4b984(f: &mut SlotFn) {
    // IDA 0x4b984: `DataModel*` `Thn4` adjustor thunk — same shape as
    // 0x45d98.
    stub_0x4b98c(f);
}

// 0x4b98c — __ZNK5boost9function1IvPN3RBX9DataModelEEclES3_
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "boost::function1<void,RBX::DataModel *>::operator()(RBX::DataModel *)const")]
pub fn stub_0x4b98c(f: &mut SlotFn) {
    // IDA 0x4b98c: `DataModel*` `function1::operator()` — same
    // throw-or-dispatch shape as 0x4a9e4.
    if !f.armed {
        panic!("bad_function_call");
    }
    f.calls += 1;
}

// 0x4ba50 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE6removeEPNS6_4slotE
// type: int __fastcall(char **, char *, int, const void *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::remove(rbx::signals::signal<void ()(RBX::DataModel *)>::slot *)")]
pub fn stub_0x4ba50(list: &mut SlotList) {
    // IDA 0x4ba50: `DataModel*` `signal::remove` — same assert/log/unlink
    // shape as 0x45eb0.
    list.slots = list.slots.saturating_sub(1);
    list.removed += 1;
}

// 0x4bb40 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot22safe_static_init_mutexEv
// type: int()
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::safe_static_init_mutex(void)")]
pub fn stub_0x4bb40() -> u32 {
    // IDA 0x4bb40: `DataModel*` slot `safe_static_init_mutex` — see
    // `SLOT_DATAMODEL_MUTEX`.
    *SLOT_DATAMODEL_MUTEX
}

// 0x4bb44 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot24safe_static_do_get_mutexEv
// type: void *()
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_0x4bb44() -> u32 {
    // IDA 0x4bb44: `DataModel*` slot `safe_static_do_get_mutex` — same
    // guarded once-init shape as 0x45fa4.
    *SLOT_DATAMODEL_MUTEX
}

// 0x4bc34 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::~callable()")]
pub fn stub_0x4bc34() {
    // IDA 0x4bc34: `DataModel*` `callable<...,1>` D1 dtor; drop glue
    // covers it — no-op.
}

// 0x4bd08 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::~callable() [0x4bd08]")]
pub fn stub_0x4bd08() {
    // IDA 0x4bd08: `DataModel*` `callable<...,1>` D0 deleting dtor; drop
    // glue covers it — no-op.
}

// 0x4bde0 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotD1Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::~slot()")]
pub fn stub_0x4bde0() {
    // IDA 0x4bde0: `DataModel*` `slot` D1 dtor; drop glue covers it —
    // no-op.
}

// 0x4be8c — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotD0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::~slot() [0x4be8c]")]
pub fn stub_0x4be8c() {
    // IDA 0x4be8c: `DataModel*` `slot` D0 deleting dtor; drop glue covers
    // it — no-op.
}

// 0x4bf3c — __ZN5boost9function1IvPN3RBX9DataModelEE13assign_to_ownERKS4_
// type: int __fastcall(int result, int *)
#[doc(alias = "boost::function1<void,RBX::DataModel *>::assign_to_own(boost::function1<void,RBX::DataModel *> const&)")]
pub fn stub_0x4bf3c(dst: &mut SlotFn, src: &SlotFn) {
    // IDA 0x4bf3c: `DataModel*` `function1::assign_to_own` — same copy
    // shape as 0x4639c.
    *dst = src.clone();
}

// 0x4bf6c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPN3RBX9DataModelEENS3_5list3INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x4bf6c(slot: &mut BindSlot, op: u32) {
    // IDA 0x4bf6c: `functor_manager<DataModel-bind>::manage` — identical
    // clone/destroy/type op shape to 0x463cc.
    stub_0x463cc(slot, op);
}

// 0x4bfcc — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPN3RBX9DataModelEENS3_5list3INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEEEEEEvSA_E6invokeERNS1_15function_bufferESA_
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::DataModel>::invoke(boost::detail::function::function_buffer &,RBX::DataModel)")]
pub fn stub_0x4bfcc(slot: &mut BindSlot) {
    // IDA 0x4bfcc: `void_function_obj_invoker1::invoke` for the
    // DataModel bind — same call-through shape as 0x4642c.
    stub_0x4642c(slot);
}

// 0x4c034 — __GLOBAL__I_a_18
#[doc(alias = "global constructor keyed to_a_18")]
pub fn stub_0x4c034() -> u32 {
    // IDA 0x4c034: `__GLOBAL__I_a_18` — see `GLOBAL_A18_INIT`.
    *GLOBAL_A18_INIT
}

// 0x4c3f4 — -[GameInputViewController dealloc]
// type: void __cdecl(GameInputViewController *self, SEL)
#[doc(alias = "-[GameInputViewController dealloc]")]
pub fn stub_0x4c3f4(vc: &mut GameInputVC) {
    // IDA 0x4c3f4: `GameInputViewController dealloc` releases the control
    // view (0x4c416) and chains to super (0x4c42e..); drop glue covers
    // it and the record resets.
    *vc = GameInputVC::default();
}

// 0x4c440 — -[GameInputViewController viewDidLoad]
// type: void __cdecl(GameInputViewController *self, SEL)
#[doc(alias = "-[GameInputViewController viewDidLoad]")]
pub fn stub_0x4c440(vc: &mut GameInputVC) {
    // IDA 0x4c440: `viewDidLoad` chains to super (0x4c45a..0x4c464);
    // the view-hierarchy glue folds into the host.
    vc.loaded = true;
}

// 0x4c46c — -[GameInputViewController viewDidUnload]
// type: void __cdecl(GameInputViewController *self, SEL)
#[doc(alias = "-[GameInputViewController viewDidUnload]")]
pub fn stub_0x4c46c(vc: &mut GameInputVC) {
    // IDA 0x4c46c: `viewDidUnload` chains to super (0x4c486..0x4c490)
    // and releases the view; the hierarchy glue folds into the host.
    vc.loaded = false;
}

// 0x4c498 — __GLOBAL__I_a_19
#[doc(alias = "global constructor keyed to_a_19")]
pub fn stub_0x4c498() -> u32 {
    // IDA 0x4c498: `__GLOBAL__I_a_19` — see `GLOBAL_A19_INIT`.
    *GLOBAL_A19_INIT
}

// 0x4c71c — -[GameKeyboard init]
// type: GameKeyboard *__cdecl(GameKeyboard *self, SEL)
#[doc(alias = "-[GameKeyboard init]")]
pub fn stub_0x4c71c() -> GameKeys {
    // IDA 0x4c71c: `GameKeyboard init` chains to super, builds the text
    // field, and registers keyboard notifications; the UIKit glue folds
    // into the host.
    GameKeys::default()
}

// 0x4ca18 — -[GameKeyboard dealloc]
// type: void __cdecl(GameKeyboard *self, SEL)
#[doc(alias = "-[GameKeyboard dealloc]")]
pub fn stub_0x4ca18(keys: &mut GameKeys) {
    // IDA 0x4ca18: `dealloc` releases the text field (0x4ca3a) and chains
    // to super (0x4ca52..); drop glue covers it and the record resets.
    *keys = GameKeys::default();
}

// 0x4ca64 — -[GameKeyboard hideKeyboard]
// type: void __cdecl(GameKeyboard *self, SEL)
#[doc(alias = "-[GameKeyboard hideKeyboard]")]
pub fn stub_0x4ca64(keys: &mut GameKeys) {
    // IDA 0x4ca64: `hideKeyboard` clears the current box (0x4ca9a..) and
    // hides the field; the release glue folds into the host.
    keys.current_box = None;
    keys.shown = false;
}

// 0x4cb80 — -[GameKeyboard keyboardWillHide:]
// type: void __cdecl(GameKeyboard *self, SEL, id)
#[doc(alias = "-[GameKeyboard keyboardWillHide:]")]
pub fn stub_0x4cb80(keys: &mut GameKeys) {
    // IDA 0x4cb80: `keyboardWillHide:` releases the box focus through
    // the text box (0x4cb92..0x4cba2), then hides (0x4cbb8); the service
    // send folds into the host.
    stub_0x4ca64(keys);
}

// 0x4cbbc — -[GameKeyboard keyboardWillChangeFrame:]
// type: void __cdecl(GameKeyboard *self, SEL, id)
#[doc(alias = "-[GameKeyboard keyboardWillChangeFrame:]")]
pub fn stub_0x4cbbc() {
    // IDA 0x4cbbc: `keyboardWillChangeFrame:` — empty body; no-op.
}

// 0x4cbc0 — -[GameKeyboard setDefaultString:]
// type: void __cdecl(GameKeyboard *self, SEL, id)
#[doc(alias = "-[GameKeyboard setDefaultString:]")]
pub fn stub_0x4cbc0(keys: &mut GameKeys, placeholder: &str) {
    // IDA 0x4cbc0: `setDefaultString:` forwards to the field placeholder
    // (0x4cbda).
    keys.placeholder = placeholder.to_string();
}

// 0x4cbe0 — -[GameKeyboard setParentView:]
// type: void __cdecl(GameKeyboard *self, SEL, id)
#[doc(alias = "-[GameKeyboard setParentView:]")]
pub fn stub_0x4cbe0() {
    // IDA 0x4cbe0: `setParentView:` adds self as a subview (0x4cbf2);
    // the view-hierarchy glue folds into the host — no-op.
}

// 0x4cbf8 — -[GameKeyboard showKeyboard:]
// type: bool __cdecl(GameKeyboard *self, SEL, const char *)
#[doc(alias = "-[GameKeyboard showKeyboard:]")]
pub fn stub_0x4cbf8(keys: &mut GameKeys, text: &str) -> bool {
    // IDA 0x4cbf8: `showKeyboard:` when the field is hidden (0x4cc20..)
    // `dispatch_sync`s the show block to main (0x4cc5e..0x4cc6e) and
    // answers true (0x4cc72), else answers false (0x4cc76); the queue
    // hop folds into the caller — see `stub_0x4cc78`.
    if keys.shown {
        false
    } else {
        stub_0x4cc78(keys, text);
        true
    }
}

// 0x4cc78 — ___29-[GameKeyboard showKeyboard:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___29-[GameKeyboard showKeyboard:]_block_invoke")]
pub fn stub_0x4cc78(keys: &mut GameKeys, text: &str) {
    // IDA 0x4cc78: the show block sets the field text (0x4ccb4..0x4ccc8)
    // and seats/shows the field (0x4cce8..).
    keys.text = text.to_string();
    keys.shown = true;
}

// 0x4ce30 — ___copy_helper_block__9
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__9")]
pub fn stub_0x4ce30() {
    // IDA 0x4ce30: `__copy_helper_block__9` retains the captured self;
    // `Arc` glue covers it — no-op.
}

// 0x4ce3c — ___destroy_helper_block__9
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__9")]
pub fn stub_0x4ce3c() {
    // IDA 0x4ce3c: `__destroy_helper_block__9` releases the captured
    // self (pair of 0x4ce30); `Arc` glue covers it — no-op.
}

// 0x4ce44 — -[GameKeyboard showKeyboardWithTextBox:]
// type: bool __cdecl(GameKeyboard *self, SEL, shared_ptr<RBX::TextBox>)
#[doc(alias = "-[GameKeyboard showKeyboardWithTextBox:]")]
pub fn stub_0x4ce44(keys: &mut GameKeys, textbox: Option<u32>, text: &str) -> bool {
    // IDA 0x4ce44: `showKeyboardWithTextBox:` binds the box (0x4ce76..)
    // and shows when it is live (0x4ce90..), else answers false; the box
    // liveness check folds into the host.
    match textbox {
        Some(b) => {
            keys.current_box = Some(b);
            stub_0x4cbf8(keys, text)
        }
        None => false,
    }
}

// 0x4cfbc — -[GameKeyboard getText]
// type: id __cdecl(GameKeyboard *self, SEL)
#[doc(alias = "-[GameKeyboard getText]")]
pub fn stub_0x4cfbc(keys: &GameKeys) -> String {
    // IDA 0x4cfbc: `getText` answers the field text (0x4cfc8..).
    keys.text.clone()
}

// 0x4cfdc — -[GameKeyboard textFieldShouldReturn:]
// type: char __cdecl(GameKeyboard *self, SEL, id)
#[doc(alias = "-[GameKeyboard textFieldShouldReturn:]")]
pub fn stub_0x4cfdc(keys: &mut GameKeys) -> bool {
    // IDA 0x4cfdc: `textFieldShouldReturn:` finishes editing with the
    // field text through the input service (0x4cff6..0x4d02e), then
    // `dispatch_async`s the hide block (0x4d060..0x4d072) and answers
    // true (0x4d07a); the service send and queue hop fold into the
    // caller — see `stub_0x4d07c`.
    keys.submitted += 1;
    stub_0x4d07c(keys);
    true
}

// 0x4d07c — ___38-[GameKeyboard textFieldShouldReturn:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___38-[GameKeyboard textFieldShouldReturn:]_block_invoke")]
pub fn stub_0x4d07c(keys: &mut GameKeys) {
    // IDA 0x4d07c: the return block hides via `hideKeyboard`.
    stub_0x4ca64(keys);
}

// 0x4d090 — ___copy_helper_block_82
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_82")]
pub fn stub_0x4d090() -> ! {
    todo!("0x4d090 ___copy_helper_block_82")
}

// 0x4d09c — ___destroy_helper_block_83
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_83")]
pub fn stub_0x4d09c() -> ! {
    todo!("0x4d09c ___destroy_helper_block_83")
}

// 0x4d0a4 — -[GameKeyboard textFieldDidEndEditing:]
// type: void __cdecl(GameKeyboard *self, SEL, id)
#[doc(alias = "-[GameKeyboard textFieldDidEndEditing:]")]
pub fn stub_0x4d0a4() -> ! {
    todo!("0x4d0a4 -[GameKeyboard textFieldDidEndEditing:]")
}

// 0x4d15c — ___39-[GameKeyboard textFieldDidEndEditing:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___39-[GameKeyboard textFieldDidEndEditing:]_block_invoke")]
pub fn stub_0x4d15c() -> ! {
    todo!("0x4d15c ___39-[GameKeyboard textFieldDidEndEditing:]_block_invoke")
}

// 0x4d170 — ___copy_helper_block_87
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_87")]
pub fn stub_0x4d170() -> ! {
    todo!("0x4d170 ___copy_helper_block_87")
}

// 0x4d17c — ___destroy_helper_block_88
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_88")]
pub fn stub_0x4d17c() -> ! {
    todo!("0x4d17c ___destroy_helper_block_88")
}

// 0x4d184 — -[GameKeyboard .cxx_destruct]
// type: void __cdecl(GameKeyboard *self, SEL)
#[doc(alias = "-[GameKeyboard .cxx_destruct]")]
pub fn stub_0x4d184() -> ! {
    todo!("0x4d184 -[GameKeyboard .cxx_destruct]")
}

// 0x4d220 — -[GameKeyboard .cxx_construct]
// type: id __cdecl(GameKeyboard *self, SEL)
#[doc(alias = "-[GameKeyboard .cxx_construct]")]
pub fn stub_0x4d220() -> ! {
    todo!("0x4d220 -[GameKeyboard .cxx_construct]")
}

// 0x4d398 — __GLOBAL__I_a_20
#[doc(alias = "global constructor keyed to_a_20")]
pub fn stub_0x4d398() -> ! {
    todo!("0x4d398 global constructor keyed to_a_20")
}

// 0x4d5ac — -[GameView initWithFrame:]
// type: GameView *__cdecl(GameView *self, SEL, CGRect)
#[doc(alias = "-[GameView initWithFrame:]")]
pub fn stub_0x4d5ac() -> ! {
    todo!("0x4d5ac -[GameView initWithFrame:]")
}

// 0x4d5e4 — -[GameView layoutSubviews]
// type: void __cdecl(GameView *self, SEL)
#[doc(alias = "-[GameView layoutSubviews]")]
pub fn stub_0x4d5e4() -> ! {
    todo!("0x4d5e4 -[GameView layoutSubviews]")
}

// 0x4d6d4 — __GLOBAL__I_a_21
// type: int()
#[doc(alias = "global constructor keyed to_a_21")]
pub fn stub_0x4d6d4() -> ! {
    todo!("0x4d6d4 global constructor keyed to_a_21")
}

// 0x4d70c — -[GameViewController initWithNibName:bundle:]
// type: GameViewController *__cdecl(GameViewController *self, SEL, id, id)
#[doc(alias = "-[GameViewController initWithNibName:bundle:]")]
pub fn stub_0x4d70c() -> ! {
    todo!("0x4d70c -[GameViewController initWithNibName:bundle:]")
}

// 0x4d8cc — -[GameViewController dealloc]
// type: void __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController dealloc]")]
pub fn stub_0x4d8cc() -> ! {
    todo!("0x4d8cc -[GameViewController dealloc]")
}

// 0x4d978 — -[GameViewController viewWillAppear:]
// type: void __cdecl(GameViewController *self, SEL, char)
#[doc(alias = "-[GameViewController viewWillAppear:]")]
pub fn stub_0x4d978() -> ! {
    todo!("0x4d978 -[GameViewController viewWillAppear:]")
}

// 0x4d9d4 — -[GameViewController viewDidAppear:]
// type: void __cdecl(GameViewController *self, SEL, char)
#[doc(alias = "-[GameViewController viewDidAppear:]")]
pub fn stub_0x4d9d4() -> ! {
    todo!("0x4d9d4 -[GameViewController viewDidAppear:]")
}

// 0x4da00 — -[GameViewController viewDidLoad]
// type: void __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController viewDidLoad]")]
pub fn stub_0x4da00() -> ! {
    todo!("0x4da00 -[GameViewController viewDidLoad]")
}

// 0x4dab8 — -[GameViewController didReceiveMemoryWarning]
// type: void __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController didReceiveMemoryWarning]")]
pub fn stub_0x4dab8() -> ! {
    todo!("0x4dab8 -[GameViewController didReceiveMemoryWarning]")
}

// 0x4dae4 — -[GameViewController resizeGameView]
// type: void __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController resizeGameView]")]
pub fn stub_0x4dae4() -> ! {
    todo!("0x4dae4 -[GameViewController resizeGameView]")
}

// 0x4db04 — -[GameViewController shouldAutorotate]
// type: char __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController shouldAutorotate]")]
pub fn stub_0x4db04() -> ! {
    todo!("0x4db04 -[GameViewController shouldAutorotate]")
}

// 0x4db08 — -[GameViewController supportedInterfaceOrientations]
// type: unsigned int __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController supportedInterfaceOrientations]")]
pub fn stub_0x4db08() -> ! {
    todo!("0x4db08 -[GameViewController supportedInterfaceOrientations]")
}

// 0x4db0c — -[GameViewController shouldAutorotateToInterfaceOrientation:]
// type: char __cdecl(GameViewController *self, SEL, int)
#[doc(alias = "-[GameViewController shouldAutorotateToInterfaceOrientation:]")]
pub fn stub_0x4db0c() -> ! {
    todo!("0x4db0c -[GameViewController shouldAutorotateToInterfaceOrientation:]")
}

// 0x4db20 — -[GameViewController getControlView]
// type: id __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController getControlView]")]
pub fn stub_0x4db20() -> ! {
    todo!("0x4db20 -[GameViewController getControlView]")
}

// 0x4db9c — -[GameViewController webView:shouldStartLoadWithRequest:navigationType:]
// type: char __cdecl(GameViewController *self, SEL, id, id, int)
#[doc(alias = "-[GameViewController webView:shouldStartLoadWithRequest:navigationType:]")]
pub fn stub_0x4db9c() -> ! {
    todo!("0x4db9c -[GameViewController webView:shouldStartLoadWithRequest:navigationType:]")
}

// 0x4dc08 — -[GameViewController closeUrlWindow:]
// type: void __cdecl(GameViewController *self, SEL, id)
#[doc(alias = "-[GameViewController closeUrlWindow:]")]
pub fn stub_0x4dc08() -> ! {
    todo!("0x4dc08 -[GameViewController closeUrlWindow:]")
}

// 0x4de58 — ___37-[GameViewController closeUrlWindow:]_block_invoke
// type: id __fastcall(_DWORD *)
#[doc(alias = "___37-[GameViewController closeUrlWindow:]_block_invoke")]
pub fn stub_0x4de58() -> ! {
    todo!("0x4de58 ___37-[GameViewController closeUrlWindow:]_block_invoke")
}

// 0x4df1c — ___37-[GameViewController closeUrlWindow:]_block_invoke_2
// type: id __fastcall(int)
#[doc(alias = "___37-[GameViewController closeUrlWindow:]_block_invoke_2")]
pub fn stub_0x4df1c() -> ! {
    todo!("0x4df1c ___37-[GameViewController closeUrlWindow:]_block_invoke_2")
}

// 0x4dfd8 — ___copy_helper_block__10
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__10")]
pub fn stub_0x4dfd8() -> ! {
    todo!("0x4dfd8 ___copy_helper_block__10")
}

// 0x4dfe4 — ___destroy_helper_block__10
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__10")]
pub fn stub_0x4dfe4() -> ! {
    todo!("0x4dfe4 ___destroy_helper_block__10")
}

// 0x4dfec — ___37-[GameViewController closeUrlWindow:]_block_invoke93
// type: id __fastcall(int)
#[doc(alias = "___37-[GameViewController closeUrlWindow:]_block_invoke93")]
pub fn stub_0x4dfec() -> ! {
    todo!("0x4dfec ___37-[GameViewController closeUrlWindow:]_block_invoke93")
}

// 0x4e01c — ___copy_helper_block_94
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_94")]
pub fn stub_0x4e01c() -> ! {
    todo!("0x4e01c ___copy_helper_block_94")
}

// 0x4e028 — ___destroy_helper_block_95
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_95")]
pub fn stub_0x4e028() -> ! {
    todo!("0x4e028 ___destroy_helper_block_95")
}

// 0x4e030 — ___copy_helper_block_100
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_100")]
pub fn stub_0x4e030() -> ! {
    todo!("0x4e030 ___copy_helper_block_100")
}

// 0x4e054 — ___destroy_helper_block_101
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_101")]
pub fn stub_0x4e054() -> ! {
    todo!("0x4e054 ___destroy_helper_block_101")
}

// 0x4e070 — -[GameViewController closeUrlWindow]
// type: void __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController closeUrlWindow]")]
pub fn stub_0x4e070() -> ! {
    todo!("0x4e070 -[GameViewController closeUrlWindow]")
}

// 0x4e084 — -[GameViewController openUrlWindow:]
// type: void __cdecl(GameViewController *self, SEL, basic_string<char, std::char_traits<char>, std::allocator<char> >)
#[doc(alias = "-[GameViewController openUrlWindow:]")]
pub fn stub_0x4e084() -> ! {
    todo!("0x4e084 -[GameViewController openUrlWindow:]")
}

// 0x4e2ac — ___36-[GameViewController openUrlWindow:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___36-[GameViewController openUrlWindow:]_block_invoke")]
pub fn stub_0x4e2ac() -> ! {
    todo!("0x4e2ac ___36-[GameViewController openUrlWindow:]_block_invoke")
}

// 0x4e4c8 — ___copy_helper_block_133
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_133")]
pub fn stub_0x4e4c8() -> ! {
    todo!("0x4e4c8 ___copy_helper_block_133")
}

// 0x4e4d4 — ___destroy_helper_block_134
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_134")]
pub fn stub_0x4e4d4() -> ! {
    todo!("0x4e4d4 ___destroy_helper_block_134")
}

// 0x4e4dc — ___36-[GameViewController openUrlWindow:]_block_invoke136
// type: id __fastcall(int)
#[doc(alias = "___36-[GameViewController openUrlWindow:]_block_invoke136")]
pub fn stub_0x4e4dc() -> ! {
    todo!("0x4e4dc ___36-[GameViewController openUrlWindow:]_block_invoke136")
}

// 0x4e5fc — ___36-[GameViewController openUrlWindow:]_block_invoke_2
// type: id __fastcall(_DWORD *)
#[doc(alias = "___36-[GameViewController openUrlWindow:]_block_invoke_2")]
pub fn stub_0x4e5fc() -> ! {
    todo!("0x4e5fc ___36-[GameViewController openUrlWindow:]_block_invoke_2")
}

// 0x4e6dc — ___copy_helper_block_148
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_148")]
pub fn stub_0x4e6dc() -> ! {
    todo!("0x4e6dc ___copy_helper_block_148")
}

// 0x4e6e8 — ___destroy_helper_block_149
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_149")]
pub fn stub_0x4e6e8() -> ! {
    todo!("0x4e6e8 ___destroy_helper_block_149")
}

// 0x4e6f0 — ___copy_helper_block_153
// type: int __fastcall(int, int)
#[doc(alias = "___copy_helper_block_153")]
pub fn stub_0x4e6f0() -> ! {
    todo!("0x4e6f0 ___copy_helper_block_153")
}

// 0x4e714 — ___destroy_helper_block_154
// type: int __fastcall(int)
#[doc(alias = "___destroy_helper_block_154")]
pub fn stub_0x4e714() -> ! {
    todo!("0x4e714 ___destroy_helper_block_154")
}

// 0x4e730 — -[GameViewController handlePromptLoginSignal]
// type: void __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController handlePromptLoginSignal]")]
pub fn stub_0x4e730() -> ! {
    todo!("0x4e730 -[GameViewController handlePromptLoginSignal]")
}

// 0x4e780 — ___45-[GameViewController handlePromptLoginSignal]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___45-[GameViewController handlePromptLoginSignal]_block_invoke")]
pub fn stub_0x4e780() -> ! {
    todo!("0x4e780 ___45-[GameViewController handlePromptLoginSignal]_block_invoke")
}

// 0x4e854 — ___copy_helper_block_174
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_174")]
pub fn stub_0x4e854() -> ! {
    todo!("0x4e854 ___copy_helper_block_174")
}

// 0x4e860 — ___destroy_helper_block_175
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_175")]
pub fn stub_0x4e860() -> ! {
    todo!("0x4e860 ___destroy_helper_block_175")
}

// 0x4e868 — -[GameViewController handlePromptSignupSignal]
// type: void __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController handlePromptSignupSignal]")]
pub fn stub_0x4e868() -> ! {
    todo!("0x4e868 -[GameViewController handlePromptSignupSignal]")
}

#[cfg(test)]
mod control_batch_tests {
    use super::*;

    #[test]
    fn functor_lifecycle() {
        let mut slot = BindSlot::default();
        stub_0x4642c(&mut slot);
        assert_eq!(slot.calls, 0);
        stub_0x463cc(&mut slot, 0);
        assert!(slot.bound);
        stub_0x4642c(&mut slot);
        stub_0x4642c(&mut slot);
        assert_eq!(slot.calls, 2);
        stub_0x463cc(&mut slot, 2);
        assert!(slot.bound);
        stub_0x463cc(&mut slot, 1);
        assert!(!slot.bound);
        stub_0x4642c(&mut slot);
        assert_eq!(slot.calls, 2);
        assert_eq!(stub_0x46490(), 1);
        assert_eq!(stub_0x46f64(), 1);
        assert_eq!(stub_0x47424(), 1);
    }

    #[test]
    fn char_move() {
        let mut move_ = stub_0x466cc([0.0, 0.0, 120.0, 120.0]);
        assert_eq!(move_.frame, [0.0, 0.0, 120.0, 120.0]);
        assert!(!move_.connected);
        stub_0x46704(&mut move_);
        assert!(move_.connected);
        stub_0x467e8();
        stub_0x467ec(&mut move_, false);
        assert!(!move_.moving);
        stub_0x469e8(&mut move_, 0.5, -0.25);
        assert!(move_.moving);
        assert_eq!(move_.move_vec, [0.5, -0.25]);
        stub_0x467ec(&mut move_, false);
        assert!(move_.moving);
        stub_0x467ec(&mut move_, true);
        assert!(!move_.moving);
        assert_eq!(move_.move_vec, [0.0, 0.0]);
        stub_0x469e8(&mut move_, 1.0, 0.0);
        stub_0x468bc(&mut move_, true);
        assert!(!move_.moving);
        stub_0x4698c(&mut move_);
        assert_eq!(move_.move_vec, [0.0, 0.0]);
    }

    #[test]
    fn components() {
        let comp = stub_0x47178();
        assert!(comp.interaction);
        assert_eq!(stub_0x471c0(&comp), None);
        assert_eq!(stub_0x47274(&comp), None);
        let wired = CtrlComp { interaction: true, view: Some(3), game: Some(9) };
        assert_eq!(stub_0x471c0(&wired), Some(3));
        assert_eq!(stub_0x47274(&wired), Some(9));
        let noview = CtrlComp { interaction: true, view: None, game: Some(9) };
        assert_eq!(stub_0x47274(&noview), None);
    }

    #[test]
    fn control_view() {
        let mut view = stub_0x47638([0.0; 4], Some(5));
        assert_eq!(view.game, Some(5));
        assert!(view.controls_visible);
        assert!(!view.events_up);
        stub_0x47afc(&mut view, Some(7));
        assert!(view.events_up);
        stub_0x47afc(&mut view, None);
        assert!(!view.events_up);
        stub_0x47afc(&mut view, Some(7));
        stub_0x47aec(&mut view);
        assert!(!view.events_up);
        stub_0x47b38(&mut view, false);
        assert!(!view.controls_visible);
        stub_0x47b90(&mut view, true);
        assert!(view.controls_visible);
        stub_0x47c18(&mut view);
        assert!(view.controls_visible);
        stub_0x479f8(&mut view, Some(11));
        assert_eq!(view.game, Some(11));
        stub_0x47c04();
        stub_0x47c10();
        stub_0x47904(&mut view);
        assert_eq!(view, CtrlView::default());
    }
}

#[cfg(test)]
mod control_view_event_tests {
    use super::*;

    #[test]
    fn events_and_binding() {
        let mut view = stub_0x47638([0.0; 4], Some(5));
        assert_eq!(stub_0x47ea4(&view), Some(5));
        stub_0x47d78();
        assert!(!view.events_up);
        stub_0x47f48(&mut view);
        assert!(view.events_up);
        stub_0x4818c(&mut view);
        assert!(!view.events_up);
        stub_0x47afc(&mut view, Some(1));
        stub_0x47aec(&mut view);
        assert!(!view.events_up);
        assert!(!view.input_bound);
        stub_0x481cc(&mut view);
        assert!(view.input_bound);
        view.input_bound = false;
        stub_0x48604(&mut view);
        assert!(view.input_bound);
        stub_0x48918();
        assert!(!view.controls_built);
        stub_0x48a50(&mut view);
        assert!(view.controls_built);
        assert!(!view.menu_built);
        stub_0x49018(&mut view);
        assert!(view.menu_built);
        stub_0x47d7c(&mut view, 21);
        assert_eq!(view.focused_textbox, Some(21));
        stub_0x479f8(&mut view, None);
        assert!(!stub_0x487d4(&view, Some("TouchEnabled")));
        stub_0x479f8(&mut view, Some(5));
        assert!(stub_0x487d4(&view, Some("TouchEnabled")));
        assert!(!stub_0x487d4(&view, Some("Parent")));
        assert!(!stub_0x487d4(&view, None));
    }

    #[test]
    fn visibility_flow() {
        let mut view = stub_0x47638([0.0; 4], None);
        stub_0x47c2c(&mut view);
        assert!(!view.controls_visible);
        stub_0x48fe8(&mut view);
        assert!(view.controls_visible);
    }

    #[test]
    fn tap_tracking() {
        let mut view = stub_0x47638([0.0; 4], None);
        stub_0x49314(&mut view, &[7, 8]);
        assert_eq!(view.tap_touch, None);
        stub_0x49314(&mut view, &[7]);
        assert_eq!(view.tap_touch, Some(7));
        stub_0x49314(&mut view, &[9]);
        assert_eq!(view.tap_touch, Some(7));
        assert_eq!(stub_0x4908c(&view, &[7, 9]), Some(7));
        assert_eq!(stub_0x4908c(&view, &[9]), None);
        stub_0x49684(&mut view, false);
        assert_eq!(view.tap_touch, Some(7));
        stub_0x497d0(&mut view, true);
        assert_eq!(view.tap_touch, None);
        stub_0x49314(&mut view, &[7]);
        stub_0x49920(&mut view, Some(8));
        assert_eq!(view.tap_touch, Some(7));
        stub_0x49920(&mut view, Some(7));
        assert_eq!(view.tap_touch, None);
        stub_0x49314(&mut view, &[7]);
        stub_0x47d48(&mut view, Some(8), true);
        assert_eq!(view.tap_touch, Some(7));
        stub_0x47d48(&mut view, Some(7), true);
        assert_eq!(view.tap_touch, None);
        stub_0x49314(&mut view, &[7]);
        stub_0x48ff8(&mut view, None);
        assert_eq!(view.tap_touch, None);
        stub_0x49314(&mut view, &[7]);
        stub_0x4951c(&mut view, Some(7));
        assert_eq!(view.tap_touch, None);
        assert_eq!(view.pinch_time, -1.0);
    }

    #[test]
    fn pinch_and_mouse() {
        let mut view = stub_0x47638([0.0; 4], None);
        stub_0x49314(&mut view, &[3]);
        stub_0x499e0(&mut view, true, 1.5);
        assert_eq!(view.pinch_scale, 1.5);
        assert_eq!(view.tap_touch, None);
        stub_0x499e0(&mut view, false, 2.0);
        assert_eq!(view.pinch_scale, 2.0);
        assert_eq!(view.mouse_events, 0);
        stub_0x47c40(&mut view);
        stub_0x4918c(&mut view);
        stub_0x49acc(&mut view);
        assert_eq!(view.mouse_events, 3);
    }
}

#[cfg(test)]
mod datamodel_signal_batch_tests {
    use super::*;
    use crate::generated_112::{SlotConn, SlotFn, SlotList};

    #[test]
    fn view_teardown() {
        let view = stub_0x47638([1.0, 2.0, 3.0, 4.0], Some(5));
        assert!(stub_0x49bb4(&view, false, false));
        assert!(stub_0x49bb4(&view, true, true));
        assert!(!stub_0x49bb4(&view, true, false));
        assert_eq!(stub_0x49e18(), CtrlView::default());
        let mut live = view.clone();
        live.events_up = true;
        live.input_bound = true;
        stub_0x49ca0(&mut live);
        assert!(!live.events_up);
        assert_eq!(live.game, None);
        assert!(!live.input_bound);
        assert_eq!(live.frame, [1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn textbox_bind() {
        let mut slot = BindSlot::default();
        stub_0x4b010(&mut slot, 0);
        assert!(slot.bound);
        stub_0x4b070(&mut slot);
        stub_0x4b088(&mut slot);
        assert_eq!(slot.calls, 2);
        stub_0x4b010(&mut slot, 1);
        assert!(!slot.bound);
    }

    #[test]
    fn datamodel_signal() {
        let mut list = SlotList::default();
        assert_eq!(stub_0x49e7c(&mut list), 1);
        stub_0x4b164(&mut list);
        assert_eq!(list.slots, 2);
        stub_0x4ba50(&mut list);
        assert_eq!(list, SlotList { slots: 1, removed: 1 });
        let mut slot = SlotConn { connected: true };
        assert!(stub_0x4b970(&slot));
        stub_0x4b860(&mut slot);
        assert!(!stub_0x4b970(&slot));
        let mut f = SlotFn { armed: true, calls: 0 };
        stub_0x4b97c(&mut f);
        stub_0x4b984(&mut f);
        assert_eq!(f.calls, 2);
        assert_eq!(stub_0x4b4bc(), 1);
        assert_eq!(stub_0x4b4c0(), 1);
        assert_eq!(stub_0x4bb40(), 1);
        assert_eq!(stub_0x4bb44(), 1);
        stub_0x4b374();
        stub_0x4b418();
        stub_0x4b5b8();
        stub_0x4b6b4();
        stub_0x4b788();
        stub_0x4bc34();
        stub_0x4bd08();
    }

    #[test]
    #[should_panic(expected = "bad_function_call")]
    fn datamodel_empty_throws() {
        stub_0x4b98c(&mut SlotFn::default());
    }
}

#[cfg(test)]
mod keyboard_batch_tests {
    use super::*;
    use crate::generated_112::SlotFn;

    #[test]
    fn datamodel_tail() {
        let mut dst = SlotFn::default();
        stub_0x4bf3c(&mut dst, &SlotFn { armed: true, calls: 3 });
        assert_eq!(dst.calls, 3);
        let mut slot = BindSlot::default();
        stub_0x4bf6c(&mut slot, 0);
        stub_0x4bfcc(&mut slot);
        assert_eq!(slot.calls, 1);
        stub_0x4bde0();
        stub_0x4be8c();
        assert_eq!(stub_0x4c034(), 1);
        assert_eq!(stub_0x4c498(), 1);
    }

    #[test]
    fn input_vc() {
        let mut vc = GameInputVC::default();
        stub_0x4c440(&mut vc);
        assert!(vc.loaded);
        stub_0x4c46c(&mut vc);
        assert!(!vc.loaded);
        stub_0x4c440(&mut vc);
        stub_0x4c3f4(&mut vc);
        assert_eq!(vc, GameInputVC::default());
    }

    #[test]
    fn keyboard_flow() {
        let mut keys = stub_0x4c71c();
        assert!(!keys.shown);
        stub_0x4cbc0(&mut keys, "Search");
        assert_eq!(keys.placeholder, "Search");
        stub_0x4cbe0();
        stub_0x4cbbc();
        assert_eq!(stub_0x4cfbc(&keys), "");
        assert!(stub_0x4cbf8(&mut keys, "hi"));
        assert!(keys.shown);
        assert_eq!(stub_0x4cfbc(&keys), "hi");
        assert!(!stub_0x4cbf8(&mut keys, "again"));
        assert_eq!(stub_0x4cfbc(&keys), "hi");
        assert!(stub_0x4cfdc(&mut keys));
        assert_eq!(keys.submitted, 1);
        assert!(!keys.shown);
        assert!(!stub_0x4ce44(&mut keys, None, "x"));
        assert!(stub_0x4ce44(&mut keys, Some(4), "yo"));
        assert_eq!(keys.current_box, Some(4));
        assert_eq!(stub_0x4cfbc(&keys), "yo");
        stub_0x4cb80(&mut keys);
        assert!(!keys.shown);
        assert_eq!(keys.current_box, None);
        stub_0x4cc78(&mut keys, "back");
        assert!(keys.shown);
        stub_0x4ce30();
        stub_0x4ce3c();
        stub_0x4ca18(&mut keys);
        assert_eq!(keys, GameKeys::default());
    }
}
