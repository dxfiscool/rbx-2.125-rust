//! reflection — generated_bg_1 — 100 stubs EA-sorted asc global gap filler 0x84e0..0x14e00 not yet in crates/reflection (global all covered, 64601 gaps reflection; 20945 distinct before, 21045 after)
//! Source: ida/export.json (85545 funcs) global EA asc not in crates/reflection/src — next 100 uncovered for reflection-bg sorted asc
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;
use rbx_core::signal::Signal;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::LazyLock;
use std::sync::OnceLock;
use rbx_core::shared_ptr::{ControlBlockPd, CreatableInstanceDeleter};
use std::collections::BTreeMap;

// 0x84e0 — start
// type: void __fastcall __noreturn(int, int, int, int, int argc, char *argv)
#[doc(alias = "start")]
pub fn stub_0x84e0(argc: usize, argv: *const *const core::ffi::c_char) -> ! {
    // IDA 0x84e0..0x8508 (`start`, ARM): `envp = &argv[argc + 1]` (0x84e0..0x84f4);
    // skip past the terminating null (0x84f8 `LDR R4,[R3],#4` / 0x84fc `CMP`
    // / 0x8500 `BNE`); `exit(main(argc, argv, envp))` (0x8504 `BLX _main`,
    // 0x8508 `B _exit`).
    // SAFETY: the CRT sets up `argv` with `argc + 2` readable slots.
    unsafe {
        let mut envp = argv.add(argc + 1);
        while !(*envp).is_null() {
            envp = envp.add(1);
        }
        std::process::exit(hosted_main(argc, argv, envp));
    }
}

/// IDA 0x8504 `BLX _main`: the hosted entry point. Nothing in this crate
/// links it, so it reports success [INFERENCE].
fn hosted_main(
    _argc: usize,
    _argv: *const *const core::ffi::c_char,
    _envp: *const *const core::ffi::c_char,
) -> i32 {
    0
}

/// IDA 0x9608..0x9794: `CRenderSettingsItem` slots touched by this shard's leaves.
/// The original object is larger; only IDA-observed slots are modelled, with
/// byte offsets from the ARM disassembly noted per field.
#[derive(Default)]
pub struct CRenderSettingsItem {
    /// +0x64 dword. IDA 0x9608 `LDR R2,[R0,#0x64]` / `STR R1,[R0,#0x64]`.
    pub graphics_mode: i32,
    /// +0x68 dword. IDA 0x971c `LDR R2,[R0,#0x68]` / `STR R1,[R0,#0x68]`.
    pub antialiasing_mode: i32,
    /// +0x6C dword. IDA 0x96fc `LDR R2,[R0,#0x6C]` / `STR R1,[R0,#0x6C]`.
    pub shadow_mode: i32,
    /// +0x70 dword. IDA 0x9628 `LDR R2,[R0,#0x70]` / `STR R1,[R0,#0x70]`.
    pub frame_rate_manager_mode: i32,
    /// +0x74 dword. IDA 0x9648 `LDR R2,[R0,#0x74]` / `STR R1,[R0,#0x74]`.
    pub quality_level: i32,
    /// +0x78 dword. IDA 0x97a4 `LDR R2,[R0,#0x78]` / `STR R1,[R0,#0x78]`.
    pub resolution_preset: i32,
    /// +0x7C dword. IDA 0x9ac8 `LDR R2,[R0,#0x7C]` / `STR R1,[R0,#0x7C]`.
    pub auto_quality_level: i32,
    /// +0x88 byte. IDA 0x973c `LDRB.W R2,[R0,#0x88]` / `STRB.W R1,[R0,#0x88]`.
    pub debug_show_bounding_boxes: bool,
    /// +0x89 byte. IDA 0x9760 `LDRB.W R2,[R0,#0x89]` / `STRB.W R1,[R0,#0x89]`.
    pub enable_frm: bool,
    /// +0x9A byte. IDA 0x96ac `LDRB.W R2,[R0,#0x9A]` / `STRB.W R1,[R0,#0x9A]`.
    pub show_aggregation: bool,
    /// +0x9B byte: the stored `AlwaysDrawConnectors` value. IDA 0x9668 `STRB.W R1,[R0,#0x9B]`.
    pub always_draw_connectors: bool,
    /// +0x9C byte: second input of the 0x9668 effective-value compare.
    /// Role inferred from the compare logic (override clear => effective tracks this byte).
    pub always_draw_connectors_base: bool,
    /// +0x9D byte. IDA 0x9b08 `LDRB.W R2,[R0,#0x9D]` / `STRB.W R1,[R0,#0x9D]`.
    pub eager_bulk_execution: bool,
    /// +0xA0 dword, no signal. IDA 0x97c0 `STR.W R1,[R0,#0xA0]` / `BX LR`.
    pub texture_cache_size: u32,
    /// +0xA4 dword, no signal. IDA 0x97c8 `STR.W R1,[R0,#0xA4]` / `BX LR`.
    pub mesh_cache_size: u32,
    /// +146 dword video-memory budget (settings-subobject +50). IDA 0x9946
    /// `STR.W R2,[R1,#0x92]` in the 0x97d0 ctor; same slot as
    /// `CRenderSettings::video_memory_budget`.
    pub video_memory_budget: u32,
    /// +168 `std::string`, empty after construction. IDA 0x9876 points it at
    /// `std::string::_Rep::_S_empty_rep_storage`.
    pub string_168: String,
    /// +172/+174 two `u16` lanes written as `800`/`600` (IDA 0x987e/0x988a);
    /// the value pushed into `resolutions` (IDA 0x991a).
    pub first_resolution: Vector2int16,
    /// +176 `std::vector<G3D::Vector2int16>` (IDA 0x9896..0x98aa zero it).
    pub resolutions: Vec<Vector2int16>,
    /// +189 byte set to 1 by the ctor (IDA 0x98b0); role not observed.
    pub byte_189: bool,
    /// Name passed to the `+28` setter virtual (IDA 0x98f6/0x9904
    /// `std::string("Rendering")`); the 0xb4fc base ctor stores
    /// `"RenderSettings"` here first (IDA 0xb5ec/0xb5f8 `setName`).
    pub render_category: String,
    /// +0xC0: `rbx::signals::signal_with_args<1, void(const PropertyDescriptor*)>`.
    /// Every setter below tail-calls it (`ADDS R0,#0xC0`) with its own
    /// `PropertyDescriptor` (`unk_130Cxxx`); modelled by descriptor name.
    pub property_changed: Signal<&'static str>,
}

/// IDA 0x96d0: `RBX::CRenderSettings::aaSamples` — a dword global, not an item
/// field (`LDR R2,[R2]; RBX::CRenderSettings::aaSamples` via `_ptr` slot).
pub static AA_SAMPLES: AtomicI32 = AtomicI32::new(0);
/// IDA 0x9784/0x9794: `RBX::PartInstance::disableInterpolation` — a byte global.
pub static DISABLE_INTERPOLATION: AtomicBool = AtomicBool::new(false);

/// `G3D::Vector2int16`: two packed `int16` lanes. IDA 0xb740 moves one
/// element with a single 4-byte `LDR`/`STR`, so `sizeof == 4` (0xf7f6
/// `operator new(4 * n)`).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Vector2int16 {
    pub x: i16,
    pub y: i16,
}

/// IDA 0x993a: `GetDXVideoMemorySize()` is compared against `&loc_F423FC + 3`
/// (address-as-constant); above it the ctor stores the high budget.
pub const VIDEO_MEMORY_THRESHOLD: u32 = 0xF423FF;
/// IDA 0x993c: budget when video memory clears the threshold.
pub const VIDEO_BUDGET_HIGH: u32 = 50_332_672;
/// IDA 0x9926: budget otherwise.
pub const VIDEO_BUDGET_LOW: u32 = 39_322_400;

/// IDA 0x9922 `GetDXVideoMemorySize()`. The host has no DX video memory
/// query, so this reports 0 and the ctor takes the low-budget arm [INFERENCE].
fn get_dx_video_memory_size() -> u32 {
    0
}

/// IDA 0x9922..0x9946: threshold select stored at item +146 (settings +50).
fn video_memory_budget() -> u32 {
    if get_dx_video_memory_size() > VIDEO_MEMORY_THRESHOLD {
        VIDEO_BUDGET_HIGH
    } else {
        VIDEO_BUDGET_LOW
    }
}

/// `sRenderSettings` declared name (IDA 0xf1dc `doDeclare`: guard-once
/// `RBX::Name::declare`; the `sRenderSettings` text is "RenderSettings").
pub static RENDER_SETTINGS_NAME: LazyLock<String> = LazyLock::new(|| "RenderSettings".to_owned());
/// `FactoryProduct<CRenderSettingsItem,...>::Creator::isConstructedE`
/// (IDA 0xf2bc stores `666` after registering; every `wasConstructed()`
/// assert compares against it, e.g. 0xee22 `CMP R1,#0x29A`).
pub static CREATOR_IS_CONSTRUCTED: AtomicI32 = AtomicI32::new(0);
/// IDA `isConstructedE` sentinel (`0x29A`).
pub const CREATOR_CONSTRUCTED_MAGIC: i32 = 666;
/// `Class::getCreators()` registry (IDA 0xf2bc `std::map::operator[]`
/// stores the Creator under the declared name).
pub static CREATOR_REGISTRY: LazyLock<parking_lot::Mutex<Vec<&'static str>>> =
    LazyLock::new(|| parking_lot::Mutex::new(Vec::new()));
/// `GlobalAdvancedSettingsItem<CRenderSettingsItem>::sing` singleton slot
/// (IDA 0xb4fc: second construct throws; 0xb626 stores `this`).
static RENDER_SETTINGS_SING: OnceLock<usize> = OnceLock::new();
/// `RBX::Reflection::ClassRegistrar<CRenderSettingsItem>::registrar`
/// (IDA 0xb5b2 `++registrar` in the 0xb4fc base ctor).
pub static CLASS_REGISTRAR_COUNT: AtomicI32 = AtomicI32::new(0);

/// `FactoryProduct<CRenderSettingsItem,...>::Creator` (`creatorPrivate`,
/// IDA 0xf500/0xf566). Construction state lives in
/// `CREATOR_IS_CONSTRUCTED`, not in the value.
pub struct RenderSettingsCreator {
    _private: (),
}
/// IDA `creatorPrivate` singleton returned by 0xf500.
static RENDER_SETTINGS_CREATOR: RenderSettingsCreator = RenderSettingsCreator { _private: () };

/// IDA `wasConstructed()` assert prologue (0xedfc `Object.h:236`, 0xee84
/// `Object.h:231`, 0xf500 `Object.h:282`): with asserts enabled,
/// `isConstructedE == 666` or `ReleaseAssert` fires (via `FLog::Asserts` /
/// `_debugHook` in the original; `debug_assert` here).
fn assert_creator_constructed(what: &str) {
    debug_assert_eq!(
        CREATOR_IS_CONSTRUCTED.load(Ordering::SeqCst),
        CREATOR_CONSTRUCTED_MAGIC,
        "{what}"
    );
}

/// IDA 0x9668: `LDRB` + `CBNZ`/`MOVNE` folds any nonzero flag byte to 1.
/// Fields here are already `bool`, so this documents the original fold.
fn normalize_flag(value: bool) -> i32 {
    i32::from(value)
}

/// IDA 0xb33c..0xb4a4: `RBX::CRenderSettings` slots read by this shard's getters.
/// IDA 0x97d0 constructs the settings subobject at item offset +96
/// (`RBX::CRenderSettings::CRenderSettings((char *)this + 96)`), so settings
/// offset +N is item offset +96+N: settings +4 == item +0x64 (`graphics_mode`),
/// +0x10 == +0x70, +0x14 == +0x74, +0x18 == +0x78, +0x1C == +0x7C,
/// +0x28 == +0x88, +0x29 == +0x89, +0x3A == +0x9A, +0x3B == +0x9B.
/// Byte offsets below are settings-relative, from the getter disassembly.
#[derive(Default)]
pub struct CRenderSettings {
    /// +4 dword. IDA 0xb33c `LDR R0,[R0,#4]`.
    pub graphics_mode: i32,
    /// +8 dword. IDA 0xb444 `LDR R0,[R0,#8]`.
    pub antialiasing_mode: i32,
    /// +0xC dword. IDA 0xb41c `LDR R0,[R0,#0xC]`.
    pub shadow_mode: i32,
    /// +0x10 dword. IDA 0xb364 `LDR R0,[R0,#0x10]`.
    pub frame_rate_manager_mode: i32,
    /// +0x14 dword. IDA 0xb38c `LDR R0,[R0,#0x14]`.
    pub quality_level: i32,
    /// +0x18 dword. IDA 0xb4a4 `LDR R0,[R0,#0x18]`.
    pub resolution_preference: i32,
    /// +0x1C dword. IDA 0xb474 `LDR R0,[R0,#0x1C]`.
    pub auto_quality_level: i32,
    /// +0x28 byte, zero-extended into R0. IDA 0xb46c `LDRB.W R0,[R0,#0x28]`.
    pub debug_show_bounding_boxes: bool,
    /// +0x29 byte, zero-extended into R0. IDA 0xb49c `LDRB.W R0,[R0,#0x29]`.
    pub enable_frm: bool,
    /// +0x3A byte, zero-extended into R0. IDA 0xb3e0 `LDRB.W R0,[R0,#0x3A]`.
    pub show_aggregation: bool,
    /// +0x3B byte, zero-extended into R0. IDA 0xb3b4 `LDRB.W R0,[R0,#0x3B]`.
    pub always_draw_connectors: bool,
    /// +0x20 dword. IDA 0xb4cc `LDR R0,[R0,#0x20]`.
    pub max_quality_level: i32,
    /// +0x32 dword video-memory budget (item +146). Written by the item ctor
    /// (IDA 0x9946 `STR.W R2,[R1,#0x92]`); see `video_memory_budget()`.
    pub video_memory_budget: u32,
    /// +0x3D byte, zero-extended into R0. IDA 0xb8b0 `LDRB.W R0,[R0,#0x3D]`.
    pub eager_bulk_execution: bool,
    /// +0x40 dword. IDA 0xb4f4 `LDR R0,[R0,#0x40]`.
    pub texture_cache_size: u32,
    /// +0x44 dword. IDA 0xb4f8 `LDR R0,[R0,#0x44]`.
    pub mesh_cache_size: u32,
}

// 0x9608 — __ZN19CRenderSettingsItem15setGraphicsModeEN3RBX15CRenderSettings12GraphicsModeE
// type: int __fastcall(int result, int)
// IDA 0x9608: store +0x64 then fire(+0xC0, &unk_130C244) iff changed; return this.
#[doc(alias = "CRenderSettingsItem::setGraphicsMode(RBX::CRenderSettings::GraphicsMode)")]
#[doc(alias = "__ZN19CRenderSettingsItem15setGraphicsModeEN3RBX15CRenderSettings12GraphicsModeE")]
pub fn stub_0x9608(this: *mut CRenderSettingsItem, value: i32) -> *mut CRenderSettingsItem {
    // SAFETY: `this` must point to a valid `CRenderSettingsItem`.
    unsafe {
        let item = &mut *this;
        if item.graphics_mode != value {
            item.graphics_mode = value;
            item.property_changed.fire("GraphicsMode");
        }
        this
    }
}

// 0x9628 — __ZN19CRenderSettingsItem23setFrameRateManagerModeEN3RBX15CRenderSettings20FrameRateManagerModeE
// type: int __fastcall(int result, int)
// IDA 0x9628: store +0x70 then fire(+0xC0, &unk_130C278) iff changed; return this.
#[doc(alias = "CRenderSettingsItem::setFrameRateManagerMode(RBX::CRenderSettings::FrameRateManagerMode)")]
#[doc(alias = "__ZN19CRenderSettingsItem23setFrameRateManagerModeEN3RBX15CRenderSettings20FrameRateManagerModeE")]
pub fn stub_0x9628(this: *mut CRenderSettingsItem, value: i32) -> *mut CRenderSettingsItem {
    // SAFETY: `this` must point to a valid `CRenderSettingsItem`.
    unsafe {
        let item = &mut *this;
        if item.frame_rate_manager_mode != value {
            item.frame_rate_manager_mode = value;
            item.property_changed.fire("FrameRateManagerMode");
        }
        this
    }
}

// 0x9648 — __ZN19CRenderSettingsItem15setQualityLevelEN3RBX15CRenderSettings12QualityLevelE
// type: int __fastcall(int result, int)
// IDA 0x9648: store +0x74 then fire(+0xC0, &unk_130C2AC) iff changed; return this.
#[doc(alias = "CRenderSettingsItem::setQualityLevel(RBX::CRenderSettings::QualityLevel)")]
#[doc(alias = "__ZN19CRenderSettingsItem15setQualityLevelEN3RBX15CRenderSettings12QualityLevelE")]
pub fn stub_0x9648(this: *mut CRenderSettingsItem, value: i32) -> *mut CRenderSettingsItem {
    // SAFETY: `this` must point to a valid `CRenderSettingsItem`.
    unsafe {
        let item = &mut *this;
        if item.quality_level != value {
            item.quality_level = value;
            item.property_changed.fire("QualityLevel");
        }
        this
    }
}

// 0x9668 — __ZN19CRenderSettingsItem23setAlwaysDrawConnectorsEb
// type: int __fastcall(int this, int)
// IDA 0x9668: effective = +0x9B ? 1 : normalize(+0x9C); store +0x9B, then fire
// (+0xC0, &unk_130C030) iff the effective value changed; return this.
#[doc(alias = "CRenderSettingsItem::setAlwaysDrawConnectors(bool)")]
#[doc(alias = "__ZN19CRenderSettingsItem23setAlwaysDrawConnectorsEb")]
pub fn stub_0x9668(this: *mut CRenderSettingsItem, value: bool) -> *mut CRenderSettingsItem {
    // SAFETY: `this` must point to a valid `CRenderSettingsItem`.
    unsafe {
        let item = &mut *this;
        let old_effective = if item.always_draw_connectors {
            1
        } else {
            normalize_flag(item.always_draw_connectors_base)
        };
        item.always_draw_connectors = value;
        if value {
            // IDA 0x9694: `CMP R2,#0` / `BXNE LR` — set override with a
            // previously nonzero effective value is a no-op signal-wise.
            if old_effective != 0 {
                return this;
            }
        } else {
            // IDA 0x968c: `TEQ.W R2,R1` / `BNE fire` — clearing the override
            // fires iff the base value differs from the old effective value.
            let new_effective = normalize_flag(item.always_draw_connectors_base);
            if old_effective == new_effective {
                return this;
            }
        }
        item.property_changed.fire("AlwaysDrawConnectors");
        this
    }
}

// 0x96ac — __ZN19CRenderSettingsItem18setShowAggregationEb
// type: int __fastcall(int this, int)
// IDA 0x96ac: store +0x9A then fire(+0xC0, &unk_130C05C) iff changed; return this.
#[doc(alias = "CRenderSettingsItem::setShowAggregation(bool)")]
#[doc(alias = "__ZN19CRenderSettingsItem18setShowAggregationEb")]
pub fn stub_0x96ac(this: *mut CRenderSettingsItem, value: bool) -> *mut CRenderSettingsItem {
    // SAFETY: `this` must point to a valid `CRenderSettingsItem`.
    unsafe {
        let item = &mut *this;
        if item.show_aggregation != value {
            item.show_aggregation = value;
            item.property_changed.fire("ShowAggregation");
        }
        this
    }
}

// 0x96d0 — __ZN19CRenderSettingsItem12setAASamplesEN3RBX15CRenderSettings9AASamplesE
// type: int __fastcall(int result, int)
// IDA 0x96d0: compares/stores the `RBX::CRenderSettings::aaSamples` GLOBAL
// (not an item field) but still fires the item's +0xC0 signal (&unk_130C2E0).
#[doc(alias = "CRenderSettingsItem::setAASamples(RBX::CRenderSettings::AASamples)")]
#[doc(alias = "__ZN19CRenderSettingsItem12setAASamplesEN3RBX15CRenderSettings9AASamplesE")]
pub fn stub_0x96d0(this: *mut CRenderSettingsItem, value: i32) -> *mut CRenderSettingsItem {
    if AA_SAMPLES.load(Ordering::SeqCst) != value {
        AA_SAMPLES.store(value, Ordering::SeqCst);
        // SAFETY: `this` must point to a valid `CRenderSettingsItem`.
        unsafe {
            (*this).property_changed.fire("AASamples");
        }
    }
    this
}

// 0x96fc — __ZN19CRenderSettingsItem13setShadowModeEN3RBX15CRenderSettings10ShadowModeE
// type: int __fastcall(int result, int)
// IDA 0x96fc: store +0x6C then fire(+0xC0, &unk_130C314) iff changed; return this.
#[doc(alias = "CRenderSettingsItem::setShadowMode(RBX::CRenderSettings::ShadowMode)")]
#[doc(alias = "__ZN19CRenderSettingsItem13setShadowModeEN3RBX15CRenderSettings10ShadowModeE")]
pub fn stub_0x96fc(this: *mut CRenderSettingsItem, value: i32) -> *mut CRenderSettingsItem {
    // SAFETY: `this` must point to a valid `CRenderSettingsItem`.
    unsafe {
        let item = &mut *this;
        if item.shadow_mode != value {
            item.shadow_mode = value;
            item.property_changed.fire("ShadowMode");
        }
        this
    }
}

// 0x971c — __ZN19CRenderSettingsItem19setAntialiasingModeEN3RBX15CRenderSettings16AntialiasingModeE
// type: int __fastcall(int result, int)
// IDA 0x971c: store +0x68 then fire(+0xC0, &unk_130C348) iff changed; return this.
#[doc(alias = "CRenderSettingsItem::setAntialiasingMode(RBX::CRenderSettings::AntialiasingMode)")]
#[doc(alias = "__ZN19CRenderSettingsItem19setAntialiasingModeEN3RBX15CRenderSettings16AntialiasingModeE")]
pub fn stub_0x971c(this: *mut CRenderSettingsItem, value: i32) -> *mut CRenderSettingsItem {
    // SAFETY: `this` must point to a valid `CRenderSettingsItem`.
    unsafe {
        let item = &mut *this;
        if item.antialiasing_mode != value {
            item.antialiasing_mode = value;
            item.property_changed.fire("AntialiasingMode");
        }
        this
    }
}

// 0x973c — __ZN19CRenderSettingsItem25setDebugShowBoundingBoxesEb
// type: int __fastcall(int this, int)
// IDA 0x973c: store +0x88 then fire(+0xC0, &unk_130C0E0) iff changed; return this.
#[doc(alias = "CRenderSettingsItem::setDebugShowBoundingBoxes(bool)")]
#[doc(alias = "__ZN19CRenderSettingsItem25setDebugShowBoundingBoxesEb")]
pub fn stub_0x973c(this: *mut CRenderSettingsItem, value: bool) -> *mut CRenderSettingsItem {
    // SAFETY: `this` must point to a valid `CRenderSettingsItem`.
    unsafe {
        let item = &mut *this;
        if item.debug_show_bounding_boxes != value {
            item.debug_show_bounding_boxes = value;
            item.property_changed.fire("DebugShowBoundingBoxes");
        }
        this
    }
}

// 0x9760 — __ZN19CRenderSettingsItem12setEnableFRMEb
// type: int __fastcall(int this, int)
// IDA 0x9760: store +0x89 then fire(+0xC0, &unk_130C138) iff changed; return this.
#[doc(alias = "CRenderSettingsItem::setEnableFRM(bool)")]
#[doc(alias = "__ZN19CRenderSettingsItem12setEnableFRMEb")]
pub fn stub_0x9760(this: *mut CRenderSettingsItem, value: bool) -> *mut CRenderSettingsItem {
    // SAFETY: `this` must point to a valid `CRenderSettingsItem`.
    unsafe {
        let item = &mut *this;
        if item.enable_frm != value {
            item.enable_frm = value;
            item.property_changed.fire("EnableFRM");
        }
        this
    }
}

// 0x9784 — __ZNK19CRenderSettingsItem28getDebugDisableInterpolationEv
// type: int __fastcall(CRenderSettingsItem *this)
// IDA 0x9784: ignores `this`; returns the `RBX::PartInstance::disableInterpolation` global byte.
#[doc(alias = "CRenderSettingsItem::getDebugDisableInterpolation(void)const")]
#[doc(alias = "__ZNK19CRenderSettingsItem28getDebugDisableInterpolationEv")]
pub fn stub_0x9784(this: *const CRenderSettingsItem) -> bool {
    let _ = this;
    DISABLE_INTERPOLATION.load(Ordering::SeqCst)
}

// 0x9794 — __ZN19CRenderSettingsItem28setDebugDisableInterpolationEb
// type: char *__fastcall(CRenderSettingsItem *this, char)
// IDA 0x9794: sets the `disableInterpolation` global and returns its address;
// no signal fires and `this` is unused.
#[doc(alias = "CRenderSettingsItem::setDebugDisableInterpolation(bool)")]
#[doc(alias = "__ZN19CRenderSettingsItem28setDebugDisableInterpolationEb")]
pub fn stub_0x9794(this: *mut CRenderSettingsItem, value: bool) -> *mut bool {
    let _ = this;
    DISABLE_INTERPOLATION.store(value, Ordering::SeqCst);
    DISABLE_INTERPOLATION.as_ptr()
}

// 0x97a4 — __ZN19CRenderSettingsItem23setResolutionPreferenceEN3RBX15CRenderSettings16ResolutionPresetE
// type: int __fastcall(int result, int)
// IDA 0x97a4: store +0x78 then fire(+0xC0, &CRenderSettingsItem::prop_resolution) iff changed; return this.
#[doc(alias = "CRenderSettingsItem::setResolutionPreference(RBX::CRenderSettings::ResolutionPreset)")]
#[doc(alias = "__ZN19CRenderSettingsItem23setResolutionPreferenceEN3RBX15CRenderSettings16ResolutionPresetE")]
pub fn stub_0x97a4(this: *mut CRenderSettingsItem, value: i32) -> *mut CRenderSettingsItem {
    // SAFETY: `this` must point to a valid `CRenderSettingsItem`.
    unsafe {
        let item = &mut *this;
        if item.resolution_preset != value {
            item.resolution_preset = value;
            item.property_changed.fire("ResolutionPreference");
        }
        this
    }
}

// 0x97c0 — __ZN19CRenderSettingsItem19setTextureCacheSizeEj
// type: int __fastcall(int this, unsigned int)
// IDA 0x97c0: unconditional store +0xA0, no signal; return this.
#[doc(alias = "CRenderSettingsItem::setTextureCacheSize(unsigned int)")]
#[doc(alias = "__ZN19CRenderSettingsItem19setTextureCacheSizeEj")]
pub fn stub_0x97c0(this: *mut CRenderSettingsItem, value: u32) -> *mut CRenderSettingsItem {
    // SAFETY: `this` must point to a valid `CRenderSettingsItem`.
    // IDA 0x97c0: unconditional `STR.W R1,[R0,#0xA0]`; no compare, no signal.
    unsafe {
        (*this).texture_cache_size = value;
        this
    }
}

// 0x97c8 — __ZN19CRenderSettingsItem16setMeshCacheSizeEj
// type: int __fastcall(int this, unsigned int)
// IDA 0x97c8: unconditional store +0xA4, no signal; return this.
#[doc(alias = "CRenderSettingsItem::setMeshCacheSize(unsigned int)")]
#[doc(alias = "__ZN19CRenderSettingsItem16setMeshCacheSizeEj")]
pub fn stub_0x97c8(this: *mut CRenderSettingsItem, value: u32) -> *mut CRenderSettingsItem {
    // SAFETY: `this` must point to a valid `CRenderSettingsItem`.
    // IDA 0x97c8: unconditional `STR.W R1,[R0,#0xA4]`; no compare, no signal.
    unsafe {
        (*this).mesh_cache_size = value;
        this
    }
}

// 0x97d0 — __ZN19CRenderSettingsItemC2Ev
// type: void __fastcall(CRenderSettingsItem *this)
#[doc(alias = "CRenderSettingsItem::CRenderSettingsItem(void)")]
#[doc(alias = "__ZN19CRenderSettingsItemC2Ev")]
pub fn stub_0x97d0(this: *mut CRenderSettingsItem) -> *mut CRenderSettingsItem {
    // SAFETY: `this` must point to valid uninitialized item storage.
    unsafe {
        // IDA 0x97f0: base `GlobalAdvancedSettingsItem` C2 (vtables at
        // 0x983c..0x985c, `classDescriptor`, `setName("RenderSettings")`,
        // singleton slot) — base-class state owned by 0xb4fc.
        stub_0xb4fc(this);
        let item = &mut *this;
        // IDA 0x9828: `CRenderSettings::CRenderSettings(this + 96)` — the
        // settings C2 owns the +96-subobject defaults (separate EA); the
        // item-side mirrors below start from `Default`.
        // IDA 0x9876: +168 string = empty.
        item.string_168 = String::new();
        // IDA 0x987e/0x988a: +172/+174 lanes = 800/600.
        let first = Vector2int16 { x: 800, y: 600 };
        item.first_resolution = first;
        // IDA 0x9896..0x98aa: +176 vector = empty; 0x991a pushes the pair.
        item.resolutions = Vec::new();
        stub_0xb740(&mut item.resolutions, &first);
        // IDA 0x98b0: +189 byte = 1.
        item.byte_189 = true;
        // IDA 0x98d0/0x98d8: signal safe-static mutex init — owned by `Signal`.
        // IDA 0x98f6/0x9904: `+28` virtual call with `std::string("Rendering")`.
        item.render_category = "Rendering".to_owned();
        // IDA 0x9922..0x9946: `GetDXVideoMemorySize()` threshold select
        // (`&loc_F423FC + 3`) stored at item +146 (settings +50).
        item.video_memory_budget = video_memory_budget();
        this
    }
}

// 0x9ac8 — __ZN19CRenderSettingsItem19setAutoQualityLevelEi
// type: int __fastcall(int this, int)
// IDA 0x9ac8: store +0x7C then fire(+0xC0, &unk_130C2AC) iff changed; return this.
#[doc(alias = "CRenderSettingsItem::setAutoQualityLevel(int)")]
#[doc(alias = "__ZN19CRenderSettingsItem19setAutoQualityLevelEi")]
pub fn stub_0x9ac8(this: *mut CRenderSettingsItem, value: i32) -> *mut CRenderSettingsItem {
    // SAFETY: `this` must point to a valid `CRenderSettingsItem`.
    unsafe {
        let item = &mut *this;
        if item.auto_quality_level != value {
            item.auto_quality_level = value;
            // IDA 0x9ade fires &unk_130C2AC — the same descriptor 0x9648
            // (`setQualityLevel`) fires, so the notification name matches.
            item.property_changed.fire("QualityLevel");
        }
        this
    }
}

// 0x9ae8 — __ZThn96_N19CRenderSettingsItem19setAutoQualityLevelEi
// type: int __fastcall(int this, int)
// IDA 0x9ae8: compare [R0,#0x1C], then `SUBS R0,#0x60` and 0x9ac8's body on the adjusted pointer; return original this.
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::setAutoQualityLevel(int)")]
#[doc(alias = "__ZThn96_N19CRenderSettingsItem19setAutoQualityLevelEi")]
pub fn stub_0x9ae8(this: *mut u8, value: i32) -> *mut u8 {
    // SAFETY: `this` must point to a valid `CRenderSettingsItem` viewed through
    // a base subobject 0x60 bytes in (`SUBS R0,#0x60` after the +0x1C compare,
    // which is the same slot as the adjusted +0x7C compare in 0x9ac8).
    // IDA 0x9ae8 body is 0x9ac8's body on the adjusted pointer, same &unk_130C2AC.
    unsafe {
        let adjusted = this.sub(0x60) as *mut CRenderSettingsItem;
        stub_0x9ac8(adjusted, value);
        this
    }
}

// 0x9b08 — __ZN19CRenderSettingsItem21setEagerBulkExecutionEb
// type: int __fastcall(int this, int)
// IDA 0x9b08: store +0x9D then fire(+0xC0, &unk_130C1E8) iff changed; return this.
#[doc(alias = "CRenderSettingsItem::setEagerBulkExecution(bool)")]
#[doc(alias = "__ZN19CRenderSettingsItem21setEagerBulkExecutionEb")]
pub fn stub_0x9b08(this: *mut CRenderSettingsItem, value: bool) -> *mut CRenderSettingsItem {
    // SAFETY: `this` must point to a valid `CRenderSettingsItem`.
    unsafe {
        let item = &mut *this;
        if item.eager_bulk_execution != value {
            item.eager_bulk_execution = value;
            // IDA 0x9b22 fires &unk_130C1E8; name follows the property-name
            // convention used by every other setter in this file.
            item.property_changed.fire("EagerBulkExecution");
        }
        this
    }
}

// 0x9b2c — __ZNSt12length_errorD1Ev
// type: void __cdecl(std::length_error *__hidden this)
#[doc(alias = "std::length_error::~length_error()")]
#[doc(alias = "__ZNSt12length_errorD1Ev")]
pub fn stub_0x9b2c() {
    // IDA 0x9b2c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x9b30 — __ZNSt12out_of_rangeD0Ev
// type: void __cdecl(std::out_of_range *__hidden this)
#[doc(alias = "std::out_of_range::~out_of_range()")]
#[doc(alias = "__ZNSt12out_of_rangeD0Ev")]
pub fn stub_0x9b30() {
    // IDA 0x9b30: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x9b44 — __ZNSt12out_of_rangeD2Ev
// type: void __cdecl(std::out_of_range *__hidden this)
#[doc(alias = "std::out_of_range::~out_of_range()")]
#[doc(alias = "__ZNSt12out_of_rangeD2Ev")]
pub fn stub_0x9b44() {
    // IDA 0x9b44: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0xb33c — __ZNK3RBX15CRenderSettings15getGraphicsModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getGraphicsMode(void)const")]
#[doc(alias = "__ZNK3RBX15CRenderSettings15getGraphicsModeEv")]
pub fn stub_0xb33c(this: *const CRenderSettings) -> i32 {
    // SAFETY: `this` must point to a valid `CRenderSettings`.
    // IDA 0xb33c `LDR R0,[R0,#4]`: plain +4 field load.
    unsafe { (*this).graphics_mode }
}

// 0xb364 — __ZNK3RBX15CRenderSettings23getFrameRateManagerModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getFrameRateManagerMode(void)const")]
#[doc(alias = "__ZNK3RBX15CRenderSettings23getFrameRateManagerModeEv")]
pub fn stub_0xb364(this: *const CRenderSettings) -> i32 {
    // SAFETY: `this` must point to a valid `CRenderSettings`.
    // IDA 0xb364 `LDR R0,[R0,#0x10]`: plain +0x10 field load.
    unsafe { (*this).frame_rate_manager_mode }
}

// 0xb38c — __ZNK3RBX15CRenderSettings15getQualityLevelEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getQualityLevel(void)const")]
#[doc(alias = "__ZNK3RBX15CRenderSettings15getQualityLevelEv")]
pub fn stub_0xb38c(this: *const CRenderSettings) -> i32 {
    // SAFETY: `this` must point to a valid `CRenderSettings`.
    // IDA 0xb38c `LDR R0,[R0,#0x14]`: plain +0x14 field load.
    unsafe { (*this).quality_level }
}

// 0xb3b4 — __ZNK3RBX15CRenderSettings23getAlwaysDrawConnectorsEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getAlwaysDrawConnectors(void)const")]
#[doc(alias = "__ZNK3RBX15CRenderSettings23getAlwaysDrawConnectorsEv")]
pub fn stub_0xb3b4(this: *const CRenderSettings) -> i32 {
    // SAFETY: `this` must point to a valid `CRenderSettings`.
    // IDA 0xb3b4 `LDRB.W R0,[R0,#0x3B]`: byte load, zero-extended into R0.
    unsafe { i32::from((*this).always_draw_connectors) }
}

// 0xb3e0 — __ZNK3RBX15CRenderSettings18getShowAggregationEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getShowAggregation(void)const")]
#[doc(alias = "__ZNK3RBX15CRenderSettings18getShowAggregationEv")]
pub fn stub_0xb3e0(this: *const CRenderSettings) -> i32 {
    // SAFETY: `this` must point to a valid `CRenderSettings`.
    // IDA 0xb3e0 `LDRB.W R0,[R0,#0x3A]`: byte load, zero-extended into R0.
    unsafe { i32::from((*this).show_aggregation) }
}

// 0xb3e8 — __ZNK3RBX15CRenderSettings12getAASamplesEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getAASamples(void)const")]
#[doc(alias = "__ZNK3RBX15CRenderSettings12getAASamplesEv")]
pub fn stub_0xb3e8(this: *const CRenderSettings) -> i32 {
    // IDA 0xb3e8 double-indirects the `aaSamples` global (`LDR R0,[R0]` twice
    // via the `_ptr` slot); `this` is unused. Modelled by `AA_SAMPLES`.
    let _ = this;
    AA_SAMPLES.load(Ordering::SeqCst)
}

// 0xb41c — __ZNK3RBX15CRenderSettings13getShadowModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getShadowMode(void)const")]
#[doc(alias = "__ZNK3RBX15CRenderSettings13getShadowModeEv")]
pub fn stub_0xb41c(this: *const CRenderSettings) -> i32 {
    // SAFETY: `this` must point to a valid `CRenderSettings`.
    // IDA 0xb41c `LDR R0,[R0,#0xC]`: plain +0xC field load.
    unsafe { (*this).shadow_mode }
}

// 0xb444 — __ZNK3RBX15CRenderSettings19getAntialiasingModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getAntialiasingMode(void)const")]
#[doc(alias = "__ZNK3RBX15CRenderSettings19getAntialiasingModeEv")]
pub fn stub_0xb444(this: *const CRenderSettings) -> i32 {
    // SAFETY: `this` must point to a valid `CRenderSettings`.
    // IDA 0xb444 `LDR R0,[R0,#8]`: plain +8 field load.
    unsafe { (*this).antialiasing_mode }
}

// 0xb46c — __ZNK3RBX15CRenderSettings25getDebugShowBoundingBoxesEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getDebugShowBoundingBoxes(void)const")]
#[doc(alias = "__ZNK3RBX15CRenderSettings25getDebugShowBoundingBoxesEv")]
pub fn stub_0xb46c(this: *const CRenderSettings) -> i32 {
    // SAFETY: `this` must point to a valid `CRenderSettings`.
    // IDA 0xb46c `LDRB.W R0,[R0,#0x28]`: byte load, zero-extended into R0.
    unsafe { i32::from((*this).debug_show_bounding_boxes) }
}

// 0xb474 — __ZNK3RBX15CRenderSettings19getAutoQualityLevelEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getAutoQualityLevel(void)const")]
#[doc(alias = "__ZNK3RBX15CRenderSettings19getAutoQualityLevelEv")]
pub fn stub_0xb474(this: *const CRenderSettings) -> i32 {
    // SAFETY: `this` must point to a valid `CRenderSettings`.
    // IDA 0xb474 `LDR R0,[R0,#0x1C]`: plain +0x1C field load.
    unsafe { (*this).auto_quality_level }
}

// 0xb49c — __ZNK3RBX15CRenderSettings12getEnableFRMEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getEnableFRM(void)const")]
#[doc(alias = "__ZNK3RBX15CRenderSettings12getEnableFRMEv")]
pub fn stub_0xb49c(this: *const CRenderSettings) -> i32 {
    // SAFETY: `this` must point to a valid `CRenderSettings`.
    // IDA 0xb49c `LDRB.W R0,[R0,#0x29]`: byte load, zero-extended into R0.
    unsafe { i32::from((*this).enable_frm) }
}

// 0xb4a4 — __ZNK3RBX15CRenderSettings23getResolutionPreferenceEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getResolutionPreference(void)const")]
#[doc(alias = "__ZNK3RBX15CRenderSettings23getResolutionPreferenceEv")]
pub fn stub_0xb4a4(this: *const CRenderSettings) -> i32 {
    // SAFETY: `this` must point to a valid `CRenderSettings`.
    // IDA 0xb4a4 `LDR R0,[R0,#0x18]`: plain +0x18 field load.
    unsafe { (*this).resolution_preference }
}

// 0xb4cc — __ZN3RBX15CRenderSettings18getMaxQualityLevelEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getMaxQualityLevel(void)")]
#[doc(alias = "__ZN3RBX15CRenderSettings18getMaxQualityLevelEv")]
pub fn stub_0xb4cc(this: *const CRenderSettings) -> i32 {
    // SAFETY: `this` must point to a valid `CRenderSettings`.
    // IDA 0xb4cc `LDR R0,[R0,#0x20]`: plain +0x20 field load.
    unsafe { (*this).max_quality_level }
}

// 0xb4f4 — __ZNK3RBX15CRenderSettings19getTextureCacheSizeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getTextureCacheSize(void)const")]
#[doc(alias = "__ZNK3RBX15CRenderSettings19getTextureCacheSizeEv")]
pub fn stub_0xb4f4(this: *const CRenderSettings) -> i32 {
    // SAFETY: `this` must point to a valid `CRenderSettings`.
    // IDA 0xb4f4 `LDR R0,[R0,#0x40]`: plain +0x40 field load.
    unsafe { (*this).texture_cache_size as i32 }
}

// 0xb4f8 — __ZNK3RBX15CRenderSettings16getMeshCacheSizeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getMeshCacheSize(void)const")]
#[doc(alias = "__ZNK3RBX15CRenderSettings16getMeshCacheSizeEv")]
pub fn stub_0xb4f8(this: *const CRenderSettings) -> i32 {
    // SAFETY: `this` must point to a valid `CRenderSettings`.
    // IDA 0xb4f8 `LDR R0,[R0,#0x44]`: plain +0x44 field load.
    unsafe { (*this).mesh_cache_size as i32 }
}

// 0xb4fc — __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEEC2Ev
// type: RBX::Instance *__fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEEC2Ev")]
pub fn stub_0xb4fc(this: *mut CRenderSettingsItem) -> *mut CRenderSettingsItem {
    // SAFETY: `this` must point to valid uninitialized base storage.
    // IDA 0xb4fc (`GlobalAdvancedSettingsItem` C2): 0xb51e
    // `Instance::Instance(this, nullptr)` + vtable installs (0xb54e..0xb5e2)
    // + `classDescriptor`/`registrar++` (0xb584..0xb5ba) are base-class state
    // with no Rust subobject; the observable effects below are mirrored.
    unsafe {
        // IDA 0xb5ec/0xb5f8: `std::string("RenderSettings")` + `setName`.
        // Stored on the item mirror (the derived ctor at 0x97d0 overwrites
        // it via the `+28` virtual call — construction order preserved).
        (*this).render_category = "RenderSettings".to_owned();
        // IDA 0xb5b2: `++ClassRegistrar<CRenderSettingsItem>::registrar`.
        CLASS_REGISTRAR_COUNT.fetch_add(1, Ordering::SeqCst);
        // IDA 0xb622..0xb6b4: `if (sing) throw runtime_error("singleton %s
        // already exists", "RenderSettings")`; 0xb626 `sing = this`.
        // `OnceLock::set` failing is exactly the second-construct throw.
        RENDER_SETTINGS_SING
            .set(this as usize)
            .unwrap_or_else(|_| panic!("singleton RenderSettings already exists"));
        this
    }
}

// 0xb740 — __ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE9push_backERKS1_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::push_back(G3D::Vector2int16 const&)")]
#[doc(alias = "__ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE9push_backERKS1_")]
pub fn stub_0xb740<'a>(vec: &'a mut Vec<Vector2int16>, value: &Vector2int16) -> &'a mut Vec<Vector2int16> {
    // IDA 0xb742..0xb75c: `finish = *(result + 4)`; unless `finish` reached
    // `end_of_storage` (0xb74c `BEQ _M_insert_aux`, i.e. len == capacity),
    // 4-byte copy the element and bump `finish`. `Vec::push` is that fast
    // path; the full case delegates to 0xf704 like the original.
    if vec.len() == vec.capacity() {
        stub_0xf704(vec, vec.len(), *value);
    } else {
        vec.push(*value);
    }
    vec
}

// 0xb8b0 — __ZNK3RBX15CRenderSettings21getEagerBulkExecutionEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getEagerBulkExecution(void)const")]
#[doc(alias = "__ZNK3RBX15CRenderSettings21getEagerBulkExecutionEv")]
pub fn stub_0xb8b0(this: *const CRenderSettings) -> i32 {
    // SAFETY: `this` must point to a valid `CRenderSettings`.
    // IDA 0xb8b0 `LDRB.W R0,[R0,#0x3D]`: byte load, zero-extended into R0.
    unsafe { i32::from((*this).eager_bulk_execution) }
}

// 0xb8b8 — __ZN19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "CRenderSettingsItem::~CRenderSettingsItem()")]
#[doc(alias = "__ZN19CRenderSettingsItemD1Ev")]
pub fn stub_0xb8b8() {
    // IDA 0xb8b8: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xb8bc — __ZN19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "CRenderSettingsItem::~CRenderSettingsItem()")]
#[doc(alias = "__ZN19CRenderSettingsItemD0Ev")]
pub fn stub_0xb8bc() {
    // IDA 0xb8bc: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xb8d0 — __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv")]
pub fn stub_0xb8d0() -> &'static str {
    // IDA 0xb8d0..0xb8dc: `Creator = static_getCreator()` then tail-calls
    // `Creator::getClassName` on it — i.e. exactly 0xedfc's body.
    let _ = stub_0xf500();
    stub_0xedfc()
}

// 0xb8e0 — __ZThn32_N19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
#[doc(alias = "__ZThn32_N19CRenderSettingsItemD1Ev")]
pub fn stub_0xb8e0() {
    // IDA 0xb8e0: __ZThn32 thunk (D1 base dtor): `this -= 32`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0xb8e8 — __ZThn32_N19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
#[doc(alias = "__ZThn32_N19CRenderSettingsItemD0Ev")]
pub fn stub_0xb8e8() {
    // IDA 0xb8e8: __ZThn32 thunk (D0 deleting dtor): `this -= 32`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xb900 — __ZThn32_NK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv")]
pub fn stub_0xb900() -> &'static str {
    // IDA `__ZThn32` thunk to 0xb8d0 (`this -= 32`, run body): returns what
    // the body returns — 0xedfc shows that is the declared `sRenderSettings`
    // name, not the C++ class-name spelling previously guessed here.
    stub_0xedfc()
}

// 0xb910 — __ZThn36_N19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
#[doc(alias = "__ZThn36_N19CRenderSettingsItemD1Ev")]
pub fn stub_0xb910() {
    // IDA 0xb910: __ZThn36 thunk (D1 base dtor): `this -= 36`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0xb918 — __ZThn36_N19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
#[doc(alias = "__ZThn36_N19CRenderSettingsItemD0Ev")]
pub fn stub_0xb918() {
    // IDA 0xb918: __ZThn36 thunk (D0 deleting dtor): `this -= 36`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xb930 — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0xb930() {
    // IDA 0xb930: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xcb94 — __ZN5boost16exception_detail12refcount_ptrINS0_20error_info_containerEED2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "boost::exception_detail::refcount_ptr<boost::exception_detail::error_info_container>::~refcount_ptr()")]
#[doc(alias = "__ZN5boost16exception_detail12refcount_ptrINS0_20error_info_containerEED2Ev")]
pub fn stub_0xcb94() {
    // IDA 0xcb94: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0xeccc — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD2Ev
// type: int __fastcall(int)
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0xeccc() {
    // IDA 0xeccc: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0xedfc — __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0xedfc() -> &'static str {
    // IDA 0xedfc..0xee5c: `wasConstructed()` assert (`isConstructedE == 0x29A`,
    // via `FLog::Asserts`/`_debugHook`/`ReleaseAssert`); 0xee60..0xee80:
    // `call_once(Name declare)` then tail-calls `Name::doDeclare` (0xf1dc) —
    // so this returns the declared `sRenderSettings` name.
    assert_creator_constructed("wasConstructed()");
    stub_0xf1d8()
}

// 0xee84 — __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator6createEv
// type: int __fastcall(int *)
#[doc(alias = "__ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator6createEv")]
pub fn stub_0xee84(out: *mut SharedPtr<CRenderSettingsItem>) -> *mut SharedPtr<CRenderSettingsItem> {
    // IDA 0xee84..0xeee8: `wasConstructed()` assert, then 0xeeec
    // `Creatable::create` into a stack `SharedPtr`; 0xeef2..0xeefe copies
    // both words to `*a1`, biasing the pointer word by `+0x20` when non-null
    // (the shared `DescribedBase` subobject — layout-selected, owned by the
    // C++ object model; the whole `Arc` is stored here).
    // SAFETY: `out` must point to valid `SharedPtr` storage.
    assert_creator_constructed("wasConstructed()");
    unsafe {
        out.write(stub_0xef04());
        out
    }
}

// 0xef04 — __ZN3RBX9CreatableINS_8InstanceEE6createI19CRenderSettingsItemEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<CRenderSettingsItem> RBX::Creatable<RBX::Instance>::create<CRenderSettingsItem>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createI19CRenderSettingsItemEEN5boost10shared_ptrIT_EEv")]
pub fn stub_0xef04() -> SharedPtr<CRenderSettingsItem> {
    // IDA 0xef04..0xef6a: `operator new(0xC4)` (0xef38), in-place C2 ctor
    // (0xef5c = 0x97d0, which runs the 0xb4fc base ctor first), then the
    // `shared_ptr` adopt (0xef6a = 0xefb4). Box→Arc is the same single-owner
    // adoption (`rbx_core::shared_ptr::shared_ptr_from_raw`).
    let mut item = Box::new(CRenderSettingsItem::default());
    stub_0x97d0(&mut *item);
    stub_0xefb4(item)
}

// 0xefb4 — __ZN5boost10shared_ptrI19CRenderSettingsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<CRenderSettingsItem>::shared_ptr<CRenderSettingsItem,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrI19CRenderSettingsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0xefb4(px: Box<CRenderSettingsItem>) -> SharedPtr<CRenderSettingsItem> {
    // IDA 0xefba..0xefd0: store `px`, build the `shared_count`, and when
    // non-null `_internal_accept_owner<...>(px + 40)` for
    // `enable_shared_from_this<DescribedBase>` — `Arc` adoption covers all
    // three (weak support is intrinsic to the `Arc` control block).
    rbx_core::shared_ptr::shared_ptr_from_raw(px)
}

// 0xefd8 — __ZNK5boost6detail15sp_counted_base9use_countEv
// type: int __fastcall(boost::detail::sp_counted_base *this)
#[doc(alias = "boost::detail::sp_counted_base::use_count(void)const")]
#[doc(alias = "__ZNK5boost6detail15sp_counted_base9use_countEv")]
pub fn stub_0xefd8<T>(shared: &SharedPtr<T>) -> i32 {
    // IDA 0xefd8..0xf078: hash `this` into `spinlock_pool<1>`, lock, load
    // `use_count` (+1 word), unlock, return it. `Arc::strong_count` is the
    // same control-block load under the block's own lock.
    SharedPtr::strong_count(shared) as i32
}

// 0xf098 — __ZN5boost6detail12shared_countC2IP19CRenderSettingsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IP19CRenderSettingsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0xf098(
    px: Box<CRenderSettingsItem>,
) -> ControlBlockPd<CRenderSettingsItem, CreatableInstanceDeleter> {
    // IDA 0xf0c4..0xf10c: `operator new(0x14)`; `use_count = weak_count = 1`;
    // vtable install; `px` stored (the deleter args are an empty tag type).
    ControlBlockPd::new(px, CreatableInstanceDeleter)
}

// 0xf198 — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0xf198() {
    // IDA 0xf198: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xf19c — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0xf19c(block: &mut ControlBlockPd<CRenderSettingsItem, CreatableInstanceDeleter>) {
    // IDA 0xf19e..0xf1b8: `px = this + 12`; `RBX::Instance::predelete(px)`;
    // `if (px) virtual-delete(px)` (0xf1b8 via vtable +8, 0xf1ac returns).
    // `predelete` is the datamodel-owned hook; the trailing delete is drop.
    block.dispose_with(|_| {});
}

// 0xf1bc — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0xf1bc<'a>(
    block: &'a ControlBlockPd<CRenderSettingsItem, CreatableInstanceDeleter>,
    type_name: &str,
) -> Option<CreatableInstanceDeleter> {
    // IDA 0xf1c0..0xf1d2: return `this + 16` iff `ti.name` is
    // `"N3RBX9CreatableINS_8InstanceEE7DeleterE"`, else null.
    block.get_deleter(type_name)
}

// 0xf1d4 — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0xf1d4(
    block: &ControlBlockPd<CRenderSettingsItem, CreatableInstanceDeleter>,
) -> CreatableInstanceDeleter {
    // IDA 0xf1d6: unconditionally return `this + 16`.
    block.get_untyped_deleter()
}

// 0xf1d8 — __ZN3RBX4Name13callDoDeclareILZ15sRenderSettingsEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZ15sRenderSettingsEEEvv")]
pub fn stub_0xf1d8() -> &'static str {
    // IDA 0xf1d8: thunk tail-calling `Name::doDeclare` (0xf1dc).
    stub_0xf1dc()
}

// 0xf1dc — __ZN3RBX4Name9doDeclareILZ15sRenderSettingsEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZ15sRenderSettingsEEERKS0_v")]
pub fn stub_0xf1dc() -> &'static str {
    // IDA 0xf1dc..0xf290: guard-once (`__cxa_guard_acquire`/`release` around
    // the function static, 0xf230..0xf262) `RBX::Name::declare(sRenderSettings)`
    // (0xf24e), then return the cached name (0xf266..0xf27a). `LazyLock` is
    // that guarded static; the `sRenderSettings` text is "RenderSettings".
    RENDER_SETTINGS_NAME.as_str()
}

// 0xf2bc — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0xf2bc() -> &'static RenderSettingsCreator {
    // IDA 0xf2f2: vtable install — no Rust equivalent.
    // IDA 0xf2f4: `call_once(Name declare)` — ensure the product name exists.
    let name = stub_0xf1d8();
    {
        let mut creators = CREATOR_REGISTRY.lock();
        if !creators.contains(&name) {
            creators.push(name);
        }
    }
    // re-checks registration + `wasConstructed()` under `FLog::Asserts`.
    CREATOR_IS_CONSTRUCTED.store(CREATOR_CONSTRUCTED_MAGIC, Ordering::SeqCst);
    &RENDER_SETTINGS_CREATOR
}

// 0xf500 — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0xf500() -> &'static RenderSettingsCreator {
    // IDA 0xf500..0xf562: `wasConstructed()` assert
    // (`isConstructedE == 0x29A`, `Object.h:282`) then return `creatorPrivate`.
    assert_creator_constructed("Creator::wasConstructed()");
    &RENDER_SETTINGS_CREATOR
}

// 0xf704 — __ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int __fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_insert_aux(__gnu_cxx::__normal_iterator<G3D::Vector2int16*,std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>>,G3D::Vector2int16 const&)")]
#[doc(alias = "__ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
pub fn stub_0xf704(vec: &mut Vec<Vector2int16>, pos: usize, value: Vector2int16) -> &mut Vec<Vector2int16> {
    // IDA 0xf704..0xf7e4 (slow path: `finish == end_of_storage`): grow to
    // `max(1, len * 3 / 2)`-ish (0xf73e..0xf7d8; `len == 0x3FFFFFFF` throws
    // `length_error`, 0xf7d2..0xf7e4), copy `[first, pos)` + `value` +
    // `[pos, finish)` over (via 0xf800's backward copy), free old storage
    // (0xf7b0..0xf7c4). `Vec::insert` grows and shifts identically; the
    // length-error arm is made explicit.
    if vec.len() == 0x3FFF_FFFF {
        panic!("vector::_M_insert_aux");
    }
    vec.insert(pos.min(vec.len()), value);
    vec
}

// 0xf7e8 — __ZNSt12_Vector_baseIN3G3D12Vector2int16ESaIS1_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3G3D12Vector2int16ESaIS1_EE11_M_allocateEm")]
pub fn stub_0xf7e8(n: usize) -> Vec<Vector2int16> {
    // IDA 0xf7ea..0xf7fc: `n >= 0x40000000` throws `bad_alloc` (0xf7f2);
    // else `operator new(4 * n)` (`sizeof(Vector2int16) == 4`, 0xf7f6).
    // Returns reserved (length-0) storage like the fresh vector buffer.
    if n >= 0x4000_0000 {
        panic!("std::bad_alloc");
    }
    Vec::with_capacity(n)
}

// 0xf800 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D12Vector2int16ES5_EET0_T_S7_S6_
// type: int __fastcall(int, int, int)
#[doc(alias = "G3D::Vector2int16 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector2int16 *,G3D::Vector2int16 *>(G3D::Vector2int16 *,G3D::Vector2int16 *,G3D::Vector2int16 *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D12Vector2int16ES5_EET0_T_S7_S6_")]
pub fn stub_0xf800(
    first: *const Vector2int16,
    last: *const Vector2int16,
    result: *mut Vector2int16,
) -> *mut Vector2int16 {
    // IDA 0xf800..0xf838: `n = (last - first)` elements copied back-to-front
    // (0xf826..0xf832) so overlapping ranges shift correctly; `n < 1`
    // returns `result` unchanged (0xf804). Returns `result - n` (0xf834).
    // SAFETY: `[first, last)` must be readable and the `n` slots ending at
    // `result` writable, all within one allocation.
    unsafe {
        let n = (last as usize).saturating_sub(first as usize)
            / core::mem::size_of::<Vector2int16>();
        // `ptr::copy` is `memmove`: overlap-safe like the backward loop.
        core::ptr::copy(last.sub(n), result.sub(n), n);
        result.sub(n)
    }
}

// 0xf83c — __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev")]
pub fn stub_0xf83c() {
    // IDA 0xf83c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xf87c — __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev")]
pub fn stub_0xf87c() {
    // IDA 0xf87c: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xf8c8 — __ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev
// type: void __fastcall(_QWORD *)
#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev")]
pub fn stub_0xf8c8() {
    // IDA 0xf8c8: __ZThn32 thunk (D1 base dtor): `this -= 32`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0xf90c — __ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev
// type: int __fastcall(_QWORD *)
#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev")]
pub fn stub_0xf90c() {
    // IDA 0xf90c: __ZThn32 thunk (D0 deleting dtor): `this -= 32`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xf964 — __ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev")]
pub fn stub_0xf964() {
    // IDA 0xf964: __ZThn36 thunk (D1 base dtor): `this -= 36`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0xf9a8 — __ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev")]
pub fn stub_0xf9a8() {
    // IDA 0xf9a8: __ZThn36 thunk (D0 deleting dtor): `this -= 36`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

/// `RBX::Name const*` keys of the `CRenderSettings` enum maps below.
/// IDA 0x142b8/0x14cf4 (`less<RBX::Name const*>`) orders by the pointer itself
/// (decompiled `v4[1]._M_color >= *a2`); the address is the key.
pub type NameKey = usize;
/// `RBX::CRenderSettings::{ResolutionPreset, QualityLevel, ShadowMode}` payloads.
/// All three are 4-byte enums (IDA moves one element per `LDR`/`STR`, `4 * n`
/// allocation scaling, `>> 2` length math).
pub type ResolutionPreset = i32;
pub type QualityLevel = i32;
pub type ShadowMode = i32;
/// `std::map<RBX::Name const*, ...>` instantiations below.
/// `BTreeMap` preserves the ordered-map semantics (`unordered_map` would be `HashMap`).
pub type ResolutionPresetNameMap = BTreeMap<NameKey, ResolutionPreset>;
pub type QualityLevelNameMap = BTreeMap<NameKey, QualityLevel>;
pub type ShadowModeNameMap = BTreeMap<NameKey, ShadowMode>;

// 0x142b8 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings16ResolutionPresetESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: _Rb_tree_node_base **__fastcall(int, int *)
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::ResolutionPreset,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings16ResolutionPresetESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_0x142b8<'a>(map: &'a mut ResolutionPresetNameMap, key: NameKey) -> &'a mut ResolutionPreset {
    // IDA 0x142b8: map::operator[] -- lower_bound walk (0x142d0 loop), miss inserts a
    // value-initialized node and returns its reference. `entry().or_default()` is that.
    // IDA 0x142b8
    map.entry(key).or_default()
}

// 0x14310 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_0x14310(map: &mut ResolutionPresetNameMap, key: NameKey, value: ResolutionPreset) -> bool {
    // IDA 0x14310: _Rb_tree::_M_insert_unique with position hint -- the hint only seeds
    // the search; a present key is a no-op returning the existing node. Returns inserted.
    // IDA 0x14310
    use std::collections::btree_map::Entry;
    match map.entry(key) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

// 0x143c4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_0x143c4(map: &mut ResolutionPresetNameMap, key: NameKey, value: ResolutionPreset) -> &mut ResolutionPreset {
    // IDA 0x143c4: _Rb_tree::_M_insert -- `operator new(0x18)`, copy the pair
    // (0x143f4), `Rb_tree_insert_and_rebalance`; the caller guarantees a miss, so the
    // node is always linked. `insert` then reborrow the value slot is that link.
    // IDA 0x143c4
    map.insert(key, value);
    map.get_mut(&key).expect("just inserted")
}

// 0x1441c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_0x1441c(map: &mut ResolutionPresetNameMap, key: NameKey, value: ResolutionPreset) -> bool {
    // IDA 0x1441c: _Rb_tree::_M_insert_unique(value) without hint -- find-or-link, single
    // node on miss. Returns whether a node was inserted.
    // IDA 0x1441c
    use std::collections::btree_map::Entry;
    match map.entry(key) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

// 0x14484 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::resize(unsigned long,RBX::CRenderSettings::ResolutionPreset)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE6resizeEmS2_")]
pub fn stub_0x14484(vec: &mut Vec<ResolutionPreset>, len: usize, value: ResolutionPreset) {
    // IDA 0x14484: vector::resize -- shrink truncates the finish pointer
    // (`start + 4 * n`, 0x144a2), grow delegates to _M_fill_insert (0x144ac).
    // IDA 0x14484
    vec.resize(len, value);
}

// 0x144b8 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::push_back(RBX::CRenderSettings::ResolutionPreset const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE9push_backERKS2_")]
pub fn stub_0x144b8(vec: &mut Vec<ResolutionPreset>, value: &ResolutionPreset) {
    // IDA 0x144b8: vector::push_back -- fast path stores and bumps finish (0x144cc),
    // full storage delegates to _M_insert_aux (0x144da).
    // IDA 0x144b8
    vec.push(*value);
}

// 0x144e0 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ResolutionPreset*,std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>>,RBX::CRenderSettings::ResolutionPreset const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_0x144e0(vec: &mut Vec<ResolutionPreset>, index: usize, value: &ResolutionPreset) {
    // IDA 0x144e0: vector::_M_insert_aux -- full storage reallocates (2x or len+1 via
    // _M_allocate, moves elements, constructs the new one); otherwise shifts the tail
    // with __copy_backward and assigns. `Vec::insert` is that.
    // IDA 0x144e0
    vec.insert(index, *value);
}

// 0x145c4 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE11_M_allocateEm")]
pub fn stub_0x145c4(n: usize) -> Vec<ResolutionPreset> {
    // IDA 0x145c4: _Vector_base::_M_allocate -- `__throw_bad_alloc` when n >= 0x40000000
    // (0x145cc), else `operator new(4 * n)`. Capacity-only; length stays 0.
    // IDA 0x145c4
    assert!(n < 0x4000_0000, "bad_alloc");
    Vec::with_capacity(n)
}

// 0x145dc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings16ResolutionPresetES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::CRenderSettings::ResolutionPreset * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *>(RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings16ResolutionPresetES6_EET0_T_S8_S7_")]
pub fn stub_0x145dc(buf: &mut Vec<ResolutionPreset>, src: std::ops::Range<usize>, dest_end: usize) -> usize {
    // IDA 0x145dc: __copy_backward dword loop (unrolled at 0x1460e), moving
    // `[first, last)` to end at `result`. Overlap-safe backward memmove.
    // IDA 0x145dc
    let dest_start = dest_end - src.len();
    buf.copy_within(src, dest_start);
    dest_start
}

// 0x14618 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ResolutionPreset*,std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>>,unsigned long,RBX::CRenderSettings::ResolutionPreset const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_0x14618(vec: &mut Vec<ResolutionPreset>, index: usize, n: usize, value: &ResolutionPreset) {
    // IDA 0x14618: vector::_M_fill_insert -- reallocates and fills when short, else
    // shifts the tail and fill-assigns the gap. `splice` with `repeat` is that.
    // IDA 0x14618
    vec.splice(index..index, std::iter::repeat_n(*value, n));
}

// 0x147a8 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings12QualityLevelESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::QualityLevel,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings12QualityLevelESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_0x147a8<'a>(map: &'a mut QualityLevelNameMap, key: NameKey) -> &'a mut QualityLevel {
    // IDA 0x147a8: map::operator[] -- lower_bound walk, miss inserts a
    // value-initialized node and returns its reference. `entry().or_default()` is that.
    // IDA 0x147a8
    map.entry(key).or_default()
}

// 0x14800 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_0x14800(map: &mut QualityLevelNameMap, key: NameKey, value: QualityLevel) -> bool {
    // IDA 0x14800: _Rb_tree::_M_insert_unique with position hint -- the hint only seeds
    // the search; a present key is a no-op returning the existing node. Returns inserted.
    // IDA 0x14800
    use std::collections::btree_map::Entry;
    match map.entry(key) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

// 0x148b4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_0x148b4(map: &mut QualityLevelNameMap, key: NameKey, value: QualityLevel) -> &mut QualityLevel {
    // IDA 0x148b4: _Rb_tree::_M_insert -- allocates the node, copies the pair,
    // `Rb_tree_insert_and_rebalance`; the caller guarantees a miss, so the node is
    // always linked. `insert` then reborrow the value slot is that link.
    // IDA 0x148b4
    map.insert(key, value);
    map.get_mut(&key).expect("just inserted")
}

// 0x1490c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_0x1490c(map: &mut QualityLevelNameMap, key: NameKey, value: QualityLevel) -> bool {
    // IDA 0x1490c: _Rb_tree::_M_insert_unique(value) without hint -- find-or-link, single
    // node on miss. Returns whether a node was inserted.
    // IDA 0x1490c
    use std::collections::btree_map::Entry;
    match map.entry(key) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

// 0x14974 — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::resize(unsigned long,RBX::CRenderSettings::QualityLevel)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE6resizeEmS2_")]
pub fn stub_0x14974(vec: &mut Vec<QualityLevel>, len: usize, value: QualityLevel) {
    // IDA 0x14974: vector::resize -- shrink truncates the finish pointer, grow delegates
    // to _M_fill_insert.
    // IDA 0x14974
    vec.resize(len, value);
}

// 0x149a8 — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::push_back(RBX::CRenderSettings::QualityLevel const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE9push_backERKS2_")]
pub fn stub_0x149a8(vec: &mut Vec<QualityLevel>, value: &QualityLevel) {
    // IDA 0x149a8: vector::push_back -- fast path stores and bumps finish, full storage
    // delegates to _M_insert_aux.
    // IDA 0x149a8
    vec.push(*value);
}

// 0x149d0 — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::QualityLevel*,std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>>,RBX::CRenderSettings::QualityLevel const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_0x149d0(vec: &mut Vec<QualityLevel>, index: usize, value: &QualityLevel) {
    // IDA 0x149d0: vector::_M_insert_aux -- full storage reallocates (2x or len+1 via
    // _M_allocate, moves elements, constructs the new one); otherwise shifts the tail
    // with __copy_backward and assigns. `Vec::insert` is that.
    // IDA 0x149d0
    vec.insert(index, *value);
}

// 0x14ab4 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings12QualityLevelESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX15CRenderSettings12QualityLevelESaIS2_EE11_M_allocateEm")]
pub fn stub_0x14ab4(n: usize) -> Vec<QualityLevel> {
    // IDA 0x14ab4: _Vector_base::_M_allocate -- `__throw_bad_alloc` when n >= 0x40000000,
    // else `operator new(4 * n)`. Capacity-only; length stays 0.
    // IDA 0x14ab4
    assert!(n < 0x4000_0000, "bad_alloc");
    Vec::with_capacity(n)
}

// 0x14acc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings12QualityLevelES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::CRenderSettings::QualityLevel * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *>(RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings12QualityLevelES6_EET0_T_S8_S7_")]
pub fn stub_0x14acc(buf: &mut Vec<QualityLevel>, src: std::ops::Range<usize>, dest_end: usize) -> usize {
    // IDA 0x14acc: __copy_backward dword loop, moving `[first, last)` to end at
    // `result`. Overlap-safe backward memmove.
    // IDA 0x14acc
    let dest_start = dest_end - src.len();
    buf.copy_within(src, dest_start);
    dest_start
}

// 0x14b08 — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::QualityLevel*,std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>>,unsigned long,RBX::CRenderSettings::QualityLevel const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_0x14b08(vec: &mut Vec<QualityLevel>, index: usize, n: usize, value: &QualityLevel) {
    // IDA 0x14b08: vector::_M_fill_insert -- reallocates and fills when short, else
    // shifts the tail and fill-assigns the gap. `splice` with `repeat` is that.
    // IDA 0x14b08
    vec.splice(index..index, std::iter::repeat_n(*value, n));
}

// 0x14c98 — __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::resize(unsigned long,RBX::CRenderSettings::ShadowMode)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE6resizeEmS2_")]
pub fn stub_0x14c98(vec: &mut Vec<ShadowMode>, len: usize, value: ShadowMode) {
    // IDA 0x14c98: vector::resize -- shrink truncates the finish pointer, grow delegates
    // to _M_fill_insert.
    // IDA 0x14c98
    vec.resize(len, value);
}

// 0x14ccc — __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::push_back(RBX::CRenderSettings::ShadowMode const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE9push_backERKS2_")]
pub fn stub_0x14ccc(vec: &mut Vec<ShadowMode>, value: &ShadowMode) {
    // IDA 0x14ccc: vector::push_back -- fast path stores and bumps finish (decompiled
    // 0x14ccc), full storage delegates to _M_insert_aux.
    // IDA 0x14ccc
    vec.push(*value);
}

// 0x14cf4 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings10ShadowModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::ShadowMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings10ShadowModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_0x14cf4<'a>(map: &'a mut ShadowModeNameMap, key: NameKey) -> &'a mut ShadowMode {
    // IDA 0x14cf4: map::operator[] -- lower_bound walk (same shape as 0x142b8), miss
    // inserts a value-initialized node and returns its reference.
    // IDA 0x14cf4
    map.entry(key).or_default()
}

// 0x14d4c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_0x14d4c(map: &mut ShadowModeNameMap, key: NameKey, value: ShadowMode) -> bool {
    // IDA 0x14d4c: _Rb_tree::_M_insert_unique with position hint -- the hint only seeds
    // the search; a present key is a no-op returning the existing node. Returns inserted.
    // IDA 0x14d4c
    use std::collections::btree_map::Entry;
    match map.entry(key) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

// 0x14e00 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_0x14e00(map: &mut ShadowModeNameMap, key: NameKey, value: ShadowMode) -> &mut ShadowMode {
    // IDA 0x14e00: _Rb_tree::_M_insert -- allocates the node, copies the pair,
    // `Rb_tree_insert_and_rebalance`; the caller guarantees a miss, so the node is
    // always linked. `insert` then reborrow the value slot is that link.
    // IDA 0x14e00
    map.insert(key, value);
    map.get_mut(&key).expect("just inserted")
}

#[cfg(test)]
mod render_settings_item_tests {
    use super::*;
    use parking_lot::Mutex;
    use std::sync::Arc;

    fn connect_log(item: &CRenderSettingsItem) -> (Arc<Mutex<Vec<&'static str>>>, Arc<impl Fn(&'static str) + Send + Sync>) {
        let log = Arc::new(Mutex::new(Vec::new()));
        let inner = Arc::clone(&log);
        let slot = Arc::new(move |name: &'static str| {
            inner.lock().push(name);
        });
        item.property_changed.connect(slot.clone());
        // `Signal` holds only a weak ref: the returned `Arc` keeps the slot live.
        (log, slot)
    }

    #[test]
    fn dword_setter_stores_and_fires_once_on_change() {
        let mut item = CRenderSettingsItem::default();
        let (log, _slot) = connect_log(&item);
        let this = &mut item as *mut CRenderSettingsItem;
        unsafe {
            assert_eq!(stub_0x9608(this, 2), this);
            assert_eq!((*this).graphics_mode, 2);
            // Same value: no store, no signal (IDA 0x960c `IT EQ` / `BXEQ LR`).
            assert_eq!(stub_0x9608(this, 2), this);
            assert_eq!(stub_0x971c(this, 1), this);
            assert_eq!((*this).antialiasing_mode, 1);
        }
        assert_eq!(*log.lock(), vec!["GraphicsMode", "AntialiasingMode"]);
    }

    #[test]
    fn byte_setters_fire_with_own_descriptor_only_on_change() {
        let mut item = CRenderSettingsItem::default();
        let (log, _slot) = connect_log(&item);
        let this = &mut item as *mut CRenderSettingsItem;
        unsafe {
            stub_0x973c(this, true);
            stub_0x973c(this, true);
            stub_0x9760(this, true);
            stub_0x96ac(this, true);
            assert!((*this).debug_show_bounding_boxes);
            assert!((*this).enable_frm);
            assert!((*this).show_aggregation);
        }
        assert_eq!(
            *log.lock(),
            vec!["DebugShowBoundingBoxes", "EnableFRM", "ShowAggregation"]
        );
    }

    #[test]
    fn always_draw_connectors_fires_on_effective_change_only() {
        let mut item = CRenderSettingsItem::default();
        let (log, _slot) = connect_log(&item);
        let this = &mut item as *mut CRenderSettingsItem;
        unsafe {
            // Old effective 0 -> fires; override now set so repeat is silent.
            stub_0x9668(this, true);
            stub_0x9668(this, true);
            // Clearing with base 0: effective 1 -> 0 fires; repeat silent.
            stub_0x9668(this, false);
            stub_0x9668(this, false);
            assert!(!(*this).always_draw_connectors);
        }
        assert_eq!(*log.lock(), vec!["AlwaysDrawConnectors", "AlwaysDrawConnectors"]);
    }

    #[test]
    fn aa_samples_uses_global_but_fires_item_signal() {
        let mut item = CRenderSettingsItem::default();
        let (log, _slot) = connect_log(&item);
        let this = &mut item as *mut CRenderSettingsItem;
        unsafe {
            stub_0x96d0(this, 4);
            assert_eq!(AA_SAMPLES.load(Ordering::SeqCst), 4);
            stub_0x96d0(this, 4);
            // Restore the shared global so sibling tests see the default.
            AA_SAMPLES.store(0, Ordering::SeqCst);
        }
        assert_eq!(*log.lock(), vec!["AASamples"]);
    }

    #[test]
    fn disable_interpolation_round_trips_through_global() {
        let mut item = CRenderSettingsItem::default();
        let this = &mut item as *mut CRenderSettingsItem;
        unsafe {
            let addr = stub_0x9794(this, true);
            assert!(stub_0x9784(this));
            assert!(!addr.is_null());
            assert_eq!(addr, DISABLE_INTERPOLATION.as_ptr());
            stub_0x9794(this, false);
            assert!(!stub_0x9784(this));
        }
    }

    #[test]
    fn resolution_preset_and_auto_quality_store_if_changed() {
        let mut item = CRenderSettingsItem::default();
        let (log, _slot) = connect_log(&item);
        let this = &mut item as *mut CRenderSettingsItem;
        unsafe {
            assert_eq!(stub_0x97a4(this, 3), this);
            assert_eq!((*this).resolution_preset, 3);
            assert_eq!(stub_0x97a4(this, 3), this);
            assert_eq!(stub_0x9ac8(this, 7), this);
            assert_eq!((*this).auto_quality_level, 7);
            assert_eq!(stub_0x9ac8(this, 7), this);
        }
        // IDA 0x9ac8 fires &unk_130C2AC — the QualityLevel descriptor.
        assert_eq!(*log.lock(), vec!["ResolutionPreference", "QualityLevel"]);
    }

    #[test]
    fn cache_size_setters_store_unconditionally_without_signal() {
        let mut item = CRenderSettingsItem::default();
        let (log, _slot) = connect_log(&item);
        let this = &mut item as *mut CRenderSettingsItem;
        unsafe {
            assert_eq!(stub_0x97c0(this, 100), this);
            assert_eq!(stub_0x97c0(this, 100), this);
            assert_eq!(stub_0x97c8(this, 200), this);
            assert_eq!(stub_0x97c8(this, 200), this);
            assert_eq!((*this).texture_cache_size, 100);
            assert_eq!((*this).mesh_cache_size, 200);
        }
        // IDA 0x97c0/0x97c8 are bare `STR.W` + `BX LR`: never fire.
        assert!(log.lock().is_empty());
    }

    #[test]
    fn eager_bulk_execution_fires_only_on_change() {
        let mut item = CRenderSettingsItem::default();
        let (log, _slot) = connect_log(&item);
        let this = &mut item as *mut CRenderSettingsItem;
        unsafe {
            stub_0x9b08(this, true);
            stub_0x9b08(this, true);
            assert!((*this).eager_bulk_execution);
            stub_0x9b08(this, false);
            assert!(!(*this).eager_bulk_execution);
        }
        assert_eq!(*log.lock(), vec!["EagerBulkExecution", "EagerBulkExecution"]);
    }

    #[test]
    fn auto_quality_thunk_adjusts_this_and_delegates() {
        let mut item = CRenderSettingsItem::default();
        let (log, _slot) = connect_log(&item);
        // The thunk views the item through a base 0x60 bytes in.
        let base = (&mut item as *mut CRenderSettingsItem) as *mut u8;
        unsafe {
            let viewed = base.add(0x60);
            assert_eq!(stub_0x9ae8(viewed, 5), viewed);
            assert_eq!(item.auto_quality_level, 5);
            assert_eq!(stub_0x9ae8(viewed, 5), viewed);
        }
        assert_eq!(*log.lock(), vec!["QualityLevel"]);
    }

    #[test]
    fn render_settings_getters_read_their_slots() {
        let mut settings = CRenderSettings {
            graphics_mode: 1,
            antialiasing_mode: 2,
            shadow_mode: 3,
            frame_rate_manager_mode: 4,
            quality_level: 5,
            resolution_preference: 6,
            auto_quality_level: 7,
            debug_show_bounding_boxes: true,
            enable_frm: true,
            show_aggregation: false,
            always_draw_connectors: true,
            max_quality_level: 8,
            video_memory_budget: 39_322_400,
            eager_bulk_execution: true,
            texture_cache_size: 512,
            mesh_cache_size: 256,
        };
        let this = &settings as *const CRenderSettings;
        unsafe {
            assert_eq!(stub_0xb33c(this), 1);
            assert_eq!(stub_0xb444(this), 2);
            assert_eq!(stub_0xb41c(this), 3);
            assert_eq!(stub_0xb364(this), 4);
            assert_eq!(stub_0xb38c(this), 5);
            assert_eq!(stub_0xb4a4(this), 6);
            assert_eq!(stub_0xb474(this), 7);
            assert_eq!(stub_0xb46c(this), 1);
            assert_eq!(stub_0xb49c(this), 1);
            assert_eq!(stub_0xb3e0(this), 0);
            assert_eq!(stub_0xb3b4(this), 1);
            // IDA 0xb4cc/0xb4f4/0xb4f8/0xb8b0: +0x20/+0x40/+0x44/+0x3D loads.
            assert_eq!(stub_0xb4cc(this), 8);
            assert_eq!(stub_0xb4f4(this), 512);
            assert_eq!(stub_0xb4f8(this), 256);
            assert_eq!(stub_0xb8b0(this), 1);
        }
        // The aaSamples getter reads the global, ignoring `this`.
        AA_SAMPLES.store(9, Ordering::SeqCst);
        assert_eq!(stub_0xb3e8(this), 9);
        AA_SAMPLES.store(0, Ordering::SeqCst);
        let _ = &mut settings;
    }

    #[test]
    fn render_settings_name_declare_is_idempotent() {
        // IDA 0xf1dc: first call declares, later calls return the cache.
        assert_eq!(stub_0xf1d8(), "RenderSettings");
        assert_eq!(stub_0xf1dc(), "RenderSettings");
        assert_eq!(stub_0xf1d8(), "RenderSettings");
    }

    #[test]
    fn vector_push_back_grows_through_insert_aux() {
        let pair = Vector2int16 { x: 800, y: 600 };
        let mut vec = Vec::new();
        // Empty: `finish == end_of_storage`, so the 0xf704 slow path grows.
        stub_0xb740(&mut vec, &pair);
        assert_eq!(vec, vec![pair]);
        // Spare capacity: the inline fast path copies + bumps.
        let mut roomy = Vec::with_capacity(4);
        stub_0xb740(&mut roomy, &pair);
        stub_0xb740(&mut roomy, &Vector2int16 { x: 1, y: 2 });
        assert_eq!(
            roomy,
            vec![pair, Vector2int16 { x: 1, y: 2 }]
        );
    }

    #[test]
    fn vector_insert_aux_shifts_middle() {
        let (a, b, c) = (
            Vector2int16 { x: 1, y: 1 },
            Vector2int16 { x: 2, y: 2 },
            Vector2int16 { x: 3, y: 3 },
        );
        let mut vec = vec![a, b, c];
        let mid = Vector2int16 { x: 9, y: 9 };
        stub_0xf704(&mut vec, 1, mid);
        assert_eq!(vec, vec![a, mid, b, c]);
        // Past-the-end clamps like an `end()` iterator.
        stub_0xf704(&mut vec, 99, mid);
        assert_eq!(*vec.last().unwrap(), mid);
    }

    #[test]
    fn vector_allocate_reserves_scaled_storage() {
        let buf = stub_0xf7e8(16);
        assert!(buf.capacity() >= 16 && buf.is_empty());
        assert_eq!(core::mem::size_of::<Vector2int16>(), 4);
    }

    #[test]
    #[should_panic(expected = "std::bad_alloc")]
    fn vector_allocate_rejects_huge() {
        // IDA 0xf7f0 `n >= 0x40000000` throws `bad_alloc`.
        let _ = stub_0xf7e8(0x4000_0000);
    }

    #[test]
    fn copy_backward_shifts_overlapping_ranges() {
        let cell = |x: i16| Vector2int16 { x, y: x };
        let mut items = vec![cell(1), cell(2), cell(3), cell(4)];
        // Shift `[first, first + 3)` to end at `first + 4` (overlap).
        let end = unsafe {
            let first = items.as_ptr();
            let result = items.as_mut_ptr().add(4);
            let back = stub_0xf800(first, first.add(3), result);
            assert_eq!(back, items.as_mut_ptr().add(1));
            back
        };
        assert_eq!(items, vec![cell(1), cell(1), cell(2), cell(3)]);
        let _ = end;
        // Empty range: no write, returns `result` unchanged.
        let mut solo = vec![cell(7)];
        unsafe {
            let first = solo.as_ptr();
            let result = solo.as_mut_ptr().add(1);
            assert_eq!(stub_0xf800(first, first, result), result);
        }
        assert_eq!(solo, vec![cell(7)]);
    }

    #[test]
    fn control_block_counts_and_deleter_match() {
        use rbx_core::shared_ptr::CREATABLE_INSTANCE_DELETER_TYPE_NAME;
        let mut block = stub_0xf098(Box::new(CRenderSettingsItem::default()));
        // IDA 0xf0fa: fresh `use_count` is 1 (`weak_count` likewise, untracked here).
        assert_eq!(block.use_count(), 1);
        assert!(block.get().is_some());
        // IDA 0xf1c0..0xf1d2: deleter iff the type name matches.
        assert!(stub_0xf1bc(&block, CREATABLE_INSTANCE_DELETER_TYPE_NAME).is_some());
        assert!(stub_0xf1bc(&block, "i").is_none());
        // IDA 0xf1d6: untyped deleter is unconditional.
        let _ = stub_0xf1d4(&block);
        // IDA 0xf19c..0xf1b8: dispose drops the pointee.
        stub_0xf19c(&mut block);
        assert!(block.get().is_none());
    }

    #[test]
    fn use_count_tracks_shared_owners() {
        let shared = stub_0xefb4(Box::new(CRenderSettingsItem::default()));
        assert_eq!(stub_0xefd8(&shared), 1);
        let second = SharedPtr::clone(&shared);
        assert_eq!(stub_0xefd8(&shared), 2);
        drop(second);
        assert_eq!(stub_0xefd8(&shared), 1);
    }

    #[test]
    fn creator_and_singleton_construction_flow() {
        // IDA 0xf2f4..0xf422: declare-once, register, `isConstructedE = 666`.
        // Idempotent: a second construction re-registers nothing new.
        let before = CLASS_REGISTRAR_COUNT.load(Ordering::SeqCst);
        let creator = stub_0xf2bc();
        assert_eq!(
            CREATOR_IS_CONSTRUCTED.load(Ordering::SeqCst),
            CREATOR_CONSTRUCTED_MAGIC
        );
        assert!(CREATOR_REGISTRY.lock().contains(&"RenderSettings"));
        assert!(std::ptr::eq(creator, stub_0xf2bc()));
        // Name + class-name chain resolve to the declared product name.
        assert_eq!(stub_0xf1dc(), "RenderSettings");
        assert_eq!(stub_0xf500() as *const _, creator as *const _);
        assert_eq!(stub_0xedfc(), "RenderSettings");
        assert_eq!(stub_0xb8d0(), "RenderSettings");
        assert_eq!(stub_0xb900(), "RenderSettings");
        // The only in-test construction: base C2 (0xb4fc) then item C2
        // (0x97d0) then adopt (0xefb4), via 0xee84 into caller storage.
        let mut slot = std::mem::MaybeUninit::<SharedPtr<CRenderSettingsItem>>::uninit();
        unsafe {
            let out = stub_0xee84(slot.as_mut_ptr());
            assert_eq!(out, slot.as_mut_ptr());
            let created = slot.assume_init_read();
            // IDA 0x987e/0x988a + 0x991a: first resolution 800x600 pushed.
            assert_eq!(created.first_resolution, Vector2int16 { x: 800, y: 600 });
            assert_eq!(created.resolutions, vec![Vector2int16 { x: 800, y: 600 }]);
            assert!(created.byte_189);
            assert!(created.string_168.is_empty());
            // IDA 0x9904: derived name wins over the base 0xb5f8 store.
            assert_eq!(created.render_category, "Rendering");
            // IDA 0x9922..0x9946: host takes the low-budget arm.
            assert_eq!(created.video_memory_budget, VIDEO_BUDGET_LOW);
            assert_eq!(stub_0xefd8(&created), 1);
        }
        assert_eq!(CLASS_REGISTRAR_COUNT.load(Ordering::SeqCst), before + 1);
    }
}
