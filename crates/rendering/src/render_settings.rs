//! RBX::CRenderSettings / CRenderSettingsItem — mirrors Client/App/rendering/*
//! Generated from ida/export.json filtered for RenderSettings / CRenderSettings (734 funcs, next 50 stubs here)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use rbx_core::SharedPtr;

use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

use rbx_core::signal::Signal;

/// Change-notification payload: EA of the `PropertyDescriptor const*` the
/// original passes to `signal_with_args<1,...>::operator()` (the `+192`
/// signal on the item). Kept as the image EA so `rg` finds either form.
pub type DescriptorEa = u32;

pub const DESC_ALWAYS_DRAW_CONNECTORS: DescriptorEa = 0x0130_C030; // IDA 0x9668 &unk_130C030
pub const DESC_SHOW_AGGREGATION: DescriptorEa = 0x0130_C05C; // IDA 0x96ac &unk_130C05C
pub const DESC_DEBUG_SHOW_BOUNDING_BOXES: DescriptorEa = 0x0130_C0E0; // IDA 0x973c &unk_130C0E0
pub const DESC_ENABLE_FRM: DescriptorEa = 0x0130_C138; // IDA 0x9760 &unk_130C138
pub const DESC_EAGER_BULK_EXECUTION: DescriptorEa = 0x0130_C1E8; // IDA 0x9b08 &unk_130C1E8
pub const DESC_GRAPHICS_MODE: DescriptorEa = 0x0130_C244; // IDA 0x9608 &unk_130C244
pub const DESC_FRAME_RATE_MANAGER_MODE: DescriptorEa = 0x0130_C278; // IDA 0x9628 &unk_130C278
pub const DESC_QUALITY_LEVEL: DescriptorEa = 0x0130_C2AC; // IDA 0x9648/0x9ac8/0x9ae8 &unk_130C2AC
pub const DESC_AA_SAMPLES: DescriptorEa = 0x0130_C2E0; // IDA 0x96d0 &unk_130C2E0
pub const DESC_SHADOW_MODE: DescriptorEa = 0x0130_C314; // IDA 0x96fc &unk_130C314
pub const DESC_ANTIALIASING_MODE: DescriptorEa = 0x0130_C348; // IDA 0x971c &unk_130C348
pub const DESC_RESOLUTION_PREFERENCE: DescriptorEa = 0x012D_2C78; // IDA 0x97a4 CRenderSettingsItem::prop_resolution

/// `GetDXVideoMemorySize` tier threshold from the ctor branch
/// (IDA 0x97d0: `vram > &loc_F423FC + 3`, i.e. `> 0x00F423FF`).
pub const DX_VIDEO_MEMORY_THRESHOLD: u32 = 0x00F4_23FF;
/// `*(this + 146)` values selected by the VRAM branch (IDA 0x97d0).
pub const VIDEO_TIER_LOW_VRAM: u32 = 39_322_400;
pub const VIDEO_TIER_HIGH_VRAM: u32 = 50_332_672;

/// `RBX::CRenderSettings::aaSamples` static: IDA 0x96d0/0xb3e8 read/write a
/// global, not a field. Atomic models the shared word without `unsafe`.
static AA_SAMPLES: AtomicI32 = AtomicI32::new(0);
/// `RBX::PartInstance::disableInterpolation` global (IDA 0x9784/0x9794).
static DISABLE_INTERPOLATION: AtomicBool = AtomicBool::new(false);

/// `RBX::CRenderSettings` — plain settings block embedded at `+96` in
/// `CRenderSettingsItem` (IDA 0x97d0 builds it at `(char *)this + 96`;
/// getters at 0xb33c.. take it directly as `this`).
///
/// `#[repr(C)]` scalar layout; offsets verified against the IDA decompiles
/// cited on each accessor. Total 72 bytes: item `+96..+168`.
#[repr(C)]
#[derive(Debug, Default)]
pub struct RenderSettings {
    pub unk_word0: i32,               // +0
    pub graphics_mode: i32,           // +4  (IDA 0xb33c words[1]; set 0x9608)
    pub antialiasing_mode: i32,       // +8  (IDA 0xb444 words[2]; set 0x971c)
    pub shadow_mode: i32,             // +12 (IDA 0xb41c words[3]; set 0x96fc)
    pub frame_rate_manager_mode: i32, // +16 (IDA 0xb364 words[4]; set 0x9628)
    pub quality_level: i32,           // +20 (IDA 0xb38c words[5]; set 0x9648)
    pub resolution_preference: i32,   // +24 (IDA 0xb4a4 words[6]; set 0x97a4)
    pub auto_quality_level: i32,      // +28 (IDA 0xb474 words[7]; set 0x9ac8/0x9ae8)
    pub max_quality_level: i32,       // +32 (IDA 0xb4cc words[8])
    pub unk_36: [u8; 4],              // +36
    pub debug_show_bounding_boxes: bool, // +40 (IDA 0xb46c; set 0x973c)
    pub enable_frm: bool,             // +41 (IDA 0xb49c; set 0x9760)
    pub unk_42: [u8; 8], // +42
    // +50: unaligned DWORD in the original (`*(_DWORD *)((char *)this + 146)`;
    // 50 % 4 == 2, so no naturally-aligned u32 can sit here). Stored as bytes;
    // use video_tier()/set_video_tier() (little-endian, matching armv7).
    pub unk_50_video_tier: [u8; 4],
    pub unk_54: [u8; 4], // +54
    pub show_aggregation: bool,       // +58 (IDA 0xb3e0; set 0x96ac)
    pub always_draw_connectors: bool, // +59 (IDA 0xb3b4; set 0x9668)
    pub connector_secondary: bool,    // +60 (IDA 0x9668 folds the +156 byte)
    pub eager_bulk_execution: bool,   // +61 (IDA 0x9b08 writes +157)
    pub unk_62: [u8; 2],              // +62
    pub texture_cache_size: u32,      // +64 (IDA 0x97c0 writes item +160)
    pub mesh_cache_size: u32,         // +68 (IDA 0x97c8 writes item +164)
}

impl RenderSettings {
    pub fn video_tier(&self) -> u32 {
        u32::from_le_bytes(self.unk_50_video_tier)
    }
    pub fn set_video_tier(&mut self, tier: u32) {
        self.unk_50_video_tier = tier.to_le_bytes();
    }
}

/// `CRenderSettingsItem` — `GlobalAdvancedSettingsItem` head (vtables,
/// 96 bytes) + embedded [`RenderSettings`] at `+96` + trailing storage
/// (IDA 0x97d0): `std::string` rep at `+168`, default `Vector2int16` at
/// `+172`, `vector<Vector2int16>` at `+176`, flag byte at `+189`, and the
/// property-changed signal at `+192` (`boost::signals` → [`Signal`]).
pub struct RenderSettingsItem {
    pub head: [u8; 96],
    pub settings: RenderSettings,
    pub name_rep: u32, // +168: std::string rep pointer; 0 models the empty rep
    pub default_resolution: [i16; 2], // +172: 800x600 (IDA 0x97d0 words 86/87)
    pub resolutions: Vec<[i16; 2]>, // +176: vector<G3D::Vector2int16>; ctor pushes the default
    pub unk_188: u8,      // +188
    pub unk_189_flag: bool, // +189: ctor sets 1
    pub _pad_190: [u8; 2], // +190
    pub changed: Signal<u32>, // +192: fired with the descriptor EA on real changes
}

// 0x850c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::EnumDesc(void)")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::EnumDesc(void)
// IDA 0x850c: 158 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_850c() {
}

// 0x86d0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::EnumDesc(void)")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::EnumDesc(void)
// IDA 0x86d0: 175 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86d0() {
}

// 0x88c4 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::EnumDesc(void)")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::EnumDesc(void)
// IDA 0x88c4: 158 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88c4() {
}

// 0x8a88 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::EnumDesc(void)")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::EnumDesc(void)
// IDA 0x8a88: 158 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8a88() {
}

// 0x8c4c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::EnumDesc(void)")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::EnumDesc(void)
// IDA 0x8c4c: 166 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c4c() {
}

// 0x8e24 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::EnumDesc(void)")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::EnumDesc(void)
// IDA 0x8e24: 262 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8e24() {
}

// 0x9100 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::EnumDesc(void)")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::EnumDesc(void)
// IDA 0x9100: 453 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_9100() {
}

// 0x9608 — __ZN19CRenderSettingsItem15setGraphicsModeEN3RBX15CRenderSettings12GraphicsModeE
#[doc(alias = "CRenderSettingsItem::setGraphicsMode(RBX::CRenderSettings::GraphicsMode)")]
// was: CRenderSettingsItem::setGraphicsMode(RBX::CRenderSettings::GraphicsMode)
// IDA 0x9608: guarded word store to +100; fires changed(&unk_130C244) at +192 only on change, else early-out.
// FIDELITY: original returns this-or-signal int; Rust returns whether the change signal fired.
pub fn stub_9608(item: &mut RenderSettingsItem, mode: i32) -> bool {
    if item.settings.graphics_mode != mode {
        item.settings.graphics_mode = mode;
        item.changed.fire(DESC_GRAPHICS_MODE);
        return true;
    }
    false
}

// 0x9628 — __ZN19CRenderSettingsItem23setFrameRateManagerModeEN3RBX15CRenderSettings20FrameRateManagerModeE
#[doc(alias = "CRenderSettingsItem::setFrameRateManagerMode(RBX::CRenderSettings::FrameRateManagerMode)")]
// was: CRenderSettingsItem::setFrameRateManagerMode(RBX::CRenderSettings::FrameRateManagerMode)
// IDA 0x9628: guarded word store to +112; fires changed(&unk_130C278) at +192 only on change.
// FIDELITY: original returns this-or-signal int; Rust returns whether the change signal fired.
pub fn stub_9628(item: &mut RenderSettingsItem, mode: i32) -> bool {
    if item.settings.frame_rate_manager_mode != mode {
        item.settings.frame_rate_manager_mode = mode;
        item.changed.fire(DESC_FRAME_RATE_MANAGER_MODE);
        return true;
    }
    false
}

// 0x9648 — __ZN19CRenderSettingsItem15setQualityLevelEN3RBX15CRenderSettings12QualityLevelE
#[doc(alias = "CRenderSettingsItem::setQualityLevel(RBX::CRenderSettings::QualityLevel)")]
// was: CRenderSettingsItem::setQualityLevel(RBX::CRenderSettings::QualityLevel)
// IDA 0x9648: guarded word store to +116; fires changed(&unk_130C2AC) at +192 only on change.
// FIDELITY: original returns this-or-signal int; Rust returns whether the change signal fired.
pub fn stub_9648(item: &mut RenderSettingsItem, level: i32) -> bool {
    if item.settings.quality_level != level {
        item.settings.quality_level = level;
        item.changed.fire(DESC_QUALITY_LEVEL);
        return true;
    }
    false
}

// 0x9668 — __ZN19CRenderSettingsItem23setAlwaysDrawConnectorsEb
#[doc(alias = "CRenderSettingsItem::setAlwaysDrawConnectors(bool)")]
// was: CRenderSettingsItem::setAlwaysDrawConnectors(bool)
// IDA 0x9668: effective-flag fold of the +155 request byte with the +156 secondary byte; fires changed(&unk_130C030) at +192 only when the effective value flips.
// FIDELITY: original returns this-or-signal int; Rust returns whether the change signal fired.
pub fn stub_9668(item: &mut RenderSettingsItem, enabled: bool) -> bool {
    // Old effective value: +155 set ? 1 : normalize(+156). (IDA normalizes the
    // +156 byte with `if (*(this + 156)) v2 = 1`; Rust bools are pre-normalized.)
    let mut old_effective = true;
    if !item.settings.always_draw_connectors {
        old_effective = item.settings.connector_secondary;
    }
    item.settings.always_draw_connectors = enabled;
    if enabled {
        if old_effective {
            return false;
        }
        item.changed.fire(DESC_ALWAYS_DRAW_CONNECTORS);
        return true;
    }
    let new_effective = item.settings.connector_secondary;
    if old_effective != new_effective {
        item.changed.fire(DESC_ALWAYS_DRAW_CONNECTORS);
        return true;
    }
    false
}

// 0x96ac — __ZN19CRenderSettingsItem18setShowAggregationEb
#[doc(alias = "CRenderSettingsItem::setShowAggregation(bool)")]
// was: CRenderSettingsItem::setShowAggregation(bool)
// IDA 0x96ac: guarded byte store to +154; fires changed(&unk_130C05C) at +192 only on change.
// FIDELITY: original returns this-or-signal int; Rust returns whether the change signal fired.
pub fn stub_96ac(item: &mut RenderSettingsItem, show: bool) -> bool {
    if item.settings.show_aggregation != show {
        item.settings.show_aggregation = show;
        item.changed.fire(DESC_SHOW_AGGREGATION);
        return true;
    }
    false
}

// 0x96d0 — __ZN19CRenderSettingsItem12setAASamplesEN3RBX15CRenderSettings9AASamplesE
#[doc(alias = "CRenderSettingsItem::setAASamples(RBX::CRenderSettings::AASamples)")]
// was: CRenderSettingsItem::setAASamples(RBX::CRenderSettings::AASamples)
// IDA 0x96d0: guarded store to the RBX::CRenderSettings::aaSamples global (not a field); fires changed(&unk_130C2E0) at +192 only on change.
// FIDELITY: original returns this-or-signal int; Rust returns whether the change signal fired.
pub fn stub_96d0(item: &mut RenderSettingsItem, samples: i32) -> bool {
    if AA_SAMPLES.load(Ordering::Relaxed) != samples {
        AA_SAMPLES.store(samples, Ordering::Relaxed);
        item.changed.fire(DESC_AA_SAMPLES);
        return true;
    }
    false
}

// 0x96fc — __ZN19CRenderSettingsItem13setShadowModeEN3RBX15CRenderSettings10ShadowModeE
#[doc(alias = "CRenderSettingsItem::setShadowMode(RBX::CRenderSettings::ShadowMode)")]
// was: CRenderSettingsItem::setShadowMode(RBX::CRenderSettings::ShadowMode)
// IDA 0x96fc: guarded word store to +108; fires changed(&unk_130C314) at +192 only on change.
// FIDELITY: original returns this-or-signal int; Rust returns whether the change signal fired.
pub fn stub_96fc(item: &mut RenderSettingsItem, mode: i32) -> bool {
    if item.settings.shadow_mode != mode {
        item.settings.shadow_mode = mode;
        item.changed.fire(DESC_SHADOW_MODE);
        return true;
    }
    false
}

// 0x971c — __ZN19CRenderSettingsItem19setAntialiasingModeEN3RBX15CRenderSettings16AntialiasingModeE
#[doc(alias = "CRenderSettingsItem::setAntialiasingMode(RBX::CRenderSettings::AntialiasingMode)")]
// was: CRenderSettingsItem::setAntialiasingMode(RBX::CRenderSettings::AntialiasingMode)
// IDA 0x971c: guarded word store to +104; fires changed(&unk_130C348) at +192 only on change.
// FIDELITY: original returns this-or-signal int; Rust returns whether the change signal fired.
pub fn stub_971c(item: &mut RenderSettingsItem, mode: i32) -> bool {
    if item.settings.antialiasing_mode != mode {
        item.settings.antialiasing_mode = mode;
        item.changed.fire(DESC_ANTIALIASING_MODE);
        return true;
    }
    false
}

// 0x973c — __ZN19CRenderSettingsItem25setDebugShowBoundingBoxesEb
#[doc(alias = "CRenderSettingsItem::setDebugShowBoundingBoxes(bool)")]
// was: CRenderSettingsItem::setDebugShowBoundingBoxes(bool)
// IDA 0x973c: guarded byte store to +136; fires changed(&unk_130C0E0) at +192 only on change.
// FIDELITY: original returns this-or-signal int; Rust returns whether the change signal fired.
pub fn stub_973c(item: &mut RenderSettingsItem, show: bool) -> bool {
    if item.settings.debug_show_bounding_boxes != show {
        item.settings.debug_show_bounding_boxes = show;
        item.changed.fire(DESC_DEBUG_SHOW_BOUNDING_BOXES);
        return true;
    }
    false
}

// 0x9760 — __ZN19CRenderSettingsItem12setEnableFRMEb
#[doc(alias = "CRenderSettingsItem::setEnableFRM(bool)")]
// was: CRenderSettingsItem::setEnableFRM(bool)
// IDA 0x9760: guarded byte store to +137; fires changed(&unk_130C138) at +192 only on change.
// FIDELITY: original returns this-or-signal int; Rust returns whether the change signal fired.
pub fn stub_9760(item: &mut RenderSettingsItem, enable: bool) -> bool {
    if item.settings.enable_frm != enable {
        item.settings.enable_frm = enable;
        item.changed.fire(DESC_ENABLE_FRM);
        return true;
    }
    false
}

// 0x9784 — __ZNK19CRenderSettingsItem28getDebugDisableInterpolationEv
#[doc(alias = "CRenderSettingsItem::getDebugDisableInterpolation(void)const")]
// was: CRenderSettingsItem::getDebugDisableInterpolation(void)const
// IDA 0x9784: returns the RBX::PartInstance::disableInterpolation global byte; no self access, no signal.
// FIDELITY: 1:1 field/global read; return type fixed from IDA signature.
pub fn stub_9784() -> bool {
    DISABLE_INTERPOLATION.load(Ordering::Relaxed)
}

// 0x9794 — __ZN19CRenderSettingsItem28setDebugDisableInterpolationEb
#[doc(alias = "CRenderSettingsItem::setDebugDisableInterpolation(bool)")]
// was: CRenderSettingsItem::setDebugDisableInterpolation(bool)
// IDA 0x9794: unconditional byte store to the RBX::PartInstance::disableInterpolation global; no signal.
// FIDELITY: original returns &global (char *); Rust returns the stored byte. Unconditional write, no change signal.
pub fn stub_9794(value: bool) -> bool {
    DISABLE_INTERPOLATION.store(value, Ordering::Relaxed);
    value
}

// 0x97a4 — __ZN19CRenderSettingsItem23setResolutionPreferenceEN3RBX15CRenderSettings16ResolutionPresetE
#[doc(alias = "CRenderSettingsItem::setResolutionPreference(RBX::CRenderSettings::ResolutionPreset)")]
// was: CRenderSettingsItem::setResolutionPreference(RBX::CRenderSettings::ResolutionPreset)
// IDA 0x97a4: guarded word store to +120; fires changed(&CRenderSettingsItem::prop_resolution = 0x12D2C78) at +192 only on change.
// FIDELITY: original returns this-or-signal int; Rust returns whether the change signal fired.
pub fn stub_97a4(item: &mut RenderSettingsItem, preset: i32) -> bool {
    if item.settings.resolution_preference != preset {
        item.settings.resolution_preference = preset;
        item.changed.fire(DESC_RESOLUTION_PREFERENCE);
        return true;
    }
    false
}

// 0x97c0 — __ZN19CRenderSettingsItem19setTextureCacheSizeEj
#[doc(alias = "CRenderSettingsItem::setTextureCacheSize(unsigned int)")]
// was: CRenderSettingsItem::setTextureCacheSize(unsigned int)
// IDA 0x97c0: unconditional word store to +160 (STR.W then BX); no compare, no signal.
// FIDELITY: original returns this with no signal on this path; Rust returns false (no signal fired).
pub fn stub_97c0(item: &mut RenderSettingsItem, size: u32) -> bool {
    item.settings.texture_cache_size = size;
    false
}

// 0x97c8 — __ZN19CRenderSettingsItem16setMeshCacheSizeEj
#[doc(alias = "CRenderSettingsItem::setMeshCacheSize(unsigned int)")]
// was: CRenderSettingsItem::setMeshCacheSize(unsigned int)
// IDA 0x97c8: unconditional word store to +164 (STR.W then BX); no compare, no signal.
// FIDELITY: original returns this with no signal on this path; Rust returns false (no signal fired).
pub fn stub_97c8(item: &mut RenderSettingsItem, size: u32) -> bool {
    item.settings.mesh_cache_size = size;
    false
}

// 0x97d0 — __ZN19CRenderSettingsItemC2Ev
#[doc(alias = "CRenderSettingsItem::CRenderSettingsItem(void)")]
// was: CRenderSettingsItem::CRenderSettingsItem(void)
// IDA 0x97d0: ctor: GlobalAdvancedSettingsItem base + CRenderSettings at +96, empty string at +168, default 800x600 Vector2int16 at +172 pushed into the +176 vector, flag byte 1 at +189, signal init at +192, and *(+146) from the GetDXVideoMemorySize tier branch.
// FIDELITY: vtable/base-class and string/vector-allocator mechanics modeled with Rust equivalents (Vec, u32 rep); observable defaults and the VRAM tier branch are 1:1. Takes the VRAM size the original reads via GetDXVideoMemorySize().
pub fn stub_97d0(dx_video_memory_bytes: u32) -> RenderSettingsItem {
    let mut settings = RenderSettings::default();
    // IDA 0x97d0: `*(_DWORD *)(this + 146)` — the unaligned +50 word — selects
    // the tier by the GetDXVideoMemorySize() branch.
    settings.set_video_tier(if dx_video_memory_bytes > DX_VIDEO_MEMORY_THRESHOLD {
        VIDEO_TIER_HIGH_VRAM
    } else {
        VIDEO_TIER_LOW_VRAM
    });
    RenderSettingsItem {
        head: [0; 96],
        settings,
        name_rep: 0,
        default_resolution: [800, 600],
        resolutions: vec![[800, 600]],
        unk_188: 0,
        unk_189_flag: true,
        _pad_190: [0; 2],
        changed: Signal::new(),
    }
}

// 0x9ac8 — __ZN19CRenderSettingsItem19setAutoQualityLevelEi
#[doc(alias = "CRenderSettingsItem::setAutoQualityLevel(int)")]
// was: CRenderSettingsItem::setAutoQualityLevel(int)
// IDA 0x9ac8: guarded word store to +124; fires changed(&unk_130C2AC, shared with quality level) at +192 only on change.
// FIDELITY: original returns this-or-signal int; Rust returns whether the change signal fired.
pub fn stub_9ac8(item: &mut RenderSettingsItem, level: i32) -> bool {
    if item.settings.auto_quality_level != level {
        item.settings.auto_quality_level = level;
        item.changed.fire(DESC_QUALITY_LEVEL);
        return true;
    }
    false
}

// 0x9ae8 — __ZThn96_N19CRenderSettingsItem19setAutoQualityLevelEi
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::setAutoQualityLevel(int)")]
// was: non-virtual thunk toCRenderSettingsItem::setAutoQualityLevel(int)
// IDA 0x9ae8: non-virtual __ZThn96_ thunk to setAutoQualityLevel: entry this points at the +96 subobject (compares *(this + 28)), then adjusts back (v2 = this - 96) to write +124 and fire +192.
// FIDELITY: the -96 adjustment is a no-op in Rust (the item is passed directly); subobject+28 == item+124 is preserved, behavior identical to 0x9ac8.
pub fn stub_9ae8(item: &mut RenderSettingsItem, level: i32) -> bool {
    stub_9ac8(item, level)
}

// 0x9b08 — __ZN19CRenderSettingsItem21setEagerBulkExecutionEb
#[doc(alias = "CRenderSettingsItem::setEagerBulkExecution(bool)")]
// was: CRenderSettingsItem::setEagerBulkExecution(bool)
// IDA 0x9b08: guarded byte store to +157; fires changed(&unk_130C1E8) at +192 only on change.
// FIDELITY: original returns this-or-signal int; Rust returns whether the change signal fired.
pub fn stub_9b08(item: &mut RenderSettingsItem, eager: bool) -> bool {
    if item.settings.eager_bulk_execution != eager {
        item.settings.eager_bulk_execution = eager;
        item.changed.fire(DESC_EAGER_BULK_EXECUTION);
        return true;
    }
    false
}

// 0x9b48 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::addPair(RBX::CRenderSettings::AASamples,char const*)")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::addPair(RBX::CRenderSettings::AASamples,char const*)
// IDA 0x9b48: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_9b48() {
}

// 0x9ea8 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::addPair(RBX::CRenderSettings::GraphicsMode,char const*)")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::addPair(RBX::CRenderSettings::GraphicsMode,char const*)
// IDA 0x9ea8: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_9ea8() {
}

// 0xa208 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE9addLegacyEiPKcS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::addLegacy(int,char const*,RBX::CRenderSettings::GraphicsMode)")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::addLegacy(int,char const*,RBX::CRenderSettings::GraphicsMode)
// IDA 0xa208: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a208() {
}

// 0xa25c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::addPair(RBX::CRenderSettings::FrameRateManagerMode,char const*)")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::addPair(RBX::CRenderSettings::FrameRateManagerMode,char const*)
// IDA 0xa25c: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a25c() {
}

// 0xa5bc — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::addPair(RBX::CRenderSettings::AntialiasingMode,char const*)")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::addPair(RBX::CRenderSettings::AntialiasingMode,char const*)
// IDA 0xa5bc: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a5bc() {
}

// 0xa91c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::addPair(RBX::CRenderSettings::ShadowMode,char const*)")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::addPair(RBX::CRenderSettings::ShadowMode,char const*)
// IDA 0xa91c: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a91c() {
}

// 0xac7c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::addPair(RBX::CRenderSettings::QualityLevel,char const*)")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::addPair(RBX::CRenderSettings::QualityLevel,char const*)
// IDA 0xac7c: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ac7c() {
}

// 0xafdc — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::addPair(RBX::CRenderSettings::ResolutionPreset,char const*)")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::addPair(RBX::CRenderSettings::ResolutionPreset,char const*)
// IDA 0xafdc: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_afdc() {
}

// 0xb33c — __ZNK3RBX15CRenderSettings15getGraphicsModeEv
#[doc(alias = "RBX::CRenderSettings::getGraphicsMode(void)const")]
// was: RBX::CRenderSettings::getGraphicsMode(void)const
// IDA 0xb33c: LDR words[1] of RBX::CRenderSettings (settings +4 == item +100).
// FIDELITY: 1:1 field/global read; return type fixed from IDA signature.
pub fn stub_b33c(settings: &RenderSettings) -> i32 {
    settings.graphics_mode
}

// 0xb340 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::~EnumPropDescriptor()")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::~EnumPropDescriptor()
// IDA 0xb340: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b340() {
}

// 0xb364 — __ZNK3RBX15CRenderSettings23getFrameRateManagerModeEv
#[doc(alias = "RBX::CRenderSettings::getFrameRateManagerMode(void)const")]
// was: RBX::CRenderSettings::getFrameRateManagerMode(void)const
// IDA 0xb364: LDR words[4] of RBX::CRenderSettings (settings +16 == item +112).
// FIDELITY: 1:1 field/global read; return type fixed from IDA signature.
pub fn stub_b364(settings: &RenderSettings) -> i32 {
    settings.frame_rate_manager_mode
}

// 0xb368 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::~EnumPropDescriptor()")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::~EnumPropDescriptor()
// IDA 0xb368: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b368() {
}

// 0xb38c — __ZNK3RBX15CRenderSettings15getQualityLevelEv
#[doc(alias = "RBX::CRenderSettings::getQualityLevel(void)const")]
// was: RBX::CRenderSettings::getQualityLevel(void)const
// IDA 0xb38c: LDR words[5] of RBX::CRenderSettings (settings +20 == item +116).
// FIDELITY: 1:1 field/global read; return type fixed from IDA signature.
pub fn stub_b38c(settings: &RenderSettings) -> i32 {
    settings.quality_level
}

// 0xb390 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::~EnumPropDescriptor()")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::~EnumPropDescriptor()
// IDA 0xb390: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b390() {
}

// 0xb3b4 — __ZNK3RBX15CRenderSettings23getAlwaysDrawConnectorsEv
#[doc(alias = "RBX::CRenderSettings::getAlwaysDrawConnectors(void)const")]
// was: RBX::CRenderSettings::getAlwaysDrawConnectors(void)const
// IDA 0xb3b4: LDRB.W byte +59 of RBX::CRenderSettings (item +155).
// FIDELITY: 1:1 field/global read; return type fixed from IDA signature.
pub fn stub_b3b4(settings: &RenderSettings) -> bool {
    settings.always_draw_connectors
}

// 0xb3bc — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::~PropDescriptor()
// IDA 0xb3bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b3bc() {
}

// 0xb3e0 — __ZNK3RBX15CRenderSettings18getShowAggregationEv
#[doc(alias = "RBX::CRenderSettings::getShowAggregation(void)const")]
// was: RBX::CRenderSettings::getShowAggregation(void)const
// IDA 0xb3e0: LDRB.W byte +58 of RBX::CRenderSettings (item +154).
// FIDELITY: 1:1 field/global read; return type fixed from IDA signature.
pub fn stub_b3e0(settings: &RenderSettings) -> bool {
    settings.show_aggregation
}

// 0xb3e8 — __ZNK3RBX15CRenderSettings12getAASamplesEv
#[doc(alias = "RBX::CRenderSettings::getAASamples(void)const")]
// was: RBX::CRenderSettings::getAASamples(void)const
// IDA 0xb3e8: MOV from the RBX::CRenderSettings::aaSamples global (5 insns); not a field.
// FIDELITY: 1:1 field/global read; return type fixed from IDA signature.
pub fn stub_b3e8() -> i32 {
    AA_SAMPLES.load(Ordering::Relaxed)
}

// 0xb3f8 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::~EnumPropDescriptor()")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::~EnumPropDescriptor()
// IDA 0xb3f8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b3f8() {
}

// 0xb41c — __ZNK3RBX15CRenderSettings13getShadowModeEv
#[doc(alias = "RBX::CRenderSettings::getShadowMode(void)const")]
// was: RBX::CRenderSettings::getShadowMode(void)const
// IDA 0xb41c: LDR words[3] of RBX::CRenderSettings (settings +12 == item +108).
// FIDELITY: 1:1 field/global read; return type fixed from IDA signature.
pub fn stub_b41c(settings: &RenderSettings) -> i32 {
    settings.shadow_mode
}

// 0xb420 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::~EnumPropDescriptor()")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::~EnumPropDescriptor()
// IDA 0xb420: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b420() {
}

// 0xb444 — __ZNK3RBX15CRenderSettings19getAntialiasingModeEv
#[doc(alias = "RBX::CRenderSettings::getAntialiasingMode(void)const")]
// was: RBX::CRenderSettings::getAntialiasingMode(void)const
// IDA 0xb444: LDR words[2] of RBX::CRenderSettings (settings +8 == item +104).
// FIDELITY: 1:1 field/global read; return type fixed from IDA signature.
pub fn stub_b444(settings: &RenderSettings) -> i32 {
    settings.antialiasing_mode
}

// 0xb448 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::~EnumPropDescriptor()")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::~EnumPropDescriptor()
// IDA 0xb448: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b448() {
}

// 0xb46c — __ZNK3RBX15CRenderSettings25getDebugShowBoundingBoxesEv
#[doc(alias = "RBX::CRenderSettings::getDebugShowBoundingBoxes(void)const")]
// was: RBX::CRenderSettings::getDebugShowBoundingBoxes(void)const
// IDA 0xb46c: LDRB.W byte +40 of RBX::CRenderSettings (item +136).
// FIDELITY: 1:1 field/global read; return type fixed from IDA signature.
pub fn stub_b46c(settings: &RenderSettings) -> bool {
    settings.debug_show_bounding_boxes
}

// 0xb474 — __ZNK3RBX15CRenderSettings19getAutoQualityLevelEv
#[doc(alias = "RBX::CRenderSettings::getAutoQualityLevel(void)const")]
// was: RBX::CRenderSettings::getAutoQualityLevel(void)const
// IDA 0xb474: LDR words[7] of RBX::CRenderSettings (settings +28 == item +124).
// FIDELITY: 1:1 field/global read; return type fixed from IDA signature.
pub fn stub_b474(settings: &RenderSettings) -> i32 {
    settings.auto_quality_level
}

// 0xb478 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::~PropDescriptor()
// IDA 0xb478: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b478() {
}

// 0xb49c — __ZNK3RBX15CRenderSettings12getEnableFRMEv
#[doc(alias = "RBX::CRenderSettings::getEnableFRM(void)const")]
// was: RBX::CRenderSettings::getEnableFRM(void)const
// IDA 0xb49c: LDRB.W byte +41 of RBX::CRenderSettings (item +137).
// FIDELITY: 1:1 field/global read; return type fixed from IDA signature.
pub fn stub_b49c(settings: &RenderSettings) -> bool {
    settings.enable_frm
}

// 0xb4a4 — __ZNK3RBX15CRenderSettings23getResolutionPreferenceEv
#[doc(alias = "RBX::CRenderSettings::getResolutionPreference(void)const")]
// was: RBX::CRenderSettings::getResolutionPreference(void)const
// IDA 0xb4a4: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b4a4() {
}

// 0xb4a8 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::~EnumPropDescriptor()")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::~EnumPropDescriptor()
// IDA 0xb4a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b4a8() {
}

// 0xb4cc — __ZN3RBX15CRenderSettings18getMaxQualityLevelEv
#[doc(alias = "RBX::CRenderSettings::getMaxQualityLevel(void)")]
// was: RBX::CRenderSettings::getMaxQualityLevel(void)
// IDA 0xb4cc: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b4cc() {
}

// 0xb4d0 — __ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::~BoundFuncDesc()
// IDA 0xb4d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b4d0() {
}

// 0xb4f4 — __ZNK3RBX15CRenderSettings19getTextureCacheSizeEv
#[doc(alias = "RBX::CRenderSettings::getTextureCacheSize(void)const")]
// was: RBX::CRenderSettings::getTextureCacheSize(void)const
// IDA 0xb4f4: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b4f4() {
}

// 0xb4f8 — __ZNK3RBX15CRenderSettings16getMeshCacheSizeEv
#[doc(alias = "RBX::CRenderSettings::getMeshCacheSize(void)const")]
// was: RBX::CRenderSettings::getMeshCacheSize(void)const
// IDA 0xb4f8: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b4f8() {
}

// 0xb4fc — __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEEC2Ev
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEEC2Ev")]
// was: __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEEC2Ev
// IDA 0xb4fc: 151 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b4fc() {
}

// 0xb8b0 — __ZNK3RBX15CRenderSettings21getEagerBulkExecutionEv
#[doc(alias = "RBX::CRenderSettings::getEagerBulkExecution(void)const")]
// was: RBX::CRenderSettings::getEagerBulkExecution(void)const
// IDA 0xb8b0: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8b0() {
}

// 0xb8b8 — __ZN19CRenderSettingsItemD1Ev
#[doc(alias = "CRenderSettingsItem::~CRenderSettingsItem()")]
// was: CRenderSettingsItem::~CRenderSettingsItem()
// IDA 0xb8b8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_b8b8() {
}

// 0xb8bc — __ZN19CRenderSettingsItemD0Ev
#[doc(alias = "CRenderSettingsItem::~CRenderSettingsItem()")]
// was: CRenderSettingsItem::~CRenderSettingsItem()
// IDA 0xb8bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b8bc() {
}

// 0xb8d0 — __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv
// IDA 0xb8d0: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8d0() {
}

// 0xb8e0 — __ZThn32_N19CRenderSettingsItemD1Ev
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
// was: non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()
// IDA 0xb8e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b8e0() {
}

// 0xb8e8 — __ZThn32_N19CRenderSettingsItemD0Ev
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
// was: non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()
// IDA 0xb8e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b8e8() {
}

// 0xb900 — __ZThn32_NK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv
// IDA 0xb900: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b900() {
}

// 0xb910 — __ZThn36_N19CRenderSettingsItemD1Ev
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
// was: non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()
// IDA 0xb910: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b910() {
}

// 0xb918 — __ZThn36_N19CRenderSettingsItemD0Ev
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
// was: non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()
// IDA 0xb918: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b918() {
}

// 0xb930 — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD1Ev
// IDA 0xb930: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_b930() {
}

// 0xb934 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()
// IDA 0xb934: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_b934() {
}

// 0xb938 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()
// IDA 0xb938: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b938() {
}

// 0xb94c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::lookup(char const*)const
// IDA 0xb94c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b94c() {
}

// 0xb97c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::lookup(RBX::Reflection::Variant const&)const
// IDA 0xb97c: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b97c() {
}

// 0xb99c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// IDA 0xb99c: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b99c() {
}

// 0xb9f8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToString(unsigned long,std::string &)const
// IDA 0xb9f8: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9f8() {
}

// 0xbb3c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()
// IDA 0xbb3c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bb3c() {
}

// 0xbb40 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()
// IDA 0xbb40: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bb40() {
}

// 0xbb54 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::lookup(char const*)const
// IDA 0xbb54: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb54() {
}

// 0xbb84 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::lookup(RBX::Reflection::Variant const&)const
// IDA 0xbb84: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb84() {
}

// 0xbba4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// IDA 0xbba4: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bba4() {
}

// 0xbc00 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToString(unsigned long,std::string &)const
// IDA 0xbc00: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bc00() {
}

// 0xbd44 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::~EnumDesc()
// IDA 0xbd44: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bd44() {
}

// 0xbd48 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::~EnumDesc()
// IDA 0xbd48: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bd48() {
}

// 0xbd5c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::lookup(char const*)const
// IDA 0xbd5c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd5c() {
}

// 0xbd8c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::lookup(RBX::Reflection::Variant const&)const
// IDA 0xbd8c: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd8c() {
}

// 0xbdac — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// IDA 0xbdac: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdac() {
}

// 0xbe08 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToString(unsigned long,std::string &)const
// IDA 0xbe08: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be08() {
}

// 0xbf4c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc()
// IDA 0xbf4c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bf4c() {
}

// 0xbf50 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc()
// IDA 0xbf50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bf50() {
}

// 0xbf64 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::lookup(char const*)const
// IDA 0xbf64: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf64() {
}

// 0xbf94 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::lookup(RBX::Reflection::Variant const&)const
// IDA 0xbf94: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf94() {
}

// 0xbfb4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// IDA 0xbfb4: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfb4() {
}

// 0xc010 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToString(unsigned long,std::string &)const
// IDA 0xc010: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c010() {
}

// 0xc154 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::~EnumDesc()
// IDA 0xc154: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_c154() {
}

// 0xc158 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::~EnumDesc()
// IDA 0xc158: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c158() {
}

// 0xc16c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::lookup(char const*)const
// IDA 0xc16c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c16c() {
}

// 0xc19c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::lookup(RBX::Reflection::Variant const&)const
// IDA 0xc19c: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c19c() {
}

// 0xc1bc — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// IDA 0xc1bc: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c1bc() {
}

// 0xc218 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToString(unsigned long,std::string &)const
// IDA 0xc218: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c218() {
}
// 0xc35c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::~EnumDesc()
// IDA 0xc35c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_c35c() {
}

// 0xc360 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::~EnumDesc()
// IDA 0xc360: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c360() {
}

// 0xc374 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::lookup(char const*)const
// IDA 0xc374: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c374() {
}

// 0xc3a4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::lookup(RBX::Reflection::Variant const&)const
// IDA 0xc3a4: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c3a4() {
}

// 0xc3c4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// IDA 0xc3c4: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c3c4() {
}

// 0xc420 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToString(unsigned long,std::string &)const
// IDA 0xc420: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c420() {
}

// 0xc564 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::~EnumDesc()
// IDA 0xc564: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_c564() {
}

// 0xc568 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::~EnumDesc()
// IDA 0xc568: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c568() {
}

// 0xc57c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::lookup(char const*)const
// IDA 0xc57c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c57c() {
}

// 0xc5ac — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::lookup(RBX::Reflection::Variant const&)const
// IDA 0xc5ac: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c5ac() {
}

// 0xc5cc — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// IDA 0xc5cc: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c5cc() {
}

// 0xc628 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToString(unsigned long,std::string &)const
// IDA 0xc628: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c628() {
}

// 0xc76c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToString(RBX::CRenderSettings::ResolutionPreset const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToString(RBX::CRenderSettings::ResolutionPreset const&)const
// IDA 0xc76c: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c76c() {
}

// 0xc90c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings16ResolutionPresetEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::ResolutionPreset>(RBX::CRenderSettings::ResolutionPreset const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::ResolutionPreset>(RBX::CRenderSettings::ResolutionPreset const&)
// IDA 0xc90c: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c90c() {
}

// 0xc95c — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16ResolutionPresetEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ResolutionPreset>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::ResolutionPreset>::singleton(void)
// IDA 0xc95c: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c95c() {
}

// 0xc9c8 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16ResolutionPresetEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ResolutionPreset>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::ResolutionPreset>::construct_func(char const*,char *)
// IDA 0xc9c8: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9c8() {
}

// 0xc9d4 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16ResolutionPresetEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ResolutionPreset>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::ResolutionPreset>::destruct_func(char *)
// IDA 0xc9d4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c9d4() {
}

// 0xc9d8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToItem(RBX::CRenderSettings::ResolutionPreset const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToItem(RBX::CRenderSettings::ResolutionPreset const&)const
// IDA 0xc9d8: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9d8() {
}

// 0xcaa4 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings16ResolutionPresetENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::CRenderSettings::ResolutionPreset const& rbx::any_cast<RBX::CRenderSettings::ResolutionPreset const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::CRenderSettings::ResolutionPreset const& rbx::any_cast<RBX::CRenderSettings::ResolutionPreset const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0xcaa4: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_caa4() {
}

// 0xcc34 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToValue(RBX::Name const&,RBX::CRenderSettings::ResolutionPreset&)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToValue(RBX::Name const&,RBX::CRenderSettings::ResolutionPreset&)const
// IDA 0xcc34: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc34() {
}

// 0xccb0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::~EnumDesc()
// IDA 0xccb0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ccb0() {
}

// 0xcd4c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToString(RBX::CRenderSettings::QualityLevel const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToString(RBX::CRenderSettings::QualityLevel const&)const
// IDA 0xcd4c: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd4c() {
}

// 0xceec — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings12QualityLevelEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::QualityLevel>(RBX::CRenderSettings::QualityLevel const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::QualityLevel>(RBX::CRenderSettings::QualityLevel const&)
// IDA 0xceec: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ceec() {
}

// 0xcf3c — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12QualityLevelEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::QualityLevel>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::QualityLevel>::singleton(void)
// IDA 0xcf3c: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cf3c() {
}

// 0xcfa8 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12QualityLevelEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::QualityLevel>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::QualityLevel>::construct_func(char const*,char *)
// IDA 0xcfa8: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cfa8() {
}

// 0xcfb4 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12QualityLevelEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::QualityLevel>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::QualityLevel>::destruct_func(char *)
// IDA 0xcfb4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cfb4() {
}

// 0xcfb8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToItem(RBX::CRenderSettings::QualityLevel const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToItem(RBX::CRenderSettings::QualityLevel const&)const
// IDA 0xcfb8: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cfb8() {
}

// 0xd084 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings12QualityLevelENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::CRenderSettings::QualityLevel const& rbx::any_cast<RBX::CRenderSettings::QualityLevel const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::CRenderSettings::QualityLevel const& rbx::any_cast<RBX::CRenderSettings::QualityLevel const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0xd084: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d084() {
}

// 0xd174 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToValue(RBX::Name const&,RBX::CRenderSettings::QualityLevel&)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToValue(RBX::Name const&,RBX::CRenderSettings::QualityLevel&)const
// IDA 0xd174: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d174() {
}

// 0xd1f0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::~EnumDesc()
// IDA 0xd1f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d1f0() {
}

// 0xd28c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToString(RBX::CRenderSettings::ShadowMode const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToString(RBX::CRenderSettings::ShadowMode const&)const
// IDA 0xd28c: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d28c() {
}

// 0xd42c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings10ShadowModeEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::ShadowMode>(RBX::CRenderSettings::ShadowMode const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::ShadowMode>(RBX::CRenderSettings::ShadowMode const&)
// IDA 0xd42c: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d42c() {
}

// 0xd47c — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings10ShadowModeEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ShadowMode>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::ShadowMode>::singleton(void)
// IDA 0xd47c: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d47c() {
}

// 0xd4e8 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings10ShadowModeEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ShadowMode>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::ShadowMode>::construct_func(char const*,char *)
// IDA 0xd4e8: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d4e8() {
}

// 0xd4f4 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings10ShadowModeEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ShadowMode>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::ShadowMode>::destruct_func(char *)
// IDA 0xd4f4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d4f4() {
}

// 0xd4f8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToItem(RBX::CRenderSettings::ShadowMode const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToItem(RBX::CRenderSettings::ShadowMode const&)const
// IDA 0xd4f8: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d4f8() {
}

// 0xd5c4 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings10ShadowModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::CRenderSettings::ShadowMode const& rbx::any_cast<RBX::CRenderSettings::ShadowMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::CRenderSettings::ShadowMode const& rbx::any_cast<RBX::CRenderSettings::ShadowMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0xd5c4: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d5c4() {
}

// 0xd6b4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::ShadowMode&)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::ShadowMode&)const
// IDA 0xd6b4: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d6b4() {
}

// 0xd730 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::~EnumDesc()
// IDA 0xd730: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d730() {
}

// 0xd7cc — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToString(RBX::CRenderSettings::AntialiasingMode const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToString(RBX::CRenderSettings::AntialiasingMode const&)const
// IDA 0xd7cc: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d7cc() {
}

// 0xd96c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings16AntialiasingModeEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::AntialiasingMode>(RBX::CRenderSettings::AntialiasingMode const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::AntialiasingMode>(RBX::CRenderSettings::AntialiasingMode const&)
// IDA 0xd96c: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d96c() {
}

// 0xd9bc — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16AntialiasingModeEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AntialiasingMode>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::AntialiasingMode>::singleton(void)
// IDA 0xd9bc: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d9bc() {
}

// 0xda28 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16AntialiasingModeEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AntialiasingMode>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::AntialiasingMode>::construct_func(char const*,char *)
// IDA 0xda28: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_da28() {
}

// 0xda34 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16AntialiasingModeEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AntialiasingMode>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::AntialiasingMode>::destruct_func(char *)
// IDA 0xda34: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_da34() {
}

// 0xda38 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToItem(RBX::CRenderSettings::AntialiasingMode const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToItem(RBX::CRenderSettings::AntialiasingMode const&)const
// IDA 0xda38: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_da38() {
}

// 0xdb04 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings16AntialiasingModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::CRenderSettings::AntialiasingMode const& rbx::any_cast<RBX::CRenderSettings::AntialiasingMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::CRenderSettings::AntialiasingMode const& rbx::any_cast<RBX::CRenderSettings::AntialiasingMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0xdb04: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db04() {
}

// 0xdbf4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::AntialiasingMode&)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::AntialiasingMode&)const
// IDA 0xdbf4: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dbf4() {
}

// 0xdc70 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc()
// IDA 0xdc70: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_dc70() {
}

// 0xdd0c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToString(RBX::CRenderSettings::FrameRateManagerMode const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToString(RBX::CRenderSettings::FrameRateManagerMode const&)const
// IDA 0xdd0c: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dd0c() {
}

// 0xdeac — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings20FrameRateManagerModeEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::FrameRateManagerMode>(RBX::CRenderSettings::FrameRateManagerMode const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::FrameRateManagerMode>(RBX::CRenderSettings::FrameRateManagerMode const&)
// IDA 0xdeac: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_deac() {
}

// 0xdefc — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings20FrameRateManagerModeEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::FrameRateManagerMode>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::FrameRateManagerMode>::singleton(void)
// IDA 0xdefc: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_defc() {
}

// 0xdf68 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings20FrameRateManagerModeEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::FrameRateManagerMode>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::FrameRateManagerMode>::construct_func(char const*,char *)
// IDA 0xdf68: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_df68() {
}

// 0xdf74 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings20FrameRateManagerModeEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::FrameRateManagerMode>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::FrameRateManagerMode>::destruct_func(char *)
// IDA 0xdf74: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_df74() {
}

// 0xdf78 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToItem(RBX::CRenderSettings::FrameRateManagerMode const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToItem(RBX::CRenderSettings::FrameRateManagerMode const&)const
// IDA 0xdf78: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_df78() {
}

// 0xe044 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings20FrameRateManagerModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::CRenderSettings::FrameRateManagerMode const& rbx::any_cast<RBX::CRenderSettings::FrameRateManagerMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::CRenderSettings::FrameRateManagerMode const& rbx::any_cast<RBX::CRenderSettings::FrameRateManagerMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0xe044: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e044() {
}

// 0xe134 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::FrameRateManagerMode&)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::FrameRateManagerMode&)const
// IDA 0xe134: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e134() {
}

// 0xe1b0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::~EnumDesc()
// IDA 0xe1b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1b0() {
}

// 0xe24c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToString(RBX::CRenderSettings::GraphicsMode const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToString(RBX::CRenderSettings::GraphicsMode const&)const
// IDA 0xe24c: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e24c() {
}

// 0xe3ec — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings12GraphicsModeEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::GraphicsMode>(RBX::CRenderSettings::GraphicsMode const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::GraphicsMode>(RBX::CRenderSettings::GraphicsMode const&)
// IDA 0xe3ec: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e3ec() {
}

// 0xe43c — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12GraphicsModeEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::GraphicsMode>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::GraphicsMode>::singleton(void)
// IDA 0xe43c: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e43c() {
}

// 0xe4a8 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12GraphicsModeEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::GraphicsMode>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::GraphicsMode>::construct_func(char const*,char *)
// IDA 0xe4a8: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4a8() {
}

// 0xe4b4 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12GraphicsModeEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::GraphicsMode>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::GraphicsMode>::destruct_func(char *)
// IDA 0xe4b4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e4b4() {
}

// 0xe4b8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToItem(RBX::CRenderSettings::GraphicsMode const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToItem(RBX::CRenderSettings::GraphicsMode const&)const
// IDA 0xe4b8: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4b8() {
}

// 0xe584 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings12GraphicsModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::CRenderSettings::GraphicsMode const& rbx::any_cast<RBX::CRenderSettings::GraphicsMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::CRenderSettings::GraphicsMode const& rbx::any_cast<RBX::CRenderSettings::GraphicsMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0xe584: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e584() {
}

// 0xe674 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::GraphicsMode&)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::GraphicsMode&)const
// IDA 0xe674: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e674() {
}

// 0xe6f0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()
// IDA 0xe6f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e6f0() {
}

// 0xe78c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToString(RBX::CRenderSettings::AASamples const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToString(RBX::CRenderSettings::AASamples const&)const
// IDA 0xe78c: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e78c() {
}

// 0xe92c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings9AASamplesEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::AASamples>(RBX::CRenderSettings::AASamples const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::AASamples>(RBX::CRenderSettings::AASamples const&)
// IDA 0xe92c: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e92c() {
}

// 0xe97c — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings9AASamplesEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::singleton(void)
// IDA 0xe97c: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e97c() {
}

// 0xe9e8 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings9AASamplesEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::construct_func(char const*,char *)
// IDA 0xe9e8: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e9e8() {
}

// 0xe9f4 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings9AASamplesEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::destruct_func(char *)
// IDA 0xe9f4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e9f4() {
}

// 0xe9f8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToItem(RBX::CRenderSettings::AASamples const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToItem(RBX::CRenderSettings::AASamples const&)const
// IDA 0xe9f8: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e9f8() {
}

// 0xeac4 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings9AASamplesENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::CRenderSettings::AASamples const& rbx::any_cast<RBX::CRenderSettings::AASamples const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::CRenderSettings::AASamples const& rbx::any_cast<RBX::CRenderSettings::AASamples const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0xeac4: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_eac4() {
}

// 0xebb4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToValue(RBX::Name const&,RBX::CRenderSettings::AASamples&)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToValue(RBX::Name const&,RBX::CRenderSettings::AASamples&)const
// IDA 0xebb4: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ebb4() {
}

// 0xec30 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()
// IDA 0xec30: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ec30() {
}

// 0xeccc — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD2Ev
// IDA 0xeccc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_eccc() {
}

// 0xedfc — __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator12getClassNameEv
// IDA 0xedfc: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_edfc() {
}

// 0xee84 — __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator6createEv
// IDA 0xee84: 45 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ee84() {
}

// 0xef04 — __ZN3RBX9CreatableINS_8InstanceEE6createI19CRenderSettingsItemEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<CRenderSettingsItem> RBX::Creatable<RBX::Instance>::create<CRenderSettingsItem>(void)")]
// was: boost::shared_ptr<CRenderSettingsItem> RBX::Creatable<RBX::Instance>::create<CRenderSettingsItem>(void)
// IDA 0xef04: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ef04() {
}

// 0xefb4 — __ZN5boost10shared_ptrI19CRenderSettingsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<CRenderSettingsItem>::shared_ptr<CRenderSettingsItem,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<CRenderSettingsItem>::shared_ptr<CRenderSettingsItem,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)
// IDA 0xefb4: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_efb4() {
}

// 0xf098 — __ZN5boost6detail12shared_countC2IP19CRenderSettingsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::detail::shared_count::shared_count<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)
// IDA 0xf098: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f098() {
}

// 0xf198 — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// IDA 0xf198: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_f198() {
}

// 0xf19c — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// IDA 0xf19c: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f19c() {
}

// 0xf1bc — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// IDA 0xf1bc: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f1bc() {
}

// 0xf1d4 — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// IDA 0xf1d4: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f1d4() {
}

// 0xf1d8 — __ZN3RBX4Name13callDoDeclareILZ15sRenderSettingsEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZ15sRenderSettingsEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZ15sRenderSettingsEEEvv
// IDA 0xf1d8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_f1d8() {
}

// 0xf1dc — __ZN3RBX4Name9doDeclareILZ15sRenderSettingsEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZ15sRenderSettingsEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZ15sRenderSettingsEEERKS0_v
// IDA 0xf1dc: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f1dc() {
}

// 0xf2bc — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorC2Ev
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorC2Ev
// IDA 0xf2bc: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f2bc() {
}

// 0xf500 — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE17static_getCreatorEv
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE17static_getCreatorEv
// IDA 0xf500: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f500() {
}

// 0xf83c — __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev")]
// was: __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev
// IDA 0xf83c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f83c() {
}

// 0xf87c — __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev")]
// was: __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev
// IDA 0xf87c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f87c() {
}

// 0xf8c8 — __ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev
#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev")]
// was: __ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev
// IDA 0xf8c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f8c8() {
}

// 0xf90c — __ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev
#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev")]
// was: __ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev
// IDA 0xf90c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f90c() {
}

// 0xf964 — __ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev
#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev")]
// was: __ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev
// IDA 0xf964: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f964() {
}

// 0xf9a8 — __ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev
#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev")]
// was: __ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev
// IDA 0xf9a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f9a8() {
}

// 0xfa00 — __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
#[doc(alias = "__ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// IDA 0xfa00: 91 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_fa00() {
}

// 0xfb1c — __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0xfb1c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_fb1c() {
}

// 0xfb20 — __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0xfb20: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_fb20() {
}

// 0xfb34 — __ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0xfb34: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_fb34() {
}

// 0xfb3c — __ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0xfb3c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_fb3c() {
}
// 0xfb54 — __ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0xfb54: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_fb54() {
}

// 0xfb5c — __ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0xfb5c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_fb5c() {
}

// 0xfb74 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiEC2IMNS_15CRenderSettingsEKFjvEMS2_FvjEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::PropDescriptor<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>(char const*,char const*,unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::PropDescriptor<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>(char const*,char const*,unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0xfb74: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_fb74() {
}

// 0xfc88 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::~PropDescriptor()
// IDA 0xfc88: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_fc88() {
}

// 0xfcb4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::isReadOnly(void)const
// IDA 0xfcb4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_fcb4() {
}

// 0xfcb8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::isWriteOnly(void)const
// IDA 0xfcb8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_fcb8() {
}

// 0xfcbc — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0xfcbc: 16 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_fcbc() {
}

// 0xfce8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE8setValueEPNS0_13DescribedBaseERKi
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const
// IDA 0xfce8: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_fce8() {
}

// 0xfd0c — __ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EEC2EMS2_FivEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::BoundFuncDesc(int (CRenderSettingsItem::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::BoundFuncDesc(int (CRenderSettingsItem::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// IDA 0xfd0c: 84 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_fd0c() {
}

// 0xfe04 — __ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::~BoundFuncDesc()
// IDA 0xfe04: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_fe04() {
}

// 0xfe30 — __ZNK3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
// IDA 0xfe30: 13 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_fe30() {
}

// 0xfe54 — __ZN3RBX10Reflection11Call0HelperI19CRenderSettingsItemMS2_FivEiE4callEPS2_S4_RNS0_7VariantE
#[doc(alias = "RBX::Reflection::Call0Helper<CRenderSettingsItem,int (CRenderSettingsItem::*)(void),int>::call(CRenderSettingsItem*,int (CRenderSettingsItem::*)(void),RBX::Reflection::Variant &)")]
// was: RBX::Reflection::Call0Helper<CRenderSettingsItem,int (CRenderSettingsItem::*)(void),int>::call(CRenderSettingsItem*,int (CRenderSettingsItem::*)(void),RBX::Reflection::Variant &)
// IDA 0xfe54: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_fe54() {
}

// 0xfe84 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::EnumPropDescriptor<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>(char const*,char const*,RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::EnumPropDescriptor<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>(char const*,char const*,RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0xfe84: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_fe84() {
}

// 0x10038 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::~EnumPropDescriptor()")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::~EnumPropDescriptor()
// IDA 0x10038: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_10038() {
}

// 0x10064 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::isReadOnly(void)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::isReadOnly(void)const
// IDA 0x10064: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_10064() {
}

// 0x10074 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::isWriteOnly(void)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::isWriteOnly(void)const
// IDA 0x10074: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_10074() {
}

// 0x10084 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
// IDA 0x10084: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_10084() {
}

// 0x100ac — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// IDA 0x100ac: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_100ac() {
}

// 0x100d0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
// IDA 0x100d0: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_100d0() {
}

// 0x10220 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
// IDA 0x10220: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_10220() {
}

// 0x10244 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::hasStringValue(void)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::hasStringValue(void)const
// IDA 0x10244: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_10244() {
}

// 0x10248 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getStringValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x10248: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_10248() {
}

// 0x1026c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
// IDA 0x1026c: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1026c() {
}

// 0x102ac — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
// IDA 0x102ac: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_102ac() {
}

// 0x102cc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// IDA 0x102cc: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_102cc() {
}

// 0x1050c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getIndexValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x1050c: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1050c() {
}

// 0x10528 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
// IDA 0x10528: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_10528() {
}

// 0x1055c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getEnumValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x1055c: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1055c() {
}

// 0x10564 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
// IDA 0x10564: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_10564() {
}

// 0x105b0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getEnumItem(RBX::Reflection::DescribedBase const*)const
// IDA 0x105b0: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_105b0() {
}

// 0x105d0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
// IDA 0x105d0: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_105d0() {
}

// 0x10604 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToIndex(RBX::CRenderSettings::ResolutionPreset)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToIndex(RBX::CRenderSettings::ResolutionPreset)const
// IDA 0x10604: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_10604() {
}

// 0x10674 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setIntValue(RBX::Reflection::DescribedBase *,int)const
// IDA 0x10674: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_10674() {
}

// 0x106b4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::isReadOnly(void)const
// IDA 0x106b4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_106b4() {
}

// 0x106b8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::isWriteOnly(void)const
// IDA 0x106b8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_106b8() {
}

// 0x106bc — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x106bc: 16 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_106bc() {
}

// 0x106e8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::ResolutionPreset const&)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::ResolutionPreset const&)const
// IDA 0x106e8: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_106e8() {
}

// 0x1070c — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::PropDescriptor<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>(char const*,char const*,bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::PropDescriptor<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>(char const*,char const*,bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x1070c: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1070c() {
}

// 0x10820 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::~PropDescriptor()
// IDA 0x10820: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_10820() {
}

// 0x1084c — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isReadOnly(void)const
// IDA 0x1084c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1084c() {
}

// 0x10850 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isWriteOnly(void)const
// IDA 0x10850: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_10850() {
}

// 0x10854 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x10854: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_10854() {
}

// 0x10878 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const
// IDA 0x10878: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_10878() {
}

// 0x1089c — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiEC2IMNS_15CRenderSettingsEKFivEMS2_FviEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::PropDescriptor<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>(char const*,char const*,int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::PropDescriptor<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>(char const*,char const*,int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x1089c: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1089c() {
}

// 0x109b0 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>::isReadOnly(void)const
// IDA 0x109b0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_109b0() {
}

// 0x109b4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>::isWriteOnly(void)const
// IDA 0x109b4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_109b4() {
}

// 0x109b8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x109b8: 16 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_109b8() {
}

// 0x109e4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const
// IDA 0x109e4: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_109e4() {
}

// 0x10a08 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::EnumPropDescriptor<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>(char const*,char const*,RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::EnumPropDescriptor<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>(char const*,char const*,RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x10a08: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_10a08() {
}

// 0x10bbc — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::~EnumPropDescriptor()")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::~EnumPropDescriptor()
// IDA 0x10bbc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_10bbc() {
}

// 0x10be8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::isReadOnly(void)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::isReadOnly(void)const
// IDA 0x10be8: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_10be8() {
}

// 0x10bf8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::isWriteOnly(void)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::isWriteOnly(void)const
// IDA 0x10bf8: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_10bf8() {
}

// 0x10c08 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
// IDA 0x10c08: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_10c08() {
}

// 0x10c30 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// IDA 0x10c30: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_10c30() {
}

// 0x10c54 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
// IDA 0x10c54: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_10c54() {
}

// 0x10da4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
// IDA 0x10da4: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_10da4() {
}

// 0x10dc8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::hasStringValue(void)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::hasStringValue(void)const
// IDA 0x10dc8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_10dc8() {
}

// 0x10dcc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getStringValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x10dcc: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_10dcc() {
}

// 0x10df0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
// IDA 0x10df0: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_10df0() {
}

// 0x10e30 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
// IDA 0x10e30: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_10e30() {
}

// 0x10e50 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// IDA 0x10e50: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_10e50() {
}

// 0x11090 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x11090: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11090() {
}

// 0x110ac — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
// IDA 0x110ac: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_110ac() {
}

// 0x110e0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x110e0: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_110e0() {
}

// 0x110e8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
// IDA 0x110e8: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_110e8() {
}

// 0x11134 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const
// IDA 0x11134: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11134() {
}

// 0x11154 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
// IDA 0x11154: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11154() {
}

// 0x11188 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToIndex(RBX::CRenderSettings::AntialiasingMode)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToIndex(RBX::CRenderSettings::AntialiasingMode)const
// IDA 0x11188: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11188() {
}

// 0x111f8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const
// IDA 0x111f8: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_111f8() {
}

// 0x11238 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::GetSetImpl<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::GetSetImpl<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>::isReadOnly(void)const
// IDA 0x11238: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11238() {
}

// 0x1123c — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::GetSetImpl<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::GetSetImpl<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>::isWriteOnly(void)const
// IDA 0x1123c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1123c() {
}

// 0x11240 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::GetSetImpl<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::GetSetImpl<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x11240: 16 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11240() {
}

// 0x1126c — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::GetSetImpl<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::AntialiasingMode const&)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::GetSetImpl<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::AntialiasingMode const&)const
// IDA 0x1126c: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1126c() {
}

// 0x11290 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::EnumPropDescriptor<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>(char const*,char const*,RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::EnumPropDescriptor<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>(char const*,char const*,RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x11290: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11290() {
}

// 0x11444 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::~EnumPropDescriptor()")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::~EnumPropDescriptor()
// IDA 0x11444: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_11444() {
}

// 0x11470 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::isReadOnly(void)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::isReadOnly(void)const
// IDA 0x11470: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11470() {
}

// 0x11480 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::isWriteOnly(void)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::isWriteOnly(void)const
// IDA 0x11480: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11480() {
}

// 0x11490 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
// IDA 0x11490: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11490() {
}

// 0x114b8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// IDA 0x114b8: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_114b8() {
}

// 0x114dc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
// IDA 0x114dc: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_114dc() {
}

// 0x1162c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
// IDA 0x1162c: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1162c() {
}

// 0x11650 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::hasStringValue(void)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::hasStringValue(void)const
// IDA 0x11650: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11650() {
}

// 0x11654 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getStringValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x11654: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11654() {
}

// 0x11678 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
// IDA 0x11678: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11678() {
}

// 0x116b8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
// IDA 0x116b8: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_116b8() {
}

// 0x116d8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// IDA 0x116d8: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_116d8() {
}

// 0x11918 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x11918: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11918() {
}

// 0x11934 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
// IDA 0x11934: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11934() {
}

// 0x11968 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x11968: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11968() {
}

// 0x11970 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
// IDA 0x11970: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11970() {
}

// 0x119bc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const
// IDA 0x119bc: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_119bc() {
}

// 0x119dc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
// IDA 0x119dc: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_119dc() {
}

// 0x11a10 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToIndex(RBX::CRenderSettings::ShadowMode)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToIndex(RBX::CRenderSettings::ShadowMode)const
// IDA 0x11a10: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11a10() {
}

// 0x11a80 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const
// IDA 0x11a80: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11a80() {
}

// 0x11ac0 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::isReadOnly(void)const
// IDA 0x11ac0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11ac0() {
}

// 0x11ac4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::isWriteOnly(void)const
// IDA 0x11ac4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11ac4() {
}

// 0x11ac8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x11ac8: 16 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11ac8() {
}

// 0x11af4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::ShadowMode const&)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::GetSetImpl<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::ShadowMode const&)const
// IDA 0x11af4: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11af4() {
}

// 0x11b18 — __ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2I19CRenderSettingsItemEEPKcS7_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<CRenderSettingsItem>(char const*,char const*,std::string  CRenderSettingsItem::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<CRenderSettingsItem>(char const*,char const*,std::string  CRenderSettingsItem::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x11b18: 153 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11b18() {
}

// 0x11ca8 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isReadOnly(void)const")]
// was: RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isReadOnly(void)const
// IDA 0x11ca8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11ca8() {
}// 0x11cac — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isWriteOnly(void)const")]
// was: RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isWriteOnly(void)const
// IDA 0x11cac: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11cac() {
}

// 0x11cb0 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x11cb0: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11cb0() {
}

// 0x11cc8 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8setValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const
// IDA 0x11cc8: 38 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11cc8() {
}

// 0x11d30 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::EnumPropDescriptor<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>(char const*,char const*,RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::EnumPropDescriptor<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>(char const*,char const*,RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x11d30: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11d30() {
}

// 0x11ee4 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::~EnumPropDescriptor()")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::~EnumPropDescriptor()
// IDA 0x11ee4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_11ee4() {
}

// 0x11f10 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::isReadOnly(void)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::isReadOnly(void)const
// IDA 0x11f10: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11f10() {
}

// 0x11f20 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::isWriteOnly(void)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::isWriteOnly(void)const
// IDA 0x11f20: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11f20() {
}

// 0x11f30 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
// IDA 0x11f30: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11f30() {
}

// 0x11f58 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// IDA 0x11f58: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11f58() {
}

// 0x11f7c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
// IDA 0x11f7c: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_11f7c() {
}

// 0x120cc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
// IDA 0x120cc: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_120cc() {
}

// 0x120f0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::hasStringValue(void)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::hasStringValue(void)const
// IDA 0x120f0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_120f0() {
}

// 0x120f4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getStringValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x120f4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_120f4() {
}

// 0x12118 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
// IDA 0x12118: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_12118() {
}

// 0x12158 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
// IDA 0x12158: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_12158() {
}

// 0x12178 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// IDA 0x12178: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_12178() {
}

// 0x123b8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getIndexValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x123b8: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_123b8() {
}

// 0x123d4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
// IDA 0x123d4: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_123d4() {
}

// 0x12408 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getEnumValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x12408: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_12408() {
}

// 0x12410 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
// IDA 0x12410: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_12410() {
}

// 0x1245c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::getEnumItem(RBX::Reflection::DescribedBase const*)const
// IDA 0x1245c: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1245c() {
}

// 0x1247c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
// IDA 0x1247c: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1247c() {
}

// 0x124b0 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToIndex(RBX::CRenderSettings::AASamples)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToIndex(RBX::CRenderSettings::AASamples)const
// IDA 0x124b0: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_124b0() {
}

// 0x12520 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setIntValue(RBX::Reflection::DescribedBase *,int)const
// IDA 0x12520: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_12520() {
}

// 0x12560 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::isReadOnly(void)const
// IDA 0x12560: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_12560() {
}

// 0x12564 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::isWriteOnly(void)const
// IDA 0x12564: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_12564() {
}

// 0x12568 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x12568: 16 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_12568() {
}

// 0x12594 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::AASamples const&)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::GetSetImpl<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::AASamples const&)const
// IDA 0x12594: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_12594() {
}

// 0x125b8 — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2I19CRenderSettingsItemEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<CRenderSettingsItem>(char const*,char const*,bool CRenderSettingsItem::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<CRenderSettingsItem>(char const*,char const*,bool CRenderSettingsItem::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x125b8: 153 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_125b8() {
}

// 0x12748 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isReadOnly(void)const")]
// was: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isReadOnly(void)const
// IDA 0x12748: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_12748() {
}

// 0x1274c — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isWriteOnly(void)const")]
// was: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::isWriteOnly(void)const
// IDA 0x1274c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1274c() {
}

// 0x12750 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x12750: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_12750() {
}

// 0x1275c — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetI19CRenderSettingsItemE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<CRenderSettingsItem>::setValue(RBX::Reflection::DescribedBase *,bool const&)const
// IDA 0x1275c: 31 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1275c() {
}

// 0x127ac — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembEC2IMNS_15CRenderSettingsEKFbvEMS2_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::PropDescriptor<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>(char const*,char const*,bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::PropDescriptor<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>(char const*,char const*,bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x127ac: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_127ac() {
}

// 0x128c0 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isReadOnly(void)const
// IDA 0x128c0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_128c0() {
}

// 0x128c4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isWriteOnly(void)const
// IDA 0x128c4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_128c4() {
}

// 0x128c8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x128c8: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_128c8() {
}

// 0x128fc — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMNS_15CRenderSettingsEKFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const
// IDA 0x128fc: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_128fc() {
}

// 0x12920 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::EnumPropDescriptor<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>(char const*,char const*,RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::EnumPropDescriptor<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>(char const*,char const*,RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x12920: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_12920() {
}

// 0x12ad4 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::~EnumPropDescriptor()")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::~EnumPropDescriptor()
// IDA 0x12ad4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_12ad4() {
}

// 0x12b00 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::isReadOnly(void)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::isReadOnly(void)const
// IDA 0x12b00: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_12b00() {
}

// 0x12b10 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::isWriteOnly(void)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::isWriteOnly(void)const
// IDA 0x12b10: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_12b10() {
}

// 0x12b20 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
// IDA 0x12b20: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_12b20() {
}

// 0x12b48 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// IDA 0x12b48: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_12b48() {
}

// 0x12b6c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
// IDA 0x12b6c: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_12b6c() {
}

// 0x12cbc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
// IDA 0x12cbc: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_12cbc() {
}

// 0x12ce0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::hasStringValue(void)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::hasStringValue(void)const
// IDA 0x12ce0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_12ce0() {
}

// 0x12ce4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getStringValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x12ce4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_12ce4() {
}

// 0x12d08 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
// IDA 0x12d08: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_12d08() {
}

// 0x12d48 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
// IDA 0x12d48: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_12d48() {
}

// 0x12d68 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// IDA 0x12d68: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_12d68() {
}

// 0x12fa8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getIndexValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x12fa8: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_12fa8() {
}

// 0x12fc4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
// IDA 0x12fc4: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_12fc4() {
}

// 0x12ff8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getEnumValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x12ff8: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_12ff8() {
}

// 0x13000 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
// IDA 0x13000: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_13000() {
}

// 0x1304c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::getEnumItem(RBX::Reflection::DescribedBase const*)const
// IDA 0x1304c: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1304c() {
}

// 0x1306c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
// IDA 0x1306c: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1306c() {
}

// 0x130a0 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToIndex(RBX::CRenderSettings::QualityLevel)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToIndex(RBX::CRenderSettings::QualityLevel)const
// IDA 0x130a0: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_130a0() {
}

// 0x13110 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setIntValue(RBX::Reflection::DescribedBase *,int)const
// IDA 0x13110: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_13110() {
}

// 0x13150 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::GetSetImpl<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::GetSetImpl<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>::isReadOnly(void)const
// IDA 0x13150: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_13150() {
}

// 0x13154 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::GetSetImpl<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::GetSetImpl<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>::isWriteOnly(void)const
// IDA 0x13154: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_13154() {
}

// 0x13158 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::GetSetImpl<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::GetSetImpl<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x13158: 16 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_13158() {
}

// 0x13184 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::GetSetImpl<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::QualityLevel const&)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::GetSetImpl<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::QualityLevel const&)const
// IDA 0x13184: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_13184() {
}

// 0x131a8 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::EnumPropDescriptor<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>(char const*,char const*,RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::EnumPropDescriptor<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>(char const*,char const*,RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x131a8: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_131a8() {
}

// 0x1335c — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::~EnumPropDescriptor()")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::~EnumPropDescriptor()
// IDA 0x1335c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_1335c() {
}

// 0x13388 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::isReadOnly(void)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::isReadOnly(void)const
// IDA 0x13388: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_13388() {
}

// 0x13398 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::isWriteOnly(void)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::isWriteOnly(void)const
// IDA 0x13398: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_13398() {
}

// 0x133a8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
// IDA 0x133a8: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_133a8() {
}

// 0x133d0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// IDA 0x133d0: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_133d0() {
}

// 0x133f4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
// IDA 0x133f4: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_133f4() {
}

// 0x13544 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
// IDA 0x13544: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_13544() {
}

// 0x13568 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::hasStringValue(void)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::hasStringValue(void)const
// IDA 0x13568: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_13568() {
}

// 0x1356c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getStringValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x1356c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1356c() {
}

// 0x13590 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
// IDA 0x13590: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_13590() {
}

// 0x135d0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
// IDA 0x135d0: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_135d0() {
}

// 0x135f0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// IDA 0x135f0: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_135f0() {
}

// 0x13830 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x13830: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_13830() {
}

// 0x1384c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
// IDA 0x1384c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1384c() {
}

// 0x13880 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x13880: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_13880() {
}

// 0x13888 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
// IDA 0x13888: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_13888() {
}

// 0x138d4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const
// IDA 0x138d4: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_138d4() {
}

// 0x138f4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
// IDA 0x138f4: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_138f4() {
}

// 0x13928 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToIndex(RBX::CRenderSettings::FrameRateManagerMode)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToIndex(RBX::CRenderSettings::FrameRateManagerMode)const
// IDA 0x13928: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_13928() {
}

// 0x13998 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const
// IDA 0x13998: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_13998() {
}

// 0x139d8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::GetSetImpl<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::GetSetImpl<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>::isReadOnly(void)const
// IDA 0x139d8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_139d8() {
}

// 0x139dc — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::GetSetImpl<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::GetSetImpl<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>::isWriteOnly(void)const
// IDA 0x139dc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_139dc() {
}

// 0x139e0 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::GetSetImpl<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::GetSetImpl<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x139e0: 16 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_139e0() {
}

// 0x13a0c — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::GetSetImpl<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::FrameRateManagerMode const&)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::GetSetImpl<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::FrameRateManagerMode const&)const
// IDA 0x13a0c: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_13a0c() {
}

// 0x13a30 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::EnumPropDescriptor<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>(char const*,char const*,RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::EnumPropDescriptor<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>(char const*,char const*,RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x13a30: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_13a30() {
}

// 0x13be4 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::~EnumPropDescriptor()")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::~EnumPropDescriptor()
// IDA 0x13be4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_13be4() {
}

// 0x13c10 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::isReadOnly(void)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::isReadOnly(void)const
// IDA 0x13c10: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_13c10() {
}

// 0x13c20 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::isWriteOnly(void)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::isWriteOnly(void)const
// IDA 0x13c20: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_13c20() {
}

// 0x13c30 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
// IDA 0x13c30: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_13c30() {
}

// 0x13c58 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// IDA 0x13c58: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_13c58() {
}

// 0x13c7c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
// IDA 0x13c7c: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_13c7c() {
}

// 0x13dcc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
// IDA 0x13dcc: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_13dcc() {
}

// 0x13df0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::hasStringValue(void)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::hasStringValue(void)const
// IDA 0x13df0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_13df0() {
}

// 0x13df4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getStringValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x13df4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_13df4() {
}

// 0x13e18 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
// IDA 0x13e18: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_13e18() {
}

// 0x13e58 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
// IDA 0x13e58: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_13e58() {
}
// 0x13e78 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// IDA 0x13e78: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_13e78() {
}

// 0x140b8 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x140b8: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_140b8() {
}

// 0x140d4 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
// IDA 0x140d4: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_140d4() {
}

// 0x14108 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x14108: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_14108() {
}

// 0x14110 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
// IDA 0x14110: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_14110() {
}

// 0x1415c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const
// IDA 0x1415c: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1415c() {
}

// 0x1417c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
// IDA 0x1417c: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1417c() {
}

// 0x141b0 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToIndex(RBX::CRenderSettings::GraphicsMode)const")]
// was: RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToIndex(RBX::CRenderSettings::GraphicsMode)const
// IDA 0x141b0: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_141b0() {
}

// 0x14220 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const
// IDA 0x14220: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_14220() {
}

// 0x14260 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::isReadOnly(void)const
// IDA 0x14260: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_14260() {
}

// 0x14264 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::isWriteOnly(void)const
// IDA 0x14264: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_14264() {
}

// 0x14268 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x14268: 16 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_14268() {
}

// 0x14294 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::GraphicsMode const&)const")]
// was: RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::GraphicsMode const&)const
// IDA 0x14294: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_14294() {
}

// 0x142b8 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings16ResolutionPresetESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::ResolutionPreset,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::operator[](RBX::Name const* const&)")]
// was: std::map<RBX::Name const*,RBX::CRenderSettings::ResolutionPreset,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::operator[](RBX::Name const* const&)
// IDA 0x142b8: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_142b8() {
}

// 0x14310 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)
// IDA 0x14310: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_14310() {
}

// 0x143c4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)
// IDA 0x143c4: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_143c4() {
}

// 0x1441c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)
// IDA 0x1441c: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1441c() {
}

// 0x14484 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::resize(unsigned long,RBX::CRenderSettings::ResolutionPreset)")]
// was: std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::resize(unsigned long,RBX::CRenderSettings::ResolutionPreset)
// IDA 0x14484: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_14484() {
}

// 0x144b8 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::push_back(RBX::CRenderSettings::ResolutionPreset const&)")]
// was: std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::push_back(RBX::CRenderSettings::ResolutionPreset const&)
// IDA 0x144b8: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_144b8() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x144e0 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ResolutionPreset*,std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>>,RBX::CRenderSettings::ResolutionPreset const&)")]
// was: std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ResolutionPreset*,std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>>,RBX::CRenderSettings::ResolutionPreset const&)
// IDA 0x144e0: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_144e0() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x145c4 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_allocate(unsigned long)")]
// was: std::_Vector_base<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_allocate(unsigned long)
// IDA 0x145c4: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_145c4() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x145dc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings16ResolutionPresetES6_EET0_T_S8_S7_
#[doc(alias = "RBX::CRenderSettings::ResolutionPreset * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *>(RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *)")]
// was: RBX::CRenderSettings::ResolutionPreset * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *>(RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *)
// IDA 0x145dc: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_145dc() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x14618 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ResolutionPreset*,std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>>,unsigned long,RBX::CRenderSettings::ResolutionPreset const&)")]
// was: std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ResolutionPreset*,std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>>,unsigned long,RBX::CRenderSettings::ResolutionPreset const&)
// IDA 0x14618: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_14618() {
}

// 0x147a8 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings12QualityLevelESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::QualityLevel,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::operator[](RBX::Name const* const&)")]
// was: std::map<RBX::Name const*,RBX::CRenderSettings::QualityLevel,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::operator[](RBX::Name const* const&)
// IDA 0x147a8: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_147a8() {
}

// 0x14800 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)
// IDA 0x14800: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_14800() {
}

// 0x148b4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)
// IDA 0x148b4: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_148b4() {
}

// 0x1490c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)
// IDA 0x1490c: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1490c() {
}

// 0x14974 — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::resize(unsigned long,RBX::CRenderSettings::QualityLevel)")]
// was: std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::resize(unsigned long,RBX::CRenderSettings::QualityLevel)
// IDA 0x14974: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_14974() {
}

// 0x149a8 — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::push_back(RBX::CRenderSettings::QualityLevel const&)")]
// was: std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::push_back(RBX::CRenderSettings::QualityLevel const&)
// IDA 0x149a8: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_149a8() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x149d0 — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::QualityLevel*,std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>>,RBX::CRenderSettings::QualityLevel const&)")]
// was: std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::QualityLevel*,std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>>,RBX::CRenderSettings::QualityLevel const&)
// IDA 0x149d0: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_149d0() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x14ab4 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings12QualityLevelESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_allocate(unsigned long)")]
// was: std::_Vector_base<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_allocate(unsigned long)
// IDA 0x14ab4: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_14ab4() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x14acc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings12QualityLevelES6_EET0_T_S8_S7_
#[doc(alias = "RBX::CRenderSettings::QualityLevel * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *>(RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *)")]
// was: RBX::CRenderSettings::QualityLevel * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *>(RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *)
// IDA 0x14acc: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_14acc() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x14b08 — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::QualityLevel*,std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>>,unsigned long,RBX::CRenderSettings::QualityLevel const&)")]
// was: std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::QualityLevel*,std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>>,unsigned long,RBX::CRenderSettings::QualityLevel const&)
// IDA 0x14b08: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_14b08() {
}

// 0x14c98 — __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::resize(unsigned long,RBX::CRenderSettings::ShadowMode)")]
// was: std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::resize(unsigned long,RBX::CRenderSettings::ShadowMode)
// IDA 0x14c98: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_14c98() {
}

// 0x14ccc — __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::push_back(RBX::CRenderSettings::ShadowMode const&)")]
// was: std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::push_back(RBX::CRenderSettings::ShadowMode const&)
// IDA 0x14ccc: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_14ccc() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x14cf4 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings10ShadowModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::ShadowMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::operator[](RBX::Name const* const&)")]
// was: std::map<RBX::Name const*,RBX::CRenderSettings::ShadowMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::operator[](RBX::Name const* const&)
// IDA 0x14cf4: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_14cf4() {
}

// 0x14d4c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)
// IDA 0x14d4c: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_14d4c() {
}

// 0x14e00 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)
// IDA 0x14e00: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_14e00() {
}

// 0x14e58 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)
// IDA 0x14e58: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_14e58() {
}

// 0x14ec0 — __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ShadowMode*,std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>>,RBX::CRenderSettings::ShadowMode const&)")]
// was: std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ShadowMode*,std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>>,RBX::CRenderSettings::ShadowMode const&)
// IDA 0x14ec0: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_14ec0() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x14fa4 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings10ShadowModeESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_allocate(unsigned long)")]
// was: std::_Vector_base<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_allocate(unsigned long)
// IDA 0x14fa4: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_14fa4() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x14fbc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings10ShadowModeES6_EET0_T_S8_S7_
#[doc(alias = "RBX::CRenderSettings::ShadowMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *>(RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *)")]
// was: RBX::CRenderSettings::ShadowMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *>(RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *)
// IDA 0x14fbc: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_14fbc() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x14ff8 — __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ShadowMode*,std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>>,unsigned long,RBX::CRenderSettings::ShadowMode const&)")]
// was: std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ShadowMode*,std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>>,unsigned long,RBX::CRenderSettings::ShadowMode const&)
// IDA 0x14ff8: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_14ff8() {
}

// 0x15188 — __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::resize(unsigned long,RBX::CRenderSettings::AntialiasingMode)")]
// was: std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::resize(unsigned long,RBX::CRenderSettings::AntialiasingMode)
// IDA 0x15188: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_15188() {
}

// 0x151bc — __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::push_back(RBX::CRenderSettings::AntialiasingMode const&)")]
// was: std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::push_back(RBX::CRenderSettings::AntialiasingMode const&)
// IDA 0x151bc: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_151bc() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x151e4 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings16AntialiasingModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::AntialiasingMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::operator[](RBX::Name const* const&)")]
// was: std::map<RBX::Name const*,RBX::CRenderSettings::AntialiasingMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::operator[](RBX::Name const* const&)
// IDA 0x151e4: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_151e4() {
}

// 0x1523c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode> const&)
// IDA 0x1523c: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1523c() {
}

// 0x152f0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode> const&)
// IDA 0x152f0: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_152f0() {
}

// 0x15348 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode> const&)
// IDA 0x15348: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_15348() {
}

// 0x153b0 — __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AntialiasingMode*,std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>>,RBX::CRenderSettings::AntialiasingMode const&)")]
// was: std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AntialiasingMode*,std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>>,RBX::CRenderSettings::AntialiasingMode const&)
// IDA 0x153b0: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_153b0() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x15494 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_allocate(unsigned long)")]
// was: std::_Vector_base<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_allocate(unsigned long)
// IDA 0x15494: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_15494() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x154ac — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings16AntialiasingModeES6_EET0_T_S8_S7_
#[doc(alias = "RBX::CRenderSettings::AntialiasingMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *>(RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *)")]
// was: RBX::CRenderSettings::AntialiasingMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *>(RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *)
// IDA 0x154ac: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_154ac() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x154e8 — __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AntialiasingMode*,std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>>,unsigned long,RBX::CRenderSettings::AntialiasingMode const&)")]
// was: std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AntialiasingMode*,std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>>,unsigned long,RBX::CRenderSettings::AntialiasingMode const&)
// IDA 0x154e8: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_154e8() {
}

// 0x15678 — __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::resize(unsigned long,RBX::CRenderSettings::FrameRateManagerMode)")]
// was: std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::resize(unsigned long,RBX::CRenderSettings::FrameRateManagerMode)
// IDA 0x15678: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_15678() {
}

// 0x156ac — __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::push_back(RBX::CRenderSettings::FrameRateManagerMode const&)")]
// was: std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::push_back(RBX::CRenderSettings::FrameRateManagerMode const&)
// IDA 0x156ac: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_156ac() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x156d4 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings20FrameRateManagerModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::FrameRateManagerMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::operator[](RBX::Name const* const&)")]
// was: std::map<RBX::Name const*,RBX::CRenderSettings::FrameRateManagerMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::operator[](RBX::Name const* const&)
// IDA 0x156d4: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_156d4() {
}

// 0x1572c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode> const&)
// IDA 0x1572c: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1572c() {
}

// 0x157e0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode> const&)
// IDA 0x157e0: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_157e0() {
}

// 0x15838 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode> const&)
// IDA 0x15838: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_15838() {
}

// 0x158a0 — __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::FrameRateManagerMode*,std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>>,RBX::CRenderSettings::FrameRateManagerMode const&)")]
// was: std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::FrameRateManagerMode*,std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>>,RBX::CRenderSettings::FrameRateManagerMode const&)
// IDA 0x158a0: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_158a0() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x15984 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_allocate(unsigned long)")]
// was: std::_Vector_base<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_allocate(unsigned long)
// IDA 0x15984: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_15984() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x1599c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings20FrameRateManagerModeES6_EET0_T_S8_S7_
#[doc(alias = "RBX::CRenderSettings::FrameRateManagerMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *>(RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *)")]
// was: RBX::CRenderSettings::FrameRateManagerMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *>(RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *)
// IDA 0x1599c: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_1599c() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x159d8 — __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::FrameRateManagerMode*,std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>>,unsigned long,RBX::CRenderSettings::FrameRateManagerMode const&)")]
// was: std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::FrameRateManagerMode*,std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>>,unsigned long,RBX::CRenderSettings::FrameRateManagerMode const&)
// IDA 0x159d8: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_159d8() {
}

// 0x15b68 — __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::resize(unsigned long,RBX::CRenderSettings::GraphicsMode)")]
// was: std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::resize(unsigned long,RBX::CRenderSettings::GraphicsMode)
// IDA 0x15b68: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_15b68() {
}

// 0x15b9c — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings12GraphicsModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::GraphicsMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::operator[](RBX::Name const* const&)")]
// was: std::map<RBX::Name const*,RBX::CRenderSettings::GraphicsMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::operator[](RBX::Name const* const&)
// IDA 0x15b9c: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_15b9c() {
}

// 0x15bf4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)
// IDA 0x15bf4: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_15bf4() {
}

// 0x15ca8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)
// IDA 0x15ca8: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_15ca8() {
}

// 0x15d00 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)
// IDA 0x15d00: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_15d00() {
}

// 0x15d68 — __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::GraphicsMode*,std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>>,unsigned long,RBX::CRenderSettings::GraphicsMode const&)")]
// was: std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::GraphicsMode*,std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>>,unsigned long,RBX::CRenderSettings::GraphicsMode const&)
// IDA 0x15d68: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_15d68() {
}

// 0x15ef8 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_allocate(unsigned long)")]
// was: std::_Vector_base<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_allocate(unsigned long)
// IDA 0x15ef8: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_15ef8() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x15f10 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings12GraphicsModeES6_EET0_T_S8_S7_
#[doc(alias = "RBX::CRenderSettings::GraphicsMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *>(RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *)")]
// was: RBX::CRenderSettings::GraphicsMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *>(RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *)
// IDA 0x15f10: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_15f10() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x15f4c — __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::push_back(RBX::CRenderSettings::GraphicsMode const&)")]
// was: std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::push_back(RBX::CRenderSettings::GraphicsMode const&)
// IDA 0x15f4c: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_15f4c() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x15f74 — __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::GraphicsMode*,std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>>,RBX::CRenderSettings::GraphicsMode const&)")]
// was: std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::GraphicsMode*,std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>>,RBX::CRenderSettings::GraphicsMode const&)
// IDA 0x15f74: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_15f74() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x16058 — __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::resize(unsigned long,RBX::CRenderSettings::AASamples)")]
// was: std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::resize(unsigned long,RBX::CRenderSettings::AASamples)
// IDA 0x16058: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_16058() {
}

// 0x1608c — __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::push_back(RBX::CRenderSettings::AASamples const&)")]
// was: std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::push_back(RBX::CRenderSettings::AASamples const&)
// IDA 0x1608c: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_1608c() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x160b4 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings9AASamplesESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::AASamples,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::operator[](RBX::Name const* const&)")]
// was: std::map<RBX::Name const*,RBX::CRenderSettings::AASamples,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::operator[](RBX::Name const* const&)
// IDA 0x160b4: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_160b4() {
}

// 0x1610c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)
// IDA 0x1610c: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1610c() {
}

// 0x161c0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)
// IDA 0x161c0: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_161c0() {
}

// 0x16218 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)
// IDA 0x16218: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_16218() {
}

// 0x16280 — __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AASamples*,std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>>,RBX::CRenderSettings::AASamples const&)")]
// was: std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AASamples*,std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>>,RBX::CRenderSettings::AASamples const&)
// IDA 0x16280: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_16280() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x16364 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings9AASamplesESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_allocate(unsigned long)")]
// was: std::_Vector_base<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_allocate(unsigned long)
// IDA 0x16364: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_16364() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x1637c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings9AASamplesES6_EET0_T_S8_S7_
#[doc(alias = "RBX::CRenderSettings::AASamples * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *>(RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *)")]
// was: RBX::CRenderSettings::AASamples * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *>(RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *)
// IDA 0x1637c: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_1637c() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x163b8 — __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AASamples*,std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>>,unsigned long,RBX::CRenderSettings::AASamples const&)")]
// was: std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AASamples*,std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>>,unsigned long,RBX::CRenderSettings::AASamples const&)
// IDA 0x163b8: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_163b8() {
}

// 0x16548 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings10ShadowModeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode> const>::initSingleton(void)
// IDA 0x16548: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_16548() {
}

// 0x1654c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings10ShadowModeEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode> const>::doGetSingleton(void)
// IDA 0x1654c: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1654c() {
}

// 0x1663c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16ResolutionPresetEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset> const>::initSingleton(void)
// IDA 0x1663c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_1663c() {
}

// 0x16640 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16ResolutionPresetEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset> const>::doGetSingleton(void)
// IDA 0x16640: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_16640() {
}

// 0x16730 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12QualityLevelEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel> const>::initSingleton(void)
// IDA 0x16730: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_16730() {
}

// 0x16734 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12QualityLevelEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel> const>::doGetSingleton(void)
// IDA 0x16734: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_16734() {
}

// 0x16824 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16AntialiasingModeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode> const>::initSingleton(void)
// IDA 0x16824: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_16824() {
}

// 0x16828 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16AntialiasingModeEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode> const>::doGetSingleton(void)
// IDA 0x16828: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_16828() {
}

// 0x16918 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode> const>::initSingleton(void)
// IDA 0x16918: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_16918() {
}

// 0x1691c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode> const>::doGetSingleton(void)
// IDA 0x1691c: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1691c() {
}

// 0x16a0c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12GraphicsModeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode> const>::initSingleton(void)
// IDA 0x16a0c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_16a0c() {
}

// 0x16a10 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12GraphicsModeEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode> const>::doGetSingleton(void)
// IDA 0x16a10: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_16a10() {
}

// 0x16b00 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings9AASamplesEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples> const>::initSingleton(void)
// IDA 0x16b00: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_16b00() {
}

// 0x16b04 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings9AASamplesEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples> const>::doGetSingleton(void)
// IDA 0x16b04: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_16b04() {
}

// 0x16bf4 — __ZN19CRenderSettingsItemD2Ev
#[doc(alias = "CRenderSettingsItem::~CRenderSettingsItem()")]
// was: CRenderSettingsItem::~CRenderSettingsItem()
// IDA 0x16bf4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_16bf4() {
}

// 0x16d34 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>> *)
// IDA 0x16d34: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_16d34() {
}

// 0x16d5c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>> *)
// IDA 0x16d5c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_16d5c() {
}

// 0x16d84 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>> *)
// IDA 0x16d84: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_16d84() {
}

// 0x16dac — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>> *)
// IDA 0x16dac: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_16dac() {
}

// 0x16dd4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>> *)
// IDA 0x16dd4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_16dd4() {
}

// 0x16dfc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>> *)
// IDA 0x16dfc: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_16dfc() {
}

// 0x16e24 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>> *)
// IDA 0x16e24: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_16e24() {
}

// 0x3a408 — __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEE9singletonEv
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEE9singletonEv")]
// was: __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEE9singletonEv
// IDA 0x3a408: 160 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a408() {
}

// 0x3e0b0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerI19CRenderSettingsItemS6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<CRenderSettingsItem,CRenderSettingsItem>(rbx_core::SharedPtr<CRenderSettingsItem> const*,CRenderSettingsItem *)const")]
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<CRenderSettingsItem,CRenderSettingsItem>(boost::shared_ptr<CRenderSettingsItem> const*,CRenderSettingsItem *)const
// IDA 0x3e0b0: 80 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e0b0() {
}

// 0x3e190 — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// IDA 0x3e190: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3e190() {
}


// 0x93d760 — __ZN3RBX12SceneUpdaterC2EN5boost10shared_ptrINS_9DataModelEEEPNS_15CRenderSettingsEPKNS_10RenderCapsEPNS_11RenderStatsE
#[doc(alias = "RBX::SceneUpdater::SceneUpdater(rbx_core::SharedPtr<RBX::DataModel>,RBX::CRenderSettings *,RBX::RenderCaps const*,RBX::RenderStats *)")]
// was: RBX::SceneUpdater::SceneUpdater(boost::shared_ptr<RBX::DataModel>,RBX::CRenderSettings *,RBX::RenderCaps const*,RBX::RenderStats *)
// IDA 0x93d760: 931 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_93d760() {
}

// 0xbe6e54 — __ZN3RBX10ViewRbxGfxC2ENS_15CRenderSettings12GraphicsModeEPNS_9OSContextEPS1_
#[doc(alias = "RBX::ViewRbxGfx::ViewRbxGfx(RBX::CRenderSettings::GraphicsMode,RBX::OSContext *,RBX::CRenderSettings*)")]
// was: RBX::ViewRbxGfx::ViewRbxGfx(RBX::CRenderSettings::GraphicsMode,RBX::OSContext *,RBX::CRenderSettings*)
// IDA 0xbe6e54: 685 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be6e54() {
}

// 0xbef270 — __ZZN3RBX21ViewRbxGfx_InitModuleEvEN17ViewRbxGfxFactory6CreateENS_15CRenderSettings12GraphicsModeEPNS_9OSContextEPS1_
#[doc(alias = "RBX::ViewRbxGfx_InitModule(void)::ViewRbxGfxFactory::Create(RBX::CRenderSettings::GraphicsMode,RBX::OSContext *,RBX::CRenderSettings*)")]
// was: RBX::ViewRbxGfx_InitModule(void)::ViewRbxGfxFactory::Create(RBX::CRenderSettings::GraphicsMode,RBX::OSContext *,RBX::CRenderSettings*)
// IDA 0xbef270: 62 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bef270() {
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU32, Ordering as AOrd};

    // Signal slots are weak: the returned handle must be kept alive or the
    // probe goes dead (same contract as rbx_core::signal::Signal::connect).
    fn fired_with(item: &RenderSettingsItem) -> (Arc<AtomicU32>, Arc<impl Fn(u32) + Send + Sync>) {
        let seen = Arc::new(AtomicU32::new(u32::MAX));
        let probe = Arc::clone(&seen);
        let handle = Arc::new(move |ea: u32| probe.store(ea, AOrd::SeqCst));
        item.changed.connect(Arc::clone(&handle));
        (seen, handle)
    }

    #[test]
    fn settings_layout_matches_ida_offsets() {
        use std::mem::{offset_of, size_of};
        assert_eq!(size_of::<RenderSettings>(), 72);
        assert_eq!(offset_of!(RenderSettings, graphics_mode), 4);
        assert_eq!(offset_of!(RenderSettings, antialiasing_mode), 8);
        assert_eq!(offset_of!(RenderSettings, shadow_mode), 12);
        assert_eq!(offset_of!(RenderSettings, frame_rate_manager_mode), 16);
        assert_eq!(offset_of!(RenderSettings, quality_level), 20);
        assert_eq!(offset_of!(RenderSettings, resolution_preference), 24);
        assert_eq!(offset_of!(RenderSettings, auto_quality_level), 28);
        assert_eq!(offset_of!(RenderSettings, max_quality_level), 32);
        assert_eq!(offset_of!(RenderSettings, debug_show_bounding_boxes), 40);
        assert_eq!(offset_of!(RenderSettings, enable_frm), 41);
        assert_eq!(offset_of!(RenderSettings, unk_50_video_tier), 50);
        assert_eq!(offset_of!(RenderSettings, show_aggregation), 58);
        assert_eq!(offset_of!(RenderSettings, always_draw_connectors), 59);
        assert_eq!(offset_of!(RenderSettings, connector_secondary), 60);
        assert_eq!(offset_of!(RenderSettings, eager_bulk_execution), 61);
        assert_eq!(offset_of!(RenderSettings, texture_cache_size), 64);
        assert_eq!(offset_of!(RenderSettings, mesh_cache_size), 68);
    }

    #[test]
    fn guarded_setter_fires_once_then_silent() {
        let mut item = stub_97d0(0);
        let (seen, _hook) = fired_with(&item);
        assert!(stub_9608(&mut item, 3));
        assert_eq!(seen.load(AOrd::SeqCst), DESC_GRAPHICS_MODE);
        assert_eq!(stub_b33c(&item.settings), 3);
        seen.store(u32::MAX, AOrd::SeqCst);
        assert!(!stub_9608(&mut item, 3));
        assert_eq!(seen.load(AOrd::SeqCst), u32::MAX);
    }

    #[test]
    fn blind_cache_setters_write_without_signal() {
        let mut item = stub_97d0(0);
        let (seen, _hook) = fired_with(&item);
        assert!(!stub_97c0(&mut item, 4096));
        assert!(!stub_97c8(&mut item, 2048));
        assert_eq!(item.settings.texture_cache_size, 4096);
        assert_eq!(item.settings.mesh_cache_size, 2048);
        assert_eq!(seen.load(AOrd::SeqCst), u32::MAX);
    }

    #[test]
    fn connectors_fold_matches_ida_effective_value() {
        // Fresh item: +155 false, +156 false -> old effective false.
        let mut item = stub_97d0(0);
        let (seen, _hook) = fired_with(&item);
        assert!(stub_9668(&mut item, true));
        assert_eq!(seen.load(AOrd::SeqCst), DESC_ALWAYS_DRAW_CONNECTORS);
        // Enabling again: old effective now true -> silent (IDA returns this).
        seen.store(u32::MAX, AOrd::SeqCst);
        assert!(!stub_9668(&mut item, true));
        assert_eq!(seen.load(AOrd::SeqCst), u32::MAX);
        // Secondary source high: disabling the request leaves effective true -> silent.
        item.settings.connector_secondary = true;
        assert!(!stub_9668(&mut item, false));
        // Secondary low: disabling flips effective true->false -> fires.
        item.settings.connector_secondary = false;
        item.settings.always_draw_connectors = true;
        assert!(stub_9668(&mut item, false));
    }

    #[test]
    fn aa_samples_global_round_trip() {
        let mut item = stub_97d0(0);
        assert!(stub_96d0(&mut item, 4));
        assert_eq!(stub_b3e8(), 4);
        assert!(!stub_96d0(&mut item, 4));
        AA_SAMPLES.store(0, Ordering::Relaxed);
    }

    #[test]
    fn interpolation_global_round_trip() {
        assert!(!stub_9784());
        assert!(stub_9794(true));
        assert!(stub_9784());
        assert!(!stub_9794(false));
    }

    #[test]
    fn thunk_matches_primary_setter() {
        let mut item = stub_97d0(0);
        assert!(stub_9ae8(&mut item, 7));
        assert_eq!(stub_b474(&item.settings), 7);
        assert!(!stub_9ae8(&mut item, 7));
    }

    #[test]
    fn ctor_defaults_match_ida() {
        let lo = stub_97d0(0);
        assert_eq!(lo.default_resolution, [800, 600]);
        assert_eq!(lo.resolutions, vec![[800, 600]]);
        assert!(lo.unk_189_flag);
        assert_eq!(lo.settings.video_tier(), VIDEO_TIER_LOW_VRAM);
        let hi = stub_97d0(DX_VIDEO_MEMORY_THRESHOLD + 1);
        assert_eq!(hi.settings.video_tier(), VIDEO_TIER_HIGH_VRAM);
    }
}
