//! audio generated_audio_wd_watchdog18 — 100 stubs EA-sorted asc gap filler not yet in audio (FMOD|Sound|Audio exhausted, global gap filler)
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 100 not in audio after 0x662800 | rbx_core::SharedPtr not boost
//! Range 0x66e380..0x670c04 | existing 37123 -> 37223 distinct
//! Batch: 100 stubs | // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR demangled")

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
use crate::generated::flog_asserts;
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
/// `RBX::TextBox`/`GuiTextMixin` text state behind the Batch-Q
/// descriptors: the `XAlignment` id (member `getXAlignment`, IDA
/// 0x66871c), the two bool members (`TextWrap`/`TextScaled`; the
/// deprecated `TextWrapped` alias shares the `TextWrap` member —
/// grounded: no `setTextWrapped` exists and its descriptor carries
/// the deprecated attributes, IDA 0x67282c-0x672856) and the
/// `TextStrokeTransparency` (sole float descriptor in the TU, IDA
/// 0x672ae4). Defaults ride the base init (host: cleared;
/// `Left` = 0 is grounded).
#[derive(Debug, Clone, Default)]
pub struct TextBoxState {
    pub x_alignment: u32,
    pub text_wrap: bool,
    pub text_scaled: bool,
    pub text_stroke_transparency: f32,
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
/// 0x66ee04): same identity-only shape over the single float
/// member (`TextStrokeTransparency`).
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
pub fn stub_066ef4c(state: &TextBoxState) -> f32 {
    // IDA 0x66ef4c (`GetSetImpl<float>::getValue`): the
    // member-pointer resolve tail-calling the getter. The member is
    // `getTextStrokeTransparency` (the only float object in the TU,
    // IDA a_270 0x672ae4); the pointer folds into the field.
    state.text_stroke_transparency
}

// 0x66ef78 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,float>::GetSetImpl<float (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,float>::GetSetImpl<float (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf")]
pub fn stub_066ef78(state: &mut TextBoxState, value: f32) {
    // IDA 0x66ef78 (`GetSetImpl<float>::setValue`): the
    // member-pointer resolve tail-calling the setter with the input
    // word. The member is `setTextStrokeTransparency`; the pointer
    // folds into the field.
    state.text_stroke_transparency = value;
}

// 0x66ef9c — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D6Color3EEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Color3>::PropDescriptor<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(G3D::Color3)>(char const*,char const*,G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(G3D::Color3),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Color3>::PropDescriptor<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(G3D::Color3)>(char const*,char const*,G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(G3D::Color3),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D6Color3EEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_066ef9c() -> ! {
    todo!("0x66ef9c RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Color3>::PropDescriptor<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(G3D::Color3)>(char const*,char const*,G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(G3D::Color3),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
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
pub fn stub_066f0dc() -> ! {
    todo!("0x66f0dc RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(G3D::Color3)>::isReadOnly(void)const")
}

// 0x66f0e0 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D6Color3EE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(G3D::Color3)>::isWriteOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(G3D::Color3)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D6Color3EE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
pub fn stub_066f0e0() -> ! {
    todo!("0x66f0e0 RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(G3D::Color3)>::isWriteOnly(void)const")
}

// 0x66f0e4 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D6Color3EE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(G3D::Color3)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(G3D::Color3)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D6Color3EE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_066f0e4() -> ! {
    todo!("0x66f0e4 RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(G3D::Color3)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x66f11c — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D6Color3EE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(G3D::Color3)>::setValue(RBX::Reflection::DescribedBase *,G3D::Color3 const&)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(G3D::Color3)>::setValue(RBX::Reflection::DescribedBase *,G3D::Color3 const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D6Color3EE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_066f11c() -> ! {
    todo!("0x66f11c RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(G3D::Color3)>::setValue(RBX::Reflection::DescribedBase *,G3D::Color3 const&)const")
}

// 0x66f158 — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEEC2IMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::PropDescriptor<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor)>(char const*,char const*,RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::PropDescriptor<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor)>(char const*,char const*,RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEEC2IMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_066f158() -> ! {
    todo!("0x66f158 RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::PropDescriptor<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor)>(char const*,char const*,RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
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
pub fn stub_066f298() -> ! {
    todo!("0x66f298 RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor)>::isReadOnly(void)const")
}

// 0x66f29c — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor)>::isWriteOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE11isWriteOnlyEv")]
pub fn stub_066f29c() -> ! {
    todo!("0x66f29c RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor)>::isWriteOnly(void)const")
}

// 0x66f2a0 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_066f2a0() -> ! {
    todo!("0x66f2a0 RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x66f2d8 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor)>::setValue(RBX::Reflection::DescribedBase *,RBX::BrickColor const&)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor)>::setValue(RBX::Reflection::DescribedBase *,RBX::BrickColor const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")]
pub fn stub_066f2d8() -> ! {
    todo!("0x66f2d8 RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::BrickColor)>::setValue(RBX::Reflection::DescribedBase *,RBX::BrickColor const&)const")
}

// 0x66f2fc — __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::EnumPropDescriptor<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font)>(char const*,char const*,RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, int, int, int, int, char, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::EnumPropDescriptor<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font)>(char const*,char const*,RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_066f2fc() -> ! {
    todo!("0x66f2fc RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::EnumPropDescriptor<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font)>(char const*,char const*,RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
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
pub fn stub_066f4dc() -> ! {
    todo!("0x66f4dc RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::isReadOnly(void)const")
}

// 0x66f4ec — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE11isWriteOnlyEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::isWriteOnly(void)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE11isWriteOnlyEv")]
pub fn stub_066f4ec() -> ! {
    todo!("0x66f4ec RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::isWriteOnly(void)const")
}

// 0x66f4fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE11equalValuesEPKNS0_13DescribedBaseES8_
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE11equalValuesEPKNS0_13DescribedBaseES8_")]
pub fn stub_066f4fc() -> ! {
    todo!("0x66f4fc RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x66f524 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_066f524() -> ! {
    todo!("0x66f524 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x66f548 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_066f548() -> ! {
    todo!("0x66f548 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x66f694 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE9copyValueEPKNS0_13DescribedBaseEPS6_
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE9copyValueEPKNS0_13DescribedBaseEPS6_")]
pub fn stub_066f694() -> ! {
    todo!("0x66f694 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x66f6b8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE14hasStringValueEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::hasStringValue(void)const
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE14hasStringValueEv")]
pub fn stub_066f6b8() -> ! {
    todo!("0x66f6b8 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::hasStringValue(void)const")
}

// 0x66f6bc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE14getStringValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::getStringValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_066f6bc() -> ! {
    todo!("0x66f6bc RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x66f6e0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE14setStringValueEPNS0_13DescribedBaseERKSs
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_066f6e0() -> ! {
    todo!("0x66f6e0 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x66f720 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_066f720() -> ! {
    todo!("0x66f720 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x66f740 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_066f740() -> ! {
    todo!("0x66f740 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x66f980 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE13getIndexValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::getIndexValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_066f980() -> ! {
    todo!("0x66f980 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x66f99c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE13setIndexValueEPNS0_13DescribedBaseEm
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE13setIndexValueEPNS0_13DescribedBaseEm")]
pub fn stub_066f99c() -> ! {
    todo!("0x66f99c RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")
}

// 0x66f9d0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE12getEnumValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::getEnumValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub fn stub_066f9d0() -> ! {
    todo!("0x66f9d0 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x66f9d8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE12setEnumValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub fn stub_066f9d8() -> ! {
    todo!("0x66f9d8 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x66fa24 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE11getEnumItemEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::getEnumItem(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub fn stub_066fa24() -> ! {
    todo!("0x66fa24 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}

// 0x66fa44 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub fn stub_066fa44() -> ! {
    todo!("0x66fa44 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

// 0x66fa78 — __ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE14convertToValueERKNS_4NameERS3_
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToValue(RBX::Name const&,RBX::TextService::Font&)const
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToValue(RBX::Name const&,RBX::TextService::Font&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_066fa78() -> ! {
    todo!("0x66fa78 RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToValue(RBX::Name const&,RBX::TextService::Font&)const")
}

// 0x66faf4 — __ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE13convertToItemERKS3_
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToItem(RBX::TextService::Font const&)const
// type: int __fastcall(int, int *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToItem(RBX::TextService::Font const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE13convertToItemERKS3_")]
pub fn stub_066faf4() -> ! {
    todo!("0x66faf4 RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToItem(RBX::TextService::Font const&)const")
}

// 0x66fbc0 — __ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE14convertToIndexES3_
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToIndex(RBX::TextService::Font)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToIndex(RBX::TextService::Font)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE14convertToIndexES3_")]
pub fn stub_066fbc0() -> ! {
    todo!("0x66fbc0 RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToIndex(RBX::TextService::Font)const")
}

// 0x66fc30 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE11setIntValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setIntValue(RBX::Reflection::DescribedBase *,int)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_066fc30() -> ! {
    todo!("0x66fc30 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x66fc70 — __ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE15convertToStringERKS3_
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToString(RBX::TextService::Font const&)const
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToString(RBX::TextService::Font const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE15convertToStringERKS3_")]
pub fn stub_066fc70() -> ! {
    todo!("0x66fc70 RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToString(RBX::TextService::Font const&)const")
}

// 0x66fe10 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font)>::isReadOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
pub fn stub_066fe10() -> ! {
    todo!("0x66fe10 RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font)>::isReadOnly(void)const")
}

// 0x66fe14 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font)>::isWriteOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
pub fn stub_066fe14() -> ! {
    todo!("0x66fe14 RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font)>::isWriteOnly(void)const")
}

// 0x66fe18 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_066fe18() -> ! {
    todo!("0x66fe18 RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x66fe44 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextService::Font const&)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextService::Font const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_066fe44() -> ! {
    todo!("0x66fe44 RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::Font)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextService::Font const&)const")
}

// 0x66fe68 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService4FontEEEE13initSingletonEv
// demangled: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::Font> const>::initSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::Font> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService4FontEEEE13initSingletonEv")]
pub fn stub_066fe68() -> ! {
    todo!("0x66fe68 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::Font> const>::initSingleton(void)")
}

// 0x66fe6c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService4FontEEEE14doGetSingletonEv
// demangled: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::Font> const>::doGetSingleton(void)
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::Font> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService4FontEEEE14doGetSingletonEv")]
pub fn stub_066fe6c() -> ! {
    todo!("0x66fe6c RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::Font> const>::doGetSingleton(void)")
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
pub fn stub_06701d4() -> ! {
    todo!("0x6701d4 RBX::Reflection::EnumDesc<RBX::TextService::Font>::lookup(char const*)const")
}

// 0x670204 — __ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE6lookupERKNS0_7VariantE
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::Font>::lookup(RBX::Reflection::Variant const&)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE6lookupERKNS0_7VariantE")]
pub fn stub_0670204() -> ! {
    todo!("0x670204 RBX::Reflection::EnumDesc<RBX::TextService::Font>::lookup(RBX::Reflection::Variant const&)const")
}

// 0x670224 — __ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE14convertToValueEmRNS0_7VariantE
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// type: int __fastcall(int, unsigned int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_0670224() -> ! {
    todo!("0x670224 RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0x670258 — __ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE15convertToStringEmRSs
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToString(unsigned long,std::string &)const
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE15convertToStringEmRSs")]
pub fn stub_0670258() -> ! {
    todo!("0x670258 RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToString(unsigned long,std::string &)const")
}

// 0x67039c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService4FontEEERS3_RKT_
// demangled: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TextService::Font>(RBX::TextService::Font const&)
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TextService::Font>(RBX::TextService::Font const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService4FontEEERS3_RKT_")]
pub fn stub_067039c() -> ! {
    todo!("0x67039c rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TextService::Font>(RBX::TextService::Font const&)")
}

// 0x6703ec — __ZN3rbx14implementation12typed_holderIN3RBX11TextService4FontEE9singletonEv
// demangled: rbx::implementation::typed_holder<RBX::TextService::Font>::singleton(void)
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::TextService::Font>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService4FontEE9singletonEv")]
pub fn stub_06703ec() -> ! {
    todo!("0x6703ec rbx::implementation::typed_holder<RBX::TextService::Font>::singleton(void)")
}

// 0x670458 — __ZN3rbx14implementation12typed_holderIN3RBX11TextService4FontEE14construct_funcEPKcPc
// demangled: rbx::implementation::typed_holder<RBX::TextService::Font>::construct_func(char const*,char *)
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::TextService::Font>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService4FontEE14construct_funcEPKcPc")]
pub fn stub_0670458() -> ! {
    todo!("0x670458 rbx::implementation::typed_holder<RBX::TextService::Font>::construct_func(char const*,char *)")
}

// 0x670464 — __ZN3rbx14implementation12typed_holderIN3RBX11TextService4FontEE13destruct_funcEPc
// demangled: rbx::implementation::typed_holder<RBX::TextService::Font>::destruct_func(char *)
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::TextService::Font>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService4FontEE13destruct_funcEPc")]
pub fn stub_0670464() -> ! {
    todo!("0x670464 rbx::implementation::typed_holder<RBX::TextService::Font>::destruct_func(char *)")
}

// 0x670468 — __ZN3rbx8any_castIRKN3RBX11TextService4FontENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// demangled: RBX::TextService::Font const& rbx::any_cast<RBX::TextService::Font const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::TextService::Font const& rbx::any_cast<RBX::TextService::Font const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX11TextService4FontENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0670468() -> ! {
    todo!("0x670468 RBX::TextService::Font const& rbx::any_cast<RBX::TextService::Font const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
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
pub fn stub_0670580() -> ! {
    todo!("0x670580 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::EnumPropDescriptor<RBX::TextService::FontSize (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::FontSize)>(char const*,char const*,RBX::TextService::FontSize (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::FontSize),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
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
pub fn stub_0670760() -> ! {
    todo!("0x670760 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::isReadOnly(void)const")
}

// 0x670770 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE11isWriteOnlyEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::isWriteOnly(void)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE11isWriteOnlyEv")]
pub fn stub_0670770() -> ! {
    todo!("0x670770 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::isWriteOnly(void)const")
}

// 0x670780 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE11equalValuesEPKNS0_13DescribedBaseES8_
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE11equalValuesEPKNS0_13DescribedBaseES8_")]
pub fn stub_0670780() -> ! {
    todo!("0x670780 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x6707a8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_06707a8() -> ! {
    todo!("0x6707a8 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x6707cc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_06707cc() -> ! {
    todo!("0x6707cc RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x670918 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE9copyValueEPKNS0_13DescribedBaseEPS6_
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE9copyValueEPKNS0_13DescribedBaseEPS6_")]
pub fn stub_0670918() -> ! {
    todo!("0x670918 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x67093c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE14hasStringValueEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::hasStringValue(void)const
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE14hasStringValueEv")]
pub fn stub_067093c() -> ! {
    todo!("0x67093c RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::hasStringValue(void)const")
}

// 0x670940 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE14getStringValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::getStringValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_0670940() -> ! {
    todo!("0x670940 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x670964 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE14setStringValueEPNS0_13DescribedBaseERKSs
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_0670964() -> ! {
    todo!("0x670964 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x6709a4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_06709a4() -> ! {
    todo!("0x6709a4 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x6709c4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_06709c4() -> ! {
    todo!("0x6709c4 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x670c04 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE13getIndexValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::getIndexValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_0670c04() -> ! {
    todo!("0x670c04 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}
