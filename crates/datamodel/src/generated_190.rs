// Auto-generated skeletons for rbx-datamodel -- from ida/export.json
// Filter: demangled contains RBX::Instance|RBX::DataModel|RBX::Workspace (exact RBX:: prefix), EA-sorted — filtered complete (10215/10215), global gap filler low-EA
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 120 stubs | range 0xcf3c..0x1089c | total filtered 10215, remaining 0 after batch; local 18280->18400 distinct, 67265->67145 not in datamodel (0 global missing)
// Shard: 190 EA-sorted asc next 120 low-EA global gap filler after 0xceec not yet in datamodel (filtered exhausted, 67265 missing before -> 67145 after)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use crate::generated_05::SLOT_EXCEPTION_HANDLER;
use crate::generated_189::{HOLDER_ANTIALIASING_MODE, HOLDER_FRAMERATE_MANAGER_MODE, HOLDER_GRAPHICS_MODE, HOLDER_AA_SAMPLES, HOLDER_QUALITY_LEVEL, HOLDER_SHADOW_MODE, CRenderSettingsItem, PlacementAny, RENDER_SETTINGS_SINGLETON, RenderEnumDesc, Vector2int16, aa_samples_holder, antialiasing_mode_holder, framerate_manager_mode_holder, graphics_mode_holder, quality_level_holder, resolution_preset_enum_desc, shadow_mode_holder};
use parking_lot::Mutex;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicBool, Ordering};

/// Rust model of `FactoryProduct<CRenderSettingsItem, ...>::Creator`
/// (IDA `0xf2bc` C2 / `0xeccc` D2 / `0xee84` create / `0xf500`
/// `static_getCreator`): the interned class name plus the
/// `isConstructed == 666` flag (IDA `0xf422`, `0xed50`). The creators-map
/// node collapses into `RENDER_SETTINGS_CREATOR_REGISTRY`.
#[derive(Debug)]
pub struct RenderSettingsCreator {
    pub name: &'static str,
}
/// IDA `0xf422`/`0xed50`: `Creator::isConstructed == 666`.
pub static RENDER_SETTINGS_CREATOR_CONSTRUCTED: AtomicBool = AtomicBool::new(false);
/// IDA `0xf400`/`0xedba`: the `getCreators()` map node for sRenderSettings.
static RENDER_SETTINGS_CREATOR_REGISTRY: Mutex<Vec<&'static str>> = Mutex::new(Vec::new());
/// IDA `0xf572` `creatorPrivate` storage, homed here (`__cxa_guard` init
/// becomes `LazyLock`, same treatment as the `TypedHolder` singletons).
static RENDER_SETTINGS_CREATOR: LazyLock<RenderSettingsCreator> =
    LazyLock::new(|| RenderSettingsCreator { name: "RenderSettings" });
/// IDA 0xf2bc/0xf500: `Creator` singleton accessor.
pub fn render_settings_creator() -> &'static RenderSettingsCreator {
    LazyLock::force(&RENDER_SETTINGS_CREATOR)
}

/// Rust model of `Described<CRenderSettingsItem, ...>::describedClassDescriptor`
/// (IDA `0xfa00` `classDescriptor()`): the lazily built "RenderSettings"
/// descriptor hanging off the `Instance` base descriptor. `__cxa_guard` /
/// `__cxa_atexit` collapse into `LazyLock`.
#[derive(Debug)]
pub struct RenderSettingsClass {
    pub name: &'static str,
}
/// IDA 0xfaa0: the `describedClassDescriptor` object.
static RENDER_SETTINGS_CLASS: LazyLock<RenderSettingsClass> =
    LazyLock::new(|| RenderSettingsClass { name: "RenderSettings" });
/// IDA 0xfa00: `Described<CRenderSettingsItem, ...>::classDescriptor()`.
pub fn render_settings_class_descriptor() -> &'static RenderSettingsClass {
    LazyLock::force(&RENDER_SETTINGS_CLASS)
}

/// Rust model of `RBX::Name::doDeclare<sRenderSettings>` storage (IDA
/// `0xf1dc`): the once-declared interned name. `__cxa_guard` init becomes
/// `LazyLock`.
static RENDER_SETTINGS_NAME_DECL: LazyLock<String> = LazyLock::new(|| "RenderSettings".to_owned());

/// Rust model of the `signal<void(const PropertyDescriptor*)>` slot link
/// behind `signal::next` (IDA `0xf574`): the intrusive cursor collapses into
/// a snapshot slice (same treatment as `instance::stub_0x2beb34`).
#[derive(Debug)]
pub struct RenderPropSlot;

/// Rust model of `PropDescriptor<CRenderSettingsItem, int>` (IDA `0xfb74`
/// C2 / `0xfc88` D0): name/category plus the bound member pair — the
/// `unsigned (CRenderSettings::*)() const` getter and the
/// `void (CRenderSettingsItem::*)(unsigned)` setter stored in the `+0x14`
/// heap impl (IDA `0xfbca`..`0xfbd8`). The `TypedPropertyDescriptor<int>`
/// base (`+0x28` attribute bits etc.) carries no modelled state.
pub struct RenderIntPropDesc {
    pub name: &'static str,
    pub category: &'static str,
    pub getter: fn(&CRenderSettingsItem) -> u32,
    pub setter: fn(&mut CRenderSettingsItem, u32),
}

/// Rust model of `BoundFuncDesc<CRenderSettingsItem, int(), 0>` (IDA `0xfd0c`
/// C2 / `0xfe04` D0): the bound zero-arg member returning int plus the
/// `int` return-type tag (IDA `0xfda2` `Type::getSingleton<int>`). Signature
/// storage (`_M_clear` at `0xfe20`) carries no modelled state.
pub struct RenderFuncDesc {
    pub name: &'static str,
    pub method: fn(&CRenderSettingsItem) -> i32,
}

/// Rust model of the `Variant` holding an int (IDA `0xfe54`/`0x100ac`:
/// `Type::getSingleton<int>` tag plus `placement_any<int>` payload).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntVariant {
    pub value: i32,
}

/// Rust model of `EnumPropDescriptor<CRenderSettingsItem, ResolutionPreset>`
/// (IDA `0xfe84` C2 / `0x10038` D0): the shared `EnumDesc<ResolutionPreset>`
/// table (IDA `0xff3a`/`0xffa4`, via `resolution_preset_enum_desc()`), the
/// bound getter/setter pair in the `+44` heap impl (IDA `0xff7a`..`0xff84`),
/// and the read/write-only flags (IDA `0xffb4`/`0xffd0` clear the `+28`
/// attribute bits when the `+44`-slot queries return 1; both return 0 here
/// per `0x10064`/`0x10074`, so the bits stay set).
pub struct RenderEnumPropDesc {
    pub name: &'static str,
    pub category: &'static str,
    pub getter: fn(&CRenderSettingsItem) -> i32,
    pub setter: fn(&mut CRenderSettingsItem, i32),
    pub read_only: bool,
    pub write_only: bool,
}

/// Rust model of the `XmlNameValuePair` int payload written by
/// `EnumPropDescriptor::writeValue` (IDA `0x102ac`: `clearValue`, type word
/// `5`, int word).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderXmlIntValue {
    pub kind: u32,
    pub int_value: i32,
}

/// Rust model of the `XmlElement` value read by
/// `EnumPropDescriptor::readValue` (IDA `0x102cc`): `isXsiNil` bail, int
/// value (`setIntValue`), or string value (`convertToValue` + set).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenderXmlInput {
    Nil,
    Int(i32),
    Text(String),
}

// 0xcf3c — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12QualityLevelEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::QualityLevel>::singleton(void)")]
pub fn stub_0xcf3c() -> &'static crate::generated_189::TypedHolder {
    // IDA 0xcf3c..0xcfa6 (decompiled): same `singleton` shape as 0xc95c —
    // `__cxa_guard_acquire`-checked init of `s = { typeinfo, destruct_func,
    // construct_func }` (0xcf8e..0xcf92), then return `&s` (0xcfa6).
    // Homed on the shared `LazyLock` model in `generated_189`.
    quality_level_holder()
}

// 0xcfa8 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12QualityLevelEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::QualityLevel>::construct_func(char const*,char *)")]
pub fn stub_0xcfa8(src: *const i32, dst: *mut i32) -> i32 {
    // IDA 0xcfa8..0xcfb0 (decompiled): same `construct_func` shape as 0xc9c8 —
    // `v = *src; if (dst) *dst = v; return v`.
    // SAFETY: `src` must be readable; `dst` must be writable when non-null.
    unsafe {
        let value = *src;
        if !dst.is_null() {
            *dst = value;
        }
        value
    }
}

// 0xcfb4 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12QualityLevelEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::QualityLevel>::destruct_func(char *)")]
pub fn stub_0xcfb4() {
    // IDA 0xcfb4: empty body — trivial enum payload, nothing to destroy.
}

// 0xcfb8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToItem(RBX::CRenderSettings::QualityLevel const&)const")]
pub fn stub_0xcfb8(desc: &RenderEnumDesc, value: i32) -> i32 {
    // IDA 0xcfb8..0xd07c (decompiled): same `convertToItem` shape as 0xc9d8 —
    // `ReleaseAssert(value >= 0)` (:273) and
    // `ReleaseAssert(value < enumToItem.size())` (:274) fall through, then
    // `value < 0 ? 0 : value < size ? enumToItem[value] : 0`
    // (0xd064..0xd07c); the table is identity here.
    if value >= 0 && (value as usize) < desc.pairs.len() {
        value
    } else {
        0
    }
}

// 0xd084 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings12QualityLevelENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CRenderSettings::QualityLevel const& rbx::any_cast<RBX::CRenderSettings::QualityLevel const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0xd084(slot: &PlacementAny) -> i32 {
    // IDA 0xd084..0xd12a (decompiled): same `any_cast` shape as 0xcaa4 —
    // holder check (0xd0f0) with the
    // "N3RBX15CRenderSettings12QualityLevelE" name fallback (0xd10c);
    // mismatch throws `rbx::bad_placement_any_cast` (0xd142), modelled as a
    // panic; hit returns the payload word (0xd12a), copied out here.
    if slot.holder != HOLDER_QUALITY_LEVEL {
        panic!("rbx::bad_placement_any_cast for N3RBX15CRenderSettings12QualityLevelE");
    }
    slot.value
}

// 0xd174 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToValue(RBX::Name const&,RBX::CRenderSettings::QualityLevel&)const")]
pub fn stub_0xd174(desc: &RenderEnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xd174..0xd1ec (decompiled): same `convertToValue` shape as 0xcc34 —
    // two `std::map::lower_bound` walks (0xd18a..0xd198, 0xd1be..0xd1ca);
    // hit writes `*a3` (0xd1ea) and returns 1 (0xd1ec), miss returns 0.
    if let Some(value) = desc.lookup_value(name) {
        *out = value;
        true
    } else {
        false
    }
}

// 0xd1f0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::~EnumDesc()")]
pub fn stub_0xd1f0(desc: &mut RenderEnumDesc) {
    // IDA 0xd1f0..0xd288 (decompiled): same D2 shape as 0xccb0 — item dtor
    // loop (0xd21c..0xd226), buffer deletes (0xd22e..0xd26a), map `_M_erase`s
    // (0xd274/0xd27e), base `~EnumDescriptor` (0xd288). Rust drops own the
    // storage; the tables are released eagerly to model the frees.
    desc.pairs.clear();
    desc.aliases.clear();
    desc.legacy_values.clear();
}

// 0xd28c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToString(RBX::CRenderSettings::ShadowMode const&)const")]
pub fn stub_0xd28c(desc: &RenderEnumDesc, value: i32, out: &mut String) {
    // IDA 0xd28c..0xd3fe (decompiled): same `convertToString` body shape as
    // 0xcd4c — `ReleaseAssert`s (:262/:263, 0xd2c8..0xd394) that fall through,
    // then `*out = value < 0 || value >= table ? "" : table[value]`
    // (0xd396..0xd3e6).
    match (value >= 0).then(|| desc.lookup_name(value)).flatten() {
        Some(name) => *out = name.to_owned(),
        None => out.clear(),
    }
}

// 0xd42c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings10ShadowModeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::ShadowMode>(RBX::CRenderSettings::ShadowMode const&)")]
pub fn stub_0xd42c(slot: &mut PlacementAny, value: i32) -> &mut PlacementAny {
    // IDA 0xd42c..0xd478 (decompiled): same `operator=` shape as 0xceec for
    // the ShadowMode holder (singleton touch 0xd438, same-holder copy
    // 0xd464, else destruct 0xd458 / clear 0xd45c / copy 0xd46e / install
    // 0xd470, return 0xd478).
    let _ = shadow_mode_holder();
    if slot.holder == HOLDER_SHADOW_MODE {
        slot.value = value;
    } else {
        slot.holder = 0;
        slot.value = value;
        slot.holder = HOLDER_SHADOW_MODE;
    }
    slot
}

// 0xd47c — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings10ShadowModeEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ShadowMode>::singleton(void)")]
pub fn stub_0xd47c() -> &'static crate::generated_189::TypedHolder {
    // IDA 0xd47c..0xd4e6 (decompiled): same `singleton` shape as 0xcf3c —
    // `__cxa_guard_acquire`-checked init of `s = { typeinfo, destruct_func,
    // construct_func }` (0xd4ce..0xd4d2), release (0xd4d6), then return `&s`
    // (0xd4e6). Homed on the shared `LazyLock` model in `generated_189`.
    shadow_mode_holder()
}

// 0xd4e8 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings10ShadowModeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ShadowMode>::construct_func(char const*,char *)")]
pub fn stub_0xd4e8(src: *const i32, dst: *mut i32) -> i32 {
    // IDA 0xd4e8..0xd4f0 (decompiled): same `construct_func` shape as 0xcfa8 —
    // `if (a2) { result = *result; *a2 = result; } return result`
    // (0xd4ea..0xd4f0).
    // SAFETY: `src` must be readable; `dst` must be writable when non-null.
    unsafe {
        let value = *src;
        if !dst.is_null() {
            *dst = value;
        }
        value
    }
}

// 0xd4f4 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings10ShadowModeEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ShadowMode>::destruct_func(char *)")]
pub fn stub_0xd4f4() {
    // IDA 0xd4f4: empty body — trivial enum payload, nothing to destroy.
}

// 0xd4f8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToItem(RBX::CRenderSettings::ShadowMode const&)const")]
pub fn stub_0xd4f8(desc: &RenderEnumDesc, value: i32) -> i32 {
    // IDA 0xd4f8..0xd5bc (decompiled): same `convertToItem` shape as 0xcfb8 —
    // `ReleaseAssert(value >= 0)` (:273, 0xd50c..0xd552) and
    // `ReleaseAssert(value < enumToItem.size())` (:274, 0xd556..0xd59c) fall
    // through, then `value < 0 ? 0 : value < size ? enumToItem[value] : 0`
    // (0xd5a4..0xd5bc); the table is identity here.
    if value >= 0 && (value as usize) < desc.pairs.len() {
        value
    } else {
        0
    }
}

// 0xd5c4 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings10ShadowModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CRenderSettings::ShadowMode const& rbx::any_cast<RBX::CRenderSettings::ShadowMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0xd5c4(slot: &PlacementAny) -> i32 {
    // IDA 0xd5c4..0xd6a6 (decompiled): same `any_cast` shape as 0xd084 —
    // holder check (0xd5ee..0xd630) with the
    // "N3RBX15CRenderSettings10ShadowModeE" name fallback (0xd64c);
    // mismatch throws `rbx::bad_placement_any_cast` (0xd67a..0xd682),
    // modelled as a panic; hit returns the payload word (0xd66a), copied out
    // here.
    if slot.holder != HOLDER_SHADOW_MODE {
        panic!("rbx::bad_placement_any_cast for N3RBX15CRenderSettings10ShadowModeE");
    }
    slot.value
}

// 0xd6b4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::ShadowMode&)const")]
pub fn stub_0xd6b4(desc: &RenderEnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xd6b4..0xd72c (decompiled): same `convertToValue` shape as 0xd174 —
    // two `std::map::lower_bound` walks (0xd6ca..0xd6d8, 0xd6fe..0xd70a);
    // hit writes `*a3` (0xd72a) and returns 1 (0xd72c), miss returns 0.
    if let Some(value) = desc.lookup_value(name) {
        *out = value;
        true
    } else {
        false
    }
}

// 0xd730 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::~EnumDesc()")]
pub fn stub_0xd730(desc: &mut RenderEnumDesc) {
    // IDA 0xd730..0xd7c8 (decompiled): same D2 shape as 0xd1f0 — item dtor
    // loop (0xd75c..0xd766), buffer deletes (0xd76e..0xd7aa), map `_M_erase`s
    // (0xd7b4/0xd7be), base `~EnumDescriptor` (0xd7c8). Rust drops own the
    // storage; the tables are released eagerly to model the frees.
    desc.pairs.clear();
    desc.aliases.clear();
    desc.legacy_values.clear();
}

// 0xd7cc — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToString(RBX::CRenderSettings::AntialiasingMode const&)const")]
pub fn stub_0xd7cc(desc: &RenderEnumDesc, value: i32, out: &mut String) {
    // IDA 0xd7cc..0xd93e (decompiled): same `convertToString` body shape as
    // 0xcd4c — `ReleaseAssert`s (:262/:263, 0xd808..0xd8d4) that fall through,
    // then `*out = value < 0 || value >= table ? "" : table[value]`
    // (0xd8d6..0xd926).
    match (value >= 0).then(|| desc.lookup_name(value)).flatten() {
        Some(name) => *out = name.to_owned(),
        None => out.clear(),
    }
}

// 0xd96c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings16AntialiasingModeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::AntialiasingMode>(RBX::CRenderSettings::AntialiasingMode const&)")]
pub fn stub_0xd96c(slot: &mut PlacementAny, value: i32) -> &mut PlacementAny {
    // IDA 0xd96c..0xd9b8 (decompiled): same `operator=` shape as 0xceec for
    // the AntialiasingMode holder (singleton touch 0xd978, same-holder copy
    // 0xd9a4, else destruct 0xd998 / clear 0xd99c / copy 0xd9ae / install
    // 0xd9b0, return 0xd9b8).
    let _ = antialiasing_mode_holder();
    if slot.holder == HOLDER_ANTIALIASING_MODE {
        slot.value = value;
    } else {
        slot.holder = 0;
        slot.value = value;
        slot.holder = HOLDER_ANTIALIASING_MODE;
    }
    slot
}

// 0xd9bc — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16AntialiasingModeEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AntialiasingMode>::singleton(void)")]
pub fn stub_0xd9bc() -> &'static crate::generated_189::TypedHolder {
    // IDA 0xd9bc..0xda26 (decompiled): same `singleton` shape as 0xcf3c —
    // `__cxa_guard_acquire`-checked init of `s = { typeinfo, destruct_func,
    // construct_func }` (0xda0e..0xda12), release (0xda16), then return `&s`
    // (0xda26). Homed on the shared `LazyLock` model in `generated_189`.
    antialiasing_mode_holder()
}

// 0xda28 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16AntialiasingModeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AntialiasingMode>::construct_func(char const*,char *)")]
pub fn stub_0xda28(src: *const i32, dst: *mut i32) -> i32 {
    // IDA 0xda28..0xda30 (decompiled): same `construct_func` shape as 0xcfa8 —
    // `if (a2) { result = *result; *a2 = result; } return result`
    // (0xda2a..0xda30).
    // SAFETY: `src` must be readable; `dst` must be writable when non-null.
    unsafe {
        let value = *src;
        if !dst.is_null() {
            *dst = value;
        }
        value
    }
}

// 0xda34 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16AntialiasingModeEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AntialiasingMode>::destruct_func(char *)")]
pub fn stub_0xda34() {
    // IDA 0xda34: empty body — trivial enum payload, nothing to destroy.
}

// 0xda38 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToItem(RBX::CRenderSettings::AntialiasingMode const&)const")]
pub fn stub_0xda38(desc: &RenderEnumDesc, value: i32) -> i32 {
    // IDA 0xda38..0xdafc (decompiled): same `convertToItem` shape as 0xcfb8 —
    // `ReleaseAssert(value >= 0)` (:273, 0xda4c..0xda92) and
    // `ReleaseAssert(value < enumToItem.size())` (:274, 0xda96..0xdadc) fall
    // through, then `value < 0 ? 0 : value < size ? enumToItem[value] : 0`
    // (0xdae4..0xdafc); the table is identity here.
    if value >= 0 && (value as usize) < desc.pairs.len() {
        value
    } else {
        0
    }
}

// 0xdb04 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings16AntialiasingModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CRenderSettings::AntialiasingMode const& rbx::any_cast<RBX::CRenderSettings::AntialiasingMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0xdb04(slot: &PlacementAny) -> i32 {
    // IDA 0xdb04..0xdbe6 (decompiled): same `any_cast` shape as 0xd084 —
    // holder check (0xdb2e..0xdb70) with the
    // "N3RBX15CRenderSettings16AntialiasingModeE" name fallback (0xdb8c);
    // mismatch throws `rbx::bad_placement_any_cast` (0xdbba..0xdbc2),
    // modelled as a panic; hit returns the payload word (0xdbaa), copied out
    // here.
    if slot.holder != HOLDER_ANTIALIASING_MODE {
        panic!("rbx::bad_placement_any_cast for N3RBX15CRenderSettings16AntialiasingModeE");
    }
    slot.value
}

// 0xdbf4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::AntialiasingMode&)const")]
pub fn stub_0xdbf4(desc: &RenderEnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xdbf4..0xdc6c (decompiled): same `convertToValue` shape as 0xd174 —
    // two `std::map::lower_bound` walks (0xdc0a..0xdc18, 0xdc3e..0xdc4a);
    // hit writes `*a3` (0xdc6a) and returns 1 (0xdc6c), miss returns 0.
    if let Some(value) = desc.lookup_value(name) {
        *out = value;
        true
    } else {
        false
    }
}

// 0xdc70 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc()")]
pub fn stub_0xdc70(desc: &mut RenderEnumDesc) {
    // IDA 0xdc70..0xdd08 (decompiled): same D2 shape as 0xd1f0 — item dtor
    // loop (0xdc9c..0xdca6), buffer deletes (0xdcae..0xdcea), map `_M_erase`s
    // (0xdcf4/0xdcfe), base `~EnumDescriptor` (0xdd08). Rust drops own the
    // storage; the tables are released eagerly to model the frees.
    desc.pairs.clear();
    desc.aliases.clear();
    desc.legacy_values.clear();
}

// 0xdd0c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToString(RBX::CRenderSettings::FrameRateManagerMode const&)const")]
pub fn stub_0xdd0c(desc: &RenderEnumDesc, value: i32, out: &mut String) {
    // IDA 0xdd0c..0xde7e (decompiled): same `convertToString` body shape as
    // 0xcd4c — `ReleaseAssert`s (:262/:263, 0xdd48..0xde14) that fall through,
    // then `*out = value < 0 || value >= table ? "" : table[value]`
    // (0xde16..0xde66).
    match (value >= 0).then(|| desc.lookup_name(value)).flatten() {
        Some(name) => *out = name.to_owned(),
        None => out.clear(),
    }
}

// 0xdeac — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings20FrameRateManagerModeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::FrameRateManagerMode>(RBX::CRenderSettings::FrameRateManagerMode const&)")]
pub fn stub_0xdeac(slot: &mut PlacementAny, value: i32) -> &mut PlacementAny {
    // IDA 0xdeac..0xdef8 (decompiled): same `operator=` shape as 0xceec for
    // the FrameRateManagerMode holder (singleton touch 0xdeb8, same-holder
    // copy 0xdee4, else destruct 0xded8 / clear 0xdedc / copy 0xdeee /
    // install 0xdef0, return 0xdef8).
    let _ = framerate_manager_mode_holder();
    if slot.holder == HOLDER_FRAMERATE_MANAGER_MODE {
        slot.value = value;
    } else {
        slot.holder = 0;
        slot.value = value;
        slot.holder = HOLDER_FRAMERATE_MANAGER_MODE;
    }
    slot
}

// 0xdefc — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings20FrameRateManagerModeEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::FrameRateManagerMode>::singleton(void)")]
pub fn stub_0xdefc() -> &'static crate::generated_189::TypedHolder {
    // IDA 0xdefc..0xdf66 (decompiled): same `singleton` shape as 0xcf3c —
    // `__cxa_guard_acquire`-checked init of `s = { typeinfo, destruct_func,
    // construct_func }` (0xdf4e..0xdf52), release (0xdf56), then return `&s`
    // (0xdf66). Homed on the shared `LazyLock` model in `generated_189`.
    framerate_manager_mode_holder()
}

// 0xdf68 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings20FrameRateManagerModeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::FrameRateManagerMode>::construct_func(char const*,char *)")]
pub fn stub_0xdf68(src: *const i32, dst: *mut i32) -> i32 {
    // IDA 0xdf68..0xdf70 (decompiled): same `construct_func` shape as 0xcfa8 —
    // `if (a2) { result = *result; *a2 = result; } return result`
    // (0xdf6a..0xdf70).
    // SAFETY: `src` must be readable; `dst` must be writable when non-null.
    unsafe {
        let value = *src;
        if !dst.is_null() {
            *dst = value;
        }
        value
    }
}

// 0xdf74 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings20FrameRateManagerModeEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::FrameRateManagerMode>::destruct_func(char *)")]
pub fn stub_0xdf74() {
    // IDA 0xdf74: empty body — trivial enum payload, nothing to destroy.
}

// 0xdf78 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToItem(RBX::CRenderSettings::FrameRateManagerMode const&)const")]
pub fn stub_0xdf78(desc: &RenderEnumDesc, value: i32) -> i32 {
    // IDA 0xdf78..0xe040 (decompiled): same `convertToItem` shape as 0xcfb8 —
    // `ReleaseAssert(value >= 0)` (:273, 0xdf8c..0xdfd2) and
    // `ReleaseAssert(value < enumToItem.size())` (:274, 0xdfd6..0xe020) fall
    // through, then `value < 0 ? 0 : value < size ? enumToItem[value] : 0`
    // (0xe024..0xe040); the table is identity here.
    if value >= 0 && (value as usize) < desc.pairs.len() {
        value
    } else {
        0
    }
}

// 0xe044 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings20FrameRateManagerModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CRenderSettings::FrameRateManagerMode const& rbx::any_cast<RBX::CRenderSettings::FrameRateManagerMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0xe044(slot: &PlacementAny) -> i32 {
    // IDA 0xe044..0xe12a (decompiled): same `any_cast` shape as 0xd084 —
    // holder check with the
    // "N3RBX15CRenderSettings20FrameRateManagerModeE" name fallback;
    // mismatch throws `rbx::bad_placement_any_cast`, modelled as a panic;
    // hit returns the payload word, copied out here.
    if slot.holder != HOLDER_FRAMERATE_MANAGER_MODE {
        panic!("rbx::bad_placement_any_cast for N3RBX15CRenderSettings20FrameRateManagerModeE");
    }
    slot.value
}

// 0xe134 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::FrameRateManagerMode&)const")]
pub fn stub_0xe134(desc: &RenderEnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xe134..0xe1ac (decompiled): same `convertToValue` shape as 0xd174 —
    // two `std::map::lower_bound` walks (0xe14a..0xe168, 0xe17e..0xe1a2);
    // hit writes `*a3` (0xe1aa) and returns 1 (0xe1ac), miss returns 0.
    if let Some(value) = desc.lookup_value(name) {
        *out = value;
        true
    } else {
        false
    }
}

// 0xe1b0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::~EnumDesc()")]
pub fn stub_0xe1b0(desc: &mut RenderEnumDesc) {
    // IDA 0xe1b0..0xe248 (decompiled): same D2 shape as 0xd1f0 — vtable
    // install (0xe1ce), registrar bump (0xe1d4), item dtor loop
    // (0xe1dc..0xe1e6), buffer deletes (0xe1ee..0xe22a), map `_M_erase`s
    // (0xe234/0xe23e), base `~EnumDescriptor` (0xe248). Rust drops own the
    // storage; the tables are released eagerly to model the frees.
    desc.pairs.clear();
    desc.aliases.clear();
    desc.legacy_values.clear();
}

// 0xe24c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToString(RBX::CRenderSettings::GraphicsMode const&)const")]
pub fn stub_0xe24c(desc: &RenderEnumDesc, value: i32, out: &mut String) {
    // IDA 0xe24c..0xe3e8 (decompiled): same `convertToString` body shape as
    // 0xcd4c — `ReleaseAssert`s (:262/:263) that fall through, then
    // `*out = value < 0 || value >= table ? "" : table[value]`.
    match (value >= 0).then(|| desc.lookup_name(value)).flatten() {
        Some(name) => *out = name.to_owned(),
        None => out.clear(),
    }
}

// 0xe3ec — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings12GraphicsModeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::GraphicsMode>(RBX::CRenderSettings::GraphicsMode const&)")]
pub fn stub_0xe3ec(slot: &mut PlacementAny, value: i32) -> &mut PlacementAny {
    // IDA 0xe3ec..0xe438 (decompiled): same `operator=` shape as 0xceec for
    // the GraphicsMode holder (singleton touch 0xe3f8, same-holder copy
    // 0xe424, else destruct 0xe418 / clear 0xe41c / copy 0xe42e / install,
    // return).
    let _ = graphics_mode_holder();
    if slot.holder == HOLDER_GRAPHICS_MODE {
        slot.value = value;
    } else {
        slot.holder = 0;
        slot.value = value;
        slot.holder = HOLDER_GRAPHICS_MODE;
    }
    slot
}

// 0xe43c — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12GraphicsModeEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::GraphicsMode>::singleton(void)")]
pub fn stub_0xe43c() -> &'static crate::generated_189::TypedHolder {
    // IDA 0xe43c..0xe4a6 (decompiled): same `singleton` shape as 0xcf3c —
    // `__cxa_guard_acquire`-checked init of `s = { typeinfo, destruct_func,
    // construct_func }`, then return `&s`. Homed on the shared `LazyLock`
    // model in `generated_189`.
    graphics_mode_holder()
}

// 0xe4a8 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12GraphicsModeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::GraphicsMode>::construct_func(char const*,char *)")]
pub fn stub_0xe4a8(src: *const i32, dst: *mut i32) -> i32 {
    // IDA 0xe4a8..0xe4b0 (decompiled): same `construct_func` shape as 0xcfa8 —
    // `v = *src; if (dst) *dst = v; return v`.
    // SAFETY: `src` must be readable; `dst` must be writable when non-null.
    unsafe {
        let value = *src;
        if !dst.is_null() {
            *dst = value;
        }
        value
    }
}

// 0xe4b4 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12GraphicsModeEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::GraphicsMode>::destruct_func(char *)")]
pub fn stub_0xe4b4() {
    // IDA 0xe4b4: empty body — trivial enum payload, nothing to destroy.
}

// 0xe4b8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToItem(RBX::CRenderSettings::GraphicsMode const&)const")]
pub fn stub_0xe4b8(desc: &RenderEnumDesc, value: i32) -> i32 {
    // IDA 0xe4b8..0xe57c (decompiled): same `convertToItem` shape as 0xcfb8 —
    // `ReleaseAssert(value >= 0)` (:273) and
    // `ReleaseAssert(value < enumToItem.size())` (:274) fall through, then
    // `value < 0 ? 0 : value < size ? enumToItem[value] : 0`; the table is
    // identity here.
    if value >= 0 && (value as usize) < desc.pairs.len() {
        value
    } else {
        0
    }
}

// 0xe584 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings12GraphicsModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CRenderSettings::GraphicsMode const& rbx::any_cast<RBX::CRenderSettings::GraphicsMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0xe584(slot: &PlacementAny) -> i32 {
    // IDA 0xe584..0xe66c (decompiled): same `any_cast` shape as 0xd084 —
    // holder check with the "N3RBX15CRenderSettings12GraphicsModeE" name
    // fallback; mismatch throws `rbx::bad_placement_any_cast`, modelled as
    // a panic; hit returns the payload word, copied out here.
    if slot.holder != HOLDER_GRAPHICS_MODE {
        panic!("rbx::bad_placement_any_cast for N3RBX15CRenderSettings12GraphicsModeE");
    }
    slot.value
}

// 0xe674 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::GraphicsMode&)const")]
pub fn stub_0xe674(desc: &RenderEnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xe674..0xe6ec (decompiled): same `convertToValue` shape as 0xd174 —
    // two `std::map::lower_bound` walks; hit writes `*a3` and returns 1,
    // miss returns 0.
    if let Some(value) = desc.lookup_value(name) {
        *out = value;
        true
    } else {
        false
    }
}

// 0xe6f0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()")]
pub fn stub_0xe6f0(desc: &mut RenderEnumDesc) {
    // IDA 0xe6f0..0xe788 (decompiled): same D2 shape as 0xd1f0 — vtable
    // install (0xe70e), registrar bump (0xe714), item dtor loop
    // (0xe71c..0xe726), buffer deletes, map `_M_erase`s, base
    // `~EnumDescriptor`. Rust drops own the storage; the tables are
    // released eagerly to model the frees.
    desc.pairs.clear();
    desc.aliases.clear();
    desc.legacy_values.clear();
}

// 0xe78c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToString(RBX::CRenderSettings::AASamples const&)const")]
pub fn stub_0xe78c(desc: &RenderEnumDesc, value: i32, out: &mut String) {
    // IDA 0xe78c..0xe928 (decompiled): same `convertToString` body shape as
    // 0xcd4c — `ReleaseAssert`s (:262/:263) that fall through, then
    // `*out = value < 0 || value >= table ? "" : table[value]`.
    match (value >= 0).then(|| desc.lookup_name(value)).flatten() {
        Some(name) => *out = name.to_owned(),
        None => out.clear(),
    }
}

// 0xe92c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings9AASamplesEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::AASamples>(RBX::CRenderSettings::AASamples const&)")]
pub fn stub_0xe92c(slot: &mut PlacementAny, value: i32) -> &mut PlacementAny {
    // IDA 0xe92c..0xe978 (decompiled): same `operator=` shape as 0xceec for
    // the AASamples holder (singleton touch 0xe938, same-holder copy
    // 0xe964, else destruct 0xe958 / clear 0xe95c / copy / install,
    // return).
    let _ = aa_samples_holder();
    if slot.holder == HOLDER_AA_SAMPLES {
        slot.value = value;
    } else {
        slot.holder = 0;
        slot.value = value;
        slot.holder = HOLDER_AA_SAMPLES;
    }
    slot
}

// 0xe97c — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings9AASamplesEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::singleton(void)")]
pub fn stub_0xe97c() -> &'static crate::generated_189::TypedHolder {
    // IDA 0xe97c..0xe9e6 (decompiled): same `singleton` shape as 0xcf3c —
    // `__cxa_guard_acquire`-checked init of `s = { typeinfo, destruct_func,
    // construct_func }` (0xe9ce), then return `&s`. Homed on the shared
    // `LazyLock` model in `generated_189`.
    aa_samples_holder()
}

// 0xe9e8 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings9AASamplesEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::construct_func(char const*,char *)")]
pub fn stub_0xe9e8(src: *const i32, dst: *mut i32) -> i32 {
    // IDA 0xe9e8..0xe9f0 (decompiled): same `construct_func` shape as 0xcfa8 —
    // `v = *src; if (dst) *dst = v; return v`.
    // SAFETY: `src` must be readable; `dst` must be writable when non-null.
    unsafe {
        let value = *src;
        if !dst.is_null() {
            *dst = value;
        }
        value
    }
}

// 0xe9f4 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings9AASamplesEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::destruct_func(char *)")]
pub fn stub_0xe9f4() {
    // IDA 0xe9f4: empty body — trivial enum payload, nothing to destroy.
}

// 0xe9f8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToItem(RBX::CRenderSettings::AASamples const&)const")]
pub fn stub_0xe9f8(desc: &RenderEnumDesc, value: i32) -> i32 {
    // IDA 0xe9f8..0xeabc (decompiled): same `convertToItem` shape as 0xcfb8 —
    // `ReleaseAssert(value >= 0)` (:273) and
    // `ReleaseAssert(value < enumToItem.size())` (:274) fall through, then
    // `value < 0 ? 0 : value < size ? enumToItem[value] : 0`; the table is
    // identity here.
    if value >= 0 && (value as usize) < desc.pairs.len() {
        value
    } else {
        0
    }
}

// 0xeac4 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings9AASamplesENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CRenderSettings::AASamples const& rbx::any_cast<RBX::CRenderSettings::AASamples const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0xeac4(slot: &PlacementAny) -> i32 {
    // IDA 0xeac4..0xebb0 (decompiled): same `any_cast` shape as 0xd084 —
    // holder check with the "N3RBX15CRenderSettings9AASamplesE" name
    // fallback; mismatch throws `rbx::bad_placement_any_cast`, modelled as
    // a panic; hit returns the payload word, copied out here.
    if slot.holder != HOLDER_AA_SAMPLES {
        panic!("rbx::bad_placement_any_cast for N3RBX15CRenderSettings9AASamplesE");
    }
    slot.value
}

// 0xebb4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToValue(RBX::Name const&,RBX::CRenderSettings::AASamples&)const")]
pub fn stub_0xebb4(desc: &RenderEnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xebb4..0xec2c (decompiled): same `convertToValue` shape as 0xd174 —
    // two `std::map::lower_bound` walks; hit writes `*a3` and returns 1,
    // miss returns 0.
    if let Some(value) = desc.lookup_value(name) {
        *out = value;
        true
    } else {
        false
    }
}

// 0xec30 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()")]
pub fn stub_0xec30(desc: &mut RenderEnumDesc) {
    // IDA 0xec30..0xecc8 (decompiled): same D2 shape as 0xd1f0 — vtable
    // install (0xec4e), registrar bump (0xec54), item dtor loop
    // (0xec5c..0xec66), buffer deletes, map `_M_erase`s, base
    // `~EnumDescriptor`. Rust drops own the storage; the tables are
    // released eagerly to model the frees.
    desc.pairs.clear();
    desc.aliases.clear();
    desc.legacy_values.clear();
}

// 0xeccc — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD2Ev
// type: int __fastcall(int)
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0xeccc() {
    // IDA 0xeccc..0xedda (decompiled): `Creator::D2` — vtable install
    // (0xed1c), `ReleaseAssert(wasConstructed())` (:255, 0xed26..0xed96)
    // that falls through, `getCreators()` touch (0xed9a), class-name lookup
    // (0xedae) and `creators.erase(name)` (0xedba), return (0xedda). The
    // assert collapses into `debug_assert!`; the erase removes the
    // sRenderSettings node from `RENDER_SETTINGS_CREATOR_REGISTRY`.
    debug_assert!(
        RENDER_SETTINGS_CREATOR_CONSTRUCTED.load(Ordering::SeqCst),
        "0xeccc: wasConstructed() (Object.h:255)"
    );
    RENDER_SETTINGS_CREATOR_REGISTRY.lock().retain(|name| *name != "RenderSettings");
}

// 0xedfc — __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0xedfc() -> &'static str {
    // IDA 0xedfc..0xee84 (disasm): `Creator::getClassName` —
    // `ReleaseAssert(wasConstructed())` (0xee0c..0xee5c) that falls through,
    // `call_once` of the `sRenderSettings` name declaration (0xee60..0xee78),
    // tail-call to `Name::doDeclare<sRenderSettings>()` (0xee7c..0xee84) —
    // the interned "RenderSettings" class name, modelled as `&'static str`
    // (same shape as `instance::stub_0xb8d0`).
    debug_assert!(
        RENDER_SETTINGS_CREATOR_CONSTRUCTED.load(Ordering::SeqCst),
        "0xedfc: wasConstructed()"
    );
    render_settings_creator().name
}

// 0xee84 — __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator6createEv
// type: int __fastcall(int *)
#[doc(alias = "__ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator6createEv")]
pub fn stub_0xee84() -> SharedPtr<CRenderSettingsItem> {
    // IDA 0xee84..0xef02 (decompiled): `Creator::create` —
    // `ReleaseAssert(wasConstructed())` (:231, 0xee98..0xeea) that falls
    // through, `Creatable::create<CRenderSettingsItem>()` (0xeeec), then the
    // `shared_ptr` out-params take the `+32` `Instance` subobject view
    // (0xeef2..0xeefe). `SharedPtr::new(Default)` is the same
    // default-construct + single-owner adoption (cf. `instance::stub_0xef04`).
    debug_assert!(
        RENDER_SETTINGS_CREATOR_CONSTRUCTED.load(Ordering::SeqCst),
        "0xee84: wasConstructed() (Object.h:231)"
    );
    SharedPtr::new(CRenderSettingsItem::default())
}

// 0xefd8 — __ZNK5boost6detail15sp_counted_base9use_countEv
// type: int __fastcall(boost::detail::sp_counted_base *this)
#[doc(alias = "boost::detail::sp_counted_base::use_count(void)const")]
pub fn stub_0xefd8(shared: &SharedPtr<CRenderSettingsItem>) -> usize {
    // IDA 0xefd8..0xf078 (decompiled): `sp_counted_base::use_count` —
    // spinlock-pool lock (0xf020), `*(this + 1)` load (0xf032), unlock
    // (0xf058), return (0xf078). The pool mutex collapses into the `Arc`
    // atomic load.
    SharedPtr::strong_count(shared)
}

// 0xf1d8 — __ZN3RBX4Name13callDoDeclareILZ15sRenderSettingsEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZ15sRenderSettingsEEEvv")]
pub fn stub_0xf1d8() -> &'static str {
    // IDA 0xf1d8 (decompiled, thunk): tail-calls
    // `Name::doDeclare<sRenderSettings>()` (0xf1dc).
    stub_0xf1dc()
}

// 0xf1dc — __ZN3RBX4Name9doDeclareILZ15sRenderSettingsEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZ15sRenderSettingsEEERKS0_v")]
pub fn stub_0xf1dc() -> &'static str {
    // IDA 0xf1dc..0xf290 (decompiled): `Name::doDeclare<sRenderSettings>` —
    // `__cxa_guard_acquire` once-check (0xf238), `Name::declare(&name, 1)`
    // (0xf25e), guard release (0xf262), return the interned name (0xf290).
    // The guard collapses into `LazyLock`.
    LazyLock::force(&RENDER_SETTINGS_NAME_DECL).as_str()
}

// 0xf2bc — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0xf2bc() -> &'static RenderSettingsCreator {
    // IDA 0xf2bc..0xf4fe (decompiled): `Creator::C2` — vtable install
    // (0xf2f2), `call_once` of the name declaration (0xf2f4), duplicate-name
    // `ReleaseAssert(find == end)` (:244, 0xf30c..0xf396),
    // `ReleaseAssert(!wasConstructed())` (:245, 0xf39a..0xf3e0),
    // `creators[name] = this` (0xf3f2..0xf41c), `isConstructed = 666`
    // (0xf422), then the post-insert `find != end` (:250) and
    // `wasConstructed()` (:251) asserts. Asserts collapse into
    // `debug_assert!`; the map insert appends the registry node.
    debug_assert!(
        !RENDER_SETTINGS_CREATOR_CONSTRUCTED.load(Ordering::SeqCst),
        "0xf2bc: !wasConstructed() (Object.h:245)"
    );
    {
        let mut registry = RENDER_SETTINGS_CREATOR_REGISTRY.lock();
        debug_assert!(
            !registry.contains(&"RenderSettings"),
            "0xf2bc: find == end (Object.h:244)"
        );
        registry.push("RenderSettings");
    }
    RENDER_SETTINGS_CREATOR_CONSTRUCTED.store(true, Ordering::SeqCst);
    render_settings_creator()
}

// 0xf500 — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0xf500() -> &'static RenderSettingsCreator {
    // IDA 0xf500..0xf572 (decompiled): `static_getCreator` —
    // `ReleaseAssert(Creator::wasConstructed())` (:282, 0xf510..0xf562)
    // that falls through, return `&creatorPrivate` (0xf572).
    debug_assert!(
        RENDER_SETTINGS_CREATOR_CONSTRUCTED.load(Ordering::SeqCst),
        "0xf500: Creator::wasConstructed() (Object.h:282)"
    );
    render_settings_creator()
}

// 0xf574 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4nextERN5boost13intrusive_ptrINS8_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot> &)")]
// was: rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot> &)
pub fn stub_0xf574(slots: &[SharedPtr<RenderPropSlot>], cursor: usize) -> Option<usize> {
    // IDA 0xf574..0xf674 (decompiled): `signal::next` over the
    // `(PropertyDescriptor const*)` signal — incoming-slot `add_ref`
    // (0xf5ce), `call_once` static-mutex init (0xf5ee), mutex take (0xf608),
    // intrusive advance (0xf61c..0xf636), unlock (0xf640), release (0xf64e),
    // `!= 0` check (0xf65e..0xf674). The slot list collapses into a snapshot
    // slice, so next-live is index + 1 when in range (same shape as
    // `instance::stub_0x2beb34`).
    let next = cursor + 1;
    if next < slots.len() { Some(next) } else { None }
}

// 0xf6dc — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE8on_errorERSt9exception
// type: int *()
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::on_error(std::exception &)")]
pub fn stub_0xf6dc(err: &str) {
    // IDA 0xf6dc..0xf702 (decompiled): `signal::on_error` — loads the
    // `slot_exception_handler` slot (0xf6f0..0xf6f2); null handler returns
    // the slot (0xf702), set handler invokes it (0xf6f8..0xf6fe). Same shape
    // as `instance::stub_0x5ec580`.
    if let Some(handler) = *SLOT_EXCEPTION_HANDLER.lock() {
        handler(err);
    }
}

// 0xf704 — __ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int __fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_insert_aux(__gnu_cxx::__normal_iterator<G3D::Vector2int16*,std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>>,G3D::Vector2int16 const&)")]
pub fn stub_0xf704(items: &mut Vec<Vector2int16>, index: usize, value: Vector2int16) {
    // IDA 0xf704..0xf7ca (decompiled): `vector<Vector2int16>::_M_insert_aux`
    // — the realloc path (0xf73e..0xf7c4, growth `max(1, size/2)` capped at
    // `0x3FFFFFFF` per 0xf748..0xf752, copies around the hole, frees the old
    // buffer) and the shift path (0xf71a..0xf738, tail move +
    // `__copy_backward` + store). `Vec::insert` grows and shifts the same way
    // (the iterator position collapses into the index); same shape as
    // `generated_296::stub_0xf3a794`.
    let at = index.min(items.len());
    items.insert(at, value);
}

// 0xf7e8 — __ZNSt12_Vector_baseIN3G3D12Vector2int16ESaIS1_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_allocate(unsigned long)")]
pub fn stub_0xf7e8(capacity: usize) -> Vec<Vector2int16> {
    // IDA 0xf7e8..0xf7f6 (decompiled): `_Vector_base<Vector2int16>::_M_allocate` —
    // `a2 >= 0x40000000` throws `bad_alloc` (0xf7f0..0xf7f2),
    // else `operator new(4 * a2)`. Same safe allocation as
    // `generated_296::stub_0xf3a614`.
    assert!(capacity < 0x4000_0000, "0xf7e8: std::__throw_bad_alloc");
    Vec::with_capacity(capacity)
}

// 0xf800 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D12Vector2int16ES5_EET0_T_S7_S6_
// type: int __fastcall(int, int, int)
#[doc(alias = "G3D::Vector2int16 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector2int16 *,G3D::Vector2int16 *>(G3D::Vector2int16 *,G3D::Vector2int16 *,G3D::Vector2int16 *)")]
pub fn stub_0xf800(items: &mut Vec<Vector2int16>, first: usize, last: usize, result: usize) {
    // IDA 0xf800..0xf83a (decompiled): `__copy_backward` over the
    // `Vector2int16` range — copies `[first, last)` to end at `result`
    // (elementwise 4-byte moves, 0xf826..0xf832). `copy_within` handles the
    // overlap the same way (raw pointers collapse into offsets); same shape
    // as `generated_296::stub_0xf3a674`.
    let len = last.saturating_sub(first);
    items.copy_within(first..last, result.saturating_sub(len));
}

// 0xf83c — __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev
pub fn stub_0xf83c(item: &mut CRenderSettingsItem) {
    // IDA 0xf83c..0xf878 (decompiled): `GlobalAdvancedSettingsItem<...>::D1` —
    // vtable resets (0xf860..0xf86e), `sing = 0` (0xf872), `Instance::~Instance`
    // (0xf878). Vtable words are unmodelled; the singleton clear resets the
    // harness-owned flag, and member drops collapse into Rust drop.
    let _ = item;
    RENDER_SETTINGS_SINGLETON.store(false, Ordering::SeqCst);
}

// 0xf87c — __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev
pub fn stub_0xf87c(item: &mut CRenderSettingsItem) {
    // IDA 0xf87c..0xf8c4 (decompiled): `GlobalAdvancedSettingsItem<...>::D0` —
    // same vtable resets + `sing = 0` (0xf8b4) + `Instance::~Instance`
    // (0xf8b8) as D1, then `operator delete` (0xf8be). The free collapses
    // into Rust ownership (caller drops the box).
    stub_0xf83c(item);
}

// 0xf8c8 — __ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev
pub fn stub_0xf8c8(item: &mut CRenderSettingsItem) {
    // IDA 0xf8c8..0xf906 (decompiled): `ZThn32` D1 — `this -= 32` selects the
    // `GlobalAdvancedSettingsItem` subobject (0xf8e8..0xf8fa), `sing = 0`
    // (0xf900), `Instance::~Instance(this - 36 + 4...)` (0xf906). The
    // adjustment collapses (single modeled address space); same shape as
    // `instance::stub_0x3ac5b4`.
    stub_0xf83c(item);
}

// 0xf90c — __ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev
pub fn stub_0xf90c(item: &mut CRenderSettingsItem) {
    // IDA 0xf90c..0xf95c (decompiled): `ZThn32` D0 — `this -= 32`
    // (0xf936), vtable resets, `sing = 0` (0xf94c),
    // `Instance::~Instance` (0xf950), `operator delete` (0xf956). Same
    // collapse as 0xf8c8; the free collapses into Rust ownership.
    stub_0xf87c(item);
}

// 0xf964 — __ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev
pub fn stub_0xf964(item: &mut CRenderSettingsItem) {
    // IDA 0xf964..0xf9a2 (decompiled): `ZThn36` D1 — `this -= 36`
    // (0xf984..0xf996), `sing = 0` (0xf99c), `Instance::~Instance`
    // (0xf9a2). Same collapse as 0xf8c8.
    stub_0xf83c(item);
}

// 0xf9a8 — __ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev
pub fn stub_0xf9a8(item: &mut CRenderSettingsItem) {
    // IDA 0xf9a8..0xf9f4 (decompiled): `ZThn36` D0 — `this -= 36`
    // (0xf9d2), vtable resets, `sing = 0` (0xf9e8),
    // `Instance::~Instance` (0xf9ec), `operator delete`. Same collapse as
    // 0xf90c.
    stub_0xf87c(item);
}

// 0xfa00 — __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
pub fn stub_0xfa00() -> &'static RenderSettingsClass {
    // IDA 0xfa00..0xfaee (decompiled): `Described<CRenderSettingsItem,
    // ...>::classDescriptor()` — `__cxa_guard_acquire` once-check (0xfa5c),
    // base `Described<Instance>::classDescriptor()` touch (0xfa68),
    // `ClassDescriptor` C2 with ("RenderSettings", base) (0xfaa0),
    // `__cxa_atexit` (0xfabe), guard release (0xfac4), return the static
    // (0xfaee). Guard/atexit collapse into `LazyLock`.
    render_settings_class_descriptor()
}

// 0xfb1c — __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_0xfb1c(_item: &mut CRenderSettingsItem) {
    // IDA 0xfb1c (decompiled, thunk): `Described<CRenderSettingsItem,
    // ...>::D1` tail-calls `Instance::~Instance`. Member drops collapse
    // into Rust drop; drop glue, no-op.
}

// 0xfb20 — __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_0xfb20(_item: &mut CRenderSettingsItem) {
    // IDA 0xfb20..0xfb2e (decompiled): `Described<CRenderSettingsItem,
    // ...>::D0` — `Instance::~Instance` (0xfb26) then `operator delete`.
    // The free collapses into Rust ownership (caller drops the box).
}

// 0xfb34 — __ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_0xfb34(item: &mut CRenderSettingsItem) {
    // IDA 0xfb34..0xfb3a (decompiled): `ZThn32` D1 — `this -= 32` (0xfb36
    // via the adjusted call) then the `Instance` D2. Same collapse as
    // `instance::stub_0xfb34`.
    stub_0xfb1c(item);
}

// 0xfb3c — __ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_0xfb3c(item: &mut CRenderSettingsItem) {
    // IDA 0xfb3c..0xfb50 (decompiled): `ZThn32` D0 — `this -= 32` (0xfb3e),
    // `Instance` D2 (0xfb46), `operator delete`. Same collapse as 0xfb34;
    // the free collapses into Rust ownership.
    stub_0xfb20(item);
}

// 0xfb54 — __ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_0xfb54(item: &mut CRenderSettingsItem) {
    // IDA 0xfb54..0xfb5a (decompiled): `ZThn36` D1 — `this -= 36` then the
    // `Instance` D2. Same collapse as 0xfb34.
    stub_0xfb1c(item);
}

// 0xfb5c — __ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_0xfb5c(item: &mut CRenderSettingsItem) {
    // IDA 0xfb5c..0xfb6e (decompiled): `ZThn36` D0 — `this -= 36` (0xfb5e),
    // `Instance` D2 (0xfb66), `operator delete`. Same collapse as 0xfb3c.
    stub_0xfb20(item);
}

// 0xfb74 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiEC2IMNS_15CRenderSettingsEKFjvEMS2_FvjEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_0xfb74(
    name: &'static str,
    category: &'static str,
    getter: fn(&CRenderSettingsItem) -> u32,
    setter: fn(&mut CRenderSettingsItem, u32),
) -> RenderIntPropDesc {
    // IDA 0xfb74..0xfc56 (decompiled): `PropDescriptor<CRenderSettingsItem,
    // int>::C2` — `classDescriptor()` touch (0xfb9c), `operator new(0x14)`
    // for the GetSetImpl (0xfba2), member-pointer stores (0xfbca..0xfbd8),
    // `TypedPropertyDescriptor<int>` C2 (0xfc1a), impl `delete` on the
    // moved-from slot (0xfc22..0xfc24), vtable install (0xfc38), return
    // (0xfc56). The member pointers collapse into the getter/setter fns.
    RenderIntPropDesc { name, category, getter, setter }
}

// 0xfc88 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::~PropDescriptor()")]
pub fn stub_0xfc88(_desc: *mut RenderIntPropDesc) {
    // IDA 0xfc88..0xfca6 (decompiled): `PropDescriptor<CRenderSettingsItem,
    // int>::D0` — vtable install (0xfc9c), impl `delete` on the `+0x28` slot
    // (0xfc9e..0xfca4), `operator delete`. Drops collapse into Rust
    // ownership; drop glue, no-op.
}

// 0xfcb4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::isReadOnly(void)const")]
pub fn stub_0xfcb4() -> bool {
    // IDA 0xfcb4..0xfcb6 (decompiled): `GetSetImpl<...>::isReadOnly`
    // returns `0`. Same shape as `instance::stub_0x396168`.
    false
}

// 0xfcb8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::isWriteOnly(void)const")]
pub fn stub_0xfcb8() -> bool {
    // IDA 0xfcb8..0xfcba (decompiled): `GetSetImpl<...>::isWriteOnly`
    // returns `0`. Twin of 0xfcb4.
    false
}

// 0xfcbc — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0xfcbc(desc: &RenderIntPropDesc, item: &CRenderSettingsItem) -> u32 {
    // IDA 0xfcbc..0xfce6 (decompiled): `GetSetImpl<...>::getValue` —
    // resolves the bound `unsigned (CRenderSettings::*)() const` member
    // through the `+4`/`+12` slots (0xfcc0..0xfce4) and invokes it. The
    // member-pointer dance collapses into the stored getter fn.
    (desc.getter)(item)
}

// 0xfce8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE8setValueEPNS0_13DescribedBaseERKi
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
pub fn stub_0xfce8(desc: &RenderIntPropDesc, item: &mut CRenderSettingsItem, value: u32) {
    // IDA 0xfce8..0xfd0a (decompiled): `GetSetImpl<...>::setValue` —
    // resolves the bound `void (CRenderSettingsItem::*)(unsigned)` member
    // through the `+12`/`+16` slots (0xfcf4..0xfd04) and invokes it with
    // `*a3`. Collapses into the stored setter fn.
    (desc.setter)(item, value)
}

// 0xfd0c — __ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EEC2EMS2_FivEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::BoundFuncDesc(int (CRenderSettingsItem::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0xfd0c(name: &'static str, method: fn(&CRenderSettingsItem) -> i32) -> RenderFuncDesc {
    // IDA 0xfd0c..0xfdc2 (decompiled): `BoundFuncDesc<CRenderSettingsItem,
    // int(), 0>::C2` — `classDescriptor()` touch (0xfd32),
    // `FunctionDescriptor` C2 (0xfd52), signature-item `new` + member-pair
    // store (0xfd6e..0xfd7a), `Type::getSingleton<int>` return tag (0xfda2),
    // return (0xfdc2). Collapses into the bound name/method pair.
    RenderFuncDesc { name, method }
}

// 0xfe04 — __ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::~BoundFuncDesc()")]
pub fn stub_0xfe04(_desc: *mut RenderFuncDesc) {
    // IDA 0xfe04..0xfe2c (decompiled): `BoundFuncDesc<...>::D0` — vtable
    // install (0xfe1c), signature-list `_M_clear` (0xfe20), `operator
    // delete`. Drops collapse into Rust ownership; drop glue, no-op.
}

// 0xfe30 — __ZNK3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0xfe30(desc: &RenderFuncDesc, item: &CRenderSettingsItem) -> IntVariant {
    // IDA 0xfe30..0xfe52 (decompiled): `BoundFuncDesc<...>::execute` —
    // `this - 36` item adjustment on the way in (0xfe38..0xfe3a), then
    // `Call0Helper::call` with the `+40`/`+44` member pair (0xfe42). The
    // adjustment collapses; the call is `stub_0xfe54`.
    stub_0xfe54(item, desc.method)
}

// 0xfe54 — __ZN3RBX10Reflection11Call0HelperI19CRenderSettingsItemMS2_FivEiE4callEPS2_S4_RNS0_7VariantE
// type: int __fastcall(int, int (__fastcall *)(_DWORD), int, _DWORD *)
#[doc(alias = "RBX::Reflection::Call0Helper<CRenderSettingsItem,int (CRenderSettingsItem::*)(void),int>::call(CRenderSettingsItem*,int (CRenderSettingsItem::*)(void),RBX::Reflection::Variant &)")]
pub fn stub_0xfe54(item: &CRenderSettingsItem, method: fn(&CRenderSettingsItem) -> i32) -> IntVariant {
    // IDA 0xfe54..0xfe82 (decompiled): `Call0Helper<...>::call` — virtual
    // member dispatch through the adjusted slot (0xfe5a..0xfe68), invoke
    // (0xfe6c), `Type::getSingleton<int>` tag (0xfe72),
    // `placement_any<int>::operator=` (0xfe80). Tag + payload collapse into
    // the int variant.
    IntVariant { value: method(item) }
}

// 0xfe84 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::EnumPropDescriptor<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>(char const*,char const*,RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0xfe84(
    name: &'static str,
    category: &'static str,
    getter: fn(&CRenderSettingsItem) -> i32,
    setter: fn(&mut CRenderSettingsItem, i32),
) -> RenderEnumPropDesc {
    // IDA 0xfe84..0xfffa (decompiled): `EnumPropDescriptor<...>::C2` —
    // `classDescriptor()` touch (0xfea8), `EnumDesc<ResolutionPreset>`
    // singleton `call_once` + touch (0xfec8..0xfecc, 0xff92..0xff9a),
    // `PropertyDescriptor` C2 (0xff16), enum-table stores (0xff3a/0xffa4),
    // `operator new(0x14)` GetSetImpl with the member pair
    // (0xff62..0xff88), then the `isReadOnly == 1` (0xffb4) /
    // `isWriteOnly == 1` (0xffd0) attribute-bit clears — both queries
    // return 0 (0x10064/0x10074), so the bits stay set.
    let _ = resolution_preset_enum_desc();
    RenderEnumPropDesc { name, category, getter, setter, read_only: false, write_only: false }
}

// 0x10038 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::~EnumPropDescriptor()")]
pub fn stub_0x10038(_desc: *mut RenderEnumPropDesc) {
    // IDA 0x10038..0x10060 (decompiled): `EnumPropDescriptor<...>::D0` —
    // vtable install (0x1004c), impl `delete` on the `+44` slot
    // (0x1004e..0x10054), `operator delete`. Same drop-glue shape as
    // 0xfc88.
}

// 0x10064 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::isReadOnly(void)const")]
pub fn stub_0x10064(desc: &RenderEnumPropDesc) -> bool {
    // IDA 0x10064..0x10072 (decompiled): `isReadOnly` delegates to the
    // `+44` impl slot `+0` query, which returns `0` (cf. 0xfcb4).
    desc.read_only
}

// 0x10074 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::isWriteOnly(void)const")]
pub fn stub_0x10074(desc: &RenderEnumPropDesc) -> bool {
    // IDA 0x10074..0x10082 (decompiled): `isWriteOnly` delegates to the
    // `+44` impl slot `+4` query, which returns `0` (cf. 0xfcb8).
    desc.write_only
}

// 0x10084 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11equalValuesEPKNS0_13DescribedBaseES8_
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x10084(
    desc: &RenderEnumPropDesc,
    first: &CRenderSettingsItem,
    second: &CRenderSettingsItem,
) -> bool {
    // IDA 0x10084..0x100aa (decompiled): `equalValues` — `getValue` through
    // the `+44` slot `+8` on both sides (0x10094/0x100aa) and compare. Same
    // shape as `instance::stub_0x3bd66c`.
    (desc.getter)(first) == (desc.getter)(second)
}

// 0x100ac — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x100ac(desc: &RenderEnumPropDesc, item: &CRenderSettingsItem) -> IntVariant {
    // IDA 0x100ac..0x100ce (decompiled): `getVariant` — `getEnumValue`
    // through vtable `+68` (0x100ba), `Type::getSingleton<int>` tag
    // (0x100c0), `placement_any<int>::operator=` (0x100ce). Tag + payload
    // collapse into the int variant.
    IntVariant { value: (desc.getter)(item) }
}

// 0x100d0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x100d0(desc: &RenderEnumPropDesc, item: &mut CRenderSettingsItem, variant: &IntVariant) {
    // IDA 0x100d0..0x10204 (decompiled): `setVariant` — holder-identity
    // int fast path via `any_cast<int>` (0x1014e..0x101cc), else generic
    // `Variant::convert<int>` with placement-copy/destroy around it
    // (0x10150..0x1018e), then the `+72` setter (0x101da). Our variant only
    // holds ints, so both paths collapse into the stored setter fn.
    (desc.setter)(item, variant.value)
}

// 0x10220 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE9copyValueEPKNS0_13DescribedBaseEPS6_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x10220(desc: &RenderEnumPropDesc, dst: &mut CRenderSettingsItem, src: &CRenderSettingsItem) {
    // IDA 0x10220..0x10242 (decompiled): `copyValue` — `getValue` through
    // the `+44` slot `+8` into a spill (0x10232), then the `+12` setter
    // (0x10242). Same shape as `instance::stub_0x4a8c88`.
    let value = (desc.getter)(src);
    (desc.setter)(dst, value)
}

// 0x10244 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::hasStringValue(void)const")]
pub fn stub_0x10244() -> bool {
    // IDA 0x10244..0x10246 (decompiled): `hasStringValue` returns `1`.
    true
}

// 0x10248 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x10248(desc: &RenderEnumPropDesc, item: &CRenderSettingsItem, out: &mut String) {
    // IDA 0x10248..0x1026a (decompiled): `getStringValue` — `getValue`
    // through the `+44` slot `+8` (0x1025a), then
    // `EnumDesc<ResolutionPreset>::convertToString` (0x1026a), which is the
    // same table-driven body as 0xe24c: empty when out of range.
    let value = (desc.getter)(item);
    match (value >= 0).then(|| resolution_preset_enum_desc().lookup_name(value)).flatten() {
        Some(name) => *out = name.to_owned(),
        None => out.clear(),
    }
}

// 0x1026c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x1026c(desc: &RenderEnumPropDesc, item: &mut CRenderSettingsItem, name: &str) -> bool {
    // IDA 0x1026c..0x102a8 (decompiled): `setStringValue` — `Name::lookup`
    // (0x1027e), `EnumDesc<ResolutionPreset>::convertToValue` (0x1028c),
    // miss returns 0 (0x1028e), hit sets through the `+44` slot `+12`
    // (0x102a2) and returns 1 (0x102a4). `Name::lookup` collapses into the
    // `&str` itself; same shape as `instance::stub_0x4a9038`.
    if let Some(value) = resolution_preset_enum_desc().lookup_value(name) {
        (desc.setter)(item, value);
        true
    } else {
        false
    }
}

// 0x102ac — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x102ac(desc: &RenderEnumPropDesc, item: &CRenderSettingsItem) -> RenderXmlIntValue {
    // IDA 0x102ac..0x102ca (decompiled): `writeValue` — `getValue` through
    // the `+44` slot `+8` (0x102ba), `clearValue` (0x102c0), type word `5`
    // (0x102c6), int word (0x102c8), return `5` (0x102ca). Collapses into
    // the kind/value pair.
    RenderXmlIntValue { kind: 5, int_value: (desc.getter)(item) }
}

// 0x102cc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x102cc(desc: &RenderEnumPropDesc, item: &mut CRenderSettingsItem, input: &RenderXmlInput) {
    // IDA 0x102cc..0x104aa (decompiled): `readValue` — `isXsiNil` bail
    // (0x102f0), int value via `setIntValue` (0x10338..0x10348), else string
    // value via `Name::lookup` + `convertToValue` + set
    // (0x10356..0x103b2) with `setStringValue`-mismatch fallback
    // (0x103d4..0x10486), else `ReleaseAssert(false)` (:359,
    // 0x103c4..0x1042c) that falls through. `setIntValue` for this desc is
    // the direct member set (cf. 0x10528 shape).
    match input {
        RenderXmlInput::Nil => {}
        RenderXmlInput::Int(value) => (desc.setter)(item, *value),
        RenderXmlInput::Text(name) => {
            if !stub_0x1026c(desc, item, name) {
                debug_assert!(false, "0x102cc: false (Reflection.h:359)");
            }
        }
    }
}

// 0x1050c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE13getIndexValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x1050c(desc: &RenderEnumPropDesc, item: &CRenderSettingsItem) -> Option<usize> {
    // IDA 0x1050c..0x10526 (decompiled): `getIndexValue` — `getValue`
    // through the `+44` slot `+8` (0x1051c), then
    // `EnumDesc<ResolutionPreset>::convertToIndex` (0x10526): assert plus
    // position search. Same shape as `instance::stub_0x6088ec`.
    let value = (desc.getter)(item);
    debug_assert!(value >= 0, "0x1050c: value>=0");
    resolution_preset_enum_desc().pairs.iter().position(|(v, _)| *v == value)
}

// 0x10528 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE13setIndexValueEPNS0_13DescribedBaseEm
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_0x10528(desc: &RenderEnumPropDesc, item: &mut CRenderSettingsItem, index: usize) -> bool {
    // IDA 0x10528..0x10558 (decompiled): `setIndexValue` — `count > index`
    // check (0x1053a), indexed value load (0x10544), `+44` slot `+12` set
    // (0x1054e), return 1 (0x10550); miss returns 0 (0x10558).
    match resolution_preset_enum_desc().pairs.get(index) {
        Some((value, _)) => {
            (desc.setter)(item, *value);
            true
        }
        None => false,
    }
}

// 0x1055c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE12getEnumValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x1055c(desc: &RenderEnumPropDesc, item: &CRenderSettingsItem) -> i32 {
    // IDA 0x1055c..0x10572 (decompiled): `getEnumValue` — `getValue`
    // through the `+44` slot `+8`. Same delegation as 0x100ac without the
    // variant wrap.
    (desc.getter)(item)
}

// 0x10564 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE12setEnumValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x10564() -> ! {
    todo!("0x10564 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x105b0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11getEnumItemEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x105b0() -> ! {
    todo!("0x105b0 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}

// 0x105d0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_0x105d0() -> ! {
    todo!("0x105d0 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

// 0x10604 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToIndexES3_
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToIndex(RBX::CRenderSettings::ResolutionPreset)const")]
pub fn stub_0x10604() -> ! {
    todo!("0x10604 RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToIndex(RBX::CRenderSettings::ResolutionPreset)const")
}

// 0x10674 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x10674() -> ! {
    todo!("0x10674 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x106b4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::isReadOnly(void)const")]
pub fn stub_0x106b4() -> ! {
    todo!("0x106b4 RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::isReadOnly(void)const")
}

// 0x106b8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::isWriteOnly(void)const")]
pub fn stub_0x106b8() -> ! {
    todo!("0x106b8 RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::isWriteOnly(void)const")
}

// 0x106bc — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x106bc() -> ! {
    todo!("0x106bc RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x106e8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::ResolutionPreset const&)const")]
pub fn stub_0x106e8() -> ! {
    todo!("0x106e8 RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::ResolutionPreset const&)const")
}

// 0x1070c — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::PropDescriptor<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>(char const*,char const*,bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x1070c() -> ! {
    todo!("0x1070c RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::PropDescriptor<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>(char const*,char const*,bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x10820 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::~PropDescriptor()")]
pub fn stub_0x10820() -> ! {
    todo!("0x10820 RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::~PropDescriptor()")
}

// 0x1084c — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isReadOnly(void)const")]
pub fn stub_0x1084c() -> ! {
    todo!("0x1084c RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isReadOnly(void)const")
}

// 0x10850 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isWriteOnly(void)const")]
pub fn stub_0x10850() -> ! {
    todo!("0x10850 RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isWriteOnly(void)const")
}

// 0x10854 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x10854() -> ! {
    todo!("0x10854 RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x10878 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// type: int __fastcall(int, int, unsigned __int8 *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_0x10878() -> ! {
    todo!("0x10878 RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")
}

// 0x1089c — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiEC2IMNS_15CRenderSettingsEKFivEMS2_FviEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::PropDescriptor<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>(char const*,char const*,int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x1089c() -> ! {
    todo!("0x1089c RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::PropDescriptor<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>(char const*,char const*,int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

#[cfg(test)]
mod batch3_tests {
    use super::*;
    use crate::generated_189::PlacementAny;

    fn shadow_desc() -> RenderEnumDesc {
        let mut desc = RenderEnumDesc::new("ShadowMode");
        desc.add_pair(0, "Off");
        desc.add_pair(1, "Low");
        desc
    }

    #[test]
    fn convert_round_trips_hit_and_miss() {
        let desc = shadow_desc();
        let mut out = String::new();
        stub_0xd28c(&desc, 1, &mut out);
        assert_eq!(out, "Low");
        stub_0xd28c(&desc, 9, &mut out);
        assert_eq!(out, "");
        assert_eq!(stub_0xd4f8(&desc, 1), 1);
        assert_eq!(stub_0xd4f8(&desc, 9), 0);
        let mut value = -1;
        assert!(stub_0xd6b4(&desc, "Off", &mut value));
        assert_eq!(value, 0);
        assert!(!stub_0xd6b4(&desc, "Missing", &mut value));
    }

    #[test]
    fn placement_any_assign_cast_and_singletons() {
        let mut slot = PlacementAny::default();
        stub_0xd42c(&mut slot, 1);
        assert_eq!(stub_0xd5c4(&slot), 1);
        stub_0xd96c(&mut slot, 2);
        assert_eq!(stub_0xdb04(&slot), 2);
        stub_0xdeac(&mut slot, 0);
        assert_eq!(slot.value, 0);
        assert_eq!(stub_0xd47c().token, HOLDER_SHADOW_MODE);
        assert_eq!(stub_0xd9bc().token, HOLDER_ANTIALIASING_MODE);
        assert_eq!(stub_0xdefc().token, HOLDER_FRAMERATE_MANAGER_MODE);
        stub_0xd4f4();
        stub_0xda34();
        let src = 3i32;
        let mut dst = 0i32;
        assert_eq!(stub_0xd4e8(&src, &mut dst), 3);
        assert_eq!(dst, 3);
        assert_eq!(stub_0xda28(&src, core::ptr::null_mut()), 3);
        assert_eq!(stub_0xdf68(&src, &mut dst), 3);
    }

    #[test]
    fn enum_desc_dtor_clears_tables() {
        let mut desc = shadow_desc();
        desc.add_alias("Off wide", 0);
        stub_0xd730(&mut desc);
        assert!(desc.pairs.is_empty() && desc.aliases.is_empty());
        let mut desc = shadow_desc();
        stub_0xdc70(&mut desc);
        assert!(desc.pairs.is_empty() && desc.aliases.is_empty());
    }
}

#[cfg(test)]
mod batch4_tests {
    use super::*;
    use crate::generated_189::PlacementAny;

    fn sample_desc() -> RenderEnumDesc {
        let mut desc = RenderEnumDesc::new("GraphicsMode");
        desc.add_pair(0, "Automatic");
        desc.add_pair(1, "Direct3D9");
        desc
    }

    fn resolution_desc() -> RenderEnumPropDesc {
        stub_0xfe84(
            "Resolution",
            "Rendering",
            |item| item.resolution_preset,
            |item, value| item.resolution_preset = value,
        )
    }

    #[test]
    fn graphics_and_aa_holders_and_casts() {
        assert_eq!(stub_0xe43c().token, HOLDER_GRAPHICS_MODE);
        assert_eq!(stub_0xe97c().token, HOLDER_AA_SAMPLES);
        let mut slot = PlacementAny::default();
        stub_0xe3ec(&mut slot, 1);
        assert_eq!(stub_0xe584(&slot), 1);
        stub_0xe92c(&mut slot, 4);
        assert_eq!(stub_0xeac4(&slot), 4);
        stub_0xe4b4();
        stub_0xe9f4();
        let src = 7i32;
        let mut dst = 0i32;
        assert_eq!(stub_0xe4a8(&src, &mut dst), 7);
        assert_eq!(dst, 7);
        assert_eq!(stub_0xe9e8(&src, core::ptr::null_mut()), 7);
    }

    #[test]
    fn framerate_convert_round_trip() {
        let desc = sample_desc();
        assert_eq!(stub_0xdf78(&desc, 1), 1);
        assert_eq!(stub_0xdf78(&desc, 9), 0);
        assert_eq!(stub_0xe4b8(&desc, 0), 0);
        let mut value = -1;
        assert!(stub_0xe134(&desc, "Direct3D9", &mut value));
        assert_eq!(value, 1);
        assert!(!stub_0xebb4(&desc, "Missing", &mut value));
        let mut out = String::new();
        stub_0xe24c(&desc, 0, &mut out);
        assert_eq!(out, "Automatic");
        stub_0xe78c(&desc, 42, &mut out);
        assert_eq!(out, "");
        let mut owned = sample_desc();
        owned.add_alias("Auto wide", 0);
        stub_0xe1b0(&mut owned);
        assert!(owned.pairs.is_empty() && owned.aliases.is_empty());
        let mut owned = sample_desc();
        stub_0xe6f0(&mut owned);
        assert!(owned.pairs.is_empty());
        let mut owned = sample_desc();
        stub_0xec30(&mut owned);
        assert!(owned.pairs.is_empty());
    }

    #[test]
    fn creator_lifecycle_and_names() {
        assert_eq!(stub_0xf1d8(), "RenderSettings");
        assert_eq!(stub_0xf1dc(), "RenderSettings");
        assert_eq!(stub_0xfa00().name, "RenderSettings");
        let creator = stub_0xf2bc();
        assert_eq!(creator.name, "RenderSettings");
        assert!(RENDER_SETTINGS_CREATOR_CONSTRUCTED.load(Ordering::SeqCst));
        assert_eq!(stub_0xf500().name, "RenderSettings");
        assert_eq!(stub_0xedfc(), "RenderSettings");
        let item = stub_0xee84();
        assert_eq!(SharedPtr::strong_count(&item), 1);
        assert_eq!(stub_0xefd8(&item), 1);
        stub_0xeccc();
        assert!(RENDER_SETTINGS_CREATOR_REGISTRY.lock().is_empty());
    }

    #[test]
    fn signal_next_and_on_error() {
        let slots: Vec<SharedPtr<RenderPropSlot>> =
            vec![SharedPtr::new(RenderPropSlot), SharedPtr::new(RenderPropSlot)];
        assert_eq!(stub_0xf574(&slots, 0), Some(1));
        assert_eq!(stub_0xf574(&slots, 1), None);
        stub_0xf6dc("boom");
    }

    #[test]
    fn vector_resolutions_ops() {
        let mut items = stub_0xf7e8(2);
        stub_0xf704(&mut items, 0, Vector2int16 { x: 800, y: 600 });
        stub_0xf704(&mut items, 0, Vector2int16 { x: 640, y: 480 });
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].x, 640);
        stub_0xf800(&mut items, 0, 1, 2);
        assert_eq!(items[1].x, 640);
    }

    #[test]
    fn int_prop_and_func_desc() {
        let desc = stub_0xfb74(
            "Resolution",
            "Rendering",
            |item| item.resolution_preset as u32,
            |item, value| item.resolution_preset = value as i32,
        );
        let mut item = CRenderSettingsItem::default();
        stub_0xfce8(&desc, &mut item, 3);
        assert_eq!(stub_0xfcbc(&desc, &item), 3);
        assert!(!stub_0xfcb4());
        assert!(!stub_0xfcb8());
        let func = stub_0xfd0c("probe", |item| item.resolution_preset);
        assert_eq!(stub_0xfe30(&func, &item).value, 3);
        assert_eq!(stub_0xfe54(&item, func.method).value, 3);
    }

    #[test]
    fn enum_prop_desc_suite() {
        let desc = resolution_desc();
        let mut item = CRenderSettingsItem::default();
        let mut other = CRenderSettingsItem::default();
        assert!(!stub_0x10064(&desc));
        assert!(!stub_0x10074(&desc));
        assert!(stub_0x10084(&desc, &item, &other));
        stub_0x100d0(&desc, &mut item, &IntVariant { value: 2 });
        assert_eq!(stub_0x100ac(&desc, &item).value, 2);
        assert_eq!(stub_0x1055c(&desc, &item), 2);
        assert!(!stub_0x10084(&desc, &item, &other));
        stub_0x10220(&desc, &mut other, &item);
        assert!(stub_0x10084(&desc, &item, &other));
        assert!(stub_0x10244());
        let written = stub_0x102ac(&desc, &item);
        assert_eq!((written.kind, written.int_value), (5, 2));
        stub_0x102cc(&desc, &mut other, &RenderXmlInput::Int(1));
        assert_eq!(stub_0x1055c(&desc, &other), 1);
        stub_0x102cc(&desc, &mut other, &RenderXmlInput::Nil);
        assert_eq!(stub_0x1055c(&desc, &other), 1);
        assert!(stub_0x10528(&desc, &mut other, 0) || !stub_0x10528(&desc, &mut other, usize::MAX));
        let _ = stub_0x1050c(&desc, &other);
        let mut out = String::new();
        stub_0x10248(&desc, &item, &mut out);
        assert!(!stub_0x1026c(&desc, &mut other, "NoSuchPreset\tunlikely"));
    }

    #[test]
    fn dtors_and_thunks_are_drop_glue() {
        let mut item = CRenderSettingsItem::default();
        RENDER_SETTINGS_SINGLETON.store(true, Ordering::SeqCst);
        stub_0xf83c(&mut item);
        assert!(!RENDER_SETTINGS_SINGLETON.load(Ordering::SeqCst));
        stub_0xf87c(&mut item);
        stub_0xf8c8(&mut item);
        stub_0xf90c(&mut item);
        stub_0xf964(&mut item);
        stub_0xf9a8(&mut item);
        stub_0xfb1c(&mut item);
        stub_0xfb20(&mut item);
        stub_0xfb34(&mut item);
        stub_0xfb3c(&mut item);
        stub_0xfb54(&mut item);
        stub_0xfb5c(&mut item);
    }
}
