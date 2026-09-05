//! audio generated_audio_wd_watchdog18 — 100 stubs EA-sorted asc gap filler not yet in audio (FMOD|Sound|Audio exhausted, global gap filler)
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 100 not in audio after 0x662800 | rbx_core::SharedPtr not boost
//! Range 0x66e380..0x670c04 | existing 37123 -> 37223 distinct
//! Batch: 100 stubs | // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR demangled")

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
use crate::generated::flog_asserts;
use crate::generated_134::{XmlIntSlot, XmlReadValue};
const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };


/// `EnumDesc<TextService::XAlignment>` items in `addPair` order
/// (IDA 0x7d8548: the `MOVS R1, #N` ahead of each call grounds the
/// values — note Center = 2 sorts after Right = 1).
pub const XALIGNMENT_ITEMS: [(&str, u32); 3] =
    [("Left", 0), ("Center", 2), ("Right", 1)];
/// Name of an `XAlignment` value for `convertToString` (IDA 0x66e380).
/// Values with no item yield "" — the writers only ever store table
/// members.
pub fn xalignment_name(value: u32) -> &'static str {
    XALIGNMENT_ITEMS
        .iter()
        .find(|(_, v)| *v == value)
        .map(|(n, _)| *n)
        .unwrap_or("")
}
/// Item index of an `XAlignment` value, -1 when it has no item (IDA
/// 0x65a544-shaped `convertToIndex` reads).
pub fn xalignment_index(value: u32) -> i32 {
    XALIGNMENT_ITEMS
        .iter()
        .position(|(_, v)| *v == value)
        .map(|i| i as i32)
        .unwrap_or(-1)
}
/// `placement_any` payload read by the `XAlignment` `any_cast` (IDA
/// 0x66eb78): the value or something else (miss throws
/// `bad_placement_any_cast`, host: panic).
#[derive(Debug, Clone, Copy)]
pub enum XAlignmentVariant {
    XAlignment(u32),
    Other,
}
/// `RBX::TextBox` cutover (IDA 0x666938/0x667144/0x667b30): the
/// `XAlignment` id (member `getXAlignment`, IDA 0x66871c; word 146,
/// +584 — C2 stores 2) and the `YAlignment` id (word 147, +588 —
/// C2 stores 1), the two bool members (`TextWrap`/`TextScaled`
/// bytes +580/+581; the deprecated `TextWrapped` alias shares the
/// `TextWrap` member — grounded: no `setTextWrapped` exists and
/// its descriptor carries the deprecated attributes, IDA
/// 0x67282c-0x672856), the two float members
/// (`TextTransparency` word 140, +560 / `TextStrokeTransparency`
/// word 144, +576 — C2 stores 0.0/1.0, IDA 0x6727de/0x672ae4),
/// the two `Color3` members (`TextColor3` words 137-139, +548 /
/// `TextStrokeColor3` +564/+568/+572, IDA 0x672782/0x672a94), the
/// `TextColor` `BrickColor` (IDA 0x672728; C2 selects palette
/// index 26), the `Font` id (word 148, +592 — read by the
/// typesetter calls at 0x66621a/0x6663a0/0x66665a) and the
/// `FontSize` id (word 136, +544), the `Text` (+540, C2 seeds
/// "TextBox") and focus-composition (+608) strings, the +620
/// compose cursor, the +612 focus time, the +628/+632/+636/
/// +640/+648 held-key (type, code, char, time, phase) repeat
/// driver, the +604 armed / +605 focused / +606 external-focus /
/// +607 clear-on-focus / +652 multi-line cells (C2: 0/0/0/1/0).
/// `Default` replays the C2-grounded values (a fresh `TextBox`).
#[derive(Debug, Clone)]
pub struct TextBoxState {
    pub x_alignment: u32,
    pub y_alignment: u32,
    pub text_wrap: bool,
    pub text_scaled: bool,
    pub text_transparency: f32,
    pub text_stroke_transparency: f32,
    pub text_color3: [f32; 3],
    pub text_stroke_color3: [f32; 3],
    pub text_color: u32,
    pub font: u32,
    pub font_size: u32,
    pub multi_line: bool,
    pub clear_text_on_focus: bool,
    pub focus_armed: bool,
    pub focused: bool,
    pub external_focus: bool,
    pub focus_text: String,
    pub text: String,
    pub cursor: usize,
    pub key_type: u32,
    pub key_code: u32,
    pub key_char: u8,
    pub key_phase: u32,
    pub key_time: f64,
    pub focus_time: f64,
}

impl Default for TextBoxState {
    fn default() -> Self {
        Self {
            x_alignment: 2,
            y_alignment: 1,
            text_wrap: false,
            text_scaled: false,
            text_transparency: 0.0,
            text_stroke_transparency: 1.0,
            text_color3: [0.0, 0.0, 0.0],
            text_stroke_color3: [0.0, 0.0, 0.0],
            text_color: 26,
            font: 0,
            font_size: 0,
            multi_line: false,
            clear_text_on_focus: true,
            focus_armed: false,
            focused: false,
            external_focus: false,
            focus_text: String::new(),
            text: "TextBox".to_owned(),
            cursor: 0,
            key_type: 0,
            key_code: 0,
            key_char: 0,
            key_phase: 0,
            key_time: 0.0,
            focus_time: 0.0,
        }
    }
}
/// Bool member selected by a `PropDescriptor<TextBox, bool>`'s
/// member-pointer pair (IDA 0x67283e-0x6728f6: three objects over
/// two members).
#[derive(Debug, Clone, Copy)]
pub enum TextBoxBoolSlot {
    TextWrap,
    TextScaled,
}
impl TextBoxState {
    pub fn bool_slot(&self, slot: TextBoxBoolSlot) -> bool {
        match slot {
            TextBoxBoolSlot::TextWrap => self.text_wrap,
            TextBoxBoolSlot::TextScaled => self.text_scaled,
        }
    }
    pub fn set_bool_slot(&mut self, slot: TextBoxBoolSlot, value: bool) {
        match slot {
            TextBoxBoolSlot::TextWrap => self.text_wrap = value,
            TextBoxBoolSlot::TextScaled => self.text_scaled = value,
        }
    }
}
/// Float member selected by a `PropDescriptor<TextBox, float>`'s
/// member-pointer pair (IDA 0x6727de/0x672ae4: two objects over two
/// members).
#[derive(Debug, Clone, Copy)]
pub enum TextBoxFloatSlot {
    TextTransparency,
    TextStrokeTransparency,
}
/// `Color3` member selected by a `PropDescriptor<TextBox, Color3>`'s
/// member-pointer pair (IDA 0x672782/0x672a94: two objects over two
/// members).
#[derive(Debug, Clone, Copy)]
pub enum TextBoxColorSlot {
    TextColor3,
    TextStrokeColor3,
}
impl TextBoxState {
    pub fn float_slot(&self, slot: TextBoxFloatSlot) -> f32 {
        match slot {
            TextBoxFloatSlot::TextTransparency => self.text_transparency,
            TextBoxFloatSlot::TextStrokeTransparency => self.text_stroke_transparency,
        }
    }
    pub fn set_float_slot(&mut self, slot: TextBoxFloatSlot, value: f32) {
        match slot {
            TextBoxFloatSlot::TextTransparency => self.text_transparency = value,
            TextBoxFloatSlot::TextStrokeTransparency => self.text_stroke_transparency = value,
        }
    }
    pub fn color_slot(&self, slot: TextBoxColorSlot) -> [f32; 3] {
        match slot {
            TextBoxColorSlot::TextColor3 => self.text_color3,
            TextBoxColorSlot::TextStrokeColor3 => self.text_stroke_color3,
        }
    }
    pub fn set_color_slot(&mut self, slot: TextBoxColorSlot, value: [f32; 3]) {
        match slot {
            TextBoxColorSlot::TextColor3 => self.text_color3 = value,
            TextBoxColorSlot::TextStrokeColor3 => self.text_stroke_color3 = value,
        }
    }
}
/// `RBX::Reflection::PropDescriptor<TextBox, bool>` cutover (IDA
/// 0x66ec90): name/category/attributes/permissions. The
/// getter/setter member-pointer pair folds into the slot selector.
#[derive(Debug, Clone)]
pub struct TextBoxBoolProp {
    pub name: String,
    pub category: String,
    pub attributes: u32,
    pub permissions: u32,
}
impl TextBoxBoolProp {
    pub fn new(name: &str, category: &str, attributes: u32, permissions: u32) -> Self {
        Self {
            name: name.to_owned(),
            category: category.to_owned(),
            attributes,
            permissions,
        }
    }
}
/// `RBX::Reflection::PropDescriptor<TextBox, float>` cutover (IDA
/// 0x66ee04): name/category/attributes/permissions. The
/// getter/setter member-pointer pair folds into the slot selector
/// (two objects: `TextTransparency`/`TextStrokeTransparency`, IDA
/// 0x6727de/0x672ae4).
#[derive(Debug, Clone)]
pub struct TextBoxFloatProp {
    pub name: String,
    pub category: String,
    pub attributes: u32,
    pub permissions: u32,
}
impl TextBoxFloatProp {
    pub fn new(name: &str, category: &str, attributes: u32, permissions: u32) -> Self {
        Self {
            name: name.to_owned(),
            category: category.to_owned(),
            attributes,
            permissions,
        }
    }
}
/// `EnumDesc<TextService::Font>` items in `addPair` order (IDA
/// 0x7d8340: the `MOVS R1, #N` ahead of each call grounds dense
/// values 0..=4).
pub const FONT_ITEMS: [(&str, u32); 5] = [
    ("Legacy", 0),
    ("Arial", 1),
    ("ArialBold", 2),
    ("SourceSans", 3),
    ("SourceSansBold", 4),
];
/// Name of a `Font` value for `convertToString`. Values with no item
/// yield "" — the writers only ever store table members.
pub fn font_name(value: u32) -> &'static str {
    FONT_ITEMS
        .iter()
        .find(|(_, v)| *v == value)
        .map(|(n, _)| *n)
        .unwrap_or("")
}
/// `placement_any` payload read by the `Font` dialogue (IDA
/// 0x66f524/0x66f548: int-tagged like the `NormalId` twin at
/// 0x662668/0x66268c): the value or something else (miss throws,
/// host: panic).
#[derive(Debug, Clone, Copy)]
pub enum FontVariant {
    Font(u32),
    Other,
}
/// `RBX::Reflection::PropDescriptor<TextBox, Color3>` cutover (IDA
/// 0x66ef9c): name/category/attributes/permissions. The
/// getter/setter member-pointer pair folds into the slot selector
/// (two objects: `TextColor3`/`TextStrokeColor3`, IDA 0x672782/0x672a94).
#[derive(Debug, Clone)]
pub struct TextBoxColorProp {
    pub name: String,
    pub category: String,
    pub attributes: u32,
    pub permissions: u32,
}
impl TextBoxColorProp {
    pub fn new(name: &str, category: &str, attributes: u32, permissions: u32) -> Self {
        Self {
            name: name.to_owned(),
            category: category.to_owned(),
            attributes,
            permissions,
        }
    }
}
/// `RBX::Reflection::PropDescriptor<TextBox, BrickColor>` cutover
/// (IDA 0x66f158): same identity-only shape over the single
/// `TextColor` member (IDA 0x672728).
#[derive(Debug, Clone)]
pub struct TextBoxBrickProp {
    pub name: String,
    pub category: String,
    pub attributes: u32,
    pub permissions: u32,
}
impl TextBoxBrickProp {
    pub fn new(name: &str, category: &str, attributes: u32, permissions: u32) -> Self {
        Self {
            name: name.to_owned(),
            category: category.to_owned(),
            attributes,
            permissions,
        }
    }
}
/// `RBX::Reflection::EnumPropDescriptor<TextBox, Font>` cutover (IDA
/// 0x66f2fc): name/category/attributes/permissions. The member pair
/// folds into the `font` field.
#[derive(Debug, Clone)]
pub struct TextBoxFontProp {
    pub name: String,
    pub category: String,
    pub attributes: u32,
    pub permissions: u32,
}
impl TextBoxFontProp {
    pub fn new(name: &str, category: &str, attributes: u32, permissions: u32) -> Self {
        Self {
            name: name.to_owned(),
            category: category.to_owned(),
            attributes,
            permissions,
        }
    }
}
/// `EnumDesc<TextService::FontSize>` items in `addPair` order (IDA
/// 0x7d80c4: the `MOVS R1, #N` ahead of each call grounds dense
/// values 0..=9).
pub const FONTSIZE_ITEMS: [(&str, u32); 10] = [
    ("Size8", 0),
    ("Size9", 1),
    ("Size10", 2),
    ("Size11", 3),
    ("Size12", 4),
    ("Size14", 5),
    ("Size18", 6),
    ("Size24", 7),
    ("Size36", 8),
    ("Size48", 9),
];
/// Name of a `FontSize` value for `convertToString`. Values with no
/// item yield "" — the writers only ever store table members.
pub fn fontsize_name(value: u32) -> &'static str {
    FONTSIZE_ITEMS
        .iter()
        .find(|(_, v)| *v == value)
        .map(|(n, _)| *n)
        .unwrap_or("")
}
/// Item index of a `FontSize` value, -1 when it has no item (the
/// `enumToItem` read in `convertToIndex`).
pub fn fontsize_index(value: u32) -> i32 {
    FONTSIZE_ITEMS
        .iter()
        .position(|(_, v)| *v == value)
        .map(|i| i as i32)
        .unwrap_or(-1)
}
/// `placement_any` payload read by the `FontSize` dialogue (IDA
/// 0x6707cc/0x6709c4: int-tagged like the `Font` twin at
/// 0x66f548): the value or something else (miss throws, host:
/// panic).
#[derive(Debug, Clone, Copy)]
pub enum FontSizeVariant {
    FontSize(u32),
    Other,
}
// 0x66e380 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE15convertToStringERKS3_
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::convertToString(RBX::TextService::XAlignment const&)const
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::convertToString(RBX::TextService::XAlignment const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE15convertToStringERKS3_")]
pub fn stub_066e380(value: i32) -> String {
    // IDA 0x66e380 (`EnumDesc<XAlignment>::convertToString`):
    // FLog::Asserts-gated `value >= 0` (enumconverter.h line 262,
    // 0x66e3bc-0x66e42c) and `value < enumToItem.size` (line 263,
    // 0x66e430-0x66e488) ReleaseAsserts — host seams; then a
    // negative value yields "" (0x66e490-0x66e4c2), an
    // over-size value yields "" (0x66e49c-0x66e4da), else the
    // value-indexed item name (0x66e4aa). Host: asserts + table
    // name with "" fallback.
    if flog_asserts() {
        assert!(
            value >= 0,
            "value>=0 file: include/reflection/enumconverter.h line: 262 (IDA 0x66e380)"
        );
        assert!(
            (value as usize) < XALIGNMENT_ITEMS.len(),
            "(size_t)value<enumToItem.size() file: include/reflection/enumconverter.h line: 263 (IDA 0x66e380)"
        );
    }
    if value >= 0 {
        xalignment_name(value as u32).to_owned()
    } else {
        String::new()
    }
}

// 0x66e520 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::XAlignment)>::isReadOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::XAlignment)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
pub fn stub_066e520() -> bool {
    // IDA 0x66e520 (`GetSetImpl<XAlignment>::isReadOnly`): `MOVS
    // R0, #0; BX LR` — always readable.
    false
}

// 0x66e524 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::XAlignment)>::isWriteOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::XAlignment)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
pub fn stub_066e524() -> bool {
    // IDA 0x66e524 (`GetSetImpl<XAlignment>::isWriteOnly`): `MOVS
    // R0, #0; BX LR` — always writable.
    false
}

// 0x66e528 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::XAlignment)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::XAlignment)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_066e528(state: &TextBoxState) -> u32 {
    // IDA 0x66e528 (`GetSetImpl<XAlignment>::getValue`): the
    // member-pointer resolve (null described reads at offset 0 with
    // the +536 `Instance` adjust, 0x66e52a-0x66e544; virtual when
    // the low bit is set, 0x66e54c-0x66e550) tail-calling the getter
    // (0x66e552). The member is `getXAlignment` (IDA 0x66871c, the
    // only `XAlignment` getter); the pointer folds into the field.
    state.x_alignment
}

// 0x66e554 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::XAlignment)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextService::XAlignment const&)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::XAlignment)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextService::XAlignment const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_066e554(state: &mut TextBoxState, value: u32) {
    // IDA 0x66e554 (`GetSetImpl<XAlignment>::setValue`): the
    // member-pointer resolve over +12/+16 (0x66e554-0x66e570)
    // tail-calling the setter with the input word. The member is
    // `setXAlignment` (the only `XAlignment` setter on `TextBox`);
    // the pointer folds into the field.
    state.x_alignment = value;
}

// 0x66e578 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10XAlignmentEEEE13initSingletonEv
// demangled: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::XAlignment> const>::initSingleton(void)
// type: int()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::XAlignment> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10XAlignmentEEEE13initSingletonEv")]
pub fn stub_066e578() {
    // IDA 0x66e578 (`Singleton<EnumDesc<XAlignment>>::initSingleton`):
    // thunk tail-calling `doGetSingleton` (host: stub_066e57c).
    // The singleton folds into the host table — carrier no-op.
}

// 0x66e57c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10XAlignmentEEEE14doGetSingletonEv
// demangled: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::XAlignment> const>::doGetSingleton(void)
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::XAlignment> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10XAlignmentEEEE14doGetSingletonEv")]
pub fn stub_066e57c() {
    // IDA 0x66e57c (`Singleton<EnumDesc<XAlignment>>::doGetSingleton`):
    // `__cxa_guard` once-init constructing the `EnumDesc` and
    // registering `__cxa_atexit` teardown (0x66e5d8-0x66e640).
    // Host statics initialize on use — carrier no-op.
}

// 0x66e66c — __ZN3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEED1Ev
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEED1Ev")]
pub fn stub_066e66c() {
    // IDA 0x066e66c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x66e670 — __ZN3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEED2Ev
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::~EnumDesc()
// type: int __fastcall(RBX::Reflection::EnumDescriptor *, int, int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEED2Ev")]
pub fn stub_066e670() {
    // IDA 0x066e670: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x66e844 — __ZN3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEED0Ev
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::~EnumDesc()
// type: void __fastcall(RBX::Reflection::EnumDescriptor *, int, int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEED0Ev")]
pub fn stub_066e844() {
    // IDA 0x066e844: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x66e8e4 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE6lookupEPKc
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::lookup(char const*)const
// type: int __fastcall(_DWORD *, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE6lookupEPKc")]
pub fn stub_066e8e4(name: &str) -> Option<u32> {
    // IDA 0x66e8e4 (`EnumDesc<XAlignment>::lookup(name)`):
    // `Name::lookup` (0x66e8f0) + `convertToValue` (0x66e8fe); on a
    // hit `convertToItem` (0x66e90a), else 0. Host: the item index
    // (`None` on a miss); the lookup folds into the compare.
    XALIGNMENT_ITEMS
        .iter()
        .position(|(n, _)| *n == name)
        .map(|i| i as u32)
}

// 0x66e914 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE6lookupERKNS0_7VariantE
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::lookup(RBX::Reflection::Variant const&)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE6lookupERKNS0_7VariantE")]
pub fn stub_066e914(variant: &XAlignmentVariant) -> Option<u32> {
    // IDA 0x66e914 (`EnumDesc<XAlignment>::lookup(variant)`):
    // `any_cast<XAlignment>` (throws on a miss, host: stub_066eb78)
    // then `convertToItem` (0x66e930). Host: the item index of the
    // cast value.
    XALIGNMENT_ITEMS
        .iter()
        .position(|(_, v)| *v == stub_066eb78(variant))
        .map(|i| i as u32)
}

// 0x66e934 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE14convertToValueEmRNS0_7VariantE
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// type: int __fastcall(int, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_066e934(index: u32) -> Option<u32> {
    // IDA 0x66e934 (`EnumDesc<XAlignment>::convertToValue`):
    // `count > index` (0x66e942) gates reading the indexed item's
    // value plus the `Type` tag and placement (0x66e948-0x66e95c),
    // returning 1 (else 0, 0x66e964). Host: the value (`None`
    // past the end); the tag/placement fold away.
    XALIGNMENT_ITEMS.get(index as usize).map(|(_, v)| *v)
}

// 0x66e968 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE15convertToStringEmRSs
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::convertToString(unsigned long,std::string &)const
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE15convertToStringEmRSs")]
pub fn stub_066e968(index: u32, out: &mut String) -> bool {
    // IDA 0x66e968 (`EnumDesc<XAlignment>::convertToString(index)`):
    // `count > index` (0x66e9bc) gates reading the value and
    // converting it to a name assigned into the out string
    // (0x66e9cc-0x66ea44), returning 1 (else 0 with `out`
    // untouched). Host: assign on hit, report.
    match XALIGNMENT_ITEMS.get(index as usize) {
        Some((_, value)) => {
            *out = xalignment_name(*value).to_owned();
            true
        }
        None => false,
    }
}

// 0x66eaac — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService10XAlignmentEEERS3_RKT_
// demangled: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TextService::XAlignment>(RBX::TextService::XAlignment const&)
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TextService::XAlignment>(RBX::TextService::XAlignment const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService10XAlignmentEEERS3_RKT_")]
pub fn stub_066eaac(value: u32) -> u32 {
    // IDA 0x66eaac (`placement_any::operator=<XAlignment>`):
    // ensures the holder singleton (0x66eab8), then stores the
    // value and (re)tags the holder (0x66eac4-0x66eaf0, destroying
    // the old payload first). Host values are `Copy` with the tag
    // in the type — the move is identity.
    value
}

// 0x66eafc — __ZN3rbx14implementation12typed_holderIN3RBX11TextService10XAlignmentEE9singletonEv
// demangled: rbx::implementation::typed_holder<RBX::TextService::XAlignment>::singleton(void)
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::TextService::XAlignment>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService10XAlignmentEE9singletonEv")]
pub fn stub_066eafc() {
    // IDA 0x66eafc (`typed_holder<XAlignment>::singleton`):
    // `__cxa_guard` once-init publishing the typeinfo and the
    // construct/destruct funcs (0x66eb16-0x66eb66). Host type tags
    // need no init — carrier no-op.
}

// 0x66eb68 — __ZN3rbx14implementation12typed_holderIN3RBX11TextService10XAlignmentEE14construct_funcEPKcPc
// demangled: rbx::implementation::typed_holder<RBX::TextService::XAlignment>::construct_func(char const*,char *)
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::TextService::XAlignment>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService10XAlignmentEE14construct_funcEPKcPc")]
pub fn stub_066eb68() {
    // IDA 0x66eb68 (`typed_holder<XAlignment>::construct_func`):
    // copies the held value pointer when non-null (0x66eb6a-0x66eb70).
    // Host values are `Copy` — carrier no-op.
}

// 0x66eb74 — __ZN3rbx14implementation12typed_holderIN3RBX11TextService10XAlignmentEE13destruct_funcEPc
// demangled: rbx::implementation::typed_holder<RBX::TextService::XAlignment>::destruct_func(char *)
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::TextService::XAlignment>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService10XAlignmentEE13destruct_funcEPc")]
pub fn stub_066eb74() {
    // IDA 0x66eb74 (`typed_holder<XAlignment>::destruct_func`):
    // empty body — carrier no-op.
}

// 0x66eb78 — __ZN3rbx8any_castIRKN3RBX11TextService10XAlignmentENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// demangled: RBX::TextService::XAlignment const& rbx::any_cast<RBX::TextService::XAlignment const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::TextService::XAlignment const& rbx::any_cast<RBX::TextService::XAlignment const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX11TextService10XAlignmentENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_066eb78(variant: &XAlignmentVariant) -> u32 {
    // IDA 0x66eb78 (`any_cast<XAlignment>`): null input misses
    // (0x66eba2-0x66ebd4); the typeinfo-pointer or mangled-name
    // (`N3RBX11TextService10XAlignmentE`) match returns the payload
    // (0x66ebe4-0x66ec1e); else `bad_placement_any_cast` is thrown
    // (0x66ec2e-0x66ec5a, host: panic). Host: tagged match.
    match *variant {
        XAlignmentVariant::XAlignment(value) => value,
        _ => panic!("rbx::bad_placement_any_cast (IDA 0x66eb78)"),
    }
}

// 0x66ec68 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10XAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::XAlignment>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::XAlignment>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::XAlignment>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TextService::XAlignment>> *)
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::XAlignment>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::XAlignment>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::XAlignment>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TextService::XAlignment>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10XAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_066ec68() {
    // IDA 0x066ec68: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x66ec90 — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEbEC2IMNS_12GuiTextMixinEKFbvEMS2_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::PropDescriptor<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(bool)>(char const*,char const*,bool (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::PropDescriptor<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(bool)>(char const*,char const*,bool (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEbEC2IMNS_12GuiTextMixinEKFbvEMS2_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_066ec90(
    name: &str,
    category: &str,
    attributes: u32,
    permissions: u32,
) -> TextBoxBoolProp {
    // IDA 0x66ec90 (`PropDescriptor<TextBox, bool>` ctor): the
    // `TextBox` `classDescriptor` call + `operator new` impl holding
    // the vtable and the getter/setter member-pointer pair, then the
    // `TypedPropertyDescriptor<bool>` base init with name/category/
    // attributes/permissions. Three objects share the type
    // (`TextWrapped` (deprecated alias), `TextWrap`, `TextScaled`,
    // IDA a_270 0x67282c-0x6728f6); the member pair folds into the
    // slot selector.
    TextBoxBoolProp::new(name, category, attributes, permissions)
}

// 0x66eda4 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(bool)>::isReadOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(bool)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE10isReadOnlyEv")]
pub fn stub_066eda4() -> bool {
    // IDA 0x66eda4 (`GetSetImpl<bool>::isReadOnly`): `MOVS R0, #0;
    // BX LR` — always readable.
    false
}

// 0x66eda8 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(bool)>::isWriteOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(bool)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE11isWriteOnlyEv")]
pub fn stub_066eda8() -> bool {
    // IDA 0x66eda8 (`GetSetImpl<bool>::isWriteOnly`): `MOVS R0, #0;
    // BX LR` — always writable.
    false
}

// 0x66edac — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_066edac(state: &TextBoxState, slot: TextBoxBoolSlot) -> bool {
    // IDA 0x66edac (`GetSetImpl<bool>::getValue`): the
    // member-pointer resolve (null described reads at offset 0 with
    // the +536 `Instance` adjust; virtual when the low bit is set)
    // tail-calling the getter. The member selects the slot; the
    // pointer folds away.
    state.bool_slot(slot)
}

// 0x66ede0 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const
// type: int __fastcall(int, int, unsigned __int8 *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_066ede0(state: &mut TextBoxState, slot: TextBoxBoolSlot, value: bool) {
    // IDA 0x66ede0 (`GetSetImpl<bool>::setValue`): the
    // member-pointer resolve over +12/+16 tail-calling the setter
    // with the input byte. The member selects the slot; the pointer
    // folds away.
    state.set_bool_slot(slot, value);
}

// 0x66ee04 — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEfEC2IMNS_12GuiTextMixinEKFfvEMS2_FvfEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,float>::PropDescriptor<float (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(float)>(char const*,char const*,float (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,float>::PropDescriptor<float (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(float)>(char const*,char const*,float (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEfEC2IMNS_12GuiTextMixinEKFfvEMS2_FvfEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_066ee04(
    name: &str,
    category: &str,
    attributes: u32,
    permissions: u32,
) -> TextBoxFloatProp {
    // IDA 0x66ee04 (`PropDescriptor<TextBox, float>` ctor): same
    // `classDescriptor` + impl + base-init shape for the single
    // float object (`TextStrokeTransparency`, IDA a_270 0x672ae4).
    // The member pair folds into the field.
    TextBoxFloatProp::new(name, category, attributes, permissions)
}

// 0x66ef18 — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEfED0Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,float>::~PropDescriptor()
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,float>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEfED0Ev")]
pub fn stub_066ef18() {
    // IDA 0x066ef18: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x66ef44 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,float>::GetSetImpl<float (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(float)>::isReadOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,float>::GetSetImpl<float (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(float)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE10isReadOnlyEv")]
pub fn stub_066ef44() -> bool {
    // IDA 0x66ef44 (`GetSetImpl<float>::isReadOnly`): `MOVS R0, #0;
    // BX LR` — always readable.
    false
}

// 0x66ef48 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,float>::GetSetImpl<float (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(float)>::isWriteOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,float>::GetSetImpl<float (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(float)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE11isWriteOnlyEv")]
pub fn stub_066ef48() -> bool {
    // IDA 0x66ef48 (`GetSetImpl<float>::isWriteOnly`): `MOVS R0, #0;
    // BX LR` — always writable.
    false
}

// 0x66ef4c — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,float>::GetSetImpl<float (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,float>::GetSetImpl<float (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_066ef4c(state: &TextBoxState, slot: TextBoxFloatSlot) -> f32 {
    // IDA 0x66ef4c (`GetSetImpl<float>::getValue`): the
    // member-pointer resolve tail-calling the getter. The member
    // selects the slot (`TextTransparency`/`TextStrokeTransparency`,
    // IDA a_270 0x6727de/0x672ae4); the pointer folds away.
    // CORRECTION to Batch Q: two float objects share this impl, not
    // one — hence the selector (same fix as the bool triple).
    state.float_slot(slot)
}

// 0x66ef78 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,float>::GetSetImpl<float (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,float>::GetSetImpl<float (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf")]
pub fn stub_066ef78(state: &mut TextBoxState, slot: TextBoxFloatSlot, value: f32) {
    // IDA 0x66ef78 (`GetSetImpl<float>::setValue`): the
    // member-pointer resolve tail-calling the setter with the input
    // word. The member selects the slot; the pointer folds away
    // (see the `getValue` correction above).
    state.set_float_slot(slot, value);
}

// 0x66ef9c — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D6Color3EEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Color3>::PropDescriptor<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(G3D::Color3)>(char const*,char const*,G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(G3D::Color3),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Color3>::PropDescriptor<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(G3D::Color3)>(char const*,char const*,G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(G3D::Color3),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D6Color3EEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_066ef9c(
    name: &str,
    category: &str,
    attributes: u32,
    permissions: u32,
) -> TextBoxColorProp {
    // IDA 0x66ef9c (`PropDescriptor<TextBox, Color3>` ctor): the
    // `TextBox` `classDescriptor` call + `operator new` impl holding
    // the vtable and the getter/setter member-pointer pair, then the
    // `TypedPropertyDescriptor<Color3>` base init with
    // name/category/attributes/permissions. Two objects share the
    // type (`TextColor3`/`TextStrokeColor3`, IDA a_270
    // 0x672782/0x672a94); the member pair folds into the slot
    // selector.
    TextBoxColorProp::new(name, category, attributes, permissions)
}

// 0x66f0b0 — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D6Color3EED0Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Color3>::~PropDescriptor()
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Color3>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D6Color3EED0Ev")]
pub fn stub_066f0b0() {
    // IDA 0x066f0b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x66f0dc — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D6Color3EE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(G3D::Color3)>::isReadOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(G3D::Color3)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D6Color3EE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
pub fn stub_066f0dc() -> bool {
    // IDA 0x66f0dc (`GetSetImpl<Color3>::isReadOnly`): `MOVS R0, #0;
    // BX LR` — always readable.
    false
}

// 0x66f0e0 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D6Color3EE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(G3D::Color3)>::isWriteOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(G3D::Color3)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D6Color3EE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
pub fn stub_066f0e0() -> bool {
    // IDA 0x66f0e0 (`GetSetImpl<Color3>::isWriteOnly`): `MOVS R0, #0;
    // BX LR` — always writable.
    false
}

// 0x66f0e4 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D6Color3EE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(G3D::Color3)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(G3D::Color3)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D6Color3EE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_066f0e4(state: &TextBoxState, slot: TextBoxColorSlot) -> [f32; 3] {
    // IDA 0x66f0e4 (`GetSetImpl<Color3>::getValue`): the
    // member-pointer resolve tail-calling the getter. The member
    // selects the slot; the pointer folds away.
    state.color_slot(slot)
}

// 0x66f11c — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D6Color3EE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(G3D::Color3)>::setValue(RBX::Reflection::DescribedBase *,G3D::Color3 const&)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(G3D::Color3)>::setValue(RBX::Reflection::DescribedBase *,G3D::Color3 const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D6Color3EE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_066f11c(state: &mut TextBoxState, slot: TextBoxColorSlot, value: [f32; 3]) {
    // IDA 0x66f11c (`GetSetImpl<Color3>::setValue`): the
    // member-pointer resolve tail-calling the setter, copying the
    // three input words first (0x66f144-0x66f14c). The member
    // selects the slot; the pointer folds away.
    state.set_color_slot(slot, value);
}

// 0x66f158 — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEEC2IMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::PropDescriptor<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor)>(char const*,char const*,RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::PropDescriptor<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor)>(char const*,char const*,RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEEC2IMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_066f158(
    name: &str,
    category: &str,
    attributes: u32,
    permissions: u32,
) -> TextBoxBrickProp {
    // IDA 0x66f158 (`PropDescriptor<TextBox, BrickColor>` ctor): same
    // `classDescriptor` + impl + base-init shape for the single
    // object (`TextColor`, IDA a_270 0x672728). The member pair
    // folds into the `text_color` field.
    TextBoxBrickProp::new(name, category, attributes, permissions)
}

// 0x66f26c — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEED0Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::~PropDescriptor()
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEED0Ev")]
pub fn stub_066f26c() {
    // IDA 0x066f26c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x66f298 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor)>::isReadOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE10isReadOnlyEv")]
pub fn stub_066f298() -> bool {
    // IDA 0x66f298 (`GetSetImpl<BrickColor>::isReadOnly`): `MOVS
    // R0, #0; BX LR` — always readable.
    false
}

// 0x66f29c — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor)>::isWriteOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE11isWriteOnlyEv")]
pub fn stub_066f29c() -> bool {
    // IDA 0x66f29c (`GetSetImpl<BrickColor>::isWriteOnly`): `MOVS
    // R0, #0; BX LR` — always writable.
    false
}

// 0x66f2a0 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_066f2a0(state: &TextBoxState) -> u32 {
    // IDA 0x66f2a0 (`GetSetImpl<BrickColor>::getValue`): the
    // member-pointer resolve tail-calling the getter. The member is
    // `getTextColor` (the only `BrickColor` object in the TU, IDA
    // a_270 0x672728); the pointer folds into the field.
    state.text_color
}

// 0x66f2d8 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor)>::setValue(RBX::Reflection::DescribedBase *,RBX::BrickColor const&)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor)>::setValue(RBX::Reflection::DescribedBase *,RBX::BrickColor const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")]
pub fn stub_066f2d8(state: &mut TextBoxState, value: u32) {
    // IDA 0x66f2d8 (`GetSetImpl<BrickColor>::setValue`): the
    // member-pointer resolve tail-calling the setter with the input
    // word. The member is `setTextColor`; the pointer folds into
    // the field.
    state.text_color = value;
}

// 0x66f2fc — __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::EnumPropDescriptor<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font)>(char const*,char const*,RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, int, int, int, int, char, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::EnumPropDescriptor<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font)>(char const*,char const*,RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_066f2fc(
    name: &str,
    category: &str,
    attributes: u32,
    permissions: u32,
) -> TextBoxFontProp {
    // IDA 0x66f2fc (`EnumPropDescriptor<TextBox, Font>` ctor): the
    // `TextBox` `classDescriptor` call, the `EnumDesc<Font>`
    // singleton once-init and the `PropertyDescriptor` base init
    // with name/category/attributes/permissions plus the impl
    // holding the getter/setter member-pointer pair. The pair folds
    // into the `font` field (same shape as `NormalIdProp` at
    // 0x662440).
    TextBoxFontProp::new(name, category, attributes, permissions)
}

// 0x66f4b0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEED0Ev
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::~EnumPropDescriptor()
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEED0Ev")]
pub fn stub_066f4b0() {
    // IDA 0x066f4b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x66f4dc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE10isReadOnlyEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::isReadOnly(void)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE10isReadOnlyEv")]
pub fn stub_066f4dc() -> bool {
    // IDA 0x66f4dc (`EnumPropDescriptor<Font>::isReadOnly`):
    // delegates to the inner `GetSet` at +44 — always readable.
    false
}

// 0x66f4ec — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE11isWriteOnlyEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::isWriteOnly(void)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE11isWriteOnlyEv")]
pub fn stub_066f4ec() -> bool {
    // IDA 0x66f4ec (`EnumPropDescriptor<Font>::isWriteOnly`):
    // delegates to the inner `GetSet` at +44 — always writable.
    false
}

// 0x66f4fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE11equalValuesEPKNS0_13DescribedBaseES8_
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE11equalValuesEPKNS0_13DescribedBaseES8_")]
pub fn stub_066f4fc(first: &TextBoxState, second: &TextBoxState) -> bool {
    // IDA 0x66f4fc (`EnumPropDescriptor<Font>::equalValues`): reads
    // the inner value for both instances via the +44 `GetSet` and
    // compares. Host: compare the fonts.
    first.font == second.font
}

// 0x66f524 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_066f524(state: &TextBoxState) -> FontVariant {
    // IDA 0x66f524 (`EnumPropDescriptor<Font>::getVariant`): reads
    // the inner value, tags it with the plain-`int` singleton and
    // placement-moves it in (same int-tagged shape as the
    // `NormalId` twin at 0x662668). Host: the `Font` tag.
    FontVariant::Font(state.font)
}

// 0x66f548 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_066f548(state: &mut TextBoxState, variant: &FontVariant) {
    // IDA 0x66f548 (`EnumPropDescriptor<Font>::setVariant`): an
    // int-typed variant runs `any_cast<int>`; anything else runs
    // `Variant::convert<int>` (throws on failure); then the +72
    // setter. Host: convert-or-throw, then store.
    let value = match *variant {
        FontVariant::Font(value) => value,
        _ => panic!("Unable to convert variant to int (IDA 0x66f548)"),
    };
    state.font = value;
}

// 0x66f694 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE9copyValueEPKNS0_13DescribedBaseEPS6_
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE9copyValueEPKNS0_13DescribedBaseEPS6_")]
pub fn stub_066f694(first: &TextBoxState, second: &mut TextBoxState) {
    // IDA 0x66f694 (`EnumPropDescriptor<Font>::copyValue`): inner
    // `getValue` on the source then inner `setValue` on the target.
    // Host: copy the font.
    second.font = first.font;
}

// 0x66f6b8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE14hasStringValueEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::hasStringValue(void)const
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE14hasStringValueEv")]
pub fn stub_066f6b8() -> bool {
    // IDA 0x66f6b8 (`EnumPropDescriptor<Font>::hasStringValue`):
    // returns 1 — always stringable.
    true
}

// 0x66f6bc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE14getStringValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::getStringValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_066f6bc(state: &TextBoxState) -> String {
    // IDA 0x66f6bc (`EnumPropDescriptor<Font>::getStringValue`):
    // reads the enum-desc singleton slot, the inner value via the
    // +44 `GetSet` and `EnumDesc::convertToString`. Host: the
    // grounded item name.
    font_name(state.font).to_owned()
}

// 0x66f6e0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE14setStringValueEPNS0_13DescribedBaseERKSs
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_066f6e0(state: &mut TextBoxState, name: &str) -> bool {
    // IDA 0x66f6e0 (`EnumPropDescriptor<Font>::setStringValue`):
    // `Name::lookup` + `EnumDesc::convertToValue`; on a hit the
    // inner `setValue` runs and 1 returns, else 0. Host: table
    // position decides.
    match FONT_ITEMS.iter().position(|(n, _)| *n == name) {
        Some(index) => {
            state.font = FONT_ITEMS[index].1;
            true
        }
        None => false,
    }
}

// 0x66f720 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_066f720(state: &TextBoxState, out: &mut XmlIntSlot) -> i32 {
    // IDA 0x66f720 (`EnumPropDescriptor<Font>::writeValue`): inner
    // `getValue`, `clearValue`, int tag `5` at +16, value at +20,
    // returns 5. Same shape as the `InputType` twin at 0x659884 —
    // needs `XmlIntSlot` (host: import from `generated_134`).
    out.value_type = 0;
    out.value_type = 5;
    out.int_value = state.font as i32;
    5
}

// 0x66f740 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_066f740(state: &mut TextBoxState, xml: &XmlReadValue) {
    // IDA 0x66f740 (`EnumPropDescriptor<Font>::readValue`): xsi:nil
    // early-out, string pair with fallthrough, raw int set, else
    // `ReleaseAssert(false)` — same shape as the `InputType` twin
    // at 0x6598a4 (needs `XmlReadValue`; the assert cites
    // Surface.cpp line 313 there — here the TU differs, host seam
    // kept generic). Host: match the pair.
    match xml {
        XmlReadValue::Nil => {}
        XmlReadValue::Int(value) => {
            state.font = *value as u32;
        }
        XmlReadValue::Text(text) => {
            if stub_066f6e0(state, text) {
                return;
            }
            if flog_asserts() {
                panic!("ReleaseAssert(false) (IDA 0x66f740)");
            }
        }
        XmlReadValue::Other => {
            if flog_asserts() {
                panic!("ReleaseAssert(false) (IDA 0x66f740)");
            }
        }
    }
}

// 0x66f980 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE13getIndexValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::getIndexValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_066f980(state: &TextBoxState) -> i32 {
    // IDA 0x66f980 (`EnumPropDescriptor<Font>::getIndexValue`):
    // singleton once + inner `getValue` + `EnumDesc::convertToIndex`.
    // Host: the item index of the live value.
    FONT_ITEMS
        .iter()
        .position(|(_, v)| *v == state.font)
        .map(|i| i as i32)
        .unwrap_or(-1)
}

// 0x66f99c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE13setIndexValueEPNS0_13DescribedBaseEm
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE13setIndexValueEPNS0_13DescribedBaseEm")]
pub fn stub_066f99c(state: &mut TextBoxState, index: u32) -> bool {
    // IDA 0x66f99c (`EnumPropDescriptor<Font>::setIndexValue`):
    // `count > index` gates storing the indexed item's value.
    // Host: table read decides.
    match FONT_ITEMS.get(index as usize) {
        Some((_, value)) => {
            state.font = *value;
            true
        }
        None => false,
    }
}

// 0x66f9d0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE12getEnumValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::getEnumValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub fn stub_066f9d0(state: &TextBoxState) -> u32 {
    // IDA 0x66f9d0 (`EnumPropDescriptor<Font>::getEnumValue`):
    // inner `getValue` through the +44 `GetSet`. Host: the live
    // font id.
    state.font
}

// 0x66f9d8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE12setEnumValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub fn stub_066f9d8(state: &mut TextBoxState, value: u32) -> bool {
    // IDA 0x66f9d8 (`EnumPropDescriptor<Font>::setEnumValue`):
    // `find_if` over the items with `equalValue(value)`
    // (0x66fa02) using the enum-desc item range at +48+28/+32;
    // found stores via the inner `setValue` and returns 1
    // (0x66fa16-0x66fa18), else 0. Host: table membership decides.
    if FONT_ITEMS.iter().any(|(_, v)| *v == value) {
        state.font = value;
        true
    } else {
        false
    }
}

// 0x66fa24 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE11getEnumItemEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::getEnumItem(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub fn stub_066fa24(state: &TextBoxState) -> Option<u32> {
    // IDA 0x66fa24 (`EnumPropDescriptor<Font>::getEnumItem`): inner
    // `getValue` (0x66fa36) then `EnumDesc::convertToItem`
    // (0x66fa42, host: stub_066faf4). Host: the item index of the
    // live value.
    FONT_ITEMS
        .iter()
        .position(|(_, v)| *v == state.font)
        .map(|i| i as u32)
}

// 0x66fa44 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub fn stub_066fa44(state: &mut TextBoxState, name: &str) -> bool {
    // IDA 0x66fa44 (`EnumPropDescriptor<Font>::setStringValue(Name)`):
    // `EnumDesc::convertToValue(name)` (0x66fa5a, host: stub_066fa78);
    // hit stores via the inner `setValue` and returns 1
    // (0x66fa70-0x66fa72), else 0. Host: table lookup decides.
    match FONT_ITEMS.iter().find(|(n, _)| *n == name) {
        Some((_, value)) => {
            state.font = *value;
            true
        }
        None => false,
    }
}

// 0x66fa78 — __ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE14convertToValueERKNS_4NameERS3_
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToValue(RBX::Name const&,RBX::TextService::Font&)const
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToValue(RBX::Name const&,RBX::TextService::Font&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_066fa78(name: &str) -> Option<u32> {
    // IDA 0x66fa78 (`EnumDesc<Font>::convertToValue(Name)`): RB-tree
    // lower_bound walks over the name→value maps (0x66fa8e-0x66fad2)
    // with exact-match checks (0x66faa2-0x66fab0); hit fills the out
    // value and returns 1, else 0. Host: table lookup.
    FONT_ITEMS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, v)| *v)
}

// 0x66faf4 — __ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE13convertToItemERKS3_
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToItem(RBX::TextService::Font const&)const
// type: int __fastcall(int, int *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToItem(RBX::TextService::Font const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE13convertToItemERKS3_")]
pub fn stub_066faf4(value: i32) -> Option<u32> {
    // IDA 0x66faf4 (`EnumDesc<Font>::convertToItem`):
    // FLog::Asserts-gated `value >= 0` (enumconverter.h line 273)
    // and `value < items.size` (line 274) ReleaseAsserts — host
    // seams; then a negative value yields 0, an over-size value
    // yields 0, else the item (0x66fba0-0x66fbb8). Host: the item
    // index (`None` for the null slots).
    if flog_asserts() {
        assert!(
            value >= 0,
            "value>=0 file: include/reflection/enumconverter.h line: 273 (IDA 0x66faf4)"
        );
        assert!(
            (value as usize) < FONT_ITEMS.len(),
            "(size_t)value<enumToItem.size() file: include/reflection/enumconverter.h line: 274 (IDA 0x66faf4)"
        );
    }
    if value >= 0 {
        FONT_ITEMS
            .iter()
            .position(|(_, v)| *v == value as u32)
            .map(|i| i as u32)
    } else {
        None
    }
}

// 0x66fbc0 — __ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE14convertToIndexES3_
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToIndex(RBX::TextService::Font)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToIndex(RBX::TextService::Font)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE14convertToIndexES3_")]
pub fn stub_066fbc0(value: i32) -> i32 {
    // IDA 0x66fbc0 (`EnumDesc<Font>::convertToIndex`):
    // FLog::Asserts-gated `value >= 0` (enumconverter.h line 350,
    // 0x66fbd4-0x66fc10 — a host seam), then the `enumToItem` table
    // read with -1 past the end (0x66fc1a-0x66fc2e). Same shape as
    // the `InputType` twin at 0x659cc8. Host: item position, -1
    // when absent.
    if flog_asserts() {
        assert!(
            value >= 0,
            "value>=0 file: include/reflection/enumconverter.h line: 350 (IDA 0x66fbc0)"
        );
    }
    if value >= 0 {
        FONT_ITEMS
            .iter()
            .position(|(_, v)| *v == value as u32)
            .map(|i| i as i32)
            .unwrap_or(-1)
    } else {
        -1
    }
}

// 0x66fc30 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE11setIntValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setIntValue(RBX::Reflection::DescribedBase *,int)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_066fc30(state: &mut TextBoxState, raw: i32) -> bool {
    // IDA 0x66fc30 (`EnumPropDescriptor<Font>::setIntValue`): `raw
    // >= 0` (0x66fc3a) and in the `enumToItem`-shaped map at +48+132
    // (0x66fc4c) with a non--1 value (0x66fc58) gates the inner
    // `setValue` (0x66fc64) returning 1; else 0. The map is dense
    // identity for `Font`, so the index reads the table. Host:
    // table read decides.
    if raw >= 0 {
        if let Some((_, value)) = FONT_ITEMS.get(raw as usize) {
            state.font = *value;
            return true;
        }
    }
    false
}

// 0x66fc70 — __ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE15convertToStringERKS3_
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToString(RBX::TextService::Font const&)const
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToString(RBX::TextService::Font const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE15convertToStringERKS3_")]
pub fn stub_066fc70(value: i32) -> String {
    // IDA 0x66fc70 (`EnumDesc<Font>::convertToString`): same
    // FLog::Asserts-gated `value >= 0` (:262) / size (:263) shape
    // as the `XAlignment` twin at 0x66e380, then "" for negative or
    // over-size values, else the value-indexed item name. Host:
    // asserts + table name with "" fallback.
    if flog_asserts() {
        assert!(
            value >= 0,
            "value>=0 file: include/reflection/enumconverter.h line: 262 (IDA 0x66fc70)"
        );
        assert!(
            (value as usize) < FONT_ITEMS.len(),
            "(size_t)value<enumToItem.size() file: include/reflection/enumconverter.h line: 263 (IDA 0x66fc70)"
        );
    }
    if value >= 0 {
        font_name(value as u32).to_owned()
    } else {
        String::new()
    }
}

// 0x66fe10 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font)>::isReadOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
pub fn stub_066fe10() -> bool {
    // IDA 0x66fe10 (`GetSetImpl<Font>::isReadOnly`): `MOVS R0, #0;
    // BX LR` — always readable.
    false
}

// 0x66fe14 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font)>::isWriteOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
pub fn stub_066fe14() -> bool {
    // IDA 0x66fe14 (`GetSetImpl<Font>::isWriteOnly`): `MOVS R0, #0;
    // BX LR` — always writable.
    false
}

// 0x66fe18 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_066fe18(state: &TextBoxState) -> u32 {
    // IDA 0x66fe18 (`GetSetImpl<Font>::getValue`): the
    // member-pointer resolve tail-calling the getter. The member is
    // `getFont` (the only `Font` getter); the pointer folds into
    // the field.
    state.font
}

// 0x66fe44 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextService::Font const&)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextService::Font const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_066fe44(state: &mut TextBoxState, value: u32) {
    // IDA 0x66fe44 (`GetSetImpl<Font>::setValue`): the
    // member-pointer resolve tail-calling the setter with the input
    // word. The member is `setFont`; the pointer folds into the
    // field.
    state.font = value;
}

// 0x66fe68 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService4FontEEEE13initSingletonEv
// demangled: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::Font> const>::initSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::Font> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService4FontEEEE13initSingletonEv")]
pub fn stub_066fe68() {
    // IDA 0x66fe68 (`Singleton<EnumDesc<Font>>::initSingleton`):
    // thunk tail-calling `doGetSingleton` (host: stub_066fe6c).
    // The singleton folds into the host table — carrier no-op.
}

// 0x66fe6c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService4FontEEEE14doGetSingletonEv
// demangled: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::Font> const>::doGetSingleton(void)
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::Font> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService4FontEEEE14doGetSingletonEv")]
pub fn stub_066fe6c() {
    // IDA 0x66fe6c (`Singleton<EnumDesc<Font>>::doGetSingleton`):
    // `__cxa_guard` once-init constructing the `EnumDesc` and
    // registering `__cxa_atexit` teardown. Host statics initialize
    // on use — carrier no-op.
}

// 0x66ff5c — __ZN3RBX10Reflection8EnumDescINS_11TextService4FontEED1Ev
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::Font>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService4FontEED1Ev")]
pub fn stub_066ff5c() {
    // IDA 0x066ff5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x66ff60 — __ZN3RBX10Reflection8EnumDescINS_11TextService4FontEED2Ev
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::Font>::~EnumDesc()
// type: int __fastcall(RBX::Reflection::EnumDescriptor *, int, int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService4FontEED2Ev")]
pub fn stub_066ff60() {
    // IDA 0x066ff60: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x670134 — __ZN3RBX10Reflection8EnumDescINS_11TextService4FontEED0Ev
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::Font>::~EnumDesc()
// type: void __fastcall(RBX::Reflection::EnumDescriptor *, int, int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService4FontEED0Ev")]
pub fn stub_0670134() {
    // IDA 0x0670134: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6701d4 — __ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE6lookupEPKc
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::Font>::lookup(char const*)const
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE6lookupEPKc")]
pub fn stub_06701d4(name: &str) -> Option<u32> {
    // IDA 0x6701d4 (`EnumDesc<Font>::lookup(name)`): `Name::lookup`
    // + `convertToValue`; on a hit `convertToItem`, else 0. Host:
    // the item index (`None` on a miss). Same shape as the
    // `XAlignment` twin at 0x66e8e4.
    FONT_ITEMS
        .iter()
        .position(|(n, _)| *n == name)
        .map(|i| i as u32)
}

// 0x670204 — __ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE6lookupERKNS0_7VariantE
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::Font>::lookup(RBX::Reflection::Variant const&)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE6lookupERKNS0_7VariantE")]
pub fn stub_0670204(variant: &FontVariant) -> Option<u32> {
    // IDA 0x670204 (`EnumDesc<Font>::lookup(variant)`):
    // `any_cast<Font>` (throws on a miss, host: stub_0670468) then
    // `convertToItem`. Host: the item index of the cast value.
    FONT_ITEMS
        .iter()
        .position(|(_, v)| *v == stub_0670468(variant))
        .map(|i| i as u32)
}

// 0x670224 — __ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE14convertToValueEmRNS0_7VariantE
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// type: int __fastcall(int, unsigned int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_0670224(index: u32) -> Option<u32> {
    // IDA 0x670224 (`EnumDesc<Font>::convertToValue`): `count >
    // index` gates reading the indexed item's value plus the `Type`
    // tag and placement, returning 1 (else 0). Host: the value
    // (`None` past the end). Same shape as the `XAlignment` twin
    // at 0x66e934.
    FONT_ITEMS.get(index as usize).map(|(_, v)| *v)
}

// 0x670258 — __ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE15convertToStringEmRSs
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToString(unsigned long,std::string &)const
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE15convertToStringEmRSs")]
pub fn stub_0670258(index: u32, out: &mut String) -> bool {
    // IDA 0x670258 (`EnumDesc<Font>::convertToString(index)`):
    // `count > index` gates reading the value and converting it to
    // a name assigned into the out string, returning 1 (else 0 with
    // `out` untouched). Same shape as the `XAlignment` twin at
    // 0x66e968. Host: assign on hit, report.
    match FONT_ITEMS.get(index as usize) {
        Some((_, value)) => {
            *out = font_name(*value).to_owned();
            true
        }
        None => false,
    }
}

// 0x67039c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService4FontEEERS3_RKT_
// demangled: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TextService::Font>(RBX::TextService::Font const&)
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TextService::Font>(RBX::TextService::Font const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService4FontEEERS3_RKT_")]
pub fn stub_067039c(value: u32) -> u32 {
    // IDA 0x67039c (`placement_any::operator=<Font>`): ensures the
    // holder singleton, then stores the value and (re)tags the
    // holder (destroying the old payload first). Host values are
    // `Copy` with the tag in the type — the move is identity. Same
    // shape as the `XAlignment` twin at 0x66eaac.
    value
}

// 0x6703ec — __ZN3rbx14implementation12typed_holderIN3RBX11TextService4FontEE9singletonEv
// demangled: rbx::implementation::typed_holder<RBX::TextService::Font>::singleton(void)
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::TextService::Font>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService4FontEE9singletonEv")]
pub fn stub_06703ec() {
    // IDA 0x6703ec (`typed_holder<Font>::singleton`): `__cxa_guard`
    // once-init publishing the typeinfo and the construct/destruct
    // funcs. Host type tags need no init — carrier no-op.
}

// 0x670458 — __ZN3rbx14implementation12typed_holderIN3RBX11TextService4FontEE14construct_funcEPKcPc
// demangled: rbx::implementation::typed_holder<RBX::TextService::Font>::construct_func(char const*,char *)
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::TextService::Font>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService4FontEE14construct_funcEPKcPc")]
pub fn stub_0670458() {
    // IDA 0x670458 (`typed_holder<Font>::construct_func`): copies
    // the held value pointer when non-null. Host values are `Copy`
    // — carrier no-op.
}

// 0x670464 — __ZN3rbx14implementation12typed_holderIN3RBX11TextService4FontEE13destruct_funcEPc
// demangled: rbx::implementation::typed_holder<RBX::TextService::Font>::destruct_func(char *)
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::TextService::Font>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService4FontEE13destruct_funcEPc")]
pub fn stub_0670464() {
    // IDA 0x670464 (`typed_holder<Font>::destruct_func`): empty
    // body — carrier no-op.
}

// 0x670468 — __ZN3rbx8any_castIRKN3RBX11TextService4FontENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// demangled: RBX::TextService::Font const& rbx::any_cast<RBX::TextService::Font const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::TextService::Font const& rbx::any_cast<RBX::TextService::Font const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX11TextService4FontENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0670468(variant: &FontVariant) -> u32 {
    // IDA 0x670468 (`any_cast<Font>`): null input misses; the
    // typeinfo-pointer or mangled-name match returns the payload;
    // else `bad_placement_any_cast` is thrown (host: panic). Same
    // shape as the `XAlignment` twin at 0x66eb78.
    match *variant {
        FontVariant::Font(value) => value,
        _ => panic!("rbx::bad_placement_any_cast (IDA 0x670468)"),
    }
}

// 0x670558 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService4FontEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::Font>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::Font>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::Font>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TextService::Font>> *)
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::Font>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::Font>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::Font>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TextService::Font>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService4FontEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_0670558() {
    // IDA 0x0670558: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x670580 — __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::EnumPropDescriptor<RBX::TextService::FontSize (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::FontSize)>(char const*,char const*,RBX::TextService::FontSize (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::FontSize),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, int, int, int, int, char, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::EnumPropDescriptor<RBX::TextService::FontSize (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::FontSize)>(char const*,char const*,RBX::TextService::FontSize (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::FontSize),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0670580(
    name: &str,
    category: &str,
    attributes: u32,
    permissions: u32,
) -> TextBoxFontProp {
    // IDA 0x670580 (`EnumPropDescriptor<FontSize>` ctor): the
    // `TextBox` `classDescriptor` call, the `EnumDesc<FontSize>`
    // singleton once-init and the `PropertyDescriptor` base init
    // with name/category/attributes/permissions plus the impl
    // holding the getter/setter member-pointer pair. The pair folds
    // into the `font_size` field (see below).
    TextBoxFontProp::new(name, category, attributes, permissions)
}

// 0x670734 — __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEED0Ev
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::~EnumPropDescriptor()
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEED0Ev")]
pub fn stub_0670734() {
    // IDA 0x0670734: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x670760 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10isReadOnlyEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::isReadOnly(void)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10isReadOnlyEv")]
pub fn stub_0670760() -> bool {
    // IDA 0x670760 (`EnumPropDescriptor<FontSize>::isReadOnly`):
    // delegates to the inner `GetSet` at +44 — always readable.
    false
}

// 0x670770 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE11isWriteOnlyEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::isWriteOnly(void)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE11isWriteOnlyEv")]
pub fn stub_0670770() -> bool {
    // IDA 0x670770 (`EnumPropDescriptor<FontSize>::isWriteOnly`):
    // delegates to the inner `GetSet` at +44 — always writable.
    false
}

// 0x670780 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE11equalValuesEPKNS0_13DescribedBaseES8_
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE11equalValuesEPKNS0_13DescribedBaseES8_")]
pub fn stub_0670780(first: &TextBoxState, second: &TextBoxState) -> bool {
    // IDA 0x670780 (`EnumPropDescriptor<FontSize>::equalValues`):
    // reads the inner value for both instances via the +44 `GetSet`
    // and compares. Host: compare the sizes.
    first.font_size == second.font_size
}

// 0x6707a8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_06707a8(state: &TextBoxState) -> FontSizeVariant {
    // IDA 0x6707a8 (`EnumPropDescriptor<FontSize>::getVariant`):
    // reads the inner value, tags it with the plain-`int`
    // singleton and placement-moves it in (same int-tagged shape
    // as the `Font` twin at 0x66f524). Host: the `FontSize` tag.
    FontSizeVariant::FontSize(state.font_size)
}

// 0x6707cc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_06707cc(state: &mut TextBoxState, variant: &FontSizeVariant) {
    // IDA 0x6707cc (`EnumPropDescriptor<FontSize>::setVariant`): an
    // int-typed variant runs `any_cast<int>`; anything else runs
    // `Variant::convert<int>` (throws on failure); then the +72
    // setter. Host: convert-or-throw, then store.
    let value = match *variant {
        FontSizeVariant::FontSize(value) => value,
        _ => panic!("Unable to convert variant to int (IDA 0x6707cc)"),
    };
    state.font_size = value;
}

// 0x670918 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE9copyValueEPKNS0_13DescribedBaseEPS6_
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE9copyValueEPKNS0_13DescribedBaseEPS6_")]
pub fn stub_0670918(first: &TextBoxState, second: &mut TextBoxState) {
    // IDA 0x670918 (`EnumPropDescriptor<FontSize>::copyValue`):
    // inner `getValue` on the source then inner `setValue` on the
    // target. Host: copy the size.
    second.font_size = first.font_size;
}

// 0x67093c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE14hasStringValueEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::hasStringValue(void)const
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE14hasStringValueEv")]
pub fn stub_067093c() -> bool {
    // IDA 0x67093c (`EnumPropDescriptor<FontSize>::hasStringValue`):
    // returns 1 — always stringable.
    true
}

// 0x670940 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE14getStringValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::getStringValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_0670940(state: &TextBoxState) -> String {
    // IDA 0x670940 (`EnumPropDescriptor<FontSize>::getStringValue`):
    // reads the enum-desc singleton slot, the inner value via the
    // +44 `GetSet` and `EnumDesc::convertToString`. Host: the
    // grounded item name.
    fontsize_name(state.font_size).to_owned()
}

// 0x670964 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE14setStringValueEPNS0_13DescribedBaseERKSs
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_0670964(state: &mut TextBoxState, name: &str) -> bool {
    // IDA 0x670964 (`EnumPropDescriptor<FontSize>::setStringValue`):
    // `Name::lookup` + `EnumDesc::convertToValue`; on a hit the
    // inner `setValue` runs and 1 returns, else 0. Host: table
    // position decides.
    match FONTSIZE_ITEMS.iter().position(|(n, _)| *n == name) {
        Some(index) => {
            state.font_size = FONTSIZE_ITEMS[index].1;
            true
        }
        None => false,
    }
}

// 0x6709a4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_06709a4(state: &TextBoxState, out: &mut XmlIntSlot) -> i32 {
    // IDA 0x6709a4 (`EnumPropDescriptor<FontSize>::writeValue`):
    // inner `getValue`, `clearValue`, int tag `5` at +16, value at
    // +20, returns 5. Same shape as the `Font` twin at 0x66f720.
    out.value_type = 0;
    out.value_type = 5;
    out.int_value = state.font_size as i32;
    5
}

// 0x6709c4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_06709c4(state: &mut TextBoxState, xml: &XmlReadValue) {
    // IDA 0x6709c4 (`EnumPropDescriptor<FontSize>::readValue`):
    // xsi:nil early-out (0x6709e8); an int pair runs `setIntValue`
    // (index→value with -1 rejection, 0x670a30-0x670a40) and
    // returns on success; a string pair runs lookup + convert +
    // inner set (0x670a4e-0x670aaa), a miss running the +64 reset
    // hook before asserting (0x670b7c-0x670b7e, ungrounded target —
    // folds away); anything else hits `ReleaseAssert(false)`
    // (Reflection.h line 359, host seam). The `enumToItem` map is
    // dense identity for `FontSize`, so the int path reads the
    // table.
    match xml {
        XmlReadValue::Nil => {}
        XmlReadValue::Int(value) => {
            if *value >= 0 {
                if let Some((_, size)) = FONTSIZE_ITEMS.get(*value as usize) {
                    state.font_size = *size;
                    return;
                }
            }
            if flog_asserts() {
                panic!("false file: include/Reflection/Reflection.h line: 359 (IDA 0x6709c4)");
            }
        }
        XmlReadValue::Text(text) => {
            if stub_0670964(state, text) {
                return;
            }
            if flog_asserts() {
                panic!("false file: include/Reflection/Reflection.h line: 359 (IDA 0x6709c4)");
            }
        }
        XmlReadValue::Other => {
            if flog_asserts() {
                panic!("false file: include/Reflection/Reflection.h line: 359 (IDA 0x6709c4)");
            }
        }
    }
}

// 0x670c04 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE13getIndexValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::getIndexValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_0670c04(state: &TextBoxState) -> i32 {
    // IDA 0x670c04 (`EnumPropDescriptor<FontSize>::getIndexValue`):
    // inner `getValue` + `EnumDesc::convertToIndex`. Host: the
    // item index of the live value.
    fontsize_index(state.font_size)
}
