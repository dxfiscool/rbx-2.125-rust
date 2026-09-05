//! Auto-generated skeletons for rbx-network — global EA-sorted filler (RakNet|Network|Replicat|Socket filtered exhausted)
//! Filter: RakNet|Network|Replicat|Socket -> 5198 funcs (cs), 5282 (ci), 0 remaining before batch; filler global ascending
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +120 stubs | range 0x46464..0x4c3f4 | existing 18009 -> 18129 total (filler global ascending EA-sorted, rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

/// Static-init state for `__GLOBAL__I_a_18` (IDA 0x4c034).
#[derive(Clone, Debug, Default)]
pub struct GlobalInitA18 {
 pub done: bool,
}

/// `signal<void(DataModel*)>` slot (IDA 0x49e7c et al.).
#[derive(Clone, Debug, Default)]
pub struct DataModelSlot {
 pub id: u64,
 pub target: usize,
 pub live: bool,
}

/// `signal<void(shared_ptr<TextBox>)>` slot (IDA 0x49f64 et al.).
#[derive(Clone, Debug, Default)]
pub struct TextBoxSlot {
 pub id: u64,
 pub target: usize,
 pub live: bool,
}

/// `signal<void(desc)>` function1 slot (IDA 0x4a04c et al.).
#[derive(Clone, Debug, Default)]
pub struct DescFnSlot {
 pub id: u64,
 pub target: usize,
 pub live: bool,
}

/// ControlView connection state (IDA 0x49e18).
#[derive(Clone, Debug, Default)]
pub struct ControlViewState {
 pub frame: (i32, i32),
 pub tap_begin: (f32, f32),
 pub game: Option<usize>,
 pub connected: bool,
}

/// `signal<void(const PropertyDescriptor*)>` objc slot (IDA 0x46c18 et al.).
#[derive(Clone, Debug, Default)]
pub struct DescObjcSlot {
 pub id: u64,
 pub target: usize,
 pub live: bool,
}

/// Static-init states for `__GLOBAL__I_a_15/16/17` (IDA 0x46490/0x46f64/0x47424).
#[derive(Clone, Debug, Default)]
pub struct GlobalInitA15 {
 pub done: bool,
}
#[derive(Clone, Debug, Default)]
pub struct GlobalInitA16 {
 pub done: bool,
}
#[derive(Clone, Debug, Default)]
pub struct GlobalInitA17 {
 pub done: bool,
}

// 0x46464 — __ZN5boost9function3IvbPvN3RBX7UIEventEE5clearEv
// demangled: boost::function3<void,bool,void *,RBX::UIEvent>::clear(void)
// type: int(void)
#[doc(alias = "boost::function3<void,bool,void *,RBX::UIEvent>::clear(void)")]
pub fn stub_46464(slot: &mut usize, destroy: &mut dyn FnMut()) -> i32 {
    // IDA 0x46464: function3::clear — heap destroy unless small-bit; clear; 0.
    let v = *slot;
    if v != 0 {
        if v & 1 == 0 {
            destroy();
        }
        *slot = 0;
    }
    0
}

// 0x46490 — __GLOBAL__I_a_15
// demangled: global constructor keyed to_a_15
#[doc(alias = "global constructor keyed to_a_15")]
pub fn stub_46490(state: &mut GlobalInitA15, flag: &str, init: &mut dyn FnMut(&str)) {
    // IDA 0x46490: boost cats + ios Init + FFlag::NewCameraControls registration.
    if !state.done {
        init(flag);
        state.done = true;
    }
}

// 0x466cc — -[CharacterMove init:]
// type: id __cdecl(CharacterMove *self, SEL, CGRect)
#[doc(alias = "-[CharacterMove init:]")]
pub fn stub_466cc(view: usize, init_super: &mut dyn FnMut(usize) -> usize) -> usize {
    // IDA 0x466cc: super ThumbStickControl init with frame.
    init_super(view)
}

// 0x46704 — -[CharacterMove setupCharacterMoveConnection]
// type: void __cdecl(CharacterMove *self, SEL)
#[doc(alias = "-[CharacterMove setupCharacterMoveConnection]")]
pub fn stub_46704(connect: &mut dyn FnMut()) {
    // IDA 0x46704: setupCharacterMoveConnection (below truncation).
    connect();
}

// 0x467e8 — -[CharacterMove localCharacterMovementEnabledChange:]
// type: void __cdecl(CharacterMove *self, SEL, const PropertyDescriptor *)
#[doc(alias = "-[CharacterMove localCharacterMovementEnabledChange:]")]
pub fn stub_467e8() {
    // IDA 0x467e8: empty localCharacterMovementEnabledChange body.
}

// 0x467ec — -[CharacterMove touchesEnded:withEvent:]
// type: void __cdecl(CharacterMove *self, SEL, id, id)
#[doc(alias = "-[CharacterMove touchesEnded:withEvent:]")]
pub fn stub_467ec(ended_ours: bool, cancel: &mut dyn FnMut(), forward: &mut dyn FnMut()) {
    // IDA 0x467ec: touchesEnded — cancel movement when ours ends; forward (below truncation).
    if ended_ours {
        cancel();
    }
    forward();
}

// 0x468bc — -[CharacterMove touchesCancelled:withEvent:]
// type: void __cdecl(CharacterMove *self, SEL, id, id)
#[doc(alias = "-[CharacterMove touchesCancelled:withEvent:]")]
pub fn stub_468bc(cancelled_ours: bool, cancel: &mut dyn FnMut(), forward: &mut dyn FnMut()) {
    // IDA 0x468bc: touchesCancelled — cancel movement when ours cancels; forward (below truncation).
    if cancelled_ours {
        cancel();
    }
    forward();
}

// 0x4698c — -[CharacterMove cancelMovement]
// type: void __cdecl(CharacterMove *self, SEL)
#[doc(alias = "-[CharacterMove cancelMovement]")]
pub fn stub_4698c(super_cancel: &mut dyn FnMut(), zero_move: &mut dyn FnMut()) {
    // IDA 0x4698c: super cancelMovement + zero the move vector (below truncation).
    super_cancel();
    zero_move();
}

// 0x469e8 — -[CharacterMove touchesMoved:withEvent:]
// type: void __cdecl(CharacterMove *self, SEL, id, id)
#[doc(alias = "-[CharacterMove touchesMoved:withEvent:]")]
pub fn stub_469e8(track: &mut dyn FnMut()) {
    // IDA 0x469e8: CharacterMove touchesMoved — thumbstick tracking (below truncation).
    track();
}

// 0x46c18 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE7connectIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP13CharacterMoveEENSL_ISF_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_
// demangled: rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>> const&)
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>> const&)")]
pub fn stub_46c18(slots: &mut Vec<DescObjcSlot>, target: usize) -> u64 {
    // IDA 0x46c18: operator new islot; callable ctor; signal connect (below truncation).
    let id = slots.len() as u64;
    slots.push(DescObjcSlot { id, target, live: true });
    id
}

// 0x46c8c — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP13CharacterMoveEENSL_ISF_EENSA_3argILi1EEEEEEEED1Ev
// demangled: rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot()
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_46c8c(slots: &mut Vec<DescObjcSlot>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x46c8c: D1: vtable resets; intrusive release (no delete).
    if let Some(s) = slots.iter_mut().find(|s| s.id == id) {
        s.live = false;
        release(s.id);
    }
}

// 0x46d38 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP13CharacterMoveEENSL_ISF_EENSA_3argILi1EEEEEEEED0Ev
// demangled: rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot()
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_46d38(slots: &mut Vec<DescObjcSlot>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x46d38: D0: vtable resets; intrusive release; operator delete.
    if let Some(pos) = slots.iter().position(|s| s.id == id) {
        let s = slots.remove(pos);
        release(s.id);
    }
}

// 0x46de8 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP13CharacterMoveEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// demangled: rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)
// type: int __fastcall(int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_46de8(target: usize, sel: usize, desc: usize, invoke: &mut dyn FnMut(usize, usize, usize)) {
    // IDA 0x46de8: callable::call forwards objc msgSend(target, sel, desc).
    invoke(target, sel, desc);
}

// 0x46df8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP13CharacterMoveEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// demangled: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)
// type: int __fastcall(int, int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_46df8(target: usize, sel: usize, desc: usize, invoke: &mut dyn FnMut(usize, usize, usize)) {
    // IDA 0x46df8: non-virtual thunk adjusts inward then tail-calls the operator().
    invoke(target, sel, desc);
}

// 0x46e08 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP13CharacterMoveEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_ED1Ev
// demangled: rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
pub fn stub_46e08(slots: &mut Vec<DescObjcSlot>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x46e08: D1: vtable resets; intrusive release (no delete).
    if let Some(s) = slots.iter_mut().find(|s| s.id == id) {
        s.live = false;
        release(s.id);
    }
}

// 0x46eb4 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP13CharacterMoveEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_ED0Ev
// demangled: rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
pub fn stub_46eb4(slots: &mut Vec<DescObjcSlot>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x46eb4: D0: vtable resets; intrusive release; operator delete.
    if let Some(pos) = slots.iter().position(|s| s.id == id) {
        let s = slots.remove(pos);
        release(s.id);
    }
}

// 0x46f64 — __GLOBAL__I_a_16
// demangled: global constructor keyed to_a_16
#[doc(alias = "global constructor keyed to_a_16")]
pub fn stub_46f64(state: &mut GlobalInitA16, init: &mut dyn FnMut()) {
    // IDA 0x46f64: boost error categories + ios_base::Init + bad_alloc static exception object.
    if !state.done {
        init();
        state.done = true;
    }
}

// 0x47178 — -[ControlComponent init]
// type: ControlComponent *__cdecl(ControlComponent *self, SEL)
#[doc(alias = "-[ControlComponent init]")]
pub fn stub_47178(ok: bool, enable: &mut dyn FnMut()) -> bool {
    // IDA 0x47178: super init; userInteractionEnabled = YES (below truncation).
    if !ok {
        return false;
    }
    enable();
    true
}

// 0x471c0 — -[ControlComponent findControlView]
// type: id __cdecl(ControlComponent *self, SEL)
#[doc(alias = "-[ControlComponent findControlView]")]
pub fn stub_471c0(is_control_view: bool, view: usize, find: &mut dyn FnMut(usize) -> usize) -> usize {
    // IDA 0x471c0: self when ControlView else walk superviews.
    if is_control_view {
        view
    } else {
        find(view)
    }
}

// 0x47274 — -[ControlComponent getGameFromControlView]
// type: Game *__cdecl(ControlComponent *self, SEL)
#[doc(alias = "-[ControlComponent getGameFromControlView]")]
pub fn stub_47274(game: usize) -> usize {
    // IDA 0x47274: getGameFromControlView (below truncation).
    game
}

// 0x47338 — -[ControlComponent getUserInputServiceForGameDataModel]
// type: UserInputService *__cdecl(ControlComponent *self, SEL)
#[doc(alias = "-[ControlComponent getUserInputServiceForGameDataModel]")]
pub fn stub_47338(service: usize) -> usize {
    // IDA 0x47338: getUserInputServiceForGameDataModel (below truncation).
    service
}

// 0x47424 — __GLOBAL__I_a_17
// demangled: global constructor keyed to_a_17
#[doc(alias = "global constructor keyed to_a_17")]
pub fn stub_47424(state: &mut GlobalInitA17, init: &mut dyn FnMut()) {
    // IDA 0x47424: boost error categories + ios_base::Init + bad_alloc static exception object.
    if !state.done {
        init();
        state.done = true;
    }
}

// 0x47638 — -[ControlView init:withGame:]
// type: id __cdecl(ControlView *self, SEL, CGRect, shared_ptr<RBX::Game>)
#[doc(alias = "-[ControlView init:withGame:]")]
pub fn stub_47638(view: usize, game: usize, init: &mut dyn FnMut(usize, usize)) -> usize {
    // IDA 0x47638: ControlView init with game + gesture setup (below truncation).
    init(view, game);
    view
}

// 0x47904 — -[ControlView dealloc]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView dealloc]")]
pub fn stub_47904(teardown: &mut dyn FnMut()) {
    // IDA 0x47904: removeObserver + release buttons/controls; super dealloc (below truncation).
    teardown();
}

// 0x479f8 — -[ControlView setGame:]
// type: void __cdecl(ControlView *self, SEL, shared_ptr<RBX::Game>)
#[doc(alias = "-[ControlView setGame:]")]
pub fn stub_479f8(slot: &mut Option<usize>, game: Option<usize>, retain: &mut dyn FnMut(usize), release: &mut dyn FnMut(usize)) {
    // IDA 0x479f8: setGame — retain new; release old (below truncation).
    if let Some(g) = game {
        retain(g);
    }
    let old = std::mem::replace(slot, game);
    if let Some(o) = old {
        release(o);
    }
}

// 0x47aec — -[ControlView gotStartLeaveGameNotification:]
// type: void __cdecl(ControlView *self, SEL, id)
#[doc(alias = "-[ControlView gotStartLeaveGameNotification:]")]
pub fn stub_47aec(disconnect: &mut dyn FnMut()) {
    // IDA 0x47aec: gotStartLeaveGameNotification -> disconnectEvents.
    disconnect();
}

// 0x47afc — -[ControlView dataModelChanged:]
// type: void __cdecl(ControlView *self, SEL, DataModel *)
#[doc(alias = "-[ControlView dataModelChanged:]")]
pub fn stub_47afc(has_model: bool, setup: &mut dyn FnMut(), setup_input: &mut dyn FnMut(), disconnect: &mut dyn FnMut()) {
    // IDA 0x47afc: model ? setupEvents + setupInputControls : disconnectEvents.
    if has_model {
        setup();
        setup_input();
    } else {
        disconnect();
    }
}

// 0x47b38 — -[ControlView setControlVisibility:]
// type: void __cdecl(ControlView *self, SEL, char)
#[doc(alias = "-[ControlView setControlVisibility:]")]
pub fn stub_47b38(visible: bool, dispatch: &mut dyn FnMut(bool)) {
    // IDA 0x47b38: dispatch_async(main, visibility block).
    dispatch(visible);
}

// 0x47b90 — ___36-[ControlView setControlVisibility:]_block_invoke
#[doc(alias = "___36-[ControlView setControlVisibility:]_block_invoke")]
pub fn stub_47b90(visible: bool, apply: &mut dyn FnMut(bool)) {
    // IDA 0x47b90: visibility block — setHidden on control views.
    apply(visible);
}

// 0x47c04 — ___copy_helper_block__8
#[doc(alias = "___copy_helper_block__8")]
pub fn stub_47c04(dst20: &mut usize, src20: usize, retain: &mut dyn FnMut(usize) -> usize) {
    // IDA 0x47c04: _Block_object_assign(dst+20, src+20, 3).
    *dst20 = retain(src20);
}

// 0x47c10 — ___destroy_helper_block__8
#[doc(alias = "___destroy_helper_block__8")]
pub fn stub_47c10(slot20: &mut usize, release: &mut dyn FnMut(usize)) {
    // IDA 0x47c10: _Block_object_dispose(slot+20, 3).
    release(*slot20);
}

// 0x47c18 — -[ControlView showControls]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView showControls]")]
pub fn stub_47c18(show: &mut dyn FnMut(bool)) {
    // IDA 0x47c18: showControls -> setControlVisibility:YES.
    show(true);
}

// 0x47c2c — -[ControlView hideControls]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView hideControls]")]
pub fn stub_47c2c(show: &mut dyn FnMut(bool)) {
    // IDA 0x47c2c: hideControls -> setControlVisibility:NO.
    show(false);
}

// 0x47c40 — -[ControlView postMouseEventProcessedFromOverlay:inputObject:event:]
// type: void __cdecl(ControlView *self, SEL, bool, void *, UIEvent)
#[doc(alias = "-[ControlView postMouseEventProcessedFromOverlay:inputObject:event:]")]
pub fn stub_47c40(forward: &mut dyn FnMut()) {
    // IDA 0x47c40: postMouseEventProcessedFromOverlay (below truncation).
    forward();
}

// 0x47d48 — -[ControlView postMouseEventProcessed:inputObject:event:]
// type: void __cdecl(ControlView *self, SEL, bool, void *, UIEvent)
#[doc(alias = "-[ControlView postMouseEventProcessed:inputObject:event:]")]
pub fn stub_47d48(has_touch: bool, matches: bool, invalidate: &mut dyn FnMut()) {
    // IDA 0x47d48: tapTouch match and processed -> invalidateTapGesture.
    if has_touch && matches {
        invalidate();
    }
}

// 0x47d78 — -[ControlView setupLocalPlayerConnections]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView setupLocalPlayerConnections]")]
pub fn stub_47d78() {
    // IDA 0x47d78: empty setupLocalPlayerConnections body.
}

// 0x47d7c — -[ControlView textBoxFocusGained:]
// type: void __cdecl(ControlView *self, SEL, shared_ptr<RBX::TextBox>)
#[doc(alias = "-[ControlView textBoxFocusGained:]")]
pub fn stub_47d7c(focus: &mut dyn FnMut()) {
    // IDA 0x47d7c: textBoxFocusGained — keyboard wiring (below truncation).
    focus();
}

// 0x47ea4 — -[ControlView getGame]
// type: shared_ptr<RBX::Game> *__cdecl(shared_ptr<RBX::Game> *__return_ptr __struct_ptr retstr, ControlView *self, SEL)
#[doc(alias = "-[ControlView getGame]")]
pub fn stub_47ea4(game: Option<usize>, retain: &mut dyn FnMut(usize)) -> Option<usize> {
    // IDA 0x47ea4: getGame — copy shared_ptr out.
    if let Some(g) = game {
        retain(g);
    }
    game
}

// 0x47f48 — -[ControlView setupEvents]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView setupEvents]")]
pub fn stub_47f48(setup: &mut dyn FnMut()) {
    // IDA 0x47f48: setupEvents (below truncation).
    setup();
}

// 0x4818c — -[ControlView disconnectEvents]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView disconnectEvents]")]
pub fn stub_4818c(disconnect: &mut dyn FnMut(u8)) {
    // IDA 0x4818c: disconnect gameLoaded/overlay/dm connections.
    disconnect(0);
    disconnect(1);
    disconnect(2);
}

// 0x481cc — -[ControlView bindToUserInputService:]
// type: void __cdecl(ControlView *self, SEL, shared_ptr<RBX::DataModel>)
#[doc(alias = "-[ControlView bindToUserInputService:]")]
pub fn stub_481cc(bind: &mut dyn FnMut()) {
    // IDA 0x481cc: bindToUserInputService (below truncation).
    bind();
}

// 0x48604 — -[ControlView bindUserInputService]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView bindUserInputService]")]
pub fn stub_48604(bind: &mut dyn FnMut()) {
    // IDA 0x48604: bindUserInputService (below truncation).
    bind();
}

// 0x48774 — -[ControlView checkUserInputPropertyChanged:onDataModel:]
// type: char __cdecl(ControlView *self, SEL, const PropertyDescriptor *, shared_ptr<RBX::DataModel>)
#[doc(alias = "-[ControlView checkUserInputPropertyChanged:onDataModel:]")]
pub fn stub_48774(has_model: bool, has_desc: bool, has_service: bool, is_modal_prop: bool, modal_enabled: bool) -> bool {
    // IDA 0x48774: model+desc+service gates; non-modal prop -> false; modal -> getModalEnabled.
    if !has_model || !has_desc || !has_service {
        return false;
    }
    if !is_modal_prop {
        return false;
    }
    modal_enabled
}

// 0x487d4 — -[ControlView isValidUserInputProperty:]
// type: char __cdecl(ControlView *self, SEL, const PropertyDescriptor *)
#[doc(alias = "-[ControlView isValidUserInputProperty:]")]
pub fn stub_487d4(has_game: bool, name: Option<&str>) -> bool {
    // IDA 0x487d4: game set and desc set and name != "Parent".
    match (has_game, name) {
        (true, Some(n)) => n != "Parent",
        _ => false,
    }
}

// 0x4880c — -[ControlView userInputPropertyChangedOnDataModel:]
// type: void __cdecl(ControlView *self, SEL, const PropertyDescriptor *)
#[doc(alias = "-[ControlView userInputPropertyChangedOnDataModel:]")]
pub fn stub_4880c(changed: &mut dyn FnMut()) {
    // IDA 0x4880c: userInputPropertyChangedOnDataModel (below truncation).
    changed();
}

// 0x48918 — -[ControlView userInputPropertyChangedOnOverlay:]
// type: void __cdecl(ControlView *self, SEL, const PropertyDescriptor *)
#[doc(alias = "-[ControlView userInputPropertyChangedOnOverlay:]")]
pub fn stub_48918(changed: &mut dyn FnMut()) {
    // IDA 0x48918: userInputPropertyChangedOnOverlay (below truncation).
    changed();
}

// 0x48a50 — -[ControlView setupInputControls]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView setupInputControls]")]
pub fn stub_48a50(setup: &mut dyn FnMut()) {
    // IDA 0x48a50: setupInputControls (below truncation).
    setup();
}

// 0x48fe8 — -[ControlView gameLoaded]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView gameLoaded]")]
pub fn stub_48fe8(show: &mut dyn FnMut()) {
    // IDA 0x48fe8: gameLoaded -> showControls.
    show();
}

// 0x48ff8 — -[ControlView invalidateTapGesture:]
// type: void __cdecl(ControlView *self, SEL, id)
#[doc(alias = "-[ControlView invalidateTapGesture:]")]
pub fn stub_48ff8(tap: &mut Option<usize>, arg: Option<usize>) {
    // IDA 0x48ff8: nil arg or arg == tapTouch -> clear tapTouch.
    if arg.is_none() || *tap == arg {
        *tap = None;
    }
}

// 0x49018 — -[ControlView createNativeMenu]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView createNativeMenu]")]
pub fn stub_49018(create: &mut dyn FnMut() -> usize, add: &mut dyn FnMut(usize)) {
    // IDA 0x49018: alloc MenuButton init + addSubview.
    let b = create();
    add(b);
}

// 0x4908c — -[ControlView checkTouchesForTap:withEvent:]
// type: id __cdecl(ControlView *self, SEL, id, id)
#[doc(alias = "-[ControlView checkTouchesForTap:withEvent:]")]
pub fn stub_4908c(has_tap: bool, check: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x4908c: tapTouch nil -> 0 else enumerate touches (below truncation).
    if !has_tap {
        return 0;
    }
    check()
}

// 0x4918c — -[ControlView sendMouseEventToGame:withTouch:]
// type: void __cdecl(ControlView *self, SEL, UIEvent, id)
#[doc(alias = "-[ControlView sendMouseEventToGame:withTouch:]")]
pub fn stub_4918c(send: &mut dyn FnMut()) {
    // IDA 0x4918c: sendMouseEventToGame (below truncation).
    send();
}

// 0x49314 — -[ControlView touchesBegan:withEvent:]
// type: void __cdecl(ControlView *self, SEL, id, id)
#[doc(alias = "-[ControlView touchesBegan:withEvent:]")]
pub fn stub_49314(began: &mut dyn FnMut()) {
    // IDA 0x49314: ControlView touchesBegan (below truncation).
    began();
}

// 0x4951c — -[ControlView touchesEnded:withEvent:]
// type: void __cdecl(ControlView *self, SEL, id, id)
#[doc(alias = "-[ControlView touchesEnded:withEvent:]")]
pub fn stub_4951c(pinch: &mut f64, check: &mut dyn FnMut() -> usize, forward: &mut dyn FnMut()) -> usize {
    // IDA 0x4951c: pinchTime = -1; checkTouchesForTap; forward (below truncation).
    *pinch = -1.0;
    let r = check();
    forward();
    r
}

// 0x49684 — -[ControlView touchesMoved:withEvent:]
// type: void __cdecl(ControlView *self, SEL, id, id)
#[doc(alias = "-[ControlView touchesMoved:withEvent:]")]
pub fn stub_49684(moved: &mut dyn FnMut()) {
    // IDA 0x49684: ControlView touchesMoved (below truncation).
    moved();
}

// 0x497d0 — -[ControlView checkTapTouchMove:]
// type: void __cdecl(ControlView *self, SEL, id)
#[doc(alias = "-[ControlView checkTapTouchMove:]")]
pub fn stub_497d0(check: &mut dyn FnMut()) {
    // IDA 0x497d0: checkTapTouchMove (below truncation).
    check();
}

// 0x49920 — -[ControlView touchesCancelled:withEvent:]
// type: void __cdecl(ControlView *self, SEL, id, id)
#[doc(alias = "-[ControlView touchesCancelled:withEvent:]")]
pub fn stub_49920(cancel: &mut dyn FnMut()) {
    // IDA 0x49920: ControlView touchesCancelled (below truncation).
    cancel();
}

// 0x499e0 — -[ControlView twoFingerPinch:]
// type: void __cdecl(ControlView *self, SEL, id)
#[doc(alias = "-[ControlView twoFingerPinch:]")]
pub fn stub_499e0(state_began: bool, scale: f32, handle: &mut dyn FnMut(bool, f32)) {
    // IDA 0x499e0: began ? lastPinchScale = 1; end pan; invalidate tap; zoom (below truncation).
    handle(state_began, scale);
}

// 0x49acc — -[ControlView oneFingerSingleTap]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView oneFingerSingleTap]")]
pub fn stub_49acc(tap: &mut dyn FnMut()) {
    // IDA 0x49acc: oneFingerSingleTap (below truncation).
    tap();
}

// 0x49bb4 — -[ControlView gestureRecognizer:shouldReceiveTouch:]
// type: char __cdecl(ControlView *self, SEL, id, id)
#[doc(alias = "-[ControlView gestureRecognizer:shouldReceiveTouch:]")]
pub fn stub_49bb4(is_pinch: bool, check: &mut dyn FnMut() -> bool) -> bool {
    // IDA 0x49bb4: non-pinch -> YES; pinch -> location checks (below truncation).
    if !is_pinch {
        return true;
    }
    check()
}

// 0x49ca0 — -[ControlView .cxx_destruct]
// type: void __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView .cxx_destruct]")]
pub fn stub_49ca0(teardown: &mut dyn FnMut()) {
    // IDA 0x49ca0: cxx_destruct — disconnect + weak releases (below truncation).
    teardown();
}

// 0x49e18 — -[ControlView .cxx_construct]
// type: id __cdecl(ControlView *self, SEL)
#[doc(alias = "-[ControlView .cxx_construct]")]
pub fn stub_49e18(state: &mut ControlViewState) {
    // IDA 0x49e18: cxx_construct — zero fields/connections.
    *state = ControlViewState::default();
}

// 0x49e7c — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
// demangled: rbx::signals::connection rbx::signals::signal<void ()(RBX::DataModel *)>::connect<boost::function<void ()(RBX::DataModel *)>>(boost::function<void ()(RBX::DataModel *)> const&)
// type: int __fastcall(char, boost::mutex *, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::DataModel *)>::connect<boost::function<void ()(RBX::DataModel *)>>(boost::function<void ()(RBX::DataModel *)> const&)")]
pub fn stub_49e7c(slots: &mut Vec<DataModelSlot>, target: usize) -> u64 {
    // IDA 0x49e7c: operator new islot; callable ctor; signal connect (below truncation).
    let id = slots.len() as u64;
    slots.push(DataModelSlot { id, target, live: true });
    id
}

// 0x49f64 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_
// demangled: rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::connect<boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>>(boost::function<void ()(boost::shared_ptr<RBX::TextBox>)> const&)
// type: int __fastcall(char, boost::mutex *, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)> const&)")]
pub fn stub_49f64(slots: &mut Vec<TextBoxSlot>, target: usize) -> u64 {
    // IDA 0x49f64: operator new islot; callable ctor; signal connect (below truncation).
    let id = slots.len() as u64;
    slots.push(TextBoxSlot { id, target, live: true });
    id
}

// 0x4a04c — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost8functionIS8_EELi1ES8_EC2IPS9_EERKSD_T_
// demangled: rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>*>(boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)> const&,rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>*)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>*>(boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)> const&,rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>*)")]
pub fn stub_4a04c(slot: usize, init: &mut dyn FnMut(usize)) -> usize {
    // IDA 0x4a04c: callable ctor — vtable + functor assign (below truncation).
    init(slot);
    slot
}

// 0x4a148 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost8functionIS8_EELi1ES8_E4callES7_
// demangled: rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_4a148(target: usize, desc: usize, invoke: &mut dyn FnMut(usize, usize)) {
    // IDA 0x4a148: callable::call forwards to function1::operator().
    invoke(target, desc);
}

// 0x4a150 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost8functionIS8_EELi1ES8_E4callES7_
// demangled: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_4a150(target: usize, desc: usize, invoke: &mut dyn FnMut(usize, usize)) {
    // IDA 0x4a150: non-virtual thunk adjusts inward then tail-calls the operator().
    invoke(target, desc);
}

// 0x4a158 — __ZNK5boost9function1IvPKN3RBX10Reflection18PropertyDescriptorEEclES5_
// demangled: boost::function1<void,RBX::Reflection::PropertyDescriptor const*>::operator()(RBX::Reflection::PropertyDescriptor const*)const
// type: int(void)
#[doc(alias = "boost::function1<void,RBX::Reflection::PropertyDescriptor const*>::operator()(RBX::Reflection::PropertyDescriptor const*)const")]
pub fn stub_4a158(has_fn: bool, invoke: &mut dyn FnMut()) {
    // IDA 0x4a158: function1::operator() — empty call throws (below truncation).
    if !has_fn {
        panic!("bad_function_call");
    }
    invoke();
}

// 0x4a21c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKN3RBX10Reflection18PropertyDescriptorEENS3_5list3INS3_5valueIS6_EENSG_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::Reflection::PropertyDescriptor const*),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::Reflection::PropertyDescriptor const*),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::Reflection::PropertyDescriptor const*),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::Reflection::PropertyDescriptor const*),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_4a21c(op: u32, manage: &mut dyn FnMut(u32) -> usize) -> usize {
    // IDA 0x4a21c: functor_manager::manage — clone/move/destroy by op (below truncation).
    manage(op)
}

// 0x4a27c — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKN3RBX10Reflection18PropertyDescriptorEENS3_5list3INS3_5valueIS6_EENSG_IS7_EENS_3argILi1EEEEEEEvSC_E6invokeERNS1_15function_bufferESC_
// demangled: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::Reflection::PropertyDescriptor const*),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::Reflection::PropertyDescriptor const>::invoke(boost::detail::function::function_buffer &,RBX::Reflection::PropertyDescriptor const)
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::Reflection::PropertyDescriptor const*),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::Reflection::PropertyDescriptor const>::invoke(boost::detail::function::function_buffer &,RBX::Reflection::PropertyDescriptor const)")]
pub fn stub_4a27c(obj: usize, sel: usize, desc: usize, invoke: &mut dyn FnMut(usize, usize, usize)) {
    // IDA 0x4a27c: invoker calls fn(obj, sel, desc).
    invoke(obj, sel, desc);
}

// 0x4a28c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE6insertEPNS8_4slotE
// demangled: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::insert(rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot *)
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::insert(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot *)")]
pub fn stub_4a28c(slots: &mut Vec<TextBoxSlot>, target: usize) -> u64 {
    // IDA 0x4a28c: signal<TextBox>::insert — new islot; insert (below truncation).
    let id = slots.len() as u64;
    slots.push(TextBoxSlot { id, target, live: true });
    id
}

// 0x4a49c — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX7TextBoxEEEEE4slotEEaSEPSA_
// demangled: boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot>::operator=(rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot*)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot*)")]
pub fn stub_4a49c(slot: &mut Option<usize>, value: Option<usize>, add_ref: &mut dyn FnMut(usize), release: &mut dyn FnMut(usize)) -> Option<usize> {
    // IDA 0x4a49c: add_ref(new); store; release(old).
    if let Some(v) = value {
        add_ref(v);
    }
    let old = std::mem::replace(slot, value);
    if let Some(o) = old {
        release(o);
    }
    *slot
}

// 0x4a540 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE22safe_static_init_mutexEv
// demangled: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::safe_static_init_mutex(void)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::safe_static_init_mutex(void)")]
pub fn stub_4a540(get: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x4a540: thunk tail-calls safe_static_do_get_mutex.
    get()
}

// 0x4a544 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_EC2IPS9_EERKSC_T_
// demangled: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ()(boost::shared_ptr<RBX::TextBox>)>::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>*>(boost::function<void ()(boost::shared_ptr<RBX::TextBox>)> const&,rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>*)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>*>(boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)> const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>*)")]
pub fn stub_4a544(slot: usize, init: &mut dyn FnMut(usize)) -> usize {
    // IDA 0x4a544: callable ctor — vtable + functor assign (below truncation).
    init(slot);
    slot
}

// 0x4a640 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE13callable_slotINS2_8functionIS7_EEED1Ev
// demangled: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>>::~callable_slot()
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>>::~callable_slot()")]
pub fn stub_4a640(slots: &mut Vec<TextBoxSlot>, id: u64, clear: &mut dyn FnMut(u64), release: &mut dyn FnMut(u64)) {
    // IDA 0x4a640: D1: function clear; vtable resets; intrusive release (no delete).
    if let Some(s) = slots.iter_mut().find(|s| s.id == id) {
        s.live = false;
        clear(s.id);
        release(s.id);
    }
}

// 0x4a714 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE13callable_slotINS2_8functionIS7_EEED0Ev
// demangled: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>>::~callable_slot()
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>>::~callable_slot()")]
pub fn stub_4a714(slots: &mut Vec<TextBoxSlot>, id: u64, clear: &mut dyn FnMut(u64), release: &mut dyn FnMut(u64)) {
    // IDA 0x4a714: D0: function clear; vtable resets; release; operator delete.
    if let Some(pos) = slots.iter().position(|s| s.id == id) {
        let s = slots.remove(pos);
        clear(s.id);
        release(s.id);
    }
}

// 0x4a7ec — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot10disconnectEv
// demangled: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::disconnect(void)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::disconnect(void)")]
pub fn stub_4a7ec(slots: &mut Vec<TextBoxSlot>, id: u64, disconnect: &mut dyn FnMut(u64)) {
    // IDA 0x4a7ec: slot::disconnect (below truncation).
    if let Some(s) = slots.iter_mut().find(|s| s.id == id) {
        s.live = false;
        disconnect(s.id);
    }
}

// 0x4a8fc — __ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot9connectedEv
// demangled: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::connected(void)const
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::connected(void)const")]
pub fn stub_4a8fc(live: bool) -> bool {
    // IDA 0x4a8fc: connected = slot word != 0.
    live
}

// 0x4a908 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_
// demangled: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ()(boost::shared_ptr<RBX::TextBox>)>::call(boost::shared_ptr<RBX::TextBox>)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::call(rbx_core::SharedPtr<RBX::TextBox>)")]
pub fn stub_4a908(target: usize, text_box: usize, invoke: &mut dyn FnMut(usize, usize)) {
    // IDA 0x4a908: callable::call forwards to function1::operator().
    invoke(target, text_box);
}

// 0x4a9dc — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_
// demangled: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ()(boost::shared_ptr<RBX::TextBox>)>::call(boost::shared_ptr<RBX::TextBox>)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::call(rbx_core::SharedPtr<RBX::TextBox>)")]
pub fn stub_4a9dc(target: usize, text_box: usize, invoke: &mut dyn FnMut(usize, usize)) {
    // IDA 0x4a9dc: non-virtual thunk (a1 - 4) tail-calls the operator().
    invoke(target, text_box);
}

// 0x4a9e4 — __ZNK5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEEclES4_
// demangled: boost::function1<void,boost::shared_ptr<RBX::TextBox>>::operator()(boost::shared_ptr<RBX::TextBox>)const
// type: int(void)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>>::operator()(rbx_core::SharedPtr<RBX::TextBox>)const")]
pub fn stub_4a9e4(has_fn: bool, invoke: &mut dyn FnMut()) {
    // IDA 0x4a9e4: function1::operator() — empty call throws (below truncation).
    if !has_fn {
        panic!("bad_function_call");
    }
    invoke();
}

// 0x4aaf4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE6removeEPNS8_4slotE
// demangled: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::remove(rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot *)
// type: int __fastcall(int, char *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::remove(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot *)")]
pub fn stub_4aaf4(slots: &mut Vec<TextBoxSlot>, id: u64, expired: bool, remove: &mut dyn FnMut(u64)) {
    // IDA 0x4aaf4: ReleaseAssert(!expired); remove slot.
    assert!(!expired, "!boost::intrusive_ptr_expired(item)");
    if let Some(pos) = slots.iter().position(|s| s.id == id) {
        let s = slots.remove(pos);
        remove(s.id);
    }
}

// 0x4abe4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot22safe_static_init_mutexEv
// demangled: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::safe_static_init_mutex(void)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::safe_static_init_mutex(void)")]
pub fn stub_4abe4(get: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x4abe4: thunk tail-calls safe_static_do_get_mutex.
    get()
}

// 0x4abe8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot24safe_static_do_get_mutexEv
// demangled: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::safe_static_do_get_mutex(void)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_4abe8(guard: &mut bool, slot: &mut Option<usize>, alloc: &mut dyn FnMut(usize) -> usize, init: &mut dyn FnMut(usize)) -> usize {
    // IDA 0x4abe8: guarded one-time mutex alloc + construct.
    if !*guard {
        let m = alloc(0x2C);
        init(m);
        *slot = Some(m);
        *guard = true;
    }
    slot.unwrap_or(0)
}

// 0x4acd8 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_ED1Ev
// demangled: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ()(boost::shared_ptr<RBX::TextBox>)>::~callable()
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::~callable()")]
pub fn stub_4acd8(slots: &mut Vec<TextBoxSlot>, id: u64, clear: &mut dyn FnMut(u64), release: &mut dyn FnMut(u64)) {
    // IDA 0x4acd8: D1: function clear; vtable resets; intrusive release (no delete).
    if let Some(s) = slots.iter_mut().find(|s| s.id == id) {
        s.live = false;
        clear(s.id);
        release(s.id);
    }
}

// 0x4adac — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_ED0Ev
// demangled: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ()(boost::shared_ptr<RBX::TextBox>)>::~callable()
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::~callable()")]
pub fn stub_4adac(slots: &mut Vec<TextBoxSlot>, id: u64, clear: &mut dyn FnMut(u64), release: &mut dyn FnMut(u64)) {
    // IDA 0x4adac: D0: function clear; vtable resets; release; operator delete.
    if let Some(pos) = slots.iter().position(|s| s.id == id) {
        let s = slots.remove(pos);
        clear(s.id);
        release(s.id);
    }
}

// 0x4ae84 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotD1Ev
// demangled: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::~slot()
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::~slot()")]
pub fn stub_4ae84(slots: &mut Vec<TextBoxSlot>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x4ae84: D1: vtable resets; intrusive release (no delete).
    if let Some(s) = slots.iter_mut().find(|s| s.id == id) {
        s.live = false;
        release(s.id);
    }
}

// 0x4af30 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotD0Ev
// demangled: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::~slot()
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::~slot()")]
pub fn stub_4af30(slots: &mut Vec<TextBoxSlot>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x4af30: D0: vtable resets; intrusive release; operator delete.
    if let Some(pos) = slots.iter().position(|s| s.id == id) {
        let s = slots.remove(pos);
        release(s.id);
    }
}

// 0x4afe0 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEE13assign_to_ownERKS5_
// demangled: boost::function1<void,boost::shared_ptr<RBX::TextBox>>::assign_to_own(boost::function1<void,boost::shared_ptr<RBX::TextBox>> const&)
// type: int(void)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>>::assign_to_own(boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>> const&)")]
pub fn stub_4afe0(dst: usize, has_src: bool, is_small: bool, copy: &mut dyn FnMut(usize, bool)) -> usize {
    // IDA 0x4afe0: function1::assign_to_own — inline small copy else heap clone; return dst.
    if has_src {
        copy(dst, is_small);
    }
    dst
}

// 0x4b010 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX7TextBoxEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_4b010(op: u32, manage: &mut dyn FnMut(u32) -> usize) -> usize {
    // IDA 0x4b010: functor_manager::manage — clone/move/destroy by op (below truncation).
    manage(op)
}

// 0x4b070 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX7TextBoxEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
// demangled: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::TextBox>::invoke(boost::detail::function::function_buffer &,RBX::TextBox)
// type: int __fastcall(int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::TextBox>::invoke(boost::detail::function::function_buffer &,RBX::TextBox)")]
pub fn stub_4b070(obj: usize, sel: usize, text_box: usize, invoke: &mut dyn FnMut(usize, usize, usize)) {
    // IDA 0x4b070: invoker forwards list3::operator() (obj, sel, textbox).
    invoke(obj, sel, text_box);
}

// 0x4b088 — __ZN5boost3_bi5list3INS0_5valueIP11objc_objectEENS2_IP13objc_selectorEENS_3argILi1EEEEclIPFvS4_S6_NS_10shared_ptrIN3RBX7TextBoxEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// demangled: void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::TextBox>),boost::_bi::list1<RBX::TextBox&>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::TextBox>) &,boost::_bi::list1<RBX::TextBox&> &,int)
// type: void __fastcall(int *, void (__fastcall **)(int, int, sp_counted_base **), const shared_count **, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,rbx_core::SharedPtr<RBX::TextBox>),boost::_bi::list1<RBX::TextBox&>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,rbx_core::SharedPtr<RBX::TextBox>) &,boost::_bi::list1<RBX::TextBox&> &,int)")]
pub fn stub_4b088(obj: usize, sel: usize, text_box: usize, invoke: &mut dyn FnMut(usize, usize, usize)) {
    // IDA 0x4b088: list3 dispatch; fn(obj, sel, textbox).
    invoke(obj, sel, text_box);
}

// 0x4b164 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE6insertEPNS6_4slotE
// demangled: rbx::signals::signal<void ()(RBX::DataModel *)>::insert(rbx::signals::signal<void ()(RBX::DataModel *)>::slot *)
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::insert(rbx::signals::signal<void ()(RBX::DataModel *)>::slot *)")]
pub fn stub_4b164(slots: &mut Vec<DataModelSlot>, target: usize) -> u64 {
    // IDA 0x4b164: signal<DataModel>::insert — new islot; insert (below truncation).
    let id = slots.len() as u64;
    slots.push(DataModelSlot { id, target, live: true });
    id
}

// 0x4b374 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotEEaSEPS9_
// demangled: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::DataModel *)>::slot*)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::DataModel *)>::slot*)")]
pub fn stub_4b374(slot: &mut Option<usize>, value: Option<usize>, add_ref: &mut dyn FnMut(usize), release: &mut dyn FnMut(usize)) -> Option<usize> {
    // IDA 0x4b374: add_ref(new); store; release(old).
    if let Some(v) = value {
        add_ref(v);
    }
    let old = std::mem::replace(slot, value);
    if let Some(o) = old {
        release(o);
    }
    *slot
}

// 0x4b418 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotEEaSERKSA_
// demangled: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot> const&)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot> const&)")]
pub fn stub_4b418(slot: &mut Option<usize>, value: Option<usize>, add_ref: &mut dyn FnMut(usize), release: &mut dyn FnMut(usize)) -> Option<usize> {
    // IDA 0x4b418: add_ref(new); store; release(old).
    if let Some(v) = value {
        add_ref(v);
    }
    let old = std::mem::replace(slot, value);
    if let Some(o) = old {
        release(o);
    }
    *slot
}

// 0x4b4bc — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE22safe_static_init_mutexEv
// demangled: rbx::signals::signal<void ()(RBX::DataModel *)>::safe_static_init_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::safe_static_init_mutex(void)")]
pub fn stub_4b4bc(get: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x4b4bc: thunk tail-calls safe_static_do_get_mutex.
    get()
}

// 0x4b4c0 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE24safe_static_do_get_mutexEv
// demangled: rbx::signals::signal<void ()(RBX::DataModel *)>::safe_static_do_get_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::safe_static_do_get_mutex(void)")]
pub fn stub_4b4c0(guard: &mut bool, slot: &mut Option<usize>, alloc: &mut dyn FnMut(usize) -> usize, init: &mut dyn FnMut(usize)) -> usize {
    // IDA 0x4b4c0: guarded one-time mutex alloc + construct.
    if !*guard {
        let m = alloc(0x2C);
        init(m);
        *slot = Some(m);
        *guard = true;
    }
    slot.unwrap_or(0)
}

// 0x4b5b8 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_EC2IPS7_EERKSB_T_
// demangled: rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::callable<rbx::signals::signal<void ()(RBX::DataModel *)>*>(boost::function<void ()(RBX::DataModel *)> const&,rbx::signals::signal<void ()(RBX::DataModel *)>*)
// type: _DWORD *__fastcall(_DWORD *, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::callable<rbx::signals::signal<void ()(RBX::DataModel *)>*>(boost::function<void ()(RBX::DataModel *)> const&,rbx::signals::signal<void ()(RBX::DataModel *)>*)")]
pub fn stub_4b5b8(slot: usize, init: &mut dyn FnMut(usize)) -> usize {
    // IDA 0x4b5b8: callable ctor — vtable + functor assign (below truncation).
    init(slot);
    slot
}

// 0x4b6b4 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE13callable_slotIN5boost8functionIS5_EEED1Ev
// demangled: rbx::signals::signal<void ()(RBX::DataModel *)>::callable_slot<boost::function<void ()(RBX::DataModel *)>>::~callable_slot()
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::callable_slot<boost::function<void ()(RBX::DataModel *)>>::~callable_slot()")]
pub fn stub_4b6b4(slots: &mut Vec<DataModelSlot>, id: u64, clear: &mut dyn FnMut(u64), release: &mut dyn FnMut(u64)) {
    // IDA 0x4b6b4: D1: function clear; vtable resets; intrusive release (no delete).
    if let Some(s) = slots.iter_mut().find(|s| s.id == id) {
        s.live = false;
        clear(s.id);
        release(s.id);
    }
}

// 0x4b788 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE13callable_slotIN5boost8functionIS5_EEED0Ev
// demangled: rbx::signals::signal<void ()(RBX::DataModel *)>::callable_slot<boost::function<void ()(RBX::DataModel *)>>::~callable_slot()
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::callable_slot<boost::function<void ()(RBX::DataModel *)>>::~callable_slot()")]
pub fn stub_4b788(slots: &mut Vec<DataModelSlot>, id: u64, clear: &mut dyn FnMut(u64), release: &mut dyn FnMut(u64)) {
    // IDA 0x4b788: D0: function clear; vtable resets; release; operator delete.
    if let Some(pos) = slots.iter().position(|s| s.id == id) {
        let s = slots.remove(pos);
        clear(s.id);
        release(s.id);
    }
}

// 0x4b860 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot10disconnectEv
// demangled: rbx::signals::signal<void ()(RBX::DataModel *)>::slot::disconnect(void)
// type: void __fastcall(int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::disconnect(void)")]
pub fn stub_4b860(slots: &mut Vec<DataModelSlot>, id: u64, disconnect: &mut dyn FnMut(u64)) {
    // IDA 0x4b860: slot::disconnect (below truncation).
    if let Some(s) = slots.iter_mut().find(|s| s.id == id) {
        s.live = false;
        disconnect(s.id);
    }
}

// 0x4b970 — __ZNK3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot9connectedEv
// demangled: rbx::signals::signal<void ()(RBX::DataModel *)>::slot::connected(void)const
// type: bool __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::connected(void)const")]
pub fn stub_4b970(live: bool) -> bool {
    // IDA 0x4b970: connected = slot word != 0.
    live
}

// 0x4b97c — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_
// demangled: rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::call(RBX::DataModel *)
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::call(RBX::DataModel *)")]
pub fn stub_4b97c(target: usize, model: usize, invoke: &mut dyn FnMut(usize, usize)) {
    // IDA 0x4b97c: callable::call forwards to function1::operator().
    invoke(target, model);
}

// 0x4b984 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_
// demangled: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::call(RBX::DataModel *)
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::call(RBX::DataModel *)")]
pub fn stub_4b984(target: usize, model: usize, invoke: &mut dyn FnMut(usize, usize)) {
    // IDA 0x4b984: non-virtual thunk adjusts inward then tail-calls the operator().
    invoke(target, model);
}

// 0x4b98c — __ZNK5boost9function1IvPN3RBX9DataModelEEclES3_
// demangled: boost::function1<void,RBX::DataModel *>::operator()(RBX::DataModel *)const
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "boost::function1<void,RBX::DataModel *>::operator()(RBX::DataModel *)const")]
pub fn stub_4b98c(has_fn: bool, invoke: &mut dyn FnMut()) {
    // IDA 0x4b98c: function1::operator() — empty call throws (below truncation).
    if !has_fn {
        panic!("bad_function_call");
    }
    invoke();
}

// 0x4ba50 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE6removeEPNS6_4slotE
// demangled: rbx::signals::signal<void ()(RBX::DataModel *)>::remove(rbx::signals::signal<void ()(RBX::DataModel *)>::slot *)
// type: int __fastcall(char **, char *, int, const void *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::remove(rbx::signals::signal<void ()(RBX::DataModel *)>::slot *)")]
pub fn stub_4ba50(slots: &mut Vec<DataModelSlot>, id: u64, expired: bool, remove: &mut dyn FnMut(u64)) {
    // IDA 0x4ba50: ReleaseAssert(!expired); remove slot.
    assert!(!expired, "!boost::intrusive_ptr_expired(item)");
    if let Some(pos) = slots.iter().position(|s| s.id == id) {
        let s = slots.remove(pos);
        remove(s.id);
    }
}

// 0x4bb40 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot22safe_static_init_mutexEv
// demangled: rbx::signals::signal<void ()(RBX::DataModel *)>::slot::safe_static_init_mutex(void)
// type: int()
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::safe_static_init_mutex(void)")]
pub fn stub_4bb40(get: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x4bb40: thunk tail-calls safe_static_do_get_mutex.
    get()
}

// 0x4bb44 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot24safe_static_do_get_mutexEv
// demangled: rbx::signals::signal<void ()(RBX::DataModel *)>::slot::safe_static_do_get_mutex(void)
// type: void *()
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_4bb44(guard: &mut bool, slot: &mut Option<usize>, alloc: &mut dyn FnMut(usize) -> usize, init: &mut dyn FnMut(usize)) -> usize {
    // IDA 0x4bb44: guarded one-time mutex alloc + construct.
    if !*guard {
        let m = alloc(0x2C);
        init(m);
        *slot = Some(m);
        *guard = true;
    }
    slot.unwrap_or(0)
}

// 0x4bc34 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_ED1Ev
// demangled: rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::~callable()
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::~callable()")]
pub fn stub_4bc34(slots: &mut Vec<DataModelSlot>, id: u64, clear: &mut dyn FnMut(u64), release: &mut dyn FnMut(u64)) {
    // IDA 0x4bc34: D1: function clear; vtable resets; intrusive release (no delete).
    if let Some(s) = slots.iter_mut().find(|s| s.id == id) {
        s.live = false;
        clear(s.id);
        release(s.id);
    }
}

// 0x4bd08 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_ED0Ev
// demangled: rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::~callable()
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::~callable()")]
pub fn stub_4bd08(slots: &mut Vec<DataModelSlot>, id: u64, clear: &mut dyn FnMut(u64), release: &mut dyn FnMut(u64)) {
    // IDA 0x4bd08: D0: function clear; vtable resets; release; operator delete.
    if let Some(pos) = slots.iter().position(|s| s.id == id) {
        let s = slots.remove(pos);
        clear(s.id);
        release(s.id);
    }
}

// 0x4bde0 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotD1Ev
// demangled: rbx::signals::signal<void ()(RBX::DataModel *)>::slot::~slot()
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::~slot()")]
pub fn stub_4bde0(slots: &mut Vec<DataModelSlot>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x4bde0: D1: vtable resets; intrusive release (no delete).
    if let Some(s) = slots.iter_mut().find(|s| s.id == id) {
        s.live = false;
        release(s.id);
    }
}

// 0x4be8c — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotD0Ev
// demangled: rbx::signals::signal<void ()(RBX::DataModel *)>::slot::~slot()
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::~slot()")]
pub fn stub_4be8c(slots: &mut Vec<DataModelSlot>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x4be8c: D0: vtable resets; intrusive release; operator delete.
    if let Some(pos) = slots.iter().position(|s| s.id == id) {
        let s = slots.remove(pos);
        release(s.id);
    }
}

// 0x4bf3c — __ZN5boost9function1IvPN3RBX9DataModelEE13assign_to_ownERKS4_
// demangled: boost::function1<void,RBX::DataModel *>::assign_to_own(boost::function1<void,RBX::DataModel *> const&)
// type: int __fastcall(int result, int *)
#[doc(alias = "boost::function1<void,RBX::DataModel *>::assign_to_own(boost::function1<void,RBX::DataModel *> const&)")]
pub fn stub_4bf3c(dst: usize, has_src: bool, is_small: bool, copy: &mut dyn FnMut(usize, bool)) -> usize {
    // IDA 0x4bf3c: function1::assign_to_own — inline small copy else heap clone; return dst.
    if has_src {
        copy(dst, is_small);
    }
    dst
}

// 0x4bf6c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPN3RBX9DataModelEENS3_5list3INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_4bf6c(op: u32, manage: &mut dyn FnMut(u32) -> usize) -> usize {
    // IDA 0x4bf6c: functor_manager::manage — clone/move/destroy by op (below truncation).
    manage(op)
}

// 0x4bfcc — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPN3RBX9DataModelEENS3_5list3INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEEEEEEvSA_E6invokeERNS1_15function_bufferESA_
// demangled: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::DataModel>::invoke(boost::detail::function::function_buffer &,RBX::DataModel)
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::DataModel>::invoke(boost::detail::function::function_buffer &,RBX::DataModel)")]
pub fn stub_4bfcc(obj: usize, sel: usize, model: usize, invoke: &mut dyn FnMut(usize, usize, usize)) {
    // IDA 0x4bfcc: invoker calls fn(obj, sel, model).
    invoke(obj, sel, model);
}

// 0x4bfdc — __ZN5boost9function1IvPKN3RBX10Reflection18PropertyDescriptorEE5clearEv
// demangled: boost::function1<void,RBX::Reflection::PropertyDescriptor const*>::clear(void)
// type: int __fastcall(int *)
#[doc(alias = "boost::function1<void,RBX::Reflection::PropertyDescriptor const*>::clear(void)")]
pub fn stub_4bfdc(slot: &mut usize, destroy: &mut dyn FnMut()) -> i32 {
    // IDA 0x4bfdc: function1::clear — heap destroy unless small-bit; clear; 0.
    let v = *slot;
    if v != 0 {
        if v & 1 == 0 {
            destroy();
        }
        *slot = 0;
    }
    0
}

// 0x4c008 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEE5clearEv
// demangled: boost::function1<void,boost::shared_ptr<RBX::TextBox>>::clear(void)
// type: int __fastcall(int *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>>::clear(void)")]
pub fn stub_4c008(slot: &mut usize, destroy: &mut dyn FnMut()) -> i32 {
    // IDA 0x4c008: function1::clear — heap destroy unless small-bit; clear; 0.
    let v = *slot;
    if v != 0 {
        if v & 1 == 0 {
            destroy();
        }
        *slot = 0;
    }
    0
}

// 0x4c034 — __GLOBAL__I_a_18
// demangled: global constructor keyed to_a_18
#[doc(alias = "global constructor keyed to_a_18")]
pub fn stub_4c034(state: &mut GlobalInitA18, init: &mut dyn FnMut()) {
    // IDA 0x4c034: boost error categories + ios_base::Init + bad_alloc static exception object.
    if !state.done {
        init();
        state.done = true;
    }
}

// 0x4c248 — -[GameInputViewController init:withBundle:withGame:overlayDataModel:]
// type: id __cdecl(GameInputViewController *self, SEL, id, id, shared_ptr<RBX::Game>, shared_ptr<RBX::OverlayDataModel>)
#[doc(alias = "-[GameInputViewController init:withBundle:withGame:overlayDataModel:]")]
pub fn stub_4c248(view: usize, game: usize, overlay: usize, init: &mut dyn FnMut(usize, usize, usize)) -> usize {
    // IDA 0x4c248: GameInputViewController init + ControlView setup (below truncation).
    init(view, game, overlay);
    view
}

// 0x4c3f4 — -[GameInputViewController dealloc]
// type: void __cdecl(GameInputViewController *self, SEL)
#[doc(alias = "-[GameInputViewController dealloc]")]
pub fn stub_4c3f4(release: &mut dyn FnMut(), teardown: &mut dyn FnMut()) {
    // IDA 0x4c3f4: release controlView; super dealloc.
    release();
    teardown();
}
