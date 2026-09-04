// Auto-generated skeletons for rbx-datamodel -- from ida/export.json
// Filter: demangled contains RBX::Instance|RBX::DataModel|RBX::Workspace (exact RBX:: prefix), EA-sorted — filtered complete (10215/10215), global gap filler low-EA
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 120 stubs | range 0xcf3c..0x1089c | total filtered 10215, remaining 0 after batch; local 18280->18400 distinct, 67265->67145 not in datamodel (0 global missing)
// Shard: 190 EA-sorted asc next 120 low-EA global gap filler after 0xceec not yet in datamodel (filtered exhausted, 67265 missing before -> 67145 after)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use crate::generated_189::{HOLDER_ANTIALIASING_MODE, HOLDER_FRAMERATE_MANAGER_MODE, HOLDER_QUALITY_LEVEL, HOLDER_SHADOW_MODE, PlacementAny, RenderEnumDesc, antialiasing_mode_holder, framerate_manager_mode_holder, quality_level_holder, shadow_mode_holder};

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
pub fn stub_0xdf74() -> ! {
    todo!("0xdf74 rbx::implementation::typed_holder<RBX::CRenderSettings::FrameRateManagerMode>::destruct_func(char *)")
}

// 0xdf78 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToItem(RBX::CRenderSettings::FrameRateManagerMode const&)const")]
pub fn stub_0xdf78() -> ! {
    todo!("0xdf78 RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToItem(RBX::CRenderSettings::FrameRateManagerMode const&)const")
}

// 0xe044 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings20FrameRateManagerModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CRenderSettings::FrameRateManagerMode const& rbx::any_cast<RBX::CRenderSettings::FrameRateManagerMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0xe044() -> ! {
    todo!("0xe044 RBX::CRenderSettings::FrameRateManagerMode const& rbx::any_cast<RBX::CRenderSettings::FrameRateManagerMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0xe134 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::FrameRateManagerMode&)const")]
pub fn stub_0xe134() -> ! {
    todo!("0xe134 RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::FrameRateManagerMode&)const")
}

// 0xe1b0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::~EnumDesc()")]
pub fn stub_0xe1b0() -> ! {
    todo!("0xe1b0 RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::~EnumDesc()")
}

// 0xe24c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToString(RBX::CRenderSettings::GraphicsMode const&)const")]
pub fn stub_0xe24c() -> ! {
    todo!("0xe24c RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToString(RBX::CRenderSettings::GraphicsMode const&)const")
}

// 0xe3ec — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings12GraphicsModeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::GraphicsMode>(RBX::CRenderSettings::GraphicsMode const&)")]
pub fn stub_0xe3ec() -> ! {
    todo!("0xe3ec rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::GraphicsMode>(RBX::CRenderSettings::GraphicsMode const&)")
}

// 0xe43c — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12GraphicsModeEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::GraphicsMode>::singleton(void)")]
pub fn stub_0xe43c() -> ! {
    todo!("0xe43c rbx::implementation::typed_holder<RBX::CRenderSettings::GraphicsMode>::singleton(void)")
}

// 0xe4a8 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12GraphicsModeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::GraphicsMode>::construct_func(char const*,char *)")]
pub fn stub_0xe4a8() -> ! {
    todo!("0xe4a8 rbx::implementation::typed_holder<RBX::CRenderSettings::GraphicsMode>::construct_func(char const*,char *)")
}

// 0xe4b4 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12GraphicsModeEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::GraphicsMode>::destruct_func(char *)")]
pub fn stub_0xe4b4() -> ! {
    todo!("0xe4b4 rbx::implementation::typed_holder<RBX::CRenderSettings::GraphicsMode>::destruct_func(char *)")
}

// 0xe4b8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToItem(RBX::CRenderSettings::GraphicsMode const&)const")]
pub fn stub_0xe4b8() -> ! {
    todo!("0xe4b8 RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToItem(RBX::CRenderSettings::GraphicsMode const&)const")
}

// 0xe584 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings12GraphicsModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CRenderSettings::GraphicsMode const& rbx::any_cast<RBX::CRenderSettings::GraphicsMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0xe584() -> ! {
    todo!("0xe584 RBX::CRenderSettings::GraphicsMode const& rbx::any_cast<RBX::CRenderSettings::GraphicsMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0xe674 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::GraphicsMode&)const")]
pub fn stub_0xe674() -> ! {
    todo!("0xe674 RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::GraphicsMode&)const")
}

// 0xe6f0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()")]
pub fn stub_0xe6f0() -> ! {
    todo!("0xe6f0 RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()")
}

// 0xe78c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToString(RBX::CRenderSettings::AASamples const&)const")]
pub fn stub_0xe78c() -> ! {
    todo!("0xe78c RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToString(RBX::CRenderSettings::AASamples const&)const")
}

// 0xe92c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings9AASamplesEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::AASamples>(RBX::CRenderSettings::AASamples const&)")]
pub fn stub_0xe92c() -> ! {
    todo!("0xe92c rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::AASamples>(RBX::CRenderSettings::AASamples const&)")
}

// 0xe97c — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings9AASamplesEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::singleton(void)")]
pub fn stub_0xe97c() -> ! {
    todo!("0xe97c rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::singleton(void)")
}

// 0xe9e8 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings9AASamplesEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::construct_func(char const*,char *)")]
pub fn stub_0xe9e8() -> ! {
    todo!("0xe9e8 rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::construct_func(char const*,char *)")
}

// 0xe9f4 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings9AASamplesEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::destruct_func(char *)")]
pub fn stub_0xe9f4() -> ! {
    todo!("0xe9f4 rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::destruct_func(char *)")
}

// 0xe9f8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToItem(RBX::CRenderSettings::AASamples const&)const")]
pub fn stub_0xe9f8() -> ! {
    todo!("0xe9f8 RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToItem(RBX::CRenderSettings::AASamples const&)const")
}

// 0xeac4 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings9AASamplesENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CRenderSettings::AASamples const& rbx::any_cast<RBX::CRenderSettings::AASamples const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0xeac4() -> ! {
    todo!("0xeac4 RBX::CRenderSettings::AASamples const& rbx::any_cast<RBX::CRenderSettings::AASamples const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0xebb4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToValue(RBX::Name const&,RBX::CRenderSettings::AASamples&)const")]
pub fn stub_0xebb4() -> ! {
    todo!("0xebb4 RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToValue(RBX::Name const&,RBX::CRenderSettings::AASamples&)const")
}

// 0xec30 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()")]
pub fn stub_0xec30() -> ! {
    todo!("0xec30 RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()")
}

// 0xeccc — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD2Ev
// type: int __fastcall(int)
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0xeccc() -> ! {
    todo!("0xeccc __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD2Ev")
}

// 0xedfc — __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0xedfc() -> ! {
    todo!("0xedfc __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator12getClassNameEv")
}

// 0xee84 — __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator6createEv
// type: int __fastcall(int *)
#[doc(alias = "__ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator6createEv")]
pub fn stub_0xee84() -> ! {
    todo!("0xee84 __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator6createEv")
}

// 0xefd8 — __ZNK5boost6detail15sp_counted_base9use_countEv
// type: int __fastcall(boost::detail::sp_counted_base *this)
#[doc(alias = "boost::detail::sp_counted_base::use_count(void)const")]
pub fn stub_0xefd8() -> ! {
    todo!("0xefd8 boost::detail::sp_counted_base::use_count(void)const")
}

// 0xf1d8 — __ZN3RBX4Name13callDoDeclareILZ15sRenderSettingsEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZ15sRenderSettingsEEEvv")]
pub fn stub_0xf1d8() -> ! {
    todo!("0xf1d8 __ZN3RBX4Name13callDoDeclareILZ15sRenderSettingsEEEvv")
}

// 0xf1dc — __ZN3RBX4Name9doDeclareILZ15sRenderSettingsEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZ15sRenderSettingsEEERKS0_v")]
pub fn stub_0xf1dc() -> ! {
    todo!("0xf1dc __ZN3RBX4Name9doDeclareILZ15sRenderSettingsEEERKS0_v")
}

// 0xf2bc — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0xf2bc() -> ! {
    todo!("0xf2bc __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorC2Ev")
}

// 0xf500 — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0xf500() -> ! {
    todo!("0xf500 __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE17static_getCreatorEv")
}

// 0xf574 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4nextERN5boost13intrusive_ptrINS8_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot> &)")]
// was: rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot> &)
pub fn stub_0xf574() -> ! {
    todo!("0xf574 rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot> &)")
}

// 0xf6dc — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE8on_errorERSt9exception
// type: int *()
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::on_error(std::exception &)")]
pub fn stub_0xf6dc() -> ! {
    todo!("0xf6dc rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::on_error(std::exception &)")
}

// 0xf704 — __ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int __fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_insert_aux(__gnu_cxx::__normal_iterator<G3D::Vector2int16*,std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>>,G3D::Vector2int16 const&)")]
pub fn stub_0xf704() -> ! {
    todo!("0xf704 std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_insert_aux(__gnu_cxx::__normal_iterator<G3D::Vector2int16*,std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>>,G3D::Vector2int16 const&)")
}

// 0xf7e8 — __ZNSt12_Vector_baseIN3G3D12Vector2int16ESaIS1_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_allocate(unsigned long)")]
pub fn stub_0xf7e8() -> ! {
    todo!("0xf7e8 std::_Vector_base<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_allocate(unsigned long)")
}

// 0xf800 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D12Vector2int16ES5_EET0_T_S7_S6_
// type: int __fastcall(int, int, int)
#[doc(alias = "G3D::Vector2int16 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector2int16 *,G3D::Vector2int16 *>(G3D::Vector2int16 *,G3D::Vector2int16 *,G3D::Vector2int16 *)")]
pub fn stub_0xf800() -> ! {
    todo!("0xf800 G3D::Vector2int16 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector2int16 *,G3D::Vector2int16 *>(G3D::Vector2int16 *,G3D::Vector2int16 *,G3D::Vector2int16 *)")
}

// 0xf83c — __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev")]
pub fn stub_0xf83c() -> ! {
    todo!("0xf83c __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev")
}

// 0xf87c — __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev")]
pub fn stub_0xf87c() -> ! {
    todo!("0xf87c __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev")
}

// 0xf8c8 — __ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev
// type: void __fastcall(_QWORD *)
#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev")]
pub fn stub_0xf8c8() -> ! {
    todo!("0xf8c8 __ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev")
}

// 0xf90c — __ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev
// type: int __fastcall(_QWORD *)
#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev")]
pub fn stub_0xf90c() -> ! {
    todo!("0xf90c __ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev")
}

// 0xf964 — __ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev")]
pub fn stub_0xf964() -> ! {
    todo!("0xf964 __ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev")
}

// 0xf9a8 — __ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev")]
pub fn stub_0xf9a8() -> ! {
    todo!("0xf9a8 __ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev")
}

// 0xfa00 — __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0xfa00() -> ! {
    todo!("0xfa00 __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}

// 0xfb1c — __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0xfb1c() -> ! {
    todo!("0xfb1c __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0xfb20 — __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: int __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0xfb20() -> ! {
    todo!("0xfb20 __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0xfb34 — __ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0xfb34() -> ! {
    todo!("0xfb34 __ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0xfb3c — __ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0xfb3c() -> ! {
    todo!("0xfb3c __ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0xfb54 — __ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0xfb54() -> ! {
    todo!("0xfb54 __ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0xfb5c — __ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0xfb5c() -> ! {
    todo!("0xfb5c __ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0xfb74 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiEC2IMNS_15CRenderSettingsEKFjvEMS2_FvjEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::PropDescriptor<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>(char const*,char const*,unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0xfb74() -> ! {
    todo!("0xfb74 RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::PropDescriptor<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>(char const*,char const*,unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xfc88 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::~PropDescriptor()")]
pub fn stub_0xfc88() -> ! {
    todo!("0xfc88 RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::~PropDescriptor()")
}

// 0xfcb4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::isReadOnly(void)const")]
pub fn stub_0xfcb4() -> ! {
    todo!("0xfcb4 RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::isReadOnly(void)const")
}

// 0xfcb8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::isWriteOnly(void)const")]
pub fn stub_0xfcb8() -> ! {
    todo!("0xfcb8 RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::isWriteOnly(void)const")
}

// 0xfcbc — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0xfcbc() -> ! {
    todo!("0xfcbc RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0xfce8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE8setValueEPNS0_13DescribedBaseERKi
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
pub fn stub_0xfce8() -> ! {
    todo!("0xfce8 RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")
}

// 0xfd0c — __ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EEC2EMS2_FivEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::BoundFuncDesc(int (CRenderSettingsItem::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0xfd0c() -> ! {
    todo!("0xfd0c RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::BoundFuncDesc(int (CRenderSettingsItem::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xfe04 — __ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::~BoundFuncDesc()")]
pub fn stub_0xfe04() -> ! {
    todo!("0xfe04 RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::~BoundFuncDesc()")
}

// 0xfe30 — __ZNK3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0xfe30() -> ! {
    todo!("0xfe30 RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0xfe54 — __ZN3RBX10Reflection11Call0HelperI19CRenderSettingsItemMS2_FivEiE4callEPS2_S4_RNS0_7VariantE
// type: int __fastcall(int, int (__fastcall *)(_DWORD), int, _DWORD *)
#[doc(alias = "RBX::Reflection::Call0Helper<CRenderSettingsItem,int (CRenderSettingsItem::*)(void),int>::call(CRenderSettingsItem*,int (CRenderSettingsItem::*)(void),RBX::Reflection::Variant &)")]
pub fn stub_0xfe54() -> ! {
    todo!("0xfe54 RBX::Reflection::Call0Helper<CRenderSettingsItem,int (CRenderSettingsItem::*)(void),int>::call(CRenderSettingsItem*,int (CRenderSettingsItem::*)(void),RBX::Reflection::Variant &)")
}

// 0xfe84 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::EnumPropDescriptor<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>(char const*,char const*,RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0xfe84() -> ! {
    todo!("0xfe84 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::EnumPropDescriptor<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>(char const*,char const*,RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x10038 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::~EnumPropDescriptor()")]
pub fn stub_0x10038() -> ! {
    todo!("0x10038 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::~EnumPropDescriptor()")
}

// 0x10064 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::isReadOnly(void)const")]
pub fn stub_0x10064() -> ! {
    todo!("0x10064 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::isReadOnly(void)const")
}

// 0x10074 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::isWriteOnly(void)const")]
pub fn stub_0x10074() -> ! {
    todo!("0x10074 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::isWriteOnly(void)const")
}

// 0x10084 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11equalValuesEPKNS0_13DescribedBaseES8_
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x10084() -> ! {
    todo!("0x10084 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x100ac — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x100ac() -> ! {
    todo!("0x100ac RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x100d0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x100d0() -> ! {
    todo!("0x100d0 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x10220 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE9copyValueEPKNS0_13DescribedBaseEPS6_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x10220() -> ! {
    todo!("0x10220 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x10244 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::hasStringValue(void)const")]
pub fn stub_0x10244() -> ! {
    todo!("0x10244 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::hasStringValue(void)const")
}

// 0x10248 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x10248() -> ! {
    todo!("0x10248 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x1026c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x1026c() -> ! {
    todo!("0x1026c RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x102ac — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x102ac() -> ! {
    todo!("0x102ac RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x102cc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x102cc() -> ! {
    todo!("0x102cc RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x1050c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE13getIndexValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x1050c() -> ! {
    todo!("0x1050c RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x10528 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE13setIndexValueEPNS0_13DescribedBaseEm
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_0x10528() -> ! {
    todo!("0x10528 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")
}

// 0x1055c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE12getEnumValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x1055c() -> ! {
    todo!("0x1055c RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
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
