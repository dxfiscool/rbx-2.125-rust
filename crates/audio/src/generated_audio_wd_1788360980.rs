//! audio generated_audio_wd_1788360980 — 120 stubs EA-sorted asc gap filler not yet in audio (FMOD|Sound|Audio exhausted, global gap filler)
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 120 not in audio after 0x66606c | rbx_core::SharedPtr not boost
//! Range 0x666094..0x66b380 | existing 36902 -> 37022 distinct
//! Batch: 120 stubs | // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR mangled")

use rbx_core::SharedPtr;
use crate::generated_audio_wd_watchdog18::TextBoxState;
use crate::generated_audio_wd_watchdog19::stub_665da0;
const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

/// Byte index of `text` floored to a char boundary (IDA `doKey`,
/// 0x667698: the binary does raw byte indexing over the +608
/// string; host `String` ops need boundary-snapped indices —
/// identical for ASCII, safe for UTF-8).
fn snap_idx(text: &str, idx: usize) -> usize {
    let mut i = idx.min(text.len());
    while !text.is_char_boundary(i) {
        i -= 1;
    }
    i
}
/// Start of the char ending at boundary `end` (`end` itself must be
/// a boundary): the byte `doKey` case 0/1 erases for ASCII input.
fn char_start_at(text: &str, end: usize) -> usize {
    (0..end).rev().find(|&i| text.is_char_boundary(i)).unwrap_or(0)
}
/// `doKey` whitespace test (IDA 0x66775a/0x667956/0x667868: `(c -
/// 9) <= 0x17 && bit set in loc_800002`). The mask bits are not
/// observable in the host; the standard C whitespace set in the
/// 9..=32 window grounds the predicate (consistent with the
/// newline-tolerant backward scans at 0x667762-0x667776).
fn is_text_ws(byte: u8) -> bool {
    matches!(byte, b'\t' | b'\n' | 0x0b | b'\x0c' | b'\r' | b' ')
}
/// Key-press kind carried by a `GuiEvent` for `processKeyEvent`
/// (IDA 0x667b80): the `isEscapeKey`/`isCarriageReturnKey`/
/// `isLeftArrowKey`/`isRightArrowKey`/`isClearKey`/`isDeleteKey`/
/// `isBackspaceKey`/`isTextCharacterKey` predicates fold into the
/// variant; `kind` is the event word (`*a3`: 10 press, 11 release),
/// `keycode`/`modifiers`/`ch` are the +12/+16/+8 event cells.
#[derive(Debug, Clone, Copy)]
pub enum TextKeyPress {
    Escape,
    Return,
    Left,
    Right,
    Clear,
    Delete,
    Backspace,
    TextChar,
    Other,
}
#[derive(Debug, Clone, Copy)]
pub struct TextKeyEvent {
    pub kind: u32,
    pub press: TextKeyPress,
    pub keycode: u32,
    pub modifiers: u32,
    pub ch: u8,
}


// 0x666094 — __ZN3RBX7TextBox11setTextWrapEb
// demangled: RBX::TextBox::setTextWrap(bool)
// type: _DWORD __fastcall(RBX::TextBox *__hidden this, bool)
#[doc(alias = "RBX::TextBox::setTextWrap(bool)")]
#[doc(alias = "__ZN3RBX7TextBox11setTextWrapEb")]
pub fn stub_666094(state: &mut TextBoxState, wrap: bool) -> bool {
    // IDA 0x666094 (`RBX::TextBox::setTextWrap`): compares the +580
    // byte (0x66609a); on change stores it (0x6660ae) and raises
    // three descriptors (0x6660b8-0x6660d0), else returns unchanged
    // (0x6660a2). The raises fold into the changed flag.
    if state.text_wrap == wrap {
        return false;
    }
    state.text_wrap = wrap;
    true
}


// 0x6660d4 — __ZN3RBX7TextBox12setTextScaleEb
// demangled: RBX::TextBox::setTextScale(bool)
// type: _DWORD __fastcall(RBX::TextBox *__hidden this, bool)
#[doc(alias = "RBX::TextBox::setTextScale(bool)")]
#[doc(alias = "__ZN3RBX7TextBox12setTextScaleEb")]
pub fn stub_6660d4(state: &mut TextBoxState, scale: bool) -> bool {
    // IDA 0x6660d4 (`RBX::TextBox::setTextScale`): compares the +581
    // byte (0x6660da); on change stores it (0x6660f0), raises
    // (0x6660fa) and — when enabling — delegates to
    // `setTextWrap(this, 1)` (0x666100-0x66610a, host: the 0x666094
    // twin); disabling raises twice more instead
    // (0x666114-0x666122). All raises fold into the changed flag.
    if state.text_scaled == scale {
        return false;
    }
    state.text_scaled = scale;
    if scale {
        stub_666094(state, true);
    }
    true
}


// 0x666128 — __ZN3RBX7TextBox13setXAlignmentENS_11TextService10XAlignmentE
// demangled: RBX::TextBox::setXAlignment(RBX::TextService::XAlignment)
#[doc(alias = "RBX::TextBox::setXAlignment(RBX::TextService::XAlignment)")]
#[doc(alias = "__ZN3RBX7TextBox13setXAlignmentENS_11TextService10XAlignmentE")]
pub fn stub_666128(state: &mut TextBoxState, value: u32) -> bool {
    // IDA 0x666128 (`RBX::TextBox::setXAlignment`): compares word
    // 146 (+584, 0x66612e); on change stores it (0x666142) and
    // raises three descriptors (0x66614c-0x666164), else returns
    // unchanged (0x666136). The raises fold into the changed flag.
    // Values ride the `XAlignment` table (`Left` 0, `Right` 1,
    // `Center` 2 — IDA 0x7d8548).
    if state.x_alignment == value {
        return false;
    }
    state.x_alignment = value;
    true
}


// 0x666168 — __ZN3RBX7TextBox13setYAlignmentENS_11TextService10YAlignmentE
// demangled: RBX::TextBox::setYAlignment(RBX::TextService::YAlignment)
#[doc(alias = "RBX::TextBox::setYAlignment(RBX::TextService::YAlignment)")]
#[doc(alias = "__ZN3RBX7TextBox13setYAlignmentENS_11TextService10YAlignmentE")]
pub fn stub_666168(state: &mut TextBoxState, value: u32) -> bool {
    // IDA 0x666168 (`RBX::TextBox::setYAlignment`): compares word
    // 147 (+588, 0x66616e); on change stores it (0x666182) and
    // raises three descriptors (0x66618c-0x6661a4), else returns
    // unchanged (0x666176). The raises fold into the changed flag.
    if state.y_alignment == value {
        return false;
    }
    state.y_alignment = value;
    true
}


// 0x6661a8 — __ZNK3RBX7TextBox13getTextBoundsEv
// demangled: RBX::TextBox::getTextBounds(void)const
// type: _DWORD __fastcall(RBX::TextBox *__hidden this)
#[doc(alias = "RBX::TextBox::getTextBounds(void)const")]
#[doc(alias = "__ZNK3RBX7TextBox13getTextBoundsEv")]
pub fn stub_6661a8() -> [f32; 2] {
    // IDA 0x6661a8 (`RBX::TextBox::getTextBounds`): the first param
    // is the out `Vector2` — the no-frontend/no-`TextService`/
    // no-typesetter path zeroes it (0x66628a-0x66629a, LABEL_8).
    // The measurable path (0x66621a-0x6662ee: typesetter over the
    // +540 text with `convertFontSize(+544)` and the +580 wrap
    // rect) needs `TextService` rasterization: gap. Host: the
    // exact no-service floor.
    [0.0, 0.0]
}


// 0x666334 — __ZNK3RBX7TextBox11getTextFitsEv
// demangled: RBX::TextBox::getTextFits(void)const
// type: _DWORD __fastcall(RBX::TextBox *__hidden this)
#[doc(alias = "RBX::TextBox::getTextFits(void)const")]
#[doc(alias = "__ZNK3RBX7TextBox11getTextFitsEv")]
pub fn stub_666334() -> bool {
    // IDA 0x666334 (`RBX::TextBox::getTextFits`): every
    // unmeasurable path yields 0 — the `!frontendProcessing` else
    // branch (0x666408) and the no-typesetter `!v18` reset
    // (0x666480-0x666482) — returned as `v16 & 1` (0x6664a2). The
    // measurable path (0x6663a0-0x66646a: typeset the +540 text,
    // compare against the rect width) needs `TextService`
    // rasterization: gap. Host: the exact no-service floor.
    false
}


// 0x6664e4 — __ZN3RBX7TextBox19setTextStrokeColor3EN3G3D6Color3E
// demangled: RBX::TextBox::setTextStrokeColor3(G3D::Color3)
#[doc(alias = "RBX::TextBox::setTextStrokeColor3(G3D::Color3)")]
#[doc(alias = "__ZN3RBX7TextBox19setTextStrokeColor3EN3G3D6Color3E")]
pub fn stub_6664e4(state: &mut TextBoxState, color: [f32; 3]) -> bool {
    // IDA 0x6664e4 (`RBX::TextBox::setTextStrokeColor3`): compares
    // +564 (0x6664f4), then +568/+572 (0x666502-0x666526); on any
    // difference stores all three (0x66652c-0x666540) and raises
    // (0x666548). The raise folds into the changed flag.
    if state.text_stroke_color3 == color {
        return false;
    }
    state.text_stroke_color3 = color;
    true
}


// 0x66654c — __ZN3RBX7TextBox25setTextStrokeTransparencyEf
// demangled: RBX::TextBox::setTextStrokeTransparency(float)
// type: _DWORD __fastcall(RBX::TextBox *__hidden this, float)
#[doc(alias = "RBX::TextBox::setTextStrokeTransparency(float)")]
#[doc(alias = "__ZN3RBX7TextBox25setTextStrokeTransparencyEf")]
pub fn stub_66654c(state: &mut TextBoxState, transparency: f32) -> bool {
    // IDA 0x66654c (`RBX::TextBox::setTextStrokeTransparency`):
    // compares word 144 (+576, 0x66655c); on change stores it
    // (0x666568) and raises (0x666572), else returns unchanged
    // (0x66655e). The raise folds into the changed flag.
    if state.text_stroke_transparency == transparency {
        return false;
    }
    state.text_stroke_transparency = transparency;
    true
}


// 0x666578 — __ZN3RBX7TextBox14checkForResizeEv
// demangled: RBX::TextBox::checkForResize(void)
// type: _DWORD __fastcall(RBX::TextBox *__hidden this)
#[doc(alias = "RBX::TextBox::checkForResize(void)")]
#[doc(alias = "__ZN3RBX7TextBox14checkForResizeEv")]
pub fn stub_666578() {
    // IDA 0x666578 (`RBX::TextBox::checkForResize`): the
    // `GuiObject::checkForResize` body plus two
    // `raisePropertyChanged` calls (0x66657e-0x666598) — no
    // `TextBox`-member effect. Carrier no-op.
}


// 0x6665a4 — __ZN3RBX7TextBox21setTransparencyLegacyEf
// demangled: RBX::TextBox::setTransparencyLegacy(float)
// type: _DWORD __fastcall(RBX::TextBox *__hidden this, float)
#[doc(alias = "RBX::TextBox::setTransparencyLegacy(float)")]
#[doc(alias = "__ZN3RBX7TextBox21setTransparencyLegacyEf")]
pub fn stub_6665a4(state: &mut TextBoxState, transparency: f32) -> bool {
    // IDA 0x6665a4 (`RBX::TextBox::setTransparencyLegacy`): on
    // change of word 140 (+560, 0x6665be) stores it (0x6665c8) and
    // raises (0x6665d4); the `GuiObject::setBackgroundTransparency`
    // tail (0x6665da) owns the GuiObject layer and folds away.
    // Host: the `TextTransparency`-member half as a changed flag.
    if state.text_transparency == transparency {
        return false;
    }
    state.text_transparency = transparency;
    true
}


// 0x6665ec — __ZNK3RBX7TextBox14getPosInStringEN3G3D7Vector2E
// demangled: RBX::TextBox::getPosInString(G3D::Vector2)const
#[doc(alias = "RBX::TextBox::getPosInString(G3D::Vector2)const")]
#[doc(alias = "__ZNK3RBX7TextBox14getPosInStringEN3G3D7Vector2E")]
pub fn stub_6665ec() -> i32 {
    // IDA 0x6665ec (`RBX::TextBox::getPosInString`): without
    // frontend processing (0x666640) or without a `TextService`
    // (0x66668e) it returns -1; the measurable path (0x66665a on:
    // typesetter over the +592 font id, the rect and the +584
    // `XAlignment`) needs `TextService` rasterization: gap. Host:
    // the exact no-service floor.
    -1
}


// 0x6668b0 — __ZNK3RBX7TextBox21getPersistentDataCostEv
// demangled: RBX::TextBox::getPersistentDataCost(void)const
// type: _DWORD __fastcall(RBX::TextBox *__hidden this)
#[doc(alias = "RBX::TextBox::getPersistentDataCost(void)const")]
#[doc(alias = "__ZNK3RBX7TextBox21getPersistentDataCostEv")]
pub fn stub_6668b0(base: i32, text: &str) -> i32 {
    // IDA 0x6668b0 (`RBX::TextBox::getPersistentDataCost`): the
    // `Instance` base cost (0x6668c4, host: `base`) plus 1 — or the
    // +540 text byte-length / 100 when that exceeds 1 (0x6668d6-
    // 0x6668f4) — plus 6 (0x666908).
    let chunks = (text.len() / 100) as i32;
    base + if chunks > 1 { chunks } else { 1 } + 6
}


// 0x666938 — __ZN3RBX7TextBoxC2Ev
// demangled: RBX::TextBox::TextBox(void)
// type: _DWORD __fastcall(RBX::TextBox *__hidden this)
#[doc(alias = "RBX::TextBox::TextBox(void)")]
#[doc(alias = "__ZN3RBX7TextBoxC2Ev")]
pub fn stub_666938() -> TextBoxState {
    // IDA 0x666938 (`RBX::TextBox::TextBox`): the `GuiObject` base,
    // vtables, class descriptor and registrar fold away; the member
    // stores ground `TextBoxState::default` — +540 `Text` =
    // "TextBox", +548..+556 the palette-26 `BrickColor::color3`,
    // +560/+564..+572 zero transparencies/colors, +576 the 1.0
    // stroke transparency, +580/+581 cleared wrap/scale, +584 = 2 /
    // +588 = 1 alignments, +604..+606 cleared focus cells, +607
    // set clear-on-focus, +608 empty focus text, +612/+620 cleared
    // time/cursor words, +648 cleared key phase, +652 cleared
    // multi-line, +656/+660 cleared connection cells.
    TextBoxState::default()
}


// 0x666d28 — __ZN3RBX7TextBox17onServiceProviderEPNS_15ServiceProviderES2_
// demangled: RBX::TextBox::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)
// type: _DWORD __fastcall(RBX::TextBox *__hidden this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "RBX::TextBox::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
#[doc(alias = "__ZN3RBX7TextBox17onServiceProviderEPNS_15ServiceProviderES2_")]
pub fn stub_666d28() {
    // IDA 0x666d28 (`RBX::TextBox::onServiceProvider`): disconnects
    // the +656 connection (0x666d84), zeroes the +624 `RunService`
    // time source (0x666d96), forwards the +596 `HeartbeatInstance`
    // cell (0x666d9e), connects `externalReleaseFocus` to the
    // `UserInputService` signal into +656 (0x666dac-0x666e16) and
    // re-resolves the +624 source via `find<RunService>`
    // (0x666e26-0x666e2a). Connections and the service pointer
    // fold into the host seams (`keyDown`'s `now: Option`,
    // `capture/gainFocus`'s create gate). Carrier no-op.
}


// 0x666e84 — __ZN3RBX7TextBox20externalReleaseFocusEPKcb
// demangled: RBX::TextBox::externalReleaseFocus(char const*,bool)
// type: _DWORD __fastcall(RBX::TextBox *__hidden this, const char *, bool)
#[doc(alias = "RBX::TextBox::externalReleaseFocus(char const*,bool)")]
#[doc(alias = "__ZN3RBX7TextBox20externalReleaseFocusEPKcb")]
pub fn stub_666e84(
    state: &mut TextBoxState,
    text: &str,
    submitted: bool,
    filter_pass: bool,
    fire_focused: impl Fn(bool),
) {
    // IDA 0x666e84 (`RBX::TextBox::externalReleaseFocus`): gated on
    // +605 (0x666eae); the +608 focus text takes the input
    // (0x666eee-0x666ef4) and is committed through `setText`
    // (0x666f28, host: the 0x665da0 twin with its filter seam);
    // +605/+606/+648 are cleared (0x666f58-0x666f60) and the
    // `Focused(bool)` signal fires with the input flag (0x666f6c).
    // The character/datamodel flag clears fold away.
    if !state.focused {
        return;
    }
    state.focus_text = text.to_owned();
    let commit = state.focus_text.clone();
    stub_665da0(state, &commit, filter_pass);
    state.focused = false;
    state.cursor_visible = false;
    state.key_phase = 0;
    fire_focused(submitted);
}


// 0x667088 — __ZN3RBX7TextBox17processMouseEventERKNS_8GuiEventE
// demangled: RBX::TextBox::processMouseEvent(RBX::GuiEvent const&)
#[doc(alias = "RBX::TextBox::processMouseEvent(RBX::GuiEvent const&)")]
#[doc(alias = "__ZN3RBX7TextBox17processMouseEventERKNS_8GuiEventE")]
pub fn stub_667088(
    state: &mut TextBoxState,
    pressed: bool,
    inside: bool,
    click_cursor: Option<usize>,
    now: f64,
    filter_pass: bool,
    input_service_created: bool,
    fire_box: impl Fn(),
    fire_bool: impl Fn(bool),
) -> bool {
    // IDA 0x667088 (`RBX::TextBox::processMouseEvent`): the
    // `GuiObject` base call and the `getRect2D` inside-test over
    // the packed event coords (0x66709e-0x6670fc) fold into
    // `pressed` (event kind 3) and `inside`. Press inside gains
    // focus with the click event (0x66710c, host: the 0x667144 twin
    // — kind 3 takes the setText-commit path); press outside
    // releases with `submitted = false` (0x667116). Returns the
    // +605 tuple fill (0x66711a-0x66712e, consumed) or the base
    // result (folds): host returns the focused flag.
    if pressed {
        if inside {
            stub_667144(state, 3, click_cursor, now, filter_pass, input_service_created, fire_box);
        } else {
            stub_667388(state, false, filter_pass, fire_bool);
        }
    }
    state.focused
}


// 0x667144 — __ZN3RBX7TextBox9gainFocusERKNS_8GuiEventE
// demangled: RBX::TextBox::gainFocus(RBX::GuiEvent const&)
#[doc(alias = "RBX::TextBox::gainFocus(RBX::GuiEvent const&)")]
#[doc(alias = "__ZN3RBX7TextBox9gainFocusERKNS_8GuiEventE")]
pub fn stub_667144(
    state: &mut TextBoxState,
    kind: u32,
    click_cursor: Option<usize>,
    now: f64,
    filter_pass: bool,
    input_service_created: bool,
    fire_focused: impl Fn(),
) {
    // IDA 0x667144 (`RBX::TextBox::gainFocus`): cursor (+620) takes
    // the +608 length and +605/+606 are set (0x66717a-0x66717e);
    // clear-on-focus (+607) empties +608 and zeroes the cursor
    // (0x667186-0x6671c4), otherwise a kind-3 event commits +608
    // through `setText` (0x6671d0-0x6671ec, host: the 0x665da0 twin
    // with its filter seam) and re-seats the cursor through
    // `getCursorPos` over the event point (0x66720e-0x66723c —
    // viewport folds into `click_cursor`); the focus time (+612),
    // a zeroed key phase (+648) and the gated `UserInputService`
    // `Focused` fire (0x667248-0x6672b8) close out. Character/
    // datamodel flag sets fold away.
    state.cursor = state.focus_text.len();
    state.focused = true;
    state.cursor_visible = true;
    if state.clear_text_on_focus {
        state.focus_text.clear();
        state.cursor = 0;
    } else if kind == 3 {
        let commit = state.focus_text.clone();
        stub_665da0(state, &commit, filter_pass);
        state.cursor = click_cursor.unwrap_or(state.cursor).min(state.focus_text.len());
    }
    state.focus_time = now;
    state.key_phase = 0;
    if input_service_created {
        fire_focused();
    }
}


// 0x667388 — __ZN3RBX7TextBox12releaseFocusERKNS_8GuiEventEb
// demangled: RBX::TextBox::releaseFocus(RBX::GuiEvent const&,bool)
// type: int __fastcall(int, int, void *)
#[doc(alias = "RBX::TextBox::releaseFocus(RBX::GuiEvent const&,bool)")]
#[doc(alias = "__ZN3RBX7TextBox12releaseFocusERKNS_8GuiEventEb")]
pub fn stub_667388(state: &mut TextBoxState, submitted: bool, filter_pass: bool, fire_focused: impl Fn(bool)) {
    // IDA 0x667388 (`RBX::TextBox::releaseFocus`): gated on +605
    // (0x6673b4); the +608 focus text is committed through
    // `setText` (0x667400-0x66740e, host: the 0x665da0 twin with
    // its filter seam); +605/+606/+648 are cleared
    // (0x667432-0x66743a) and the `Focused(bool)` signal fires
    // with the submitted flag (0x667454). The character/datamodel
    // flag clears fold away.
    if !state.focused {
        return;
    }
    let commit = state.focus_text.clone();
    stub_665da0(state, &commit, filter_pass);
    state.focused = false;
    state.cursor_visible = false;
    state.key_phase = 0;
    fire_focused(submitted);
}


// 0x667500 — __ZN3RBX7TextBox12getCursorPosEN3G3D7Vector2E
// demangled: RBX::TextBox::getCursorPos(G3D::Vector2)
#[doc(alias = "RBX::TextBox::getCursorPos(G3D::Vector2)")]
#[doc(alias = "__ZN3RBX7TextBox12getCursorPosEN3G3D7Vector2E")]
pub fn stub_667500(state: &TextBoxState, pos: i32, past_midpoint: bool) -> usize {
    // IDA 0x667500 (`RBX::TextBox::getCursorPos`): a valid
    // `getPosInString` over the input point returns it (0x667556,
    // host: `pos` — the 0x6665ec twin, viewport folds into the
    // caller); -1 falls back to the rect-midpoint test
    // (0x667520-0x66754a, viewport folds into `past_midpoint`):
    // at/past the midpoint yields the +608 length (0x667550),
    // else 0. The cursor>=0 assert (TextBox.cpp:444) rides the
    // `usize` return.
    if pos >= 0 {
        return pos as usize;
    }
    if past_midpoint {
        state.focus_text.len()
    } else {
        0
    }
}


// 0x667558 — __ZN3RBX7TextBox17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// demangled: RBX::TextBox::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)
// type: _DWORD __fastcall(RBX::TextBox *__hidden this, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "RBX::TextBox::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
#[doc(alias = "__ZN3RBX7TextBox17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE")]
pub fn stub_667558(state: &mut TextBoxState, is_text_prop: bool) {
    // IDA 0x667558 (`RBX::TextBox::onPropertyChanged`): when the
    // changed descriptor is the `Text` one, the +608 focus text
    // re-syncs from the +540 committed text; the `GuiObject` tail
    // result folds away.
    if is_text_prop {
        state.focus_text = state.text.clone();
    }
}


// 0x6675f8 — __ZN3RBX7TextBox11onHeartbeatERKNS_9HeartbeatE
// demangled: RBX::TextBox::onHeartbeat(RBX::Heartbeat const&)
#[doc(alias = "RBX::TextBox::onHeartbeat(RBX::Heartbeat const&)")]
#[doc(alias = "__ZN3RBX7TextBox11onHeartbeatERKNS_9HeartbeatE")]
pub fn stub_6675f8(state: &mut TextBoxState, now: f64) {
    // IDA 0x6675f8 (`RBX::TextBox::onHeartbeat`): gated on +605
    // (0x667604); phase 1 fires `doKey` with the held
    // (+628 key, +636 char) once the +640 time is 0.5 past
    // (0x667612-0x667646, host: the 0x667698 twin) and advances
    // to phase 2; phase 2 repeats every 0.05 (0x66764e-0x667688).
    if !state.focused {
        return;
    }
    if state.key_phase == 1 {
        if state.key_time + 0.5 < now {
            stub_667698(state, state.key_type, state.key_char);
            state.key_phase = 2;
            state.key_time = now;
        }
    } else if state.key_phase == 2 {
        while state.key_time + 0.05 < now {
            stub_667698(state, state.key_type, state.key_char);
            state.key_time += 0.05;
        }
    }
}


// 0x667698 — __ZN3RBX7TextBox5doKeyENS0_14RepeatKeyState7KeyTypeEc
// demangled: RBX::TextBox::doKey(RBX::TextBox::RepeatKeyState::KeyType,char)
#[doc(alias = "RBX::TextBox::doKey(RBX::TextBox::RepeatKeyState::KeyType,char)")]
#[doc(alias = "__ZN3RBX7TextBox5doKeyENS0_14RepeatKeyState7KeyTypeEc")]
pub fn stub_667698(state: &mut TextBoxState, key_type: u32, key_char: u8) {
    // IDA 0x667698 (`RBX::TextBox::doKey(RepeatKeyState::KeyType,
    // char)`): the switch over the key type (0x6676ee) edits the
    // +608 focus text around the +620/word-155 cursor (byte
    // indices — host snaps to char boundaries, ASCII-identical):
    // 0 backspace — with a char (ctrl+backspace) erases the word
    // back (0x667708-0x667984: trailing-whitespace skip, word
    // skip, all-whitespace prefix truncate via LABEL_54 at
    // 0x667986-0x6679c8), without erases one byte back
    // (0x6678ee-0x667908); 1 delete erases one byte at the cursor
    // (0x667788-0x6677ac); 2 inserts the empty string when the
    // phase is 0 (0x6677ae-0x667842, total no-op — the guard
    // folds); 3 inserts printable ASCII (33..=126 except 127) or
    // mask-whitespace (0x667868) and advances (0x667870-0x6678c2);
    // 4/5 step the cursor within 0..=len (0x6678c8-0x6678e6).
    match key_type {
        0 => {
            if key_char != 0 {
                let len = state.focus_text.len();
                let start = (len as isize - 1).min(state.cursor as isize);
                let bytes = state.focus_text.as_bytes();
                let mut i = start;
                while i >= 0 && is_text_ws(bytes[i as usize]) {
                    i -= 1;
                }
                if i <= -1 {
                    let at = snap_idx(&state.focus_text, state.cursor);
                    state.focus_text = state.focus_text[at..].to_owned();
                    state.cursor = 0;
                    return;
                }
                let mut j = i;
                loop {
                    if j < 0 {
                        let at = snap_idx(&state.focus_text, state.cursor);
                        state.focus_text = state.focus_text[at..].to_owned();
                        state.cursor = 0;
                        return;
                    }
                    if is_text_ws(bytes[j as usize]) {
                        break;
                    }
                    j -= 1;
                }
                let at = snap_idx(&state.focus_text, state.cursor);
                let j = j as usize;
                if at != j {
                    state.focus_text.drain(j..at);
                    state.cursor = j;
                }
            } else if !state.focus_text.is_empty() {
                let at = snap_idx(&state.focus_text, state.cursor);
                if at >= 1 {
                    state.focus_text.remove(char_start_at(&state.focus_text, at));
                    state.cursor = at - 1;
                }
            }
        }
        1 => {
            let at = snap_idx(&state.focus_text, state.cursor);
            if !state.focus_text.is_empty() && at < state.focus_text.len() {
                state.focus_text.remove(at);
            }
        }
        2 => {}
        3 => {
            if (key_char >= 33 && key_char != 127) || is_text_ws(key_char) {
                let at = snap_idx(&state.focus_text, state.cursor);
                // Binary inserts the raw byte; host `char` insert
                // is byte-identical for ASCII.
                state.focus_text.insert(at, key_char as char);
                state.cursor = at + 1;
            }
        }
        4 => {
            let at = snap_idx(&state.focus_text, state.cursor);
            if at >= 1 {
                state.cursor = at - 1;
            }
        }
        5 => {
            let at = snap_idx(&state.focus_text, state.cursor);
            if state.focus_text.len() >= at + 1 {
                state.cursor = at + 1;
            }
        }
        _ => {}
    }
}


// 0x667b28 — __ZThn596_N3RBX7TextBox11onHeartbeatERKNS_9HeartbeatE
// demangled: non-virtual thunk toRBX::TextBox::onHeartbeat(RBX::Heartbeat const&)
#[doc(alias = "non-virtual thunk toRBX::TextBox::onHeartbeat(RBX::Heartbeat const&)")]
#[doc(alias = "__ZThn596_N3RBX7TextBox11onHeartbeatERKNS_9HeartbeatE")]
pub fn stub_667b28() {
    // IDA 0x667b28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x667b30 — __ZN3RBX7TextBox7keyDownENS0_14RepeatKeyState7KeyTypeENS_7KeyCodeEc
// demangled: RBX::TextBox::keyDown(RBX::TextBox::RepeatKeyState::KeyType,RBX::KeyCode,char)
#[doc(alias = "RBX::TextBox::keyDown(RBX::TextBox::RepeatKeyState::KeyType,RBX::KeyCode,char)")]
#[doc(alias = "__ZN3RBX7TextBox7keyDownENS0_14RepeatKeyState7KeyTypeENS_7KeyCodeEc")]
pub fn stub_667b30(
    state: &mut TextBoxState,
    key_type: u32,
    key_code: u32,
    key_char: u8,
    now: Option<f64>,
) {
    // IDA 0x667b30 (`RBX::TextBox::keyDown(RepeatKeyState::KeyType,
    // RBX::KeyCode, char)`): with the phase at 0/1 (0x667b46)
    // `doKey`s the type/char pair (0x667b4e, host: the 0x667698
    // twin), latches type/code/char into +628/+632/+636
    // (0x667b52-0x667b5e), arms phase 1 for every type but 2
    // (0x667b62-0x667b66) and refreshes the +640 key time from
    // the +624 `RunService` cell's +140 double when present
    // (0x667b6a-0x667b76, host: `now: Option` — `None` folds the
    // null source). The returned cell is caller-ignored.
    if state.key_phase <= 1 {
        stub_667698(state, key_type, key_char);
        state.key_type = key_type;
        state.key_code = key_code;
        state.key_char = key_char;
        if key_type != 2 {
            state.key_phase = 1;
        }
        if let Some(t) = now {
            state.key_time = t;
        }
    }
}


// 0x667b80 — __ZN3RBX7TextBox15processKeyEventERKNS_8GuiEventE
// demangled: RBX::TextBox::processKeyEvent(RBX::GuiEvent const&)
#[doc(alias = "RBX::TextBox::processKeyEvent(RBX::GuiEvent const&)")]
#[doc(alias = "__ZN3RBX7TextBox15processKeyEventERKNS_8GuiEventE")]
pub fn stub_667b80(
    state: &mut TextBoxState,
    event: &TextKeyEvent,
    now: f64,
    time_source: Option<f64>,
    filter_pass: bool,
    input_service_created: bool,
    fire_box: impl Fn(),
    fire_bool: impl Fn(bool),
) -> bool {
    // IDA 0x667b80 (`RBX::TextBox::processKeyEvent`): an armed
    // +604 first gains focus with the key event and disarms
    // (0x667b8e-0x667ba0 — the kind is 10/11, never 3, so the
    // 0x667144 twin keeps the cursor). Kind-10 presses dispatch:
    // escape releases unsubmitted (0x667bde-0x667bea); return
    // types `\\n` through `keyDown(3, code, 10)` in multi-line
    // (0x667c1c-0x667c2c) else releases submitted (0x667cc0);
    // left/right arrow `keyDown(4/5, code, 108/114)`
    // (0x667c52-0x667c98); clear empties the focus text; delete
    // `keyDown(1, code, 0)`; backspace `keyDown(0, code, bit)`
    // with the bit from the modifiers (`0x40` set: 1, else
    // `(mods >> 7) & 1`); text char pastes through `keyDown(2,
    // 118, 118)` for ctrl/cmd+v (0x667d6c-0x667dd2) else
    // `keyDown(3, keycode, ch)`. Kind-11 releases only stop a
    // matching repeat (0x667bbe-0x667d0e: phase 1/2 plus the
    // stored +628 type 1/0/4/5/3 for delete/backspace/left/
    // right/textchar — carriage, multi or not, falls through to
    // LABEL_42 with no effect at 0x667bc2-0x667bd0); LABEL_40
    // zeroes the phase on a keycode match (0x667d0a-0x667d0e).
    // LABEL_42 zeroes the phase for keycodes 303/304
    // (0x667d24-0x667d28). Returns consumed: the +605 tuple fill
    // (0x667d2c-0x667d44) or the folded base result.
    if state.focus_armed {
        stub_667144(state, event.kind, None, now, filter_pass, input_service_created, fire_box);
        state.focus_armed = false;
    }
    if event.kind == 10 {
        match event.press {
            TextKeyPress::Escape => stub_667388(state, false, filter_pass, fire_bool),
            TextKeyPress::Return => {
                if state.multi_line {
                    stub_667b30(state, 3, event.keycode, 10, time_source);
                } else {
                    stub_667388(state, true, filter_pass, fire_bool);
                }
            }
            TextKeyPress::Left => stub_667b30(state, 4, event.keycode, 108, time_source),
            TextKeyPress::Right => stub_667b30(state, 5, event.keycode, 114, time_source),
            TextKeyPress::Clear => state.focus_text.clear(),
            TextKeyPress::Delete => stub_667b30(state, 1, event.keycode, 0, time_source),
            TextKeyPress::Backspace => {
                let bit = if event.modifiers & 0x40 != 0 {
                    1
                } else {
                    ((event.modifiers >> 7) & 1) as u8
                };
                stub_667b30(state, 0, event.keycode, bit, time_source);
            }
            TextKeyPress::TextChar => {
                if event.ch == 118 && (event.modifiers == 1024 || event.modifiers == 2048) {
                    stub_667b30(state, 2, 118, 118, time_source);
                } else {
                    stub_667b30(state, 3, event.keycode, event.ch, time_source);
                }
            }
            TextKeyPress::Other => {}
        }
    } else if event.kind == 11 {
        let expected = match event.press {
            TextKeyPress::Delete => Some(1),
            TextKeyPress::Backspace => Some(0),
            TextKeyPress::Left => Some(4),
            TextKeyPress::Right => Some(5),
            TextKeyPress::TextChar => Some(3),
            _ => None,
        };
        if let Some(exp) = expected {
            if (state.key_phase == 1 || state.key_phase == 2) && state.key_type == exp {
                if state.key_code == event.keycode {
                    state.key_phase = 0;
                }
            }
        }
    }
    if (event.keycode == 303 || event.keycode == 304) && (state.key_phase == 1 || state.key_phase == 2) {
        state.key_phase = 0;
    }
    state.focused
}


// 0x667dd8 — __ZN3RBX7TextBox17getTextWithCursorEv
// demangled: RBX::TextBox::getTextWithCursor(void)
// type: _DWORD __fastcall(RBX::TextBox *__hidden this)
#[doc(alias = "RBX::TextBox::getTextWithCursor(void)")]
#[doc(alias = "__ZN3RBX7TextBox17getTextWithCursorEv")]
pub fn stub_667dd8(state: &mut TextBoxState, new_fonts: bool) -> String {
    // IDA 0x667dd8 (`RBX::TextBox::getTextWithCursor`): the
    // cursor>=0 assert (TextBox.cpp:444) rides the `usize` cursor;
    // the cursor clamps to the +608 length and the clamped value
    // is written back (0x667e86-0x667ea2); the marker goes in at
    // the cursor — `\x01` under the new fonts flag
    // Binary clamps both ways (0x667e86-0x667ea2); `snap_idx`
    // already floors to 0..=len.
    let clamped = snap_idx(&state.focus_text, state.cursor);
    state.cursor = clamped;
    let marker = if new_fonts { "\x01" } else { "|" };
    let mut out = state.focus_text[..clamped].to_owned();
    out.push_str(marker);
    out.push_str(&state.focus_text[clamped..]);
    out
}


// 0x667f3c — __ZN3RBX7TextBox22getTextWithBlankCursorEv
// demangled: RBX::TextBox::getTextWithBlankCursor(void)
// type: _DWORD __fastcall(RBX::TextBox *__hidden this)
#[doc(alias = "RBX::TextBox::getTextWithBlankCursor(void)")]
#[doc(alias = "__ZN3RBX7TextBox22getTextWithBlankCursorEv")]
pub fn stub_667f3c(state: &mut TextBoxState, new_fonts: bool) -> String {
    // IDA 0x667f3c (`RBX::TextBox::getTextWithBlankCursor`): the
    // cursor>=0 assert (TextBox.cpp:460) rides the `usize` cursor;
    // same clamp-and-write-back as `getTextWithCursor`
    let clamped = snap_idx(&state.focus_text, state.cursor);
    state.cursor = clamped;
    let mut out = state.focus_text.clone();
    if !new_fonts {
        out.insert(clamped, '\x01');
    }
    out
}


// 0x668088 — __ZN3RBX7TextBox8render2dEPNS_5AdornE
// demangled: RBX::TextBox::render2d(RBX::Adorn *)
// type: _DWORD __fastcall(RBX::TextBox *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::TextBox::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX7TextBox8render2dEPNS_5AdornE")]
pub fn stub_668088(state: &mut TextBoxState, now: f64) {
    // IDA 0x668088 (`RBX::TextBox::render2d`): gated on +605
    // (0x6680b6); past 0.5 since the +612 focus time the +606
    // cursor-blink flag toggles and +612 refreshes
    // (0x6680ea-0x668122, host: the `now` step); the blink-on
    // path draws `getTextWithCursor`, the blink-off path
    // `getTextWithBlankCursor` (both read-only), each through
    // `getRenderBackgroundColor4`/`render2dTextImpl` triplets —
    // the `Adorn` rasterization folds away (no host renderer).
    if !state.focused {
        return;
    }
    if now - state.focus_time > 0.5 {
        state.cursor_visible = !state.cursor_visible;
        state.focus_time = now;
    }
}


// 0x66856c — __ZThn96_N3RBX7TextBox8render2dEPNS_5AdornE
// demangled: non-virtual thunk toRBX::TextBox::render2d(RBX::Adorn *)
// type: _DWORD __fastcall(RBX::TextBox *__hidden this, RBX::Adorn *)
#[doc(alias = "non-virtual thunk toRBX::TextBox::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZThn96_N3RBX7TextBox8render2dEPNS_5AdornE")]
pub fn stub_66856c() {
    // IDA 0x66856c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x668574 — __ZNK3RBX7TextBox12getMultiLineEv
// demangled: RBX::TextBox::getMultiLine(void)const
// type: _DWORD __fastcall(RBX::TextBox *__hidden this)
#[doc(alias = "RBX::TextBox::getMultiLine(void)const")]
#[doc(alias = "__ZNK3RBX7TextBox12getMultiLineEv")]
pub fn stub_668574(state: &TextBoxState) -> bool {
    // IDA 0x668574 (`RBX::TextBox::getMultiLine`): returns the +652
    // byte (0x668578).
    state.multi_line
}


// 0x66857c — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEbED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEbED1Ev")]
pub fn stub_66857c() {
    // IDA 0x66857c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x6685a0 — __ZNK3RBX7TextBox19getClearTextOnFocusEv
// demangled: RBX::TextBox::getClearTextOnFocus(void)const
// type: _DWORD __fastcall(RBX::TextBox *__hidden this)
#[doc(alias = "RBX::TextBox::getClearTextOnFocus(void)const")]
#[doc(alias = "__ZNK3RBX7TextBox19getClearTextOnFocusEv")]
pub fn stub_6685a0(state: &TextBoxState) -> bool {
    // IDA 0x6685a0 (`RBX::TextBox::getClearTextOnFocus`): returns
    // the +607 byte (0x6685a4).
    state.clear_text_on_focus
}


// 0x6685a8 — __ZN3RBX10Reflection13BoundFuncDescINS_7TextBoxEFvvELi0EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::TextBox,void ()(void),0>::~BoundFuncDesc()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TextBox,void ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_7TextBoxEFvvELi0EED1Ev")]
pub fn stub_6685a8() {
    // IDA 0x6685a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x6685cc — __ZN3RBX10Reflection9EventDescINS_7TextBoxEFvbEN3rbx6signalIS3_EEMS2_S6_ED1Ev
// demangled: RBX::Reflection::EventDesc<RBX::TextBox,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::TextBox::*>::~EventDesc()
#[doc(alias = "RBX::Reflection::EventDesc<RBX::TextBox,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::TextBox::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7TextBoxEFvbEN3rbx6signalIS3_EEMS2_S6_ED1Ev")]
pub fn stub_6685cc() {
    // IDA 0x6685cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x6685f0 — __ZNK3RBX12GuiTextMixin7getTextEv
// demangled: RBX::GuiTextMixin::getText(void)const
// type: _DWORD __fastcall(RBX::GuiTextMixin *__hidden this)
#[doc(alias = "RBX::GuiTextMixin::getText(void)const")]
#[doc(alias = "__ZNK3RBX12GuiTextMixin7getTextEv")]
pub fn stub_6685f0(state: &TextBoxState) -> String {
    // IDA 0x6685f0 (`RBX::GuiTextMixin::getText`): copies the
    // mixin +4 string (0x6685fa) — the `TextBox` +540 `Text`.
    state.text.clone()
}


// 0x6685fc — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxESsED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,std::string>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,std::string>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextBoxESsED1Ev")]
pub fn stub_6685fc() {
    // IDA 0x6685fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x668620 — __ZNK3RBX12GuiTextMixin11getFontSizeEv
// demangled: RBX::GuiTextMixin::getFontSize(void)const
// type: _DWORD __fastcall(RBX::GuiTextMixin *__hidden this)
#[doc(alias = "RBX::GuiTextMixin::getFontSize(void)const")]
#[doc(alias = "__ZNK3RBX12GuiTextMixin11getFontSizeEv")]
pub fn stub_668620(state: &TextBoxState) -> u32 {
    // IDA 0x668620 (`RBX::GuiTextMixin::getFontSize`): returns
    // mixin word 2 (0x668622) — the `TextBox` +544 `FontSize` id.
    state.font_size
}


// 0x668624 — __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEED1Ev
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::~EnumPropDescriptor()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEED1Ev")]
pub fn stub_668624() {
    // IDA 0x668624: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x668648 — __ZNK3RBX12GuiTextMixin7getFontEv
// demangled: RBX::GuiTextMixin::getFont(void)const
// type: _DWORD __fastcall(RBX::GuiTextMixin *__hidden this)
#[doc(alias = "RBX::GuiTextMixin::getFont(void)const")]
#[doc(alias = "__ZNK3RBX12GuiTextMixin7getFontEv")]
pub fn stub_668648(state: &TextBoxState) -> u32 {
    // IDA 0x668648 (`RBX::GuiTextMixin::getFont`): returns mixin
    // word 14 (0x66864a) — the `TextBox` +592 `Font` id.
    state.font
}


// 0x66864c — __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEED1Ev
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::~EnumPropDescriptor()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEED1Ev")]
pub fn stub_66864c() {
    // IDA 0x66864c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x668670 — __ZNK3RBX12GuiTextMixin12getTextColorEv
// demangled: RBX::GuiTextMixin::getTextColor(void)const
// type: _DWORD __fastcall(RBX::GuiTextMixin *__hidden this)
#[doc(alias = "RBX::GuiTextMixin::getTextColor(void)const")]
#[doc(alias = "__ZNK3RBX12GuiTextMixin12getTextColorEv")]
pub fn stub_668670(state: &TextBoxState) -> u32 {
    // IDA 0x668670 (`RBX::GuiTextMixin::getTextColor`):
    // `BrickColor::closest` over the mixin words 3-5 color3
    // (0x668678-0x66868a). The recompute needs the runtime
    // `BrickMap` palette (same gap as `setTextColor` at
    // 0x665fcc): host returns the cached `TextColor` id.
    state.text_color
}


// 0x66868c — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEED1Ev")]
pub fn stub_66868c() {
    // IDA 0x66868c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x6686b0 — __ZNK3RBX12GuiTextMixin13getTextColor3Ev
// demangled: RBX::GuiTextMixin::getTextColor3(void)const
// type: _DWORD __fastcall(RBX::GuiTextMixin *__hidden this)
#[doc(alias = "RBX::GuiTextMixin::getTextColor3(void)const")]
#[doc(alias = "__ZNK3RBX12GuiTextMixin13getTextColor3Ev")]
pub fn stub_6686b0(state: &TextBoxState) -> [f32; 3] {
    // IDA 0x6686b0 (`RBX::GuiTextMixin::getTextColor3`): copies
    // mixin words 3-5 (0x6686b2-0x6686ba) — the `TextBox`
    // +548/+552/+556 `TextColor3`.
    state.text_color3
}


// 0x6686c0 — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D6Color3EED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Color3>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Color3>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D6Color3EED1Ev")]
pub fn stub_6686c0() {
    // IDA 0x6686c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x6686e4 — __ZNK3RBX12GuiTextMixin19getTextTransparencyEv
// demangled: RBX::GuiTextMixin::getTextTransparency(void)const
// type: _DWORD __fastcall(RBX::GuiTextMixin *__hidden this)
#[doc(alias = "RBX::GuiTextMixin::getTextTransparency(void)const")]
#[doc(alias = "__ZNK3RBX12GuiTextMixin19getTextTransparencyEv")]
pub fn stub_6686e4(state: &TextBoxState) -> f32 {
    // IDA 0x6686e4 (`RBX::GuiTextMixin::getTextTransparency`):
    // returns mixin word 6 (0x6686e6) — the `TextBox` +560
    // `TextTransparency`.
    state.text_transparency
}


// 0x6686e8 — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEfED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,float>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,float>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEfED1Ev")]
pub fn stub_6686e8() {
    // IDA 0x6686e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x66870c — __ZNK3RBX12GuiTextMixin11getTextWrapEv
// demangled: RBX::GuiTextMixin::getTextWrap(void)const
// type: _DWORD __fastcall(RBX::GuiTextMixin *__hidden this)
#[doc(alias = "RBX::GuiTextMixin::getTextWrap(void)const")]
#[doc(alias = "__ZNK3RBX12GuiTextMixin11getTextWrapEv")]
pub fn stub_66870c(state: &TextBoxState) -> bool {
    // IDA 0x66870c (`RBX::GuiTextMixin::getTextWrap`): returns
    // mixin byte 44 (0x668710) — the `TextBox` +580 `TextWrap`.
    state.text_wrap
}


// 0x668714 — __ZNK3RBX12GuiTextMixin12getTextScaleEv
// demangled: RBX::GuiTextMixin::getTextScale(void)const
// type: _DWORD __fastcall(RBX::GuiTextMixin *__hidden this)
#[doc(alias = "RBX::GuiTextMixin::getTextScale(void)const")]
#[doc(alias = "__ZNK3RBX12GuiTextMixin12getTextScaleEv")]
pub fn stub_668714(state: &TextBoxState) -> bool {
    // IDA 0x668714 (`RBX::GuiTextMixin::getTextScale`): returns
    // mixin byte 45 (0x668718) — the `TextBox` +581 `TextScaled`.
    state.text_scaled
}


// 0x66871c — __ZNK3RBX12GuiTextMixin13getXAlignmentEv
// demangled: RBX::GuiTextMixin::getXAlignment(void)const
// type: _DWORD __fastcall(RBX::GuiTextMixin *__hidden this)
#[doc(alias = "RBX::GuiTextMixin::getXAlignment(void)const")]
#[doc(alias = "__ZNK3RBX12GuiTextMixin13getXAlignmentEv")]
pub fn stub_66871c(state: &TextBoxState) -> u32 {
    // IDA 0x66871c (`RBX::GuiTextMixin::getXAlignment`): returns
    // mixin word 12 (0x66871e) — the `TextBox` +584 alignment id.
    state.x_alignment
}


// 0x668720 — __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEED1Ev
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::~EnumPropDescriptor()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEED1Ev")]
pub fn stub_668720() {
    // IDA 0x668720: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x668744 — __ZNK3RBX12GuiTextMixin13getYAlignmentEv
// demangled: RBX::GuiTextMixin::getYAlignment(void)const
// type: _DWORD __fastcall(RBX::GuiTextMixin *__hidden this)
#[doc(alias = "RBX::GuiTextMixin::getYAlignment(void)const")]
#[doc(alias = "__ZNK3RBX12GuiTextMixin13getYAlignmentEv")]
pub fn stub_668744(state: &TextBoxState) -> u32 {
    // IDA 0x668744 (`RBX::GuiTextMixin::getYAlignment`): returns
    // mixin word 13 (0x668746) — the `TextBox` +588 alignment id.
    state.y_alignment
}


// 0x668748 — __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEED1Ev
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::~EnumPropDescriptor()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEED1Ev")]
pub fn stub_668748() {
    // IDA 0x668748: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x66876c — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D7Vector2EED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Vector2>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Vector2>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D7Vector2EED1Ev")]
pub fn stub_66876c() {
    // IDA 0x66876c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x668790 — __ZNK3RBX12GuiTextMixin19getTextStrokeColor3Ev
// demangled: RBX::GuiTextMixin::getTextStrokeColor3(void)const
// type: _DWORD __fastcall(RBX::GuiTextMixin *__hidden this)
#[doc(alias = "RBX::GuiTextMixin::getTextStrokeColor3(void)const")]
#[doc(alias = "__ZNK3RBX12GuiTextMixin19getTextStrokeColor3Ev")]
pub fn stub_668790(state: &TextBoxState) -> [f32; 3] {
    // IDA 0x668790 (`RBX::GuiTextMixin::getTextStrokeColor3`):
    // copies mixin words 7-9 (0x668792-0x66879a) — the `TextBox`
    // +564/+568/+572 `TextStrokeColor3`.
    state.text_stroke_color3
}


// 0x6687a0 — __ZNK3RBX12GuiTextMixin25getTextStrokeTransparencyEv
// demangled: RBX::GuiTextMixin::getTextStrokeTransparency(void)const
// type: _DWORD __fastcall(RBX::GuiTextMixin *__hidden this)
#[doc(alias = "RBX::GuiTextMixin::getTextStrokeTransparency(void)const")]
#[doc(alias = "__ZNK3RBX12GuiTextMixin25getTextStrokeTransparencyEv")]
pub fn stub_6687a0(state: &TextBoxState) -> f32 {
    // IDA 0x6687a0 (`RBX::GuiTextMixin::getTextStrokeTransparency`):
    // returns mixin word 10 (0x6687a2) — the `TextBox` +576
    // `TextStrokeTransparency`.
    state.text_stroke_transparency
}


// 0x6687a4 — __ZN3RBX9GuiObject21setTransparencyLegacyEf
// demangled: RBX::GuiObject::setTransparencyLegacy(float)
// type: _DWORD __fastcall(RBX::GuiObject *__hidden this, float)
#[doc(alias = "RBX::GuiObject::setTransparencyLegacy(float)")]
#[doc(alias = "__ZN3RBX9GuiObject21setTransparencyLegacyEf")]
pub fn stub_6687a4() {
    // IDA 0x6687a4 (`RBX::GuiObject::setTransparencyLegacy`,
    // thunk): forwards straight to `setBackgroundTransparency`
    // (0x6687a4) — GuiObject-layer only, no `TextBox`-member
    // effect (unlike the `TextBox` override at 0x6665a4).
    // Carrier no-op.
}


// 0x6687a8 — __ZN3RBX15ServiceProvider6createINS_11TextServiceEEEPT_PKNS_8InstanceE
// demangled: RBX::TextService * RBX::ServiceProvider::create<RBX::TextService>(RBX::Instance const*)
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::TextService * RBX::ServiceProvider::create<RBX::TextService>(RBX::Instance const*)")]
#[doc(alias = "__ZN3RBX15ServiceProvider6createINS_11TextServiceEEEPT_PKNS_8InstanceE")]
pub fn stub_6687a8() -> bool {
    // IDA 0x6687a8 (`RBX::ServiceProvider::create<RBX::TextService>`):
    // resolves the provider (0x6687ac) and recurses, else
    // returns 0 (0x6687b4). Service-locator plumbing with no host
    // provider: the exact no-provider floor.
    false
}


// 0x6687c0 — __ZN3RBX9GuiObject15convertFontSizeENS_11TextService8FontSizeE
// demangled: RBX::GuiObject::convertFontSize(RBX::TextService::FontSize)
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::GuiObject::convertFontSize(RBX::TextService::FontSize)")]
#[doc(alias = "__ZN3RBX9GuiObject15convertFontSizeENS_11TextService8FontSizeE")]
pub fn stub_6687c0(size: u32) -> f32 {
    // IDA 0x6687c0 (`RBX::GuiObject::convertFontSize`): the
    // `FontSize`-id to point-size table — 0..=9 map to 8, 9, 10,
    // 11, 12, 14, 18, 24, 36, 48 (0x668824-0x668866); anything
    // else asserts (`GuiObject.h:141`) and yields 0.0
    // (0x6687cc-0x66881e, host: silent floor per the crate's
    // assert-folding convention).
    match size {
        0 => 8.0,
        1 => 9.0,
        2 => 10.0,
        3 => 11.0,
        4 => 12.0,
        5 => 14.0,
        6 => 18.0,
        7 => 24.0,
        8 => 36.0,
        9 => 48.0,
        _ => 0.0,
    }
}


// 0x668878 — __ZNK3RBX9GuiObject21getPersistentDataCostEv
// demangled: RBX::GuiObject::getPersistentDataCost(void)const
// type: _DWORD __fastcall(RBX::GuiObject *__hidden this)
#[doc(alias = "RBX::GuiObject::getPersistentDataCost(void)const")]
#[doc(alias = "__ZNK3RBX9GuiObject21getPersistentDataCostEv")]
pub fn stub_668878(base: i32) -> i32 {
    // IDA 0x668878 (`RBX::GuiObject::getPersistentDataCost`): the
    // `Instance` base cost (0x668882, host: `base`) plus 6.
    base + 6
}


// 0x668884 — __ZN3rbx7signals6signalIFvPKcbEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvN3RBX7TextBoxES3_bEENS8_5list3INS8_5valueIPSD_EENS7_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_
// demangled: rbx::signals::connection rbx::signals::signal<void ()(char const*,bool)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>> const&)
// type: int(void)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(char const*,bool)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPKcbEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvN3RBX7TextBoxES3_bEENS8_5list3INS8_5valueIPSD_EENS7_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_668884() {
    // IDA 0x668884 (`rbx::signals::signal<void (char
    // const*,bool)>::connect` for the `externalReleaseFocus`
    // bind): allocates the slot, inserts it into the signal and
    // hands back the connection (0x66889c-0x6688f4). Connection
    // management folds into the host fire-closure seams. Carrier
    // no-op.
}


// 0x6688f8 — __ZN3rbx7signals16signal_with_argsILi1EFvN5boost10shared_ptrIN3RBX7TextBoxEEEEEclES6_
// demangled: rbx::signals::signal_with_args<1,void ()(boost::shared_ptr<RBX::TextBox>)>::operator()(boost::shared_ptr<RBX::TextBox>)
// type: int(void)
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::operator()(rbx_core::SharedPtr<RBX::TextBox>)")]
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFvN5boost10shared_ptrIN3RBX7TextBoxEEEEEclES6_")]
pub fn stub_6688f8() {
    // IDA 0x6688f8 (`rbx::signals::signal_with_args<1, void
    // (shared_ptr<TextBox>)>::operator()`): slot iteration and
    // invocation for the `Focused` fire (0x66890a-0x668ab4). The
    // fire itself is the host closure (`capture/gainFocus`).
    // Carrier no-op.
}


// 0x668adc — __ZN3RBX11shared_fromINS_7TextBoxEEEN5boost10shared_ptrIT_EEPS4_
// demangled: boost::shared_ptr<RBX::TextBox> RBX::shared_from<RBX::TextBox>(RBX::TextBox*)
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::TextBox> RBX::shared_from<RBX::TextBox>(RBX::TextBox*)")]
#[doc(alias = "__ZN3RBX11shared_fromINS_7TextBoxEEEN5boost10shared_ptrIT_EEPS4_")]
pub fn stub_668adc() {
    // IDA 0x668adc (`RBX::shared_from<RBX::TextBox>`): lifts the
    // weak `enable_shared_from_this` to a `shared_ptr`
    // (0x668b2a-0x668ba0), throwing `bad_weak_ptr` when expired
    // (0x668bde-0x668bf8). `SharedPtr` is `Arc` in the host and
    // the pre-fire lift folds into the fire-closure seams (the
    // object is borrowed live). Carrier no-op.
}


// 0x668c4c — __ZN3RBX7TextBoxD1Ev
// demangled: RBX::TextBox::~TextBox()
// type: void __fastcall(RBX::TextBox *__hidden this)
#[doc(alias = "RBX::TextBox::~TextBox()")]
#[doc(alias = "__ZN3RBX7TextBoxD1Ev")]
pub fn stub_668c4c() {
    // IDA 0x668c4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x668c50 — __ZN3RBX7TextBoxD0Ev
// demangled: RBX::TextBox::~TextBox()
// type: void __fastcall(RBX::TextBox *__hidden this)
#[doc(alias = "RBX::TextBox::~TextBox()")]
#[doc(alias = "__ZN3RBX7TextBoxD0Ev")]
pub fn stub_668c50() {
    // IDA 0x668c50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x668d00 — __ZNK3RBX9GuiObject26canProcessMeAndDescendantsEv
// demangled: RBX::GuiObject::canProcessMeAndDescendants(void)const
// type: _DWORD __fastcall(RBX::GuiObject *__hidden this)
#[doc(alias = "RBX::GuiObject::canProcessMeAndDescendants(void)const")]
#[doc(alias = "__ZNK3RBX9GuiObject26canProcessMeAndDescendantsEv")]
pub fn stub_668d00(can_process: bool) -> bool {
    // IDA 0x668d00 (`RBX::GuiObject::canProcessMeAndDescendants`):
    // returns the +512 byte (0x668d04) — a `GuiObject`-base cell
    // outside the modeled `TextBox` members. Host: pass-through
    // seam for the base flag.
    can_process
}


// 0x668d08 — __ZNK3RBX9GuiBase2d9getZIndexEv
// demangled: RBX::GuiBase2d::getZIndex(void)const
// type: _DWORD __fastcall(RBX::GuiBase2d *__hidden this)
#[doc(alias = "RBX::GuiBase2d::getZIndex(void)const")]
#[doc(alias = "__ZNK3RBX9GuiBase2d9getZIndexEv")]
pub fn stub_668d08(z_index: i32) -> i32 {
    // IDA 0x668d08 (`RBX::GuiBase2d::getZIndex`): returns word 34
    // (0x668d0c) — a `GuiBase2d`-base cell outside the modeled
    // `TextBox` members. Host: pass-through seam for the base
    // field.
    z_index
}


// 0x668d10 — __ZNK3RBX9GuiBase2d11getGuiQueueEv
// demangled: RBX::GuiBase2d::getGuiQueue(void)const
// type: _DWORD __fastcall(RBX::GuiBase2d *__hidden this)
#[doc(alias = "RBX::GuiBase2d::getGuiQueue(void)const")]
#[doc(alias = "__ZNK3RBX9GuiBase2d11getGuiQueueEv")]
pub fn stub_668d10(gui_queue: i32) -> i32 {
    // IDA 0x668d10 (`RBX::GuiBase2d::getGuiQueue`): returns word
    // 35 (0x668d14) — a `GuiBase2d`-base cell outside the modeled
    // `TextBox` members. Host: pass-through seam.
    gui_queue
}


// 0x668d18 — __ZNK3RBX9GuiBase2d9isGuiLeafEv
// demangled: RBX::GuiBase2d::isGuiLeaf(void)const
// type: _DWORD __fastcall(RBX::GuiBase2d *__hidden this)
#[doc(alias = "RBX::GuiBase2d::isGuiLeaf(void)const")]
#[doc(alias = "__ZNK3RBX9GuiBase2d9isGuiLeafEv")]
pub fn stub_668d18() -> bool {
    // IDA 0x668d18 (`RBX::GuiBase2d::isGuiLeaf`): returns constant
    // 0 (0x668d1a).
    false
}


// 0x668d1c — __ZNK3RBX9GuiBase2d14getChildRect2DEv
// demangled: RBX::GuiBase2d::getChildRect2D(void)const
// type: _DWORD __fastcall(RBX::GuiBase2d *__hidden this)
#[doc(alias = "RBX::GuiBase2d::getChildRect2D(void)const")]
#[doc(alias = "__ZNK3RBX9GuiBase2d14getChildRect2DEv")]
pub fn stub_668d1c() {
    // IDA 0x668d1c (`RBX::GuiBase2d::getChildRect2D`): forwards to
    // `getRect2D` (0x668d24) — viewport geometry with no modeled
    // cells. Carrier no-op.
}


// 0x668d28 — __ZNK3RBX9GuiBase2d14shouldRender2dEv
// demangled: RBX::GuiBase2d::shouldRender2d(void)const
// type: _DWORD __fastcall(RBX::GuiBase2d *__hidden this)
#[doc(alias = "RBX::GuiBase2d::shouldRender2d(void)const")]
#[doc(alias = "__ZNK3RBX9GuiBase2d14shouldRender2dEv")]
pub fn stub_668d28() -> bool {
    // IDA 0x668d28 (`RBX::GuiBase2d::shouldRender2d`): returns
    // constant 0 (0x668d2a).
    false
}


// 0x668d2c — __ZNK3RBX9GuiBase2d9isVisibleERKN3G3D6Rect2DE
// demangled: RBX::GuiBase2d::isVisible(G3D::Rect2D const&)const
// type: int(void)
#[doc(alias = "RBX::GuiBase2d::isVisible(G3D::Rect2D const&)const")]
#[doc(alias = "__ZNK3RBX9GuiBase2d9isVisibleERKN3G3D6Rect2DE")]
pub fn stub_668d2c(other: &[f32; 4], own: &[f32; 4]) -> bool {
    // IDA 0x668d2c (`RBX::GuiBase2d::isVisible`): the `getRect2D`
    // rect (0x668d38, host: `own` — viewport seam) overlapped
    // against the input rect (0x668d4e-0x668d8a): `other[0] <
    // own[2] && other[1] < own[3] && other[2] > own[0] &&
    // other[3] > own[1]`.
    other[0] < own[2] && other[1] < own[3] && other[2] > own[0] && other[3] > own[1]
}


// 0x668d90 — __ZThn32_N3RBX7TextBoxD1Ev
// demangled: non-virtual thunk toRBX::TextBox::~TextBox()
// type: void __fastcall(RBX::TextBox *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::TextBox::~TextBox()")]
#[doc(alias = "__ZThn32_N3RBX7TextBoxD1Ev")]
pub fn stub_668d90() {
    // IDA 0x668d90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x668d98 — __ZThn32_N3RBX7TextBoxD0Ev
// demangled: non-virtual thunk toRBX::TextBox::~TextBox()
// type: void __fastcall(RBX::TextBox *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::TextBox::~TextBox()")]
#[doc(alias = "__ZThn32_N3RBX7TextBoxD0Ev")]
pub fn stub_668d98() {
    // IDA 0x668d98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x668e4c — __ZThn36_N3RBX7TextBoxD1Ev
// demangled: non-virtual thunk toRBX::TextBox::~TextBox()
// type: void __fastcall(RBX::TextBox *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::TextBox::~TextBox()")]
#[doc(alias = "__ZThn36_N3RBX7TextBoxD1Ev")]
pub fn stub_668e4c() {
    // IDA 0x668e4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x668e54 — __ZThn36_N3RBX7TextBoxD0Ev
// demangled: non-virtual thunk toRBX::TextBox::~TextBox()
// type: void __fastcall(RBX::TextBox *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::TextBox::~TextBox()")]
#[doc(alias = "__ZThn36_N3RBX7TextBoxD0Ev")]
pub fn stub_668e54() {
    // IDA 0x668e54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x668ef8 — __ZThn96_NK3RBX9GuiBase2d14shouldRender2dEv
// demangled: non-virtual thunk toRBX::GuiBase2d::shouldRender2d(void)const
// type: _DWORD __fastcall(RBX::GuiBase2d *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::GuiBase2d::shouldRender2d(void)const")]
#[doc(alias = "__ZThn96_NK3RBX9GuiBase2d14shouldRender2dEv")]
pub fn stub_668ef8() {
    // IDA 0x668ef8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x668efc — __ZThn96_NK3RBX9GuiBase2d9isVisibleERKN3G3D6Rect2DE
// demangled: non-virtual thunk toRBX::GuiBase2d::isVisible(G3D::Rect2D const&)const
#[doc(alias = "non-virtual thunk toRBX::GuiBase2d::isVisible(G3D::Rect2D const&)const")]
#[doc(alias = "__ZThn96_NK3RBX9GuiBase2d9isVisibleERKN3G3D6Rect2DE")]
pub fn stub_668efc() {
    // IDA 0x668efc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x668f08 — __ZThn596_N3RBX7TextBoxD1Ev
// demangled: non-virtual thunk toRBX::TextBox::~TextBox()
// type: void __fastcall(RBX::TextBox *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::TextBox::~TextBox()")]
#[doc(alias = "__ZThn596_N3RBX7TextBoxD1Ev")]
pub fn stub_668f08() {
    // IDA 0x668f08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x668f10 — __ZThn596_N3RBX7TextBoxD0Ev
// demangled: non-virtual thunk toRBX::TextBox::~TextBox()
// type: void __fastcall(RBX::TextBox *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::TextBox::~TextBox()")]
#[doc(alias = "__ZThn596_N3RBX7TextBoxD0Ev")]
pub fn stub_668f10() {
    // IDA 0x668f10: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x669220 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_7TextBoxEEEN5boost10shared_ptrIT_EEv
// demangled: boost::shared_ptr<RBX::TextBox> RBX::Creatable<RBX::Instance>::create<RBX::TextBox>(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::TextBox> RBX::Creatable<RBX::Instance>::create<RBX::TextBox>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_7TextBoxEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_669220() -> TextBoxState {
    // IDA 0x669220 (`RBX::Creatable<Instance>::create<TextBox>`):
    // heap-allocates (0x669256), runs the `TextBox` C2 (0x66927a,
    // host: the 0x666938 twin) and wraps it in the
    // `shared_ptr`+`Deleter` (0x669288). Host: the fresh state
    // (`SharedPtr` is `Arc` — ownership folds).
    TextBoxState::default()
}


// 0x6692d4 — __ZN5boost10shared_ptrIN3RBX7TextBoxEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// demangled: boost::shared_ptr<RBX::TextBox>::shared_ptr<RBX::TextBox,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TextBox *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::TextBox>::shared_ptr<RBX::TextBox,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TextBox *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX7TextBoxEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_6692d4() {
    // IDA 0x6692d4 (`boost::shared_ptr<TextBox>::shared_ptr` with
    // the `Creatable::Deleter`): installs the count and accepts
    // the `enable_shared_from_this` owner (0x6692f2-0x66935c).
    // `SharedPtr` is `Arc` in the host. Carrier no-op.
}


// 0x66939c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7TextBoxES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TextBox,RBX::TextBox>(boost::shared_ptr<RBX::TextBox> const*,RBX::TextBox *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TextBox,RBX::TextBox>(rbx_core::SharedPtr<RBX::TextBox> const*,RBX::TextBox *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7TextBoxES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_66939c() {
    // IDA 0x66939c: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}


// 0x669484 — __ZN5boost6detail12shared_countC2IPN3RBX7TextBoxENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// demangled: boost::detail::shared_count::shared_count<RBX::TextBox *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TextBox *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TextBox *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TextBox *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX7TextBoxENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_669484() {
    // IDA 0x669484: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}


// 0x66958c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextBoxENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::TextBox *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextBox *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextBoxENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_66958c() {
    // IDA 0x66958c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x669590 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextBoxENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::TextBox *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextBox *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextBoxENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_669590() {
    // IDA 0x669590: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x669594 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextBoxENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::TextBox *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextBox *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextBoxENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_669594() {
    // IDA 0x669594: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}


// 0x6695b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextBoxENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_pd<RBX::TextBox *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextBox *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextBoxENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_6695b4() {
    // IDA 0x6695b4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}


// 0x6695cc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextBoxENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::TextBox *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextBox *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextBoxENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_6695cc() {
    // IDA 0x6695cc: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}


// 0x66996c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4nextERNS2_13intrusive_ptrINS8_4slotEEE
// demangled: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot> &)
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot> &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4nextERNS2_13intrusive_ptrINS8_4slotEEE")]
pub fn stub_66996c() {
    // IDA 0x66996c (`rbx::signals::signal<shared_ptr<TextBox>>::
    // next`): locked slot iteration for the `Focused` emission
    // (0x66996c+). Dispatch folds into the host fire closures.
    // Carrier no-op.
}


// 0x669acc — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE8on_errorERSt9exception
// demangled: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::on_error(std::exception &)
// type: int(void)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::on_error(std::exception &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE8on_errorERSt9exception")]
pub fn stub_669acc() {
    // IDA 0x669acc (`rbx::signals::signal<shared_ptr<TextBox>>::
    // on_error`): returns the slot exception handler
    // (0x669ae0-0x669af2). Carrier no-op.
}


// 0x669af4 — __ZN3rbx7signals6signalIFvPKcbEE6insertEPNS5_4slotE
// demangled: rbx::signals::signal<void ()(char const*,bool)>::insert(rbx::signals::signal<void ()(char const*,bool)>::slot *)
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(char const*,bool)>::insert(rbx::signals::signal<void ()(char const*,bool)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPKcbEE6insertEPNS5_4slotE")]
pub fn stub_669af4() {
    // IDA 0x669af4 (`rbx::signals::signal<void (char
    // const*,bool)>::insert`): locked slot-list insert with
    // refcount handoff (0x669af4-0x669c82). Connection
    // management folds into the host fire-closure seams. Carrier
    // no-op.
}


// 0x669d00 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPKcbEE4slotEEaSEPS8_
// demangled: boost::intrusive_ptr<rbx::signals::signal<void ()(char const*,bool)>::slot>::operator=(rbx::signals::signal<void ()(char const*,bool)>::slot*)
// type: int(void)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(char const*,bool)>::slot>::operator=(rbx::signals::signal<void ()(char const*,bool)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPKcbEE4slotEEaSEPS8_")]
pub fn stub_669d00() {
    // IDA 0x669d00 (`boost::intrusive_ptr<signal slot>::operator=`):
    // addref-new/release-old (0x669d0a-0x669d1a). `Arc` move —
    // carrier no-op.
}


// 0x669d24 — __ZN3rbx7signals6signalIFvPKcbEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvN3RBX7TextBoxES3_bEENS8_5list3INS8_5valueIPSD_EENS7_3argILi1EEENSJ_ILi2EEEEEEEED1Ev
// demangled: rbx::signals::signal<void ()(char const*,bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()
#[doc(alias = "rbx::signals::signal<void ()(char const*,bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPKcbEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvN3RBX7TextBoxES3_bEENS8_5list3INS8_5valueIPSD_EENS7_3argILi1EEENSJ_ILi2EEEEEEEED1Ev")]
pub fn stub_669d24() {
    // IDA 0x669d24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x669d50 — __ZN3rbx7signals6signalIFvPKcbEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvN3RBX7TextBoxES3_bEENS8_5list3INS8_5valueIPSD_EENS7_3argILi1EEENSJ_ILi2EEEEEEEED0Ev
// demangled: rbx::signals::signal<void ()(char const*,bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()
#[doc(alias = "rbx::signals::signal<void ()(char const*,bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPKcbEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvN3RBX7TextBoxES3_bEENS8_5list3INS8_5valueIPSD_EENS7_3argILi1EEENSJ_ILi2EEEEEEEED0Ev")]
pub fn stub_669d50() {
    // IDA 0x669d50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x669e24 — __ZN3rbx7signals6signalIFvPKcbEE4slot10disconnectEv
// demangled: rbx::signals::signal<void ()(char const*,bool)>::slot::disconnect(void)
#[doc(alias = "rbx::signals::signal<void ()(char const*,bool)>::slot::disconnect(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPKcbEE4slot10disconnectEv")]
pub fn stub_669e24() {
    // IDA 0x669e24 (`rbx::signals::signal<void (char
    // const*,bool)>::slot::disconnect`): clears the slot's +12
    // cell and removes it from the signal (0x669eb4-0x669ec2).
    // Connections fold into the host fire-closure seams. Carrier
    // no-op.
}


// 0x669f34 — __ZNK3rbx7signals6signalIFvPKcbEE4slot9connectedEv
// demangled: rbx::signals::signal<void ()(char const*,bool)>::slot::connected(void)const
#[doc(alias = "rbx::signals::signal<void ()(char const*,bool)>::slot::connected(void)const")]
#[doc(alias = "__ZNK3rbx7signals6signalIFvPKcbEE4slot9connectedEv")]
pub fn stub_669f34() -> bool {
    // IDA 0x669f34 (`rbx::signals::signal<void (char
    // const*,bool)>::slot::connected`): the slot's +12 cell is
    // nonzero (0x669f3c). No connection is ever modeled in the
    // host: the exact unconnected floor.
    false
}


// 0x669f40 — __ZN3rbx8callableINS_7signals6signalIFvPKcbEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvN3RBX7TextBoxES4_bEENS9_5list3INS9_5valueIPSE_EENS8_3argILi1EEENSK_ILi2EEEEEEELi2ES5_E4callES4_b
// demangled: rbx::callable<rbx::signals::signal<void ()(char const*,bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>,2,void ()(char const*,bool)>::call(char const*,bool)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(char const*,bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>,2,void ()(char const*,bool)>::call(char const*,bool)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvPKcbEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvN3RBX7TextBoxES4_bEENS9_5list3INS9_5valueIPSE_EENS8_3argILi1EEENSK_ILi2EEEEEEELi2ES5_E4callES4_b")]
pub fn stub_669f40(
    state: &mut TextBoxState,
    text: &str,
    submitted: bool,
    filter_pass: bool,
    fire_focused: impl Fn(bool),
) {
    // IDA 0x669f40 (`rbx::callable<... TextBox::externalReleaseFocus
    // bind ...>::call`): repacks the `(char const*, bool)` signal
    // args (0x669f46-0x669f56) and dispatches through the bind
    // (0x669f66, host: the 0x669f90 list-forward folds into this
    // edge) into `externalReleaseFocus` — host: the 0x666e84 twin.
    stub_666e84(state, text, submitted, filter_pass, fire_focused);
}


// 0x669f68 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPKcbEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvN3RBX7TextBoxES4_bEENS9_5list3INS9_5valueIPSE_EENS8_3argILi1EEENSK_ILi2EEEEEEELi2ES5_E4callES4_b
// demangled: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(char const*,bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>,2,void ()(char const*,bool)>::call(char const*,bool)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(char const*,bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>,2,void ()(char const*,bool)>::call(char const*,bool)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvPKcbEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvN3RBX7TextBoxES4_bEENS9_5list3INS9_5valueIPSE_EENS8_3argILi1EEENSK_ILi2EEEEEEELi2ES5_E4callES4_b")]
pub fn stub_669f68() {
    // IDA 0x669f68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x669f90 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX7TextBoxEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_PKcbEENS0_5list2IRSF_RbEEEEvNS0_4typeIvEERT_RT0_i
// demangled: void boost::_bi::list3<boost::_bi::value<RBX::TextBox *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list2<char const*&,bool &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool> &,boost::_bi::list2<char const*&,bool &> &,int)
// type: int(void)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::TextBox *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list2<char const*&,bool &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool> &,boost::_bi::list2<char const*&,bool &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueIPN3RBX7TextBoxEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_PKcbEENS0_5list2IRSF_RbEEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_669f90() {
    // IDA 0x669f90 (`boost::_bi::list3<TextBox*, arg<1>,
    // arg<2>>::operator()` for the `externalReleaseFocus` bind):
    // forwards the box plus `(char const*, bool)` into the member
    // (0x669f92-0x669fb0). Forwarding folds into the 0x669f40
    // dispatch edge. Carrier no-op.
}


// 0x669fbc — __ZN3rbx7signals6signalIFvPKcbEE6removeEPNS5_4slotE
// demangled: rbx::signals::signal<void ()(char const*,bool)>::remove(rbx::signals::signal<void ()(char const*,bool)>::slot *)
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(char const*,bool)>::remove(rbx::signals::signal<void ()(char const*,bool)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPKcbEE6removeEPNS5_4slotE")]
pub fn stub_669fbc() {
    // IDA 0x669fbc (`rbx::signals::signal<void (char
    // const*,bool)>::remove`): asserts the slot is live and
    // unlinks it (0x669fd0-0x66a094, `signal.h:284`). Connections
    // fold into the host fire-closure seams. Carrier no-op.
}


// 0x66a0ac — __ZN3rbx7signals6signalIFvPKcbEE4slot22safe_static_init_mutexEv
// demangled: rbx::signals::signal<void ()(char const*,bool)>::slot::safe_static_init_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(char const*,bool)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPKcbEE4slot22safe_static_init_mutexEv")]
pub fn stub_66a0ac() {
    // IDA 0x66a0ac (`rbx::signals::signal<void (char
    // const*,bool)>::slot::safe_static_init_mutex`): forwards to
    // the once-mutex getter (host: the 0x66a0b0 twin folds).
    // Carrier no-op.
}


// 0x66a0b0 — __ZN3rbx7signals6signalIFvPKcbEE4slot24safe_static_do_get_mutexEv
// demangled: rbx::signals::signal<void ()(char const*,bool)>::slot::safe_static_do_get_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(char const*,bool)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPKcbEE4slot24safe_static_do_get_mutexEv")]
pub fn stub_66a0b0() {
    // IDA 0x66a0b0 (`rbx::signals::signal<void (char
    // const*,bool)>::slot::safe_static_do_get_mutex`): once-guarded
    // static mutex init (0x66a10c-0x66a144). Host mutexes fold.
    // Carrier no-op.
}


// 0x66a1a0 — __ZN3rbx7signals6signalIFvPKcbEE4slotD1Ev
// demangled: rbx::signals::signal<void ()(char const*,bool)>::slot::~slot()
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(char const*,bool)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPKcbEE4slotD1Ev")]
pub fn stub_66a1a0() {
    // IDA 0x66a1a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x66a1cc — __ZN3rbx7signals6signalIFvPKcbEE4slotD0Ev
// demangled: rbx::signals::signal<void ()(char const*,bool)>::slot::~slot()
#[doc(alias = "rbx::signals::signal<void ()(char const*,bool)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPKcbEE4slotD0Ev")]
pub fn stub_66a1cc() {
    // IDA 0x66a1cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x66a2a0 — __ZN3rbx8callableINS_7signals6signalIFvPKcbEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvN3RBX7TextBoxES4_bEENS9_5list3INS9_5valueIPSE_EENS8_3argILi1EEENSK_ILi2EEEEEEELi2ES5_ED1Ev
// demangled: rbx::callable<rbx::signals::signal<void ()(char const*,bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>,2,void ()(char const*,bool)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(char const*,bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>,2,void ()(char const*,bool)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvPKcbEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvN3RBX7TextBoxES4_bEENS9_5list3INS9_5valueIPSE_EENS8_3argILi1EEENSK_ILi2EEEEEEELi2ES5_ED1Ev")]
pub fn stub_66a2a0() {
    // IDA 0x66a2a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x66a2cc — __ZN3rbx8callableINS_7signals6signalIFvPKcbEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvN3RBX7TextBoxES4_bEENS9_5list3INS9_5valueIPSE_EENS8_3argILi1EEENSK_ILi2EEEEEEELi2ES5_ED0Ev
// demangled: rbx::callable<rbx::signals::signal<void ()(char const*,bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>,2,void ()(char const*,bool)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(char const*,bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>,2,void ()(char const*,bool)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvPKcbEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvN3RBX7TextBoxES4_bEENS9_5list3INS9_5valueIPSE_EENS8_3argILi1EEENSK_ILi2EEEEEEELi2ES5_ED0Ev")]
pub fn stub_66a2cc() {
    // IDA 0x66a2cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x66a8b4 — __ZN3RBX9GuiObjectD2Ev
// demangled: RBX::GuiObject::~GuiObject()
// type: void __fastcall(RBX::GuiObject *__hidden this)
#[doc(alias = "RBX::GuiObject::~GuiObject()")]
#[doc(alias = "__ZN3RBX9GuiObjectD2Ev")]
pub fn stub_66a8b4() {
    // IDA 0x66a8b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x66ac8c — __ZN3RBX9GuiObjectD1Ev
// demangled: RBX::GuiObject::~GuiObject()
// type: void __fastcall(RBX::GuiObject *__hidden this)
#[doc(alias = "RBX::GuiObject::~GuiObject()")]
#[doc(alias = "__ZN3RBX9GuiObjectD1Ev")]
pub fn stub_66ac8c() {
    // IDA 0x66ac8c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x66ac90 — __ZN3RBX9GuiObjectD0Ev
// demangled: RBX::GuiObject::~GuiObject()
// type: void __fastcall(RBX::GuiObject *__hidden this)
#[doc(alias = "RBX::GuiObject::~GuiObject()")]
#[doc(alias = "__ZN3RBX9GuiObjectD0Ev")]
pub fn stub_66ac90() {
    // IDA 0x66ac90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x66ad34 — __ZThn32_N3RBX9GuiObjectD1Ev
// demangled: non-virtual thunk toRBX::GuiObject::~GuiObject()
// type: void __fastcall(RBX::GuiObject *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::GuiObject::~GuiObject()")]
#[doc(alias = "__ZThn32_N3RBX9GuiObjectD1Ev")]
pub fn stub_66ad34() {
    // IDA 0x66ad34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x66ad3c — __ZThn32_N3RBX9GuiObjectD0Ev
// demangled: non-virtual thunk toRBX::GuiObject::~GuiObject()
// type: void __fastcall(RBX::GuiObject *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::GuiObject::~GuiObject()")]
#[doc(alias = "__ZThn32_N3RBX9GuiObjectD0Ev")]
pub fn stub_66ad3c() {
    // IDA 0x66ad3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x66ade4 — __ZThn36_N3RBX9GuiObjectD1Ev
// demangled: non-virtual thunk toRBX::GuiObject::~GuiObject()
// type: void __fastcall(RBX::GuiObject *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::GuiObject::~GuiObject()")]
#[doc(alias = "__ZThn36_N3RBX9GuiObjectD1Ev")]
pub fn stub_66ade4() {
    // IDA 0x66ade4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x66adec — __ZThn36_N3RBX9GuiObjectD0Ev
// demangled: non-virtual thunk toRBX::GuiObject::~GuiObject()
// type: void __fastcall(RBX::GuiObject *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::GuiObject::~GuiObject()")]
#[doc(alias = "__ZThn36_N3RBX9GuiObjectD0Ev")]
pub fn stub_66adec() {
    // IDA 0x66adec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x66afbc — __ZN5boost10scoped_ptrIN3RBX9GuiObject5TweenEED2Ev
// demangled: boost::scoped_ptr<RBX::GuiObject::Tween>::~scoped_ptr()
#[doc(alias = "boost::scoped_ptr<RBX::GuiObject::Tween>::~scoped_ptr()")]
#[doc(alias = "__ZN5boost10scoped_ptrIN3RBX9GuiObject5TweenEED2Ev")]
pub fn stub_66afbc() {
    // IDA 0x66afbc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x66b068 — __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE5clearEv
// demangled: boost::function1<void,RBX::GuiObject::TweenStatus>::clear(void)
// type: int __fastcall(_DWORD)
#[doc(alias = "boost::function1<void,RBX::GuiObject::TweenStatus>::clear(void)")]
#[doc(alias = "__ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE5clearEv")]
pub fn stub_66b068() {
    // IDA 0x66b068: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}


// 0x66b094 — __ZN3rbx13remote_signalIFviiEED2Ev
// demangled: rbx::remote_signal<void ()(int,int)>::~remote_signal()
// type: int __fastcall(int, int, int, int, char, int)
#[doc(alias = "rbx::remote_signal<void ()(int,int)>::~remote_signal()")]
#[doc(alias = "__ZN3rbx13remote_signalIFviiEED2Ev")]
pub fn stub_66b094() {
    // IDA 0x66b094: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


// 0x66b1e0 — __ZN3rbx7signals6signalIFviiEE13disconnectAllEv
// demangled: rbx::signals::signal<void ()(int,int)>::disconnectAll(void)
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::disconnectAll(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFviiEE13disconnectAllEv")]
pub fn stub_66b1e0() {
    // IDA 0x66b1e0 (`rbx::signals::signal<void (int,int)>::
    // disconnectAll`): locked slot-list teardown (0x66b1e0+).
    // Connections fold into the host fire-closure seams. Carrier
    // no-op.
}


// 0x66b358 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviiEE4slotEEaSERKS7_
// demangled: boost::intrusive_ptr<rbx::signals::signal<void ()(int,int)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(int,int)>::slot> const&)
// type: int(void)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(int,int)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(int,int)>::slot> const&)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviiEE4slotEEaSERKS7_")]
pub fn stub_66b358() {
    // IDA 0x66b358 (`boost::intrusive_ptr<signal<void
    // (int,int)>::slot>::operator=`): addref-new/release-old
    // (0x66b35a-0x66b372). `Arc` move — carrier no-op.
}


// 0x66b37c — __ZN3rbx7signals6signalIFviiEE22safe_static_init_mutexEv
// demangled: rbx::signals::signal<void ()(int,int)>::safe_static_init_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFviiEE22safe_static_init_mutexEv")]
pub fn stub_66b37c() {
    // IDA 0x66b37c (`rbx::signals::signal<void (int,int)>::
    // safe_static_init_mutex`): forwards to the once-mutex getter
    // (host: the 0x66b380 twin folds). Carrier no-op.
}


// 0x66b380 — __ZN3rbx7signals6signalIFviiEE24safe_static_do_get_mutexEv
// demangled: rbx::signals::signal<void ()(int,int)>::safe_static_do_get_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFviiEE24safe_static_do_get_mutexEv")]
pub fn stub_66b380() {
    // IDA 0x66b380 (`rbx::signals::signal<void (int,int)>::
    // safe_static_do_get_mutex`): once-guarded static mutex init
    // (0x66b3dc-0x66b40c). Host mutexes fold. Carrier no-op.
}