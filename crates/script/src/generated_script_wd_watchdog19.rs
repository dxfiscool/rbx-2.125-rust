// Auto-generated skeletons for rbx-script — Script/Lua/Luau gap filler (watchdog19)
// Filter: Script|Lua|Luau (filtered 0 remaining) — EA-sorted asc gap filler distinct not yet in crates/script/src
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0xc564..0x106b8 | EA-sorted asc distinct not yet in script (remaining 64404->64304, rbx_core::SharedPtr not boost)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};
use std::collections::HashMap;

use crate::generated_24::{PlacementAnyRegion3, stub_0xc90c, stub_0xc95c};
use rbx_core::shared_ptr::{ControlBlockPd, CreatableInstanceDeleter, shared_ptr_from_raw};
use rbx_datamodel::instance::{
    CRenderSettingsItem, RENDER_SETTINGS_CLASS_DESCRIPTOR, RENDER_SETTINGS_ITEM_CREATOR,
    RenderSettingsClassDescriptor, RenderSettingsItemCreator,
};
use rbx_reflection::generated::{
    CRenderSettingsBoundFunc, CRenderSettingsIntAccess, CRenderSettingsIntProp,
    CRenderSettingsItemState,
};
use rbx_reflection::descriptor::Variant;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicBool, Ordering};

/// Host image of `RBX::Reflection::EnumDesc<T>` tables read by the
/// 0xc564..0xd7cc instantiations (App/include/reflection/enumconverter.h).
/// Each instantiation keeps its own table set; the code below is shared.
#[derive(Debug, Default)]
pub struct RenderEnumDesc {
    /// [this+0x28]/[this+40]: mapped value count (0xc5cc/0xc628 bound).
    pub value_count: usize,
    /// [this+0x90]: value -> item payload (0xc5cc source).
    pub value_to_item: Vec<i32>,
    /// [this+144]: value -> ordinal input for the string path (0xc628).
    pub value_ordinals: Vec<i32>,
    /// [this+108]: item index -> name (string source).
    pub item_names: Vec<String>,
    /// [this+120]: ordinal -> item payload (convertToItem source).
    pub enum_to_item: Vec<i32>,
    /// Primary + legacy Name-key -> value maps (convertToValue searches both).
    pub name_to_value: HashMap<u32, i32>,
    pub legacy_values: HashMap<u32, i32>,
    /// Set once the D2 destructor has run (vtable retarget marker).
    pub destroyed: bool,
}

/// MODEL of `FLog::Asserts`: the original gates every ReleaseAssert below on
/// this flag (e.g. IDA 0xc7c8 `if (!FLog::Asserts) goto LABEL_13`).
fn enum_asserts_enabled() -> bool {
    true
}

/// Shared bounds check (enumconverter.h:262-263 / :273-274).
fn enum_desc_check_value(len: usize, value: i32, neg_line: u32, size_line: u32) {
    if enum_asserts_enabled() {
        assert!(
            value >= 0,
            "value>=0 file: ../App/include/reflection/enumconverter.h line: {neg_line}"
        );
        assert!(
            (value as usize) < len,
            "(size_t)value<enumToItem.size() file: ../App/include/reflection/enumconverter.h line: {size_line}"
        );
    }
}

/// IDA convertToItem LABEL_13 core (0xca84..0xca9c): negative or overlarge
/// reads 0, otherwise the ordinal slot.
fn enum_desc_item(desc: &RenderEnumDesc, value: i32) -> i32 {
    if value >= 0 && (value as usize) < desc.enum_to_item.len() {
        desc.enum_to_item[value as usize]
    } else {
        0
    }
}

/// convertToItem + asserts shared by the 0xc9d8/0xcfb8/0xd4f8 instantiations.
fn enum_desc_item_of(desc: &RenderEnumDesc, value: i32) -> i32 {
    enum_desc_check_value(desc.enum_to_item.len(), value, 273, 274);
    enum_desc_item(desc, value)
}

/// IDA convertToString(const&) LABEL_13 core (0xc876..0xc896): negative or
/// overlarge values stringify empty, otherwise the item name.
fn enum_desc_name(desc: &RenderEnumDesc, value: i32) -> &str {
    if value >= 0 && (value as usize) < desc.item_names.len() {
        &desc.item_names[value as usize]
    } else {
        ""
    }
}

/// convertToString(const&) + asserts shared by 0xc76c/0xcd4c/0xd28c/0xd7cc.
fn enum_desc_string_to(desc: &RenderEnumDesc, value: i32, out: &mut String) {
    // NOTE: the original asserts against enumToItem but indexes item_names;
    // that quirk is preserved here.
    enum_desc_check_value(desc.enum_to_item.len(), value, 262, 263);
    out.clear();
    out.push_str(enum_desc_name(desc, value));
}

/// IDA 0xcc34 lower_bound core: exact hit in either map writes the payload.
fn enum_desc_value_by_name(desc: &RenderEnumDesc, name: u32, out: &mut i32) -> bool {
    if let Some(&v) = desc
        .name_to_value
        .get(&name)
        .or_else(|| desc.legacy_values.get(&name))
    {
        *out = v;
        true
    } else {
        false
    }
}

/// IDA 0xccdc loop: release each live item observer (host keeps none).
fn enum_desc_release_items(_desc: &mut RenderEnumDesc) {}

/// Shared D2 core (0xccb0/0xd1f0/0xd730): drop heap tables, mark destroyed.
fn enum_desc_destroy(desc: &mut RenderEnumDesc) {
    enum_desc_release_items(desc);
    desc.value_to_item.clear();
    desc.value_ordinals.clear();
    desc.item_names.clear();
    desc.enum_to_item.clear();
    desc.name_to_value.clear();
    desc.legacy_values.clear();
    desc.value_count = 0;
    desc.destroyed = true;
}

/// MODEL of `RBX::Name::lookup`: interns a C string to its key (FNV-1a/32
/// stand-in for the interned `Name*` the maps are keyed on).
fn intern_name(name: &str) -> u32 {
    let mut hash: u32 = 0x811c_9dc5;
    for byte in name.bytes() {
        hash ^= u32::from(byte);
        hash = hash.wrapping_mul(0x0100_0193);
    }
    hash
}

// 0xc564 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED1Ev — RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::~EnumDesc()
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED1Ev")]
pub fn stub_0xc564(desc: &mut RenderEnumDesc) {
    // IDA 0xc564 (thunk): tail-calls the D2 destructor below.
    stub_0xccb0(desc);
}

// 0xc568 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED0Ev — RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::~EnumDesc()
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::~EnumDesc() [0xc568]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED0Ev")]
pub fn stub_0xc568(desc: Box<RenderEnumDesc>) {
    // IDA 0xc568: D1 destructor (0xc56e) + operator delete; consuming the Box
    // frees the allocation, so only the explicit D2 step is modeled.
    let mut desc = desc;
    stub_0xccb0(&mut desc);
}

// 0xc57c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE6lookupEPKc — RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::lookup(char const*)const
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE6lookupEPKc")]
pub fn stub_0xc57c(desc: &RenderEnumDesc, name: &str) -> i32 {
    // IDA 0xc586..0xc5a8: Name::lookup, convertToValue, convertToItem on hit,
    // 0 on miss.
    let mut value = 0;
    if enum_desc_value_by_name(desc, intern_name(name), &mut value) {
        stub_0xc9d8(desc, value)
    } else {
        0
    }
}

// 0xc5ac — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE6lookupERKNS0_7VariantE — RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::lookup(RBX::Reflection::Variant const&)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE6lookupERKNS0_7VariantE")]
pub fn stub_0xc5ac(desc: &RenderEnumDesc, variant: &PlacementAnyRegion3) -> i32 {
    // IDA 0xc5be: any_cast<ResolutionPreset const&>(variant + 4); host holders
    // are unique per type, so identity subsumes the typeinfo-name fallback.
    if variant.holder != stub_0xc95c() {
        panic!("rbx::bad_placement_any_cast at IDA 0xc5ac");
    }
    stub_0xc9d8(desc, variant.storage as i32)
}

// 0xc5cc — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToValueEmRNS0_7VariantE — RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_0xc5cc(desc: &RenderEnumDesc, value: u32, out: &mut PlacementAnyRegion3) -> bool {
    // IDA 0xc5d4..0xc5e6: HI([this+0x28] > value) gates the [this+0x90] load.
    // BUG: original at 0xc5e8..0xc61c still stores the holder and runs
    // operator= over the uninitialized stack temp when the bound fails; the
    // host assigns 0 there instead and reports failure.
    let hit = (value as usize) < desc.value_count
        && (value as usize) < desc.value_to_item.len();
    let temp = if hit { desc.value_to_item[value as usize] as u32 } else { 0 };
    // 0xc5fc..0xc61c: singleton holder store + operator=<ResolutionPreset>.
    out.holder = stub_0xc95c();
    stub_0xc90c(out as *mut PlacementAnyRegion3, &temp as *const u32);
    hit
}

// 0xc628 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE15convertToStringEmRSs — RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToString(unsigned long,std::string &)const
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE15convertToStringEmRSs")]
pub fn stub_0xc628(desc: &RenderEnumDesc, value: u32, out: &mut String) -> bool {
    // IDA 0xc67c: ([this+40] <= value) returns 0; 0xc68c: ordinal input is
    // [this+144][value]; 0xc696: const&-overload fills a temp; 0xc6a2: assign.
    if (value as usize) >= desc.value_count {
        return false;
    }
    let ordinal = desc
        .value_ordinals
        .get(value as usize)
        .copied()
        .unwrap_or(value as i32);
    let mut tmp = String::new();
    stub_0xc76c(desc, ordinal, &mut tmp);
    *out = tmp;
    true
}

// 0xc76c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE15convertToStringERKS3_ — RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToString(RBX::CRenderSettings::ResolutionPreset const&)const
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToString(RBX::CRenderSettings::ResolutionPreset const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE15convertToStringERKS3_")]
pub fn stub_0xc76c(desc: &RenderEnumDesc, value: i32, out: &mut String) {
    // IDA 0xc76c: ResolutionPreset instantiation of the const& overload.
    enum_desc_string_to(desc, value, out);
}

// 0xc9d8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE13convertToItemERKS3_ — RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToItem(RBX::CRenderSettings::ResolutionPreset const&)const
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToItem(RBX::CRenderSettings::ResolutionPreset const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE13convertToItemERKS3_")]
pub fn stub_0xc9d8(desc: &RenderEnumDesc, value: i32) -> i32 {
    // IDA 0xc9d8: ResolutionPreset instantiation; LABEL_13 core in helper.
    enum_desc_item_of(desc, value)
}

// 0xcc34 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToValueERKNS_4NameERS3_ — RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToValue(RBX::Name const&,RBX::CRenderSettings::ResolutionPreset&)const
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToValue(RBX::Name const&,RBX::CRenderSettings::ResolutionPreset&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xcc34(desc: &RenderEnumDesc, name: u32, out: &mut i32) -> bool {
    // IDA 0xcc4a..0xccac: lower_bound over the primary map then the legacy
    // map; exact hit writes the payload (*out) and returns 1.
    enum_desc_value_by_name(desc, name, out)
}

// 0xccb0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED2Ev — RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::~EnumDesc()
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::~EnumDesc() [0xccb0]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED2Ev")]
pub fn stub_0xccb0(desc: &mut RenderEnumDesc) {
    // IDA 0xccce: vtable retarget; 0xccd4: registrar bump (host: none);
    // 0xccdc..0xcd48: observer releases, heap deletes, names drop, both map
    // erases, base EnumDescriptor dtor.
    enum_desc_destroy(desc);
}

// 0xcd4c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE15convertToStringERKS3_ — RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToString(RBX::CRenderSettings::QualityLevel const&)const
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToString(RBX::CRenderSettings::QualityLevel const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE15convertToStringERKS3_")]
pub fn stub_0xcd4c(desc: &RenderEnumDesc, value: i32, out: &mut String) {
    // IDA 0xcd4c: QualityLevel instantiation; identical shape to 0xc76c.
    enum_desc_string_to(desc, value, out);
}

// 0xcfb8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE13convertToItemERKS3_ — RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToItem(RBX::CRenderSettings::QualityLevel const&)const
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToItem(RBX::CRenderSettings::QualityLevel const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE13convertToItemERKS3_")]
pub fn stub_0xcfb8(desc: &RenderEnumDesc, value: i32) -> i32 {
    // IDA 0xcfb8: QualityLevel instantiation; identical shape to 0xc9d8.
    enum_desc_item_of(desc, value)
}

// 0xd174 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToValueERKNS_4NameERS3_ — RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToValue(RBX::Name const&,RBX::CRenderSettings::QualityLevel&)const
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToValue(RBX::Name const&,RBX::CRenderSettings::QualityLevel&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xd174(desc: &RenderEnumDesc, name: u32, out: &mut i32) -> bool {
    // IDA 0xd174: QualityLevel instantiation; identical shape to 0xcc34.
    enum_desc_value_by_name(desc, name, out)
}

// 0xd1f0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED2Ev — RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::~EnumDesc()
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::~EnumDesc() [0xd1f0]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED2Ev")]
pub fn stub_0xd1f0(desc: &mut RenderEnumDesc) {
    // IDA 0xd1f0: QualityLevel D2; identical shape to 0xccb0.
    enum_desc_destroy(desc);
}

// 0xd28c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE15convertToStringERKS3_ — RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToString(RBX::CRenderSettings::ShadowMode const&)const
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToString(RBX::CRenderSettings::ShadowMode const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE15convertToStringERKS3_")]
pub fn stub_0xd28c(desc: &RenderEnumDesc, value: i32, out: &mut String) {
    // IDA 0xd28c: ShadowMode instantiation; identical shape to 0xc76c.
    enum_desc_string_to(desc, value, out);
}

// 0xd4f8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE13convertToItemERKS3_ — RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToItem(RBX::CRenderSettings::ShadowMode const&)const
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToItem(RBX::CRenderSettings::ShadowMode const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE13convertToItemERKS3_")]
pub fn stub_0xd4f8(desc: &RenderEnumDesc, value: i32) -> i32 {
    // IDA 0xd4f8: ShadowMode instantiation; identical shape to 0xc9d8.
    enum_desc_item_of(desc, value)
}

// 0xd6b4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToValueERKNS_4NameERS3_ — RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::ShadowMode&)const
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::ShadowMode&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xd6b4(desc: &RenderEnumDesc, name: u32, out: &mut i32) -> bool {
    // IDA 0xd6b4: ShadowMode instantiation; identical shape to 0xcc34.
    enum_desc_value_by_name(desc, name, out)
}

// 0xd730 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED2Ev — RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::~EnumDesc()
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::~EnumDesc() [0xd730]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED2Ev")]
pub fn stub_0xd730(desc: &mut RenderEnumDesc) {
    // IDA 0xd730: ShadowMode D2; identical shape to 0xccb0.
    enum_desc_destroy(desc);
}

// 0xd7cc — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE15convertToStringERKS3_ — RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToString(RBX::CRenderSettings::AntialiasingMode const&)const
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToString(RBX::CRenderSettings::AntialiasingMode const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE15convertToStringERKS3_")]
pub fn stub_0xd7cc(desc: &RenderEnumDesc, value: i32, out: &mut String) {
    // IDA 0xd7cc: AntialiasingMode instantiation; identical shape to 0xc76c.
    enum_desc_string_to(desc, value, out);
}

// 0xda38 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE13convertToItemERKS3_ — RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToItem(RBX::CRenderSettings::AntialiasingMode const&)const
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToItem(RBX::CRenderSettings::AntialiasingMode const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE13convertToItemERKS3_")]
pub fn stub_0xda38(desc: &RenderEnumDesc, value: i32) -> i32 {
    // IDA 0xda38: AntialiasingMode instantiation of convertToItem; LABEL_13 core in helper.
    enum_desc_item_of(desc, value)
}

// 0xdbf4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToValueERKNS_4NameERS3_ — RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::AntialiasingMode&)const
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::AntialiasingMode&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xdbf4(desc: &RenderEnumDesc, name: u32, out: &mut i32) -> bool {
    // IDA 0xdbf4: AntialiasingMode instantiation; identical shape to 0xcc34: lower_bound over
    // the primary map then the legacy map; exact hit writes the payload.
    enum_desc_value_by_name(desc, name, out)
}

// 0xdc70 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED2Ev — RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc()
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc() [0xdc70]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED2Ev")]
pub fn stub_0xdc70(desc: &mut RenderEnumDesc) {
    // IDA 0xdc70: AntialiasingMode D2; identical shape to 0xccb0.
    enum_desc_destroy(desc);
}

// 0xdd0c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE15convertToStringERKS3_ — RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToString(RBX::CRenderSettings::FrameRateManagerMode const&)const
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToString(RBX::CRenderSettings::FrameRateManagerMode const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE15convertToStringERKS3_")]
pub fn stub_0xdd0c(desc: &RenderEnumDesc, value: i32, out: &mut String) {
    // IDA 0xdd0c: FrameRateManagerMode instantiation; identical shape to 0xc76c.
    enum_desc_string_to(desc, value, out);
}

// 0xdf78 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE13convertToItemERKS3_ — RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToItem(RBX::CRenderSettings::FrameRateManagerMode const&)const
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToItem(RBX::CRenderSettings::FrameRateManagerMode const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE13convertToItemERKS3_")]
pub fn stub_0xdf78(desc: &RenderEnumDesc, value: i32) -> i32 {
    // IDA 0xdf78: FrameRateManagerMode instantiation of convertToItem; LABEL_13 core in helper.
    enum_desc_item_of(desc, value)
}

// 0xe134 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueERKNS_4NameERS3_ — RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::FrameRateManagerMode&)const
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::FrameRateManagerMode&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xe134(desc: &RenderEnumDesc, name: u32, out: &mut i32) -> bool {
    // IDA 0xe134: FrameRateManagerMode instantiation; identical shape to 0xcc34: lower_bound over
    // the primary map then the legacy map; exact hit writes the payload.
    enum_desc_value_by_name(desc, name, out)
}

// 0xe1b0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED2Ev — RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::~EnumDesc()
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::~EnumDesc() [0xe1b0]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED2Ev")]
pub fn stub_0xe1b0(desc: &mut RenderEnumDesc) {
    // IDA 0xe1b0: FrameRateManagerMode D2; identical shape to 0xccb0.
    enum_desc_destroy(desc);
}

// 0xe24c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringERKS3_ — RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToString(RBX::CRenderSettings::GraphicsMode const&)const
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToString(RBX::CRenderSettings::GraphicsMode const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringERKS3_")]
pub fn stub_0xe24c(desc: &RenderEnumDesc, value: i32, out: &mut String) {
    // IDA 0xe24c: GraphicsMode instantiation; identical shape to 0xc76c.
    enum_desc_string_to(desc, value, out);
}

// 0xe4b8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE13convertToItemERKS3_ — RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToItem(RBX::CRenderSettings::GraphicsMode const&)const
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToItem(RBX::CRenderSettings::GraphicsMode const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE13convertToItemERKS3_")]
pub fn stub_0xe4b8(desc: &RenderEnumDesc, value: i32) -> i32 {
    // IDA 0xe4b8: GraphicsMode instantiation of convertToItem; LABEL_13 core in helper.
    enum_desc_item_of(desc, value)
}

// 0xe674 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueERKNS_4NameERS3_ — RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::GraphicsMode&)const
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::GraphicsMode&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xe674(desc: &RenderEnumDesc, name: u32, out: &mut i32) -> bool {
    // IDA 0xe674: GraphicsMode instantiation; identical shape to 0xcc34: lower_bound over
    // the primary map then the legacy map; exact hit writes the payload.
    enum_desc_value_by_name(desc, name, out)
}

// 0xe6f0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED2Ev — RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc() [0xe6f0]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED2Ev")]
pub fn stub_0xe6f0(desc: &mut RenderEnumDesc) {
    // IDA 0xe6f0: GraphicsMode D2; identical shape to 0xccb0.
    enum_desc_destroy(desc);
}

// 0xe78c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE15convertToStringERKS3_ — RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToString(RBX::CRenderSettings::AASamples const&)const
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToString(RBX::CRenderSettings::AASamples const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE15convertToStringERKS3_")]
pub fn stub_0xe78c(desc: &RenderEnumDesc, value: i32, out: &mut String) {
    // IDA 0xe78c: AASamples instantiation; identical shape to 0xc76c.
    enum_desc_string_to(desc, value, out);
}

// 0xe9f8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE13convertToItemERKS3_ — RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToItem(RBX::CRenderSettings::AASamples const&)const
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToItem(RBX::CRenderSettings::AASamples const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE13convertToItemERKS3_")]
pub fn stub_0xe9f8(desc: &RenderEnumDesc, value: i32) -> i32 {
    // IDA 0xe9f8: AASamples instantiation of convertToItem; LABEL_13 core in helper.
    enum_desc_item_of(desc, value)
}

/// `sRenderSettings` interned-name key behind `Name::doDeclare<sRenderSettings>`
/// (IDA 0xf1dc): the process-static `n` filled once under `__cxa_guard_acquire`
/// (0xf238..0xf262); collapses into static init.
static S_RENDER_SETTINGS_NAME: LazyLock<u32> = LazyLock::new(|| intern_name("RenderSettings"));

/// Host image of `G3D::Vector2int16` (IDA 0xf704): two i16 lanes, one u32 word.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Vector2int16 {
    pub x: i16,
    pub y: i16,
}

/// Presence latch for `rbx::signals::slot_exception_handler` read by `on_error`
/// (IDA 0xf6dc).
static SLOT_EXCEPTION_HANDLER_INSTALLED: AtomicBool = AtomicBool::new(false);

// 0xebb4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueERKNS_4NameERS3_ — RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToValue(RBX::Name const&,RBX::CRenderSettings::AASamples&)const
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToValue(RBX::Name const&,RBX::CRenderSettings::AASamples&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xebb4(desc: &RenderEnumDesc, name: u32, out: &mut i32) -> bool {
    // IDA 0xebb4: AASamples instantiation; identical shape to 0xcc34.
    enum_desc_value_by_name(desc, name, out)
}

// 0xec30 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED2Ev — RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc() [0xec30]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED2Ev")]
pub fn stub_0xec30(desc: &mut RenderEnumDesc) {
    // IDA 0xec30: AASamples D2; identical shape to 0xccb0.
    enum_desc_destroy(desc);
}

// 0xeccc — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD2Ev
// type: int __fastcall(int)
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0xeccc() {
    // IDA 0xeccc: Creator D2 — vtable reset (0xed1c), `wasConstructed` assert
    // (0xed26..0xed96), `getCreators` + erase by class name (0xed9a..0xedba).
    // No members to drop; Rust Drop glue covers it.
}

// 0xedfc — __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0xedfc() -> &'static str {
    // IDA 0xedfc: `wasConstructed` assert (0xedfe..0xee5c) then tail-calls
    // `Name::doDeclare<sRenderSettings>()` — the "RenderSettings" literal.
    stub_0xf1d8();
    "RenderSettings"
}

// 0xee84 — __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator6createEv
// type: int __fastcall(int *)
#[doc(alias = "__ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator6createEv")]
pub fn stub_0xee84() -> SharedPtr<CRenderSettingsItem> {
    // IDA 0xee84: `wasConstructed` assert (0xee98..0xeee8), then
    // `Creatable::create<CRenderSettingsItem>()` (0xeeec -> stub_0xef04); the
    // +0x20 empty-base adjust (0xeef8) collapses under single inheritance.
    stub_0xef04()
}

// 0xef04 — __ZN3RBX9CreatableINS_8InstanceEE6createI19CRenderSettingsItemEEN5boost10shared_ptrIT_EEv — boost::shared_ptr<CRenderSettingsItem> RBX::Creatable<RBX::Instance>::create<CRenderSettingsItem>(void)
// type: void __fastcall(int)
#[doc(alias = "boost::shared_ptr<CRenderSettingsItem> RBX::Creatable<RBX::Instance>::create<CRenderSettingsItem>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createI19CRenderSettingsItemEEN5boost10shared_ptrIT_EEv")]
pub fn stub_0xef04() -> SharedPtr<CRenderSettingsItem> {
    // IDA 0xef04: `operator new(0xc4)` (0xef38) + `CRenderSettingsItem` default
    // ctor (0xef5c) + `shared_ptr` adoption with the Creatable deleter (0xef6a).
    // `SharedPtr::new` with `Default` is the same default-construct +
    // single-owner adoption.
    SharedPtr::new(CRenderSettingsItem::default())
}

// 0xefb4 — __ZN5boost10shared_ptrI19CRenderSettingsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_ — boost::shared_ptr<CRenderSettingsItem>::shared_ptr<CRenderSettingsItem,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "boost::shared_ptr<CRenderSettingsItem>::shared_ptr<CRenderSettingsItem,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrI19CRenderSettingsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0xefb4(ptr: *mut CRenderSettingsItem, _deleter: CreatableInstanceDeleter) -> SharedPtr<CRenderSettingsItem> {
    // IDA 0xefb4: store px (0xefba), `shared_count` ctor (0xefc0 -> stub_0xf098);
    // non-null px runs `enable_shared_from_this::accept_owner` at +40 (0xefd0).
    // Arc adoption is the same single-owner take; a null px — which only skips
    // the weak-owner link — collapses to a default-owned handle. The unit
    // deleter tag carries no state.
    // SAFETY: `ptr` must be null or a live caller-owned pointer.
    if ptr.is_null() {
        return SharedPtr::new(CRenderSettingsItem::default());
    }
    shared_ptr_from_raw(unsafe { Box::from_raw(ptr) })
}

// 0xefd8 — __ZNK5boost6detail15sp_counted_base9use_countEv — boost::detail::sp_counted_base::use_count(void)const
// type: int __fastcall(boost::detail::sp_counted_base *this)
#[doc(alias = "boost::detail::sp_counted_base::use_count(void)const")]
#[doc(alias = "__ZNK5boost6detail15sp_counted_base9use_countEv")]
pub fn stub_0xefd8(block: &ControlBlockPd<CRenderSettingsItem, CreatableInstanceDeleter>) -> usize {
    // IDA 0xefd8 `sp_counted_base::use_count`: spinlock-pool lock (0xf020),
    // load of use_count (0xf032), unlock (0xf058). The host Arc locks
    // internally; the observable effect is the load.
    block.use_count()
}

// 0xf098 — __ZN5boost6detail12shared_countC2IP19CRenderSettingsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_ — boost::detail::shared_count::shared_count<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IP19CRenderSettingsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0xf098(ptr: *mut CRenderSettingsItem, _deleter: CreatableInstanceDeleter) -> ControlBlockPd<CRenderSettingsItem, CreatableInstanceDeleter> {
    // IDA 0xf098 `shared_count` ctor: fresh `sp_counted_impl_pd` block
    // (0xf0ec: new 0x14, use = weak = 1, vtable, px at 0xf0fa..0xf10a). A null
    // px never reaches here with a live block in the original either; the host
    // adopts a default payload instead of forming an empty `Box::from_raw`.
    if ptr.is_null() {
        return ControlBlockPd::new(Box::new(CRenderSettingsItem::default()), CreatableInstanceDeleter);
    }
    // SAFETY: `ptr` is a live caller-owned pointer (checked non-null above).
    ControlBlockPd::new(unsafe { Box::from_raw(ptr) }, CreatableInstanceDeleter)
}

// 0xf198 — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev — boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0xf198]")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0xf198() {
    // IDA 0xf198: `BX LR` — empty `sp_counted_impl_pd` dtor body.
}

// 0xf19c — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv — boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0xf19c(block: *mut ControlBlockPd<CRenderSettingsItem, CreatableInstanceDeleter>) {
    // IDA 0xf19c `dispose`: `predelete(px)` (0xf1a4), null-px early-out
    // (0xf1aa..0xf1ac), then the deleter virtual-delete (0xf1b8).
    // `dispose_with` with the no-op predelete (the base `Instance::predelete`
    // hook is unmodeled for opaque classes) takes the payload — the delete.
    // SAFETY: `block` must point to a valid block.
    unsafe {
        (*block).dispose_with(|_| {});
    }
}

// 0xf1bc — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info — boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0xf1bc(block: *const ControlBlockPd<CRenderSettingsItem, CreatableInstanceDeleter>, type_name: &str) -> Option<CreatableInstanceDeleter> {
    // IDA 0xf1bc `get_deleter`: typeinfo-name compare vs
    // "N3RBX9CreatableINS_8InstanceEE7DeleterE" (0xf1c6..0xf1ce); a hit returns
    // the deleter at +16 (0xf1c0), a miss returns 0.
    // SAFETY: `block` must point to a valid block.
    unsafe { (*block).get_deleter(type_name) }
}

// 0xf1d4 — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv — boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0xf1d4(block: *const ControlBlockPd<CRenderSettingsItem, CreatableInstanceDeleter>) -> CreatableInstanceDeleter {
    // IDA 0xf1d4 `get_untyped_deleter`: unconditional deleter at +16.
    // SAFETY: `block` must point to a valid block.
    unsafe { (*block).get_untyped_deleter() }
}

// 0xf1d8 — __ZN3RBX4Name13callDoDeclareILZ15sRenderSettingsEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZ15sRenderSettingsEEEvv")]
pub fn stub_0xf1d8() -> u32 {
    // IDA 0xf1d8: `Name::callDoDeclare<sRenderSettings>` thunk tail-calling
    // `doDeclare` (0xf1dc).
    stub_0xf1dc()
}

// 0xf1dc — __ZN3RBX4Name9doDeclareILZ15sRenderSettingsEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZ15sRenderSettingsEEERKS0_v")]
pub fn stub_0xf1dc() -> u32 {
    // IDA 0xf1dc `Name::doDeclare<sRenderSettings>`: `__cxa_guard_acquire`
    // once-init around `Name::declare(&sRenderSettings, 1)` (0xf238..0xf290);
    // collapses into static init.
    *S_RENDER_SETTINGS_NAME
}

// 0xf2bc — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0xf2bc() {
    // IDA 0xf2bc: Creator C2 — vtable install (0xf2f2), one-shot
    // `Name::declare<sRenderSettings>` via `call_once` (0xf2f4), then the
    // `getCreators` binary-search insert with the double-asserted
    // find/add/verify sequence (0xf316..0xf4fe). The process-static creator is
    // the constructed singleton; Rust static init covers it.
}

// 0xf500 — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0xf500() -> &'static RenderSettingsItemCreator {
    // IDA 0xf500 `static_getCreator`: `wasConstructed` assert
    // (0xf510..0xf55c), then returns `creatorPrivate` (0xf572).
    &RENDER_SETTINGS_ITEM_CREATOR
}

// 0xf574 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4nextERN5boost13intrusive_ptrINS8_4slotEEE — rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot> &)
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot> &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4nextERN5boost13intrusive_ptrINS8_4slotEEE")]
pub fn stub_0xf574(out_slot: &mut u32, slot: u32) -> bool {
    // IDA 0xf574 `signal<...>::next`: `add_ref(slot)` (0xf5a2..0xf5ce),
    // `call_once` static-mutex init + fetch (0xf5ee..0xf5fa), lock (0xf608),
    // slot `operator=` into the cursor (0xf61c), unlock (0xf636..0xf640),
    // `release(slot)` (0xf646..0xf64e); returns cursor-slot != 0
    // (0xf658..0xf674). The host lock only guards the copy.
    *out_slot = slot;
    *out_slot != 0
}

// 0xf6dc — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE8on_errorERSt9exception — rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::on_error(std::exception &)
// type: int *()
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::on_error(std::exception &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE8on_errorERSt9exception")]
pub fn stub_0xf6dc() -> bool {
    // IDA 0xf6dc `signal<...>::on_error`: fetch `slot_exception_handler`
    // (0xf6f0..0xf6f2); a set handler routes through
    // `function1::dummy::nonnull` and its call runs (0xf6f6..0xf6fe),
    // otherwise the slot address returns (0xf702). Host reports whether a
    // handler is installed.
    SLOT_EXCEPTION_HANDLER_INSTALLED.load(Ordering::Relaxed)
}

// 0xf704 — __ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_ — std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_insert_aux(__gnu_cxx::__normal_iterator<G3D::Vector2int16*,std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>>,G3D::Vector2int16 const&)
// type: int __fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_insert_aux(__gnu_cxx::__normal_iterator<G3D::Vector2int16*,std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>>,G3D::Vector2int16 const&)")]
#[doc(alias = "__ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
pub fn stub_0xf704(vec: &mut Vec<Vector2int16>, pos: usize, value: Vector2int16) {
    // IDA 0xf704 `vector<G3D::Vector2int16>::_M_insert_aux`: the capacity-full
    // path (0xf73e..0xf7c4) grows (1 or size/2, `__throw_length_error` at
    // 0x3FFFFFFF) and relocates around the hole; the fast path
    // (0xf71a..0xf738) shifts right via `__copy_backward` (0xf734 ->
    // stub_0xf800) and stores. `Vec::insert` is the same insert-at semantics.
    let at = pos.min(vec.len());
    vec.insert(at, value);
}

// 0xf7e8 — __ZNSt12_Vector_baseIN3G3D12Vector2int16ESaIS1_EE11_M_allocateEm — std::_Vector_base<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_allocate(unsigned long)
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3G3D12Vector2int16ESaIS1_EE11_M_allocateEm")]
pub fn stub_0xf7e8(vec: &mut Vec<Vector2int16>, count: usize) {
    // IDA 0xf7e8 `_M_allocate`: `__throw_bad_alloc` at >= 0x40000000
    // (0xf7f0..0xf7f2), else `operator new(4 * n)`. Host reserves the capacity.
    assert!(count < 0x4000_0000, "std::bad_alloc at IDA 0xf7e8");
    vec.reserve(count);
}

// 0xf800 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D12Vector2int16ES5_EET0_T_S7_S6_ — G3D::Vector2int16 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector2int16 *,G3D::Vector2int16 *>(G3D::Vector2int16 *,G3D::Vector2int16 *,G3D::Vector2int16 *)
// type: int __fastcall(int, int, int)
#[doc(alias = "G3D::Vector2int16 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector2int16 *,G3D::Vector2int16 *>(G3D::Vector2int16 *,G3D::Vector2int16 *,G3D::Vector2int16 *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D12Vector2int16ES5_EET0_T_S7_S6_")]
pub fn stub_0xf800(buf: &mut [Vector2int16], first: usize, last: usize, result_end: usize) -> usize {
    // IDA 0xf800 `__copy_backward`: moves [first, last) to end at result_end
    // (0xf800..0xf834). `copy_within` is the same memmove; returns result_end
    // (0xf83a).
    buf.copy_within(first..last, result_end - (last - first));
    result_end
}

// 0xf83c — __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev")]
pub fn stub_0xf83c() {
    // IDA 0xf83c: `GlobalAdvancedSettingsItem` D1 — vtable installs
    // (0xf860..0xf86e), `sing = 0` (0xf872), `Instance` dtor (0xf878). Drop glue.
}

// 0xf87c — __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev")]
pub fn stub_0xf87c() {
    // IDA 0xf87c: D0 twin of 0xf83c plus `operator delete`; Arc Drop glue
    // covers both.
}

// 0xf8c8 — __ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev
// type: void __fastcall(_QWORD *)
#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev")]
pub fn stub_0xf8c8() {
    // IDA 0xf8c8: ZThn32 D1 — `this -= 32` adjust (0xf8e8..0xf8fa), `sing = 0`
    // (0xf900), `Instance` dtor in place (0xf906); the base offset collapses
    // under single inheritance.
}

// 0xf90c — __ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev
// type: int __fastcall(_QWORD *)
#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev")]
pub fn stub_0xf90c() {
    // IDA 0xf90c: ZThn32 D0 — `this -= 32`, `sing = 0`, `Instance` dtor,
    // `operator delete`; Drop glue covers it.
}

// 0xf964 — __ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev")]
pub fn stub_0xf964() {
    // IDA 0xf964: ZThn36 D1 — `this -= 36` adjust (0xf984..0xf996), `sing = 0`
    // (0xf99c), `Instance` dtor in place (0xf9a2); same collapse as 0xf8c8.
}

// 0xf9a8 — __ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev")]
pub fn stub_0xf9a8() {
    // IDA 0xf9a8: ZThn36 D0 — `this -= 36`, `sing = 0`, `Instance` dtor,
    // `operator delete`; Drop glue covers it.
}

// 0xfa00 — __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0xfa00() -> &'static RenderSettingsClassDescriptor {
    // IDA 0xfa00 `Described::classDescriptor`: guard-var once-init
    // (`__cxa_guard_acquire`, 0xfa5c) building `describedClassDescriptor` off
    // the Instance base ("RenderSettings", 0xfa68..0xfac4); collapses into
    // static init.
    &RENDER_SETTINGS_CLASS_DESCRIPTOR
}

// 0xfb1c — __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0xfb1c() {
    // IDA 0xfb1c: `B.W RBX::Instance::~Instance()` — D1 runs the base dtor in
    // place; Rust Drop glue covers it.
}

/// Get/set pair behind `EnumPropDescriptor<CRenderSettingsItem, ResolutionPreset>`
/// (the +0x2c member desc: IDA 0xfe84 `new(0x14)` holding the getter/setter
/// member pointers at 0xff7e..0xff84).
pub struct CRenderSettingsResolutionAccess {
    pub get: Box<dyn Fn(&CRenderSettingsItemState) -> i32 + Send + Sync>,
    pub set: Box<dyn Fn(&mut CRenderSettingsItemState, i32) + Send + Sync>,
}

/// `RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem, ResolutionPreset>`
/// (IDA 0xfe84): base `PropertyDescriptor` init plus the GetSet impl (+44) and
/// the `EnumDesc<ResolutionPreset>` singleton links (+40/+48).
pub struct CRenderSettingsResolutionProp {
    pub name: String,
    pub category: String,
    pub access: CRenderSettingsResolutionAccess,
    pub enum_desc: &'static RenderEnumDesc,
    pub attributes: u32,
    pub permissions: u32,
}

/// `Singleton<EnumDesc<ResolutionPreset>>::doGetSingleton` (IDA 0xfecc/0xff9a):
/// empty until the ClassDescriptor registration path lands.
static RESOLUTION_PRESET_DESC: LazyLock<RenderEnumDesc> = LazyLock::new(RenderEnumDesc::default);

/// Minimal `XmlNameValuePair` image behind `EnumPropDescriptor::writeValue` /
/// `readValue` (IDA 0x102ac/0x102cc): the int and text payload views with
/// presence flags standing in for the `getValue<T>` overloads.
#[derive(Debug, Default, Clone)]
pub struct ScriptXmlNameValuePair {
    pub int_value: Option<i32>,
    pub text: Option<String>,
}

/// Minimal `XmlElement` image (IDA 0x102cc): the xsi:nil flag (0x102f0) plus the
/// value pair at +12 (0x10326).
#[derive(Debug, Default, Clone)]
pub struct ScriptXmlElement {
    pub is_xsi_nil: bool,
    pub pair: ScriptXmlNameValuePair,
}

/// IDA 0x1050c `EnumDesc::convertToIndex`: position of the item payload in the
/// table (0 when absent).
fn enum_desc_index_of(desc: &RenderEnumDesc, value: i32) -> i32 {
    desc.enum_to_item
        .iter()
        .position(|&v| v == value)
        .map(|i| i as i32)
        .unwrap_or(0)
}

// 0xfb20 — __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: int __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0xfb20() {
    // IDA 0xfb20: D0 — `Instance` dtor (0xfb26) + `operator delete` (0xfb30);
    // Arc Drop glue covers both.
}

// 0xfb34 — __ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0xfb34() {
    // IDA 0xfb34: ZThn32 D1 — `this -= 32` (0xfb36 region) then the `Instance`
    // dtor in place; the base offset collapses under single inheritance.
}

// 0xfb3c — __ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0xfb3c() {
    // IDA 0xfb3c: ZThn32 D0 — `this -= 32`, `Instance` dtor,
    // `operator delete`; Drop glue covers it.
}

// 0xfb54 — __ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0xfb54() {
    // IDA 0xfb54: ZThn36 D1 — `this -= 36` then the `Instance` dtor in place;
    // same collapse as 0xfb34.
}

// 0xfb5c — __ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0xfb5c() {
    // IDA 0xfb5c: ZThn36 D0 — `this -= 36`, `Instance` dtor,
    // `operator delete`; Drop glue covers it.
}

// 0xfb74 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiEC2IMNS_15CRenderSettingsEKFjvEMS2_FvjEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE — RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::PropDescriptor<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>(char const*,char const*,unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::PropDescriptor<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>(char const*,char const*,unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiEC2IMNS_15CRenderSettingsEKFjvEMS2_FvjEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xfb74(
    name: &str,
    category: &str,
    get: Box<dyn Fn(&CRenderSettingsItemState) -> i32 + Send + Sync>,
    set: Box<dyn Fn(&mut CRenderSettingsItemState, i32) + Send + Sync>,
    attributes: u32,
    permissions: u32,
) -> CRenderSettingsIntProp {
    // IDA 0xfb74: `Described<CRenderSettingsItem>::classDescriptor()` init
    // (0xfb9c), `new(0x14)` member desc holding the (getter, setter)
    // member-pointer pair (0xfba2..0xfbd8), base
    // `TypedPropertyDescriptor<int>` init (0xfc1a), temp release
    // (0xfc22..0xfc24), vtable install (0xfc38). Unlike the
    // EnumPropDescriptor ctors there is no read-only/write-only mask here.
    CRenderSettingsIntProp {
        name: name.to_owned(),
        category: category.to_owned(),
        access: CRenderSettingsIntAccess { get, set },
        attributes,
        permissions,
    }
}

// 0xfc88 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiED0Ev — RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::~PropDescriptor()
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::~PropDescriptor() [0xfc88]")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiED0Ev")]
pub fn stub_0xfc88() {
    // IDA 0xfc88: D0 deleting destructor — vtable reset (0xfc9c), member delete
    // (0xfc9e..0xfc9a4), `operator delete`; Arc Drop glue covers it.
}

// 0xfcb4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE10isReadOnlyEv — RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::isReadOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE10isReadOnlyEv")]
pub fn stub_0xfcb4() -> bool {
    // IDA 0xfcb4 `GetSetImpl::isReadOnly`: hardcoded `return 0` — get/set-bound
    // props are never read-only.
    false
}

// 0xfcb8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE11isWriteOnlyEv — RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::isWriteOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE11isWriteOnlyEv")]
pub fn stub_0xfcb8() -> bool {
    // IDA 0xfcb8 `GetSetImpl::isWriteOnly`: hardcoded `return 0` — get/set-bound
    // props are never write-only.
    false
}

// 0xfcbc — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE8getValueEPKNS0_13DescribedBaseE — RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0xfcbc(access: &CRenderSettingsIntAccess, obj: &CRenderSettingsItemState) -> i32 {
    // IDA 0xfcbc `GetSetImpl::getValue`: member-pointer dispatch out of the
    // +4/+8 pair with the `a2 ? a2-36 : 0` base adjust and the virtual/low-bit
    // branches (0xfcbe..0xfce4). The pair folds into the access closure; the
    // unsigned-int payload folds to i32, matching the EnumDesc tables.
    (access.get)(obj)
}

// 0xfce8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE8setValueEPNS0_13DescribedBaseERKi — RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE8setValueEPNS0_13DescribedBaseERKi")]
pub fn stub_0xfce8(access: &CRenderSettingsIntAccess, obj: &mut CRenderSettingsItemState, value: i32) {
    // IDA 0xfce8 `GetSetImpl::setValue`: same member-pointer dispatch as the
    // getValue twin at 0xfcbc, forwarding the int payload (0xfce8..0xfd08).
    (access.set)(obj, value);
}

// 0xfd0c — __ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EEC2EMS2_FivEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE — RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::BoundFuncDesc(int (CRenderSettingsItem::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// type: int __fastcall(int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::BoundFuncDesc(int (CRenderSettingsItem::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EEC2EMS2_FivEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0xfd0c(
    name: &str,
    call: Box<dyn Fn(&CRenderSettingsItemState) -> i32 + Send + Sync>,
    permissions: u32,
    attributes: u32,
) -> CRenderSettingsBoundFunc {
    // IDA 0xfd0c: base `FunctionDescriptor` init against
    // `describedClassDescriptor` (0xfd32..0xfd52), vtable install (0xfd6e), the
    // member-function pair stored at +0x28 (0xfd7a STRD), return-type
    // `Type::getSingleton<int>` recorded at +0x1c (0xfd9c..0xfda2). The member
    // function is fixed (arity 0, int return), so the pair folds into `call`.
    CRenderSettingsBoundFunc { name: name.to_owned(), call, permissions, attributes }
}

// 0xfe04 — __ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EED0Ev — RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::~BoundFuncDesc()
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::~BoundFuncDesc() [0xfe04]")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EED0Ev")]
pub fn stub_0xfe04() {
    // IDA 0xfe04: D0 deleting destructor — vtable reset (0xfe1c), signature-list
    // clear (0xfe20), `operator delete`; Arc Drop glue covers it.
}

// 0xfe30 — __ZNK3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE — RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0xfe30(func: &CRenderSettingsBoundFunc, obj: &CRenderSettingsItemState) -> i32 {
    // IDA 0xfe30 `execute`: member-offset adjust (`a2 ? a2-36 : 0`,
    // 0xfe38..0xfe3a), then `Call0Helper::call(member-fn@+40/+44, args@+4)`
    // (0xfe3c..0xfe4e). The adjust is member-pointer mechanics; the observable
    // effect is invoking the bound callable (see stub_0xfe54).
    (func.call)(obj)
}

// 0xfe54 — __ZN3RBX10Reflection11Call0HelperI19CRenderSettingsItemMS2_FivEiE4callEPS2_S4_RNS0_7VariantE — RBX::Reflection::Call0Helper<CRenderSettingsItem,int (CRenderSettingsItem::*)(void),int>::call(CRenderSettingsItem*,int (CRenderSettingsItem::*)(void),RBX::Reflection::Variant &)
// type: int __fastcall(int, int (__fastcall *)(_DWORD), int, _DWORD *)
#[doc(alias = "RBX::Reflection::Call0Helper<CRenderSettingsItem,int (CRenderSettingsItem::*)(void),int>::call(CRenderSettingsItem*,int (CRenderSettingsItem::*)(void),RBX::Reflection::Variant &)")]
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperI19CRenderSettingsItemMS2_FivEiE4callEPS2_S4_RNS0_7VariantE")]
pub fn stub_0xfe54(
    func: &dyn Fn(&CRenderSettingsItemState) -> i32,
    obj: &CRenderSettingsItemState,
) -> Variant {
    // IDA 0xfe54 `Call0Helper::call`: member-pointer adjust (`a1 + (a3 >> 1)`,
    // virtual via `a3 & 1`, 0xfe5a..0xfe68), `v7 = mf(obj)` (0xfe6c), out =
    // `Variant(int, v7)` via `Type::getSingleton<int>` + `operator=<int>`
    // (0xfe72..0xfe80). The pair folds into `func`; `Variant::Int` is the out.
    Variant::Int(func(obj))
}

// 0xfe84 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE — RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::EnumPropDescriptor<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>(char const*,char const*,RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::EnumPropDescriptor<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>(char const*,char const*,RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xfe84(
    name: &str,
    category: &str,
    get: Box<dyn Fn(&CRenderSettingsItemState) -> i32 + Send + Sync>,
    set: Box<dyn Fn(&mut CRenderSettingsItemState, i32) + Send + Sync>,
    attributes: u32,
    permissions: u32,
) -> CRenderSettingsResolutionProp {
    // IDA 0xfe84 `EnumPropDescriptor` ctor: `classDescriptor` init (0xfea8),
    // `EnumDesc<ResolutionPreset>` singleton `call_once` init (0xfec8..0xff9a),
    // base `PropertyDescriptor` init (0xff16), singleton links at +40/+48
    // (0xff3a..0xffa4), `new(0x14)` GetSet impl (0xff62..0xff88), then the
    // read-only/write-only attribute masks (0xffb4..0xffda).
    CRenderSettingsResolutionProp {
        name: name.to_owned(),
        category: category.to_owned(),
        access: CRenderSettingsResolutionAccess { get, set },
        enum_desc: &RESOLUTION_PRESET_DESC,
        attributes,
        permissions,
    }
}

// 0x10038 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEED0Ev — RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::~EnumPropDescriptor()
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEED0Ev")]
pub fn stub_0x10038() {
    // IDA 0x10038: D0 deleting destructor — vtable reset (0x1004c), member
    // delete (0x1004e..0x10054), `operator delete`; Arc Drop glue covers it.
}

// 0x10064 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10isReadOnlyEv — RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::isReadOnly(void)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10isReadOnlyEv")]
pub fn stub_0x10064(prop: &CRenderSettingsResolutionProp) -> bool {
    // IDA 0x10064 `isReadOnly`: routes through the +44 GetSet impl
    // (0x10070). Get/set-bound enum props are never read-only (cf. 0xfcb4).
    let _ = prop;
    false
}

// 0x10074 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11isWriteOnlyEv — RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::isWriteOnly(void)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11isWriteOnlyEv")]
pub fn stub_0x10074(prop: &CRenderSettingsResolutionProp) -> bool {
    // IDA 0x10074 `isWriteOnly`: routes through the +44 GetSet impl
    // (0x10080). Get/set-bound enum props are never write-only (cf. 0xfcb8).
    let _ = prop;
    false
}

// 0x10084 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11equalValuesEPKNS0_13DescribedBaseES8_ — RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11equalValuesEPKNS0_13DescribedBaseES8_")]
pub fn stub_0x10084(prop: &CRenderSettingsResolutionProp, obj: &CRenderSettingsItemState, value: i32) -> bool {
    // IDA 0x10084 `equalValues`: v5 = getValue(impl) (0x10094);
    // return v5 == getValue(impl, a3) (0x100aa).
    (prop.access.get)(obj) == value
}

// 0x100ac — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE — RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_0x100ac(prop: &CRenderSettingsResolutionProp, obj: &CRenderSettingsItemState) -> Variant {
    // IDA 0x100ac `getVariant`: v5 = getIntValue (0x100ba),
    // out = `Variant(int, v5)` via `Type::getSingleton<int>` +
    // `operator=<int>` (0x100c0..0x100ce). `Variant::Int` is the out.
    Variant::Int((prop.access.get)(obj))
}

// 0x100d0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE — RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_0x100d0(prop: &CRenderSettingsResolutionProp, obj: &mut CRenderSettingsItemState, variant: &Variant) {
    // IDA 0x100d0 `setVariant`: int-typed variants read the `any_cast<int>`
    // payload (0x1014e..0x101cc); anything else goes through the holder clone
    // + `Variant::convert<int>` (0x10150..0x1018e); then `setIntValue`
    // (0x101da). `convert_to_int` covers both int-source paths.
    let value = variant.convert_to_int();
    (prop.access.set)(obj, value);
}

// 0x10220 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE9copyValueEPKNS0_13DescribedBaseEPS6_ — RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE9copyValueEPKNS0_13DescribedBaseEPS6_")]
pub fn stub_0x10220(prop: &CRenderSettingsResolutionProp, src: &CRenderSettingsItemState, dst: &mut CRenderSettingsItemState) {
    // IDA 0x10220 `copyValue`: v6 = getValue(src-impl) (0x10232), then
    // setValue(dst-impl, &v6) (0x10242).
    let value = (prop.access.get)(src);
    (prop.access.set)(dst, value);
}

// 0x10244 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14hasStringValueEv — RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::hasStringValue(void)const
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14hasStringValueEv")]
pub fn stub_0x10244() -> bool {
    // IDA 0x10244 `hasStringValue`: hardcoded `return 1` — enum props always
    // have a string form.
    true
}

// 0x10248 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14getStringValueEPKNS0_13DescribedBaseE — RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getStringValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x10248(prop: &CRenderSettingsResolutionProp, obj: &CRenderSettingsItemState, out: &mut String) {
    // IDA 0x10248 `getStringValue`: v = getValue(impl, obj) (0x1025a), then
    // `EnumDesc<ResolutionPreset>::convertToString(desc, v, out)` (0x1026a ->
    // stub_0xc76c).
    let value = (prop.access.get)(obj);
    stub_0xc76c(prop.enum_desc, value, out);
}

// 0x1026c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14setStringValueEPNS0_13DescribedBaseERKSs — RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_0x1026c(prop: &CRenderSettingsResolutionProp, obj: &mut CRenderSettingsItemState, name: &str) -> bool {
    // IDA 0x1026c `setStringValue`: `Name::lookup(text)` (0x1027e),
    // `EnumDesc::convertToValue` (0x1028c); on a hit `setValue` runs
    // (0x10298..0x102a2) and 1 returns, otherwise 0.
    let mut value = 0;
    if enum_desc_value_by_name(prop.enum_desc, intern_name(name), &mut value) {
        (prop.access.set)(obj, value);
        true
    } else {
        false
    }
}

// 0x102ac — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement — RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_0x102ac(prop: &CRenderSettingsResolutionProp, obj: &CRenderSettingsItemState, pair: &mut ScriptXmlNameValuePair) -> u32 {
    // IDA 0x102ac `writeValue`: v = getValue (0x102ba), `clearValue`
    // (0x102c0), type tag 5 (0x102c6), payload v (0x102c8); returns 5.
    let value = (prop.access.get)(obj);
    *pair = ScriptXmlNameValuePair { int_value: Some(value), text: None };
    5
}

// 0x102cc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE — RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_0x102cc(prop: &CRenderSettingsResolutionProp, obj: &mut CRenderSettingsItemState, element: &ScriptXmlElement) {
    // IDA 0x102cc `readValue`: xsi:nil returns (0x102f0); an int payload runs
    // `setIntValue` and returns (0x10338..0x10348, void setters always
    // succeed); a string payload goes through `Name::lookup` +
    // `convertToValue` into `setValue` (0x10356..0x103b2), with the empty
    // string falling back to the +64 setter with 0 (0x103d4..0x10486);
    // anything else hits `ReleaseAssert(false)` (0x103c4..0x1042c).
    if element.is_xsi_nil {
        return;
    }
    if let Some(value) = element.pair.int_value {
        (prop.access.set)(obj, value);
        return;
    }
    if let Some(text) = &element.pair.text {
        let mut value = 0;
        if enum_desc_value_by_name(prop.enum_desc, intern_name(text), &mut value) {
            (prop.access.set)(obj, value);
            return;
        }
        if text.is_empty() {
            (prop.access.set)(obj, 0);
            return;
        }
    }
    panic!("false file: ../App/include/Reflection/Reflection.h line: 359");
}

// 0x1050c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE13getIndexValueEPKNS0_13DescribedBaseE — RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getIndexValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x1050c(prop: &CRenderSettingsResolutionProp, obj: &CRenderSettingsItemState) -> i32 {
    // IDA 0x1050c `getIndexValue`: v = getValue(impl) (0x1051c), then
    // `EnumDesc::convertToIndex(desc, v)` (0x10522).
    let value = (prop.access.get)(obj);
    enum_desc_index_of(prop.enum_desc, value)
}

// 0x10528 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE13setIndexValueEPNS0_13DescribedBaseEm — RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE13setIndexValueEPNS0_13DescribedBaseEm")]
pub fn stub_0x10528(prop: &CRenderSettingsResolutionProp, obj: &mut CRenderSettingsItemState, index: u32) -> bool {
    // IDA 0x10528 `setIndexValue`: count = [desc+40] (0x1053a); idx < count
    // loads the ordinal from [desc+144][idx] (0x10544), runs `setValue`
    // (0x1054e) and returns 1 (0x10550); otherwise 0.
    if (index as usize) < prop.enum_desc.value_count {
        let ordinal = prop.enum_desc.value_ordinals.get(index as usize).copied().unwrap_or(index as i32);
        (prop.access.set)(obj, ordinal);
        true
    } else {
        false
    }
}

// 0x1055c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE12getEnumValueEPKNS0_13DescribedBaseE — RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getEnumValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x1055c() -> ! {
    todo!("0x1055c __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE12getEnumValueEPKNS0_13DescribedBaseE")
}

// 0x10564 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE12setEnumValueEPNS0_13DescribedBaseEi — RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub fn stub_0x10564() -> ! {
    todo!("0x10564 __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE12setEnumValueEPNS0_13DescribedBaseEi")
}

// 0x105b0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11getEnumItemEPKNS0_13DescribedBaseE — RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getEnumItem(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub fn stub_0x105b0() -> ! {
    todo!("0x105b0 __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11getEnumItemEPKNS0_13DescribedBaseE")
}

// 0x105d0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE — RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub fn stub_0x105d0() -> ! {
    todo!("0x105d0 __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")
}

// 0x10604 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToIndexES3_ — RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToIndex(RBX::CRenderSettings::ResolutionPreset)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToIndex(RBX::CRenderSettings::ResolutionPreset)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToIndexES3_")]
pub fn stub_0x10604() -> ! {
    todo!("0x10604 __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToIndexES3_")
}

// 0x10674 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11setIntValueEPNS0_13DescribedBaseEi — RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setIntValue(RBX::Reflection::DescribedBase *,int)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0x10674() -> ! {
    todo!("0x10674 __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11setIntValueEPNS0_13DescribedBaseEi")
}

// 0x106b4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv — RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::isReadOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
pub fn stub_0x106b4() -> ! {
    todo!("0x106b4 __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv")
}

// 0x106b8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv — RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::isWriteOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
pub fn stub_0x106b8() -> ! {
    todo!("0x106b8 __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv")
}
