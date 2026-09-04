//! platform generated_171 — next 100 stubs EA-sorted asc global filler continuation
//! Filter: platform/iOS/Apple strict (0 remaining — ObjC 2763 done, RobloxView 160 done, iOSSettingsService 54 done, RBX::Platform 0) + global EA-sorted asc filler (rbx_core::SharedPtr not boost)
//! Batch: 100 stubs EA-sorted asc | skeleton batch | range 0x84e0..0xc010 (rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use rbx_core::signal::Signal;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

/// Property-descriptor identities passed to the `propertyChanged` signal at
/// `CRenderSettingsItem + 192` (IDA `this + 0xC0`,
/// `rbx::signals::signal_with_args<1, void(const PropertyDescriptor*)>`).
/// The original passes `&unk_130Cxxx` globals (or
/// `CRenderSettingsItem::prop_resolution` at 0x12d2c78); the host model
/// passes the descriptor address. `Signal` is `rbx_core::signal::Signal`,
/// never `boost::signals`.
pub const PROP_ALWAYS_DRAW_CONNECTORS: u32 = 0x130C030;
pub const PROP_SHOW_AGGREGATION: u32 = 0x130C05C;
pub const PROP_DEBUG_SHOW_BOUNDING_BOXES: u32 = 0x130C0E0;
pub const PROP_ENABLE_FRM: u32 = 0x130C138;
pub const PROP_EAGER_BULK_EXECUTION: u32 = 0x130C1E8;
pub const PROP_GRAPHICS_MODE: u32 = 0x130C244;
pub const PROP_FRAME_RATE_MANAGER_MODE: u32 = 0x130C278;
pub const PROP_QUALITY_LEVEL: u32 = 0x130C2AC;
pub const PROP_AA_SAMPLES: u32 = 0x130C2E0;
pub const PROP_SHADOW_MODE: u32 = 0x130C314;
pub const PROP_ANTIALIASING_MODE: u32 = 0x130C348;
pub const PROP_RESOLUTION: u32 = 0x12D2C78;

/// `RBX::CRenderSettings::AASamples` values (IDA 0x850c pairs).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum AaSamples {
    /// IDA "None".
    None_ = 1,
    /// IDA "4".
    X4 = 4,
    /// IDA "8".
    X8 = 8,
}

/// `RBX::CRenderSettings::GraphicsMode` values (IDA 0x86d0 pairs).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GraphicsMode {
    Automatic = 1,
    Direct3D = 3,
    OpenGL = 4,
    NoGraphics = 5,
}

/// `RBX::CRenderSettings::FrameRateManagerMode` values (IDA 0x88c4 pairs).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum FrameRateManagerMode {
    Automatic = 0,
    On = 1,
    Off = 2,
}

/// `RBX::CRenderSettings::AntialiasingMode` values (IDA 0x8a88 pairs).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum AntialiasingMode {
    Automatic = 0,
    On = 1,
    Off = 2,
}

/// `RBX::CRenderSettings::ShadowMode` values (IDA 0x8c4c pairs).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ShadowMode {
    Automatic = 0,
    All = 1,
    Off = 2,
    CharacterOnly = 3,
}

/// `RBX::CRenderSettings::QualityLevel` values: `Automatic` plus
/// `Level01`..`Level21` (IDA 0x8e24 loop `for (i = 1; i < 22; ++i)`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum QualityLevel {
    Automatic = 0,
    Level01 = 1,
    Level02 = 2,
    Level03 = 3,
    Level04 = 4,
    Level05 = 5,
    Level06 = 6,
    Level07 = 7,
    Level08 = 8,
    Level09 = 9,
    Level10 = 10,
    Level11 = 11,
    Level12 = 12,
    Level13 = 13,
    Level14 = 14,
    Level15 = 15,
    Level16 = 16,
    Level17 = 17,
    Level18 = 18,
    Level19 = 19,
    Level20 = 20,
    Level21 = 21,
}

/// `RBX::CRenderSettings::ResolutionPreset` values (IDA 0x9100 pairs).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ResolutionPreset {
    Automatic = 0,
    R720x526 = 1,
    R800x600 = 2,
    R1024x600 = 3,
    R1024x768 = 4,
    R1280x720 = 5,
    R1280x768 = 6,
    R1152x864 = 7,
    R1280x800 = 8,
    R1360x768 = 9,
    R1280x960 = 10,
    R1280x1024 = 11,
    R1440x900 = 12,
    R1600x900 = 13,
    R1600x1024 = 14,
    R1600x1200 = 15,
    R1680x1050 = 16,
    R1920x1080 = 17,
    R1920x1200 = 18,
}

/// One `EnumDesc::addPair` entry (IDA `addPair(desc, value, name)`).
#[derive(Debug, Clone)]
pub struct RenderEnumPair {
    pub value: i32,
    pub name: String,
}

/// One legacy-name entry: `addLegacy` (IDA 0x880e) or the extra
/// `Name::declare` + name-to-value map inserts (IDA 0x8f9e..0x8ff2,
/// 0x923e..0x9534). `maps_to` is the value the legacy name resolves to.
#[derive(Debug, Clone)]
pub struct RenderEnumAlias {
    pub value: i32,
    pub name: String,
    pub maps_to: i32,
}

/// Host model of `RBX::Reflection::EnumDesc<T>`: the enum name plus the
/// pair/alias table built 1:1 from each ctor's `addPair`/`addLegacy` call
/// sequence. The vtable store and inline `std::map` node inits collapse into
/// the two vecs; the `EnumDescriptor` base name/typeinfo have no host
/// equivalent beyond `enum_name`.
#[derive(Debug, Clone, Default)]
pub struct RenderEnumDesc {
    pub enum_name: &'static str,
    pub pairs: Vec<RenderEnumPair>,
    pub legacy_aliases: Vec<RenderEnumAlias>,
}

impl RenderEnumDesc {
    pub fn new(enum_name: &'static str) -> Self {
        Self { enum_name, pairs: Vec::new(), legacy_aliases: Vec::new() }
    }
    pub fn add_pair(&mut self, value: i32, name: &str) {
        self.pairs.push(RenderEnumPair { value, name: name.to_string() });
    }
    pub fn add_legacy_alias(&mut self, value: i32, name: &str, maps_to: i32) {
        self.legacy_aliases.push(RenderEnumAlias { value, name: name.to_string(), maps_to });
    }
}

/// `RBX::CRenderSettings::aaSamples` process global written by
/// `setAASamples` (IDA 0x96d0..0x96ee). BSS zero-init on the original.
static AA_SAMPLES: AtomicI32 = AtomicI32::new(0);

/// `RBX::PartInstance::disableInterpolation` process global read/written by
/// `get/setDebugDisableInterpolation` (IDA 0x9784..0x97a2). BSS zero-init.
static DISABLE_INTERPOLATION: AtomicBool = AtomicBool::new(false);

/// Host model of `CRenderSettingsItem` covering every cell the 0x9608..0x9b08
/// setters touch. Offsets are `this +` byte offsets from the IDA
/// decompilations/disassemblies; the +96..+124 enum cells live in the
/// `RBX::CRenderSettings` subobject constructed at `this + 96`
/// (IDA 0x9828, out of slice). The vtables (+0/+12/+32/+36/+96) and the
/// `call_once` signal init (IDA 0x98d0..0x98d8) have no host equivalent.
pub struct RenderSettingsItem {
    pub graphics_mode: i32,            // +100 (0x64)
    pub antialiasing_mode: i32,       // +104 (0x68)
    pub shadow_mode: i32,             // +108 (0x6C)
    pub frame_rate_manager_mode: i32, // +112 (0x70)
    pub quality_level: i32,           // +116 (0x74)
    pub resolution_preference: i32,   // +120 (0x78)
    pub auto_quality_level: i32,      // +124 (0x7C)
    pub max_quality_level: i32,       // +128 (0x80)
    pub debug_show_bounding_boxes: bool, // +136 (0x88)
    pub enable_frm: bool,             // +137 (0x89)
    pub show_aggregation: bool,       // +154 (0x9A)
    pub always_draw_connectors: bool, // +155 (0x9B)
    pub connector_draw: bool,         // +156 (0x9C): coupled flag read by 0x9668, original name unknown
    pub eager_bulk_execution: bool,   // +157 (0x9D)
    pub texture_cache_size: u32,      // +160 (0xA0)
    pub mesh_cache_size: u32,         // +164 (0xA4)
    pub instance_name: String,        // "Rendering" via vtable[7] call (IDA 0x98ec..0x9904)
    pub aux_string_168: String,       // +168 (0xA8) empty std::string (IDA 0x9876)
    pub startup_resolution: (u16, u16), // +172/+174 = 800/600 (IDA 0x987e/0x988a)
    pub supported_resolutions: Vec<(u16, u16)>, // +176 (0xB0) vector, seeded {(800,600)} (IDA 0x991a)
    pub flag_189: bool,               // +189 (0xBD) = 1 (IDA 0x98b0), purpose unknown
    pub video_memory_class: u32,      // +146 (0x92): VRAM-gated cache class (IDA 0x9946)
    pub property_changed: Signal<u32>, // +192 (0xC0)
}

impl Default for RenderSettingsItem {
    fn default() -> Self {
        Self {
            graphics_mode: 0,
            antialiasing_mode: 0,
            shadow_mode: 0,
            frame_rate_manager_mode: 0,
            quality_level: 0,
            resolution_preference: 0,
            auto_quality_level: 0,
            max_quality_level: 0,
            debug_show_bounding_boxes: false,
            enable_frm: false,
            show_aggregation: false,
            always_draw_connectors: false,
            connector_draw: false,
            eager_bulk_execution: false,
            texture_cache_size: 0,
            mesh_cache_size: 0,
            instance_name: String::new(),
            aux_string_168: String::new(),
            startup_resolution: (0, 0),
            supported_resolutions: Vec::new(),
            flag_189: false,
            video_memory_class: 0,
            property_changed: Signal::new(),
        }
    }
}

impl std::fmt::Debug for RenderSettingsItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RenderSettingsItem")
            .field("graphics_mode", &self.graphics_mode)
            .field("antialiasing_mode", &self.antialiasing_mode)
            .field("shadow_mode", &self.shadow_mode)
            .field("frame_rate_manager_mode", &self.frame_rate_manager_mode)
            .field("quality_level", &self.quality_level)
            .field("resolution_preference", &self.resolution_preference)
            .field("max_quality_level", &self.max_quality_level)
            .field("auto_quality_level", &self.auto_quality_level)
            .field("debug_show_bounding_boxes", &self.debug_show_bounding_boxes)
            .field("enable_frm", &self.enable_frm)
            .field("show_aggregation", &self.show_aggregation)
            .field("always_draw_connectors", &self.always_draw_connectors)
            .field("connector_draw", &self.connector_draw)
            .field("eager_bulk_execution", &self.eager_bulk_execution)
            .field("texture_cache_size", &self.texture_cache_size)
            .field("mesh_cache_size", &self.mesh_cache_size)
            .field("instance_name", &self.instance_name)
            .field("aux_string_168", &self.aux_string_168)
            .field("startup_resolution", &self.startup_resolution)
            .field("supported_resolutions", &self.supported_resolutions)
            .field("flag_189", &self.flag_189)
            .field("video_memory_class", &self.video_memory_class)
            .field("property_changed", &"<signal>")
            .finish()
    }
}
/// Host model of `std::length_error` / `std::logic_error`: the only state is
/// the `what()` message (`std::string` in the base subobject). Thrown across
/// the render-settings enum tables when a value fails its range assert.
#[derive(Debug, Clone, Default)]
pub struct StdLengthError {
    pub message: String,
}

/// Host model of `std::out_of_range`: same layout as `StdLengthError`, kept
/// distinct so `rg` finds either C++ form.
#[derive(Debug, Clone, Default)]
pub struct StdOutOfRange {
    pub message: String,
}

/// Host model of `RBX::Reflection::PropDescriptor<T>` and
/// `EnumPropDescriptor<T, E>`: the vtable, name, getter/setter closures and
/// inline `std::map` nodes collapse into `prop_name`; the heap-owned default
/// (`a1[10]` for `PropDescriptor`, `a1[11]` for `EnumPropDescriptor`) is
/// `extra`. The destructor bodies below only reset the vtable (host nop) and
/// conditionally delete that member (`Option::take` + drop).
#[derive(Debug, Clone, Default)]
pub struct RenderPropDescriptor {
    pub prop_name: String,
    pub extra: Option<Box<[u8]>>,
}

// 0x84e0 — start
// mangled: start
// type: void __fastcall __noreturn(int, int, int, int, int argc, char *argv)
#[doc(alias = "start")]
pub fn stub_84e0(
    main: fn(i32, Vec<String>, Vec<String>) -> i32,
    argv: Vec<String>,
    envp: Vec<String>,
) -> ! {
    // IDA 0x84e0..0x84f4: R0 = argc, R1 = argv, R2 = &argv[argc + 1] (envp).
    // 0x84f0 `BIC SP, SP, #7` 16-byte stack alignment is a host nop.
    // IDA 0x84f8..0x8500: `while (*v6++) ;` walks envp to its NULL
    // terminator with no observable effect, so the host takes envp as given.
    let code = main(argv.len() as i32, argv, envp);
    // IDA 0x8504 `BLX _main`, 0x8508 `B _exit`.
    std::process::exit(code);
}

// 0x850c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEEC2Ev
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::EnumDesc(void)")]
pub fn stub_850c() -> RenderEnumDesc {
    // IDA 0x8542: `EnumDescriptor` base ("AASamples", typeinfo AASamples).
    // 0x855a..0x85c0: vtable + inline map inits (host: empty vecs).
    let mut desc = RenderEnumDesc::new("AASamples");
    // IDA 0x85f0..0x861c.
    desc.add_pair(1, "None");
    desc.add_pair(4, "4");
    desc.add_pair(8, "8");
    // IDA 0x863e: return a1.
    desc
}

// 0x86d0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEEC2Ev
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::EnumDesc(void)")]
pub fn stub_86d0() -> RenderEnumDesc {
    // IDA 0x8706: `EnumDescriptor` base ("GraphicsMode", typeinfo GraphicsMode).
    // 0x871e..0x8784: vtable + inline map inits (host: empty vecs).
    let mut desc = RenderEnumDesc::new("GraphicsMode");
    // IDA 0x87b4..0x87f6.
    desc.add_pair(1, "Automatic");
    desc.add_pair(3, "Direct3D");
    desc.add_pair(4, "OpenGL");
    desc.add_pair(5, "NoGraphics");
    // IDA 0x880e: `addLegacy(a1, 2, "OpenGL legacy", 1)`.
    desc.add_legacy_alias(2, "OpenGL legacy", 1);
    // IDA 0x8830: return a1.
    desc
}

// 0x88c4 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEC2Ev
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::EnumDesc(void)")]
pub fn stub_88c4() -> RenderEnumDesc {
    // IDA 0x88fa: `EnumDescriptor` base ("FramerateManagerMode" — IDA spelling).
    // 0x8912..0x8978: vtable + inline map inits (host: empty vecs).
    let mut desc = RenderEnumDesc::new("FramerateManagerMode");
    // IDA 0x89a8..0x89d4.
    desc.add_pair(0, "Automatic");
    desc.add_pair(1, "On");
    desc.add_pair(2, "Off");
    // IDA 0x89f6: return a1.
    desc
}

// 0x8a88 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEEC2Ev
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::EnumDesc(void)")]
pub fn stub_8a88() -> RenderEnumDesc {
    // IDA 0x8abe: `EnumDescriptor` base ("Antialiasing", typeinfo AntialiasingMode).
    // 0x8ad6..0x8b3c: vtable + inline map inits (host: empty vecs).
    let mut desc = RenderEnumDesc::new("Antialiasing");
    // IDA 0x8b6c..0x8b98 (Off precedes On in the original sequence).
    desc.add_pair(0, "Automatic");
    desc.add_pair(2, "Off");
    desc.add_pair(1, "On");
    // IDA 0x8bba: return a1.
    desc
}

// 0x8c4c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEEC2Ev
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::EnumDesc(void)")]
pub fn stub_8c4c() -> RenderEnumDesc {
    // IDA 0x8c82: `EnumDescriptor` base ("Shadow", typeinfo ShadowMode).
    // 0x8c9a..0x8d00: vtable + inline map inits (host: empty vecs).
    let mut desc = RenderEnumDesc::new("Shadow");
    // IDA 0x8d30..0x8d72.
    desc.add_pair(0, "Automatic");
    desc.add_pair(1, "All");
    desc.add_pair(3, "CharacterOnly");
    desc.add_pair(2, "Off");
    // IDA 0x8d94: return a1.
    desc
}

// 0x8e24 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEEC2Ev
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEEC2Ev
// type: RBX::Reflection::EnumDescriptor *__fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::EnumDesc(void)")]
pub fn stub_8e24() -> RenderEnumDesc {
    // IDA 0x8e6c: `EnumDescriptor` base ("QualityLevel", typeinfo QualityLevel).
    // 0x8e84..0x8ef0: vtable + inline map inits (host: empty vecs).
    let mut desc = RenderEnumDesc::new("QualityLevel");
    // IDA 0x8f20.
    desc.add_pair(0, "Automatic");
    // IDA 0x8f28..0x8f92: `for (i = 1; i < 22; ++i) { RBX::format(&t, "Level%.2d", i); addPair(v25, i, t); ... }`.
    for i in 1..22 {
        desc.add_pair(i, &format!("Level{i:02}"));
    }
    // IDA 0x8f9e..0x8ff2: legacy `Name::declare` + name-to-value map fill.
    // `strcpy(v29, "Level 00"); snprintf(&v29[6], 3, "%2u", v)` pads with a
    // leading space, so single-digit levels keep two spaces ("Level  1").
    for i in 1..22 {
        desc.add_legacy_alias(i, &format!("Level {i:2}"), i);
    }
    // IDA 0x9020: return v25.
    desc
}

// 0x9100 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEEC2Ev
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEEC2Ev
// type: RBX::Reflection::EnumDescriptor *__fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::EnumDesc(void)")]
pub fn stub_9100() -> RenderEnumDesc {
    // IDA 0x9136: `EnumDescriptor` base ("Resolution", typeinfo ResolutionPreset).
    // 0x914e..0x91bc: vtable + inline map inits (host: empty vecs).
    let mut desc = RenderEnumDesc::new("Resolution");
    // IDA 0x91ee..0x9534: addPair calls interleaved with "(wide)" alias
    // `Name::declare` + name-to-value map inserts, in original order.
    desc.add_pair(0, "Automatic");
    desc.add_pair(1, "720x526");
    desc.add_pair(2, "800x600");
    desc.add_pair(3, "1024x600");
    desc.add_legacy_alias(3, "1024x600 (wide)", 3);
    desc.add_pair(4, "1024x768");
    desc.add_pair(5, "1280x720");
    desc.add_legacy_alias(5, "1280x720 (wide)", 5);
    desc.add_pair(6, "1280x768");
    desc.add_legacy_alias(6, "1280x768 (wide)", 6);
    desc.add_pair(7, "1152x864");
    desc.add_pair(8, "1280x800");
    desc.add_legacy_alias(8, "1280x800 (wide)", 8);
    desc.add_pair(9, "1360x768");
    desc.add_legacy_alias(9, "1360x768 (wide)", 9);
    desc.add_pair(10, "1280x960");
    desc.add_pair(11, "1280x1024");
    desc.add_pair(12, "1440x900");
    desc.add_legacy_alias(12, "1440x900 (wide)", 12);
    desc.add_pair(13, "1600x900");
    desc.add_legacy_alias(13, "1600x900 (wide)", 13);
    desc.add_pair(14, "1600x1024");
    desc.add_legacy_alias(14, "1600x1024 (wide)", 14);
    desc.add_pair(15, "1600x1200");
    desc.add_pair(16, "1680x1050");
    desc.add_legacy_alias(16, "1680x1050 (wide)", 16);
    desc.add_pair(17, "1920x1080");
    desc.add_legacy_alias(17, "1920x1080 (wide)", 17);
    desc.add_pair(18, "1920x1200");
    desc.add_legacy_alias(18, "1920x1200 (wide)", 18);
    // IDA 0x9554: return v49.
    desc
}

// 0x9608 — __ZN19CRenderSettingsItem15setGraphicsModeEN3RBX15CRenderSettings12GraphicsModeE
// mangled: __ZN19CRenderSettingsItem15setGraphicsModeEN3RBX15CRenderSettings12GraphicsModeE
// type: int __fastcall(int result, int)
#[doc(alias = "CRenderSettingsItem::setGraphicsMode(RBX::CRenderSettings::GraphicsMode)")]
pub fn stub_9608(item: &mut RenderSettingsItem, mode: GraphicsMode) -> bool {
    // IDA 0x9608..0x960e: `LDR R2,[R0,#0x64]; CMP R2,R1; BXEQ LR` — unchanged.
    // The original returns `this`/signal int; the host reports changed.
    if item.graphics_mode == mode as i32 {
        return false;
    }
    // IDA 0x9618: `STR R1,[R0,#0x64]`.
    item.graphics_mode = mode as i32;
    // IDA 0x961c..0x9622: signal(this + 0xC0, &unk_130C244).
    item.property_changed.fire(PROP_GRAPHICS_MODE);
    true
}

// 0x9628 — __ZN19CRenderSettingsItem23setFrameRateManagerModeEN3RBX15CRenderSettings20FrameRateManagerModeE
// mangled: __ZN19CRenderSettingsItem23setFrameRateManagerModeEN3RBX15CRenderSettings20FrameRateManagerModeE
// type: int __fastcall(int result, int)
#[doc(alias = "CRenderSettingsItem::setFrameRateManagerMode(RBX::CRenderSettings::FrameRateManagerMode)")]
pub fn stub_9628(item: &mut RenderSettingsItem, mode: FrameRateManagerMode) -> bool {
    // IDA 0x9628..0x962e: `LDR R2,[R0,#0x70]; CMP R2,R1; BXEQ LR` — unchanged.
    if item.frame_rate_manager_mode == mode as i32 {
        return false;
    }
    // IDA 0x9638: `STR R1,[R0,#0x70]`.
    item.frame_rate_manager_mode = mode as i32;
    // IDA 0x963c..0x9642: signal(this + 0xC0, &unk_130C278).
    item.property_changed.fire(PROP_FRAME_RATE_MANAGER_MODE);
    true
}

// 0x9648 — __ZN19CRenderSettingsItem15setQualityLevelEN3RBX15CRenderSettings12QualityLevelE
// mangled: __ZN19CRenderSettingsItem15setQualityLevelEN3RBX15CRenderSettings12QualityLevelE
// type: int __fastcall(int result, int)
#[doc(alias = "CRenderSettingsItem::setQualityLevel(RBX::CRenderSettings::QualityLevel)")]
pub fn stub_9648(item: &mut RenderSettingsItem, level: QualityLevel) -> bool {
    // IDA 0x9648..0x964e: `LDR R2,[R0,#0x74]; CMP R2,R1; BXEQ LR` — unchanged.
    if item.quality_level == level as i32 {
        return false;
    }
    // IDA 0x9658: `STR R1,[R0,#0x74]`.
    item.quality_level = level as i32;
    // IDA 0x965c..0x9662: signal(this + 0xC0, &unk_130C2AC).
    item.property_changed.fire(PROP_QUALITY_LEVEL);
    true
}

// 0x9668 — __ZN19CRenderSettingsItem23setAlwaysDrawConnectorsEb
// mangled: __ZN19CRenderSettingsItem23setAlwaysDrawConnectorsEb
// type: int __fastcall(int this, int)
#[doc(alias = "CRenderSettingsItem::setAlwaysDrawConnectors(bool)")]
pub fn stub_9668(item: &mut RenderSettingsItem, value: bool) -> bool {
    // IDA 0x9668..0x9678: v2 = +155 (+0x9B) ? 1 : normalize(+156, +0x9C).
    let old_effective = if item.always_draw_connectors { true } else { item.connector_draw };
    // IDA 0x967a: `STRB R1,[R0,#0x9B]` — store happens before the branch.
    item.always_draw_connectors = value;
    if value {
        // IDA 0x9680,0x9694..0x9698: a2 == 1 → return unless v2 was clear.
        if old_effective {
            return false;
        }
    } else {
        // IDA 0x9682..0x9692: a2 != 1 → fire iff v2 != normalize(+156).
        if old_effective == item.connector_draw {
            return false;
        }
    }
    // IDA 0x969a..0x96a8: signal(this + 0xC0, &unk_130C030).
    item.property_changed.fire(PROP_ALWAYS_DRAW_CONNECTORS);
    true
}

// 0x96ac — __ZN19CRenderSettingsItem18setShowAggregationEb
// mangled: __ZN19CRenderSettingsItem18setShowAggregationEb
// type: int __fastcall(int this, int)
#[doc(alias = "CRenderSettingsItem::setShowAggregation(bool)")]
pub fn stub_96ac(item: &mut RenderSettingsItem, value: bool) -> bool {
    // IDA 0x96ac..0x96b4: `LDRB R2,[R0,#0x9A]; CMP R1,R2; BXEQ LR` — unchanged.
    if item.show_aggregation == value {
        return false;
    }
    // IDA 0x96be: `STRB R1,[R0,#0x9A]`.
    item.show_aggregation = value;
    // IDA 0x96c2..0x96ca: signal(this + 0xC0, &unk_130C05C).
    item.property_changed.fire(PROP_SHOW_AGGREGATION);
    true
}

// 0x96d0 — __ZN19CRenderSettingsItem12setAASamplesEN3RBX15CRenderSettings9AASamplesE
// mangled: __ZN19CRenderSettingsItem12setAASamplesEN3RBX15CRenderSettings9AASamplesE
// type: int __fastcall(int result, int)
#[doc(alias = "CRenderSettingsItem::setAASamples(RBX::CRenderSettings::AASamples)")]
pub fn stub_96d0(item: &mut RenderSettingsItem, samples: AaSamples) -> bool {
    // IDA 0x96d0..0x96e2: compare the `RBX::CRenderSettings::aaSamples`
    // process global (double-indirect load); unchanged → return.
    if AA_SAMPLES.load(Ordering::SeqCst) == samples as i32 {
        return false;
    }
    // IDA 0x96ee: global = a2.
    AA_SAMPLES.store(samples as i32, Ordering::SeqCst);
    // IDA 0x96e8..0x96f6: signal(result + 0xC0, &unk_130C2E0).
    item.property_changed.fire(PROP_AA_SAMPLES);
    true
}

// 0x96fc — __ZN19CRenderSettingsItem13setShadowModeEN3RBX15CRenderSettings10ShadowModeE
// mangled: __ZN19CRenderSettingsItem13setShadowModeEN3RBX15CRenderSettings10ShadowModeE
// type: int __fastcall(int result, int)
#[doc(alias = "CRenderSettingsItem::setShadowMode(RBX::CRenderSettings::ShadowMode)")]
pub fn stub_96fc(item: &mut RenderSettingsItem, mode: ShadowMode) -> bool {
    // IDA 0x96fc..0x9702: `LDR R2,[R0,#0x6C]; CMP R2,R1; BXEQ LR` — unchanged.
    if item.shadow_mode == mode as i32 {
        return false;
    }
    // IDA 0x970c: `STR R1,[R0,#0x6C]`.
    item.shadow_mode = mode as i32;
    // IDA 0x9710..0x9716: signal(this + 0xC0, &unk_130C314).
    item.property_changed.fire(PROP_SHADOW_MODE);
    true
}

// 0x971c — __ZN19CRenderSettingsItem19setAntialiasingModeEN3RBX15CRenderSettings16AntialiasingModeE
// mangled: __ZN19CRenderSettingsItem19setAntialiasingModeEN3RBX15CRenderSettings16AntialiasingModeE
// type: int __fastcall(int result, int)
#[doc(alias = "CRenderSettingsItem::setAntialiasingMode(RBX::CRenderSettings::AntialiasingMode)")]
pub fn stub_971c(item: &mut RenderSettingsItem, mode: AntialiasingMode) -> bool {
    // IDA 0x971c..0x9722: `LDR R2,[R0,#0x68]; CMP R2,R1; BXEQ LR` — unchanged.
    if item.antialiasing_mode == mode as i32 {
        return false;
    }
    // IDA 0x972c: `STR R1,[R0,#0x68]`.
    item.antialiasing_mode = mode as i32;
    // IDA 0x9730..0x9736: signal(this + 0xC0, &unk_130C348).
    item.property_changed.fire(PROP_ANTIALIASING_MODE);
    true
}

// 0x973c — __ZN19CRenderSettingsItem25setDebugShowBoundingBoxesEb
// mangled: __ZN19CRenderSettingsItem25setDebugShowBoundingBoxesEb
// type: int __fastcall(int this, int)
#[doc(alias = "CRenderSettingsItem::setDebugShowBoundingBoxes(bool)")]
pub fn stub_973c(item: &mut RenderSettingsItem, value: bool) -> bool {
    // IDA 0x973c..0x9744: `LDRB R2,[R0,#0x88]; CMP R1,R2; BXEQ LR` — unchanged.
    if item.debug_show_bounding_boxes == value {
        return false;
    }
    // IDA 0x974e: `STRB R1,[R0,#0x88]`.
    item.debug_show_bounding_boxes = value;
    // IDA 0x9752..0x975a: signal(this + 0xC0, &unk_130C0E0).
    item.property_changed.fire(PROP_DEBUG_SHOW_BOUNDING_BOXES);
    true
}

// 0x9760 — __ZN19CRenderSettingsItem12setEnableFRMEb
// mangled: __ZN19CRenderSettingsItem12setEnableFRMEb
// type: int __fastcall(int this, int)
#[doc(alias = "CRenderSettingsItem::setEnableFRM(bool)")]
pub fn stub_9760(item: &mut RenderSettingsItem, value: bool) -> bool {
    // IDA 0x9760..0x9768: `LDRB R2,[R0,#0x89]; CMP R1,R2; BXEQ LR` — unchanged.
    if item.enable_frm == value {
        return false;
    }
    // IDA 0x9772: `STRB R1,[R0,#0x89]`.
    item.enable_frm = value;
    // IDA 0x9776..0x977e: signal(this + 0xC0, &unk_130C138).
    item.property_changed.fire(PROP_ENABLE_FRM);
    true
}

// 0x9784 — __ZNK19CRenderSettingsItem28getDebugDisableInterpolationEv
// mangled: __ZNK19CRenderSettingsItem28getDebugDisableInterpolationEv
// type: int __fastcall(CRenderSettingsItem *this)
#[doc(alias = "CRenderSettingsItem::getDebugDisableInterpolation(void)const")]
pub fn stub_9784() -> bool {
    // IDA 0x9784..0x9792: ignores `this`; returns the
    // `RBX::PartInstance::disableInterpolation` process global (LDRB).
    DISABLE_INTERPOLATION.load(Ordering::SeqCst)
}

// 0x9794 — __ZN19CRenderSettingsItem28setDebugDisableInterpolationEb
// mangled: __ZN19CRenderSettingsItem28setDebugDisableInterpolationEb
// type: char *__fastcall(CRenderSettingsItem *this, char)
#[doc(alias = "CRenderSettingsItem::setDebugDisableInterpolation(bool)")]
pub fn stub_9794(value: bool) -> bool {
    // IDA 0x9794..0x97a0: `STRB R1,[R0]` into disableInterpolation; ignores
    // `this`. The original returns `&global`; the host returns the value.
    DISABLE_INTERPOLATION.store(value, Ordering::SeqCst);
    // IDA 0x97a2: return &RBX::PartInstance::disableInterpolation.
    DISABLE_INTERPOLATION.load(Ordering::SeqCst)
}

// 0x97a4 — __ZN19CRenderSettingsItem23setResolutionPreferenceEN3RBX15CRenderSettings16ResolutionPresetE
// mangled: __ZN19CRenderSettingsItem23setResolutionPreferenceEN3RBX15CRenderSettings16ResolutionPresetE
// type: int __fastcall(int result, int)
#[doc(alias = "CRenderSettingsItem::setResolutionPreference(RBX::CRenderSettings::ResolutionPreset)")]
pub fn stub_97a4(item: &mut RenderSettingsItem, preset: ResolutionPreset) -> bool {
    // IDA 0x97a4..0x97aa: `LDR R2,[R0,#0x78]; CMP R2,R1; BXEQ LR` — unchanged.
    if item.resolution_preference == preset as i32 {
        return false;
    }
    // IDA 0x97b4: `STR R1,[R0,#0x78]`.
    item.resolution_preference = preset as i32;
    // IDA 0x97b6..0x97bc: signal(this + 0xC0, &CRenderSettingsItem::prop_resolution @ 0x12d2c78).
    item.property_changed.fire(PROP_RESOLUTION);
    true
}

// 0x97c0 — __ZN19CRenderSettingsItem19setTextureCacheSizeEj
// mangled: __ZN19CRenderSettingsItem19setTextureCacheSizeEj
// type: int __fastcall(int this, unsigned int)
#[doc(alias = "CRenderSettingsItem::setTextureCacheSize(unsigned int)")]
pub fn stub_97c0(item: &mut RenderSettingsItem, size: u32) {
    // IDA 0x97c0: `STR R1,[R0,#0xA0]` — unconditional, no signal.
    // 0x97c4: return this.
    item.texture_cache_size = size;
}

// 0x97c8 — __ZN19CRenderSettingsItem16setMeshCacheSizeEj
// mangled: __ZN19CRenderSettingsItem16setMeshCacheSizeEj
// type: int __fastcall(int this, unsigned int)
#[doc(alias = "CRenderSettingsItem::setMeshCacheSize(unsigned int)")]
pub fn stub_97c8(item: &mut RenderSettingsItem, size: u32) {
    // IDA 0x97c8: `STR R1,[R0,#0xA4]` — unconditional, no signal.
    // 0x97cc: return this.
    item.mesh_cache_size = size;
}

// 0x97d0 — __ZN19CRenderSettingsItemC2Ev
// mangled: __ZN19CRenderSettingsItemC2Ev
// type: void __fastcall(CRenderSettingsItem *this)
#[doc(alias = "CRenderSettingsItem::CRenderSettingsItem(void)")]
pub fn stub_97d0(dx_video_memory_size: u32) -> RenderSettingsItem {
    // IDA 0x97f0: `GlobalAdvancedSettingsItem` base ctor (out of slice).
    // IDA 0x9828: `CRenderSettings::CRenderSettings(this + 96)` fills the
    // +96..+167 enum cells (defaults out of slice; host starts at zero).
    let mut item = RenderSettingsItem::default();
    // IDA 0x983c..0x985c: vtable stores (no host equivalent).
    // IDA 0x986c..0x98aa: +168 empty string, +176 empty vector (host: Default).
    // IDA 0x987e/0x988a: `*(u16 *)(this + 172) = 800`, `*(u16 *)(this + 174) = 600`.
    item.startup_resolution = (800, 600);
    // IDA 0x98b0: `*(u8 *)(this + 189) = 1`.
    item.flag_189 = true;
    // IDA 0x98ca..0x98d8: +192 property-changed signal init (host: Signal::new).
    // IDA 0x98ec..0x9904: vtable[7](this, "Rendering") → instance name.
    item.instance_name = "Rendering".to_string();
    // IDA 0x991a: `push_back(+176, *(Vector2int16 *)(this + 172))` — the
    // packed int at +172 is 800 | (600 << 16), i.e. (800, 600).
    item.supported_resolutions.push((800, 600));
    // IDA 0x9922..0x9946: `GetDXVideoMemorySize() > 0xF423FF ? 0x3000400
    // (50332672) : 0x2580320 (39322400)` stored at +146. The DX call has no
    // host equivalent, so the byte count is a parameter.
    item.video_memory_class = if dx_video_memory_size > 0xF423FF { 0x3000400 } else { 0x2580320 };
    item
}

// 0x9ac8 — __ZN19CRenderSettingsItem19setAutoQualityLevelEi
// mangled: __ZN19CRenderSettingsItem19setAutoQualityLevelEi
// type: int __fastcall(int this, int)
#[doc(alias = "CRenderSettingsItem::setAutoQualityLevel(int)")]
pub fn stub_9ac8(item: &mut RenderSettingsItem, level: i32) -> bool {
    // IDA 0x9ac8..0x9ace: `LDR R2,[R0,#0x7C]; CMP R2,R1; BXEQ LR` — unchanged.
    if item.auto_quality_level == level {
        return false;
    }
    // IDA 0x9ad8: `STR R1,[R0,#0x7C]`.
    item.auto_quality_level = level;
    // IDA 0x9adc..0x9ae2: signal(this + 0xC0, &unk_130C2AC).
    item.property_changed.fire(PROP_QUALITY_LEVEL);
    true
}

// 0x9ae8 — __ZThn96_N19CRenderSettingsItem19setAutoQualityLevelEi
// mangled: __ZThn96_N19CRenderSettingsItem19setAutoQualityLevelEi
// type: int __fastcall(int this, int)
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::setAutoQualityLevel(int)")]
pub fn stub_9ae8(item: &mut RenderSettingsItem, level: i32) -> bool {
    // Non-virtual thunk: incoming `this` points 96 (0x60) past the item start
    // (IDA 0x9af4 `SUBS R0, #0x60`), so the +0x1C compare (IDA 0x9ae8..0x9aec)
    // is the item's +0x7C cell. The host takes the adjusted item, making the
    // body identical to 0x9ac8 (IDA 0x9afc..0x9b04: store + signal &unk_130C2AC).
    stub_9ac8(item, level)
}

// 0x9b08 — __ZN19CRenderSettingsItem21setEagerBulkExecutionEb
// mangled: __ZN19CRenderSettingsItem21setEagerBulkExecutionEb
// type: int __fastcall(int this, int)
#[doc(alias = "CRenderSettingsItem::setEagerBulkExecution(bool)")]
pub fn stub_9b08(item: &mut RenderSettingsItem, value: bool) -> bool {
    // IDA 0x9b08..0x9b10: `LDRB R2,[R0,#0x9D]; CMP R1,R2; BXEQ LR` — unchanged.
    if item.eager_bulk_execution == value {
        return false;
    }
    // IDA 0x9b1a: `STRB R1,[R0,#0x9D]`.
    item.eager_bulk_execution = value;
    // IDA 0x9b1e..0x9b26: signal(this + 0xC0, &unk_130C1E8).
    item.property_changed.fire(PROP_EAGER_BULK_EXECUTION);
    true
}

// 0x9b2c — __ZNSt12length_errorD1Ev
// mangled: __ZNSt12length_errorD1Ev
// type: void __cdecl(std::length_error *__hidden this)
#[doc(alias = "std::length_error::~length_error()")]
pub fn stub_9b2c(this: *mut StdLengthError) {
    // IDA 0x9b2c: `B.W std::logic_error::~logic_error` thunk (D1 complete-object
    // dtor, non-deleting). Base dtor frees the `what()` string; the host owner
    // drops `message`, so running it here clears the string in place.
    if !this.is_null() {
        unsafe { (*this).message.clear(); }
    }
}

// 0x9b30 — __ZNSt12out_of_rangeD0Ev
// mangled: __ZNSt12out_of_rangeD0Ev
// type: void __cdecl(std::out_of_range *__hidden this)
#[doc(alias = "std::out_of_range::~out_of_range()")]
pub fn stub_9b30(this: *mut StdOutOfRange) {
    // IDA 0x9b30..0x9b36: `logic_error::~logic_error(this)` then 0x9b3a..0x9b40
    // `operator delete(this)` (D0 deleting dtor). Host: run the base dtor and
    // free the heap object. Caller must have come from `Box::into_raw`.
    if this.is_null() {
        return;
    }
    unsafe {
        (*this).message.clear();
        drop(Box::from_raw(this));
    }
}

// 0x9b44 — __ZNSt12out_of_rangeD2Ev
// mangled: __ZNSt12out_of_rangeD2Ev
// type: void __cdecl(std::out_of_range *__hidden this)
#[doc(alias = "std::out_of_range::~out_of_range()")]
pub fn stub_9b44(this: *mut StdOutOfRange) {
    // IDA 0x9b44: `B.W std::logic_error::~logic_error` thunk (D2 base-object
    // dtor, non-deleting). Same as 0x9b2c but for `out_of_range`.
    if !this.is_null() {
        unsafe { (*this).message.clear(); }
    }
}

// 0x9b48 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE7addPairES3_PKc
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::addPair(RBX::CRenderSettings::AASamples,char const*)")]
pub fn stub_9b48(desc: &mut RenderEnumDesc, value: i32, name: &str) {
    // IDA 0x9b48..0x9df6: `EnumDesc<AASamples>::addPair`. Allocates an
    // `EnumDescriptor::Item` (0x9b7e..0x9bee), grows the value/name vectors
    // with `-1`/null padding (0x9bf6..0x9dbc), inserts the `Name` and the
    // name->value map entry (0x9d10..0x9dd4), bumps the serial (0x9dde..0x9df6).
    // Host collapses all of that into the pair vec; the 0x9c2a..0x9cc8
    // `value>=0` / `value<=2304` asserts survive as a debug assert.
    debug_assert!(value >= 0 && value <= 2304, "addPair value out of range");
    desc.add_pair(value, name);
}

// 0x9ea8 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE7addPairES3_PKc
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::addPair(RBX::CRenderSettings::GraphicsMode,char const*)")]
pub fn stub_9ea8(desc: &mut RenderEnumDesc, value: i32, name: &str) {
    // IDA 0x9ea8: same body as 0x9b48 for `EnumDesc<GraphicsMode>::addPair`
    // (descriptor alloc, vector grows, asserts, map insert, serial bump).
    debug_assert!(value >= 0 && value <= 2304, "addPair value out of range");
    desc.add_pair(value, name);
}

// 0xa208 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE9addLegacyEiPKcS3_
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE9addLegacyEiPKcS3_
// type: _DWORD *__fastcall(int, unsigned int, int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::addLegacy(int,char const*,RBX::CRenderSettings::GraphicsMode)")]
pub fn stub_a208(desc: &mut RenderEnumDesc, index: u32, name: &str, maps_to: i32) {
    // IDA 0xa208..0xa25a: `EnumDesc<GraphicsMode>::addLegacy`. Grows the
    // legacy vector with `-1` padding to `index` (0xa22a..0xa238), stores
    // `maps_to` at `index` (0xa23a), declares the legacy `Name` (0xa244) and
    // maps it to `maps_to` (0xa24c..0xa250), returns the map slot.
    desc.add_legacy_alias(index as i32, name, maps_to);
}

// 0xa25c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE7addPairES3_PKc
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::addPair(RBX::CRenderSettings::FrameRateManagerMode,char const*)")]
pub fn stub_a25c(desc: &mut RenderEnumDesc, value: i32, name: &str) {
    // IDA 0xa25c: same body as 0x9b48 for
    // `EnumDesc<FrameRateManagerMode>::addPair`.
    debug_assert!(value >= 0 && value <= 2304, "addPair value out of range");
    desc.add_pair(value, name);
}

// 0xa5bc — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE7addPairES3_PKc
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::addPair(RBX::CRenderSettings::AntialiasingMode,char const*)")]
pub fn stub_a5bc(desc: &mut RenderEnumDesc, value: i32, name: &str) {
    // IDA 0xa5bc: same body as 0x9b48 for
    // `EnumDesc<AntialiasingMode>::addPair`.
    debug_assert!(value >= 0 && value <= 2304, "addPair value out of range");
    desc.add_pair(value, name);
}

// 0xa91c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE7addPairES3_PKc
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::addPair(RBX::CRenderSettings::ShadowMode,char const*)")]
pub fn stub_a91c(desc: &mut RenderEnumDesc, value: i32, name: &str) {
    // IDA 0xa91c: same body as 0x9b48 for `EnumDesc<ShadowMode>::addPair`.
    debug_assert!(value >= 0 && value <= 2304, "addPair value out of range");
    desc.add_pair(value, name);
}

// 0xac7c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE7addPairES3_PKc
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::addPair(RBX::CRenderSettings::QualityLevel,char const*)")]
pub fn stub_ac7c(desc: &mut RenderEnumDesc, value: i32, name: &str) {
    // IDA 0xac7c: same body as 0x9b48 for `EnumDesc<QualityLevel>::addPair`.
    debug_assert!(value >= 0 && value <= 2304, "addPair value out of range");
    desc.add_pair(value, name);
}

// 0xafdc — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE7addPairES3_PKc
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::addPair(RBX::CRenderSettings::ResolutionPreset,char const*)")]
pub fn stub_afdc(desc: &mut RenderEnumDesc, value: i32, name: &str) {
    // IDA 0xafdc: same body as 0x9b48 for
    // `EnumDesc<ResolutionPreset>::addPair`.
    debug_assert!(value >= 0 && value <= 2304, "addPair value out of range");
    desc.add_pair(value, name);
}

// 0xb33c — __ZNK3RBX15CRenderSettings15getGraphicsModeEv
// mangled: __ZNK3RBX15CRenderSettings15getGraphicsModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getGraphicsMode(void)const")]
pub fn stub_b33c(item: &RenderSettingsItem) -> i32 {
    // IDA 0xb33c..0xb33e: `return *((_DWORD *)this + 1)` — graphicsMode cell
    // (host `RenderSettingsItem::graphics_mode`, IDA `this + 100`).
    item.graphics_mode
}

// 0xb340 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEED1Ev
// mangled: __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::~EnumPropDescriptor()")]
pub fn stub_b340(desc: *mut RenderPropDescriptor) -> *mut RenderPropDescriptor {
    // IDA 0xb340..0xb362: `~EnumPropDescriptor<GraphicsMode>`: vtable reset
    // (host nop), `delete a1[11]` if non-null, return `a1`.
    if !desc.is_null() {
        unsafe { (*desc).extra.take(); }
    }
    desc
}

// 0xb364 — __ZNK3RBX15CRenderSettings23getFrameRateManagerModeEv
// mangled: __ZNK3RBX15CRenderSettings23getFrameRateManagerModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getFrameRateManagerMode(void)const")]
pub fn stub_b364(item: &RenderSettingsItem) -> i32 {
    // IDA 0xb364..0xb366: `return *((_DWORD *)this + 4)` — frameRateManagerMode
    // cell (host `frame_rate_manager_mode`, IDA `this + 112`).
    item.frame_rate_manager_mode
}

// 0xb368 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEED1Ev
// mangled: __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::~EnumPropDescriptor()")]
pub fn stub_b368(desc: *mut RenderPropDescriptor) -> *mut RenderPropDescriptor {
    // IDA 0xb368..0xb38a: `~EnumPropDescriptor<FrameRateManagerMode>`:
    // vtable reset, `delete a1[11]`, return `a1`.
    if !desc.is_null() {
        unsafe { (*desc).extra.take(); }
    }
    desc
}

// 0xb38c — __ZNK3RBX15CRenderSettings15getQualityLevelEv
// mangled: __ZNK3RBX15CRenderSettings15getQualityLevelEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getQualityLevel(void)const")]
pub fn stub_b38c(item: &RenderSettingsItem) -> i32 {
    // IDA 0xb38c..0xb38e: `return *((_DWORD *)this + 5)` — qualityLevel cell
    // (host `quality_level`, IDA `this + 116`).
    item.quality_level
}

// 0xb390 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEED1Ev
// mangled: __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::~EnumPropDescriptor()")]
pub fn stub_b390(desc: *mut RenderPropDescriptor) -> *mut RenderPropDescriptor {
    // IDA 0xb390..0xb3b2: `~EnumPropDescriptor<QualityLevel>`: vtable reset,
    // `delete a1[11]`, return `a1`.
    if !desc.is_null() {
        unsafe { (*desc).extra.take(); }
    }
    desc
}

// 0xb3b4 — __ZNK3RBX15CRenderSettings23getAlwaysDrawConnectorsEv
// mangled: __ZNK3RBX15CRenderSettings23getAlwaysDrawConnectorsEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getAlwaysDrawConnectors(void)const")]
pub fn stub_b3b4(item: &RenderSettingsItem) -> i32 {
    // IDA 0xb3b4..0xb3b8: `return *((u8 *)this + 59)` — alwaysDrawConnectors
    // flag (host `always_draw_connectors`, IDA `this + 155`).
    item.always_draw_connectors as i32
}

// 0xb3bc — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembED1Ev
// mangled: __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::~PropDescriptor()")]
pub fn stub_b3bc(desc: *mut RenderPropDescriptor) -> *mut RenderPropDescriptor {
    // IDA 0xb3bc..0xb3de: `~PropDescriptor<bool>`: vtable reset,
    // `delete a1[10]` (note: index 10, not 11 as in the enum variant),
    // return `a1`.
    if !desc.is_null() {
        unsafe { (*desc).extra.take(); }
    }
    desc
}

// 0xb3e0 — __ZNK3RBX15CRenderSettings18getShowAggregationEv
// mangled: __ZNK3RBX15CRenderSettings18getShowAggregationEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getShowAggregation(void)const")]
pub fn stub_b3e0(item: &RenderSettingsItem) -> i32 {
    // IDA 0xb3e0..0xb3e4: `return *((u8 *)this + 58)` — showAggregation flag
    // (host `show_aggregation`, IDA `this + 154`).
    item.show_aggregation as i32
}

// 0xb3e8 — __ZNK3RBX15CRenderSettings12getAASamplesEv
// mangled: __ZNK3RBX15CRenderSettings12getAASamplesEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getAASamples(void)const")]
pub fn stub_b3e8(_item: &RenderSettingsItem) -> i32 {
    // IDA 0xb3e8..0xb3f6: ignores `this`, loads the
    // `CRenderSettings::aaSamples` process global (host `AA_SAMPLES`,
    // written by `setAASamples` at 0x96d0).
    AA_SAMPLES.load(Ordering::Relaxed)
}

// 0xb3f8 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEED1Ev
// mangled: __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::~EnumPropDescriptor()")]
pub fn stub_b3f8(desc: *mut RenderPropDescriptor) -> *mut RenderPropDescriptor {
    // IDA 0xb3f8..0xb41a: `~EnumPropDescriptor<AASamples>`: vtable reset,
    // `delete a1[11]`, return `a1`.
    if !desc.is_null() {
        unsafe { (*desc).extra.take(); }
    }
    desc
}

// 0xb41c — __ZNK3RBX15CRenderSettings13getShadowModeEv
// mangled: __ZNK3RBX15CRenderSettings13getShadowModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getShadowMode(void)const")]
pub fn stub_b41c(item: &RenderSettingsItem) -> i32 {
    // IDA 0xb41c..0xb41e: `return *((_DWORD *)this + 3)` — shadowMode cell
    // (host `shadow_mode`, IDA `this + 108`).
    item.shadow_mode
}

// 0xb420 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEED1Ev
// mangled: __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::~EnumPropDescriptor()")]
pub fn stub_b420(desc: *mut RenderPropDescriptor) -> *mut RenderPropDescriptor {
    // IDA 0xb420..0xb442: `~EnumPropDescriptor<ShadowMode>`: vtable reset,
    // `delete a1[11]`, return `a1`.
    if !desc.is_null() {
        unsafe { (*desc).extra.take(); }
    }
    desc
}

// 0xb444 — __ZNK3RBX15CRenderSettings19getAntialiasingModeEv
// mangled: __ZNK3RBX15CRenderSettings19getAntialiasingModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getAntialiasingMode(void)const")]
pub fn stub_b444(item: &RenderSettingsItem) -> i32 {
    // IDA 0xb444..0xb446: `return *((_DWORD *)this + 2)` — antialiasingMode
    // cell (host `antialiasing_mode`, IDA `this + 104`).
    item.antialiasing_mode
}

// 0xb448 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEED1Ev
// mangled: __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::~EnumPropDescriptor()")]
pub fn stub_b448(desc: *mut RenderPropDescriptor) -> *mut RenderPropDescriptor {
    // IDA 0xb448..0xb46a: `~EnumPropDescriptor<AntialiasingMode>`: vtable
    // reset, `delete a1[11]`, return `a1`.
    if !desc.is_null() {
        unsafe { (*desc).extra.take(); }
    }
    desc
}

// 0xb46c — __ZNK3RBX15CRenderSettings25getDebugShowBoundingBoxesEv
// mangled: __ZNK3RBX15CRenderSettings25getDebugShowBoundingBoxesEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getDebugShowBoundingBoxes(void)const")]
pub fn stub_b46c(item: &RenderSettingsItem) -> i32 {
    // IDA 0xb46c..0xb470: `return *((u8 *)this + 40)` —
    // debugShowBoundingBoxes flag (host `debug_show_bounding_boxes`,
    // IDA `this + 136`).
    item.debug_show_bounding_boxes as i32
}

// 0xb474 — __ZNK3RBX15CRenderSettings19getAutoQualityLevelEv
// mangled: __ZNK3RBX15CRenderSettings19getAutoQualityLevelEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getAutoQualityLevel(void)const")]
pub fn stub_b474(item: &RenderSettingsItem) -> i32 {
    // IDA 0xb474..0xb476: `return *((_DWORD *)this + 7)` — autoQualityLevel
    // cell (host `auto_quality_level`, IDA `this + 124`).
    item.auto_quality_level
}

// 0xb478 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiED1Ev
// mangled: __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::~PropDescriptor()")]
pub fn stub_b478(desc: *mut RenderPropDescriptor) -> *mut RenderPropDescriptor {
    // IDA 0xb478..0xb49a: `~PropDescriptor<int>`: vtable reset,
    // `delete a1[10]`, return `a1`.
    if !desc.is_null() {
        unsafe { (*desc).extra.take(); }
    }
    desc
}

// 0xb49c — __ZNK3RBX15CRenderSettings12getEnableFRMEv
// mangled: __ZNK3RBX15CRenderSettings12getEnableFRMEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getEnableFRM(void)const")]
pub fn stub_b49c(item: &RenderSettingsItem) -> i32 {
    // IDA 0xb49c..0xb4a0: `return *((u8 *)this + 41)` — enableFRM flag
    // (host `enable_frm`, IDA `this + 137`).
    item.enable_frm as i32
}

// 0xb4a4 — __ZNK3RBX15CRenderSettings23getResolutionPreferenceEv
// mangled: __ZNK3RBX15CRenderSettings23getResolutionPreferenceEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getResolutionPreference(void)const")]
pub fn stub_b4a4(item: &RenderSettingsItem) -> i32 {
    // IDA 0xb4a4..0xb4a6: `return *((_DWORD *)this + 6)` — resolutionPreference
    // cell (host `resolution_preference`, IDA `this + 120`).
    item.resolution_preference
}

// 0xb4a8 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEED1Ev
// mangled: __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::~EnumPropDescriptor()")]
pub fn stub_b4a8(desc: *mut RenderPropDescriptor) -> *mut RenderPropDescriptor {
    // IDA 0xb4b2..0xb4ca: `~EnumPropDescriptor<ResolutionPreset>`: vtable reset
    // (host nop), `delete a1[11]` if non-null, return `a1`.
    if !desc.is_null() {
        unsafe { (*desc).extra.take(); }
    }
    desc
}

// 0xb4cc — __ZN3RBX15CRenderSettings18getMaxQualityLevelEv
// mangled: __ZN3RBX15CRenderSettings18getMaxQualityLevelEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getMaxQualityLevel(void)")]
pub fn stub_b4cc(item: &RenderSettingsItem) -> i32 {
    // IDA 0xb4cc..0xb4ce: `return *((_DWORD *)this + 8)` — maxQualityLevel cell
    // (host `max_quality_level`, IDA `this + 128`).
    item.max_quality_level
}

// 0xb4d0 — __ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EED1Ev
// mangled: __ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::~BoundFuncDesc()")]
pub fn stub_b4d0() {
    // IDA 0xb4d0..0xb4f2: `~BoundFuncDesc<CRenderSettingsItem,int ()(void),0>`:
    // vtable reset + `std::list:: _M_clear(a1 + 8)` (signature items). Drop
    // glue — no-op (audio `stub_559614` precedent).
}

// 0xb4f4 — __ZNK3RBX15CRenderSettings19getTextureCacheSizeEv
// mangled: __ZNK3RBX15CRenderSettings19getTextureCacheSizeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getTextureCacheSize(void)const")]
pub fn stub_b4f4(item: &RenderSettingsItem) -> i32 {
    // IDA 0xb4f4..0xb4f6: `return *((_DWORD *)this + 16)` — textureCacheSize
    // cell (host `texture_cache_size`, IDA `this + 160`).
    item.texture_cache_size as i32
}

// 0xb4f8 — __ZNK3RBX15CRenderSettings16getMeshCacheSizeEv
// mangled: __ZNK3RBX15CRenderSettings16getMeshCacheSizeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getMeshCacheSize(void)const")]
pub fn stub_b4f8(item: &RenderSettingsItem) -> i32 {
    // IDA 0xb4f8..0xb4fa: `return *((_DWORD *)this + 17)` — meshCacheSize cell
    // (host `mesh_cache_size`, IDA `this + 164`).
    item.mesh_cache_size as i32
}

// 0xb4fc — __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEEC2Ev
// mangled: __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEEC2Ev
// type: RBX::Instance *__fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEEC2Ev")]
pub fn stub_b4fc(dx_video_memory_size: u32) -> RenderSettingsItem {
    // IDA 0xb51e: `Instance::C2(nullptr)`; 0xb522..0xb5d4: vtable stores +
    // classDescriptor/registrar (host nops); 0xb5d8..0xb5f8:
    // `setName("RenderSettings")`; 0xb61c..0xb6b4: singleton `singE` check —
    // `RBX::runtime_error("singleton %s already exists")` + throw if set.
    // The body never calls `CRenderSettingsItem::C2`; the host folds the full
    // item init (0x97d0) in and renames to the singleton name.
    static SINGLETON_SET: AtomicBool = AtomicBool::new(false);
    if SINGLETON_SET.swap(true, Ordering::SeqCst) {
        panic!("singleton RenderSettings already exists");
    }
    let mut item = stub_97d0(dx_video_memory_size);
    item.instance_name = "RenderSettings".to_string();
    item
}

// 0xb740 — __ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE9push_backERKS1_
// mangled: __ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE9push_backERKS1_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::push_back(G3D::Vector2int16 const&)")]
pub fn stub_b740(vec: &mut Vec<(u16, u16)>, value: (u16, u16)) {
    // IDA 0xb742..0xb75c: `size == capacity` (0xb74c) routes to the
    // `_M_insert_aux` realloc slow path (0xb766); else `*finish = *a2`
    // (0xb756), `finish += 4` (0xb75c). Host `Vec::push` covers both paths.
    vec.push(value);
}

// 0xb76c — __ZN3rbx7signals16signal_with_argsILi1EFvPKN3RBX10Reflection18PropertyDescriptorEEEclES6_
// mangled: __ZN3rbx7signals16signal_with_argsILi1EFvPKN3RBX10Reflection18PropertyDescriptorEEEclES6_
// type: void __fastcall(_DWORD *, int, int, const void *, int, int, int, int, void *, int)
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Reflection::PropertyDescriptor const*)>::operator()(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_b76c(signal: &Signal<u32>, descriptor: u32) {
    // IDA 0xb7ba: empty slot list returns; 0xb7c8..0xb7e0: `FLog::SignalPrints`
    // "Signal with 1 arg executed" trace (host nop); 0xb7e6..0xb80a: per-slot
    // vtable call with the descriptor; 0xb7fe..0xb812: `next()` + `on_error`.
    // Host `Signal::fire` runs the slot loop.
    signal.fire(descriptor);
}

// 0xb8b0 — __ZNK3RBX15CRenderSettings21getEagerBulkExecutionEv
// mangled: __ZNK3RBX15CRenderSettings21getEagerBulkExecutionEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getEagerBulkExecution(void)const")]
pub fn stub_b8b0(item: &RenderSettingsItem) -> i32 {
    // IDA 0xb8b0..0xb8b4: `return *((unsigned __int8 *)this + 61)` —
    // eagerBulkExecution flag (host `eager_bulk_execution`, IDA `this + 157`).
    item.eager_bulk_execution as i32
}

// 0xb8b8 — __ZN19CRenderSettingsItemD1Ev
// mangled: __ZN19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "CRenderSettingsItem::~CRenderSettingsItem()")]
pub fn stub_b8b8(_this: *mut RenderSettingsItem) {
    // IDA 0xb8b8: thunk to `CRenderSettingsItem::D2` (non-deleting, complete
    // object). Member drops belong to the host owner — no-op.
}

// 0xb8bc — __ZN19CRenderSettingsItemD0Ev
// mangled: __ZN19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "CRenderSettingsItem::~CRenderSettingsItem()")]
pub fn stub_b8bc(this: *mut RenderSettingsItem) {
    // IDA 0xb8bc..0xb8cc: `D2(this)` then `operator delete(this)` (D0 deleting
    // dtor). Host: full drop + free; caller must have come from `Box::into_raw`
    // (9b30 `out_of_range` D0 precedent).
    if this.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(this));
    }
}

// 0xb8d0 — __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv
// mangled: __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv")]
pub fn stub_b8d0() -> &'static str {
    // IDA 0xb8d0..0xb8d4: `static_getCreator()` then `Creator::getClassName`
    // (0xedfc): `ReleaseAssert(wasConstructed())` (`../App/include/Util/Object.h`),
    // `Name::declare<sRenderSettings>()`, tail-jump to
    // `Name::doDeclare<sRenderSettings>()` returning the `RenderSettings` name.
    "RenderSettings"
}

// 0xb8e0 — __ZThn32_N19CRenderSettingsItemD1Ev
// mangled: __ZThn32_N19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
pub fn stub_b8e0(this: *mut RenderSettingsItem) {
    // IDA 0xb8e0..0xb8e2: non-virtual thunk, `this - 32`, tail-call D1 (0xb8b8).
    unsafe { stub_b8b8(this.cast::<u8>().sub(32).cast()) }
}

// 0xb8e8 — __ZThn32_N19CRenderSettingsItemD0Ev
// mangled: __ZThn32_N19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
pub fn stub_b8e8(this: *mut RenderSettingsItem) {
    // IDA 0xb8e8..0xb8fc: non-virtual thunk, `this - 32`, D2 + `delete(v1)`
    // (D0). Host: deleting dtor on the adjusted object.
    unsafe { stub_b8bc(this.cast::<u8>().sub(32).cast()) }
}

// 0xb900 — __ZThn32_NK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv
// mangled: __ZThn32_NK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv")]
pub fn stub_b900() -> &'static str {
    // IDA 0xb900..0xb904: `__ZThn32_` non-virtual thunk (+32 `this`), same
    // `static_getCreator` + `Creator::getClassName` sequence as 0xb8d0.
    stub_b8d0()
}

// 0xb910 — __ZThn36_N19CRenderSettingsItemD1Ev
// mangled: __ZThn36_N19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
pub fn stub_b910(this: *mut RenderSettingsItem) {
    // IDA 0xb910..0xb912: non-virtual thunk, `this - 36`, tail-call D1 (0xb8b8).
    unsafe { stub_b8b8(this.cast::<u8>().sub(36).cast()) }
}

// 0xb918 — __ZThn36_N19CRenderSettingsItemD0Ev
// mangled: __ZThn36_N19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
pub fn stub_b918(this: *mut RenderSettingsItem) {
    // IDA 0xb918..0xb92c: non-virtual thunk, `this - 36`, D2 + `delete(v1)`
    // (D0). Host: deleting dtor on the adjusted object.
    unsafe { stub_b8bc(this.cast::<u8>().sub(36).cast()) }
}

// 0xb930 — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD1Ev
// mangled: __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_b930() {
    // IDA 0xb930: thunk to `Creator::D2` (`FactoryProduct<...>::Creator`
    // stateless) — drop glue, no-op (audio `stub_559614` precedent).
}

// 0xb934 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED1Ev
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()")]
pub fn stub_b934(desc: *mut RenderEnumDesc) {
    // IDA 0xb934: thunk to `EnumDesc<AASamples>::D2` (non-deleting). Member
    // vectors drop in place; the host owner frees (9b2c `length_error` D1
    // precedent: clear owned heap state, keep the object).
    if !desc.is_null() {
        unsafe {
            (*desc).pairs.clear();
            (*desc).legacy_aliases.clear();
        }
    }
}

// 0xb938 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED0Ev
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()")]
pub fn stub_b938(desc: *mut RenderEnumDesc) {
    // IDA 0xb938..0xb948: `D2(a1)` then `operator delete(a1)` (D0 deleting
    // dtor). Host: clear + full drop + free; caller must have come from
    // `Box::into_raw` (9b30 `out_of_range` D0 precedent).
    if desc.is_null() {
        return;
    }
    unsafe {
        (*desc).pairs.clear();
        (*desc).legacy_aliases.clear();
        drop(Box::from_raw(desc));
    }
}

// 0xb94c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupEPKc
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::lookup(char const*)const")]
pub fn stub_b94c(desc: &RenderEnumDesc, name: &str) -> Option<i32> {
    // IDA 0xb956..0xb972: `Name::lookup(name)` (0xb958), `convertToValue` hit
    // (0xb966..0xb96c) returns `convertToItem` (0xb972), else null (0xb968).
    // The name map holds primary + legacy names, so both tables are searched;
    // the host returns the value where the original returns the item pointer.
    desc.pairs.iter().find(|p| p.name == name).map(|p| p.value).or_else(|| {
        desc.legacy_aliases.iter().find(|a| a.name == name).map(|a| a.maps_to)
    })
}

// 0xb97c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupERKNS0_7VariantE
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_b97c(desc: &RenderEnumDesc, value: i32) -> Option<i32> {
    // IDA 0xb98e..0xb998: `any_cast<AASamples const&>` on the variant payload
    // (host: typed parameter, check is static), then `convertToItem` — the
    // item for that value, null when absent. Host returns the value itself.
    desc.pairs.iter().find(|p| p.value == value).map(|p| p.value)
}

// 0xb99c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueEmRNS0_7VariantE
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_b99c(desc: &RenderEnumDesc, value: u32, out: &mut i32) -> bool {
    // IDA 0xb9a4..0xb9b6: `count = [R0,#0x28]`; `count > value` (unsigned HI)
    // loads `item = table[value]` (`[R0,#0x90]` indexed `LSL#2`), stores to the
    // out `Variant`, returns 1; else returns 0. Host searches the pair table.
    match desc.pairs.iter().find(|p| p.value == value as i32) {
        Some(p) => {
            *out = p.value;
            true
        }
        None => false,
    }
}

// 0xb9f8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE15convertToStringEmRSs
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToString(unsigned long,std::string &)const")]
pub fn stub_b9f8(desc: &RenderEnumDesc, value: u32, out: &mut String) -> bool {
    // IDA 0xba4a..0xba4c: `count <= value` (BLS) skips to the epilogue with 0,
    // leaving `out` untouched; else 0xba4e..0xba72: `item = table[value]`,
    // `convertToString(item)` to a temp, `out->assign(temp)`, return 1.
    match desc.pairs.iter().find(|p| p.value == value as i32) {
        Some(p) => {
            out.clear();
            out.push_str(&p.name);
            true
        }
        None => false,
    }
}

// 0xbb3c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED1Ev
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()")]
pub fn stub_bb3c(desc: *mut RenderEnumDesc) {
    // IDA 0xbb3c: thunk to `EnumDesc<GraphicsMode>::D2` (non-deleting). Same
    // shape as the AASamples twin at 0xb934: clear member tables in place.
    if !desc.is_null() {
        unsafe {
            (*desc).pairs.clear();
            (*desc).legacy_aliases.clear();
        }
    }
}

// 0xbb40 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED0Ev
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()")]
pub fn stub_bb40(desc: *mut RenderEnumDesc) {
    // IDA 0xbb40..0xbb52: `D2(a1)` (0xbb46) then `operator delete(a1)` (D0).
    // Same shape as the AASamples twin at 0xb938.
    if desc.is_null() {
        return;
    }
    unsafe {
        (*desc).pairs.clear();
        (*desc).legacy_aliases.clear();
        drop(Box::from_raw(desc));
    }
}

// 0xbb54 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE6lookupEPKc
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::lookup(char const*)const")]
pub fn stub_bb54(desc: &RenderEnumDesc, name: &str) -> Option<i32> {
    // IDA 0xbb5e..0xbb7a: `Name::lookup` (0xbb60), `convertToValue` (0xbb6e),
    // hit returns `convertToItem` (0xbb7a), else null. Same as 0xb94c.
    desc.pairs.iter().find(|p| p.name == name).map(|p| p.value).or_else(|| {
        desc.legacy_aliases.iter().find(|a| a.name == name).map(|a| a.maps_to)
    })
}

// 0xbb84 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE6lookupERKNS0_7VariantE
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_bb84(desc: &RenderEnumDesc, value: i32) -> Option<i32> {
    // IDA 0xbb96..0xbba0: `any_cast<GraphicsMode const&>` (0xbb96) then
    // `convertToItem` (0xbba0). Same shape as the AASamples twin at 0xb97c.
    desc.pairs.iter().find(|p| p.value == value).map(|p| p.value)
}

// 0xbba4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueEmRNS0_7VariantE
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_bba4(desc: &RenderEnumDesc, value: u32, out: &mut i32) -> bool {
    // IDA 0xbbac..0xbbbe: `count = [R0,#0x28]`, `count > value` loads
    // `table[value]`, stores to the out `Variant` (0xbbbc), returns 1
    // (0xbbbe); 0xbbc0..0xbbf4: singleton + `placement_any` publish; miss
    // returns 0. Same shape as the AASamples twin at 0xb99c.
    match desc.pairs.iter().find(|p| p.value == value as i32) {
        Some(p) => {
            *out = p.value;
            true
        }
        None => false,
    }
}

// 0xbc00 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringEmRSs
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToString(unsigned long,std::string &)const")]
pub fn stub_bc00(desc: &RenderEnumDesc, value: u32, out: &mut String) -> bool {
    // IDA 0xbc00: same body as the AASamples `convertToString` at 0xb9f8 —
    // out-of-range returns 0 with `out` untouched, else `convertToString(item)`
    // to a temp, `out->assign(temp)`, return 1.
    match desc.pairs.iter().find(|p| p.value == value as i32) {
        Some(p) => {
            out.clear();
            out.push_str(&p.name);
            true
        }
        None => false,
    }
}

// 0xbd44 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED1Ev
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::~EnumDesc()")]
pub fn stub_bd44(desc: *mut RenderEnumDesc) {
    // IDA 0xbd44: thunk to `EnumDesc<FrameRateManagerMode>::D2`
    // (non-deleting). Same shape as the AASamples twin at 0xb934.
    if !desc.is_null() {
        unsafe {
            (*desc).pairs.clear();
            (*desc).legacy_aliases.clear();
        }
    }
}

// 0xbd48 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED0Ev
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::~EnumDesc()")]
pub fn stub_bd48(desc: *mut RenderEnumDesc) {
    // IDA 0xbd48..0xbd5a: `D2(a1)` (0xbd4e) then `operator delete(a1)` (D0).
    // Same shape as the AASamples twin at 0xb938.
    if desc.is_null() {
        return;
    }
    unsafe {
        (*desc).pairs.clear();
        (*desc).legacy_aliases.clear();
        drop(Box::from_raw(desc));
    }
}

// 0xbd5c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupEPKc
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::lookup(char const*)const")]
pub fn stub_bd5c(desc: &RenderEnumDesc, name: &str) -> Option<i32> {
    // IDA 0xbd66..0xbd82: `Name::lookup` (0xbd68), `convertToValue` (0xbd76),
    // hit returns `convertToItem` (0xbd82), else null. Same as 0xb94c.
    desc.pairs.iter().find(|p| p.name == name).map(|p| p.value).or_else(|| {
        desc.legacy_aliases.iter().find(|a| a.name == name).map(|a| a.maps_to)
    })
}

// 0xbd8c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupERKNS0_7VariantE
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_bd8c(desc: &RenderEnumDesc, value: i32) -> Option<i32> {
    // IDA 0xbd9e..0xbda8: `any_cast<FrameRateManagerMode const&>` (0xbd9e)
    // then `convertToItem` (0xbda8). Same shape as the twin at 0xb97c.
    desc.pairs.iter().find(|p| p.value == value).map(|p| p.value)
}

// 0xbdac — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueEmRNS0_7VariantE
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_bdac(desc: &RenderEnumDesc, value: u32, out: &mut i32) -> bool {
    // IDA 0xbdac: decompilation fails (like 0xb99c/0xbba4); disassembly shows
    // the same `count`/`table[value]` hit-flag body. Same as 0xb99c.
    match desc.pairs.iter().find(|p| p.value == value as i32) {
        Some(p) => {
            *out = p.value;
            true
        }
        None => false,
    }
}

// 0xbe08 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE15convertToStringEmRSs
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToString(unsigned long,std::string &)const")]
pub fn stub_be08(desc: &RenderEnumDesc, value: u32, out: &mut String) -> bool {
    // IDA 0xbe08: same body as the AASamples `convertToString` at 0xb9f8 —
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

// 0xbf4c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED1Ev
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc()")]
pub fn stub_bf4c(desc: *mut RenderEnumDesc) {
    // IDA 0xbf4c: thunk to `EnumDesc<AntialiasingMode>::D2` (non-deleting).
    // Same shape as the AASamples twin at 0xb934.
    if !desc.is_null() {
        unsafe {
            (*desc).pairs.clear();
            (*desc).legacy_aliases.clear();
        }
    }
}

// 0xbf50 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED0Ev
// mangled: __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc()")]
pub fn stub_bf50(desc: *mut RenderEnumDesc) {
    // IDA 0xbf50..0xbf62: `D2(a1)` (0xbf56) then `operator delete(a1)` (D0).
    // Same shape as the AASamples twin at 0xb938.
    if desc.is_null() {
        return;
    }
    unsafe {
        (*desc).pairs.clear();
        (*desc).legacy_aliases.clear();
        drop(Box::from_raw(desc));
    }
}

// 0xbf64 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE6lookupEPKc
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::lookup(char const*)const")]
pub fn stub_bf64(desc: &RenderEnumDesc, name: &str) -> Option<i32> {
    // IDA 0xbf6e..0xbf8a: `Name::lookup` (0xbf70), `convertToValue` (0xbf7e),
    // hit returns `convertToItem` (0xbf8a), else null. Same as 0xb94c.
    desc.pairs.iter().find(|p| p.name == name).map(|p| p.value).or_else(|| {
        desc.legacy_aliases.iter().find(|a| a.name == name).map(|a| a.maps_to)
    })
}

// 0xbf94 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE6lookupERKNS0_7VariantE
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_bf94(desc: &RenderEnumDesc, value: i32) -> Option<i32> {
    // IDA 0xbfa6..0xbfb0: `any_cast<AntialiasingMode const&>` (0xbfa6) then
    // `convertToItem` (0xbfb0). Same shape as the twin at 0xb97c.
    desc.pairs.iter().find(|p| p.value == value).map(|p| p.value)
}

// 0xbfb4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToValueEmRNS0_7VariantE
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_bfb4(desc: &RenderEnumDesc, value: u32, out: &mut i32) -> bool {
    // IDA 0xbfb4..0xbfce: `count = [R0,#0x28]`, `count > value` loads
    // `table[value]` (0xbfc4..0xbfc8), stores to out (0xbfcc), returns 1
    // (0xbfce); miss returns 0. Same shape as 0xb99c.
    match desc.pairs.iter().find(|p| p.value == value as i32) {
        Some(p) => {
            *out = p.value;
            true
        }
        None => false,
    }
}

// 0xc010 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE15convertToStringEmRSs
// mangled: __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToString(unsigned long,std::string &)const")]
pub fn stub_c010(desc: &RenderEnumDesc, value: u32, out: &mut String) -> bool {
    // IDA 0xc010: same body as the AASamples `convertToString` at 0xb9f8 —
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
