//! platform generated_172 — next 100 stubs EA-sorted asc global filler continuation
//! Filter: platform/iOS/Apple strict (0 remaining — ObjC 2763 done, RobloxView 160 done, iOSSettingsService 54 done, RBX::Platform 0) + global EA-sorted asc filler (rbx_core::SharedPtr not boost)
//! Batch: 100 stubs EA-sorted asc | skeleton batch | range 0xc154..0xf704 (rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use rbx_core::shared_ptr::{ControlBlockPd, CreatableInstanceDeleter};
use rbx_core::signal::Signal;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use super::generated_171::{RenderEnumDesc, RenderSettingsItem};

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};
/// Host model of `rbx::placement_any<RBX::Region3>` as used for the
/// `CRenderSettings` enum payloads (IDA 0xc90c/0xceec family). The holder
/// vtable pointer (IDA `a1[0]`, compared against each
/// `typed_holder<T>::singleton` at 0xc92c/0xcf0c) folds into `tag`; the
/// inline payload (IDA `a1[1]`) is the enum value. `tag == 0` is the empty
/// (`void`-holder, IDA 0xcafc/0xcb00) state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionAny {
    pub tag: u32,
    pub value: i32,
}
pub const REGION_ANY_EMPTY: u32 = 0;
pub const REGION_ANY_RESOLUTION_PRESET: u32 = 1;
pub const REGION_ANY_QUALITY_LEVEL: u32 = 2;
pub const REGION_ANY_SHADOW_MODE: u32 = 3;
pub const REGION_ANY_ANTIALIASING_MODE: u32 = 4;
pub const REGION_ANY_FRAME_RATE_MANAGER_MODE: u32 = 5;
pub const REGION_ANY_GRAPHICS_MODE: u32 = 6;
pub const REGION_ANY_AA_SAMPLES: u32 = 7;
/// `EnumRegistrar<ResolutionPreset>::registrar` bumped by the D2 at 0xccd4.
/// Siblings below bump their own registrar on D2 (IDA 0xd1f0/0xd730/0xdc70/
/// 0xe1b0/0xe6f0/0xec30); the host folds each registrar into a counter.
static ENUM_REGISTRAR_RESOLUTION_PRESET: AtomicU32 = AtomicU32::new(0);
static ENUM_REGISTRAR_QUALITY_LEVEL: AtomicU32 = AtomicU32::new(0);
static ENUM_REGISTRAR_SHADOW_MODE: AtomicU32 = AtomicU32::new(0);
static ENUM_REGISTRAR_ANTIALIASING_MODE: AtomicU32 = AtomicU32::new(0);
static ENUM_REGISTRAR_FRAME_RATE_MANAGER_MODE: AtomicU32 = AtomicU32::new(0);
static ENUM_REGISTRAR_GRAPHICS_MODE: AtomicU32 = AtomicU32::new(0);
static ENUM_REGISTRAR_AA_SAMPLES: AtomicU32 = AtomicU32::new(0);

/// `FactoryProduct<CRenderSettingsItem,...>::Creator::isConstructed` (IDA
/// `isConstructedE == 666` checks at 0xeccc/0xee84/0xf500, set by the C2 at
/// 0xf2bc). Host process flag; the `creatorPrivate` singleton itself is
/// folded into this bit.
static CREATOR_IS_CONSTRUCTED: AtomicBool = AtomicBool::new(false);

// 0xc154 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED1Ev
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::~EnumDesc()")]
pub fn stub_c154(desc: *mut RenderEnumDesc) {
    // IDA 0xc154: thunk to `EnumDesc<ShadowMode>::D2` (non-deleting). Same
    // shape as the AASamples twin at 0xb934 (`generated_171`).
    if !desc.is_null() {
        unsafe {
            (*desc).pairs.clear();
            (*desc).legacy_aliases.clear();
        }
    }
}

// 0xc158 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED0Ev
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::~EnumDesc()")]
pub fn stub_c158(desc: *mut RenderEnumDesc) {
    // IDA 0xc158..0xc16a: `D2(a1)` (0xc15e) then `operator delete(a1)` (D0).
    // Same shape as the AASamples twin at 0xb938 (`generated_171`).
    if desc.is_null() {
        return;
    }
    unsafe {
        (*desc).pairs.clear();
        (*desc).legacy_aliases.clear();
        drop(Box::from_raw(desc));
    }
}

// 0xc16c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE6lookupEPKc
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::lookup(char const*)const")]
pub fn stub_c16c(desc: &RenderEnumDesc, name: &str) -> Option<i32> {
    // IDA 0xc176..0xc192: `Name::lookup` (0xc178), `convertToValue` (0xc186),
    // hit returns `convertToItem` (0xc192), else null. Same as 0xb94c.
    desc.pairs.iter().find(|p| p.name == name).map(|p| p.value).or_else(|| {
        desc.legacy_aliases.iter().find(|a| a.name == name).map(|a| a.maps_to)
    })
}

// 0xc19c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE6lookupERKNS0_7VariantE
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_c19c(desc: &RenderEnumDesc, value: i32) -> Option<i32> {
    // IDA 0xc1ae..0xc1b8: `any_cast<ShadowMode const&>` (0xc1ae) then
    // `convertToItem` (0xc1b8). Same shape as the twin at 0xb97c.
    desc.pairs.iter().find(|p| p.value == value).map(|p| p.value)
}

// 0xc1bc — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToValueEmRNS0_7VariantE
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_c1bc(desc: &RenderEnumDesc, value: u32, out: &mut i32) -> bool {
    // IDA 0xc1bc..0xc1d6: `count = [R0,#0x28]`, `count > value` loads
    // `table[value]` (0xc1cc..0xc1d0), stores to out (0xc1d4), returns 1
    // (0xc1d6); miss returns 0. Same shape as 0xb99c.
    match desc.pairs.iter().find(|p| p.value == value as i32) {
        Some(p) => {
            *out = p.value;
            true
        }
        None => false,
    }
}

// 0xc218 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE15convertToStringEmRSs
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToString(unsigned long,std::string &)const")]
pub fn stub_c218(desc: &RenderEnumDesc, value: u32, out: &mut String) -> bool {
    // IDA 0xc218: same body as the AASamples `convertToString` at 0xb9f8 —
    // out-of-range returns 0 with `out` untouched, else assign + return 1.
    match desc.pairs.iter().find(|p| p.value == value as i32) {
        Some(p) => {
            out.clear();
            out.push_str(&p.name);
            true
        }
        None => false,
    }
}

// 0xc35c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED1Ev
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::~EnumDesc()")]
pub fn stub_c35c(desc: *mut RenderEnumDesc) {
    // IDA 0xc35c: thunk to `EnumDesc<QualityLevel>::D2` (non-deleting). Same
    // shape as the AASamples twin at 0xb934 (`generated_171`).
    if !desc.is_null() {
        unsafe {
            (*desc).pairs.clear();
            (*desc).legacy_aliases.clear();
        }
    }
}

// 0xc360 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED0Ev
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::~EnumDesc()")]
pub fn stub_c360(desc: *mut RenderEnumDesc) {
    // IDA 0xc360..0xc372: `D2(a1)` (0xc366) then `operator delete(a1)` (D0).
    // Same shape as the AASamples twin at 0xb938 (`generated_171`).
    if desc.is_null() {
        return;
    }
    unsafe {
        (*desc).pairs.clear();
        (*desc).legacy_aliases.clear();
        drop(Box::from_raw(desc));
    }
}

// 0xc374 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE6lookupEPKc
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::lookup(char const*)const")]
pub fn stub_c374(desc: &RenderEnumDesc, name: &str) -> Option<i32> {
    // IDA 0xc37e..0xc39a: `Name::lookup` (0xc380), `convertToValue` (0xc38e),
    // hit returns `convertToItem` (0xc39a), else null. Same as 0xb94c.
    desc.pairs.iter().find(|p| p.name == name).map(|p| p.value).or_else(|| {
        desc.legacy_aliases.iter().find(|a| a.name == name).map(|a| a.maps_to)
    })
}

// 0xc3a4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE6lookupERKNS0_7VariantE
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_c3a4(desc: &RenderEnumDesc, value: i32) -> Option<i32> {
    // IDA 0xc3b6..0xc3c0: `any_cast<QualityLevel const&>` (0xc3b6) then
    // `convertToItem` (0xc3c0). Same shape as the twin at 0xb97c.
    desc.pairs.iter().find(|p| p.value == value).map(|p| p.value)
}

// 0xc3c4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToValueEmRNS0_7VariantE
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_c3c4(desc: &RenderEnumDesc, value: u32, out: &mut i32) -> bool {
    // IDA 0xc3c4..0xc3de: `count = [R0,#0x28]`, `count > value` loads
    // `table[value]`, stores to out, returns 1; miss returns 0 — identical to
    // the verified 0xb99c body. Same as 0xb99c.
    match desc.pairs.iter().find(|p| p.value == value as i32) {
        Some(p) => {
            *out = p.value;
            true
        }
        None => false,
    }
}

// 0xc420 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE15convertToStringEmRSs
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToString(unsigned long,std::string &)const")]
pub fn stub_c420(desc: &RenderEnumDesc, value: u32, out: &mut String) -> bool {
    // IDA 0xc420: same body as the AASamples `convertToString` at 0xb9f8 —
    // out-of-range returns 0 with `out` untouched, else assign + return 1.
    match desc.pairs.iter().find(|p| p.value == value as i32) {
        Some(p) => {
            out.clear();
            out.push_str(&p.name);
            true
        }
        None => false,
    }
}

// 0xc564 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED1Ev
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::~EnumDesc()")]
pub fn stub_c564(desc: *mut RenderEnumDesc) {
    // IDA 0xc564: thunk to `EnumDesc<ResolutionPreset>::D2` (non-deleting).
    // Same shape as the AASamples twin at 0xb934 (`generated_171`).
    if !desc.is_null() {
        unsafe {
            (*desc).pairs.clear();
            (*desc).legacy_aliases.clear();
        }
    }
}

// 0xc568 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED0Ev
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::~EnumDesc()")]
pub fn stub_c568(desc: *mut RenderEnumDesc) {
    // IDA 0xc568..0xc57a: `D2(a1)` (0xc56e) then `operator delete(a1)` (D0).
    // Same shape as the AASamples twin at 0xb938 (`generated_171`).
    if desc.is_null() {
        return;
    }
    unsafe {
        (*desc).pairs.clear();
        (*desc).legacy_aliases.clear();
        drop(Box::from_raw(desc));
    }
}

// 0xc57c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE6lookupEPKc
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::lookup(char const*)const")]
pub fn stub_c57c(desc: &RenderEnumDesc, name: &str) -> Option<i32> {
    // IDA 0xc586..0xc5a2: `Name::lookup` (0xc588), `convertToValue` (0xc596),
    // hit returns `convertToItem` (0xc5a2), else null. Same as 0xb94c.
    desc.pairs.iter().find(|p| p.name == name).map(|p| p.value).or_else(|| {
        desc.legacy_aliases.iter().find(|a| a.name == name).map(|a| a.maps_to)
    })
}

// 0xc5ac — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE6lookupERKNS0_7VariantE
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_c5ac(desc: &RenderEnumDesc, value: i32) -> Option<i32> {
    // IDA 0xc5be..0xc5c8: `any_cast<ResolutionPreset const&>` (0xc5be) then
    // `convertToItem` (0xc5c8). Same shape as the twin at 0xb97c.
    desc.pairs.iter().find(|p| p.value == value).map(|p| p.value)
}

// 0xc5cc — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToValueEmRNS0_7VariantE
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_c5cc(desc: &RenderEnumDesc, value: u32, out: &mut i32) -> bool {
    // IDA 0xc5cc..0xc5e6: `count = [R0,#0x28]`, `count > value` loads
    // `table[value]`, stores to out, returns 1; miss returns 0 — identical to
    // the verified 0xb99c body. Same as 0xb99c.
    match desc.pairs.iter().find(|p| p.value == value as i32) {
        Some(p) => {
            *out = p.value;
            true
        }
        None => false,
    }
}

// 0xc628 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE15convertToStringEmRSs
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToString(unsigned long,std::string &)const")]
pub fn stub_c628(desc: &RenderEnumDesc, value: u32, out: &mut String) -> bool {
    // IDA 0xc628: same body as the AASamples `convertToString` at 0xb9f8 —
    // out-of-range returns 0 with `out` untouched, else assign + return 1.
    match desc.pairs.iter().find(|p| p.value == value as i32) {
        Some(p) => {
            out.clear();
            out.push_str(&p.name);
            true
        }
        None => false,
    }
}

// 0xc76c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE15convertToStringERKS3_
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToString(RBX::CRenderSettings::ResolutionPreset const&)const")]
pub fn stub_c76c(desc: &RenderEnumDesc, value: i32, out: &mut String) {
    // IDA 0xc7c6..0xc872: `ReleaseAssert(value>=0)` (enumconverter.h:262) and
    // `ReleaseAssert((size_t)value<enumToItem.size())` (:263, via `_debugHook`
    // or plain assert); 0xc876..0xc8ac: in-range assigns the name at
    // `[+0x6C][value]` (copy-ctor into `out`), out-of-range assigns `""`.
    // Void function — `out` is always assigned.
    debug_assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    out.clear();
    if let Some(p) = desc.pairs.iter().find(|p| p.value == value) {
        out.push_str(&p.name);
    }
}

// 0xc90c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings16ResolutionPresetEEERS3_RKT_
// mangled: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings16ResolutionPresetEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::ResolutionPreset>(RBX::CRenderSettings::ResolutionPreset const&)")]
pub fn stub_c90c(any: &mut RegionAny, value: i32) {
    // IDA 0xc918: `typed_holder<ResolutionPreset>::singleton()` (init);
    // 0xc92c: same-holder fast path `a1[1] = *a2` (0xc944); else 0xc930: live
    // old holder → `destruct` (0xc938, nop for enum payloads — cf. 0xc9d4)
    // + null (0xc93c), then store value + holder (0xc94e..). Host: tag + store.
    any.tag = REGION_ANY_RESOLUTION_PRESET;
    any.value = value;
}

// 0xc95c — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16ResolutionPresetEE9singletonEv
// mangled: __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16ResolutionPresetEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ResolutionPreset>::singleton(void)")]
pub fn stub_c95c() -> u32 {
    // IDA 0xc95c..0xc9c6: `__cxa_guard_acquire` one-time init (0xc976) of the
    // holder `s = {typeinfo (0xc9ae), destruct_func, ...}`; returns `&s`. The
    // host folds the holder address into its tag (cf. `RegionAny`).
    REGION_ANY_RESOLUTION_PRESET
}

// 0xc9c8 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16ResolutionPresetEE14construct_funcEPKcPc
// mangled: __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16ResolutionPresetEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ResolutionPreset>::construct_func(char const*,char *)")]
pub fn stub_c9c8(src: RegionAny, dst: *mut RegionAny) -> RegionAny {
    // IDA 0xc9c8..0xc9d0: `if (a2) { result = *result; *a2 = result; }` —
    // copy-construct the trivial payload into the buffer; return the value.
    if !dst.is_null() {
        unsafe {
            *dst = src;
        }
    }
    src
}

// 0xc9d4 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16ResolutionPresetEE13destruct_funcEPc
// mangled: __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16ResolutionPresetEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ResolutionPreset>::destruct_func(char *)")]
pub fn stub_c9d4() {
    // IDA 0xc9d4: empty body — `typed_holder<ResolutionPreset>::destruct_func`
    // is a no-op (trivial enum payload).
}

// 0xc9d8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE13convertToItemERKS3_
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToItem(RBX::CRenderSettings::ResolutionPreset const&)const")]
pub fn stub_c9d8(desc: &RenderEnumDesc, value: i32) -> Option<i32> {
    // IDA 0xc9ec..0xca32: `ReleaseAssert(value>=0)` (enumconverter.h:273);
    // 0xca36..0xca80: `ReleaseAssert((size_t)value<enumToItem.size())` (:274);
    // 0xca84..0xca9c: `value < 0` → null, `value < size` → `table[value]`,
    // else null. Host returns the value where the original returns the item.
    debug_assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    desc.pairs.iter().find(|p| p.value == value).map(|p| p.value)
}

// 0xcaa4 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings16ResolutionPresetENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// mangled: __ZN3rbx8any_castIRKN3RBX15CRenderSettings16ResolutionPresetENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CRenderSettings::ResolutionPreset const& rbx::any_cast<RBX::CRenderSettings::ResolutionPreset const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_caa4(any: &RegionAny) -> i32 {
    // IDA 0xcaee..0xcb2e: holder typeinfo vs `typeinfo ResolutionPreset`
    // (0xcb02..0xcb0e), empty holder reads as `void` (0xcb00); match returns
    // the payload (`a1 + 4`, 0xcb34), mismatch throws
    // `rbx::bad_placement_any_cast` (a `std::bad_cast`, 0xcb4c..0xcb62).
    if any.tag != REGION_ANY_RESOLUTION_PRESET {
        panic!("rbx::bad_placement_any_cast");
    }
    any.value
}

// 0xcb94 — __ZN5boost16exception_detail12refcount_ptrINS0_20error_info_containerEED2Ev
// mangled: __ZN5boost16exception_detail12refcount_ptrINS0_20error_info_containerEED2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "boost::exception_detail::refcount_ptr<boost::exception_detail::error_info_container>::~refcount_ptr()")]
pub fn stub_cb94(_held: Option<SharedPtr<u8>>) {
    // IDA 0xcb94..0xcc14: `~refcount_ptr`: `*a1 && release(*a1) == 1` nulls the
    // slot. Host `SharedPtr` (Arc) drop glue runs the release — drop the held
    // ref (rbx-core `boost_skeletons` carrier precedent).
}

// 0xcc34 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToValueERKNS_4NameERS3_
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToValue(RBX::Name const&,RBX::CRenderSettings::ResolutionPreset&)const")]
pub fn stub_cc34(desc: &RenderEnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xcc4a..0xcc6e: `lower_bound` over the primary name map (+0x34);
    // 0xcc78..0xcca4: same over the legacy map (+0x4C); exact hit stores
    // `node[0x14]` to out and returns 1, else 0. Host searches both tables
    // (interned `Name` keys compare as their strings).
    match desc.pairs.iter().find(|p| p.name == name).map(|p| p.value).or_else(|| {
        desc.legacy_aliases.iter().find(|a| a.name == name).map(|a| a.maps_to)
    }) {
        Some(v) => {
            *out = v;
            true
        }
        None => false,
    }
}

// 0xccb0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED2Ev
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::~EnumDesc()")]
pub fn stub_ccb0(desc: *mut RenderEnumDesc) {
    // IDA 0xccce..0xcd48 (`EnumDesc<ResolutionPreset>::D2`): vtable reset
    // (host nop), `++EnumRegistrar::registrar` (0xccd4), item-ptr range
    // destroy (0xccdc..0xccec), heap-array deletes (0xccee..0xcd18),
    // `vector<string>` dtor (0xcd1c..0xcd20), both RB-tree erases
    // (0xcd2e..0xcd3e), tail-call `EnumDescriptor::D2`. Host clears both tables.
    if desc.is_null() {
        return;
    }
    unsafe {
        (*desc).pairs.clear();
        (*desc).legacy_aliases.clear();
    }
    ENUM_REGISTRAR_RESOLUTION_PRESET.fetch_add(1, Ordering::SeqCst);
}

// 0xcd4c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE15convertToStringERKS3_
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToString(RBX::CRenderSettings::QualityLevel const&)const")]
pub fn stub_cd4c(desc: &RenderEnumDesc, value: i32, out: &mut String) {
    // IDA 0xcd4c: same body as the ResolutionPreset by-ref `convertToString`
    // at 0xc76c — asserts (enumconverter.h:262/263), in-range assigns the
    // name, out-of-range assigns `""`. Void function, `out` always assigned.
    debug_assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    out.clear();
    if let Some(p) = desc.pairs.iter().find(|p| p.value == value) {
        out.push_str(&p.name);
    }
}

// 0xceec — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings12QualityLevelEEERS3_RKT_
// mangled: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings12QualityLevelEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::QualityLevel>(RBX::CRenderSettings::QualityLevel const&)")]
pub fn stub_ceec(any: &mut RegionAny, value: i32) {
    // IDA 0xcef8..0xcf2e: same `placement_any::operator=<QualityLevel>` body
    // as 0xc90c — singleton init, same-holder store, else destruct + store.
    any.tag = REGION_ANY_QUALITY_LEVEL;
    any.value = value;
}

// 0xcf3c — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12QualityLevelEE9singletonEv
// mangled: __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12QualityLevelEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::QualityLevel>::singleton(void)")]
pub fn stub_cf3c() -> u32 {
    // IDA 0xcf3c: `typed_holder<QualityLevel>::singleton` — guard-protected
    // holder init (`s[0] = typeinfo`, `s[1] = destruct_func`, 0xcf8e),
    // returns `&s`. Host folds the holder address into its tag.
    REGION_ANY_QUALITY_LEVEL
}

// 0xcfa8 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12QualityLevelEE14construct_funcEPKcPc
// mangled: __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12QualityLevelEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::QualityLevel>::construct_func(char const*,char *)")]
pub fn stub_cfa8(src: RegionAny, dst: *mut RegionAny) -> RegionAny {
    // IDA 0xcfa8..0xcfb0: `if (a2) { result = *result; *a2 = result; }` —
    // same `construct_func` body as 0xc9c8 for `QualityLevel`.
    if !dst.is_null() {
        unsafe {
            *dst = src;
        }
    }
    src
}

// 0xcfb4 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12QualityLevelEE13destruct_funcEPc
// mangled: __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12QualityLevelEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::QualityLevel>::destruct_func(char *)")]
pub fn stub_cfb4() {
    // IDA 0xcfb4..0xcfb6: empty body — `typed_holder<QualityLevel>::destruct_func`
    // is a no-op (trivial enum payload), same as the ResolutionPreset twin at
    // 0xc9d4. Verified via IDA decompile.
}

// 0xcfb8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE13convertToItemERKS3_
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToItem(RBX::CRenderSettings::QualityLevel const&)const")]
pub fn stub_cfb8(desc: &RenderEnumDesc, value: i32) -> Option<i32> {
    // IDA 0xcfb8..0xd082: `ReleaseAssert(value>=0)` (enumconverter.h:273),
    // `ReleaseAssert((size_t)value<enumToItem.size())` (:274), then
    // `value < 0` → null, `value < size` → `table[value]`, else null.
    // Same shape as the ResolutionPreset twin at 0xc9d8. Verified via IDA
    // decompile.
    debug_assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    desc.pairs.iter().find(|p| p.value == value).map(|p| p.value)
}

// 0xd084 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings12QualityLevelENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// mangled: __ZN3rbx8any_castIRKN3RBX15CRenderSettings12QualityLevelENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CRenderSettings::QualityLevel const& rbx::any_cast<RBX::CRenderSettings::QualityLevel const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_d084(any: &RegionAny) -> i32 {
    // IDA 0xd084..0xd16a: holder typeinfo vs `typeinfo QualityLevel`, empty
    // holder reads as `void`; match returns the payload (`a1 + 4`), mismatch
    // throws `rbx::bad_placement_any_cast` (a `std::bad_cast`). Same shape as
    // the ResolutionPreset twin at 0xcaa4. Verified via IDA decompile.
    if any.tag != REGION_ANY_QUALITY_LEVEL {
        panic!("rbx::bad_placement_any_cast");
    }
    any.value
}

// 0xd174 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToValueERKNS_4NameERS3_
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToValue(RBX::Name const&,RBX::CRenderSettings::QualityLevel&)const")]
pub fn stub_d174(desc: &RenderEnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xd174..0xd1ee: `lower_bound` over the primary name map, then over
    // the legacy map; exact hit stores the item to out and returns 1, else 0.
    // Same shape as the ResolutionPreset twin at 0xcc34. Verified via IDA
    // decompile.
    match desc.pairs.iter().find(|p| p.name == name).map(|p| p.value).or_else(|| {
        desc.legacy_aliases.iter().find(|a| a.name == name).map(|a| a.maps_to)
    }) {
        Some(v) => {
            *out = v;
            true
        }
        None => false,
    }
}

// 0xd1f0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED2Ev
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::~EnumDesc()")]
pub fn stub_d1f0(desc: *mut RenderEnumDesc) {
    // IDA 0xd1f0 (`EnumDesc<QualityLevel>::D2`): vtable reset (host nop),
    // `++EnumRegistrar<QualityLevel>::registrar`, item-ptr range destroy,
    // heap-array deletes, `vector<string>` dtor, both RB-tree erases,
    // tail-call `EnumDescriptor::D2`. Same shape as the ResolutionPreset D2
    // at 0xccb0 (verified via IDA decompile of the ShadowMode twin 0xd730).
    if desc.is_null() {
        return;
    }
    unsafe {
        (*desc).pairs.clear();
        (*desc).legacy_aliases.clear();
    }
    ENUM_REGISTRAR_QUALITY_LEVEL.fetch_add(1, Ordering::SeqCst);
}

// 0xd28c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE15convertToStringERKS3_
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToString(RBX::CRenderSettings::ShadowMode const&)const")]
pub fn stub_d28c(desc: &RenderEnumDesc, value: i32, out: &mut String) {
    // IDA 0xd28c: same body as the ResolutionPreset by-ref `convertToString`
    // at 0xc76c — asserts (enumconverter.h:262/263), in-range assigns the
    // name, out-of-range assigns `""`. Void function, `out` always assigned.
    // Verified via IDA decompile.
    debug_assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    out.clear();
    if let Some(p) = desc.pairs.iter().find(|p| p.value == value) {
        out.push_str(&p.name);
    }
}

// 0xd42c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings10ShadowModeEEERS3_RKT_
// mangled: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings10ShadowModeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::ShadowMode>(RBX::CRenderSettings::ShadowMode const&)")]
pub fn stub_d42c(any: &mut RegionAny, value: i32) {
    // IDA 0xd42c: same `placement_any::operator=<ShadowMode>` body as 0xc90c
    // — singleton init, same-holder store, else destruct + store. Verified
    // via IDA decompile.
    any.tag = REGION_ANY_SHADOW_MODE;
    any.value = value;
}

// 0xd47c — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings10ShadowModeEE9singletonEv
// mangled: __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings10ShadowModeEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ShadowMode>::singleton(void)")]
pub fn stub_d47c() -> u32 {
    // IDA 0xd47c: `typed_holder<ShadowMode>::singleton` — guard-protected
    // holder init, returns `&s`. Host folds the holder address into its tag.
    // Verified via IDA decompile.
    REGION_ANY_SHADOW_MODE
}

// 0xd4e8 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings10ShadowModeEE14construct_funcEPKcPc
// mangled: __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings10ShadowModeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ShadowMode>::construct_func(char const*,char *)")]
pub fn stub_d4e8(src: RegionAny, dst: *mut RegionAny) -> RegionAny {
    // IDA 0xd4e8: `if (a2) { result = *result; *a2 = result; }` — same
    // `construct_func` body as 0xc9c8.
    if !dst.is_null() {
        unsafe {
            *dst = src;
        }
    }
    src
}

// 0xd4f4 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings10ShadowModeEE13destruct_funcEPc
// mangled: __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings10ShadowModeEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ShadowMode>::destruct_func(char *)")]
pub fn stub_d4f4() {
    // IDA 0xd4f4: empty body — `typed_holder<ShadowMode>::destruct_func` is a
    // no-op (trivial enum payload), same as 0xc9d4.
}

// 0xd4f8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE13convertToItemERKS3_
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToItem(RBX::CRenderSettings::ShadowMode const&)const")]
pub fn stub_d4f8(desc: &RenderEnumDesc, value: i32) -> Option<i32> {
    // IDA 0xd4f8: asserts (enumconverter.h:273/274), `value < 0` → null,
    // `value < size` → `table[value]`, else null. Same as 0xc9d8.
    debug_assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    desc.pairs.iter().find(|p| p.value == value).map(|p| p.value)
}

// 0xd5c4 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings10ShadowModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// mangled: __ZN3rbx8any_castIRKN3RBX15CRenderSettings10ShadowModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CRenderSettings::ShadowMode const& rbx::any_cast<RBX::CRenderSettings::ShadowMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_d5c4(any: &RegionAny) -> i32 {
    // IDA 0xd5c4: holder typeinfo vs `typeinfo ShadowMode`, empty reads as
    // `void`; match returns the payload, mismatch throws
    // `rbx::bad_placement_any_cast`. Same as 0xcaa4.
    if any.tag != REGION_ANY_SHADOW_MODE {
        panic!("rbx::bad_placement_any_cast");
    }
    any.value
}

// 0xd6b4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToValueERKNS_4NameERS3_
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::ShadowMode&)const")]
pub fn stub_d6b4(desc: &RenderEnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xd6b4: `lower_bound` over the primary name map, then the legacy
    // map; exact hit stores the item to out and returns 1, else 0. Same as
    // 0xcc34.
    match desc.pairs.iter().find(|p| p.name == name).map(|p| p.value).or_else(|| {
        desc.legacy_aliases.iter().find(|a| a.name == name).map(|a| a.maps_to)
    }) {
        Some(v) => {
            *out = v;
            true
        }
        None => false,
    }
}

// 0xd730 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED2Ev
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::~EnumDesc()")]
pub fn stub_d730(desc: *mut RenderEnumDesc) {
    // IDA 0xd730 (`EnumDesc<ShadowMode>::D2`): vtable reset (host nop),
    // `++EnumRegistrar<ShadowMode>::registrar`, item-ptr range destroy,
    // heap-array deletes, `vector<string>` dtor, both RB-tree erases,
    // tail-call `EnumDescriptor::D2`. Same as 0xccb0. Verified via IDA
    // decompile.
    if desc.is_null() {
        return;
    }
    unsafe {
        (*desc).pairs.clear();
        (*desc).legacy_aliases.clear();
    }
    ENUM_REGISTRAR_SHADOW_MODE.fetch_add(1, Ordering::SeqCst);
}

// 0xd7cc — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE15convertToStringERKS3_
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToString(RBX::CRenderSettings::AntialiasingMode const&)const")]
pub fn stub_d7cc(desc: &RenderEnumDesc, value: i32, out: &mut String) {
    // IDA 0xd7cc: same body as the ResolutionPreset by-ref `convertToString`
    // at 0xc76c — asserts (enumconverter.h:262/263), in-range assigns the
    // name, out-of-range assigns `""`. Void function, `out` always assigned.
    debug_assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    out.clear();
    if let Some(p) = desc.pairs.iter().find(|p| p.value == value) {
        out.push_str(&p.name);
    }
}

// 0xd96c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings16AntialiasingModeEEERS3_RKT_
// mangled: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings16AntialiasingModeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::AntialiasingMode>(RBX::CRenderSettings::AntialiasingMode const&)")]
pub fn stub_d96c(any: &mut RegionAny, value: i32) {
    // IDA 0xd96c: same `placement_any::operator=<AntialiasingMode>` body as
    // 0xc90c — singleton init, same-holder store, else destruct + store.
    any.tag = REGION_ANY_ANTIALIASING_MODE;
    any.value = value;
}

// 0xd9bc — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16AntialiasingModeEE9singletonEv
// mangled: __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16AntialiasingModeEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AntialiasingMode>::singleton(void)")]
pub fn stub_d9bc() -> u32 {
    // IDA 0xd9bc: `typed_holder<AntialiasingMode>::singleton` —
    // guard-protected holder init, returns `&s`. Host folds the holder
    // address into its tag.
    REGION_ANY_ANTIALIASING_MODE
}

// 0xda28 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16AntialiasingModeEE14construct_funcEPKcPc
// mangled: __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16AntialiasingModeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AntialiasingMode>::construct_func(char const*,char *)")]
pub fn stub_da28(src: RegionAny, dst: *mut RegionAny) -> RegionAny {
    // IDA 0xda28: `if (a2) { result = *result; *a2 = result; }` — same
    // `construct_func` body as 0xc9c8.
    if !dst.is_null() {
        unsafe {
            *dst = src;
        }
    }
    src
}

// 0xda34 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16AntialiasingModeEE13destruct_funcEPc
// mangled: __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16AntialiasingModeEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AntialiasingMode>::destruct_func(char *)")]
pub fn stub_da34() {
    // IDA 0xda34: empty body — `typed_holder<AntialiasingMode>::destruct_func`
    // is a no-op (trivial enum payload), same as 0xc9d4.
}

// 0xda38 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE13convertToItemERKS3_
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToItem(RBX::CRenderSettings::AntialiasingMode const&)const")]
pub fn stub_da38(desc: &RenderEnumDesc, value: i32) -> Option<i32> {
    // IDA 0xda38: asserts (enumconverter.h:273/274), `value < 0` → null,
    // `value < size` → `table[value]`, else null. Same as 0xc9d8.
    debug_assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    desc.pairs.iter().find(|p| p.value == value).map(|p| p.value)
}

// 0xdb04 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings16AntialiasingModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// mangled: __ZN3rbx8any_castIRKN3RBX15CRenderSettings16AntialiasingModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CRenderSettings::AntialiasingMode const& rbx::any_cast<RBX::CRenderSettings::AntialiasingMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_db04(any: &RegionAny) -> i32 {
    // IDA 0xdb04: holder typeinfo vs `typeinfo AntialiasingMode`, empty reads
    // as `void`; match returns the payload, mismatch throws
    // `rbx::bad_placement_any_cast`. Same as 0xcaa4.
    if any.tag != REGION_ANY_ANTIALIASING_MODE {
        panic!("rbx::bad_placement_any_cast");
    }
    any.value
}

// 0xdbf4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToValueERKNS_4NameERS3_
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::AntialiasingMode&)const")]
pub fn stub_dbf4(desc: &RenderEnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xdbf4: `lower_bound` over the primary name map, then the legacy
    // map; exact hit stores the item to out and returns 1, else 0. Same as
    // 0xcc34.
    match desc.pairs.iter().find(|p| p.name == name).map(|p| p.value).or_else(|| {
        desc.legacy_aliases.iter().find(|a| a.name == name).map(|a| a.maps_to)
    }) {
        Some(v) => {
            *out = v;
            true
        }
        None => false,
    }
}

// 0xdc70 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED2Ev
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc()")]
pub fn stub_dc70(desc: *mut RenderEnumDesc) {
    // IDA 0xdc70 (`EnumDesc<AntialiasingMode>::D2`): vtable reset (host nop),
    // `++EnumRegistrar<AntialiasingMode>::registrar`, item-ptr range destroy,
    // heap-array deletes, `vector<string>` dtor, both RB-tree erases,
    // tail-call `EnumDescriptor::D2`. Same as 0xccb0.
    if desc.is_null() {
        return;
    }
    unsafe {
        (*desc).pairs.clear();
        (*desc).legacy_aliases.clear();
    }
    ENUM_REGISTRAR_ANTIALIASING_MODE.fetch_add(1, Ordering::SeqCst);
}

// 0xdd0c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE15convertToStringERKS3_
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToString(RBX::CRenderSettings::FrameRateManagerMode const&)const")]
pub fn stub_dd0c(desc: &RenderEnumDesc, value: i32, out: &mut String) {
    // IDA 0xdd0c: same body as the ResolutionPreset by-ref `convertToString`
    // at 0xc76c — asserts (enumconverter.h:262/263), in-range assigns the
    // name, out-of-range assigns `""`. Void function, `out` always assigned.
    debug_assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    out.clear();
    if let Some(p) = desc.pairs.iter().find(|p| p.value == value) {
        out.push_str(&p.name);
    }
}

// 0xdeac — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings20FrameRateManagerModeEEERS3_RKT_
// mangled: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings20FrameRateManagerModeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::FrameRateManagerMode>(RBX::CRenderSettings::FrameRateManagerMode const&)")]
pub fn stub_deac(any: &mut RegionAny, value: i32) {
    // IDA 0xdeac: same `placement_any::operator=<FrameRateManagerMode>` body
    // as 0xc90c — singleton init, same-holder store, else destruct + store.
    any.tag = REGION_ANY_FRAME_RATE_MANAGER_MODE;
    any.value = value;
}

// 0xdefc — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings20FrameRateManagerModeEE9singletonEv
// mangled: __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings20FrameRateManagerModeEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::FrameRateManagerMode>::singleton(void)")]
pub fn stub_defc() -> u32 {
    // IDA 0xdefc: `typed_holder<FrameRateManagerMode>::singleton` —
    // guard-protected holder init, returns `&s`. Host folds the holder
    // address into its tag.
    REGION_ANY_FRAME_RATE_MANAGER_MODE
}

// 0xdf68 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings20FrameRateManagerModeEE14construct_funcEPKcPc
// mangled: __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings20FrameRateManagerModeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::FrameRateManagerMode>::construct_func(char const*,char *)")]
pub fn stub_df68(src: RegionAny, dst: *mut RegionAny) -> RegionAny {
    // IDA 0xdf68: `if (a2) { result = *result; *a2 = result; }` — same
    // `construct_func` body as 0xc9c8.
    if !dst.is_null() {
        unsafe {
            *dst = src;
        }
    }
    src
}

// 0xdf74 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings20FrameRateManagerModeEE13destruct_funcEPc
// mangled: __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings20FrameRateManagerModeEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::FrameRateManagerMode>::destruct_func(char *)")]
pub fn stub_df74() {
    // IDA 0xdf74: empty body — `typed_holder<FrameRateManagerMode>::destruct_func`
    // is a no-op (trivial enum payload), same as 0xc9d4.
}

// 0xdf78 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE13convertToItemERKS3_
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToItem(RBX::CRenderSettings::FrameRateManagerMode const&)const")]
pub fn stub_df78(desc: &RenderEnumDesc, value: i32) -> Option<i32> {
    // IDA 0xdf78: asserts (enumconverter.h:273/274), `value < 0` → null,
    // `value < size` → `table[value]`, else null. Same as 0xc9d8.
    debug_assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    desc.pairs.iter().find(|p| p.value == value).map(|p| p.value)
}

// 0xe044 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings20FrameRateManagerModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// mangled: __ZN3rbx8any_castIRKN3RBX15CRenderSettings20FrameRateManagerModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CRenderSettings::FrameRateManagerMode const& rbx::any_cast<RBX::CRenderSettings::FrameRateManagerMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_e044(any: &RegionAny) -> i32 {
    // IDA 0xe044: holder typeinfo vs `typeinfo FrameRateManagerMode`, empty
    // reads as `void`; match returns the payload, mismatch throws
    // `rbx::bad_placement_any_cast`. Same as 0xcaa4.
    if any.tag != REGION_ANY_FRAME_RATE_MANAGER_MODE {
        panic!("rbx::bad_placement_any_cast");
    }
    any.value
}

// 0xe134 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueERKNS_4NameERS3_
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::FrameRateManagerMode&)const")]
pub fn stub_e134(desc: &RenderEnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xe134: `lower_bound` over the primary name map, then the legacy
    // map; exact hit stores the item to out and returns 1, else 0. Same as
    // 0xcc34.
    match desc.pairs.iter().find(|p| p.name == name).map(|p| p.value).or_else(|| {
        desc.legacy_aliases.iter().find(|a| a.name == name).map(|a| a.maps_to)
    }) {
        Some(v) => {
            *out = v;
            true
        }
        None => false,
    }
}

// 0xe1b0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED2Ev
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::~EnumDesc()")]
pub fn stub_e1b0(desc: *mut RenderEnumDesc) {
    // IDA 0xe1b0 (`EnumDesc<FrameRateManagerMode>::D2`): vtable reset (host
    // nop), `++EnumRegistrar<FrameRateManagerMode>::registrar`, item-ptr
    // range destroy, heap-array deletes, `vector<string>` dtor, both RB-tree
    // erases, tail-call `EnumDescriptor::D2`. Same as 0xccb0.
    if desc.is_null() {
        return;
    }
    unsafe {
        (*desc).pairs.clear();
        (*desc).legacy_aliases.clear();
    }
    ENUM_REGISTRAR_FRAME_RATE_MANAGER_MODE.fetch_add(1, Ordering::SeqCst);
}

// 0xe24c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringERKS3_
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToString(RBX::CRenderSettings::GraphicsMode const&)const")]
pub fn stub_e24c(desc: &RenderEnumDesc, value: i32, out: &mut String) {
    // IDA 0xe24c: same body as the ResolutionPreset by-ref `convertToString`
    // at 0xc76c — asserts (enumconverter.h:262/263), in-range assigns the
    // name, out-of-range assigns `""`. Void function, `out` always assigned.
    debug_assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    out.clear();
    if let Some(p) = desc.pairs.iter().find(|p| p.value == value) {
        out.push_str(&p.name);
    }
}

// 0xe3ec — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings12GraphicsModeEEERS3_RKT_
// mangled: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings12GraphicsModeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::GraphicsMode>(RBX::CRenderSettings::GraphicsMode const&)")]
pub fn stub_e3ec(any: &mut RegionAny, value: i32) {
    // IDA 0xe3ec: same `placement_any::operator=<GraphicsMode>` body as
    // 0xc90c — singleton init, same-holder store, else destruct + store.
    any.tag = REGION_ANY_GRAPHICS_MODE;
    any.value = value;
}

// 0xe43c — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12GraphicsModeEE9singletonEv
// mangled: __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12GraphicsModeEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::GraphicsMode>::singleton(void)")]
pub fn stub_e43c() -> u32 {
    // IDA 0xe43c: `typed_holder<GraphicsMode>::singleton` — guard-protected
    // holder init, returns `&s`. Host folds the holder address into its tag.
    REGION_ANY_GRAPHICS_MODE
}

// 0xe4a8 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12GraphicsModeEE14construct_funcEPKcPc
// mangled: __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12GraphicsModeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::GraphicsMode>::construct_func(char const*,char *)")]
pub fn stub_e4a8(src: RegionAny, dst: *mut RegionAny) -> RegionAny {
    // IDA 0xe4a8: `if (a2) { result = *result; *a2 = result; }` — same
    // `construct_func` body as 0xc9c8.
    if !dst.is_null() {
        unsafe {
            *dst = src;
        }
    }
    src
}

// 0xe4b4 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12GraphicsModeEE13destruct_funcEPc
// mangled: __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12GraphicsModeEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::GraphicsMode>::destruct_func(char *)")]
pub fn stub_e4b4() {
    // IDA 0xe4b4: empty body — `typed_holder<GraphicsMode>::destruct_func` is
    // a no-op (trivial enum payload), same as 0xc9d4.
}

// 0xe4b8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE13convertToItemERKS3_
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToItem(RBX::CRenderSettings::GraphicsMode const&)const")]
pub fn stub_e4b8(desc: &RenderEnumDesc, value: i32) -> Option<i32> {
    // IDA 0xe4b8: asserts (enumconverter.h:273/274), `value < 0` → null,
    // `value < size` → `table[value]`, else null. Same as 0xc9d8.
    debug_assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    desc.pairs.iter().find(|p| p.value == value).map(|p| p.value)
}

// 0xe584 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings12GraphicsModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// mangled: __ZN3rbx8any_castIRKN3RBX15CRenderSettings12GraphicsModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CRenderSettings::GraphicsMode const& rbx::any_cast<RBX::CRenderSettings::GraphicsMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_e584(any: &RegionAny) -> i32 {
    // IDA 0xe584: holder typeinfo vs `typeinfo GraphicsMode`, empty reads as
    // `void`; match returns the payload, mismatch throws
    // `rbx::bad_placement_any_cast`. Same as 0xcaa4.
    if any.tag != REGION_ANY_GRAPHICS_MODE {
        panic!("rbx::bad_placement_any_cast");
    }
    any.value
}

// 0xe674 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueERKNS_4NameERS3_
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::GraphicsMode&)const")]
pub fn stub_e674(desc: &RenderEnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xe674: `lower_bound` over the primary name map, then the legacy
    // map; exact hit stores the item to out and returns 1, else 0. Same as
    // 0xcc34.
    match desc.pairs.iter().find(|p| p.name == name).map(|p| p.value).or_else(|| {
        desc.legacy_aliases.iter().find(|a| a.name == name).map(|a| a.maps_to)
    }) {
        Some(v) => {
            *out = v;
            true
        }
        None => false,
    }
}

// 0xe6f0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED2Ev
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()")]
pub fn stub_e6f0(desc: *mut RenderEnumDesc) {
    // IDA 0xe6f0 (`EnumDesc<GraphicsMode>::D2`): vtable reset (host nop),
    // `++EnumRegistrar<GraphicsMode>::registrar`, item-ptr range destroy,
    // heap-array deletes, `vector<string>` dtor, both RB-tree erases,
    // tail-call `EnumDescriptor::D2`. Same as 0xccb0.
    if desc.is_null() {
        return;
    }
    unsafe {
        (*desc).pairs.clear();
        (*desc).legacy_aliases.clear();
    }
    ENUM_REGISTRAR_GRAPHICS_MODE.fetch_add(1, Ordering::SeqCst);
}

// 0xe78c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE15convertToStringERKS3_
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToString(RBX::CRenderSettings::AASamples const&)const")]
pub fn stub_e78c(desc: &RenderEnumDesc, value: i32, out: &mut String) {
    // IDA 0xe78c: same body as the ResolutionPreset by-ref `convertToString`
    // at 0xc76c — asserts (enumconverter.h:262/263), in-range assigns the
    // name, out-of-range assigns `""`. Void function, `out` always assigned.
    debug_assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    out.clear();
    if let Some(p) = desc.pairs.iter().find(|p| p.value == value) {
        out.push_str(&p.name);
    }
}

// 0xe92c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings9AASamplesEEERS3_RKT_
// mangled: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings9AASamplesEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::AASamples>(RBX::CRenderSettings::AASamples const&)")]
pub fn stub_e92c(any: &mut RegionAny, value: i32) {
    // IDA 0xe92c: same `placement_any::operator=<AASamples>` body as 0xc90c
    // — singleton init, same-holder store, else destruct + store.
    any.tag = REGION_ANY_AA_SAMPLES;
    any.value = value;
}

// 0xe97c — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings9AASamplesEE9singletonEv
// mangled: __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings9AASamplesEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::singleton(void)")]
pub fn stub_e97c() -> u32 {
    // IDA 0xe97c: `typed_holder<AASamples>::singleton` — guard-protected
    // holder init, returns `&s`. Host folds the holder address into its tag.
    REGION_ANY_AA_SAMPLES
}

// 0xe9e8 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings9AASamplesEE14construct_funcEPKcPc
// mangled: __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings9AASamplesEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::construct_func(char const*,char *)")]
pub fn stub_e9e8(src: RegionAny, dst: *mut RegionAny) -> RegionAny {
    // IDA 0xe9e8: `if (a2) { result = *result; *a2 = result; }` — same
    // `construct_func` body as 0xc9c8.
    if !dst.is_null() {
        unsafe {
            *dst = src;
        }
    }
    src
}

// 0xe9f4 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings9AASamplesEE13destruct_funcEPc
// mangled: __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings9AASamplesEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::destruct_func(char *)")]
pub fn stub_e9f4() {
    // IDA 0xe9f4: empty body — `typed_holder<AASamples>::destruct_func` is a
    // no-op (trivial enum payload), same as 0xc9d4.
}

// 0xe9f8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE13convertToItemERKS3_
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToItem(RBX::CRenderSettings::AASamples const&)const")]
pub fn stub_e9f8(desc: &RenderEnumDesc, value: i32) -> Option<i32> {
    // IDA 0xe9f8: asserts (enumconverter.h:273/274), `value < 0` → null,
    // `value < size` → `table[value]`, else null. Same as 0xc9d8.
    debug_assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    desc.pairs.iter().find(|p| p.value == value).map(|p| p.value)
}

// 0xeac4 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings9AASamplesENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// mangled: __ZN3rbx8any_castIRKN3RBX15CRenderSettings9AASamplesENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CRenderSettings::AASamples const& rbx::any_cast<RBX::CRenderSettings::AASamples const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_eac4(any: &RegionAny) -> i32 {
    // IDA 0xeac4: holder typeinfo vs `typeinfo AASamples`, empty reads as
    // `void`; match returns the payload, mismatch throws
    // `rbx::bad_placement_any_cast`. Same as 0xcaa4.
    if any.tag != REGION_ANY_AA_SAMPLES {
        panic!("rbx::bad_placement_any_cast");
    }
    any.value
}

// 0xebb4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueERKNS_4NameERS3_
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToValue(RBX::Name const&,RBX::CRenderSettings::AASamples&)const")]
pub fn stub_ebb4(desc: &RenderEnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xebb4: `lower_bound` over the primary name map, then the legacy
    // map; exact hit stores the item to out and returns 1, else 0. Same as
    // 0xcc34.
    match desc.pairs.iter().find(|p| p.name == name).map(|p| p.value).or_else(|| {
        desc.legacy_aliases.iter().find(|a| a.name == name).map(|a| a.maps_to)
    }) {
        Some(v) => {
            *out = v;
            true
        }
        None => false,
    }
}

// 0xec30 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED2Ev
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()")]
pub fn stub_ec30(desc: *mut RenderEnumDesc) {
    // IDA 0xec30 (`EnumDesc<AASamples>::D2`): vtable reset (host nop),
    // `++EnumRegistrar<AASamples>::registrar`, item-ptr range destroy,
    // heap-array deletes, `vector<string>` dtor, both RB-tree erases,
    // tail-call `EnumDescriptor::D2`. Same as 0xccb0.
    if desc.is_null() {
        return;
    }
    unsafe {
        (*desc).pairs.clear();
        (*desc).legacy_aliases.clear();
    }
    ENUM_REGISTRAR_AA_SAMPLES.fetch_add(1, Ordering::SeqCst);
}

// 0xeccc — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD2Ev
// mangled: __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD2Ev
// type: int __fastcall(int)
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_eccc() {
    // IDA 0xeccc (`Creator::D2`): vtable reset (host nop) +
    // `ReleaseAssert(wasConstructed())` (`../App/include/Util/Object.h:255`,
    // `isConstructed == 666`). Verified via IDA decompile.
    debug_assert!(CREATOR_IS_CONSTRUCTED.load(Ordering::SeqCst), "wasConstructed() ../App/include/Util/Object.h:255");
}

// 0xedfc — __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator12getClassNameEv
// mangled: __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_edfc() -> &'static str {
    // IDA 0xedfc (`Creator::getClassName`): `ReleaseAssert(wasConstructed())`,
    // `Name::declare<sRenderSettings>()`, tail-jump to
    // `Name::doDeclare<sRenderSettings>()` (0xf1dc) returning the
    // `RenderSettings` name. Same sequence as 0xb8d0 documents. Verified via
    // IDA disasm (assert prologue vs `isConstructed == 0x29A`).
    debug_assert!(CREATOR_IS_CONSTRUCTED.load(Ordering::SeqCst), "wasConstructed() ../App/include/Util/Object.h");
    stub_f1dc()
}

// 0xee84 — __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator6createEv
// mangled: __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator6createEv
// type: int __fastcall(int *)
#[doc(alias = "__ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator6createEv")]
pub fn stub_ee84() -> SharedPtr<RenderSettingsItem> {
    // IDA 0xee84 (`Creator::create`): `ReleaseAssert(wasConstructed())`
    // (`Object.h:231`), `Creatable::create<CRenderSettingsItem>` (0xef04),
    // then the `+32` thunk offset into the `shared_ptr` pair. Host: the fresh
    // item behind `SharedPtr` (the `Instance` base offset is a layout detail
    // with no host counterpart). Verified via IDA decompile.
    debug_assert!(CREATOR_IS_CONSTRUCTED.load(Ordering::SeqCst), "wasConstructed() ../App/include/Util/Object.h:231");
    stub_ef04()
}

// 0xef04 — __ZN3RBX9CreatableINS_8InstanceEE6createI19CRenderSettingsItemEEN5boost10shared_ptrIT_EEv
// mangled: __ZN3RBX9CreatableINS_8InstanceEE6createI19CRenderSettingsItemEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<CRenderSettingsItem> RBX::Creatable<RBX::Instance>::create<CRenderSettingsItem>(void)")]
// was: boost::shared_ptr -> rbx_core::SharedPtr
pub fn stub_ef04() -> SharedPtr<RenderSettingsItem> {
    // IDA 0xef04 (`Creatable<Instance>::create<CRenderSettingsItem>`):
    // `operator new(0xC4)`, `CRenderSettingsItem::CRenderSettingsItem`,
    // `shared_ptr` ctor with the `Creatable::Deleter`. Host: default item
    // adopted by `SharedPtr` (`Arc`); `boost::shared_ptr` never appears
    // (AGENTS.md). Verified via IDA decompile.
    SharedPtr::from(Box::new(RenderSettingsItem::default()))
}

// 0xefb4 — __ZN5boost10shared_ptrI19CRenderSettingsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_
// mangled: __ZN5boost10shared_ptrI19CRenderSettingsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<CRenderSettingsItem>::shared_ptr<CRenderSettingsItem,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr -> rbx_core::SharedPtr
pub fn stub_efb4(item: RenderSettingsItem) -> SharedPtr<RenderSettingsItem> {
    // IDA 0xefb4 (`shared_ptr` ctor): `px = p`, `shared_count` control-block
    // alloc (0xf098), `_internal_accept_owner` for the
    // `enable_shared_from_this<DescribedBase>` base. Host: `Arc` adoption
    // covers all three (cf. `shared_ptr_from_raw`). Verified via IDA
    // decompile.
    SharedPtr::from(Box::new(item))
}

// 0xefd8 — __ZNK5boost6detail15sp_counted_base9use_countEv
// mangled: __ZNK5boost6detail15sp_counted_base9use_countEv
// type: int __fastcall(boost::detail::sp_counted_base *this)
#[doc(alias = "boost::detail::sp_counted_base::use_count(void)const")]
pub fn stub_efd8(shared: &SharedPtr<RenderSettingsItem>) -> usize {
    // IDA 0xefd8 (`sp_counted_base::use_count`): spinlock-mutex lock, load
    // `use_count`, unlock, return. Host `Arc::strong_count` is the same
    // atomic load. Verified via IDA decompile.
    SharedPtr::strong_count(shared)
}

// 0xf098 — __ZN5boost6detail12shared_countC2IP19CRenderSettingsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_
// mangled: __ZN5boost6detail12shared_countC2IP19CRenderSettingsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f098(item: RenderSettingsItem) -> ControlBlockPd<RenderSettingsItem, CreatableInstanceDeleter> {
    // IDA 0xf098 (`shared_count` ctor): `*a1 = 0`, `new 0x14` control block
    // with both counts 1, vtable + `px` stores. Host: fresh
    // `ControlBlockPd` (counts 1/1). Verified via IDA decompile.
    ControlBlockPd::new(Box::new(item), CreatableInstanceDeleter)
}

// 0xf198 — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev
// mangled: __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_f198() {
    // IDA 0xf198 (`sp_counted_impl_pd::D1`): empty body. Verified via IDA
    // decompile.
}

// 0xf19c — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv
// mangled: __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_f19c(block: &mut ControlBlockPd<RenderSettingsItem, CreatableInstanceDeleter>) {
    // IDA 0xf19c (`dispose`): `v2 = px`, `Instance::predelete(v2)`, then the
    // virtual deleting dtor when non-null. Host: `predelete` is
    // datamodel-owned (passed as the hook), the trailing delete is the drop.
    // Verified via IDA decompile.
    block.dispose_with(|_| {});
}

// 0xf1bc — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// mangled: __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_f1bc(block: &ControlBlockPd<RenderSettingsItem, CreatableInstanceDeleter>, type_name: &str) -> Option<CreatableInstanceDeleter> {
    // IDA 0xf1bc (`get_deleter`): `return a1 + 16` iff the queried
    // `type_info` name is `N3RBX9CreatableINS_8InstanceEE7DeleterE`, else
    // null. Host delegates to the block. Verified via IDA decompile.
    block.get_deleter(type_name)
}

// 0xf1d4 — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv
// mangled: __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_f1d4(block: &ControlBlockPd<RenderSettingsItem, CreatableInstanceDeleter>) -> CreatableInstanceDeleter {
    // IDA 0xf1d4 (`get_untyped_deleter`): unconditionally `return a1 + 16`.
    // Host delegates to the block. Verified via IDA decompile.
    block.get_untyped_deleter()
}

// 0xf1d8 — __ZN3RBX4Name13callDoDeclareILZ15sRenderSettingsEEEvv
// mangled: __ZN3RBX4Name13callDoDeclareILZ15sRenderSettingsEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZ15sRenderSettingsEEEvv")]
pub fn stub_f1d8() -> &'static str {
    // IDA 0xf1d8 (`Name::callDoDeclare<sRenderSettings>`): thunk tail-calling
    // `doDeclare` (0xf1dc). Verified via IDA decompile.
    stub_f1dc()
}

// 0xf1dc — __ZN3RBX4Name9doDeclareILZ15sRenderSettingsEEERKS0_v
// mangled: __ZN3RBX4Name9doDeclareILZ15sRenderSettingsEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZ15sRenderSettingsEEERKS0_v")]
pub fn stub_f1dc() -> &'static str {
    // IDA 0xf1dc (`Name::doDeclare<sRenderSettings>`): `__cxa_guard` one-shot
    // `Name::declare(&sRenderSettings)`, returns the interned name. Host
    // folds the interned `RenderSettings` name into a `LazyLock` literal.
    // Verified via IDA decompile.
    static DECLARED: LazyLock<&'static str> = LazyLock::new(|| "RenderSettings");
    *DECLARED
}

// 0xf2bc — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorC2Ev
// mangled: __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_f2bc() {
    // IDA 0xf2bc (`Creator::C2`): vtable store then `boost::call_once` with
    // `callDoDeclare<sRenderSettings>` (0xf1d8) declaring the class name;
    // the constructed marker (`isConstructed = 666`) arms the D2/create
    // asserts. Host runs the declare thunk once and sets the flag. Verified
    // via IDA decompile.
    stub_f1d8();
    CREATOR_IS_CONSTRUCTED.store(true, Ordering::SeqCst);
}

// 0xf500 — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE17static_getCreatorEv
// mangled: __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_f500() {
    // IDA 0xf500 (`static_getCreator`): `ReleaseAssert(Creator::
    // wasConstructed())` (`Object.h:282`), returns `&creatorPrivate`. The
    // singleton creator is folded into `CREATOR_IS_CONSTRUCTED`; the assert
    // is the observable contract. Verified via IDA decompile.
    debug_assert!(CREATOR_IS_CONSTRUCTED.load(Ordering::SeqCst), "Creator::wasConstructed() ../App/include/Util/Object.h:282");
}

// 0xf574 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4nextERN5boost13intrusive_ptrINS8_4slotEEE
// mangled: __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4nextERN5boost13intrusive_ptrINS8_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot> &)")]
// was: boost::shared_ptr -> rbx_core::SharedPtr
pub fn stub_f574(_signal: &Signal<u32>) {
    // IDA 0xf574 (`signal<...>::next`): `call_once` mutex init,
    // `intrusive_ptr_add_ref` on the slot, iterator advance to the next live
    // slot. Host `Signal::fire` (cf. 0xb76c) subsumes the slot walk; the
    // advance itself is drop glue — no-op. `Signal` is
    // `rbx_core::signal::Signal`, never `boost`. Verified via IDA decompile.
}

// 0xf6dc — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE8on_errorERSt9exception
// mangled: __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE8on_errorERSt9exception
// type: int *()
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::on_error(std::exception &)")]
pub fn stub_f6dc() {
    // IDA 0xf6dc (`signal<...>::on_error`): loads the
    // `slot_exception_handler`; when non-null invokes it, else returns the
    // handler slot. Host `Signal::fire` handles slot errors inline; the
    // default-handler path is a no-op. Verified via IDA decompile.
}

// 0xf704 — __ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// mangled: __ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int __fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_insert_aux(__gnu_cxx::__normal_iterator<G3D::Vector2int16*,std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>>,G3D::Vector2int16 const&)")]
pub fn stub_f704(vec: &mut Vec<i32>, index: usize, value: i32) {
    // IDA 0xf704 (`vector<Vector2int16>::_M_insert_aux`): capacity check,
    // `length_error` at the `0x3FFFFFFF` cap, realloc + shift + insert at
    // `pos`, else shift-right + store. `Vector2int16` is 4 bytes (host
    // `i32`); `Vec::insert` covers both paths. Verified via IDA decompile.
    vec.insert(index, value);
}
