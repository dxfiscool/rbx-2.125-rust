//! audio generated_audio_wd_watchdog_Y -- 100 stubs EA-sorted asc gap filler distinct from existing watchdogs
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 100 not in audio after 0x670c04 | rbx_core::SharedPtr not boost
//! Range 0x670c20..0x674a4c | distinct EA gap filler
//! Batch: 100 stubs | // 0xADDR -- mangled + #[doc(alias = "demangled")] sanitized (backtick and single-quote removed) + todo!("0xADDR mangled")

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
use crate::generated::flog_asserts;
use crate::generated_audio_wd_watchdog18::{
    stub_0670964, FontSizeVariant, GuiTextButtonState, TextBoxBoolProp, TextBoxBoolSlot, TextBoxEventDesc,
    TextBoxState, TextBoxStringProp, TextBoxVoidFunc, FONTSIZE_ITEMS, fontsize_index, fontsize_name,
};
use crate::generated_audio_wd_watchdog19::stub_665da0;
const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x670c20 -- __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE13setIndexValueEPNS0_13DescribedBaseEm
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE13setIndexValueEPNS0_13DescribedBaseEm")]
pub fn stub_670c20(state: &mut TextBoxState, index: u32) -> bool {
    // IDA 0x670c20 (`EnumPropDescriptor<FontSize>::setIndexValue`):
    // bounds-checks the index against the item count, stores
    // `items[index]` through the inner `setValue` and returns 1,
    // else 0. Same shape as the `YAlignment` twin at 0x66ce28.
    match FONTSIZE_ITEMS.get(index as usize) {
        Some((_, size)) => {
            state.font_size = *size;
            true
        }
        None => false,
    }
}

// 0x670c54 -- __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE12getEnumValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::getEnumValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub fn stub_670c54(state: &TextBoxState) -> u32 {
    // IDA 0x670c54 (`EnumPropDescriptor<FontSize>::getEnumValue`):
    // the inner `getValue` straight through.
    state.font_size
}

// 0x670c5c -- __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE12setEnumValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub fn stub_670c5c(state: &mut TextBoxState, value: u32) -> bool {
    // IDA 0x670c5c (`EnumPropDescriptor<FontSize>::setEnumValue`):
    // `find_if` with `equalValue` over the items; on a hit the
    // inner `setValue` runs and 1 returns, else 0. Host:
    // membership decides.
    if FONTSIZE_ITEMS.iter().any(|(_, v)| *v == value) {
        state.font_size = value;
        true
    } else {
        false
    }
}

// 0x670ca8 -- __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE11getEnumItemEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::getEnumItem(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub fn stub_670ca8(state: &TextBoxState) -> i32 {
    // IDA 0x670ca8 (`EnumPropDescriptor<FontSize>::getEnumItem`):
    // inner `getValue` + `EnumDesc::convertToItem`. Host: the item
    // position of the live value (-1 when missing).
    fontsize_index(state.font_size)
}

// 0x670cc8 -- __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub fn stub_670cc8(state: &mut TextBoxState, name: &str) -> bool {
    // IDA 0x670cc8 (`EnumPropDescriptor<FontSize>::setStringValue`
    // over `Name`): `convertToValue` on the name; on a hit the
    // inner `setValue` runs and 1 returns, else 0. Same string edge
    // as 0x670964 — host forwards into that twin (`Name` folds into
    // `&str`).
    stub_0670964(state, name)
}

// 0x670cfc -- __ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE14convertToValueERKNS_4NameERS3_
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::convertToValue(RBX::Name const&,RBX::TextService::FontSize&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::convertToValue(RBX::Name const&,RBX::TextService::FontSize&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_670cfc(name: &str) -> Option<u32> {
    // IDA 0x670cfc (`EnumDesc<FontSize>::convertToValue(Name)`):
    // red-black-tree search for the name id, returning 1 with the
    // value on a hit (else 0). Host: the table value (`None` on a
    // miss).
    FONTSIZE_ITEMS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, v)| *v)
}

// 0x670d78 -- __ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE13convertToItemERKS3_
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::convertToItem(RBX::TextService::FontSize const&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::convertToItem(RBX::TextService::FontSize const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE13convertToItemERKS3_")]
pub fn stub_670d78(value: u32) -> i32 {
    // IDA 0x670d78 (`EnumDesc<FontSize>::convertToItem`):
    // FLog::Asserts-gated `value>=0` (enumconverter.h:273) and
    // `value<enumToItem.size` (line 274) ReleaseAsserts — host
    // seams; then the value-indexed item. Host: asserts + table
    // index.
    if flog_asserts() {
        assert!(
            value < FONTSIZE_ITEMS.len() as u32,
            "(size_t)value<enumToItem.size() file: include/reflection/enumconverter.h line: 274 (IDA 0x670d78)"
        );
    }
    fontsize_index(value)
}

// 0x670e44 -- __ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE14convertToIndexES3_
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::convertToIndex(RBX::TextService::FontSize)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::convertToIndex(RBX::TextService::FontSize)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE14convertToIndexES3_")]
pub fn stub_670e44(value: u32) -> i32 {
    // IDA 0x670e44 (`EnumDesc<FontSize>::convertToIndex`):
    // FLog::Asserts-gated `value>=0` (enumconverter.h:350) — host
    // seam; then the value-indexed `enumToItem` entry or -1 past
    // the end. Host: the table index (-1 on a miss).
    fontsize_index(value)
}

// 0x670eb4 -- __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE11setIntValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::setIntValue(RBX::Reflection::DescribedBase *,int)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_670eb4(state: &mut TextBoxState, index: i32) -> bool {
    // IDA 0x670eb4 (`EnumPropDescriptor<FontSize>::setIntValue`):
    // rejects negative indices, bounds-checks against the item
    // count and rejects `-1`-valued items, then stores through the
    // inner `setValue` and returns 1, else 0. Table values are
    // non-negative by type, so the `-1` check folds away.
    if index >= 0 {
        if let Some((_, size)) = FONTSIZE_ITEMS.get(index as usize) {
            state.font_size = *size;
            return true;
        }
    }
    false
}

// 0x670ef4 -- __ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE15convertToStringERKS3_
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::convertToString(RBX::TextService::FontSize const&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::convertToString(RBX::TextService::FontSize const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE15convertToStringERKS3_")]
pub fn stub_670ef4(value: i32) -> String {
    // IDA 0x670ef4 (`EnumDesc<FontSize>::convertToString`):
    // FLog::Asserts-gated `value >= 0` (enumconverter.h:262) and
    // `value < enumToItem.size` (line 263) ReleaseAsserts — host
    // seams; then a negative value yields "", else the
    // value-indexed item name. Same shape as the `XAlignment` twin
    // at 0x66e380. Host: asserts + table name with "" fallback.
    if flog_asserts() {
        assert!(
            value >= 0,
            "value>=0 file: include/reflection/enumconverter.h line: 262 (IDA 0x670ef4)"
        );
        assert!(
            (value as usize) < FONTSIZE_ITEMS.len(),
            "(size_t)value<enumToItem.size() file: include/reflection/enumconverter.h line: 263 (IDA 0x670ef4)"
        );
    }
    if value >= 0 {
        fontsize_name(value as u32).to_owned()
    } else {
        String::new()
    }
}

// 0x671094 -- __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::GetSetImpl<RBX::TextService::FontSize (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::FontSize)>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::GetSetImpl<RBX::TextService::FontSize (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::FontSize)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
pub fn stub_671094() -> bool {
    // IDA 0x671094 (`GetSetImpl<FontSize>::isReadOnly`): returns
    // constant 0.
    false
}

// 0x671098 -- __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::GetSetImpl<RBX::TextService::FontSize (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::FontSize)>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::GetSetImpl<RBX::TextService::FontSize (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::FontSize)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
pub fn stub_671098() -> bool {
    // IDA 0x671098 (`GetSetImpl<FontSize>::isWriteOnly`): returns
    // constant 0.
    false
}

// 0x67109c -- __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::GetSetImpl<RBX::TextService::FontSize (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::FontSize)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::GetSetImpl<RBX::TextService::FontSize (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::FontSize)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_67109c(state: &TextBoxState) -> u32 {
    // IDA 0x67109c (`GetSetImpl<FontSize>::getValue`): the
    // member-pointer resolve (null described reads at offset 0 with
    // the +536 `Instance`-to-mixin adjust, 0x67109e-0x6710ac;
    // virtual when the low bit is set) tail-calling the getter. The
    // member is `getFontSize` (IDA 0x668620, the only `FontSize`
    // getter); the pointer folds into the field.
    state.font_size
}

// 0x6710c8 -- __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::GetSetImpl<RBX::TextService::FontSize (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::FontSize)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextService::FontSize const&)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::GetSetImpl<RBX::TextService::FontSize (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::FontSize)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextService::FontSize const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_6710c8(state: &mut TextBoxState, value: u32) {
    // IDA 0x6710c8 (`GetSetImpl<FontSize>::setValue`): the
    // member-pointer resolve over +12/+16 tail-calling the setter
    // with the input word. The member is `setFontSize` (the only
    // `FontSize` setter on `TextBox`); the pointer folds into the
    // field (its raises fold into the store).
    state.font_size = value;
}

// 0x6710ec -- __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService8FontSizeEEEE13initSingletonEv
// demangled: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::FontSize> const>::initSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::FontSize> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService8FontSizeEEEE13initSingletonEv")]
pub fn stub_6710ec() {
    // IDA 0x6710ec (`Singleton<EnumDesc<FontSize>>::initSingleton`):
    // thunk tail-calling `doGetSingleton` (host: stub_06710f0).
    // The singleton folds into the host table — carrier no-op.
}

// 0x6710f0 -- __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService8FontSizeEEEE14doGetSingletonEv
// demangled: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::FontSize> const>::doGetSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::FontSize> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService8FontSizeEEEE14doGetSingletonEv")]
pub fn stub_6710f0() {
    // IDA 0x6710f0 (`Singleton<EnumDesc<FontSize>>::doGetSingleton`):
    // `__cxa_guard` once-init constructing the `EnumDesc` and
    // registering `__cxa_atexit` teardown. Host statics initialize
    // on use — carrier no-op.
}

// 0x6711e0 -- __ZN3RBX10Reflection8EnumDescINS_11TextService8FontSizeEED1Ev
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService8FontSizeEED1Ev")]
pub fn stub_6711e0() {
    // IDA 0x6711e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6711e4 -- __ZN3RBX10Reflection8EnumDescINS_11TextService8FontSizeEED2Ev
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService8FontSizeEED2Ev")]
pub fn stub_6711e4() {
    // IDA 0x6711e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6713b8 -- __ZN3RBX10Reflection8EnumDescINS_11TextService8FontSizeEED0Ev
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService8FontSizeEED0Ev")]
pub fn stub_6713b8() {
    // IDA 0x6713b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x671458 -- __ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE6lookupEPKc
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::lookup(char const*)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE6lookupEPKc")]
pub fn stub_671458(name: &str) -> Option<u32> {
    // IDA 0x671458 (`EnumDesc<FontSize>::lookup(name)`):
    // `Name::lookup` + `convertToValue`; on a hit `convertToItem`,
    // else 0 (0x671464-0x671484). Host: the item index (`None` on a
    // miss); the lookup folds into the compare.
    FONTSIZE_ITEMS
        .iter()
        .position(|(n, _)| *n == name)
        .map(|i| i as u32)
}

// 0x671488 -- __ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE6lookupERKNS0_7VariantE
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::lookup(RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE6lookupERKNS0_7VariantE")]
pub fn stub_671488(variant: &FontSizeVariant) -> Option<u32> {
    // IDA 0x671488 (`EnumDesc<FontSize>::lookup(variant)`):
    // `any_cast<FontSize>` then `convertToItem` (0x67149a-0x6714a4).
    // No `any_cast<FontSize>` twin exists yet in the host; the cast
    // folds into the tagged match (miss panics like the `Font` twin
    // at 0x670468). Host: the item index of the cast value.
    let value = match *variant {
        FontSizeVariant::FontSize(value) => value,
        _ => panic!("rbx::bad_placement_any_cast (IDA 0x671488)"),
    };
    FONTSIZE_ITEMS
        .iter()
        .position(|(_, v)| *v == value)
        .map(|i| i as u32)
}

// 0x6714a8 -- __ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE14convertToValueEmRNS0_7VariantE
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_6714a8(index: u32) -> Option<u32> {
    // IDA 0x6714a8 (`EnumDesc<FontSize>::convertToValue(index)`):
    // `count > index` gates reading the indexed item's value plus
    // the `Type` tag and placement, returning 1 (else 0). Host: the
    // value (`None` past the end); the tag/placement fold away.
    FONTSIZE_ITEMS.get(index as usize).map(|(_, v)| *v)
}

// 0x6714dc -- __ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE15convertToStringEmRSs
// demangled: RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::convertToString(unsigned long,std::string &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE15convertToStringEmRSs")]
pub fn stub_6714dc(index: u32, out: &mut String) -> bool {
    // IDA 0x6714dc (`EnumDesc<FontSize>::convertToString(index)`):
    // `count > index` gates reading the value and converting it to
    // a name assigned into the out string, returning 1 (else 0 with
    // `out` untouched). Host: assign on hit, report.
    match FONTSIZE_ITEMS.get(index as usize) {
        Some((_, value)) => {
            *out = fontsize_name(*value).to_owned();
            true
        }
        None => false,
    }
}

// 0x671620 -- __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService8FontSizeEEERS3_RKT_
// demangled: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TextService::FontSize>(RBX::TextService::FontSize const&)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TextService::FontSize>(RBX::TextService::FontSize const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService8FontSizeEEERS3_RKT_")]
pub fn stub_671620(value: u32) -> u32 {
    // IDA 0x671620 (`placement_any::operator=<FontSize>`): ensures
    // the holder singleton, then stores the value and (re)tags the
    // holder (destroying the old payload first). Host values are
    // `Copy` with the tag in the type — the move is identity.
    value
}

// 0x671670 -- __ZN3rbx14implementation12typed_holderIN3RBX11TextService8FontSizeEE9singletonEv
// demangled: rbx::implementation::typed_holder<RBX::TextService::FontSize>::singleton(void)
#[doc(alias = "rbx::implementation::typed_holder<RBX::TextService::FontSize>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService8FontSizeEE9singletonEv")]
pub fn stub_671670() {
    // IDA 0x671670 (`typed_holder<FontSize>::singleton`):
    // `__cxa_guard` once-init publishing the typeinfo and the
    // construct/destruct funcs. Host type tags need no init —
    // carrier no-op.
}

// 0x6716dc -- __ZN3rbx14implementation12typed_holderIN3RBX11TextService8FontSizeEE14construct_funcEPKcPc
// demangled: rbx::implementation::typed_holder<RBX::TextService::FontSize>::construct_func(char const*,char *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::TextService::FontSize>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService8FontSizeEE14construct_funcEPKcPc")]
pub fn stub_6716dc() {
    // IDA 0x6716dc (`typed_holder<FontSize>::construct_func`):
    // copies the held value pointer when non-null. Host values are
    // `Copy` — carrier no-op.
}

// 0x6716e8 -- __ZN3rbx14implementation12typed_holderIN3RBX11TextService8FontSizeEE13destruct_funcEPc
// demangled: rbx::implementation::typed_holder<RBX::TextService::FontSize>::destruct_func(char *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::TextService::FontSize>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11TextService8FontSizeEE13destruct_funcEPc")]
pub fn stub_6716e8() {
    // IDA 0x6716e8 (`typed_holder<FontSize>::destruct_func`):
    // empty body — carrier no-op.
}

// 0x6716ec -- __ZN3rbx8any_castIRKN3RBX11TextService8FontSizeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// demangled: RBX::TextService::FontSize const& rbx::any_cast<RBX::TextService::FontSize const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
#[doc(alias = "RBX::TextService::FontSize const& rbx::any_cast<RBX::TextService::FontSize const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX11TextService8FontSizeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_6716ec(variant: &FontSizeVariant) -> u32 {
    // IDA 0x6716ec (`any_cast<FontSize>`): null input misses; the
    // typeinfo-pointer or mangled-name
    // (`N3RBX11TextService8FontSizeE`) match returns the payload,
    // else `bad_placement_any_cast` is thrown (host: panic). Host:
    // tagged match. Same shape as the `Font` twin at 0x670468.
    match *variant {
        FontSizeVariant::FontSize(value) => value,
        _ => panic!("rbx::bad_placement_any_cast (IDA 0x6716ec)"),
    }
}

// 0x6717dc -- __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService8FontSizeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::FontSize>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::FontSize>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::FontSize>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TextService::FontSize>> *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::FontSize>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::FontSize>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::FontSize>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TextService::FontSize>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService8FontSizeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_6717dc() {
    // IDA 0x6717dc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x671804 -- __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxESsEC2IMNS_12GuiTextMixinEKFSsvEMS2_FvSsEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,std::string>::PropDescriptor<std::string (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(std::string)>(char const*,char const*,std::string (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,std::string>::PropDescriptor<std::string (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(std::string)>(char const*,char const*,std::string (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextBoxESsEC2IMNS_12GuiTextMixinEKFSsvEMS2_FvSsEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_671804(name: &str, category: &str, attributes: u32, permissions: u32) -> TextBoxStringProp {
    // IDA 0x671804 (`PropDescriptor<TextBox, string>::PropDescriptor`):
    // builds the `GetSetImpl` member-pair cell plus the typed
    // descriptor identity with name/category/attributes/permissions.
    // The pair folds into the `text` field (`getText`/`setText`).
    // Host: the identity half.
    TextBoxStringProp::new(name, category, attributes, permissions)
}

// 0x671918 -- __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxESsED0Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,std::string>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,std::string>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextBoxESsED0Ev")]
pub fn stub_671918() {
    // IDA 0x671918: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x671944 -- __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxESsE10GetSetImplIMNS_12GuiTextMixinEKFSsvEMS2_FvSsEE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,std::string>::GetSetImpl<std::string (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(std::string)>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,std::string>::GetSetImpl<std::string (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(std::string)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxESsE10GetSetImplIMNS_12GuiTextMixinEKFSsvEMS2_FvSsEE10isReadOnlyEv")]
pub fn stub_671944() -> bool {
    // IDA 0x671944 (`GetSetImpl<string>::isReadOnly`): returns
    // constant 0 (0x671946).
    false
}

// 0x671948 -- __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxESsE10GetSetImplIMNS_12GuiTextMixinEKFSsvEMS2_FvSsEE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,std::string>::GetSetImpl<std::string (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(std::string)>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,std::string>::GetSetImpl<std::string (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(std::string)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxESsE10GetSetImplIMNS_12GuiTextMixinEKFSsvEMS2_FvSsEE11isWriteOnlyEv")]
pub fn stub_671948() -> bool {
    // IDA 0x671948 (`GetSetImpl<string>::isWriteOnly`): returns
    // constant 0 (0x67194a).
    false
}

// 0x67194c -- __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxESsE10GetSetImplIMNS_12GuiTextMixinEKFSsvEMS2_FvSsEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,std::string>::GetSetImpl<std::string (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(std::string)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,std::string>::GetSetImpl<std::string (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(std::string)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxESsE10GetSetImplIMNS_12GuiTextMixinEKFSsvEMS2_FvSsEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_67194c(state: &TextBoxState) -> String {
    // IDA 0x67194c (`GetSetImpl<string>::getValue`): the
    // member-pointer resolve (null described reads at offset 0 with
    // the +536 `Instance`-to-mixin adjust, 0x671952-0x671962;
    // virtual when the low bit is set) tail-calling the getter. The
    // member is `getText` (IDA 0x6685f0, the only string getter);
    // the pointer folds into the field.
    state.text.clone()
}

// 0x671984 -- __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxESsE10GetSetImplIMNS_12GuiTextMixinEKFSsvEMS2_FvSsEE8setValueEPNS0_13DescribedBaseERKSs
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,std::string>::GetSetImpl<std::string (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(std::string)>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,std::string>::GetSetImpl<std::string (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(std::string)>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxESsE10GetSetImplIMNS_12GuiTextMixinEKFSsvEMS2_FvSsEE8setValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_671984(state: &mut TextBoxState, text: &str, filter_pass: bool) {
    // IDA 0x671984 (`GetSetImpl<string>::setValue`): copies the
    // input and tail-calls the setter member-pointer (0x6719fc-0x671a08).
    // The member is `setText` (the only string setter on `TextBox`);
    // the pointer folds into the 0x665da0 twin with its filter seam.
    stub_665da0(state, text, filter_pass);
}

// 0x671ac8 -- __ZN3RBX10Reflection9EventDescINS_7TextBoxEFvbEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// demangled: RBX::Reflection::EventDesc<RBX::TextBox,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::TextBox::*>::EventDesc(rbx::signal<void ()(bool)> RBX::TextBox::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::TextBox,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::TextBox::*>::EventDesc(rbx::signal<void ()(bool)> RBX::TextBox::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7TextBoxEFvbEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_671ac8(name: &str) -> TextBoxEventDesc {
    // IDA 0x671ac8 (`EventDesc<TextBox, void(bool)>` C2): builds the
    // `EventDescriptor` with the `void(bool)` signature item
    // (0x671afa-0x671bda, host: the `Focused` bool edge). The
    // signal member and the registry wiring fold away; host keeps
    // the event identity.
    TextBoxEventDesc::new(name)
}

// 0x671c4c -- __ZN3RBX10Reflection9EventDescINS_7TextBoxEFvbEN3rbx6signalIS3_EEMS2_S6_ED0Ev
// demangled: RBX::Reflection::EventDesc<RBX::TextBox,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::TextBox::*>::~EventDesc()
#[doc(alias = "RBX::Reflection::EventDesc<RBX::TextBox,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::TextBox::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7TextBoxEFvbEN3rbx6signalIS3_EEMS2_S6_ED0Ev")]
pub fn stub_671c4c() {
    // IDA 0x671c4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x671d00 -- __ZNK3RBX10Reflection13EventDescImplILi1ENS_7TextBoxEFvbEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// demangled: RBX::Reflection::EventDescImpl<1,RBX::TextBox,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::TextBox::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::TextBox,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::TextBox::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_7TextBoxEFvbEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_671d00() {
    // IDA 0x671d00 (`EventDescImpl::connectGeneric`): wraps the
    // `GenericSlotWrapper` in a bool closure and connects it to the
    // member signal (0x671d8c-0x671daa). Connection management folds
    // into the host fire-closure seams. Carrier no-op.
}

// 0x671e54 -- __ZNK3RBX10Reflection13EventDescImplILi1ENS_7TextBoxEFvbEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// demangled: RBX::Reflection::EventDescImpl<1,RBX::TextBox,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::TextBox::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::TextBox,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::TextBox::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_7TextBoxEFvbEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
pub fn stub_671e54(fire_focused: impl Fn(bool), submitted: bool) {
    // IDA 0x671e54 (`EventDescImpl::fireEvent`): asserts a single
    // arg (Event.h:320), `any_cast`s the bool out of the variant
    // vector (0x671ece) and fires the member `signal_with_args`
    // (0x671ed2). Host: the `Focused(bool)` edge — the arg count
    // and cast fold into the params.
    fire_focused(submitted);
}

// 0x671ee0 -- __ZNK3RBX10Reflection13EventDescBaseINS_7TextBoxEFvbEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// demangled: RBX::Reflection::EventDescBase<RBX::TextBox,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::TextBox::*>::disconnectAll(RBX::Reflection::EventSource *)const
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::TextBox,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::TextBox::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_7TextBoxEFvbEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_671ee0() {
    // IDA 0x671ee0 (`EventDescBase::disconnectAll`): forwards to the
    // member signal's teardown. Connections fold into the host
    // fire-closure seams. Carrier no-op.
}

// 0x671ef4 -- __ZN3RBX10Reflection13BoundFuncDescINS_7TextBoxEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// demangled: RBX::Reflection::BoundFuncDesc<RBX::TextBox,void ()(void),0>::BoundFuncDesc(void (RBX::TextBox::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TextBox,void ()(void),0>::BoundFuncDesc(void (RBX::TextBox::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_7TextBoxEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_671ef4(name: &str, attributes: u32, permissions: u32) -> TextBoxVoidFunc {
    // IDA 0x671ef4 (`BoundFuncDesc<TextBox, void(), 0>` C2): the
    // `TextBox` `classDescriptor` call plus the
    // `FunctionDescriptor` init with the member-pointer pair at +40
    // and a void return tag at +28 (0x671f1a-0x671faa). The pair
    // folds into the bound call; host keeps name/attributes/
    // permissions.
    TextBoxVoidFunc::new(name, attributes, permissions)
}

// 0x671ff8 -- __ZN3RBX10Reflection13BoundFuncDescINS_7TextBoxEFvvELi0EED0Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::TextBox,void ()(void),0>::~BoundFuncDesc()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TextBox,void ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_7TextBoxEFvvELi0EED0Ev")]
pub fn stub_671ff8() {
    // IDA 0x671ff8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6720ac -- __ZNK3RBX10Reflection13BoundFuncDescINS_7TextBoxEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// demangled: RBX::Reflection::BoundFuncDesc<RBX::TextBox,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TextBox,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_7TextBoxEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_6720ac(state: &mut TextBoxState, input_service_created: bool, fire_focused: impl Fn()) {
    // IDA 0x6720ac (`BoundFuncDesc<TextBox, void(), 0>::execute`):
    // dispatches the stored member-pointer over the object
    // (0x6720ac-0x6720c6). The bound member is `captureFocus`
    // (grounded: the global ctor at 0x672440 binds "CaptureFocus"
    // to it); the pointer folds into the 0x665c98 twin.
    crate::generated_audio_wd_watchdog19::stub_665c98(state, input_service_created, fire_focused);
}

// 0x6720cc -- __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::PropDescriptor<bool (RBX::TextBox::*)(void)const,void (RBX::TextBox::*)(bool)>(char const*,char const*,bool (RBX::TextBox::*)(void)const,void (RBX::TextBox::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::PropDescriptor<bool (RBX::TextBox::*)(void)const,void (RBX::TextBox::*)(bool)>(char const*,char const*,bool (RBX::TextBox::*)(void)const,void (RBX::TextBox::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_6720cc(name: &str, category: &str, attributes: u32, permissions: u32) -> TextBoxBoolProp {
    // IDA 0x6720cc (`PropDescriptor<TextBox, bool>::PropDescriptor`
    // with get+set pair): builds the `GetSetImpl` member-pair cell
    // plus the typed descriptor identity with name/category/
    // attributes/permissions. The pair folds into the caller's
    // `TextBoxBoolSlot`. Host: the identity half (same shape as the
    // getter-only C2 at 0x66c194).
    TextBoxBoolProp::new(name, category, attributes, permissions)
}

// 0x6721e0 -- __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetSetImpl<bool (RBX::TextBox::*)(void)const,void (RBX::TextBox::*)(bool)>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetSetImpl<bool (RBX::TextBox::*)(void)const,void (RBX::TextBox::*)(bool)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv")]
pub fn stub_6721e0() -> bool {
    // IDA 0x6721e0 (`GetSetImpl<bool(), void(bool)>::isReadOnly`):
    // returns constant 0 (0x6721e2).
    false
}

// 0x6721e4 -- __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetSetImpl<bool (RBX::TextBox::*)(void)const,void (RBX::TextBox::*)(bool)>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetSetImpl<bool (RBX::TextBox::*)(void)const,void (RBX::TextBox::*)(bool)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv")]
pub fn stub_6721e4() -> bool {
    // IDA 0x6721e4 (`GetSetImpl<bool(), void(bool)>::isWriteOnly`):
    // returns constant 0 (0x6721e6).
    false
}

// 0x6721e8 -- __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetSetImpl<bool (RBX::TextBox::*)(void)const,void (RBX::TextBox::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetSetImpl<bool (RBX::TextBox::*)(void)const,void (RBX::TextBox::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_6721e8(state: &TextBoxState, slot: TextBoxBoolSlot) -> bool {
    // IDA 0x6721e8 (`GetSetImpl<bool(), void(bool)>::getValue`):
    // dispatches the stored getter member-pointer over the object
    // (0x6721ea-0x67220a, host: the `slot` selects the
    // `TextBoxState` bool).
    state.bool_slot(slot)
}

// 0x67220c -- __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// demangled: RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetSetImpl<bool (RBX::TextBox::*)(void)const,void (RBX::TextBox::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetSetImpl<bool (RBX::TextBox::*)(void)const,void (RBX::TextBox::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_67220c(state: &mut TextBoxState, slot: TextBoxBoolSlot, value: bool) -> bool {
    // IDA 0x67220c (`GetSetImpl<bool(), void(bool)>::setValue`):
    // dispatches the stored setter member-pointer with the input
    // word (0x67220c-0x672228, host: the `slot` selects the member
    // setter twin — `setTextWrap`/`setTextScale` at 0x666094/0x6660d4,
    // whose raises fold into the changed flag).
    match slot {
        TextBoxBoolSlot::TextWrap => crate::generated_audio_wd_1788360980::stub_666094(state, value),
        TextBoxBoolSlot::TextScaled => crate::generated_audio_wd_1788360980::stub_6660d4(state, value),
    }
}

// 0x672230 -- __ZN3RBX7TextBoxD2Ev
// demangled: RBX::TextBox::~TextBox()
#[doc(alias = "RBX::TextBox::~TextBox()")]
#[doc(alias = "__ZN3RBX7TextBoxD2Ev")]
pub fn stub_672230() {
    // IDA 0x672230: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x672d68 -- __ZN3RBX13GuiTextButtonC2Ev
// demangled: RBX::GuiTextButton::GuiTextButton(void)
#[doc(alias = "RBX::GuiTextButton::GuiTextButton(void)")]
#[doc(alias = "__ZN3RBX13GuiTextButtonC2Ev")]
pub fn stub_672d68() -> GuiTextButtonState {
    // IDA 0x672d68 (`RBX::GuiTextButton::GuiTextButton`): the
    // `GuiButton` base, vtables, class descriptor and registrar
    // fold away; the member stores ground
    // `GuiTextButtonState::default` — +804 `Text` = "Button",
    // +812..+820 the palette-26 `BrickColor::color3`, +824/+828..
    // zero transparencies/colors, +840 the 1.0 stroke
    // transparency, +844/+845 cleared wrap/scale, +848 = 2 / +852
    // = 1 alignments, +808/+856 cleared font ids.
    GuiTextButtonState::default()
}

// 0x67303c -- __ZN3RBX13GuiTextButton7setTextESs
// demangled: RBX::GuiTextButton::setText(std::string)
#[doc(alias = "RBX::GuiTextButton::setText(std::string)")]
#[doc(alias = "__ZN3RBX13GuiTextButton7setTextESs")]
pub fn stub_67303c(state: &mut GuiTextButtonState, text: &str, filter_pass: bool) {
    // IDA 0x67303c (`RBX::GuiTextButton::setText`): over-0x400
    // inputs are cut down; a profanity hit without the fw+22
    // override skips silently; on difference from the +804 text it
    // assigns it, zeroes word 200 (+800) and raises three
    // descriptors (folds into the mutation). Same shape as the
    // `TextBox` twin at 0x665da0. Host: mutate on change only.
    if !filter_pass {
        return;
    }
    let mut clipped = text.to_owned();
    if clipped.len() > 0x400 {
        let mut end = 0x400;
        while !clipped.is_char_boundary(end) {
            end -= 1;
        }
        clipped.truncate(end);
    }
    if state.text == clipped {
        return;
    }
    state.text = clipped;
}

// 0x6731f8 -- __ZN3RBX13GuiTextButton11setFontSizeENS_11TextService8FontSizeE
// demangled: RBX::GuiTextButton::setFontSize(RBX::TextService::FontSize)
#[doc(alias = "RBX::GuiTextButton::setFontSize(RBX::TextService::FontSize)")]
#[doc(alias = "__ZN3RBX13GuiTextButton11setFontSizeENS_11TextService8FontSizeE")]
pub fn stub_6731f8(state: &mut GuiTextButtonState, font_size: u32) -> bool {
    // IDA 0x6731f8 (`RBX::GuiTextButton::setFontSize`): compares
    // word 202 (+808); on change stores it and raises twice, else
    // returns unchanged. The raises fold into the changed flag.
    if state.font_size == font_size {
        return false;
    }
    state.font_size = font_size;
    true
}

// 0x673230 -- __ZN3RBX13GuiTextButton7setFontENS_11TextService4FontE
// demangled: RBX::GuiTextButton::setFont(RBX::TextService::Font)
#[doc(alias = "RBX::GuiTextButton::setFont(RBX::TextService::Font)")]
#[doc(alias = "__ZN3RBX13GuiTextButton7setFontENS_11TextService4FontE")]
pub fn stub_673230(state: &mut GuiTextButtonState, font: u32) -> bool {
    // IDA 0x673230 (`RBX::GuiTextButton::setFont`): compares word
    // 214 (+856); on change stores it and raises twice, else
    // returns unchanged. The raises fold into the changed flag.
    if state.font == font {
        return false;
    }
    state.font = font;
    true
}

// 0x673268 -- __ZN3RBX13GuiTextButton12setTextColorENS_10BrickColorE
// demangled: RBX::GuiTextButton::setTextColor(RBX::BrickColor)
#[doc(alias = "RBX::GuiTextButton::setTextColor(RBX::BrickColor)")]
#[doc(alias = "__ZN3RBX13GuiTextButton12setTextColorENS_10BrickColorE")]
pub fn stub_673268(state: &mut GuiTextButtonState, text_color: u32) {
    // IDA 0x673268 (`RBX::GuiTextButton::setTextColor`): converts
    // the `BrickColor` id via `BrickColor::color3` (runtime
    // `BrickMap` palette — ungrounded in this range) and delegates
    // to `setTextColor3` (host: stub_0673288). The id itself is
    // cached (reads derive via `closest`; same gap as the `TextBox`
    // twin at 0x665fcc).
    state.text_color = text_color;
}

// 0x673288 -- __ZN3RBX13GuiTextButton13setTextColor3EN3G3D6Color3E
// demangled: RBX::GuiTextButton::setTextColor3(G3D::Color3)
#[doc(alias = "RBX::GuiTextButton::setTextColor3(G3D::Color3)")]
#[doc(alias = "__ZN3RBX13GuiTextButton13setTextColor3EN3G3D6Color3E")]
pub fn stub_673288(state: &mut GuiTextButtonState, color: [f32; 3]) -> bool {
    // IDA 0x673288 (`RBX::GuiTextButton::setTextColor3`): compares
    // words 203-205 (+812); on any difference stores all three and
    // raises twice. The raises fold into the changed flag.
    if state.text_color3 == color {
        return false;
    }
    state.text_color3 = color;
    true
}

// 0x673308 -- __ZN3RBX13GuiTextButton19setTextTransparencyEf
// demangled: RBX::GuiTextButton::setTextTransparency(float)
#[doc(alias = "RBX::GuiTextButton::setTextTransparency(float)")]
#[doc(alias = "__ZN3RBX13GuiTextButton19setTextTransparencyEf")]
pub fn stub_673308(state: &mut GuiTextButtonState, transparency: f32) -> bool {
    // IDA 0x673308 (`RBX::GuiTextButton::setTextTransparency`):
    // compares word 206 (+824); on change stores it and raises,
    // else returns unchanged. The raise folds into the changed flag.
    if state.text_transparency == transparency {
        return false;
    }
    state.text_transparency = transparency;
    true
}

// 0x673330 -- __ZN3RBX13GuiTextButton11setTextWrapEb
// demangled: RBX::GuiTextButton::setTextWrap(bool)
#[doc(alias = "RBX::GuiTextButton::setTextWrap(bool)")]
#[doc(alias = "__ZN3RBX13GuiTextButton11setTextWrapEb")]
pub fn stub_673330(state: &mut GuiTextButtonState, wrap: bool) -> bool {
    // IDA 0x673330 (`RBX::GuiTextButton::setTextWrap`): compares
    // the +844 byte; on change stores it and raises three
    // descriptors, else returns unchanged. The raises fold into the
    // changed flag.
    if state.text_wrap == wrap {
        return false;
    }
    state.text_wrap = wrap;
    true
}

// 0x673370 -- __ZN3RBX13GuiTextButton12setTextScaleEb
// demangled: RBX::GuiTextButton::setTextScale(bool)
#[doc(alias = "RBX::GuiTextButton::setTextScale(bool)")]
#[doc(alias = "__ZN3RBX13GuiTextButton12setTextScaleEb")]
pub fn stub_673370(state: &mut GuiTextButtonState, scale: bool) -> bool {
    // IDA 0x673370 (`RBX::GuiTextButton::setTextScale`): compares
    // the +845 byte; on change stores it, raises, and — when
    // enabling — delegates to `setTextWrap(this, 1)` (host: the
    // 0x673330 twin); disabling raises twice more instead. All
    // raises fold into the changed flag.
    if state.text_scaled == scale {
        return false;
    }
    state.text_scaled = scale;
    if scale {
        stub_673330(state, true);
    }
    true
}

// 0x6733c4 -- __ZN3RBX13GuiTextButton13setXAlignmentENS_11TextService10XAlignmentE
// demangled: RBX::GuiTextButton::setXAlignment(RBX::TextService::XAlignment)
#[doc(alias = "RBX::GuiTextButton::setXAlignment(RBX::TextService::XAlignment)")]
#[doc(alias = "__ZN3RBX13GuiTextButton13setXAlignmentENS_11TextService10XAlignmentE")]
pub fn stub_6733c4(state: &mut GuiTextButtonState, value: u32) -> bool {
    // IDA 0x6733c4 (`RBX::GuiTextButton::setXAlignment`): compares
    // word 212 (+848); on change stores it and raises three
    // descriptors, else returns unchanged. The raises fold into the
    // changed flag.
    if state.x_alignment == value {
        return false;
    }
    state.x_alignment = value;
    true
}

// 0x673404 -- __ZN3RBX13GuiTextButton13setYAlignmentENS_11TextService10YAlignmentE
// demangled: RBX::GuiTextButton::setYAlignment(RBX::TextService::YAlignment)
#[doc(alias = "RBX::GuiTextButton::setYAlignment(RBX::TextService::YAlignment)")]
#[doc(alias = "__ZN3RBX13GuiTextButton13setYAlignmentENS_11TextService10YAlignmentE")]
pub fn stub_673404(state: &mut GuiTextButtonState, value: u32) -> bool {
    // IDA 0x673404 (`RBX::GuiTextButton::setYAlignment`): compares
    // word 213 (+852); on change stores it and raises three
    // descriptors, else returns unchanged. The raises fold into the
    // changed flag.
    if state.y_alignment == value {
        return false;
    }
    state.y_alignment = value;
    true
}

// 0x673444 -- __ZNK3RBX13GuiTextButton13getTextBoundsEv
// demangled: RBX::GuiTextButton::getTextBounds(void)const
#[doc(alias = "RBX::GuiTextButton::getTextBounds(void)const")]
#[doc(alias = "__ZNK3RBX13GuiTextButton13getTextBoundsEv")]
pub fn stub_673444() -> [f32; 2] {
    // IDA 0x673444 (`RBX::GuiTextButton::getTextBounds`): the same
    // no-frontend/no-`TextService`/no-typesetter zero path as the
    // `TextBox` twin at 0x6661a8 (the measurable path reads the
    // +804/+808/+844 cells through `TextService` rasterization:
    // gap). Host: the shared floor.
    crate::generated_audio_wd_1788360980::stub_6661a8()
}

// 0x6735d0 -- __ZNK3RBX13GuiTextButton11getTextFitsEv
// demangled: RBX::GuiTextButton::getTextFits(void)const
#[doc(alias = "RBX::GuiTextButton::getTextFits(void)const")]
#[doc(alias = "__ZNK3RBX13GuiTextButton11getTextFitsEv")]
pub fn stub_6735d0() -> bool {
    // IDA 0x6735d0 (`RBX::GuiTextButton::getTextFits`): every
    // unmeasurable path yields 0, like the `TextBox` twin at
    // 0x666334. Host: the shared floor.
    crate::generated_audio_wd_1788360980::stub_666334()
}

// 0x673780 -- __ZN3RBX13GuiTextButton19setTextStrokeColor3EN3G3D6Color3E
// demangled: RBX::GuiTextButton::setTextStrokeColor3(G3D::Color3)
#[doc(alias = "RBX::GuiTextButton::setTextStrokeColor3(G3D::Color3)")]
#[doc(alias = "__ZN3RBX13GuiTextButton19setTextStrokeColor3EN3G3D6Color3E")]
pub fn stub_673780(state: &mut GuiTextButtonState, color: [f32; 3]) -> bool {
    // IDA 0x673780 (`RBX::GuiTextButton::setTextStrokeColor3`):
    // compares words 207-209 (+828); on any difference stores all
    // three and raises. The raise folds into the changed flag.
    if state.text_stroke_color3 == color {
        return false;
    }
    state.text_stroke_color3 = color;
    true
}

// 0x6737e8 -- __ZN3RBX13GuiTextButton25setTextStrokeTransparencyEf
// demangled: RBX::GuiTextButton::setTextStrokeTransparency(float)
#[doc(alias = "RBX::GuiTextButton::setTextStrokeTransparency(float)")]
#[doc(alias = "__ZN3RBX13GuiTextButton25setTextStrokeTransparencyEf")]
pub fn stub_6737e8(state: &mut GuiTextButtonState, transparency: f32) -> bool {
    // IDA 0x6737e8 (`RBX::GuiTextButton::setTextStrokeTransparency`):
    // compares word 210 (+840); on change stores it and raises,
    // else returns unchanged. The raise folds into the changed flag.
    if state.text_stroke_transparency == transparency {
        return false;
    }
    state.text_stroke_transparency = transparency;
    true
}

// 0x673814 -- __ZN3RBX13GuiTextButton14checkForResizeEv
// demangled: RBX::GuiTextButton::checkForResize(void)
#[doc(alias = "RBX::GuiTextButton::checkForResize(void)")]
#[doc(alias = "__ZN3RBX13GuiTextButton14checkForResizeEv")]
pub fn stub_673814() {
    // IDA 0x673814 (`RBX::GuiTextButton::checkForResize`): the
    // `GuiObject::checkForResize` body plus two
    // `raisePropertyChanged` calls — no `GuiTextButton`-member
    // effect. Carrier no-op.
}

// 0x673840 -- __ZN3RBX13GuiTextButton21setTransparencyLegacyEf
// demangled: RBX::GuiTextButton::setTransparencyLegacy(float)
#[doc(alias = "RBX::GuiTextButton::setTransparencyLegacy(float)")]
#[doc(alias = "__ZN3RBX13GuiTextButton21setTransparencyLegacyEf")]
pub fn stub_673840(state: &mut GuiTextButtonState, transparency: f32) -> bool {
    // IDA 0x673840 (`RBX::GuiTextButton::setTransparencyLegacy`):
    // on change of word 206 (+824) stores it and raises; the
    // `GuiObject::setBackgroundTransparency` tail owns the
    // GuiObject layer and folds away. Host: the member half as a
    // changed flag.
    if state.text_transparency == transparency {
        return false;
    }
    state.text_transparency = transparency;
    true
}

// 0x673888 -- __ZNK3RBX13GuiTextButton21getPersistentDataCostEv
// demangled: RBX::GuiTextButton::getPersistentDataCost(void)const
#[doc(alias = "RBX::GuiTextButton::getPersistentDataCost(void)const")]
#[doc(alias = "__ZNK3RBX13GuiTextButton21getPersistentDataCostEv")]
pub fn stub_673888(base: i32, text: &str) -> i32 {
    // IDA 0x673888 (`RBX::GuiTextButton::getPersistentDataCost`):
    // the `Instance` base cost plus 1 — or the +804 text
    // byte-length / 100 when that exceeds 1 — plus 6. Same shape as
    // the `TextBox` twin at 0x6668b0.
    let chunks = (text.len() / 100) as i32;
    base + if chunks > 1 { chunks } else { 1 } + 6
}

// 0x67390c -- __ZN3RBX13GuiTextButton8render2dEPNS_5AdornE
// demangled: RBX::GuiTextButton::render2d(RBX::Adorn *)
#[doc(alias = "RBX::GuiTextButton::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX13GuiTextButton8render2dEPNS_5AdornE")]
pub fn stub_67390c() {
    // IDA 0x67390c (`RBX::GuiTextButton::render2d`): forwards to the
    // virtual at +196 — `Adorn` rasterization with no modeled-cell
    // effect. Carrier no-op.
}

// 0x673918 -- __ZThn96_N3RBX13GuiTextButton8render2dEPNS_5AdornE
// demangled: non-virtual thunk toRBX::GuiTextButton::render2d(RBX::Adorn *)
#[doc(alias = "non-virtual thunk toRBX::GuiTextButton::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZThn96_N3RBX13GuiTextButton8render2dEPNS_5AdornE")]
pub fn stub_673918() {
    // IDA 0x673918: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x673924 -- __ZN3RBX13GuiTextButton15render2dContextEPNS_5AdornEPKNS_8InstanceE
// demangled: RBX::GuiTextButton::render2dContext(RBX::Adorn *,RBX::Instance const*)
#[doc(alias = "RBX::GuiTextButton::render2dContext(RBX::Adorn *,RBX::Instance const*)")]
#[doc(alias = "__ZN3RBX13GuiTextButton15render2dContextEPNS_5AdornEPKNS_8InstanceE")]
pub fn stub_673924() -> ! {
    todo!("0x673924 __ZN3RBX13GuiTextButton15render2dContextEPNS_5AdornEPKNS_8InstanceE")
}

// 0x673b74 -- __ZThn96_N3RBX13GuiTextButton15render2dContextEPNS_5AdornEPKNS_8InstanceE
// demangled: non-virtual thunk toRBX::GuiTextButton::render2dContext(RBX::Adorn *,RBX::Instance const*)
#[doc(alias = "non-virtual thunk toRBX::GuiTextButton::render2dContext(RBX::Adorn *,RBX::Instance const*)")]
#[doc(alias = "__ZThn96_N3RBX13GuiTextButton15render2dContextEPNS_5AdornEPKNS_8InstanceE")]
pub fn stub_673b74() {
    // IDA 0x673b74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x673b7c -- __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonESsED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,std::string>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,std::string>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonESsED1Ev")]
pub fn stub_673b7c() {
    // IDA 0x673b7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x673ba0 -- __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService8FontSizeEED1Ev
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::FontSize>::~EnumPropDescriptor()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::FontSize>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService8FontSizeEED1Ev")]
pub fn stub_673ba0() {
    // IDA 0x673ba0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x673bc4 -- __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEED1Ev
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::~EnumPropDescriptor()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEED1Ev")]
pub fn stub_673bc4() {
    // IDA 0x673bc4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x673be8 -- __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_10BrickColorEED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::BrickColor>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::BrickColor>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_10BrickColorEED1Ev")]
pub fn stub_673be8() {
    // IDA 0x673be8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x673c0c -- __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D6Color3EED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Color3>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Color3>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D6Color3EED1Ev")]
pub fn stub_673c0c() {
    // IDA 0x673c0c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x673c30 -- __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEfED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,float>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,float>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEfED1Ev")]
pub fn stub_673c30() {
    // IDA 0x673c30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x673c54 -- __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbED1Ev")]
pub fn stub_673c54() {
    // IDA 0x673c54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x673c78 -- __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEED1Ev
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::~EnumPropDescriptor()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEED1Ev")]
pub fn stub_673c78() {
    // IDA 0x673c78: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x673c9c -- __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEED1Ev
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::~EnumPropDescriptor()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEED1Ev")]
pub fn stub_673c9c() {
    // IDA 0x673c9c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x673cc0 -- __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D7Vector2EED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Vector2>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Vector2>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D7Vector2EED1Ev")]
pub fn stub_673cc0() {
    // IDA 0x673cc0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x673ce4 -- __ZN3RBX13GuiTextButtonD1Ev
// demangled: RBX::GuiTextButton::~GuiTextButton()
#[doc(alias = "RBX::GuiTextButton::~GuiTextButton()")]
#[doc(alias = "__ZN3RBX13GuiTextButtonD1Ev")]
pub fn stub_673ce4() {
    // IDA 0x673ce4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x673cfc -- __ZN3RBX13GuiTextButtonD0Ev
// demangled: RBX::GuiTextButton::~GuiTextButton()
#[doc(alias = "RBX::GuiTextButton::~GuiTextButton()")]
#[doc(alias = "__ZN3RBX13GuiTextButtonD0Ev")]
pub fn stub_673cfc() {
    // IDA 0x673cfc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x673db8 -- __ZThn32_N3RBX13GuiTextButtonD1Ev
// demangled: non-virtual thunk toRBX::GuiTextButton::~GuiTextButton()
#[doc(alias = "non-virtual thunk toRBX::GuiTextButton::~GuiTextButton()")]
#[doc(alias = "__ZThn32_N3RBX13GuiTextButtonD1Ev")]
pub fn stub_673db8() {
    // IDA 0x673db8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x673dd4 -- __ZThn32_N3RBX13GuiTextButtonD0Ev
// demangled: non-virtual thunk toRBX::GuiTextButton::~GuiTextButton()
#[doc(alias = "non-virtual thunk toRBX::GuiTextButton::~GuiTextButton()")]
#[doc(alias = "__ZThn32_N3RBX13GuiTextButtonD0Ev")]
pub fn stub_673dd4() {
    // IDA 0x673dd4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x673e90 -- __ZThn36_N3RBX13GuiTextButtonD1Ev
// demangled: non-virtual thunk toRBX::GuiTextButton::~GuiTextButton()
#[doc(alias = "non-virtual thunk toRBX::GuiTextButton::~GuiTextButton()")]
#[doc(alias = "__ZThn36_N3RBX13GuiTextButtonD1Ev")]
pub fn stub_673e90() {
    // IDA 0x673e90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x673eac -- __ZThn36_N3RBX13GuiTextButtonD0Ev
// demangled: non-virtual thunk toRBX::GuiTextButton::~GuiTextButton()
#[doc(alias = "non-virtual thunk toRBX::GuiTextButton::~GuiTextButton()")]
#[doc(alias = "__ZThn36_N3RBX13GuiTextButtonD0Ev")]
pub fn stub_673eac() {
    // IDA 0x673eac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6741c4 -- __ZN3RBX9CreatableINS_8InstanceEE6createINS_13GuiTextButtonEEEN5boost10shared_ptrIT_EEv
// demangled: boost::shared_ptr<RBX::GuiTextButton> RBX::Creatable<RBX::Instance>::create<RBX::GuiTextButton>(void)
#[doc(alias = "boost::shared_ptr<RBX::GuiTextButton> RBX::Creatable<RBX::Instance>::create<RBX::GuiTextButton>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_13GuiTextButtonEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_6741c4() -> ! {
    todo!("0x6741c4 __ZN3RBX9CreatableINS_8InstanceEE6createINS_13GuiTextButtonEEEN5boost10shared_ptrIT_EEv")
}

// 0x674278 -- __ZN5boost10shared_ptrIN3RBX13GuiTextButtonEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// demangled: boost::shared_ptr<RBX::GuiTextButton>::shared_ptr<RBX::GuiTextButton,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiTextButton *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "boost::shared_ptr<RBX::GuiTextButton>::shared_ptr<RBX::GuiTextButton,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiTextButton *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX13GuiTextButtonEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_674278() -> ! {
    todo!("0x674278 __ZN5boost10shared_ptrIN3RBX13GuiTextButtonEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

// 0x674340 -- __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13GuiTextButtonES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiTextButton,RBX::GuiTextButton>(boost::shared_ptr<RBX::GuiTextButton> const*,RBX::GuiTextButton *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiTextButton,RBX::GuiTextButton>(boost::shared_ptr<RBX::GuiTextButton> const*,RBX::GuiTextButton *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13GuiTextButtonES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_674340() {
    // IDA 0x674340: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x674428 -- __ZN5boost6detail12shared_countC2IPN3RBX13GuiTextButtonENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// demangled: boost::detail::shared_count::shared_count<RBX::GuiTextButton *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiTextButton *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GuiTextButton *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiTextButton *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX13GuiTextButtonENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_674428() {
    // IDA 0x674428: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x674530 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13GuiTextButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::GuiTextButton *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiTextButton *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13GuiTextButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_674530() {
    // IDA 0x674530: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x674534 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13GuiTextButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::GuiTextButton *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiTextButton *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13GuiTextButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_674534() {
    // IDA 0x674534: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x674538 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13GuiTextButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::GuiTextButton *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiTextButton *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13GuiTextButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_674538() {
    // IDA 0x674538: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x674558 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13GuiTextButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_pd<RBX::GuiTextButton *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiTextButton *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13GuiTextButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_674558() {
    // IDA 0x674558: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x674570 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13GuiTextButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::GuiTextButton *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiTextButton *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13GuiTextButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_674570() {
    // IDA 0x674570: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x674910 -- __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbEC2IMS2_KFbvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::PropDescriptor<bool (RBX::GuiTextButton::*)(void)const,int>(char const*,char const*,bool (RBX::GuiTextButton::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::PropDescriptor<bool (RBX::GuiTextButton::*)(void)const,int>(char const*,char const*,bool (RBX::GuiTextButton::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbEC2IMS2_KFbvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_674910() -> ! {
    todo!("0x674910 __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbEC2IMS2_KFbvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0x674a1c -- __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbED0Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbED0Ev")]
pub fn stub_674a1c() {
    // IDA 0x674a1c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x674a48 -- __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE7GetImplIMS2_KFbvEE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetImpl<bool (RBX::GuiTextButton::*)(void)const>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetImpl<bool (RBX::GuiTextButton::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE7GetImplIMS2_KFbvEE10isReadOnlyEv")]
pub fn stub_674a48() -> ! {
    todo!("0x674a48 __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE7GetImplIMS2_KFbvEE10isReadOnlyEv")
}

// 0x674a4c -- __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE7GetImplIMS2_KFbvEE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetImpl<bool (RBX::GuiTextButton::*)(void)const>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetImpl<bool (RBX::GuiTextButton::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE7GetImplIMS2_KFbvEE11isWriteOnlyEv")]
pub fn stub_674a4c() -> ! {
    todo!("0x674a4c __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE7GetImplIMS2_KFbvEE11isWriteOnlyEv")
}
