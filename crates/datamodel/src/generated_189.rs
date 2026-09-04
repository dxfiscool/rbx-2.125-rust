// Auto-generated skeletons for rbx-datamodel -- from ida/export.json
// Filter: demangled contains RBX::Instance|RBX::DataModel|RBX::Workspace (exact RBX:: prefix), EA-sorted — filtered complete (10215/10215), global gap filler low-EA
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 120 stubs | range 0x84e0..0xceec | total filtered 10215, remaining 0 after batch; local 18258->18378 distinct, 67287->67167 not in datamodel (0 global missing)
// Shard: 189 EA-sorted asc next 120 low-EA global gap filler after 0x84e0 not yet in datamodel (filtered exhausted, 67287 missing before -> 67167 after)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use rbx_core::signal::Signal;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::LazyLock;

/// `G3D::Vector2int16`: two packed `int16` lanes. IDA 0xb740 moves one
/// element with a single 4-byte `LDR`/`STR`, so `sizeof == 4`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Vector2int16 {
    pub x: i16,
    pub y: i16,
}

/// Rust model of `CRenderSettingsItem` (IDA `0x97d0` ctor): only the
/// IDA-observed slots are modelled, with ARM byte offsets noted per field.
/// The `GlobalAdvancedSettingsItem` base subobject (vtables, class
/// descriptor, singleton slot) is owned by `stub_0xb4fc`; the `+96`
/// `CRenderSettings` subobject defaults are owned by its own ctor.
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
    /// +146 dword video-memory budget. IDA 0x97d0 `STR.W R2,[R1,#0x92]`
    /// (word 146/4 is the `+0x92`-byte store: `*(this+146) = select`).
    pub video_memory_budget: u32,
    /// +168 `std::string`, empty after construction. IDA 0x97d0 points it at
    /// `std::string::_Rep::_S_empty_rep_storage`.
    pub string_168: String,
    /// +172/+174 two `u16` lanes written as `800`/`600`; the value pushed
    /// into `resolutions` (IDA 0x97d0 `push_back`).
    pub first_resolution: Vector2int16,
    /// +176 `std::vector<G3D::Vector2int16>` (IDA 0x97d0 zeroes it, then pushes).
    pub resolutions: Vec<Vector2int16>,
    /// +189 byte set to 1 by the ctor (IDA 0x97d0); role not observed.
    pub byte_189: bool,
    /// Name passed to the `+28` setter virtual (IDA 0x97d0
    /// `std::string("Rendering")`).
    pub render_category: String,
    /// +0xC0: `rbx::signals::signal_with_args<1, void(const PropertyDescriptor*)>`.
    /// Every setter tail-calls it (`ADDS R0,#0xC0`) with its own
    /// `PropertyDescriptor` (`unk_130Cxxx`); modelled by descriptor name.
    pub property_changed: Signal<&'static str>,
    /// +0x5C byte, set to 1 by the `GlobalAdvancedSettingsItem` base C2.
    /// IDA 0xb5ba `STRB.W R3,[R1,#0x5C]`; owned by `stub_0xb4fc`.
    pub global_base_ready: bool,
}

/// IDA 0x96d0: `RBX::CRenderSettings::aaSamples` — a dword global, not an item
/// field (`LDR R2,[R2]; RBX::CRenderSettings::aaSamples` via `_ptr` slot).
pub static AA_SAMPLES: AtomicI32 = AtomicI32::new(0);
/// IDA 0x9784/0x9794: `RBX::PartInstance::disableInterpolation` — a byte global.
pub static DISABLE_INTERPOLATION: AtomicBool = AtomicBool::new(false);
/// IDA 0xb626/0xb622: `GlobalAdvancedSettingsItem<CRenderSettingsItem>::sing`
/// singleton slot — set by `stub_0xb4fc`, which throws `runtime_error` when
/// it is already set (0xb670/0xb688..0xb6b4).
pub static RENDER_SETTINGS_SINGLETON: AtomicBool = AtomicBool::new(false);

/// Rust model of `RBX::Reflection::EnumDesc<T>` (IDA `0x850c` family): the
/// name/value table built by `addPair`, plus the `RBX::Name`-declared wide
/// and `Level NN` aliases stored via `std::map::operator[]` (IDA 0x8e24/0x9100).
#[derive(Debug, Clone, Default)]
pub struct RenderEnumDesc {
    pub enum_name: &'static str,
    pub pairs: Vec<(i32, String)>,
    pub aliases: HashMap<String, i32>,
    /// IDA 0xa208: legacy value table (`a1 + 132`), grown by `addLegacy`.
    pub legacy_values: Vec<i32>,
}

impl RenderEnumDesc {
    /// IDA 0x850c: `EnumDescriptor::EnumDescriptor(this, name, typeinfo)`,
    /// vtable install, empty tables.
    pub fn new(enum_name: &'static str) -> Self {
        Self { enum_name, pairs: Vec::new(), aliases: HashMap::new(), legacy_values: Vec::new() }
    }
    /// IDA 0x9b48 family: push the (value, name) pair.
    pub fn add_pair(&mut self, value: i32, name: &str) {
        self.pairs.push((value, name.to_owned()));
    }
    /// IDA 0x8e24/0x9100: `RBX::Name::declare` + `std::map::operator[]`
    /// alias entries alongside the pairs.
    pub fn add_alias(&mut self, name: &str, value: i32) {
        self.aliases.insert(name.to_owned(), value);
    }
    /// IDA 0xa208: grow the legacy vector, map the legacy name to the value.
    pub fn add_legacy(&mut self, index: usize, name: &str, value: i32) {
        if self.legacy_values.len() <= index {
            self.legacy_values.resize(index + 1, -1);
        }
        self.legacy_values[index] = value;
        self.aliases.insert(name.to_owned(), value);
    }
    pub fn lookup_value(&self, name: &str) -> Option<i32> {
        self.pairs.iter().find(|(_, n)| n == name).map(|(v, _)| *v)
            .or_else(|| self.aliases.get(name).copied())
    }
    pub fn lookup_name(&self, value: i32) -> Option<&str> {
        self.pairs.iter().find(|(v, _)| *v == value).map(|(_, n)| n.as_str())
    }
}
/// Rust model of `rbx::placement_any<RBX::Region3>` (IDA 0xc90c/0xceec family):
/// a holder tag plus one enum payload word. The original stores the
/// `typed_holder<T>::singleton()` address at +0 and the value at +4; the tag
/// below stands in for that holder identity.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PlacementAny {
    pub holder: u32,
    pub value: i32,
}
/// Holder identity for `typed_holder<ResolutionPreset>::singleton()` (IDA 0xc95c).
pub const HOLDER_RESOLUTION_PRESET: u32 = 1;
/// Holder identity for `typed_holder<QualityLevel>::singleton()` (IDA 0xcf3c).
pub const HOLDER_QUALITY_LEVEL: u32 = 2;
/// Rust model of `rbx::implementation::typed_holder<T>` (IDA 0xc95c/0xcf3c):
/// the `{ typeinfo, destruct, construct }` triple. Destruct/construct are
/// no-ops for these trivial enum payloads (IDA 0xc9d4/0xcfb4 and the
/// 0xc9c8/0xcfa8 copy shape), so only the type identity is retained.
#[derive(Debug, Clone, Copy)]
pub struct TypedHolder {
    pub type_name: &'static str,
    pub token: u32,
}
/// IDA 0xc95c `singleton()::s` — `__cxa_guard` init becomes `LazyLock`.
static RESOLUTION_PRESET_HOLDER: LazyLock<TypedHolder> = LazyLock::new(|| TypedHolder {
    type_name: "N3RBX15CRenderSettings16ResolutionPresetE",
    token: HOLDER_RESOLUTION_PRESET,
});
/// IDA 0xcf3c `singleton()::s`, homed here so `generated_190` shares one model.
static QUALITY_LEVEL_HOLDER: LazyLock<TypedHolder> = LazyLock::new(|| TypedHolder {
    type_name: "N3RBX15CRenderSettings12QualityLevelE",
    token: HOLDER_QUALITY_LEVEL,
});
/// IDA 0xcf3c: `typed_holder<QualityLevel>::singleton()` (see `stub_0xc95c`
/// for the init shape).
pub fn quality_level_holder() -> &'static TypedHolder {
    LazyLock::force(&QUALITY_LEVEL_HOLDER)
}

/// IDA 0xb33c..0xb4f8: `RBX::CRenderSettings` slots read by this file's getters.
/// IDA 0x97d0 constructs the settings subobject at item offset +96
/// (`RBX::CRenderSettings::CRenderSettings((char *)this + 96)`), so settings
/// offset +N is item offset +96+N: settings +4 == item +0x64 (`graphics_mode`),
/// +0x10 == +0x70, +0x14 == +0x74, +0x3B == +0x9B, +0x3D == +0x9D.
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
    /// +0x1C dword. IDA 0x9aec (via the 0x9ae8 thunk) `LDR R2,[R0,#0x1C]`.
    pub auto_quality_level: i32,
    /// +0x28 byte, zero-extended into R0. IDA 0xb46c `LDRB.W R0,[R0,#0x28]`.
    pub debug_show_bounding_boxes: bool,
    /// +0x3A byte, zero-extended into R0. IDA 0xb3e0 `LDRB.W R0,[R0,#0x3A]`.
    pub show_aggregation: bool,
    /// +0x3B byte, zero-extended into R0. IDA 0xb3b4 `LDRB.W R0,[R0,#0x3B]`.
    pub always_draw_connectors: bool,
    /// +0x3D byte, zero-extended into R0. IDA 0x9b08 `LDRB.W R2,[R0,#0x9D]` on the item.
    pub eager_bulk_execution: bool,
}

/// IDA 0x9922 `GetDXVideoMemorySize()`. The host has no DX video memory
/// query, so this reports 0 and the ctor takes the low-budget arm [INFERENCE].
fn get_dx_video_memory_size() -> u32 {
    0
}

/// IDA 0x97d0: `GetDXVideoMemorySize() > (&loc_F423FC + 3)` selects
/// `50332672`, else `39322400`; stored at item +146.
fn video_memory_budget() -> u32 {
    // IDA 0x97d0 compares against `&loc_F423FC + 3` (address-as-constant).
    const VIDEO_MEMORY_THRESHOLD: u32 = 0xF423FF;
    if get_dx_video_memory_size() > VIDEO_MEMORY_THRESHOLD {
        50332672
    } else {
        39322400
    }
}

/// IDA 0x9668: `LDRB` + `CBNZ`/`MOVNE` folds any nonzero flag byte to 1.
/// Fields here are already `bool`, so this documents the original fold.
fn normalize_flag(value: bool) -> i32 {
    i32::from(value)
}

// 0x84e0 — start
// type: void __fastcall __noreturn(int, int, int, int, int argc, char *argv)
// IDA 0x84e0..0x8508 (`start`, ARM): `envp = &argv[argc + 1]` (0x84e0..0x84f4);
// skip past the terminating null (0x84f8 `LDR R4,[R3],#4` / 0x84fc `CMP` /
// 0x8500 `BNE`); `exit(main(argc, argv, envp))` (0x8504 `BLX _main`, 0x8508 `B _exit`).
#[doc(alias = "start")]
pub fn stub_0x84e0(argc: usize, argv: *const *const core::ffi::c_char) -> ! {
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

// 0x850c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEEC2Ev
// type: int __fastcall(int)
// IDA 0x850c: base `EnumDescriptor(this, "AASamples", typeinfo)` (0x8542),
// vtable `off_1221308` (0x855a), empty tables, then the pairs below.
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEEC2Ev")]
pub fn stub_0x850c() -> RenderEnumDesc {
    let mut desc = RenderEnumDesc::new("AASamples");
    desc.add_pair(1, "None");
    desc.add_pair(4, "4");
    desc.add_pair(8, "8");
    desc
}

// 0x86d0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEEC2Ev
// type: int __fastcall(int)
// IDA 0x86d0: base `EnumDescriptor(this, "GraphicsMode", typeinfo)` (0x8706),
// vtable `off_1221338` (0x871e), empty tables, then the pairs below.
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEEC2Ev")]
pub fn stub_0x86d0() -> RenderEnumDesc {
    let mut desc = RenderEnumDesc::new("GraphicsMode");
    desc.add_pair(1, "Automatic");
    desc.add_pair(3, "Direct3D");
    desc.add_pair(4, "OpenGL");
    desc.add_pair(5, "NoGraphics");
    desc
}

// 0x88c4 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEC2Ev
// type: int __fastcall(int)
// IDA 0x88c4: base `EnumDescriptor(this, "FramerateManagerMode", typeinfo)`
// (0x88fa) — note the original string spells "Framerate", unlike the type
// name `FrameRateManagerMode`; vtable `off_1221368` (0x8912), then pairs.
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEC2Ev")]
pub fn stub_0x88c4() -> RenderEnumDesc {
    let mut desc = RenderEnumDesc::new("FramerateManagerMode");
    desc.add_pair(0, "Automatic");
    desc.add_pair(1, "On");
    desc.add_pair(2, "Off");
    desc
}

// 0x8a88 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEEC2Ev
// type: int __fastcall(int)
// IDA 0x8a88: base `EnumDescriptor(this, "Antialiasing", typeinfo)` (0x8abe),
// vtable `off_1221398` (0x8ad6), then the pairs below in original order.
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEEC2Ev")]
pub fn stub_0x8a88() -> RenderEnumDesc {
    let mut desc = RenderEnumDesc::new("Antialiasing");
    desc.add_pair(0, "Automatic");
    desc.add_pair(2, "Off");
    desc.add_pair(1, "On");
    desc
}

// 0x8c4c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEEC2Ev
// type: int __fastcall(int)
// IDA 0x8c4c: base `EnumDescriptor(this, "Shadow", typeinfo)` (0x8c82),
// vtable `off_12213C8` (0x8c9a), then the pairs below in original order.
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEEC2Ev")]
pub fn stub_0x8c4c() -> RenderEnumDesc {
    let mut desc = RenderEnumDesc::new("Shadow");
    desc.add_pair(0, "Automatic");
    desc.add_pair(1, "All");
    desc.add_pair(3, "CharacterOnly");
    desc.add_pair(2, "Off");
    desc
}

// 0x8e24 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEEC2Ev
// type: RBX::Reflection::EnumDescriptor *__fastcall(RBX::Reflection::EnumDescriptor *)
// IDA 0x8e24: base `EnumDescriptor(this, "QualityLevel", typeinfo)`, vtable
// `off_12213F8`, empty tables; `addPair(0, "Automatic")`, then
// `for (i = 1; i < 22; ++i) addPair(i, format("Level%.2d", i))`, then the
// `Level NN` wide aliases (`snprintf("%2u")` + `RBX::Name::declare` +
// `std::map::operator[]`, `do { ... } while (v21 + 1 < 22)`).
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEEC2Ev")]
pub fn stub_0x8e24() -> RenderEnumDesc {
    let mut desc = RenderEnumDesc::new("QualityLevel");
    desc.add_pair(0, "Automatic");
    for i in 1..22 {
        desc.add_pair(i, &format!("Level{i:02}"));
    }
    for i in 1..22 {
        desc.add_alias(&format!("Level {i:2}"), i);
    }
    desc
}

// 0x9100 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEEC2Ev
// type: RBX::Reflection::EnumDescriptor *__fastcall(RBX::Reflection::EnumDescriptor *)
// IDA 0x9100: base `EnumDescriptor(this, "Resolution", typeinfo)`, vtable
// `off_1221428`, empty tables; `addPair(0..18, ...)` interleaved with the
// `(wide)` aliases (`RBX::Name::declare` + `std::map::operator[]`).
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEEC2Ev")]
pub fn stub_0x9100() -> RenderEnumDesc {
    let mut desc = RenderEnumDesc::new("Resolution");
    desc.add_pair(0, "Automatic");
    desc.add_pair(1, "720x526");
    desc.add_pair(2, "800x600");
    desc.add_pair(3, "1024x600");
    desc.add_alias("1024x600 (wide)", 3);
    desc.add_pair(4, "1024x768");
    desc.add_pair(5, "1280x720");
    desc.add_alias("1280x720 (wide)", 5);
    desc.add_pair(6, "1280x768");
    desc.add_alias("1280x768 (wide)", 6);
    desc.add_pair(7, "1152x864");
    desc.add_pair(8, "1280x800");
    desc.add_alias("1280x800 (wide)", 8);
    desc.add_pair(9, "1360x768");
    desc.add_alias("1360x768 (wide)", 9);
    desc.add_pair(10, "1280x960");
    desc.add_pair(11, "1280x1024");
    desc.add_pair(12, "1440x900");
    desc.add_alias("1440x900 (wide)", 12);
    desc.add_pair(13, "1600x900");
    desc.add_alias("1600x900 (wide)", 13);
    desc.add_pair(14, "1600x1024");
    desc.add_alias("1600x1024 (wide)", 14);
    desc.add_pair(15, "1600x1200");
    desc.add_pair(16, "1680x1050");
    desc.add_alias("1680x1050 (wide)", 16);
    desc.add_pair(17, "1920x1080");
    desc.add_alias("1920x1080 (wide)", 17);
    desc.add_pair(18, "1920x1200");
    desc.add_alias("1920x1200 (wide)", 18);
    desc
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
// IDA 0x97d0: base `GlobalAdvancedSettingsItem` C2 (0x97f0, owned by 0xb4fc),
// `CRenderSettings::CRenderSettings(this + 96)` (0x9828, settings-subobject
// defaults), item vtables, +168 string = empty (0x9876), +172/+174 =
// 800/600 (0x987e/0x988a), +176 vector = empty (0x9896..0x98aa), +189 byte =
// 1 (0x98b0), signal safe-static init (0x98d0..0x98d8, owned by `Signal`),
// `+28` virtual with `std::string("Rendering")` (0x98f6/0x9904),
// `push_back(+176, first)` (0x991a), video-memory threshold select at +146
// (0x9922..0x9946).
#[doc(alias = "CRenderSettingsItem::CRenderSettingsItem(void)")]
#[doc(alias = "__ZN19CRenderSettingsItemC2Ev")]
pub fn stub_0x97d0(this: *mut CRenderSettingsItem) -> *mut CRenderSettingsItem {
    // SAFETY: `this` must point to valid uninitialized item storage.
    unsafe {
        // IDA 0x97f0: base-class C2 state is owned by 0xb4fc (still a stub);
        // `Default` zeroes the modelled item-side mirrors the same way the
        // 0x9896..0x98aa stores zero the vector and the flag stores start.
        core::ptr::write(this, CRenderSettingsItem::default());
        let item = &mut *this;
        // IDA 0x9828: `CRenderSettings::CRenderSettings(this + 96)` — the
        // settings C2 owns the +96-subobject defaults (separate EA).
        // IDA 0x9876: +168 string = empty.
        item.string_168 = String::new();
        // IDA 0x987e/0x988a: +172/+174 lanes = 800/600.
        let first = Vector2int16 { x: 800, y: 600 };
        item.first_resolution = first;
        // IDA 0x9896..0x98aa: +176 vector = empty.
        item.resolutions = Vec::new();
        // IDA 0x98b0: +189 byte = 1.
        item.byte_189 = true;
        // IDA 0x98d0/0x98d8: signal safe-static mutex init — owned by `Signal`.
        // IDA 0x98f6/0x9904: `+28` virtual call with `std::string("Rendering")`.
        item.render_category = "Rendering".to_owned();
        // IDA 0x991a: `std::vector<G3D::Vector2int16>::push_back(+176, first)`
        // (0xb740, still a stub — `Vec::push` is the mapping).
        item.resolutions.push(first);
        // IDA 0x9922..0x9946: `GetDXVideoMemorySize()` threshold select
        // stored at item +146.
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
    // IDA 0x9b2c: `attributes: thunk` into `std::logic_error::~logic_error`;
    // drops are compiler-managed in Rust — no explicit body.
}

// 0x9b30 — __ZNSt12out_of_rangeD0Ev
// type: void __cdecl(std::out_of_range *__hidden this)
#[doc(alias = "std::out_of_range::~out_of_range()")]
#[doc(alias = "__ZNSt12out_of_rangeD0Ev")]
pub fn stub_0x9b30() {
    // IDA 0x9b30: D0 deleting destructor (`logic_error` base dtor at 0x9b36 +
    // `operator delete` at 0x9b40); drops and storage reclaim are
    // compiler-managed in Rust — no explicit body.
}

// 0x9b44 — __ZNSt12out_of_rangeD2Ev
// type: void __cdecl(std::out_of_range *__hidden this)
#[doc(alias = "std::out_of_range::~out_of_range()")]
#[doc(alias = "__ZNSt12out_of_rangeD2Ev")]
pub fn stub_0x9b44() {
    // IDA 0x9b44: `attributes: thunk` into `std::logic_error::~logic_error`;
    // drops are compiler-managed in Rust — no explicit body.
}

// 0x9b48 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::addPair(RBX::CRenderSettings::AASamples,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE7addPairES3_PKc")]
pub fn stub_0x9b48(desc: &mut RenderEnumDesc, value: i32, name: &str) {
    // IDA 0x9b48: Item alloc + `Descriptor` C2 + `push_back` (0x9b7e..0x9bee),
    // value-vector grow (0x9c04..0x9c22), `value>=0` / `value<=2304` asserts
    // (0x9c2a..0x9cba), ordinal/name/item maps (0x9ccc..0x9df6); all collapse
    // into the pair-table push below.
    desc.add_pair(value, name);
}

// 0x9ea8 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::addPair(RBX::CRenderSettings::GraphicsMode,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE7addPairES3_PKc")]
pub fn stub_0x9ea8(desc: &mut RenderEnumDesc, value: i32, name: &str) {
    // IDA 0x9ea8: same `addPair` body shape as 0x9b48 (Item alloc, vector
    // grows, asserts, maps); collapses into the pair-table push below.
    desc.add_pair(value, name);
}

// 0xa208 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE9addLegacyEiPKcS3_
// type: _DWORD *__fastcall(int, unsigned int, int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::addLegacy(int,char const*,RBX::CRenderSettings::GraphicsMode)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE9addLegacyEiPKcS3_")]
pub fn stub_0xa208(desc: &mut RenderEnumDesc, legacy_index: usize, name: &str, value: i32) {
    // IDA 0xa208: grow the legacy vector at +132 (0xa22a..0xa23a), then
    // `RBX::Name::declare` + `std::map::operator[]` at +72 (0xa244..0xa250);
    // the original returns the map slot, dropped here.
    desc.add_legacy(legacy_index, name, value);
}

// 0xa25c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::addPair(RBX::CRenderSettings::FrameRateManagerMode,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE7addPairES3_PKc")]
pub fn stub_0xa25c(desc: &mut RenderEnumDesc, value: i32, name: &str) {
    // IDA 0xa25c: same `addPair` body shape as 0x9b48; collapses into the
    // pair-table push below.
    desc.add_pair(value, name);
}

// 0xa5bc — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::addPair(RBX::CRenderSettings::AntialiasingMode,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE7addPairES3_PKc")]
pub fn stub_0xa5bc(desc: &mut RenderEnumDesc, value: i32, name: &str) {
    // IDA 0xa5bc: same `addPair` body shape as 0x9b48; collapses into the
    // pair-table push below.
    desc.add_pair(value, name);
}

// 0xa91c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::addPair(RBX::CRenderSettings::ShadowMode,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE7addPairES3_PKc")]
pub fn stub_0xa91c(desc: &mut RenderEnumDesc, value: i32, name: &str) {
    // IDA 0xa91c: same `addPair` body shape as 0x9b48; collapses into the
    // pair-table push below.
    desc.add_pair(value, name);
}

// 0xac7c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::addPair(RBX::CRenderSettings::QualityLevel,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE7addPairES3_PKc")]
pub fn stub_0xac7c(desc: &mut RenderEnumDesc, value: i32, name: &str) {
    // IDA 0xac7c: same `addPair` body shape as 0x9b48; collapses into the
    // pair-table push below.
    desc.add_pair(value, name);
}

// 0xafdc — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::addPair(RBX::CRenderSettings::ResolutionPreset,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE7addPairES3_PKc")]
pub fn stub_0xafdc(desc: &mut RenderEnumDesc, value: i32, name: &str) {
    // IDA 0xafdc: same `addPair` body shape as 0x9b48; collapses into the
    // pair-table push below.
    desc.add_pair(value, name);
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

// 0xb340 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEED1Ev")]
pub fn stub_0xb340() {
    // IDA 0xb340: D1 destructor (vtable reset + conditional `operator delete`
    // of the heap word at +11, 0xb354..0xb35c); drops are compiler-managed
    // in Rust — no explicit body.
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

// 0xb368 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEED1Ev")]
pub fn stub_0xb368() {
    // IDA 0xb368: D1 destructor (vtable reset + conditional `operator delete`
    // of the heap word at +11, 0xb37c..0xb384); drops are compiler-managed
    // in Rust — no explicit body.
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

// 0xb390 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEED1Ev")]
pub fn stub_0xb390() {
    // IDA 0xb390: D1 destructor (vtable reset + conditional `operator delete`
    // of the heap word at +11, 0xb3a4..0xb3ac); drops are compiler-managed
    // in Rust — no explicit body.
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

// 0xb3bc — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembED1Ev")]
pub fn stub_0xb3bc() {
    // IDA 0xb3bc: D1 destructor (vtable reset + conditional `operator delete`
    // of the heap word at +10, 0xb3d0..0xb3d8); drops are compiler-managed
    // in Rust — no explicit body.
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

// 0xb3f8 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEED1Ev")]
pub fn stub_0xb3f8() {
    // IDA 0xb3f8: D1 destructor (vtable reset + conditional `operator delete`
    // of the heap word at +11, 0xb40c..0xb414); drops are compiler-managed
    // in Rust — no explicit body.
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

// 0xb420 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEED1Ev")]
pub fn stub_0xb420() {
    // IDA 0xb420: D1 destructor (vtable reset + conditional `operator delete`
    // of the heap word at +11, 0xb434..0xb43c); drops are compiler-managed
    // in Rust — no explicit body.
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

// 0xb448 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEED1Ev")]
pub fn stub_0xb448() {
    // IDA 0xb448: D1 destructor (vtable reset + conditional `operator delete`
    // of the heap word at +11, 0xb45c..0xb464); drops are compiler-managed
    // in Rust — no explicit body.
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

// 0xb478 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiED1Ev")]
pub fn stub_0xb478() {
    // IDA 0xb478: D1 destructor (vtable reset + conditional `operator delete`
    // of the heap word at +10, 0xb48c..0xb494); drops are compiler-managed
    // in Rust — no explicit body.
}

// 0xb4a8 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEED1Ev")]
pub fn stub_0xb4a8() {
    // IDA 0xb4a8: D1 destructor (vtable reset at 0xb4bc + conditional
    // `operator delete` of the heap word at +11/0x2C, 0xb4be..0xb4c4);
    // drops are compiler-managed in Rust — no explicit body.
}

// 0xb4d0 — __ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EED1Ev")]
pub fn stub_0xb4d0() {
    // IDA 0xb4d0: D1 destructor (vtable reset to `FunctionDescriptor`+8 at
    // 0xb4e8, then `std::_List_base<SignatureDescriptor::Item>::_M_clear`
    // of the list at +8, 0xb4ec); drops are compiler-managed in Rust —
    // no explicit body.
}

// 0xb4fc — __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEEC2Ev
// type: RBX::Instance *__fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEEC2Ev")]
pub fn stub_0xb4fc(this: *mut CRenderSettingsItem) -> *mut CRenderSettingsItem {
    // SAFETY: `this` must point to valid base-subobject storage. IDA 0x97d0
    // calls into this C2 for its base-class state (0x97f0).
    unsafe {
        // IDA 0xb51e `Instance::Instance(this, 0)`, vtable installs
        // (0xb54e..0xb564, 0xb5c2..0xb5e2), `classDescriptor()` (0xb584),
        // `describedClassDescriptor` store + `registrar++` (0xb5a2..0xb5b2):
        // harness-owned, no modelled state.
        // IDA 0xb5ec..0xb5f8 `setName(std::string("RenderSettings"))`: the
        // `Instance` name, not an item field — noted, not stored.
        // IDA 0xb622/0xb670/0xb688..0xb6b4: `runtime_error("singleton %s
        // already exists")` when the singleton slot is already set.
        if RENDER_SETTINGS_SINGLETON.load(Ordering::SeqCst) {
            panic!("singleton RenderSettings already exists");
        }
        // IDA 0xb5ba: `STRB.W R3,[R1,#0x5C]` — base-ready byte.
        (*this).global_base_ready = true;
        // IDA 0xb626: `sing = this`.
        RENDER_SETTINGS_SINGLETON.store(true, Ordering::SeqCst);
        this
    }
}

// 0xb740 — __ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE9push_backERKS1_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::push_back(G3D::Vector2int16 const&)")]
#[doc(alias = "__ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE9push_backERKS1_")]
pub fn stub_0xb740<'v>(vec: &'v mut Vec<Vector2int16>, value: &Vector2int16) -> &'v mut Vec<Vector2int16> {
    // IDA 0xb740: `LDRD R4,R5,[R0,#4]` loads the finish/capacity pair
    // (0xb742); the fast path (0xb74e..0xb75c) copies one 4-byte lane
    // (`LDR R1,[R2]` / `STR R1,[R4]`) and bumps finish by 4, while the full
    // arm tail-calls `_M_insert_aux` (0xb760..0xb766, reallocation).
    // `Vec::push` owns both arms; the original returns `this`.
    vec.push(*value);
    vec
}

// 0xb76c — __ZN3rbx7signals16signal_with_argsILi1EFvPKN3RBX10Reflection18PropertyDescriptorEEEclES6_
// type: void __fastcall(_DWORD *, int, int, const void *, int, int, int, int, void *, int)
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Reflection::PropertyDescriptor const*)>::operator()(RBX::Reflection::PropertyDescriptor const*)")]
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFvPKN3RBX10Reflection18PropertyDescriptorEEEclES6_")]
pub fn stub_0xb76c(signal: &Signal<&'static str>, desc_name: &'static str) {
    // IDA 0xb76c `signal_with_args<1, void(const PropertyDescriptor*)>::
    // operator()`: 0xb79c early-outs on an empty head; 0xb7ce gates on
    // `FLog::SignalPrints` (no `FLog` in this harness — dropped); slots are
    // walked via `next()` with a per-slot call (0xb7e6..0xb80a, slot throws
    // are routed to `on_error`, 0xb840..0xb85a), then `intrusive_ptr_release`
    // (0xb80c..0xb812). `rbx::signals::signal` maps to
    // `rbx_core::signal::Signal` (AGENTS.md §4); `Signal::fire` owns the
    // walk, and Rust slots cannot throw across the boundary, so there is no
    // `on_error` arm.
    signal.fire(desc_name);
}

// 0xb8b8 — __ZN19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "CRenderSettingsItem::~CRenderSettingsItem()")]
#[doc(alias = "__ZN19CRenderSettingsItemD1Ev")]
pub fn stub_0xb8b8() {
    // IDA 0xb8b8: D1 destructor, `attributes: thunk` — a single `B.W` to
    // the D2 body; drops are compiler-managed in Rust — no explicit body.
}

// 0xb8bc — __ZN19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "CRenderSettingsItem::~CRenderSettingsItem()")]
#[doc(alias = "__ZN19CRenderSettingsItemD0Ev")]
pub fn stub_0xb8bc() {
    // IDA 0xb8bc: D0 deleting destructor — D2 call (0xb8c2) + `operator
    // delete` (0xb8cc); drops and storage reclaim are compiler-managed in
    // Rust — no explicit body.
}

// 0xb8e0 — __ZThn32_N19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
#[doc(alias = "__ZThn32_N19CRenderSettingsItemD1Ev")]
pub fn stub_0xb8e0(this: *mut u8) -> *mut u8 {
    // SAFETY: `this` must point to a `CRenderSettingsItem` viewed through a
    // base subobject 0x20 bytes in.
    // IDA 0xb8e0: `SUBS R0,#0x20`, then `B.W` to the D2 body (0xb8e2); the
    // D2 tail-call is Drop-owned, only the adjustment is modelled.
    unsafe { this.sub(0x20) }
}

// 0xb8e8 — __ZThn32_N19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
#[doc(alias = "__ZThn32_N19CRenderSettingsItemD0Ev")]
pub fn stub_0xb8e8(this: *mut u8) -> *mut u8 {
    // SAFETY: `this` must point to a `CRenderSettingsItem` viewed through a
    // base subobject 0x20 bytes in.
    // IDA 0xb8e8: D0 variant — `SUB.W R4,R0,#0x20` (0xb8ea), D2 call
    // (0xb8f2), `operator delete` (0xb8fc); Drop and reclaim are
    // compiler-owned, only the adjustment is modelled.
    unsafe { this.sub(0x20) }
}

// 0xb910 — __ZThn36_N19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
#[doc(alias = "__ZThn36_N19CRenderSettingsItemD1Ev")]
pub fn stub_0xb910(this: *mut u8) -> *mut u8 {
    // SAFETY: `this` must point to a `CRenderSettingsItem` viewed through a
    // base subobject 0x24 bytes in.
    // IDA 0xb910: `SUBS R0,#0x24`, then `B.W` to the D2 body (0xb912); the
    // D2 tail-call is Drop-owned, only the adjustment is modelled.
    unsafe { this.sub(0x24) }
}

// 0xb918 — __ZThn36_N19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
#[doc(alias = "__ZThn36_N19CRenderSettingsItemD0Ev")]
pub fn stub_0xb918(this: *mut u8) -> *mut u8 {
    // SAFETY: `this` must point to a `CRenderSettingsItem` viewed through a
    // base subobject 0x24 bytes in.
    // IDA 0xb918: D0 variant — `SUB.W R4,R0,#0x24` (0xb91a), D2 call
    // (0xb922), `operator delete` (0xb92c); Drop and reclaim are
    // compiler-owned, only the adjustment is modelled.
    unsafe { this.sub(0x24) }
}

// 0xb934 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED1Ev")]
pub fn stub_0xb934() {
    // IDA 0xb934: D1 destructor, `attributes: thunk` — a single `B.W` to
    // the D2 body; drops are compiler-managed in Rust — no explicit body.
}

// 0xb938 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED0Ev")]
pub fn stub_0xb938() {
    // IDA 0xb938: D0 deleting destructor — D2 call (0xb93e) + `operator
    // delete` (0xb948); drops and storage reclaim are compiler-managed in
    // Rust — no explicit body.
}

// 0xb94c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupEPKc")]
pub fn stub_0xb94c(desc: &RenderEnumDesc, name: &str) -> i32 {
    // IDA 0xb94c: `RBX::Name::lookup` (0xb958) interns the name, then
    // `convertToValue(Name, value)` (0xb966); on 1 the result is
    // `convertToItem(value)` (0xb972), else 0 is returned (0xb968/0xb976).
    // `Name` interning collapses into the `str` probe and both converts
    // into the pair/alias table lookup below (`convertToItem` is the
    // identity into R0).
    desc.lookup_value(name).unwrap_or(0)
}

// 0xb97c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupERKNS0_7VariantE")]
pub fn stub_0xb97c(desc: &RenderEnumDesc, value: i32) -> i32 {
    // IDA 0xb97c: `rbx::any_cast<const AASamples&, Region3>(variant + 4)`
    // (0xb984..0xb98e, a direct `i32` load through the placement) then the
    // `convertToItem` tail-call (0xb990..0xb998, identity into R0).
    let _ = desc;
    value
}

// 0xb99c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_0xb99c(desc: &RenderEnumDesc, index: usize, out: &mut i32) -> bool {
    // IDA 0xb99c (disasm-only; decompilation failed): the count at +0x28 is
    // compared against the index (0xb9a4..0xb9aa); when below, the value
    // loads from the +0x90 table (`LDR [R0,R1,LSL#2]`, 0xb9ac..0xb9b0),
    // stages on the stack (0xb9b4) and the 1-arm is taken (0xb9b6, R5 = 1).
    // The `Singleton::singleton`/`doGetSingleton` round-trip (0xb9b8..0xb9e4,
    // one-time descriptor registration via `boost::call_once`) has no
    // modelled effect; the placement write + `placement_any::operator=`
    // (0xb9e6..0xb9ec) collapse into the `out` store. Returns R5.
    // The +0x90 value table grows alongside the pair table (`addPair`
    // 0x9b48), so `pairs[index].0` is the loaded value.
    if let Some((value, _)) = desc.pairs.get(index) {
        *out = *value;
        true
    } else {
        false
    }
}

// 0xb9f8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE15convertToStringEmRSs")]
pub fn stub_0xb9f8(desc: &RenderEnumDesc, index: usize, out: &mut String) -> bool {
    // IDA 0xb9f8: the count at +0x28 bounds-checks the index (0xba4c); on a
    // hit the value loads from the +0x90 table (0xba5c), renders through the
    // `convertToString(string*, value)` overload into a temp (0xba66),
    // assigns into `out` (0xba72) and returns 1 (0xba7a/0xbad4); on a miss
    // returns 0 (0xba30/0xbaaa). The temp `std::string` refcount dance
    // (0xba84..0xbad0) is `String`-owned.
    if let Some((value, _)) = desc.pairs.get(index) {
        if let Some(name) = desc.lookup_name(*value) {
            *out = name.to_owned();
            return true;
        }
    }
    false
}

// 0xbb3c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED1Ev")]
pub fn stub_0xbb3c() {
    // IDA 0xbb3c: D1 destructor, `attributes: thunk` — a single `B.W` to
    // the D2 body; drops are compiler-managed in Rust — no explicit body.
}

// 0xbb40 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED0Ev")]
pub fn stub_0xbb40() {
    // IDA 0xbb40: D0 deleting destructor — D2 call (0xbb46) + `operator
    // delete` (0xbb50); drops and storage reclaim are compiler-managed in
    // Rust — no explicit body.
}

// 0xbb54 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE6lookupEPKc")]
pub fn stub_0xbb54(desc: &RenderEnumDesc, name: &str) -> i32 {
    // IDA 0xbb54: same `lookup` body shape as 0xb94c — `RBX::Name::lookup`
    // (0xbb60), `convertToValue` (0xbb6e), `convertToItem` on 1 (0xbb7a),
    // else 0 (0xbb70/0xbb7e); collapses into the table probe below.
    desc.lookup_value(name).unwrap_or(0)
}

// 0xbb84 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE6lookupERKNS0_7VariantE")]
pub fn stub_0xbb84(desc: &RenderEnumDesc, value: i32) -> i32 {
    // IDA 0xbb84: same `lookup` body shape as 0xb97c — `rbx::any_cast`
    // (0xbb8c..0xbb96) then the `convertToItem` tail-call (0xbb98..0xbba0,
    // identity into R0).
    let _ = desc;
    value
}

// 0xbba4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_0xbba4(desc: &RenderEnumDesc, index: usize, out: &mut i32) -> bool {
    // IDA 0xbba4 (disasm-only; decompilation failed): same `convertToValue`
    // body shape as 0xb99c — +0x28 count check (0xbbac..0xbbb2), +0x90 table
    // load (0xbbb4..0xbbbc, 1-arm 0xbbbe), `Singleton`/`call_once` round-trip
    // (0xbbc0..0xbbec, no modelled effect), placement write +
    // `placement_any::operator=` (0xbbee..0xbbf4). Returns R5 (0xbbf8).
    if let Some((value, _)) = desc.pairs.get(index) {
        *out = *value;
        true
    } else {
        false
    }
}

// 0xbc00 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringEmRSs")]
pub fn stub_0xbc00(desc: &RenderEnumDesc, index: usize, out: &mut String) -> bool {
    // IDA 0xbc00: same `convertToString` body shape as 0xb9f8 — +0x28
    // bounds check (0xbc54), +0x90 table load (0xbc64), temp render via the
    // value overload (0xbc6e), assign (0xbc72), 1 on hit / 0 on miss.
    if let Some((value, _)) = desc.pairs.get(index) {
        if let Some(name) = desc.lookup_name(*value) {
            *out = name.to_owned();
            return true;
        }
    }
    false
}

// 0xbd44 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED1Ev")]
pub fn stub_0xbd44() {
    // IDA 0xbd44: D1 destructor, `attributes: thunk` — a single `B.W` to
    // the D2 body; drops are compiler-managed in Rust — no explicit body.
}

// 0xbd48 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED0Ev")]
pub fn stub_0xbd48() {
    // IDA 0xbd48: D0 deleting destructor — D2 call (0xbd4e) + `operator
    // delete` (0xbd58); drops and storage reclaim are compiler-managed in
    // Rust — no explicit body.
}

// 0xbd5c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupEPKc")]
pub fn stub_0xbd5c(desc: &RenderEnumDesc, name: &str) -> i32 {
    // IDA 0xbd5c: same `lookup` body shape as 0xb94c — `RBX::Name::lookup`
    // (0xbd68), `convertToValue` (0xbd76), `convertToItem` on 1 (0xbd82),
    // else 0 (0xbd78/0xbd86); collapses into the table probe below.
    desc.lookup_value(name).unwrap_or(0)
}

// 0xbd8c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupERKNS0_7VariantE")]
pub fn stub_0xbd8c(desc: &RenderEnumDesc, value: i32) -> i32 {
    // IDA 0xbd8c: same `lookup` body shape as 0xb97c — `rbx::any_cast`
    // (0xbd94..0xbd9e) then the `convertToItem` tail-call (0xbda0..0xbda8,
    // identity into R0).
    let _ = desc;
    value
}

// 0xbdac — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_0xbdac(desc: &RenderEnumDesc, index: usize, out: &mut i32) -> bool {
    // IDA 0xbdac (disasm-only; decompilation failed): same `convertToValue`
    // body shape as 0xb99c — +0x28 count check (0xbdb4..0xbdb8), +0x90 table
    // load (0xbdbc..0xbdc4, 1-arm 0xbdc6), `Singleton`/`call_once` round-trip
    // (0xbdc8..0xbdf4, no modelled effect), placement write +
    // `placement_any::operator=` (0xbdf6..0xbdfc). Returns R5 (0xbe00).
    if let Some((value, _)) = desc.pairs.get(index) {
        *out = *value;
        true
    } else {
        false
    }
}

// 0xbe08 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE15convertToStringEmRSs")]
pub fn stub_0xbe08(desc: &RenderEnumDesc, index: usize, out: &mut String) -> bool {
    // IDA 0xbe08: same `convertToString` body shape as 0xb9f8 — +0x28
    // bounds check (0xbe5c), +0x90 table load (0xbe6c), temp render via the
    // value overload (0xbe76), assign (0xbe82), 1 on hit (0xbe8a) / 0 on
    // miss (0xbe40/0xbeba).
    if let Some((value, _)) = desc.pairs.get(index) {
        if let Some(name) = desc.lookup_name(*value) {
            *out = name.to_owned();
            return true;
        }
    }
    false
}

// 0xbf4c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED1Ev")]
pub fn stub_0xbf4c() {
    // IDA 0xbf4c: D1 destructor, `attributes: thunk` — a single `B.W` to
    // the D2 body; drops are compiler-managed in Rust — no explicit body.
}

// 0xbf50 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED0Ev")]
pub fn stub_0xbf50() {
    // IDA 0xbf50: D0 deleting destructor — D2 call (0xbf56) + `operator
    // delete` (0xbf60); drops and storage reclaim are compiler-managed in
    // Rust — no explicit body.
}

// 0xbf64 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE6lookupEPKc")]
pub fn stub_0xbf64(desc: &RenderEnumDesc, name: &str) -> i32 {
    // IDA 0xbf64: same `lookup` body shape as 0xb94c — `RBX::Name::lookup`
    // (0xbf70), `convertToValue` (0xbf7a), `convertToItem` on 1 (0xbf8a),
    // else 0 (0xbf80/0xbf84); collapses into the table probe below.
    desc.lookup_value(name).unwrap_or(0)
}

// 0xbf94 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE6lookupERKNS0_7VariantE")]
pub fn stub_0xbf94(desc: &RenderEnumDesc, value: i32) -> i32 {
    // IDA 0xbf94: same `lookup` body shape as 0xb97c — `rbx::any_cast`
    // (0xbf9e, `LDR` 0xbfa2) then the `convertToItem` tail-call (0xbfaa,
    // identity into R0).
    let _ = desc;
    value
}

// 0xbfb4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_0xbfb4(desc: &RenderEnumDesc, index: usize, out: &mut i32) -> bool {
    // IDA 0xbfb4: same `convertToValue` body shape as 0xb99c — +0x28 count
    // check (0xbfbc..0xbfc0), +0x90 table load (0xbfc4..0xbfc8, 1-arm 0xbfce),
    // `Singleton`/`call_once` round-trip (0xbfe8/0xbfec, no modelled effect),
    // placement write + `placement_any::operator=` (0xbffe..0xc004).
    // Returns R5.
    if let Some((value, _)) = desc.pairs.get(index) {
        *out = *value;
        true
    } else {
        false
    }
}

// 0xc010 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE15convertToStringEmRSs")]
pub fn stub_0xc010(desc: &RenderEnumDesc, index: usize, out: &mut String) -> bool {
    // IDA 0xc010: same `convertToString` body shape as 0xb9f8 — +0x28 count
    // load (0xc03c), bounds check, +0x90 table load, temp render via the
    // value overload, assign, 1 on hit / 0 on miss.
    if let Some((value, _)) = desc.pairs.get(index) {
        if let Some(name) = desc.lookup_name(*value) {
            *out = name.to_owned();
            return true;
        }
    }
    false
}

// 0xc154 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED1Ev")]
pub fn stub_0xc154() {
    // IDA 0xc154: D1 destructor, `attributes: thunk` — a single `B.W` to
    // the D2 body; drops are compiler-managed in Rust — no explicit body.
}

// 0xc158 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED0Ev")]
pub fn stub_0xc158() {
    // IDA 0xc158: D0 deleting destructor — D2 call (0xc15e) + `operator
    // delete` (0xc168); drops and storage reclaim are compiler-managed in
    // Rust — no explicit body.
}

// 0xc16c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE6lookupEPKc")]
pub fn stub_0xc16c(desc: &RenderEnumDesc, name: &str) -> i32 {
    // IDA 0xc16c: same `lookup` body shape as 0xb94c — `RBX::Name::lookup`
    // (0xc178), `convertToValue` (0xc182), `convertToItem` on 1 (0xc192),
    // else 0 (0xc188/0xc18c); collapses into the table probe below.
    desc.lookup_value(name).unwrap_or(0)
}

// 0xc19c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE6lookupERKNS0_7VariantE")]
pub fn stub_0xc19c(desc: &RenderEnumDesc, value: i32) -> i32 {
    // IDA 0xc19c: same `lookup` body shape as 0xb97c — `rbx::any_cast`
    // (0xc1a6, `LDR` 0xc1aa) then the `convertToItem` tail-call (0xc1b2,
    // identity into R0).
    let _ = desc;
    value
}

// 0xc1bc — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_0xc1bc(desc: &RenderEnumDesc, index: usize, out: &mut i32) -> bool {
    // IDA 0xc1bc: same `convertToValue` body shape as 0xb99c — +0x28 count
    // check (0xc1c4..0xc1c8), +0x90 table load (0xc1cc..0xc1d0, 1-arm 0xc1d6),
    // `Singleton`/`call_once` round-trip (0xc1f0/0xc1f4, no modelled effect),
    // placement write + `placement_any::operator=` (0xc206..0xc20c).
    // Returns R5.
    if let Some((value, _)) = desc.pairs.get(index) {
        *out = *value;
        true
    } else {
        false
    }
}

// 0xc218 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE15convertToStringEmRSs")]
pub fn stub_0xc218(desc: &RenderEnumDesc, index: usize, out: &mut String) -> bool {
    // IDA 0xc218: same `convertToString` body shape as 0xb9f8 — +0x28 count
    // load (0xc244), bounds check, +0x90 table load, temp render via the
    // value overload, assign, 1 on hit / 0 on miss.
    if let Some((value, _)) = desc.pairs.get(index) {
        if let Some(name) = desc.lookup_name(*value) {
            *out = name.to_owned();
            return true;
        }
    }
    false
}

// 0xc35c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED1Ev")]
pub fn stub_0xc35c() {
    // IDA 0xc35c: D1 destructor, `attributes: thunk` — a single `B.W` to
    // the D2 body; drops are compiler-managed in Rust — no explicit body.
}

// 0xc360 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED0Ev")]
pub fn stub_0xc360() {
    // IDA 0xc360: D0 deleting destructor — D2 call (0xc366) + `operator
    // delete` (0xc370); drops and storage reclaim are compiler-managed in
    // Rust — no explicit body.
}

// 0xc374 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE6lookupEPKc")]
pub fn stub_0xc374(desc: &RenderEnumDesc, name: &str) -> i32 {
    // IDA 0xc374: same `lookup` body shape as 0xb94c — `RBX::Name::lookup`
    // (0xc380), `convertToValue` (0xc38a), `convertToItem` on 1 (0xc39a),
    // else 0 (0xc390/0xc394); collapses into the table probe below.
    desc.lookup_value(name).unwrap_or(0)
}

// 0xc3a4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE6lookupERKNS0_7VariantE")]
pub fn stub_0xc3a4(desc: &RenderEnumDesc, value: i32) -> i32 {
    // IDA 0xc3a4: same `lookup` body shape as 0xb97c — `rbx::any_cast`
    // (0xc3ae, `LDR` 0xc3b2) then the `convertToItem` tail-call (0xc3ba,
    // identity into R0).
    let _ = desc;
    value
}

// 0xc3c4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_0xc3c4(desc: &RenderEnumDesc, index: usize, out: &mut i32) -> bool {
    // IDA 0xc3c4: same `convertToValue` body shape as 0xb99c — +0x28 count
    // check (0xc3cc..0xc3d0), +0x90 table load (0xc3d4..0xc3d8, 1-arm 0xc3de),
    // `Singleton`/`call_once` round-trip (0xc3f8/0xc3fc, no modelled effect),
    // placement write + `placement_any::operator=` (0xc40e..0xc414).
    // Returns R5.
    if let Some((value, _)) = desc.pairs.get(index) {
        *out = *value;
        true
    } else {
        false
    }
}

// 0xc420 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE15convertToStringEmRSs")]
pub fn stub_0xc420(desc: &RenderEnumDesc, index: usize, out: &mut String) -> bool {
    // IDA 0xc420: same `convertToString` body shape as 0xb9f8 — +0x28 count
    // load (0xc44c), bounds check, +0x90 table load, temp render via the
    // value overload, assign, 1 on hit / 0 on miss.
    if let Some((value, _)) = desc.pairs.get(index) {
        if let Some(name) = desc.lookup_name(*value) {
            *out = name.to_owned();
            return true;
        }
    }
    false
}

// 0xc564 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED1Ev")]
pub fn stub_0xc564() {
    // IDA 0xc564: D1 destructor, `attributes: thunk` — a single `B.W` to
    // the D2 body; drops are compiler-managed in Rust — no explicit body.
}

// 0xc568 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED0Ev")]
pub fn stub_0xc568() {
    // IDA 0xc568: D0 deleting destructor — D2 call (0xc56e) + `operator
    // delete` (0xc578); drops and storage reclaim are compiler-managed in
    // Rust — no explicit body.
}

// 0xc57c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE6lookupEPKc")]
pub fn stub_0xc57c(desc: &RenderEnumDesc, name: &str) -> i32 {
    // IDA 0xc57c: same `lookup` body shape as 0xb94c — `RBX::Name::lookup`
    // (0xc588), `convertToValue` (0xc592), `convertToItem` on 1 (0xc5a2),
    // else 0 (0xc598/0xc59c); collapses into the table probe below.
    desc.lookup_value(name).unwrap_or(0)
}

// 0xc5ac — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE6lookupERKNS0_7VariantE")]
pub fn stub_0xc5ac(desc: &RenderEnumDesc, value: i32) -> i32 {
    // IDA 0xc5ac: same `lookup` body shape as 0xb97c — `rbx::any_cast`
    // (0xc5b6, `LDR` 0xc5ba) then the `convertToItem` tail-call (0xc5c2,
    // identity into R0).
    let _ = desc;
    value
}

// 0xc5cc — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_0xc5cc(desc: &RenderEnumDesc, index: usize, out: &mut i32) -> bool {
    // IDA 0xc5cc: same `convertToValue` body shape as 0xb99c — +0x28 count
    // check (0xc5d4..0xc5d8), +0x90 table load (0xc5dc..0xc5e0, 1-arm 0xc5e6),
    // `Singleton`/`call_once` round-trip (0xc600/0xc604, no modelled effect),
    // placement write + `placement_any::operator=` (0xc616..0xc61c).
    // Returns R5.
    if let Some((value, _)) = desc.pairs.get(index) {
        *out = *value;
        true
    } else {
        false
    }
}

// 0xc628 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE15convertToStringEmRSs")]
pub fn stub_0xc628(desc: &RenderEnumDesc, index: usize, out: &mut String) -> bool {
    // IDA 0xc628: same `convertToString` body shape as 0xb9f8 — +0x28 count
    // load (0xc654), bounds check, +0x90 table load, temp render via the
    // value overload, assign, 1 on hit / 0 on miss.
    if let Some((value, _)) = desc.pairs.get(index) {
        if let Some(name) = desc.lookup_name(*value) {
            *out = name.to_owned();
            return true;
        }
    }
    false
}

// 0xc76c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToString(RBX::CRenderSettings::ResolutionPreset const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE15convertToStringERKS3_")]
pub fn stub_0xc76c(desc: &RenderEnumDesc, value: i32, out: &mut String) {
    // IDA 0xc76c (decompiled): `ReleaseAssert(value >= 0)` (0xc7c8..0xc818,
    // enumconverter.h:262) and `ReleaseAssert(value < enumToItem.size())`
    // (0xc82c..0xc860, :263), then `*out = value < 0 || value >= table ? ""`
    // (0xc87c/0xc888/0xc8ae/0xc8c6) `: enumToItem[value]` (0xc87e/0xc896).
    // The asserts log but fall through to the empty-string arm, modelled by
    // the `None` branch below.
    match (value >= 0).then(|| desc.lookup_name(value)).flatten() {
        Some(name) => *out = name.to_owned(),
        None => out.clear(),
    }
}

// 0xc90c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings16ResolutionPresetEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::ResolutionPreset>(RBX::CRenderSettings::ResolutionPreset const&)")]
pub fn stub_0xc90c(slot: &mut PlacementAny, value: i32) -> &mut PlacementAny {
    // IDA 0xc90c..0xc958 (decompiled): touch `singleton()` (0xc918); holder
    // already ours → copy the payload word (0xc944); else run the old
    // holder's destruct (a no-op for this trivial payload, 0xc938), clear
    // (0xc93c), copy the payload (0xc94e) and install our holder (0xc950).
    let _ = stub_0xc95c();
    if slot.holder == HOLDER_RESOLUTION_PRESET {
        slot.value = value;
    } else {
        slot.holder = 0;
        slot.value = value;
        slot.holder = HOLDER_RESOLUTION_PRESET;
    }
    slot
}

// 0xc95c — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16ResolutionPresetEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ResolutionPreset>::singleton(void)")]
pub fn stub_0xc95c() -> &'static TypedHolder {
    // IDA 0xc95c..0xc9c6 (decompiled): `__cxa_guard_acquire`-checked init —
    // `s = { typeinfo, destruct_func, construct_func }` (0xc9ae..0xc9b2) —
    // then return `&s` (0xc9c6). `LazyLock` is the `__cxa_guard` equivalent.
    LazyLock::force(&RESOLUTION_PRESET_HOLDER)
}

// 0xc9c8 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16ResolutionPresetEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ResolutionPreset>::construct_func(char const*,char *)")]
pub fn stub_0xc9c8(src: *const i32, dst: *mut i32) -> i32 {
    // IDA 0xc9c8..0xc9d0 (decompiled): `v = *src; if (dst) *dst = v; return
    // v` — trivial copy-construct of one enum word.
    // SAFETY: `src` must be readable; `dst` must be writable when non-null.
    unsafe {
        let value = *src;
        if !dst.is_null() {
            *dst = value;
        }
        value
    }
}

// 0xc9d4 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16ResolutionPresetEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ResolutionPreset>::destruct_func(char *)")]
pub fn stub_0xc9d4() {
    // IDA 0xc9d4: empty body — trivial enum payload, nothing to destroy.
}

// 0xc9d8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToItem(RBX::CRenderSettings::ResolutionPreset const&)const")]
pub fn stub_0xc9d8(desc: &RenderEnumDesc, value: i32) -> i32 {
    // IDA 0xc9d8..0xca9c (decompiled): `ReleaseAssert(value >= 0)` (:273)
    // and `ReleaseAssert(value < enumToItem.size())` (:274) — both log and
    // fall through — then `value < 0 ? 0 : value < size ? enumToItem[value]
    // : 0` (0xca84..0xca9c). The table maps each value to itself here (cf.
    // the 0xbd8c `lookup` tail-call, "identity into R0"), so a hit returns
    // the value.
    if value >= 0 && (value as usize) < desc.pairs.len() {
        value
    } else {
        0
    }
}

// 0xcaa4 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings16ResolutionPresetENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CRenderSettings::ResolutionPreset const& rbx::any_cast<RBX::CRenderSettings::ResolutionPreset const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0xcaa4(slot: &PlacementAny) -> i32 {
    // IDA 0xcaa4..0xcb4a (decompiled): holder check (0xcb10, typeinfo
    // compare) with a name-string fallback (0xcb2c,
    // "N3RBX15CRenderSettings16ResolutionPresetE"); mismatch throws
    // `rbx::bad_placement_any_cast` (0xcb62) — a throw becomes a panic here
    // (`boost::exception` maps to panics per docs/BOOST.md). Hit returns the
    // payload word (`a1 + 1`, 0xcb4a); the original returns a pointer to it,
    // copied out here.
    if slot.holder != HOLDER_RESOLUTION_PRESET {
        panic!("rbx::bad_placement_any_cast for N3RBX15CRenderSettings16ResolutionPresetE");
    }
    slot.value
}

// 0xcb94 — __ZN5boost16exception_detail12refcount_ptrINS0_20error_info_containerEED2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "boost::exception_detail::refcount_ptr<boost::exception_detail::error_info_container>::~refcount_ptr()")]
pub fn stub_0xcb94(slot: *mut *mut u8) -> *mut *mut u8 {
    // IDA 0xcb94..0xcc14 (decompiled): `if (*a1 &&
    // (*a1)->release() == 1) *a1 = 0; return a1` — `release()` lives behind
    // the pointee vtable (+16) and is unmodelled, so its result is unknowable
    // here and the slot is preserved [INFERENCE].
    // SAFETY: `slot` must point to a valid slot.
    slot
}

// 0xcc34 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToValue(RBX::Name const&,RBX::CRenderSettings::ResolutionPreset&)const")]
pub fn stub_0xcc34(desc: &RenderEnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xcc34..0xccac (decompiled): two `std::map::lower_bound` walks
    // (0xcc4a..0xcc58, then 0xcc7e..0xcc8a) with exact-match checks; hit
    // writes `*a3 = value` (0xccaa) and returns 1 (0xccac), miss returns 0.
    // Collapses into the table probe below.
    if let Some(value) = desc.lookup_value(name) {
        *out = value;
        true
    } else {
        false
    }
}

// 0xccb0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::~EnumDesc()")]
pub fn stub_0xccb0(desc: &mut RenderEnumDesc) {
    // IDA 0xccb0..0xcd48 (decompiled): D2 — item dtor loop (0xccdc..0xcce6),
    // buffer `operator delete`s (0xccee..0xcd2a), map `_M_erase`s
    // (0xcd34/0xcd3e), base `~EnumDescriptor` (0xcd48). Rust drops own the
    // storage; the tables are released eagerly to model the frees.
    desc.pairs.clear();
    desc.aliases.clear();
    desc.legacy_values.clear();
}

// 0xcd4c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToString(RBX::CRenderSettings::QualityLevel const&)const")]
pub fn stub_0xcd4c(desc: &RenderEnumDesc, value: i32, out: &mut String) {
    // IDA 0xcd4c..0xcea6 (decompiled): same `convertToString` body shape as
    // 0xc76c — `ReleaseAssert`s (:262/:263) that fall through, then
    // `*out = value < 0 || value >= table ? "" : table[value]`
    // (0xce56..0xcea6).
    match (value >= 0).then(|| desc.lookup_name(value)).flatten() {
        Some(name) => *out = name.to_owned(),
        None => out.clear(),
    }
}

// 0xceec — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings12QualityLevelEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::QualityLevel>(RBX::CRenderSettings::QualityLevel const&)")]
pub fn stub_0xceec(slot: &mut PlacementAny, value: i32) -> &mut PlacementAny {
    // IDA 0xceec..0xcf38 (decompiled): same `operator=` shape as 0xc90c for
    // the QualityLevel holder (singleton touch 0xcef8, same-holder copy
    // 0xcf24, else destruct 0xcf18 / clear 0xcf1c / copy 0xcf2e / install
    // 0xcf30).
    let _ = quality_level_holder();
    if slot.holder == HOLDER_QUALITY_LEVEL {
        slot.value = value;
    } else {
        slot.holder = 0;
        slot.value = value;
        slot.holder = HOLDER_QUALITY_LEVEL;
    }
    slot
}

#[cfg(test)]
mod batch2_tests {
    use super::*;
    use parking_lot::Mutex;
    use std::sync::Arc;

    /// Connect a recording slot; the returned handler `Arc` must stay alive
    /// (`Signal` keeps only `Weak` slots) until firing is done.
    fn fired_names(item: &CRenderSettingsItem) -> (Arc<Mutex<Vec<&'static str>>>, Arc<impl Fn(&'static str) + Send + Sync>) {
        let seen: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(Vec::new()));
        let probe = seen.clone();
        let handler = Arc::new(move |name: &'static str| {
            probe.lock().push(name);
        });
        item.property_changed.connect(handler.clone());
        (seen, handler)
    }

    #[test]
    fn eager_bulk_execution_fires_only_on_change() {
        let mut item = CRenderSettingsItem::default();
        let (seen, _handler) = fired_names(&item);
        let this = &mut item as *mut CRenderSettingsItem;
        stub_0x9b08(this, true);
        stub_0x9b08(this, true);
        stub_0x9b08(this, false);
        assert!(!unsafe { (*this).eager_bulk_execution });
        assert_eq!(*seen.lock(), vec!["EagerBulkExecution", "EagerBulkExecution"]);
    }

    #[test]
    fn auto_quality_thunk_adjusts_this_by_96() {
        let mut item = CRenderSettingsItem::default();
        let (seen, _handler) = fired_names(&item);
        // IDA 0x9ae8 views the item through a base 96 bytes in.
        let sub = unsafe { (&mut item as *mut CRenderSettingsItem as *mut u8).add(96) };
        let back = stub_0x9ae8(sub, 7);
        assert_eq!(back, sub);
        assert_eq!(item.auto_quality_level, 7);
        assert_eq!(*seen.lock(), vec!["QualityLevel"]);
    }

    #[test]
    fn settings_getters_read_their_slots() {
        let settings = CRenderSettings {
            graphics_mode: 3,
            antialiasing_mode: 1,
            shadow_mode: 2,
            frame_rate_manager_mode: 1,
            quality_level: 9,
            auto_quality_level: 4,
            debug_show_bounding_boxes: true,
            show_aggregation: true,
            always_draw_connectors: true,
            eager_bulk_execution: true,
        };
        let this = &settings as *const CRenderSettings;
        assert_eq!(stub_0xb33c(this), 3);
        assert_eq!(stub_0xb444(this), 1);
        assert_eq!(stub_0xb41c(this), 2);
        assert_eq!(stub_0xb364(this), 1);
        assert_eq!(stub_0xb38c(this), 9);
        assert_eq!(stub_0xb3b4(this), 1);
        assert_eq!(stub_0xb3e0(this), 1);
        assert_eq!(stub_0xb46c(this), 1);
        AA_SAMPLES.store(8, Ordering::SeqCst);
        assert_eq!(stub_0xb3e8(this), 8);
        AA_SAMPLES.store(0, Ordering::SeqCst);
    }

    #[test]
    fn enum_desc_pair_and_legacy_round_trip() {
        let mut desc = RenderEnumDesc::new("GraphicsMode");
        stub_0x9ea8(&mut desc, 4, "OpenGL");
        stub_0xa208(&mut desc, 2, "LegacyGL", 4);
        assert_eq!(desc.lookup_value("OpenGL"), Some(4));
        assert_eq!(desc.lookup_value("LegacyGL"), Some(4));
        assert_eq!(desc.lookup_name(4), Some("OpenGL"));
        assert_eq!(desc.legacy_values[2], 4);
    }
}

#[cfg(test)]
mod batch3_tests {
    use super::*;
    use parking_lot::Mutex;
    use std::sync::Arc;

    #[test]
    fn base_ctor_sets_ready_and_singleton() {
        let mut item = CRenderSettingsItem::default();
        assert!(!item.global_base_ready);
        let this = &mut item as *mut CRenderSettingsItem;
        assert_eq!(stub_0xb4fc(this), this);
        assert!(unsafe { (*this).global_base_ready });
        assert!(RENDER_SETTINGS_SINGLETON.load(Ordering::SeqCst));
    }

    #[test]
    fn vector_push_back_appends_and_returns_this() {
        let mut vec = Vec::new();
        let v = Vector2int16 { x: 800, y: 600 };
        // IDA 0xb740 returns `this`; the fast path copies one 4-byte lane.
        let back = stub_0xb740(&mut vec, &v);
        assert_eq!(back.len(), 1);
        assert_eq!(vec, core::slice::from_ref(&v));
    }

    #[test]
    fn signal_operator_fires_connected_slots() {
        let sig = Signal::<&'static str>::new();
        // IDA 0xb79c: empty head is a no-op.
        stub_0xb76c(&sig, "GraphicsMode");
        let seen: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(Vec::new()));
        let probe = seen.clone();
        let handler = Arc::new(move |name: &'static str| {
            probe.lock().push(name);
        });
        sig.connect(handler.clone());
        stub_0xb76c(&sig, "GraphicsMode");
        assert_eq!(*seen.lock(), vec!["GraphicsMode"]);
    }

    #[test]
    fn dtor_thunks_adjust_this() {
        let mut slot: u64 = 0;
        let base = &mut slot as *mut u64 as *mut u8;
        // IDA 0xb8e0/0xb8e8 adjust by 0x20, 0xb910/0xb918 by 0x24.
        let sub20 = unsafe { base.add(0x20) };
        let sub24 = unsafe { base.add(0x24) };
        assert_eq!(stub_0xb8e0(sub20), base);
        assert_eq!(stub_0xb8e8(sub20), base);
        assert_eq!(stub_0xb910(sub24), base);
        assert_eq!(stub_0xb918(sub24), base);
    }

    #[test]
    fn aasamples_lookup_convert_round_trip() {
        let mut desc = RenderEnumDesc::new("AASamples");
        stub_0x9b48(&mut desc, 1, "None");
        stub_0x9b48(&mut desc, 4, "4");
        // IDA 0xb94c: hit returns the item, miss returns 0.
        assert_eq!(stub_0xb94c(&desc, "4"), 4);
        assert_eq!(stub_0xb94c(&desc, "nope"), 0);
        // IDA 0xb97c: any_cast + convertToItem identity.
        assert_eq!(stub_0xb97c(&desc, 4), 4);
        // IDA 0xb99c: +0x28 count check, +0x90 table load, R5 result.
        let mut out = 0;
        assert!(stub_0xb99c(&desc, 1, &mut out));
        assert_eq!(out, 4);
        assert!(!stub_0xb99c(&desc, 9, &mut out));
        // IDA 0xb9f8: name render on hit, 0 on miss.
        let mut s = String::new();
        assert!(stub_0xb9f8(&desc, 0, &mut s));
        assert_eq!(s, "None");
        let mut t = String::new();
        assert!(!stub_0xb9f8(&desc, 5, &mut t));
    }

    #[test]
    fn sibling_enums_share_lookup_convert_shape() {
        let mut gfx = RenderEnumDesc::new("GraphicsMode");
        stub_0x9ea8(&mut gfx, 4, "OpenGL");
        assert_eq!(stub_0xbb54(&gfx, "OpenGL"), 4);
        assert_eq!(stub_0xbb54(&gfx, "nope"), 0);
        assert_eq!(stub_0xbb84(&gfx, 4), 4);
        let mut out = 0;
        assert!(stub_0xbba4(&gfx, 0, &mut out));
        assert_eq!(out, 4);
        let mut s = String::new();
        assert!(stub_0xbc00(&gfx, 0, &mut s));
        assert_eq!(s, "OpenGL");
        let mut frm = RenderEnumDesc::new("FrameRateManagerMode");
        frm.add_pair(0, "Automatic");
        assert_eq!(stub_0xbd5c(&frm, "Automatic"), 0);
        assert_eq!(stub_0xbd8c(&frm, 0), 0);
        assert!(stub_0xbdac(&frm, 0, &mut out));
        assert_eq!(out, 0);
        assert!(stub_0xbe08(&frm, 0, &mut s));
        assert_eq!(s, "Automatic");
    }
}

#[cfg(test)]
mod batch4_tests {
    use super::*;
    use crate::generated_190 as g190;
    fn resolution_desc() -> RenderEnumDesc {
        stub_0x9100()
    }
    #[test]
    fn holder_singletons_carry_distinct_identities() {
        let res = stub_0xc95c();
        let qual = g190::stub_0xcf3c();
        assert_eq!(res.token, HOLDER_RESOLUTION_PRESET);
        assert_eq!(qual.token, HOLDER_QUALITY_LEVEL);
        assert_ne!(res.token, qual.token);
        assert!(res.type_name.contains("ResolutionPreset"));
        assert!(qual.type_name.contains("QualityLevel"));
        assert!(std::ptr::eq(res, stub_0xc95c()));
    }
    #[test]
    fn placement_assign_installs_and_overwrites() {
        let mut slot = PlacementAny::default();
        stub_0xc90c(&mut slot, 3);
        assert_eq!((slot.holder, slot.value), (HOLDER_RESOLUTION_PRESET, 3));
        stub_0xc90c(&mut slot, 5);
        assert_eq!((slot.holder, slot.value), (HOLDER_RESOLUTION_PRESET, 5));
        stub_0xceec(&mut slot, 9);
        assert_eq!((slot.holder, slot.value), (HOLDER_QUALITY_LEVEL, 9));
        assert_eq!(stub_0xcaa4(&PlacementAny { holder: HOLDER_RESOLUTION_PRESET, value: 4 }), 4);
        assert_eq!(g190::stub_0xd084(&slot), 9);
    }
    #[test]
    #[should_panic(expected = "bad_placement_any_cast")]
    fn any_cast_mismatched_holder_throws() {
        stub_0xcaa4(&PlacementAny { holder: HOLDER_QUALITY_LEVEL, value: 1 });
    }
    #[test]
    #[should_panic(expected = "bad_placement_any_cast")]
    fn any_cast_quality_mismatched_holder_throws() {
        g190::stub_0xd084(&PlacementAny::default());
    }
    #[test]
    fn construct_copies_word_and_tolerates_null_dst() {
        let src = 7i32;
        let mut dst = 0i32;
        assert_eq!(stub_0xc9c8(&src, &mut dst), 7);
        assert_eq!(dst, 7);
        assert_eq!(stub_0xc9c8(&src, core::ptr::null_mut()), 7);
        assert_eq!(g190::stub_0xcfa8(&src, &mut dst), 7);
        stub_0xc9d4();
        g190::stub_0xcfb4();
    }
    #[test]
    fn convert_round_trips_hit_and_miss() {
        let desc = resolution_desc();
        assert!(desc.pairs.len() > 2);
        assert_eq!(stub_0xc9d8(&desc, 2), 2);
        assert_eq!(stub_0xc9d8(&desc, -1), 0);
        assert_eq!(stub_0xc9d8(&desc, desc.pairs.len() as i32), 0);
        assert_eq!(g190::stub_0xcfb8(&desc, 1), 1);
        let mut out = -1;
        assert!(stub_0xcc34(&desc, "Automatic", &mut out));
        assert_eq!(out, 0);
        assert!(!g190::stub_0xd174(&desc, "nope", &mut out));
        let mut s = String::from("stale");
        stub_0xcd4c(&desc, 0, &mut s);
        assert!(!s.is_empty());
        stub_0xcd4c(&desc, -1, &mut s);
        assert!(s.is_empty());
    }
    #[test]
    fn dtors_release_the_tables() {
        let mut desc = resolution_desc();
        stub_0xccb0(&mut desc);
        assert!(desc.pairs.is_empty() && desc.aliases.is_empty());
        let mut desc = resolution_desc();
        g190::stub_0xd1f0(&mut desc);
        assert!(desc.pairs.is_empty() && desc.aliases.is_empty());
    }
}
