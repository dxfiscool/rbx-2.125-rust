//! audio generated_123 — next 100 stubs EA-sorted, from ida/export.json
//! Filter: FMOD|Soundscape exhausted (2398 distinct) — filler EA-sorted asc, skip existing, rbx_core::SharedPtr not boost
//! Batch: 100 stubs | skeleton batch | range 0xc154..0xf800 EA-sorted asc filler after 0xf6f8c4, skip existing, rbx_core::SharedPtr not boost
//! Generated: 2026-09-01

use std::sync::LazyLock;
use crate::generated::flog_asserts;

// ---- CRenderSettings EnumDesc host model (IDA 0x8c4c/0xc16c..0xd6b4) ----
// Mirrors the ReverbType template in generated.rs (IDA 0x3775f8..0x377c10):
// convert tables live in per-enum ITEMS slices; holder tags in
// RenderSettingsAny; guarded singletons in LazyLock (host cxa_guard).
// The item-registration (addItem static-init) path for these CRenderSettings
// enums is not yet recovered, so the tables match a freshly-constructed desc
// (cf. rbx_reflection EnumDesc::new) — empty, and every convert path behaves
// as the original does on an empty desc.
// Boost mapping: throw_exception<bad_placement_any_cast> -> Err,
// intrusive_ptr/shared_ptr -> Arc (SharedPtr), std::map lower_bound ->
// binary_search over name-sorted slices.

/// rbx::placement_any<RBX::Region3> holdings for the CRenderSettings enums.
#[derive(Clone, Default)]
pub enum RenderSettingsAny {
    #[default]
    Empty,
    ShadowMode(i32),
    QualityLevel(i32),
    ResolutionPreset(i32),
    AntialiasingMode(i32),
    FrameRateManagerMode(i32),
    GraphicsMode(i32),
}

/// Cast failures on the CRenderSettings convert paths (bad_cast).
#[derive(Debug, thiserror::Error)]
pub enum RenderEnumCastError {
    #[error("rbx::bad_placement_any_cast")]
    BadPlacementAnyCast,
}

/// (name, value) in image/index order; index doubles as the value slot.
/// IDA 0x8c4c (ShadowMode EnumDesc C2): only the base EnumDescriptor +
/// empty tables are recovered, no addItem calls seen.
pub const SHADOW_MODE_ITEMS: &[(&str, i32)] = &[];
/// Name-sorted view for the convertToValue tree search (IDA 0xd6b4).
pub const SHADOW_MODE_BY_NAME: &[(&str, i32)] = &[];
/// Legacy-name view for the second walk in convertToValue (IDA 0xd6ee..0xd722).
pub const SHADOW_MODE_LEGACY_BY_NAME: &[(&str, i32)] = &[];
/// IDA QualityLevel EnumDesc C2: base + empty tables, no addItems recovered.
pub const QUALITY_LEVEL_ITEMS: &[(&str, i32)] = &[];
/// Name-sorted view for the convertToValue tree search (IDA 0xd174).
pub const QUALITY_LEVEL_BY_NAME: &[(&str, i32)] = &[];
/// Legacy-name view for the second walk in convertToValue (IDA 0xcc6e..0xcca2).
pub const QUALITY_LEVEL_LEGACY_BY_NAME: &[(&str, i32)] = &[];
/// IDA ResolutionPreset EnumDesc C2: base + empty tables, no addItems recovered.
pub const RESOLUTION_PRESET_ITEMS: &[(&str, i32)] = &[];
/// Name-sorted view for the convertToValue tree search (IDA 0xcc34).
pub const RESOLUTION_PRESET_BY_NAME: &[(&str, i32)] = &[];
/// Legacy-name view for the second walk in convertToValue (IDA 0xcc6e..0xcca2).
pub const RESOLUTION_PRESET_LEGACY_BY_NAME: &[(&str, i32)] = &[];
/// IDA AntialiasingMode EnumDesc C2: base + empty tables, no addItems recovered.
pub const ANTIALIASING_MODE_ITEMS: &[(&str, i32)] = &[];
/// Name-sorted view for the convertToValue tree search (IDA 0xdbf4).
pub const ANTIALIASING_MODE_BY_NAME: &[(&str, i32)] = &[];
/// Legacy-name view for the second walk in convertToValue.
pub const ANTIALIASING_MODE_LEGACY_BY_NAME: &[(&str, i32)] = &[];
/// IDA FrameRateManagerMode EnumDesc C2: base + empty tables, no addItems recovered.
pub const FRAME_RATE_MANAGER_MODE_ITEMS: &[(&str, i32)] = &[];
/// Name-sorted view for the convertToValue tree search (IDA 0xe134).
pub const FRAME_RATE_MANAGER_MODE_BY_NAME: &[(&str, i32)] = &[];
/// Legacy-name view for the second walk in convertToValue.
pub const FRAME_RATE_MANAGER_MODE_LEGACY_BY_NAME: &[(&str, i32)] = &[];
/// IDA GraphicsMode EnumDesc C2: base + empty tables, no addItems recovered.
pub const GRAPHICS_MODE_ITEMS: &[(&str, i32)] = &[];
/// Name-sorted view for the convertToValue tree search (IDA 0xe674).
pub const GRAPHICS_MODE_BY_NAME: &[(&str, i32)] = &[];
/// Legacy-name view for the second walk in convertToValue.
pub const GRAPHICS_MODE_LEGACY_BY_NAME: &[(&str, i32)] = &[];

/// Holder tag for the ShadowMode placement_any (typeinfo name, IDA 0xd64c).
pub struct ShadowModeHolder {
    pub type_name: &'static str,
    pub construct: fn(*const i32, *mut i32) -> i32,
    pub destruct: fn(),
}

static SHADOW_MODE_HOLDER: LazyLock<ShadowModeHolder> = LazyLock::new(|| ShadowModeHolder {
    type_name: "N3RBX15CRenderSettings10ShadowModeE",
    construct: stub_d4e8,
    destruct: stub_d4f4,
});

/// Holder tag for the QualityLevel placement_any (typeinfo name, IDA 0xd084 shape).
pub struct QualityLevelHolder {
    pub type_name: &'static str,
    pub construct: fn(*const i32, *mut i32) -> i32,
    pub destruct: fn(),
}

static QUALITY_LEVEL_HOLDER: LazyLock<QualityLevelHolder> = LazyLock::new(|| QualityLevelHolder {
    type_name: "N3RBX15CRenderSettings12QualityLevelE",
    construct: stub_cfa8,
    destruct: stub_cfb4,
});

/// Holder tag for the ResolutionPreset placement_any (typeinfo name, IDA 0xcb2c).
pub struct ResolutionPresetHolder {
    pub type_name: &'static str,
    pub construct: fn(*const i32, *mut i32) -> i32,
    pub destruct: fn(),
}

static RESOLUTION_PRESET_HOLDER: LazyLock<ResolutionPresetHolder> =
    LazyLock::new(|| ResolutionPresetHolder {
        type_name: "N3RBX15CRenderSettings16ResolutionPresetE",
        construct: stub_c9c8,
        destruct: stub_c9d4,
    });

/// Holder tag for the AntialiasingMode placement_any (typeinfo name, IDA 0xdb8c).
pub struct AntialiasingModeHolder {
    pub type_name: &'static str,
    pub construct: fn(*const i32, *mut i32) -> i32,
    pub destruct: fn(),
}

static ANTIALIASING_MODE_HOLDER: LazyLock<AntialiasingModeHolder> =
    LazyLock::new(|| AntialiasingModeHolder {
        type_name: "N3RBX15CRenderSettings16AntialiasingModeE",
        construct: stub_da28,
        destruct: stub_da34,
    });

/// Holder tag for the FrameRateManagerMode placement_any (typeinfo name per mangled length 20).
pub struct FrameRateManagerModeHolder {
    pub type_name: &'static str,
    pub construct: fn(*const i32, *mut i32) -> i32,
    pub destruct: fn(),
}

static FRAME_RATE_MANAGER_MODE_HOLDER: LazyLock<FrameRateManagerModeHolder> =
    LazyLock::new(|| FrameRateManagerModeHolder {
        type_name: "N3RBX15CRenderSettings20FrameRateManagerModeE",
        construct: stub_df68,
        destruct: stub_df74,
    });

/// Holder tag for the GraphicsMode placement_any (typeinfo name per mangled length 12).
pub struct GraphicsModeHolder {
    pub type_name: &'static str,
    pub construct: fn(*const i32, *mut i32) -> i32,
    pub destruct: fn(),
}

static GRAPHICS_MODE_HOLDER: LazyLock<GraphicsModeHolder> = LazyLock::new(|| GraphicsModeHolder {
    type_name: "N3RBX15CRenderSettings12GraphicsModeE",
    construct: stub_e4a8,
    destruct: stub_e4b4,
});

// 0xc154 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::~EnumDesc()")]
pub fn stub_c154() {
    // IDA 0xc154: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xc158 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::~EnumDesc()")]
pub fn stub_c158() {
    // IDA 0xc158: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xc16c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::lookup(char const*)const")]
pub fn stub_c16c(name: &str, value_out: &mut i32) -> u32 {
    // IDA 0xc16c: Name::lookup (0xc178, host: the &str itself), then
    // convertToValue(Name) (0xc186 = stub_d6b4); on success convertToItem
    // (0xc192 = stub_d4f8), else 0 (0xc188/0xc198).
    if stub_d6b4(name, value_out) {
        stub_d4f8(*value_out)
    } else {
        0
    }
}

// 0xc19c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_c19c(any: &RenderSettingsAny) -> Result<u32, RenderEnumCastError> {
    // IDA 0xc19c: any_cast<const ShadowMode&> (0xc1ae, throws bad_cast on
    // mismatch) then convertToItem (0xc1b8 = stub_d4f8).
    let value = stub_d5c4(any)?;
    Ok(stub_d4f8(*value))
}

// 0xc1bc — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_c1bc(index: u32, out: &mut i32) -> bool {
    // IDA 0xc1bc (disasm): ok = false (0xc1c6); count = [this,#0x28] (0xc1c4),
    // HI (index < count) -> tmp = table(+0x90)[index], ok = true
    // (0xc1c8..0xc1d6); Singleton call_once + doGetSingleton (0xc1d8..0xc204),
    // out = singleton-typed Variant via placement op= (0xc206..0xc20c, host:
    // the i32 assignment below carries the ShadowMode tag); return ok (0xc210).
    let _ = stub_d47c();
    if (index as usize) < SHADOW_MODE_ITEMS.len() {
        *out = SHADOW_MODE_ITEMS[index as usize].1;
        true
    } else {
        false
    }
}

// 0xc218 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToString(unsigned long,std::string &)const")]
pub fn stub_c218(index: u32, out: &mut String) -> bool {
    // IDA 0xc218: if (*(this+40) > index) (0xc26c): item = *(this+144)[index]
    // (0xc27c, same table as 0xc1bc), convertToString(item) into a temp
    // (0xc286 = the 0xd28c overload), string::assign (0xc292), destroy the
    // temp (0xc2a4..0xc2f0, host: String drop) and return 1; else return 0.
    if (index as usize) < SHADOW_MODE_ITEMS.len() {
        stub_d28c(SHADOW_MODE_ITEMS[index as usize].1, out);
        true
    } else {
        false
    }
}

// 0xc35c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::~EnumDesc()")]
pub fn stub_c35c() {
    // IDA 0xc35c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xc360 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::~EnumDesc()")]
pub fn stub_c360() {
    // IDA 0xc360: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xc374 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::lookup(char const*)const")]
pub fn stub_c374(name: &str, value_out: &mut i32) -> u32 {
    // IDA 0xc374: same lookup(char const*) shape as 0xc16c — Name::lookup
    // (0xc380), convertToValue(Name) (0xc38e = stub_d174); success ->
    // convertToItem (0xc39a = stub_cfb8), else 0.
    if stub_d174(name, value_out) {
        stub_cfb8(*value_out)
    } else {
        0
    }
}

// 0xc3a4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_c3a4(any: &RenderSettingsAny) -> Result<u32, RenderEnumCastError> {
    // IDA 0xc3a4: any_cast<const QualityLevel&> (0xc3b6) then convertToItem
    // (0xc3c0 = stub_cfb8). Same shape as 0xc19c.
    let value = stub_d084(any)?;
    Ok(stub_cfb8(*value))
}

// 0xc3c4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_c3c4(index: u32, out: &mut i32) -> bool {
    // IDA 0xc3c4 (disasm): same convertToValue(ulong) shape as 0xc1bc —
    // count(+0x28)/table(+0x90) check, Singleton init, typed assign, ok flag.
    let _ = stub_cf3c();
    if (index as usize) < QUALITY_LEVEL_ITEMS.len() {
        *out = QUALITY_LEVEL_ITEMS[index as usize].1;
        true
    } else {
        false
    }
}

// 0xc420 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToString(unsigned long,std::string &)const")]
pub fn stub_c420(index: u32, out: &mut String) -> bool {
    // IDA 0xc420: same convertToString(ulong) shape as 0xc218 — table lookup,
    // item overload (the 0xcd4c overload) into a temp, assign, return 1.
    if (index as usize) < QUALITY_LEVEL_ITEMS.len() {
        stub_cd4c(QUALITY_LEVEL_ITEMS[index as usize].1, out);
        true
    } else {
        false
    }
}

// 0xc564 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::~EnumDesc()")]
pub fn stub_c564() {
    // IDA 0xc564: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xc568 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::~EnumDesc()")]
pub fn stub_c568() {
    // IDA 0xc568: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xc57c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::lookup(char const*)const")]
pub fn stub_c57c(name: &str, value_out: &mut i32) -> u32 {
    // IDA 0xc57c: same lookup(char const*) shape as 0xc16c — Name::lookup
    // (0xc588), convertToValue(Name) (0xc596 = stub_cc34); success ->
    // convertToItem (0xc5a2 = stub_c9d8), else 0.
    if stub_cc34(name, value_out) {
        stub_c9d8(*value_out)
    } else {
        0
    }
}

// 0xc5ac — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_c5ac(any: &RenderSettingsAny) -> Result<u32, RenderEnumCastError> {
    // IDA 0xc5ac: any_cast<const ResolutionPreset&> (0xc5be) then
    // convertToItem (0xc5c8 = stub_c9d8). Same shape as 0xc19c.
    let value = stub_caa4(any)?;
    Ok(stub_c9d8(*value))
}

// 0xc5cc — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_c5cc(index: u32, out: &mut i32) -> bool {
    // IDA 0xc5cc (disasm): same convertToValue(ulong) shape as 0xc1bc —
    // count(+0x28)/table(+0x90) check, Singleton init, typed assign, ok flag.
    let _ = stub_c95c();
    if (index as usize) < RESOLUTION_PRESET_ITEMS.len() {
        *out = RESOLUTION_PRESET_ITEMS[index as usize].1;
        true
    } else {
        false
    }
}

// 0xc628 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToString(unsigned long,std::string &)const")]
pub fn stub_c628(index: u32, out: &mut String) -> bool {
    // IDA 0xc628: same convertToString(ulong) shape as 0xc218 — table lookup,
    // item overload (the 0xc76c overload) into a temp, assign, return 1.
    if (index as usize) < RESOLUTION_PRESET_ITEMS.len() {
        stub_c76c(RESOLUTION_PRESET_ITEMS[index as usize].1, out);
        true
    } else {
        false
    }
}

// 0xc76c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToString(RBX::CRenderSettings::ResolutionPreset const&)const")]
pub fn stub_c76c(value: i32, out: &mut String) {
    // IDA 0xc76c: FLog::Asserts-gated ReleaseAsserts "value>=0"
    // (enumconverter.h:262, 0xc7a8..0xc818) and
    // "(size_t)value<enumToItem.size()" (:263, 0xc81c..0xc874; host: panic).
    // Then value <= -1 -> "" (0xc8ae); value >= names.size -> "" (0xc8c6);
    // else names[value] (0xc896).
    if flog_asserts() {
        assert!(
            value >= 0,
            "value>=0 file: include/reflection/enumconverter.h line: 262"
        );
        assert!(
            (value as usize) < RESOLUTION_PRESET_ITEMS.len(),
            "(size_t)value<enumToItem.size() file: include/reflection/enumconverter.h line: 263"
        );
    }
    if value < 0 || (value as usize) >= RESOLUTION_PRESET_ITEMS.len() {
        out.clear();
    } else {
        out.clear();
        out.push_str(RESOLUTION_PRESET_ITEMS[value as usize].0);
    }
}

// 0xc90c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings16ResolutionPresetEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::ResolutionPreset>(RBX::CRenderSettings::ResolutionPreset const&)")]
pub fn stub_c90c<'a>(slot: &'a mut RenderSettingsAny, value: i32) -> &'a mut RenderSettingsAny {
    // IDA 0xc90c: singleton() (0xc918); same holder -> payload word copy
    // (0xc944); else destroy the old payload via its holder (0xc930..0xc93c,
    // host: enum drop) then store + retag (0xc94e/0xc950); return self (0xc958).
    // Trivial enum: no-op dtor.
    match &mut *slot {
        RenderSettingsAny::ResolutionPreset(current) => {
            *current = value;
        }
        other => {
            *other = RenderSettingsAny::ResolutionPreset(value);
        }
    }
    slot
}

// 0xc95c — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16ResolutionPresetEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ResolutionPreset>::singleton(void)")]
pub fn stub_c95c() -> &'static ResolutionPresetHolder {
    // IDA 0xc95c: cxa_guard_acquire/release around s (0xc976..0xc9b6);
    // s = {typeinfo, destruct_func} + construct_func word (0xc9ae/0xc9b2).
    // Host: LazyLock never drops (atexit equivalent).
    &*RESOLUTION_PRESET_HOLDER
}

// 0xc9c8 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16ResolutionPresetEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
pub fn stub_c9c8(src: *const i32, dst: *mut i32) -> i32 {
    // IDA 0xc9c8: null dst -> return src word untouched (0xc9ca/0xc9d0);
    // else *dst = loaded word (0xc9cc/0xc9ce, trivial 4-byte enum copy).
    // The original returns the loaded word verbatim; host returns it by value.
    // SAFETY: holder protocol guarantees src readable and dst writable-or-null.
    let value = unsafe { src.read() };
    if !dst.is_null() {
        unsafe {
            dst.write(value);
        }
    }
    value
}

// 0xc9d4 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16ResolutionPresetEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ResolutionPreset>::destruct_func(char *)")]
pub fn stub_c9d4() {
    // IDA 0xc9d4: BX LR — trivial enum, nothing to destroy.
}

// 0xc9d8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToItem(RBX::CRenderSettings::ResolutionPreset const&)const")]
pub fn stub_c9d8(value: i32) -> u32 {
    // IDA 0xc9d8: same assert pair as 0xc76c at enumconverter.h:273/274
    // (0xc9ec..0xca32/0xca36..0xca74, host: panic); then value < 0 -> 0,
    // out of range -> 0, else enumToItem[value] (0xca84..0xca9c, dense identity).
    // NOTE: failure returns 0, which collides with item 0 — as in the original.
    if flog_asserts() {
        assert!(
            value >= 0,
            "value>=0 file: include/reflection/enumconverter.h line: 273"
        );
        assert!(
            (value as usize) < RESOLUTION_PRESET_ITEMS.len(),
            "(size_t)value<enumToItem.size() file: include/reflection/enumconverter.h line: 274"
        );
    }
    if value >= 0 && (value as usize) < RESOLUTION_PRESET_ITEMS.len() {
        value as u32
    } else {
        0
    }
}

// 0xcaa4 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings16ResolutionPresetENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CRenderSettings::ResolutionPreset const& rbx::any_cast<RBX::CRenderSettings::ResolutionPreset const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_caa4(slot: &RenderSettingsAny) -> Result<&i32, RenderEnumCastError> {
    // IDA 0xcaa4: null holder -> void typeinfo (0xcace..0xcb00); holder or
    // name ("N3RBX15CRenderSettings16ResolutionPresetE", 0xcb10..0xcb2c)
    // mismatch -> throw bad_placement_any_cast (0xcb5a..0xcb62, host: Err);
    // else payload at +1 (0xcb4a). Host: the enum tag subsumes both checks.
    match slot {
        RenderSettingsAny::ResolutionPreset(value) => Ok(value),
        _ => Err(RenderEnumCastError::BadPlacementAnyCast),
    }
}

// 0xcb94 — __ZN5boost16exception_detail12refcount_ptrINS0_20error_info_containerEED2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "boost::exception_detail::refcount_ptr<boost::exception_detail::error_info_container>::~refcount_ptr()")]
pub fn stub_cb94() {
    // IDA 0xcb94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xcc34 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToValue(RBX::Name const&,RBX::CRenderSettings::ResolutionPreset&)const")]
pub fn stub_cc34(name: &str, out: &mut i32) -> bool {
    // IDA 0xcc34: lower_bound walk over the value map (0xcc4a..0xcc6c) then the
    // legacy map (0xcc7e..0xcca2) with exact-key rechecks (0xcc66/0xcc9a);
    // hit stores the value word (node+5, 0xccaa) and returns 1, else 0.
    // Host: binary search, primary table then legacy table.
    match RESOLUTION_PRESET_BY_NAME.binary_search_by(|probe| probe.0.cmp(name)) {
        Ok(found) => {
            *out = RESOLUTION_PRESET_BY_NAME[found].1;
            true
        }
        Err(_) => match RESOLUTION_PRESET_LEGACY_BY_NAME
            .binary_search_by(|probe| probe.0.cmp(name))
        {
            Ok(found) => {
                *out = RESOLUTION_PRESET_LEGACY_BY_NAME[found].1;
                true
            }
            Err(_) => false,
        },
    }
}

// 0xccb0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::~EnumDesc()")]
pub fn stub_ccb0() {
    // IDA 0xccb0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xcd4c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToString(RBX::CRenderSettings::QualityLevel const&)const")]
pub fn stub_cd4c(value: i32, out: &mut String) {
    // IDA 0xcd4c: same convertToString(item) shape as 0xc76c — asserts at
    // enumconverter.h:262/263, then "" or names[value].
    if flog_asserts() {
        assert!(
            value >= 0,
            "value>=0 file: include/reflection/enumconverter.h line: 262"
        );
        assert!(
            (value as usize) < QUALITY_LEVEL_ITEMS.len(),
            "(size_t)value<enumToItem.size() file: include/reflection/enumconverter.h line: 263"
        );
    }
    if value < 0 || (value as usize) >= QUALITY_LEVEL_ITEMS.len() {
        out.clear();
    } else {
        out.clear();
        out.push_str(QUALITY_LEVEL_ITEMS[value as usize].0);
    }
}

// 0xceec — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings12QualityLevelEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::QualityLevel>(RBX::CRenderSettings::QualityLevel const&)")]
pub fn stub_ceec<'a>(slot: &'a mut RenderSettingsAny, value: i32) -> &'a mut RenderSettingsAny {
    // IDA 0xceec: same placement_any op= shape as 0xc90c — singleton()
    // (0xcef8); same holder -> payload copy (0xcf24); else destroy + retag
    // (0xcf10..0xcf1c/0xcf2e..); return self. Trivial enum: no-op dtor.
    match &mut *slot {
        RenderSettingsAny::QualityLevel(current) => {
            *current = value;
        }
        other => {
            *other = RenderSettingsAny::QualityLevel(value);
        }
    }
    slot
}

// 0xcf3c — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12QualityLevelEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::QualityLevel>::singleton(void)")]
pub fn stub_cf3c() -> &'static QualityLevelHolder {
    // IDA 0xcf3c: cxa_guard_acquire/release around s (0xcf56..); s =
    // {typeinfo, destruct_func} + construct_func word (0xcf8e..). Host:
    // LazyLock never drops (atexit equivalent).
    &*QUALITY_LEVEL_HOLDER
}

// 0xcfa8 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12QualityLevelEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::QualityLevel>::construct_func(char const*,char *)")]
pub fn stub_cfa8(src: *const i32, dst: *mut i32) -> i32 {
    // IDA 0xcfa8: null dst -> return src word untouched (0xcfaa/0xcfb0);
    // else *dst = loaded word (0xcfac/0xcfae, trivial 4-byte enum copy).
    // SAFETY: holder protocol guarantees src readable and dst writable-or-null.
    let value = unsafe { src.read() };
    if !dst.is_null() {
        unsafe {
            dst.write(value);
        }
    }
    value
}

// 0xcfb4 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12QualityLevelEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::QualityLevel>::destruct_func(char *)")]
pub fn stub_cfb4() {
    // IDA 0xcfb4: BX LR — trivial enum, nothing to destroy.
}

// 0xcfb8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToItem(RBX::CRenderSettings::QualityLevel const&)const")]
pub fn stub_cfb8(value: i32) -> u32 {
    // IDA 0xcfb8: same assert pair as 0xc76c at enumconverter.h:273/274
    // (0xcfcc..0xd012/0xd036..; host: panic); then value < 0 -> 0, out of
    // range -> 0, else enumToItem[value] (dense identity).
    // NOTE: failure returns 0, which collides with item 0 — as in the original.
    if flog_asserts() {
        assert!(
            value >= 0,
            "value>=0 file: include/reflection/enumconverter.h line: 273"
        );
        assert!(
            (value as usize) < QUALITY_LEVEL_ITEMS.len(),
            "(size_t)value<enumToItem.size() file: include/reflection/enumconverter.h line: 263"
        );
    }
    if value >= 0 && (value as usize) < QUALITY_LEVEL_ITEMS.len() {
        value as u32
    } else {
        0
    }
}

// 0xd084 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings12QualityLevelENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CRenderSettings::QualityLevel const& rbx::any_cast<RBX::CRenderSettings::QualityLevel const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_d084(slot: &RenderSettingsAny) -> Result<&i32, RenderEnumCastError> {
    // IDA 0xd084: same any_cast shape as 0xcaa4 — null holder -> void
    // typeinfo; holder or name ("N3RBX15CRenderSettings12QualityLevelE")
    // mismatch -> throw bad_placement_any_cast (host: Err); else payload.
    match slot {
        RenderSettingsAny::QualityLevel(value) => Ok(value),
        _ => Err(RenderEnumCastError::BadPlacementAnyCast),
    }
}

// 0xd174 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToValue(RBX::Name const&,RBX::CRenderSettings::QualityLevel&)const")]
pub fn stub_d174(name: &str, out: &mut i32) -> bool {
    // IDA 0xd174: same two-map lower_bound shape as 0xcc34 (walks
    // 0xd18a..0xd1a6 then 0xd1b8..; hit stores node+5 and returns 1, else 0).
    // Host: binary search, primary table then legacy table.
    match QUALITY_LEVEL_BY_NAME.binary_search_by(|probe| probe.0.cmp(name)) {
        Ok(found) => {
            *out = QUALITY_LEVEL_BY_NAME[found].1;
            true
        }
        Err(_) => match QUALITY_LEVEL_LEGACY_BY_NAME.binary_search_by(|probe| probe.0.cmp(name)) {
            Ok(found) => {
                *out = QUALITY_LEVEL_LEGACY_BY_NAME[found].1;
                true
            }
            Err(_) => false,
        },
    }
}

// 0xd1f0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::~EnumDesc()")]
pub fn stub_d1f0() {
    // IDA 0xd1f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd28c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToString(RBX::CRenderSettings::ShadowMode const&)const")]
pub fn stub_d28c(value: i32, out: &mut String) {
    // IDA 0xd28c: same convertToString(item) shape as 0xc76c — asserts at
    // enumconverter.h:262 (0xd2c8..0xd338) / :263, then "" or names[value]
    // (0xd388..0xd3d0 shape).
    if flog_asserts() {
        assert!(
            value >= 0,
            "value>=0 file: include/reflection/enumconverter.h line: 262"
        );
        assert!(
            (value as usize) < SHADOW_MODE_ITEMS.len(),
            "(size_t)value<enumToItem.size() file: include/reflection/enumconverter.h line: 263"
        );
    }
    if value < 0 || (value as usize) >= SHADOW_MODE_ITEMS.len() {
        out.clear();
    } else {
        out.clear();
        out.push_str(SHADOW_MODE_ITEMS[value as usize].0);
    }
}

// 0xd42c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings10ShadowModeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::ShadowMode>(RBX::CRenderSettings::ShadowMode const&)")]
pub fn stub_d42c<'a>(slot: &'a mut RenderSettingsAny, value: i32) -> &'a mut RenderSettingsAny {
    // IDA 0xd42c: same placement_any op= shape as 0xc90c — singleton(); same
    // holder -> payload copy; else destroy + retag; return self.
    // Trivial enum: no-op dtor.
    match &mut *slot {
        RenderSettingsAny::ShadowMode(current) => {
            *current = value;
        }
        other => {
            *other = RenderSettingsAny::ShadowMode(value);
        }
    }
    slot
}

// 0xd47c — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings10ShadowModeEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ShadowMode>::singleton(void)")]
pub fn stub_d47c() -> &'static ShadowModeHolder {
    // IDA 0xd47c: cxa_guard_acquire/release around s; s = {typeinfo,
    // destruct_func} + construct_func word. Host: LazyLock never drops.
    &*SHADOW_MODE_HOLDER
}

// 0xd4e8 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings10ShadowModeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ShadowMode>::construct_func(char const*,char *)")]
pub fn stub_d4e8(src: *const i32, dst: *mut i32) -> i32 {
    // IDA 0xd4e8: null dst -> return src word untouched (0xd4ea/0xd4f0);
    // else *dst = loaded word (0xd4ec/0xd4ee, trivial 4-byte enum copy).
    // SAFETY: holder protocol guarantees src readable and dst writable-or-null.
    let value = unsafe { src.read() };
    if !dst.is_null() {
        unsafe {
            dst.write(value);
        }
    }
    value
}

// 0xd4f4 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings10ShadowModeEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ShadowMode>::destruct_func(char *)")]
pub fn stub_d4f4() {
    // IDA 0xd4f4: BX LR — trivial enum, nothing to destroy.
}

// 0xd4f8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToItem(RBX::CRenderSettings::ShadowMode const&)const")]
pub fn stub_d4f8(value: i32) -> u32 {
    // IDA 0xd4f8: asserts "value>=0" (enumconverter.h:273, 0xd50c..0xd552)
    // and "(size_t)value<enumToItem.size()" (:274, 0xd556..) then value < 0
    // -> 0 (0xd5a6..0xd5aa), out of range -> 0, else enumToItem[value]
    // (0xd5a4..0xd5c0, dense identity). Failure collides with item 0, as original.
    if flog_asserts() {
        assert!(
            value >= 0,
            "value>=0 file: include/reflection/enumconverter.h line: 273"
        );
        assert!(
            (value as usize) < SHADOW_MODE_ITEMS.len(),
            "(size_t)value<enumToItem.size() file: include/reflection/enumconverter.h line: 274"
        );
    }
    if value >= 0 && (value as usize) < SHADOW_MODE_ITEMS.len() {
        value as u32
    } else {
        0
    }
}

// 0xd5c4 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings10ShadowModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CRenderSettings::ShadowMode const& rbx::any_cast<RBX::CRenderSettings::ShadowMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_d5c4(slot: &RenderSettingsAny) -> Result<&i32, RenderEnumCastError> {
    // IDA 0xd5c4: null holder -> void typeinfo (0xd5ee..0xd620); holder or
    // name ("N3RBX15CRenderSettings10ShadowModeE", 0xd630..0xd64c) mismatch
    // -> throw bad_placement_any_cast (0xd67a.., host: Err); else payload.
    match slot {
        RenderSettingsAny::ShadowMode(value) => Ok(value),
        _ => Err(RenderEnumCastError::BadPlacementAnyCast),
    }
}

// 0xd6b4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::ShadowMode&)const")]
pub fn stub_d6b4(name: &str, out: &mut i32) -> bool {
    // IDA 0xd6b4: lower_bound walk over the value map (0xd6ca..0xd6ec) then
    // the legacy map (0xd6fe..0xd722) with exact-key rechecks (0xd6e6/0xd71a);
    // hit stores the value word (node+5, 0xd72a) and returns 1, else 0.
    // Host: binary search, primary table then legacy table.
    match SHADOW_MODE_BY_NAME.binary_search_by(|probe| probe.0.cmp(name)) {
        Ok(found) => {
            *out = SHADOW_MODE_BY_NAME[found].1;
            true
        }
        Err(_) => match SHADOW_MODE_LEGACY_BY_NAME.binary_search_by(|probe| probe.0.cmp(name)) {
            Ok(found) => {
                *out = SHADOW_MODE_LEGACY_BY_NAME[found].1;
                true
            }
            Err(_) => false,
        },
    }
}

// 0xd730 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::~EnumDesc()")]
pub fn stub_d730() {
    // IDA 0xd730: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd7cc — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToString(RBX::CRenderSettings::AntialiasingMode const&)const")]
pub fn stub_d7cc(value: i32, out: &mut String) {
    // IDA 0xd7cc: same convertToString(item) shape as 0xc76c — asserts at
    // enumconverter.h:262 (0xd808/0xd828 skip; 0xd860/0xd864/0xd878 hook path)
    // and :263 (0xd87c/0xd88c skip; 0xd89c/0xd8bc/0xd8be hook, 0xd8c0
    // ReleaseAssert), then "" or names[value].
    if flog_asserts() {
        assert!(
            value >= 0,
            "value>=0 file: include/reflection/enumconverter.h line: 262"
        );
        assert!(
            (value as usize) < ANTIALIASING_MODE_ITEMS.len(),
            "(size_t)value<enumToItem.size() file: include/reflection/enumconverter.h line: 263"
        );
    }
    if value < 0 || (value as usize) >= ANTIALIASING_MODE_ITEMS.len() {
        out.clear();
    } else {
        out.clear();
        out.push_str(ANTIALIASING_MODE_ITEMS[value as usize].0);
    }
}

// 0xd96c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings16AntialiasingModeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::AntialiasingMode>(RBX::CRenderSettings::AntialiasingMode const&)")]
pub fn stub_d96c<'a>(slot: &'a mut RenderSettingsAny, value: i32) -> &'a mut RenderSettingsAny {
    // IDA 0xd96c: singleton() (0xd978); holder load (0xd984); same holder?
    // (0xd98c) -> payload word copy (0xd9a4); else destroy the old payload via
    // its holder (0xd990..0xd998) and null the tag (0xd99c), then store +
    // retag (0xd9ae/0xd9b0); return self (0xd9b8). Trivial enum: no-op dtor.
    match &mut *slot {
        RenderSettingsAny::AntialiasingMode(current) => {
            *current = value;
        }
        other => {
            *other = RenderSettingsAny::AntialiasingMode(value);
        }
    }
    slot
}

// 0xd9bc — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16AntialiasingModeEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AntialiasingMode>::singleton(void)")]
pub fn stub_d9bc() -> &'static AntialiasingModeHolder {
    // IDA 0xd9bc: cxa_guard_acquire/release around s (0xd9d6..); s =
    // {typeinfo, destruct_func} (0xda0e..) + construct_func word into
    // dword_12217A0 (0xda12/0xda16); return s (0xda26). Host: LazyLock.
    &*ANTIALIASING_MODE_HOLDER
}

// 0xda28 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16AntialiasingModeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AntialiasingMode>::construct_func(char const*,char *)")]
pub fn stub_da28(src: *const i32, dst: *mut i32) -> i32 {
    // IDA 0xda28: null dst -> return src word untouched (0xda2a/0xda30);
    // else *dst = loaded word (0xda2c/0xda2e, trivial 4-byte enum copy).
    // SAFETY: holder protocol guarantees src readable and dst writable-or-null.
    let value = unsafe { src.read() };
    if !dst.is_null() {
        unsafe {
            dst.write(value);
        }
    }
    value
}

// 0xda34 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16AntialiasingModeEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AntialiasingMode>::destruct_func(char *)")]
pub fn stub_da34() {
    // IDA 0xda34: BX LR — trivial enum, nothing to destroy.
}

// 0xda38 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToItem(RBX::CRenderSettings::AntialiasingMode const&)const")]
pub fn stub_da38(value: i32) -> u32 {
    // IDA 0xda38: asserts "value>=0" (enumconverter.h:273, 0xda4c/0xda50 skip;
    // 0xda82/0xda86 hook, 0xda92 onward) and
    // "(size_t)value<enumToItem.size()" (:274, 0xda96..; 0xdab6/0xdad0 hook,
    // 0xdad2 skip, 0xdad4 reload, ReleaseAssert); then 0 (0xdae6), range
    // recheck (0xdaec..), else enumToItem[value] (0xdafc). Failure -> 0, as original.
    if flog_asserts() {
        assert!(
            value >= 0,
            "value>=0 file: include/reflection/enumconverter.h line: 273"
        );
        assert!(
            (value as usize) < ANTIALIASING_MODE_ITEMS.len(),
            "(size_t)value<enumToItem.size() file: include/reflection/enumconverter.h line: 274"
        );
    }
    if value >= 0 && (value as usize) < ANTIALIASING_MODE_ITEMS.len() {
        value as u32
    } else {
        0
    }
}

// 0xdb04 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings16AntialiasingModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CRenderSettings::AntialiasingMode const& rbx::any_cast<RBX::CRenderSettings::AntialiasingMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_db04(slot: &RenderSettingsAny) -> Result<&i32, RenderEnumCastError> {
    // IDA 0xdb04: holder load (0xdb2e); null holder -> void typeinfo
    // (0xdb60); holder mismatch (0xdb70) or name mismatch
    // ("N3RBX15CRenderSettings16AntialiasingModeE", 0xdb7a/0xdb8c) ->
    // throw bad_placement_any_cast (0xdbba/0xdbc2, host: Err, 0xdbda drops
    // the bad_cast); else payload at +1 (0xdbaa).
    match slot {
        RenderSettingsAny::AntialiasingMode(value) => Ok(value),
        _ => Err(RenderEnumCastError::BadPlacementAnyCast),
    }
}

// 0xdbf4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::AntialiasingMode&)const")]
pub fn stub_dbf4(name: &str, out: &mut i32) -> bool {
    // IDA 0xdbf4: same two-map lower_bound shape as 0xd6b4 (walks
    // 0xdbf6..0xdc28; hit stores node+5 and returns 1, else 0).
    // Host: binary search, primary table then legacy table.
    match ANTIALIASING_MODE_BY_NAME.binary_search_by(|probe| probe.0.cmp(name)) {
        Ok(found) => {
            *out = ANTIALIASING_MODE_BY_NAME[found].1;
            true
        }
        Err(_) => match ANTIALIASING_MODE_LEGACY_BY_NAME.binary_search_by(|probe| probe.0.cmp(name)) {
            Ok(found) => {
                *out = ANTIALIASING_MODE_LEGACY_BY_NAME[found].1;
                true
            }
            Err(_) => false,
        },
    }
}

// 0xdc70 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc()")]
pub fn stub_dc70() {
    // IDA 0xdc70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xdd0c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToString(RBX::CRenderSettings::FrameRateManagerMode const&)const")]
pub fn stub_dd0c(value: i32, out: &mut String) {
    // IDA 0xdd0c: same convertToString(item) shape as 0xd7cc — asserts at
    // enumconverter.h:262/263 (0xdd48..0xddb8 hook/skip paths), then "" or
    // names[value].
    if flog_asserts() {
        assert!(
            value >= 0,
            "value>=0 file: include/reflection/enumconverter.h line: 262"
        );
        assert!(
            (value as usize) < FRAME_RATE_MANAGER_MODE_ITEMS.len(),
            "(size_t)value<enumToItem.size() file: include/reflection/enumconverter.h line: 263"
        );
    }
    if value < 0 || (value as usize) >= FRAME_RATE_MANAGER_MODE_ITEMS.len() {
        out.clear();
    } else {
        out.clear();
        out.push_str(FRAME_RATE_MANAGER_MODE_ITEMS[value as usize].0);
    }
}

// 0xdeac — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings20FrameRateManagerModeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::FrameRateManagerMode>(RBX::CRenderSettings::FrameRateManagerMode const&)")]
pub fn stub_deac<'a>(slot: &'a mut RenderSettingsAny, value: i32) -> &'a mut RenderSettingsAny {
    // IDA 0xdeac: same placement_any op= shape as 0xd96c — singleton(); same
    // holder -> payload copy; else destroy + retag; return self.
    // Trivial enum: no-op dtor.
    match &mut *slot {
        RenderSettingsAny::FrameRateManagerMode(current) => {
            *current = value;
        }
        other => {
            *other = RenderSettingsAny::FrameRateManagerMode(value);
        }
    }
    slot
}

// 0xdefc — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings20FrameRateManagerModeEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::FrameRateManagerMode>::singleton(void)")]
pub fn stub_defc() -> &'static FrameRateManagerModeHolder {
    // IDA 0xdefc: cxa_guard_acquire/release around s; s = {typeinfo,
    // destruct_func} + construct_func word. Host: LazyLock never drops.
    &*FRAME_RATE_MANAGER_MODE_HOLDER
}

// 0xdf68 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings20FrameRateManagerModeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::FrameRateManagerMode>::construct_func(char const*,char *)")]
pub fn stub_df68(src: *const i32, dst: *mut i32) -> i32 {
    // IDA 0xdf68: same construct_func shape as 0xda28 — null dst -> src word;
    // else 4-byte copy. SAFETY: holder protocol on src/dst as for 0xda28.
    let value = unsafe { src.read() };
    if !dst.is_null() {
        unsafe {
            dst.write(value);
        }
    }
    value
}

// 0xdf74 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings20FrameRateManagerModeEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::FrameRateManagerMode>::destruct_func(char *)")]
pub fn stub_df74() {
    // IDA 0xdf74: BX LR — trivial enum, nothing to destroy.
}

// 0xdf78 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToItem(RBX::CRenderSettings::FrameRateManagerMode const&)const")]
pub fn stub_df78(value: i32) -> u32 {
    // IDA 0xdf78: asserts "value>=0" (enumconverter.h:273, 0xdf8c/0xdf90 skip;
    // 0xdfc2/0xdfc6 hook, 0xdfd2 onward) and
    // "(size_t)value<enumToItem.size()" (:274, 0xdfd6..; 0xdff6/0xe010 hook,
    // 0xe012 skip, 0xe014 reload, ReleaseAssert at 0xe024..); then 0,
    // range recheck, else enumToItem[value]. Failure -> 0, as original.
    if flog_asserts() {
        assert!(
            value >= 0,
            "value>=0 file: include/reflection/enumconverter.h line: 273"
        );
        assert!(
            (value as usize) < FRAME_RATE_MANAGER_MODE_ITEMS.len(),
            "(size_t)value<enumToItem.size() file: include/reflection/enumconverter.h line: 274"
        );
    }
    if value >= 0 && (value as usize) < FRAME_RATE_MANAGER_MODE_ITEMS.len() {
        value as u32
    } else {
        0
    }
}

// 0xe044 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings20FrameRateManagerModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CRenderSettings::FrameRateManagerMode const& rbx::any_cast<RBX::CRenderSettings::FrameRateManagerMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_e044(slot: &RenderSettingsAny) -> Result<&i32, RenderEnumCastError> {
    // IDA 0xe044: holder load (0xe06e..); null holder -> void typeinfo
    // (0xe09c/0xe0a0); holder mismatch or name mismatch
    // ("N3RBX15CRenderSettings20FrameRateManagerModeE", 0xe0b0/0xe0b4) ->
    // throw bad_placement_any_cast (host: Err); else payload at +1.
    match slot {
        RenderSettingsAny::FrameRateManagerMode(value) => Ok(value),
        _ => Err(RenderEnumCastError::BadPlacementAnyCast),
    }
}

// 0xe134 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::FrameRateManagerMode&)const")]
pub fn stub_e134(name: &str, out: &mut i32) -> bool {
    // IDA 0xe134: same two-map lower_bound shape as 0xd6b4 (walks
    // 0xe136..0xe168; hit stores node+5 and returns 1, else 0).
    // Host: binary search, primary table then legacy table.
    match FRAME_RATE_MANAGER_MODE_BY_NAME.binary_search_by(|probe| probe.0.cmp(name)) {
        Ok(found) => {
            *out = FRAME_RATE_MANAGER_MODE_BY_NAME[found].1;
            true
        }
        Err(_) => match FRAME_RATE_MANAGER_MODE_LEGACY_BY_NAME.binary_search_by(|probe| probe.0.cmp(name)) {
            Ok(found) => {
                *out = FRAME_RATE_MANAGER_MODE_LEGACY_BY_NAME[found].1;
                true
            }
            Err(_) => false,
        },
    }
}

// 0xe1b0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::~EnumDesc()")]
pub fn stub_e1b0() {
    // IDA 0xe1b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xe24c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToString(RBX::CRenderSettings::GraphicsMode const&)const")]
pub fn stub_e24c(value: i32, out: &mut String) {
    // IDA 0xe24c: same convertToString(item) shape as 0xd7cc — asserts at
    // enumconverter.h:262/263 (0xe288..0xe2f8 hook/skip paths), then "" or
    // names[value].
    if flog_asserts() {
        assert!(
            value >= 0,
            "value>=0 file: include/reflection/enumconverter.h line: 262"
        );
        assert!(
            (value as usize) < GRAPHICS_MODE_ITEMS.len(),
            "(size_t)value<enumToItem.size() file: include/reflection/enumconverter.h line: 263"
        );
    }
    if value < 0 || (value as usize) >= GRAPHICS_MODE_ITEMS.len() {
        out.clear();
    } else {
        out.clear();
        out.push_str(GRAPHICS_MODE_ITEMS[value as usize].0);
    }
}

// 0xe3ec — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings12GraphicsModeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::GraphicsMode>(RBX::CRenderSettings::GraphicsMode const&)")]
pub fn stub_e3ec<'a>(slot: &'a mut RenderSettingsAny, value: i32) -> &'a mut RenderSettingsAny {
    // IDA 0xe3ec: same placement_any op= shape as 0xd96c — singleton(); same
    // holder -> payload copy; else destroy + retag; return self.
    // Trivial enum: no-op dtor.
    match &mut *slot {
        RenderSettingsAny::GraphicsMode(current) => {
            *current = value;
        }
        other => {
            *other = RenderSettingsAny::GraphicsMode(value);
        }
    }
    slot
}

// 0xe43c — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12GraphicsModeEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::GraphicsMode>::singleton(void)")]
pub fn stub_e43c() -> &'static GraphicsModeHolder {
    // IDA 0xe43c: cxa_guard_acquire/release around s; s = {typeinfo,
    // destruct_func} + construct_func word. Host: LazyLock never drops.
    &*GRAPHICS_MODE_HOLDER
}

// 0xe4a8 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12GraphicsModeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::GraphicsMode>::construct_func(char const*,char *)")]
pub fn stub_e4a8(src: *const i32, dst: *mut i32) -> i32 {
    // IDA 0xe4a8: same construct_func shape as 0xda28 — null dst -> src word;
    // else 4-byte copy. SAFETY: holder protocol on src/dst as for 0xda28.
    let value = unsafe { src.read() };
    if !dst.is_null() {
        unsafe {
            dst.write(value);
        }
    }
    value
}

// 0xe4b4 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12GraphicsModeEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::GraphicsMode>::destruct_func(char *)")]
pub fn stub_e4b4() {
    // IDA 0xe4b4: BX LR — trivial enum, nothing to destroy.
}

// 0xe4b8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToItem(RBX::CRenderSettings::GraphicsMode const&)const")]
pub fn stub_e4b8(value: i32) -> u32 {
    // IDA 0xe4b8: asserts "value>=0" (enumconverter.h:273, 0xe4cc/0xe4d0 skip;
    // 0xe502/0xe506 hook, 0xe512 onward) and
    // "(size_t)value<enumToItem.size()" (:274, 0xe516..; 0xe536/0xe550 hook,
    // 0xe552 skip, 0xe554 reload, ReleaseAssert at 0xe564..); then 0,
    // range recheck, else enumToItem[value]. Failure -> 0, as original.
    if flog_asserts() {
        assert!(
            value >= 0,
            "value>=0 file: include/reflection/enumconverter.h line: 273"
        );
        assert!(
            (value as usize) < GRAPHICS_MODE_ITEMS.len(),
            "(size_t)value<enumToItem.size() file: include/reflection/enumconverter.h line: 274"
        );
    }
    if value >= 0 && (value as usize) < GRAPHICS_MODE_ITEMS.len() {
        value as u32
    } else {
        0
    }
}

// 0xe584 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings12GraphicsModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CRenderSettings::GraphicsMode const& rbx::any_cast<RBX::CRenderSettings::GraphicsMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_e584(slot: &RenderSettingsAny) -> Result<&i32, RenderEnumCastError> {
    // IDA 0xe584: holder load; null holder -> void typeinfo; holder mismatch
    // (0xe5ae) or name mismatch ("N3RBX15CRenderSettings12GraphicsModeE",
    // 0xe5f0..) -> throw bad_placement_any_cast (host: Err); else payload.
    match slot {
        RenderSettingsAny::GraphicsMode(value) => Ok(value),
        _ => Err(RenderEnumCastError::BadPlacementAnyCast),
    }
}

// 0xe674 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::GraphicsMode&)const")]
pub fn stub_e674(name: &str, out: &mut i32) -> bool {
    // IDA 0xe674: same two-map lower_bound shape as 0xd6b4 (walks
    // 0xe676..0xe6a8; hit stores node+5 and returns 1, else 0).
    // Host: binary search, primary table then legacy table.
    match GRAPHICS_MODE_BY_NAME.binary_search_by(|probe| probe.0.cmp(name)) {
        Ok(found) => {
            *out = GRAPHICS_MODE_BY_NAME[found].1;
            true
        }
        Err(_) => match GRAPHICS_MODE_LEGACY_BY_NAME.binary_search_by(|probe| probe.0.cmp(name)) {
            Ok(found) => {
                *out = GRAPHICS_MODE_LEGACY_BY_NAME[found].1;
                true
            }
            Err(_) => false,
        },
    }
}

// 0xe6f0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()")]
pub fn stub_e6f0() {
    // IDA 0xe6f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xe78c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToString(RBX::CRenderSettings::AASamples const&)const")]
pub fn stub_e78c() -> ! {
    todo!("0xe78c RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToString(RBX::CRenderSettings::AASamples const&)const")
}

// 0xe92c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings9AASamplesEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::AASamples>(RBX::CRenderSettings::AASamples const&)")]
pub fn stub_e92c() -> ! {
    todo!("0xe92c rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::AASamples>(RBX::CRenderSettings::AASamples const&)")
}

// 0xe97c — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings9AASamplesEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::singleton(void)")]
pub fn stub_e97c() -> ! {
    todo!("0xe97c rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::singleton(void)")
}

// 0xe9e8 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings9AASamplesEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::construct_func(char const*,char *)")]
pub fn stub_e9e8() -> ! {
    todo!("0xe9e8 rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::construct_func(char const*,char *)")
}

// 0xe9f4 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings9AASamplesEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::destruct_func(char *)")]
pub fn stub_e9f4() -> ! {
    todo!("0xe9f4 rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::destruct_func(char *)")
}

// 0xe9f8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToItem(RBX::CRenderSettings::AASamples const&)const")]
pub fn stub_e9f8() -> ! {
    todo!("0xe9f8 RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToItem(RBX::CRenderSettings::AASamples const&)const")
}

// 0xeac4 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings9AASamplesENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CRenderSettings::AASamples const& rbx::any_cast<RBX::CRenderSettings::AASamples const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_eac4() -> ! {
    todo!("0xeac4 RBX::CRenderSettings::AASamples const& rbx::any_cast<RBX::CRenderSettings::AASamples const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0xebb4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToValue(RBX::Name const&,RBX::CRenderSettings::AASamples&)const")]
pub fn stub_ebb4() -> ! {
    todo!("0xebb4 RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToValue(RBX::Name const&,RBX::CRenderSettings::AASamples&)const")
}

// 0xec30 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()")]
pub fn stub_ec30() {
    // IDA 0xec30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xeccc — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD2Ev
// type: int __fastcall(int)
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_eccc() {
    // IDA 0xeccc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xedfc — __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_edfc() -> ! {
    todo!("0xedfc __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator12getClassNameEv")
}

// 0xee84 — __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator6createEv
// type: int __fastcall(int *)
#[doc(alias = "__ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator6createEv")]
pub fn stub_ee84() -> ! {
    todo!("0xee84 __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator6createEv")
}

// 0xef04 — __ZN3RBX9CreatableINS_8InstanceEE6createI19CRenderSettingsItemEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<CRenderSettingsItem> RBX::Creatable<RBX::Instance>::create<CRenderSettingsItem>(void)")]
pub fn stub_ef04() -> ! {
    todo!("0xef04 boost::shared_ptr<CRenderSettingsItem> RBX::Creatable<RBX::Instance>::create<CRenderSettingsItem>(void)")
}

// 0xefb4 — __ZN5boost10shared_ptrI19CRenderSettingsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<CRenderSettingsItem>::shared_ptr<CRenderSettingsItem,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_efb4() -> ! {
    todo!("0xefb4 boost::shared_ptr<CRenderSettingsItem>::shared_ptr<CRenderSettingsItem,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xefd8 — __ZNK5boost6detail15sp_counted_base9use_countEv
// type: int __fastcall(boost::detail::sp_counted_base *this)
#[doc(alias = "boost::detail::sp_counted_base::use_count(void)const")]
pub fn stub_efd8() -> ! {
    todo!("0xefd8 boost::detail::sp_counted_base::use_count(void)const")
}

// 0xf098 — __ZN5boost6detail12shared_countC2IP19CRenderSettingsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f098() {
    // IDA 0xf098: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0xf198 — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_f198() {
    // IDA 0xf198: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf19c — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_f19c() {
    // IDA 0xf19c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0xf1bc — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_f1bc() {
    // IDA 0xf1bc: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0xf1d4 — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_f1d4() {
    // IDA 0xf1d4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0xf1d8 — __ZN3RBX4Name13callDoDeclareILZ15sRenderSettingsEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZ15sRenderSettingsEEEvv")]
pub fn stub_f1d8() -> ! {
    todo!("0xf1d8 __ZN3RBX4Name13callDoDeclareILZ15sRenderSettingsEEEvv")
}

// 0xf1dc — __ZN3RBX4Name9doDeclareILZ15sRenderSettingsEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZ15sRenderSettingsEEERKS0_v")]
pub fn stub_f1dc() -> ! {
    todo!("0xf1dc __ZN3RBX4Name9doDeclareILZ15sRenderSettingsEEERKS0_v")
}

// 0xf2bc — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_f2bc() -> ! {
    todo!("0xf2bc __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorC2Ev")
}

// 0xf500 — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_f500() -> ! {
    todo!("0xf500 __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE17static_getCreatorEv")
}

// 0xf704 — __ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int __fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_insert_aux(__gnu_cxx::__normal_iterator<G3D::Vector2int16*,std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>>,G3D::Vector2int16 const&)")]
pub fn stub_f704() -> ! {
    todo!("0xf704 std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_insert_aux(__gnu_cxx::__normal_iterator<G3D::Vector2int16*,std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>>,G3D::Vector2int16 const&)")
}

// 0xf7e8 — __ZNSt12_Vector_baseIN3G3D12Vector2int16ESaIS1_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_allocate(unsigned long)")]
pub fn stub_f7e8() -> ! {
    todo!("0xf7e8 std::_Vector_base<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_allocate(unsigned long)")
}

// 0xf800 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D12Vector2int16ES5_EET0_T_S7_S6_
// type: int __fastcall(int, int, int)
#[doc(alias = "G3D::Vector2int16 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector2int16 *,G3D::Vector2int16 *>(G3D::Vector2int16 *,G3D::Vector2int16 *,G3D::Vector2int16 *)")]
pub fn stub_f800() -> ! {
    todo!("0xf800 G3D::Vector2int16 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector2int16 *,G3D::Vector2int16 *>(G3D::Vector2int16 *,G3D::Vector2int16 *,G3D::Vector2int16 *)")
}
