// Auto-generated skeletons for rbx-script — filler EA-sorted asc next 100 uncovered (script 71992->71892)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use rbx_core::signal::Signal;
use rbx_reflection::enum_desc::EnumDesc;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

/// `RBX::CRenderSettings::aaSamples` global backing store (IDA 0x96d0).
static AA_SAMPLES: AtomicI32 = AtomicI32::new(0);
/// `RBX::PartInstance::disableInterpolation` global bit (IDA 0x9784/0x9794).
static DISABLE_INTERPOLATION: AtomicBool = AtomicBool::new(false);

/// `CRenderSettingsItem` property bag (IDA 0x97d0).
/// Field comments give the original `this` offset and width; the +192
/// `changed` signal replaces `rbx::signals::signal_with_args<1, ...>`.
pub struct CRenderSettingsItem {
    /// +104 `AntialiasingMode` (i32).
    pub antialiasing_mode: i32,
    /// +108 `ShadowMode` (i32).
    pub shadow_mode: i32,
    /// +100 `GraphicsMode` (i32).
    pub graphics_mode: i32,
    /// +112 `FrameRateManagerMode` (i32).
    pub frame_rate_manager_mode: i32,
    /// +116 `QualityLevel` (i32).
    pub quality_level: i32,
    /// +120 `ResolutionPreference` (i32).
    pub resolution_preference: i32,
    /// +124 `AutoQualityLevel` (i32).
    pub auto_quality_level: i32,
    /// +136 `DebugShowBoundingBoxes` (bool).
    pub debug_show_bounding_boxes: bool,
    /// +137 `EnableFRM` (bool).
    pub enable_frm: bool,
    /// +154 `ShowAggregation` (bool).
    pub show_aggregation: bool,
    /// +155 `AlwaysDrawConnectors` (bool).
    pub always_draw_connectors: bool,
    /// +156 connector-visibility bit feeding the 0x9668 effective state.
    pub connector_draw_enabled: bool,
    /// +157 `EagerBulkExecution` (bool).
    pub eager_bulk_execution: bool,
    /// +160 `TextureCacheSize` (u32).
    pub texture_cache_size: u32,
    /// +164 `MeshCacheSize` (u32).
    pub mesh_cache_size: u32,
    /// +176 resolution list; ctor seeds 800x600 (IDA 0x97d0).
    pub resolutions: Vec<(u16, u16)>,
    /// Reflection category assigned by the ctor (`"Rendering"`, IDA 0x97d0).
    pub category: String,
    /// +192 property-changed signal; payload is the descriptor name.
    pub property_changed: Signal<String>,
}

// 0x850c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEEC2Ev
// type: int __fastcall(int)
// IDA 0x850c: EnumDescriptor base ("AASamples") + vtable + empty tables,
// then addPair(1, "None"), addPair(4, "4"), addPair(8, "8").
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::EnumDesc(void)")]
pub fn enum_desc_aa_samples_ctor() -> EnumDesc {
    let mut desc = EnumDesc::new("AASamples");
    desc.add_pair(1, "None");
    desc.add_pair(4, "4");
    desc.add_pair(8, "8");
    desc
}

// 0x86d0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEEC2Ev
// type: int __fastcall(int)
// IDA 0x86d0: EnumDescriptor base ("GraphicsMode") + vtable, then four
// addPair calls plus addLegacy(2, "OpenGL legacy", 1).
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::EnumDesc(void)")]
pub fn enum_desc_graphics_mode_ctor() -> EnumDesc {
    let mut desc = EnumDesc::new("GraphicsMode");
    desc.add_pair(1, "Automatic");
    desc.add_pair(3, "Direct3D");
    desc.add_pair(4, "OpenGL");
    desc.add_pair(5, "NoGraphics");
    desc.add_legacy(2, "OpenGL legacy", 1);
    desc
}

// 0x88c4 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEC2Ev
// type: int __fastcall(int)
// IDA 0x88c4: EnumDescriptor base ("FramerateManagerMode", note original
// spelling) + vtable, then addPair(0/1/2, Automatic/On/Off).
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::EnumDesc(void)")]
pub fn enum_desc_frame_rate_manager_mode_ctor() -> EnumDesc {
    let mut desc = EnumDesc::new("FramerateManagerMode");
    desc.add_pair(0, "Automatic");
    desc.add_pair(1, "On");
    desc.add_pair(2, "Off");
    desc
}

// 0x8a88 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEEC2Ev
// type: int __fastcall(int)
// IDA 0x8a88: EnumDescriptor base ("Antialiasing") + vtable, then
// addPair(0, "Automatic"), addPair(2, "Off"), addPair(1, "On").
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::EnumDesc(void)")]
pub fn enum_desc_antialiasing_mode_ctor() -> EnumDesc {
    let mut desc = EnumDesc::new("Antialiasing");
    desc.add_pair(0, "Automatic");
    desc.add_pair(2, "Off");
    desc.add_pair(1, "On");
    desc
}

// 0x8c4c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEEC2Ev
// type: int __fastcall(int)
// IDA 0x8c4c: EnumDescriptor base ("Shadow") + vtable, then addPair(0/1/3/2,
// Automatic/All/CharacterOnly/Off).
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::EnumDesc(void)")]
pub fn enum_desc_shadow_mode_ctor() -> EnumDesc {
    let mut desc = EnumDesc::new("Shadow");
    desc.add_pair(0, "Automatic");
    desc.add_pair(1, "All");
    desc.add_pair(3, "CharacterOnly");
    desc.add_pair(2, "Off");
    desc
}

// 0x8e24 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEEC2Ev
// type: RBX::Reflection::EnumDescriptor *__fastcall(RBX::Reflection::EnumDescriptor *)
// IDA 0x8e24: EnumDescriptor base ("QualityLevel") + vtable, addPair(0,
// "Automatic"), addPair(i, "Level%02d") for i in 1..22, then per-level
// `Name::declare("Level %2u")` aliases into the name map.
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::EnumDesc(void)")]
pub fn enum_desc_quality_level_ctor() -> EnumDesc {
    let mut desc = EnumDesc::new("QualityLevel");
    desc.add_pair(0, "Automatic");
    for level in 1..22 {
        desc.add_pair(level, &format!("Level{level:02}"));
    }
    for level in 1..22i32 {
        desc.add_legacy(level as usize, &format!("Level {level:2}"), level);
    }
    desc
}

// 0x9100 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEEC2Ev
// type: RBX::Reflection::EnumDescriptor *__fastcall(RBX::Reflection::EnumDescriptor *)
// IDA 0x9100: EnumDescriptor base ("Resolution") + vtable, addPair 0..18 with
// canonical names, plus `Name::declare("<WxH> (wide)")` aliases for the
// wide entries mapping back to the same value.
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::EnumDesc(void)")]
pub fn enum_desc_resolution_preset_ctor() -> EnumDesc {
    let mut desc = EnumDesc::new("Resolution");
    const PAIRS: [(i32, &str); 19] = [
        (0, "Automatic"),
        (1, "720x526"),
        (2, "800x600"),
        (3, "1024x600"),
        (4, "1024x768"),
        (5, "1280x720"),
        (6, "1280x768"),
        (7, "1152x864"),
        (8, "1280x800"),
        (9, "1360x768"),
        (10, "1280x960"),
        (11, "1280x1024"),
        (12, "1440x900"),
        (13, "1600x900"),
        (14, "1600x1024"),
        (15, "1600x1200"),
        (16, "1680x1050"),
        (17, "1920x1080"),
        (18, "1920x1200"),
    ];
    for (value, name) in PAIRS {
        desc.add_pair(value, name);
    }
    const WIDE_ALIASES: [(i32, &str); 11] = [
        (3, "1024x600 (wide)"),
        (5, "1280x720 (wide)"),
        (6, "1280x768 (wide)"),
        (8, "1280x800 (wide)"),
        (9, "1360x768 (wide)"),
        (12, "1440x900 (wide)"),
        (13, "1600x900 (wide)"),
        (14, "1600x1024 (wide)"),
        (16, "1680x1050 (wide)"),
        (17, "1920x1080 (wide)"),
        (18, "1920x1200 (wide)"),
    ];
    for (value, name) in WIDE_ALIASES {
        desc.add_legacy(value as usize, name, value);
    }
    desc
}

// 0x9608 — __ZN19CRenderSettingsItem15setGraphicsModeEN3RBX15CRenderSettings12GraphicsModeE
// type: int __fastcall(int result, int)
// IDA 0x9608: if (this[100] != mode) { this[100] = mode; changed(prop); }
impl CRenderSettingsItem {
    #[doc(alias = "CRenderSettingsItem::setGraphicsMode(RBX::CRenderSettings::GraphicsMode)")]
    pub fn set_graphics_mode(&mut self, mode: i32) -> &mut Self {
        if self.graphics_mode != mode {
            self.graphics_mode = mode;
            self.property_changed.fire("GraphicsMode".to_owned());
        }
        self
    }

// 0x9628 — __ZN19CRenderSettingsItem23setFrameRateManagerModeEN3RBX15CRenderSettings20FrameRateManagerModeE
// type: int __fastcall(int result, int)
// IDA 0x9628: if (this[112] != mode) { this[112] = mode; changed(prop); }
    #[doc(alias = "CRenderSettingsItem::setFrameRateManagerMode(RBX::CRenderSettings::FrameRateManagerMode)")]
    pub fn set_frame_rate_manager_mode(&mut self, mode: i32) -> &mut Self {
        if self.frame_rate_manager_mode != mode {
            self.frame_rate_manager_mode = mode;
            self.property_changed.fire("FrameRateManagerMode".to_owned());
        }
        self
    }

// 0x9648 — __ZN19CRenderSettingsItem15setQualityLevelEN3RBX15CRenderSettings12QualityLevelE
// type: int __fastcall(int result, int)
// IDA 0x9648: if (this[116] != level) { this[116] = level; changed(prop); }
    #[doc(alias = "CRenderSettingsItem::setQualityLevel(RBX::CRenderSettings::QualityLevel)")]
    pub fn set_quality_level(&mut self, level: i32) -> &mut Self {
        if self.quality_level != level {
            self.quality_level = level;
            self.property_changed.fire("QualityLevel".to_owned());
        }
        self
    }

// 0x9668 — __ZN19CRenderSettingsItem23setAlwaysDrawConnectorsEb
// type: int __fastcall(int this, int)
// IDA 0x9668: effective-old = this[155] ? true : !!this[156]; store a2 to
// this[155]; when enabling, notify only if previously ineffective;
// otherwise notify iff the effective state changed.
    #[doc(alias = "CRenderSettingsItem::setAlwaysDrawConnectors(bool)")]
    pub fn set_always_draw_connectors(&mut self, value: bool) -> &mut Self {
        let old_effective = self.always_draw_connectors || self.connector_draw_enabled;
        self.always_draw_connectors = value;
        if value {
            if !old_effective {
                self.property_changed.fire("AlwaysDrawConnectors".to_owned());
            }
        } else if old_effective != self.connector_draw_enabled {
            self.property_changed.fire("AlwaysDrawConnectors".to_owned());
        }
        self
    }

// 0x96ac — __ZN19CRenderSettingsItem18setShowAggregationEb
// type: int __fastcall(int this, int)
// IDA 0x96ac: if (a2 != this[154]) { this[154] = a2; changed(prop); }
    #[doc(alias = "CRenderSettingsItem::setShowAggregation(bool)")]
    pub fn set_show_aggregation(&mut self, value: bool) -> &mut Self {
        if self.show_aggregation != value {
            self.show_aggregation = value;
            self.property_changed.fire("ShowAggregation".to_owned());
        }
        self
    }

// 0x96d0 — __ZN19CRenderSettingsItem12setAASamplesEN3RBX15CRenderSettings9AASamplesE
// type: int __fastcall(int result, int)
// IDA 0x96d0: compares/stores the `RBX::CRenderSettings::aaSamples` global,
// then notifies through this+192.
    #[doc(alias = "CRenderSettingsItem::setAASamples(RBX::CRenderSettings::AASamples)")]
    pub fn set_aa_samples(&mut self, samples: i32) -> &mut Self {
        if AA_SAMPLES.load(Ordering::Relaxed) != samples {
            AA_SAMPLES.store(samples, Ordering::Relaxed);
            self.property_changed.fire("AASamples".to_owned());
        }
        self
    }

// 0x96fc — __ZN19CRenderSettingsItem13setShadowModeEN3RBX15CRenderSettings10ShadowModeE
// type: int __fastcall(int result, int)
// IDA 0x96fc: if (this[108] != mode) { this[108] = mode; changed(prop); }
    #[doc(alias = "CRenderSettingsItem::setShadowMode(RBX::CRenderSettings::ShadowMode)")]
    pub fn set_shadow_mode(&mut self, mode: i32) -> &mut Self {
        if self.shadow_mode != mode {
            self.shadow_mode = mode;
            self.property_changed.fire("ShadowMode".to_owned());
        }
        self
    }

// 0x971c — __ZN19CRenderSettingsItem19setAntialiasingModeEN3RBX15CRenderSettings16AntialiasingModeE
// type: int __fastcall(int result, int)
// IDA 0x971c: if (this[104] != mode) { this[104] = mode; changed(prop); }
    #[doc(alias = "CRenderSettingsItem::setAntialiasingMode(RBX::CRenderSettings::AntialiasingMode)")]
    pub fn set_antialiasing_mode(&mut self, mode: i32) -> &mut Self {
        if self.antialiasing_mode != mode {
            self.antialiasing_mode = mode;
            self.property_changed.fire("AntialiasingMode".to_owned());
        }
        self
    }

// 0x973c — __ZN19CRenderSettingsItem25setDebugShowBoundingBoxesEb
// type: int __fastcall(int this, int)
// IDA 0x973c: if (a2 != this[136]) { this[136] = a2; changed(prop); }
    #[doc(alias = "CRenderSettingsItem::setDebugShowBoundingBoxes(bool)")]
    pub fn set_debug_show_bounding_boxes(&mut self, value: bool) -> &mut Self {
        if self.debug_show_bounding_boxes != value {
            self.debug_show_bounding_boxes = value;
            self.property_changed.fire("DebugShowBoundingBoxes".to_owned());
        }
        self
    }

// 0x9760 — __ZN19CRenderSettingsItem12setEnableFRMEb
// type: int __fastcall(int this, int)
// IDA 0x9760: if (a2 != this[137]) { this[137] = a2; changed(prop); }
    #[doc(alias = "CRenderSettingsItem::setEnableFRM(bool)")]
    pub fn set_enable_frm(&mut self, value: bool) -> &mut Self {
        if self.enable_frm != value {
            self.enable_frm = value;
            self.property_changed.fire("EnableFRM".to_owned());
        }
        self
    }
}

// 0x9784 — __ZNK19CRenderSettingsItem28getDebugDisableInterpolationEv
// type: int __fastcall(CRenderSettingsItem *this)
// IDA 0x9784: return `RBX::PartInstance::disableInterpolation` global bit.
#[doc(alias = "CRenderSettingsItem::getDebugDisableInterpolation(void)const")]
pub fn debug_disable_interpolation() -> bool {
    DISABLE_INTERPOLATION.load(Ordering::Relaxed)
}

// 0x9794 — __ZN19CRenderSettingsItem28setDebugDisableInterpolationEb
// type: char *__fastcall(CRenderSettingsItem *this, char)
// IDA 0x9794: store a2 to the global bit, return its address.
#[doc(alias = "CRenderSettingsItem::setDebugDisableInterpolation(bool)")]
pub fn set_debug_disable_interpolation(value: bool) -> &'static AtomicBool {
    DISABLE_INTERPOLATION.store(value, Ordering::Relaxed);
    &DISABLE_INTERPOLATION
}

// 0x97a4 — __ZN19CRenderSettingsItem23setResolutionPreferenceEN3RBX15CRenderSettings16ResolutionPresetE
// type: int __fastcall(int result, int)
// IDA 0x97a4: if (this[120] != preset) { this[120] = preset; changed(prop); }
impl CRenderSettingsItem {
    #[doc(alias = "CRenderSettingsItem::setResolutionPreference(RBX::CRenderSettings::ResolutionPreset)")]
    pub fn set_resolution_preference(&mut self, preset: i32) -> &mut Self {
        if self.resolution_preference != preset {
            self.resolution_preference = preset;
            self.property_changed.fire("ResolutionPreference".to_owned());
        }
        self
    }

// 0x97c0 — __ZN19CRenderSettingsItem19setTextureCacheSizeEj
// type: int __fastcall(int this, unsigned int)
// IDA 0x97c0: unconditional store to this[160].
    #[doc(alias = "CRenderSettingsItem::setTextureCacheSize(unsigned int)")]
    pub fn set_texture_cache_size(&mut self, size: u32) -> &mut Self {
        self.texture_cache_size = size;
        self
    }

// 0x97c8 — __ZN19CRenderSettingsItem16setMeshCacheSizeEj
// type: int __fastcall(int this, unsigned int)
// IDA 0x97c8: unconditional store to this[164].
    #[doc(alias = "CRenderSettingsItem::setMeshCacheSize(unsigned int)")]
    pub fn set_mesh_cache_size(&mut self, size: u32) -> &mut Self {
        self.mesh_cache_size = size;
        self
    }

// 0x97d0 — __ZN19CRenderSettingsItemC2Ev
// type: void __fastcall(CRenderSettingsItem *this)
// IDA 0x97d0: base `GlobalAdvancedSettingsItem` + `CRenderSettings` ctors,
// vtable installs, changed-signal init, category "Rendering", seed the
// resolution list with 800x600, then size the texture cache from
// `GetDXVideoMemorySize()` (low 39322400 vs high 50332672).
    #[doc(alias = "CRenderSettingsItem::CRenderSettingsItem(void)")]
    pub fn new() -> Self {
        let mut item = Self {
            resolutions: vec![(800, 600)],
            category: "Rendering".to_owned(),
            ..Default::default()
        };
        let video_memory = Self::dx_video_memory_size();
        item.texture_cache_size = if video_memory > Self::HIGH_VIDEO_MEMORY_THRESHOLD {
            Self::TEXTURE_CACHE_HIGH
        } else {
            Self::TEXTURE_CACHE_LOW
        };
        item
    }
    /// Low-branch texture cache size (IDA 0x97d0, `v14 = 39322400`).
    pub const TEXTURE_CACHE_LOW: u32 = 39_322_400;
    /// High-branch texture cache size (IDA 0x97d0, `v14 = 50332672`).
    pub const TEXTURE_CACHE_HIGH: u32 = 50_332_672;
    /// `GetDXVideoMemorySize()` comparison point (`loc_F423FC + 3`, IDA 0x97d0).
    /// Unresolved platform constant; the host query below reports 0 so the
    /// low branch is taken until the iOS video-memory probe lands.
    pub const HIGH_VIDEO_MEMORY_THRESHOLD: u32 = u32::MAX;
    /// Host video-memory probe standing in for `GetDXVideoMemorySize()`.
    pub fn dx_video_memory_size() -> u32 {
        0
    }
}
impl Default for CRenderSettingsItem {
    fn default() -> Self {
        Self {
            antialiasing_mode: 0,
            shadow_mode: 0,
            graphics_mode: 0,
            frame_rate_manager_mode: 0,
            quality_level: 0,
            resolution_preference: 0,
            auto_quality_level: 0,
            debug_show_bounding_boxes: false,
            enable_frm: false,
            show_aggregation: false,
            always_draw_connectors: false,
            connector_draw_enabled: false,
            eager_bulk_execution: false,
            texture_cache_size: 0,
            mesh_cache_size: 0,
            resolutions: Vec::new(),
            category: String::new(),
            property_changed: Signal::new(),
        }
    }
}

// 0x9ac8 — __ZN19CRenderSettingsItem19setAutoQualityLevelEi
// type: int __fastcall(int this, int)
// IDA 0x9ac8: if (this[124] != level) { this[124] = level; changed(prop); }
impl CRenderSettingsItem {
    #[doc(alias = "CRenderSettingsItem::setAutoQualityLevel(int)")]
    pub fn set_auto_quality_level(&mut self, level: i32) -> &mut Self {
        if self.auto_quality_level != level {
            self.auto_quality_level = level;
            self.property_changed.fire("AutoQualityLevel".to_owned());
        }
        self
    }

// 0x9ae8 — __ZThn96_N19CRenderSettingsItem19setAutoQualityLevelEi
// type: int __fastcall(int this, int)
// was: non-virtual thunk to CRenderSettingsItem::setAutoQualityLevel(int)
// IDA 0x9ae8: this-adjust (`this -= 96`, i.e. field +28 aliases +124) then
// the 0x9ac8 body verbatim.
    #[doc(alias = "non-virtual thunk toCRenderSettingsItem::setAutoQualityLevel(int)")]
    pub fn set_auto_quality_level_thunk(&mut self, level: i32) -> &mut Self {
        self.set_auto_quality_level(level)
    }

// 0x9b08 — __ZN19CRenderSettingsItem21setEagerBulkExecutionEb
// type: int __fastcall(int this, int)
// IDA 0x9b08: if (a2 != this[157]) { this[157] = a2; changed(prop); }
    #[doc(alias = "CRenderSettingsItem::setEagerBulkExecution(bool)")]
    pub fn set_eager_bulk_execution(&mut self, value: bool) -> &mut Self {
        if self.eager_bulk_execution != value {
            self.eager_bulk_execution = value;
            self.property_changed.fire("EagerBulkExecution".to_owned());
        }
        self
    }
}

// 0x9b48 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::addPair(RBX::CRenderSettings::AASamples,char const*)")]
pub fn stub_0x9b48() -> ! {
    todo!("0x9b48 __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE7addPairES3_PKc")
}

// 0x9ea8 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::addPair(RBX::CRenderSettings::GraphicsMode,char const*)")]
pub fn stub_0x9ea8() -> ! {
    todo!("0x9ea8 __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE7addPairES3_PKc")
}

// 0xa208 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE9addLegacyEiPKcS3_
// type: _DWORD *__fastcall(int, unsigned int, int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::addLegacy(int,char const*,RBX::CRenderSettings::GraphicsMode)")]
pub fn stub_0xa208() -> ! {
    todo!("0xa208 __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE9addLegacyEiPKcS3_")
}

// 0xa25c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::addPair(RBX::CRenderSettings::FrameRateManagerMode,char const*)")]
pub fn stub_0xa25c() -> ! {
    todo!("0xa25c __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE7addPairES3_PKc")
}

// 0xa5bc — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::addPair(RBX::CRenderSettings::AntialiasingMode,char const*)")]
pub fn stub_0xa5bc() -> ! {
    todo!("0xa5bc __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE7addPairES3_PKc")
}

// 0xa91c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::addPair(RBX::CRenderSettings::ShadowMode,char const*)")]
pub fn stub_0xa91c() -> ! {
    todo!("0xa91c __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE7addPairES3_PKc")
}

// 0xac7c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::addPair(RBX::CRenderSettings::QualityLevel,char const*)")]
pub fn stub_0xac7c() -> ! {
    todo!("0xac7c __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE7addPairES3_PKc")
}

// 0xafdc — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::addPair(RBX::CRenderSettings::ResolutionPreset,char const*)")]
pub fn stub_0xafdc() -> ! {
    todo!("0xafdc __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE7addPairES3_PKc")
}

// 0xb33c — __ZNK3RBX15CRenderSettings15getGraphicsModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getGraphicsMode(void)const")]
pub fn stub_0xb33c() -> ! {
    todo!("0xb33c __ZNK3RBX15CRenderSettings15getGraphicsModeEv")
}

// 0xb340 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::~EnumPropDescriptor() [0xb340]")]
pub fn stub_0xb340() -> ! {
    todo!("0xb340 __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEED1Ev")
}

// 0xb364 — __ZNK3RBX15CRenderSettings23getFrameRateManagerModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getFrameRateManagerMode(void)const")]
pub fn stub_0xb364() -> ! {
    todo!("0xb364 __ZNK3RBX15CRenderSettings23getFrameRateManagerModeEv")
}

// 0xb368 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::~EnumPropDescriptor() [0xb368]")]
pub fn stub_0xb368() -> ! {
    todo!("0xb368 __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEED1Ev")
}

// 0xb38c — __ZNK3RBX15CRenderSettings15getQualityLevelEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getQualityLevel(void)const")]
pub fn stub_0xb38c() -> ! {
    todo!("0xb38c __ZNK3RBX15CRenderSettings15getQualityLevelEv")
}

// 0xb390 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::~EnumPropDescriptor() [0xb390]")]
pub fn stub_0xb390() -> ! {
    todo!("0xb390 __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEED1Ev")
}

// 0xb3b4 — __ZNK3RBX15CRenderSettings23getAlwaysDrawConnectorsEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getAlwaysDrawConnectors(void)const")]
pub fn stub_0xb3b4() -> ! {
    todo!("0xb3b4 __ZNK3RBX15CRenderSettings23getAlwaysDrawConnectorsEv")
}

// 0xb3bc — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::~PropDescriptor() [0xb3bc]")]
pub fn stub_0xb3bc() -> ! {
    todo!("0xb3bc __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembED1Ev")
}

// 0xb3e0 — __ZNK3RBX15CRenderSettings18getShowAggregationEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getShowAggregation(void)const")]
pub fn stub_0xb3e0() -> ! {
    todo!("0xb3e0 __ZNK3RBX15CRenderSettings18getShowAggregationEv")
}

// 0xb3e8 — __ZNK3RBX15CRenderSettings12getAASamplesEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getAASamples(void)const")]
pub fn stub_0xb3e8() -> ! {
    todo!("0xb3e8 __ZNK3RBX15CRenderSettings12getAASamplesEv")
}

// 0xb3f8 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::~EnumPropDescriptor() [0xb3f8]")]
pub fn stub_0xb3f8() -> ! {
    todo!("0xb3f8 __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEED1Ev")
}

// 0xb41c — __ZNK3RBX15CRenderSettings13getShadowModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getShadowMode(void)const")]
pub fn stub_0xb41c() -> ! {
    todo!("0xb41c __ZNK3RBX15CRenderSettings13getShadowModeEv")
}

// 0xb420 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::~EnumPropDescriptor() [0xb420]")]
pub fn stub_0xb420() -> ! {
    todo!("0xb420 __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEED1Ev")
}

// 0xb444 — __ZNK3RBX15CRenderSettings19getAntialiasingModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getAntialiasingMode(void)const")]
pub fn stub_0xb444() -> ! {
    todo!("0xb444 __ZNK3RBX15CRenderSettings19getAntialiasingModeEv")
}

// 0xb448 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::~EnumPropDescriptor() [0xb448]")]
pub fn stub_0xb448() -> ! {
    todo!("0xb448 __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEED1Ev")
}

// 0xb46c — __ZNK3RBX15CRenderSettings25getDebugShowBoundingBoxesEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getDebugShowBoundingBoxes(void)const")]
pub fn stub_0xb46c() -> ! {
    todo!("0xb46c __ZNK3RBX15CRenderSettings25getDebugShowBoundingBoxesEv")
}

// 0xb478 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::~PropDescriptor()")]
pub fn stub_0xb478() -> ! {
    todo!("0xb478 __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiED1Ev")
}

// 0xb4a8 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::~EnumPropDescriptor() [0xb4a8]")]
pub fn stub_0xb4a8() -> ! {
    todo!("0xb4a8 __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEED1Ev")
}

// 0xb4d0 — __ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::~BoundFuncDesc()")]
pub fn stub_0xb4d0() -> ! {
    todo!("0xb4d0 __ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EED1Ev")
}

// 0xb4fc — __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEEC2Ev
// type: RBX::Instance *__fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEEC2Ev")]
pub fn stub_0xb4fc() -> ! {
    todo!("0xb4fc __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEEC2Ev")
}

// 0xb76c — __ZN3rbx7signals16signal_with_argsILi1EFvPKN3RBX10Reflection18PropertyDescriptorEEEclES6_
// type: void __fastcall(_DWORD *, int, int, const void *, int, int, int, int, void *, int)
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Reflection::PropertyDescriptor const*)>::operator()(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_0xb76c() -> ! {
    todo!("0xb76c __ZN3rbx7signals16signal_with_argsILi1EFvPKN3RBX10Reflection18PropertyDescriptorEEEclES6_")
}

// 0xb8b8 — __ZN19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "CRenderSettingsItem::~CRenderSettingsItem() [0xb8b8]")]
pub fn stub_0xb8b8() -> ! {
    todo!("0xb8b8 __ZN19CRenderSettingsItemD1Ev")
}

// 0xb8bc — __ZN19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "CRenderSettingsItem::~CRenderSettingsItem() [0xb8bc]")]
pub fn stub_0xb8bc() -> ! {
    todo!("0xb8bc __ZN19CRenderSettingsItemD0Ev")
}

// 0xb8d0 — __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv")]
pub fn stub_0xb8d0() -> ! {
    todo!("0xb8d0 __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv")
}

// 0xb8e0 — __ZThn32_N19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
// was: non-virtual thunk to CRenderSettingsItem::~CRenderSettingsItem()
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
pub fn stub_0xb8e0() -> ! {
    todo!("0xb8e0 __ZThn32_N19CRenderSettingsItemD1Ev")
}

// 0xb8e8 — __ZThn32_N19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
// was: non-virtual thunk to CRenderSettingsItem::~CRenderSettingsItem()
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem() [0xb8e8]")]
pub fn stub_0xb8e8() -> ! {
    todo!("0xb8e8 __ZThn32_N19CRenderSettingsItemD0Ev")
}

// 0xb900 — __ZThn32_NK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv")]
pub fn stub_0xb900() -> ! {
    todo!("0xb900 __ZThn32_NK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv")
}

// 0xb910 — __ZThn36_N19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
// was: non-virtual thunk to CRenderSettingsItem::~CRenderSettingsItem()
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem() [0xb910]")]
pub fn stub_0xb910() -> ! {
    todo!("0xb910 __ZThn36_N19CRenderSettingsItemD1Ev")
}

// 0xb918 — __ZThn36_N19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
// was: non-virtual thunk to CRenderSettingsItem::~CRenderSettingsItem()
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem() [0xb918]")]
pub fn stub_0xb918() -> ! {
    todo!("0xb918 __ZThn36_N19CRenderSettingsItemD0Ev")
}

// 0xb930 — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0xb930() -> ! {
    todo!("0xb930 __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD1Ev")
}

// 0xb934 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()")]
pub fn stub_0xb934() -> ! {
    todo!("0xb934 __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED1Ev")
}

// 0xb938 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc() [0xb938]")]
pub fn stub_0xb938() -> ! {
    todo!("0xb938 __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED0Ev")
}

// 0xb94c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::lookup(char const*)const")]
pub fn stub_0xb94c() -> ! {
    todo!("0xb94c __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupEPKc")
}

// 0xb97c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_0xb97c() -> ! {
    todo!("0xb97c __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupERKNS0_7VariantE")
}

// 0xb99c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_0xb99c() -> ! {
    todo!("0xb99c __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueEmRNS0_7VariantE")
}

// 0xb9f8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToString(unsigned long,std::string &)const")]
pub fn stub_0xb9f8() -> ! {
    todo!("0xb9f8 __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE15convertToStringEmRSs")
}

// 0xbb3c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()")]
pub fn stub_0xbb3c() -> ! {
    todo!("0xbb3c __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED1Ev")
}

// 0xbb40 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc() [0xbb40]")]
pub fn stub_0xbb40() -> ! {
    todo!("0xbb40 __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED0Ev")
}

// 0xbb54 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::lookup(char const*)const")]
pub fn stub_0xbb54() -> ! {
    todo!("0xbb54 __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE6lookupEPKc")
}

// 0xbb84 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_0xbb84() -> ! {
    todo!("0xbb84 __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE6lookupERKNS0_7VariantE")
}

// 0xbba4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_0xbba4() -> ! {
    todo!("0xbba4 __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueEmRNS0_7VariantE")
}

// 0xbc00 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToString(unsigned long,std::string &)const")]
pub fn stub_0xbc00() -> ! {
    todo!("0xbc00 __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringEmRSs")
}

// 0xbd44 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::~EnumDesc()")]
pub fn stub_0xbd44() -> ! {
    todo!("0xbd44 __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED1Ev")
}

// 0xbd48 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::~EnumDesc() [0xbd48]")]
pub fn stub_0xbd48() -> ! {
    todo!("0xbd48 __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED0Ev")
}

// 0xbd5c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::lookup(char const*)const")]
pub fn stub_0xbd5c() -> ! {
    todo!("0xbd5c __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupEPKc")
}

// 0xbd8c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_0xbd8c() -> ! {
    todo!("0xbd8c __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupERKNS0_7VariantE")
}

// 0xbdac — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_0xbdac() -> ! {
    todo!("0xbdac __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueEmRNS0_7VariantE")
}

// 0xbe08 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToString(unsigned long,std::string &)const")]
pub fn stub_0xbe08() -> ! {
    todo!("0xbe08 __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE15convertToStringEmRSs")
}

// 0xbf4c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc()")]
pub fn stub_0xbf4c() -> ! {
    todo!("0xbf4c __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED1Ev")
}

// 0xbf50 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc() [0xbf50]")]
pub fn stub_0xbf50() -> ! {
    todo!("0xbf50 __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED0Ev")
}

// 0xbf64 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::lookup(char const*)const")]
pub fn stub_0xbf64() -> ! {
    todo!("0xbf64 __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE6lookupEPKc")
}

// 0xbf94 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_0xbf94() -> ! {
    todo!("0xbf94 __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE6lookupERKNS0_7VariantE")
}

// 0xbfb4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_0xbfb4() -> ! {
    todo!("0xbfb4 __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToValueEmRNS0_7VariantE")
}

// 0xc010 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToString(unsigned long,std::string &)const")]
pub fn stub_0xc010() -> ! {
    todo!("0xc010 __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE15convertToStringEmRSs")
}

// 0xc154 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::~EnumDesc()")]
pub fn stub_0xc154() -> ! {
    todo!("0xc154 __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED1Ev")
}

// 0xc158 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::~EnumDesc() [0xc158]")]
pub fn stub_0xc158() -> ! {
    todo!("0xc158 __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED0Ev")
}

// 0xc16c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::lookup(char const*)const")]
pub fn stub_0xc16c() -> ! {
    todo!("0xc16c __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE6lookupEPKc")
}

// 0xc19c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_0xc19c() -> ! {
    todo!("0xc19c __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE6lookupERKNS0_7VariantE")
}

// 0xc1bc — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_0xc1bc() -> ! {
    todo!("0xc1bc __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToValueEmRNS0_7VariantE")
}

// 0xc218 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToString(unsigned long,std::string &)const")]
pub fn stub_0xc218() -> ! {
    todo!("0xc218 __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE15convertToStringEmRSs")
}

// 0xc35c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::~EnumDesc()")]
pub fn stub_0xc35c() -> ! {
    todo!("0xc35c __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED1Ev")
}

// 0xc360 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::~EnumDesc() [0xc360]")]
pub fn stub_0xc360() -> ! {
    todo!("0xc360 __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED0Ev")
}

// 0xc374 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::lookup(char const*)const")]
pub fn stub_0xc374() -> ! {
    todo!("0xc374 __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE6lookupEPKc")
}

// 0xc3a4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_0xc3a4() -> ! {
    todo!("0xc3a4 __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE6lookupERKNS0_7VariantE")
}

// 0xc3c4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_0xc3c4() -> ! {
    todo!("0xc3c4 __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToValueEmRNS0_7VariantE")
}

// 0xc420 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToString(unsigned long,std::string &)const")]
pub fn stub_0xc420() -> ! {
    todo!("0xc420 __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE15convertToStringEmRSs")
}
