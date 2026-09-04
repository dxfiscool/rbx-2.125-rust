//! audio generated_18 — next 100 stubs EA-sorted, from ida/export.json
//! Filter: FMOD|Sound|Audio (2541 distinct all stubbed) | filler workspace EA-sorted asc after 0x84e0 (skip existing, rbx_core::SharedPtr not boost)
//! Batch: 100 stubs | skeleton batch | range 0x84e0..0xc010
//! Generated: 2026-09-01

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use rbx_core::signal::Signal;

// Ensure SharedPtr is seen as used — mirrors boost::shared_ptr<T> -> rbx_core::SharedPtr<T>
const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// Host models for IDA 0x84e0..0xa25c (CRenderSettings / CRenderSettingsItem /
// RBX::Reflection::EnumDesc). Field offsets cited per function; link words are
// host-sized. was: boost::shared_ptr / boost::signals -> SharedPtr / Signal.

/// Image addresses of the property descriptors passed to the changed signal
/// (IDA `unk_130Cxxx` operands at 0x9622..0x9b26, `prop_resolution` at 0x97ac).
pub const PROP_ALWAYS_DRAW_CONNECTORS: u32 = 0x0130_C030;
pub const PROP_SHOW_AGGREGATION: u32 = 0x0130_C05C;
pub const PROP_DEBUG_SHOW_BOUNDING_BOXES: u32 = 0x0130_C0E0;
pub const PROP_ENABLE_FRM: u32 = 0x0130_C138;
pub const PROP_EAGER_BULK_EXECUTION: u32 = 0x0130_C1E8;
pub const PROP_GRAPHICS_MODE: u32 = 0x0130_C244;
pub const PROP_FRAME_RATE_MANAGER_MODE: u32 = 0x0130_C278;
pub const PROP_QUALITY_LEVEL: u32 = 0x0130_C2AC;
pub const PROP_AA_SAMPLES: u32 = 0x0130_C2E0;
pub const PROP_SHADOW_MODE: u32 = 0x0130_C314;
pub const PROP_ANTIALIASING_MODE: u32 = 0x0130_C348;
pub const PROP_RESOLUTION: u32 = 0x012D_2C78;

/// IDA 0x992a/0x9934: `loc_F423FC+3` threshold for the VRAM budget bump.
pub const VRAM_BUDGET_BUMP: u32 = 0x00F4_23FF;
/// IDA 0x8f96/0x8ff0: QualityLevel loop bound (`CMP R4, #0x16`).
pub const QUALITY_LEVEL_MAX: i32 = 0x15;

/// Host model of `RBX::Reflection::EnumDescriptor` + the `EnumDesc<T>` pair
/// tables (IDA 0x850c..0x9100 ctors, 0x9b48..0xafdc addPair, 0xa208 addLegacy).
/// `pairs` is the ordered (value, name) table, `legacy` the index->value
/// vector (`resize` fill -1 at IDA 0xa234), `by_name` the `std::map<Name const*,
/// T>` all three write into.
#[derive(Default)]
pub struct EnumDescModel {
    pub name: &'static str,
    pub pairs: Vec<(i32, String)>,
    pub legacy: Vec<i32>,
    pub by_name: HashMap<String, i32>,
}

impl EnumDescModel {
    /// IDA 0x850c/0x8542: base `EnumDescriptor(name)` + vtable + empty tables.
    pub fn init(&mut self, name: &'static str) {
        self.name = name;
        self.pairs.clear();
        self.legacy.clear();
        self.by_name.clear();
    }

    /// IDA 0x9b48: `Descriptor` item alloc + parallel-table grow + value
    /// push_back + `string::assign(name)` + `map[declare(name)] = value`.
    pub fn add_pair(&mut self, value: i32, name: &str) {
        self.pairs.push((value, name.to_owned()));
        self.by_name.insert(name.to_owned(), value);
    }

    /// IDA 0xa208: `legacy.resize(index + 1, -1)` (0xa234), `legacy[index] =
    /// value` (0xa23a), `map[declare(name)] = value` (0xa240/0xa24c).
    pub fn add_legacy(&mut self, index: usize, name: &str, value: i32) {
        if self.legacy.len() <= index {
            self.legacy.resize(index + 1, -1);
        }
        self.legacy[index] = value;
        self.by_name.insert(name.to_owned(), value);
    }

    /// Legacy-name-only entry (IDA 0x8f9a..0x8ff2 QualityLevel loop).
    pub fn add_legacy_name(&mut self, name: &str, value: i32) {
        self.by_name.insert(name.to_owned(), value);
    }

    pub fn lookup(&self, name: &str) -> Option<i32> {
        self.by_name.get(name).copied()
    }
}

/// `RBX::CRenderSettings::aaSamples` (IDA 0x96d0 reads/writes the global).
pub static AA_SAMPLES: AtomicI32 = AtomicI32::new(0);
/// `RBX::PartInstance::disableInterpolation` (IDA 0x9784/0x9794).
pub static DISABLE_INTERPOLATION: AtomicBool = AtomicBool::new(false);

/// Host model of `CRenderSettingsItem`. Offsets are the ARM field offsets
/// read/stored by the setters below (+100..+164); `changed` is the host
/// stand-in for the `rbx::signals::signal<...>` at this+192.
/// was: boost::signals::signal -> rbx_core::Signal.
#[derive(Default)]
pub struct RenderSettingsItem {
    pub graphics_mode: i32,
    pub antialiasing_mode: i32,
    pub shadow_mode: i32,
    pub frame_rate_manager_mode: i32,
    pub quality_level: i32,
    pub resolution_preset: i32,
    pub auto_quality_level: i32,
    pub debug_show_bounding_boxes: bool,
    pub enable_frm: bool,
    pub show_aggregation: bool,
    pub always_draw_connectors: bool,
    pub connector_draw_mode: u8,
    pub eager_bulk_execution: bool,
    pub texture_cache_size: u32,
    pub mesh_cache_size: u32,
    /// G3D::Vector2int16 default at +0xAC (IDA 0x987e/0x988a: 800x600).
    pub default_resolution: (u16, u16),
    /// `std::vector<G3D::Vector2int16>` at +0xB0 (IDA 0x991a push_back).
    pub resolutions: Vec<(u16, u16)>,
    /// Texture budget at +0x92 (IDA 0x9946).
    pub texture_budget: (u16, u16),
    /// Byte at +0xBD forced to 1 (IDA 0x98b0).
    pub init_flag_bd: u8,
    /// Category string passed to the vtable registrar (IDA 0x98e0 "Rendering").
    pub category: String,
    pub changed: Signal<u32>,
    /// Descriptor ids emitted, in order (test seam for the signal).
    pub fired: Vec<u32>,
}

impl RenderSettingsItem {
    /// IDA 0x9608..: `signal_with_args<...>::operator()(this + 192, desc)`.
    pub fn emit_prop_changed(&mut self, desc: u32) {
        self.fired.push(desc);
        self.changed.fire(desc);
    }
}
/// Host model of `RBX::CRenderSettings` (the global settings object the
/// `get*` leaves at 0xb33c..0xb8b0 read — distinct from the
/// `CRenderSettingsItem` Instance above). Only IDA-observed slots are
/// modelled, in image offset order; `aaSamples` is the `AA_SAMPLES` global,
/// not a field (IDA 0xb3e8 double-indirects it and ignores `this`).
/// was: boost::shared_ptr / boost::signals -> SharedPtr / Signal.
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
    /// +0x20 dword. IDA 0xb4cc `LDR R0,[R0,#0x20]`.
    pub max_quality_level: i32,
    /// +0x28 byte, zero-extended. IDA 0xb46c `LDRB.W R0,[R0,#0x28]`.
    pub debug_show_bounding_boxes: bool,
    /// +0x29 byte, zero-extended. IDA 0xb49c `LDRB.W R0,[R0,#0x29]`.
    pub enable_frm: bool,
    /// +0x3A byte, zero-extended. IDA 0xb3e0 `LDRB.W R0,[R0,#0x3A]`.
    pub show_aggregation: bool,
    /// +0x3B byte, zero-extended. IDA 0xb3b4 `LDRB.W R0,[R0,#0x3B]`.
    pub always_draw_connectors: bool,
    /// +0x3D byte, zero-extended. IDA 0xb8b0 `LDRB.W R0,[R0,#0x3D]`.
    pub eager_bulk_execution: bool,
    /// +0x40 dword. IDA 0xb4f4 `LDR R0,[R0,#0x40]`.
    pub texture_cache_size: u32,
    /// +0x44 dword. IDA 0xb4f8 `LDR R0,[R0,#0x44]`.
    pub mesh_cache_size: u32,
}

/// IDA 0x9922 `GetDXVideoMemorySize()` — OS query with no image-side body;
/// the host has no DX device, so report 0 and keep the 800x600 budget.
pub fn dx_video_memory_size() -> u32 {
    0
}

/// Host stand-in for the image `main` invoked at IDA 0x8504.
fn host_main(_argc: i32, _argv: &[*const u8], _envp: &[*const u8]) -> i32 {
    0
}

// 0x84e0 — start
// type: void __fastcall __noreturn(int, int, int, int, int argc, char *argv)
#[doc(alias = "start")]
pub fn stub_84e0() -> ! {
    // IDA 0x84e0: v6 = &argv[argc + 1] (0x84f4); while (*v6++) (0x84f8/0x8500);
    // exit(main(argc, argv, envp)) (0x8504/0x8508).
    let args: Vec<std::ffi::CString> = std::env::args_os()
        .map(|a| std::ffi::CString::new(a.as_encoded_bytes()).unwrap_or_default())
        .collect();
    let env: Vec<std::ffi::CString> = std::env::vars_os()
        .map(|(k, v)| {
            let mut kv = k.as_encoded_bytes().to_vec();
            kv.push(b'=');
            kv.extend_from_slice(v.as_encoded_bytes());
            std::ffi::CString::new(kv).unwrap_or_default()
        })
        .collect();
    let argv: Vec<*const u8> = args.iter().map(|a| a.as_ptr() as *const u8).collect();
    let mut envp: Vec<*const u8> = env.iter().map(|e| e.as_ptr() as *const u8).collect();
    // Mirror `while (*v6++)`: walk to the envp NULL terminator.
    envp.push(std::ptr::null());
    let mut scan = envp.as_ptr();
    while !unsafe { *scan }.is_null() {
        scan = unsafe { scan.add(1) };
    }
    let code = host_main(argv.len() as i32, &argv, &envp);
    std::process::exit(code);
}

// 0x850c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::EnumDesc(void)")]
pub fn stub_850c(desc: &mut EnumDescModel) {
    // IDA 0x850c: base EnumDescriptor("AASamples") (0x8542), vtable off_1221308
    // (0x855a), count 0 (0x8562), empty inline vectors (0x8566..0x857c).
    desc.init("AASamples");
}

// 0x86d0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::EnumDesc(void)")]
pub fn stub_86d0(desc: &mut EnumDescModel) {
    // IDA 0x86d0: base EnumDescriptor("GraphicsMode") (0x8706), vtable
    // off_1221338 (0x871e), empty tables (0x8726..0x8740).
    desc.init("GraphicsMode");
}

// 0x88c4 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::EnumDesc(void)")]
pub fn stub_88c4(desc: &mut EnumDescModel) {
    // IDA 0x88c4: base EnumDescriptor("FramerateManagerMode") (0x88fa) — note
    // the image literal drops the capital R — vtable off_1221368 (0x8912).
    desc.init("FramerateManagerMode");
}

// 0x8a88 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::EnumDesc(void)")]
pub fn stub_8a88(desc: &mut EnumDescModel) {
    // IDA 0x8a88: base EnumDescriptor("Antialiasing") (0x8abe), vtable
    // off_1221398 (0x8ad6), empty tables (0x8ade..0x8af8).
    desc.init("Antialiasing");
}

// 0x8c4c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::EnumDesc(void)")]
pub fn stub_8c4c(desc: &mut EnumDescModel) {
    // IDA 0x8c4c: base EnumDescriptor("Shadow") (0x8c82), vtable off_12213C8
    // (0x8c9a), empty tables (0x8ca2..0x8cbc).
    desc.init("Shadow");
}

// 0x8e24 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEEC2Ev
// type: RBX::Reflection::EnumDescriptor *__fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::EnumDesc(void)")]
pub fn stub_8e24(desc: &mut EnumDescModel) {
    // IDA 0x8e24: base EnumDescriptor("QualityLevel") (0x8e6c), vtable +
    // empty tables, then addPair(0, "Automatic") (0x8f20), a Level%.2d loop
    // for 1..=0x15 (0x8f5c..0x8f98), and a legacy snprintf "%2u" loop over
    // "Level 00"+6 feeding Name::declare + map[name] = level (0x8f9a..0x8ff2).
    desc.init("QualityLevel");
    desc.add_pair(0, "Automatic");
    for level in 1..=QUALITY_LEVEL_MAX {
        desc.add_pair(level, &format!("Level{level:02}"));
    }
    for level in 1..=QUALITY_LEVEL_MAX {
        desc.add_legacy_name(&format!("Level{level:>3}"), level);
    }
}

// 0x9100 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEEC2Ev
// type: RBX::Reflection::EnumDescriptor *__fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::EnumDesc(void)")]
pub fn stub_9100(desc: &mut EnumDescModel) {
    // IDA 0x9100: base EnumDescriptor("Resolution") (0x911c), 19 addPair
    // calls with values 0..=0x12 (0x91ee..0x950a), each followed (except the
    // last) by a Name::declare + map insert for the "(wide)" legacy alias
    // carrying the same value (0x9254..0x952e).
    desc.init("Resolution");
    const PAIRS: [(i32, &str); 19] = [
        (0x00, "Automatic"),
        (0x01, "720x526"),
        (0x02, "800x600"),
        (0x03, "1024x600"),
        (0x04, "1024x768"),
        (0x05, "1280x720"),
        (0x06, "1280x768"),
        (0x07, "1152x864"),
        (0x08, "1280x800"),
        (0x09, "1360x768"),
        (0x0a, "1280x960"),
        (0x0b, "1280x1024"),
        (0x0c, "1440x900"),
        (0x0d, "1600x900"),
        (0x0e, "1600x1024"),
        (0x0f, "1600x1200"),
        (0x10, "1680x1050"),
        (0x11, "1920x1080"),
        (0x12, "1920x1200"),
    ];
    for (value, name) in PAIRS {
        desc.add_pair(value, name);
    }
    const LEGACY: [(&str, i32); 11] = [
        ("1024x600 (wide)", 0x03),
        ("1280x720 (wide)", 0x05),
        ("1280x768 (wide)", 0x06),
        ("1280x800 (wide)", 0x08),
        ("1360x768 (wide)", 0x09),
        ("1440x900 (wide)", 0x0c),
        ("1600x900 (wide)", 0x0d),
        ("1600x1024 (wide)", 0x0e),
        ("1680x1050 (wide)", 0x10),
        ("1920x1080 (wide)", 0x11),
        ("1920x1200 (wide)", 0x12),
    ];
    for (name, value) in LEGACY {
        desc.add_legacy_name(name, value);
    }
}

// 0x9608 — __ZN19CRenderSettingsItem15setGraphicsModeEN3RBX15CRenderSettings12GraphicsModeE
// type: int __fastcall(int result, int)
#[doc(alias = "CRenderSettingsItem::setGraphicsMode(RBX::CRenderSettings::GraphicsMode)")]
pub fn stub_9608(item: &mut RenderSettingsItem, mode: i32) {
    // IDA 0x9608: if (*(result + 100) != a2) { *(result + 100) = a2;
    // signal(result + 192, &unk_130C244); } (0x960c..0x9622).
    if item.graphics_mode != mode {
        item.graphics_mode = mode;
        item.emit_prop_changed(PROP_GRAPHICS_MODE);
    }
}

// 0x9628 — __ZN19CRenderSettingsItem23setFrameRateManagerModeEN3RBX15CRenderSettings20FrameRateManagerModeE
// type: int __fastcall(int result, int)
#[doc(alias = "CRenderSettingsItem::setFrameRateManagerMode(RBX::CRenderSettings::FrameRateManagerMode)")]
pub fn stub_9628(item: &mut RenderSettingsItem, mode: i32) {
    // IDA 0x9628: offset +112, descriptor unk_130C278 (0x962c..0x9642).
    if item.frame_rate_manager_mode != mode {
        item.frame_rate_manager_mode = mode;
        item.emit_prop_changed(PROP_FRAME_RATE_MANAGER_MODE);
    }
}

// 0x9648 — __ZN19CRenderSettingsItem15setQualityLevelEN3RBX15CRenderSettings12QualityLevelE
// type: int __fastcall(int result, int)
#[doc(alias = "CRenderSettingsItem::setQualityLevel(RBX::CRenderSettings::QualityLevel)")]
pub fn stub_9648(item: &mut RenderSettingsItem, level: i32) {
    // IDA 0x9648: offset +116, descriptor unk_130C2AC (0x964c..0x9662).
    if item.quality_level != level {
        item.quality_level = level;
        item.emit_prop_changed(PROP_QUALITY_LEVEL);
    }
}

// 0x9668 — __ZN19CRenderSettingsItem23setAlwaysDrawConnectorsEb
// type: int __fastcall(int this, int)
#[doc(alias = "CRenderSettingsItem::setAlwaysDrawConnectors(bool)")]
pub fn stub_9668(item: &mut RenderSettingsItem, enabled: bool) {
    // IDA 0x9668: v2 = old[155] ? 1 : norm(old[156]) (0x9668..0x9678); store
    // (0x967a); a2 == 1 signals iff v2 == 0 (0x9680/0x9694..0x969a), else
    // v3 = norm([156]) and signal iff v2 != v3 (0x9682..0x969a). Both arms
    // reduce to signalling on effective-value change.
    fn norm(v: u8) -> i32 {
        if v != 0 { 1 } else { 0 }
    }
    let old_effective = if item.always_draw_connectors { 1 } else { norm(item.connector_draw_mode) };
    item.always_draw_connectors = enabled;
    let new_effective = if enabled { 1 } else { norm(item.connector_draw_mode) };
    if old_effective != new_effective {
        item.emit_prop_changed(PROP_ALWAYS_DRAW_CONNECTORS);
    }
}

// 0x96ac — __ZN19CRenderSettingsItem18setShowAggregationEb
// type: int __fastcall(int this, int)
#[doc(alias = "CRenderSettingsItem::setShowAggregation(bool)")]
pub fn stub_96ac(item: &mut RenderSettingsItem, enabled: bool) {
    // IDA 0x96ac: byte at +154, descriptor unk_130C05C (0x96b2..0x96ca).
    if item.show_aggregation != enabled {
        item.show_aggregation = enabled;
        item.emit_prop_changed(PROP_SHOW_AGGREGATION);
    }
}

// 0x96d0 — __ZN19CRenderSettingsItem12setAASamplesEN3RBX15CRenderSettings9AASamplesE
// type: int __fastcall(int result, int)
#[doc(alias = "CRenderSettingsItem::setAASamples(RBX::CRenderSettings::AASamples)")]
pub fn stub_96d0(item: &mut RenderSettingsItem, samples: i32) {
    // IDA 0x96d0: compares/stores the RBX::CRenderSettings::aaSamples global
    // (0x96e0/0x96ee), descriptor unk_130C2E0 (0x96f6).
    if AA_SAMPLES.load(Ordering::SeqCst) != samples {
        AA_SAMPLES.store(samples, Ordering::SeqCst);
        item.emit_prop_changed(PROP_AA_SAMPLES);
    }
}

// 0x96fc — __ZN19CRenderSettingsItem13setShadowModeEN3RBX15CRenderSettings10ShadowModeE
// type: int __fastcall(int result, int)
#[doc(alias = "CRenderSettingsItem::setShadowMode(RBX::CRenderSettings::ShadowMode)")]
pub fn stub_96fc(item: &mut RenderSettingsItem, mode: i32) {
    // IDA 0x96fc: offset +108, descriptor unk_130C314 (0x9700..0x9716).
    if item.shadow_mode != mode {
        item.shadow_mode = mode;
        item.emit_prop_changed(PROP_SHADOW_MODE);
    }
}

// 0x971c — __ZN19CRenderSettingsItem19setAntialiasingModeEN3RBX15CRenderSettings16AntialiasingModeE
// type: int __fastcall(int result, int)
#[doc(alias = "CRenderSettingsItem::setAntialiasingMode(RBX::CRenderSettings::AntialiasingMode)")]
pub fn stub_971c(item: &mut RenderSettingsItem, mode: i32) {
    // IDA 0x971c: offset +104, descriptor unk_130C348 (0x9720..0x9736).
    if item.antialiasing_mode != mode {
        item.antialiasing_mode = mode;
        item.emit_prop_changed(PROP_ANTIALIASING_MODE);
    }
}

// 0x973c — __ZN19CRenderSettingsItem25setDebugShowBoundingBoxesEb
// type: int __fastcall(int this, int)
#[doc(alias = "CRenderSettingsItem::setDebugShowBoundingBoxes(bool)")]
pub fn stub_973c(item: &mut RenderSettingsItem, enabled: bool) {
    // IDA 0x973c: byte at +136, descriptor unk_130C0E0 (0x9742..0x975a).
    if item.debug_show_bounding_boxes != enabled {
        item.debug_show_bounding_boxes = enabled;
        item.emit_prop_changed(PROP_DEBUG_SHOW_BOUNDING_BOXES);
    }
}

// 0x9760 — __ZN19CRenderSettingsItem12setEnableFRMEb
// type: int __fastcall(int this, int)
#[doc(alias = "CRenderSettingsItem::setEnableFRM(bool)")]
pub fn stub_9760(item: &mut RenderSettingsItem, enabled: bool) {
    // IDA 0x9760: byte at +137, descriptor unk_130C138 (0x9766..0x977e).
    if item.enable_frm != enabled {
        item.enable_frm = enabled;
        item.emit_prop_changed(PROP_ENABLE_FRM);
    }
}

// 0x9784 — __ZNK19CRenderSettingsItem28getDebugDisableInterpolationEv
// type: int __fastcall(CRenderSettingsItem *this)
#[doc(alias = "CRenderSettingsItem::getDebugDisableInterpolation(void)const")]
pub fn stub_9784() -> bool {
    // IDA 0x9784: return RBX::PartInstance::disableInterpolation (0x9792).
    DISABLE_INTERPOLATION.load(Ordering::SeqCst)
}

// 0x9794 — __ZN19CRenderSettingsItem28setDebugDisableInterpolationEb
// type: char *__fastcall(CRenderSettingsItem *this, char)
#[doc(alias = "CRenderSettingsItem::setDebugDisableInterpolation(bool)")]
pub fn stub_9794(disabled: bool) -> &'static AtomicBool {
    // IDA 0x9794: disableInterpolation = a2 (0x97a0); return &global (0x97a2).
    DISABLE_INTERPOLATION.store(disabled, Ordering::SeqCst);
    &DISABLE_INTERPOLATION
}

// 0x97a4 — __ZN19CRenderSettingsItem23setResolutionPreferenceEN3RBX15CRenderSettings16ResolutionPresetE
// type: int __fastcall(int result, int)
#[doc(alias = "CRenderSettingsItem::setResolutionPreference(RBX::CRenderSettings::ResolutionPreset)")]
pub fn stub_97a4(item: &mut RenderSettingsItem, preset: i32) {
    // IDA 0x97a4: offset +120, descriptor prop_resolution (0x97a8..0x97bc).
    if item.resolution_preset != preset {
        item.resolution_preset = preset;
        item.emit_prop_changed(PROP_RESOLUTION);
    }
}

// 0x97c0 — __ZN19CRenderSettingsItem19setTextureCacheSizeEj
// type: int __fastcall(int this, unsigned int)
#[doc(alias = "CRenderSettingsItem::setTextureCacheSize(unsigned int)")]
pub fn stub_97c0(item: &mut RenderSettingsItem, size: u32) {
    // IDA 0x97c0: *(this + 160) = a2, no signal (0x97c0..0x97c4).
    item.texture_cache_size = size;
}

// 0x97c8 — __ZN19CRenderSettingsItem16setMeshCacheSizeEj
// type: int __fastcall(int this, unsigned int)
#[doc(alias = "CRenderSettingsItem::setMeshCacheSize(unsigned int)")]
pub fn stub_97c8(item: &mut RenderSettingsItem, size: u32) {
    // IDA 0x97c8: *(this + 164) = a2, no signal (0x97c8..0x97cc).
    item.mesh_cache_size = size;
}

// 0x97d0 — __ZN19CRenderSettingsItemC2Ev
// type: void __fastcall(CRenderSettingsItem *this)
#[doc(alias = "CRenderSettingsItem::CRenderSettingsItem(void)")]
pub fn stub_97d0(item: &mut RenderSettingsItem) {
    // IDA 0x97d0: base GlobalAdvancedSettingsItem C2 (0x97f0), CRenderSettings
    // C2 at +0x60 (0x9828), vtables (0x9836..0x985c), default 800x600 at
    // +0xAC (0x987e/0x988a), [0xBD] = 1 (0x98b0), signal once-init
    // (0x98d0/0x98d8), category "Rendering" via vtable +0x1C (0x98e0..0x9904),
    // resolution push_back (0x991a), VRAM budget at +0x92: 800x600, or
    // 1024x768 when GetDXVideoMemorySize() > 0xF423FF (0x9922..0x9946).
    let fresh = RenderSettingsItem {
        default_resolution: (800, 600),
        texture_budget: (800, 600),
        init_flag_bd: 1,
        category: "Rendering".to_owned(),
        ..RenderSettingsItem::default()
    };
    *item = fresh;
    item.resolutions.push(item.default_resolution);
    if dx_video_memory_size() > VRAM_BUDGET_BUMP {
        item.texture_budget = (1024, 768);
    }
}

// 0x9ac8 — __ZN19CRenderSettingsItem19setAutoQualityLevelEi
// type: int __fastcall(int this, int)
#[doc(alias = "CRenderSettingsItem::setAutoQualityLevel(int)")]
pub fn stub_9ac8(item: &mut RenderSettingsItem, level: i32) {
    // IDA 0x9ac8: offset +124, descriptor unk_130C2AC shared with quality
    // (0x9acc..0x9ae2).
    if item.auto_quality_level != level {
        item.auto_quality_level = level;
        item.emit_prop_changed(PROP_QUALITY_LEVEL);
    }
}

// 0x9b08 — __ZN19CRenderSettingsItem21setEagerBulkExecutionEb
// type: int __fastcall(int this, int)
#[doc(alias = "CRenderSettingsItem::setEagerBulkExecution(bool)")]
pub fn stub_9b08(item: &mut RenderSettingsItem, enabled: bool) {
    // IDA 0x9b08: byte at +157, descriptor unk_130C1E8 (0x9b0e..0x9b26).
    if item.eager_bulk_execution != enabled {
        item.eager_bulk_execution = enabled;
        item.emit_prop_changed(PROP_EAGER_BULK_EXECUTION);
    }
}

// 0x9b48 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::addPair(RBX::CRenderSettings::AASamples,char const*)")]
pub fn stub_9b48(desc: &mut EnumDescModel, value: i32, name: &str) {
    // IDA 0x9b48: Item descriptor alloc (Znwm + Descriptor C2), Item* table
    // push_back, parallel value/name/string table grow + push_back,
    // string::assign(name), map[declare(name)] = value.
    desc.add_pair(value, name);
}

// 0x9ea8 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::addPair(RBX::CRenderSettings::GraphicsMode,char const*)")]
pub fn stub_9ea8(desc: &mut EnumDescModel, value: i32, name: &str) {
    // IDA 0x9ea8: same addPair shape as 0x9b48 for the GraphicsMode tables.
    desc.add_pair(value, name);
}

// 0xa208 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE9addLegacyEiPKcS3_
// type: _DWORD *__fastcall(int, unsigned int, int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::addLegacy(int,char const*,RBX::CRenderSettings::GraphicsMode)")]
pub fn stub_a208(desc: &mut EnumDescModel, index: u32, name: &str, value: i32) {
    // IDA 0xa208: legacy.resize(index + 1, -1) (0xa234), legacy[index] =
    // value (0xa23a), Name::declare (0xa240), map[name] = value (0xa24c).
    desc.add_legacy(index as usize, name, value);
}

// 0xa25c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::addPair(RBX::CRenderSettings::FrameRateManagerMode,char const*)")]
pub fn stub_a25c(desc: &mut EnumDescModel, value: i32, name: &str) {
    // IDA 0xa25c: same addPair shape as 0x9b48 for FrameRateManagerMode.
    desc.add_pair(value, name);
}

// 0xa5bc — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE7addPairES3_PKc
pub fn stub_a5bc(desc: &mut EnumDescModel, value: i32, name: &str) {
    // IDA 0xa5bc: same addPair shape as 0x9b48 for the AntialiasingMode tables.
    desc.add_pair(value, name);
}

// 0xa91c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE7addPairES3_PKc
pub fn stub_a91c(desc: &mut EnumDescModel, value: i32, name: &str) {
    // IDA 0xa91c: same addPair shape as 0x9b48 for the ShadowMode tables.
    desc.add_pair(value, name);
}

// 0xac7c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE7addPairES3_PKc
pub fn stub_ac7c(desc: &mut EnumDescModel, value: i32, name: &str) {
    // IDA 0xac7c: same addPair shape as 0x9b48 for the QualityLevel tables.
    desc.add_pair(value, name);
}

// 0xafdc — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE7addPairES3_PKc
pub fn stub_afdc(desc: &mut EnumDescModel, value: i32, name: &str) {
    // IDA 0xafdc: same addPair shape as 0x9b48 for the ResolutionPreset tables.
    desc.add_pair(value, name);
}

// 0xb33c — __ZNK3RBX15CRenderSettings15getGraphicsModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getGraphicsMode(void)const")]
pub fn stub_b33c(settings: &CRenderSettings) -> i32 {
    // IDA 0xb33c `LDR R0,[R0,#4]` (disasm 0xb33c..0xb33e): plain +4 field load.
    settings.graphics_mode
}

// 0xb340 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::~EnumPropDescriptor()")]
pub fn stub_b340() {
    // IDA 0xb340: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xb364 — __ZNK3RBX15CRenderSettings23getFrameRateManagerModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getFrameRateManagerMode(void)const")]
pub fn stub_b364(settings: &CRenderSettings) -> i32 {
    // IDA 0xb364 `LDR R0,[R0,#0x10]` (disasm 0xb364..0xb366): plain +0x10 field load.
    settings.frame_rate_manager_mode
}

// 0xb368 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::~EnumPropDescriptor()")]
pub fn stub_b368() {
    // IDA 0xb368: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xb38c — __ZNK3RBX15CRenderSettings15getQualityLevelEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getQualityLevel(void)const")]
pub fn stub_b38c(settings: &CRenderSettings) -> i32 {
    // IDA 0xb38c `LDR R0,[R0,#0x14]` (disasm 0xb38c..0xb38e): plain +0x14 field load.
    settings.quality_level
}

// 0xb390 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::~EnumPropDescriptor()")]
pub fn stub_b390() {
    // IDA 0xb390: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xb3b4 — __ZNK3RBX15CRenderSettings23getAlwaysDrawConnectorsEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getAlwaysDrawConnectors(void)const")]
pub fn stub_b3b4(settings: &CRenderSettings) -> i32 {
    // IDA 0xb3b4 `LDRB.W R0,[R0,#0x3B]` (disasm 0xb3b4..0xb3b8): byte load, zero-extended.
    i32::from(settings.always_draw_connectors)
}

// 0xb3bc — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::~PropDescriptor()")]
pub fn stub_b3bc() {
    // IDA 0xb3bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xb3e0 — __ZNK3RBX15CRenderSettings18getShowAggregationEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getShowAggregation(void)const")]
pub fn stub_b3e0(settings: &CRenderSettings) -> i32 {
    // IDA 0xb3e0 `LDRB.W R0,[R0,#0x3A]` (disasm 0xb3e0..0xb3e4): byte load, zero-extended.
    i32::from(settings.show_aggregation)
}

// 0xb3e8 — __ZNK3RBX15CRenderSettings12getAASamplesEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getAASamples(void)const")]
pub fn stub_b3e8(_settings: &CRenderSettings) -> i32 {
    // IDA 0xb3e8 double-indirects the `aaSamples` global via its `_ptr` slot
    // (disasm 0xb3e8..0xb3f6); `this` is unused. Modelled by `AA_SAMPLES`.
    AA_SAMPLES.load(Ordering::SeqCst)
}

// 0xb3f8 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::~EnumPropDescriptor()")]
pub fn stub_b3f8() {
    // IDA 0xb3f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xb41c — __ZNK3RBX15CRenderSettings13getShadowModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getShadowMode(void)const")]
pub fn stub_b41c(settings: &CRenderSettings) -> i32 {
    // IDA 0xb41c `LDR R0,[R0,#0xC]` (disasm 0xb41c..0xb41e): plain +0xC field load.
    settings.shadow_mode
}

// 0xb420 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::~EnumPropDescriptor()")]
pub fn stub_b420() {
    // IDA 0xb420: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xb444 — __ZNK3RBX15CRenderSettings19getAntialiasingModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getAntialiasingMode(void)const")]
pub fn stub_b444(settings: &CRenderSettings) -> i32 {
    // IDA 0xb444 `LDR R0,[R0,#8]` (disasm 0xb444..0xb446): plain +8 field load.
    settings.antialiasing_mode
}

// 0xb448 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::~EnumPropDescriptor()")]
pub fn stub_b448() {
    // IDA 0xb448: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xb46c — __ZNK3RBX15CRenderSettings25getDebugShowBoundingBoxesEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getDebugShowBoundingBoxes(void)const")]
pub fn stub_b46c(settings: &CRenderSettings) -> i32 {
    // IDA 0xb46c `LDRB.W R0,[R0,#0x28]` (disasm 0xb46c..0xb470): byte load, zero-extended.
    i32::from(settings.debug_show_bounding_boxes)
}

// 0xb474 — __ZNK3RBX15CRenderSettings19getAutoQualityLevelEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getAutoQualityLevel(void)const")]
pub fn stub_b474(settings: &CRenderSettings) -> i32 {
    // IDA 0xb474 `LDR R0,[R0,#0x1C]` (disasm 0xb474..0xb476): plain +0x1C field load.
    settings.auto_quality_level
}

// 0xb478 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::~PropDescriptor()")]
pub fn stub_b478() {
    // IDA 0xb478: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xb49c — __ZNK3RBX15CRenderSettings12getEnableFRMEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getEnableFRM(void)const")]
pub fn stub_b49c(settings: &CRenderSettings) -> i32 {
    // IDA 0xb49c `LDRB.W R0,[R0,#0x29]` (disasm 0xb49c..0xb4a0): byte load, zero-extended.
    i32::from(settings.enable_frm)
}

// 0xb4a4 — __ZNK3RBX15CRenderSettings23getResolutionPreferenceEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getResolutionPreference(void)const")]
pub fn stub_b4a4(settings: &CRenderSettings) -> i32 {
    // IDA 0xb4a4 `LDR R0,[R0,#0x18]` (disasm 0xb4a4..0xb4a6): plain +0x18 field load.
    settings.resolution_preference
}

// 0xb4a8 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::~EnumPropDescriptor()")]
pub fn stub_b4a8() {
    // IDA 0xb4a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xb4cc — __ZN3RBX15CRenderSettings18getMaxQualityLevelEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getMaxQualityLevel(void)")]
pub fn stub_b4cc(settings: &CRenderSettings) -> i32 {
    // IDA 0xb4cc `LDR R0,[R0,#0x20]` (disasm 0xb4cc..0xb4ce): plain +0x20 field load.
    settings.max_quality_level
}

// 0xb4d0 — __ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::~BoundFuncDesc()")]
pub fn stub_b4d0() {
    // IDA 0xb4d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xb4f4 — __ZNK3RBX15CRenderSettings19getTextureCacheSizeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getTextureCacheSize(void)const")]
pub fn stub_b4f4(settings: &CRenderSettings) -> i32 {
    // IDA 0xb4f4 `LDR R0,[R0,#0x40]` (disasm 0xb4f4..0xb4f6): plain +0x40 field load.
    settings.texture_cache_size as i32
}

// 0xb4f8 — __ZNK3RBX15CRenderSettings16getMeshCacheSizeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getMeshCacheSize(void)const")]
pub fn stub_b4f8(settings: &CRenderSettings) -> i32 {
    // IDA 0xb4f8 `LDR R0,[R0,#0x44]` (disasm 0xb4f8..0xb4fa): plain +0x44 field load.
    settings.mesh_cache_size as i32
}

// 0xb4fc — __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEEC2Ev
// type: RBX::Instance *__fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEEC2Ev")]
pub fn stub_b4fc() {
    // IDA 0xb4fc (`GlobalAdvancedSettingsItem` C2, decompiled 0xb4fc..0xb6b4):
    // 0xb51e `Instance::Instance(this, 0)`, vtable installs (0xb54e..0xb5e2),
    // `classDescriptor` + `registrar++` (0xb584..0xb5ba), byte +92 = 1
    // (0xb5ba), `std::string("RenderSettings")` + `setName` (0xb5ec..0xb5f8).
    // Every effect lands on the `Instance` base / vtables / registrar, none
    // of which exist in this crate's `RenderSettingsItem` model (the derived
    // 0x97d0 ctor owns member init), so the host cutover is drop glue.
}

// 0xb740 — __ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE9push_backERKS1_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::push_back(G3D::Vector2int16 const&)")]
pub fn stub_b740<'a>(vec: &'a mut Vec<(u16, u16)>, value: &(u16, u16)) -> &'a mut Vec<(u16, u16)> {
    // IDA 0xb740..0xb75e: `finish = *(result + 4)` (0xb742); unless `finish`
    // reached `end_of_storage` (0xb74c), 4-byte copy the element and bump
    // `finish` (0xb756..0xb75c); full case delegates to `_M_insert_aux`
    // (0xb766). `Vec::push` is both paths (grow + move); the element is one
    // 4-byte `G3D::Vector2int16` lane pair, modelled as `(u16, u16)`.
    vec.push(*value);
    vec
}

// 0xb76c — __ZN3rbx7signals16signal_with_argsILi1EFvPKN3RBX10Reflection18PropertyDescriptorEEEclES6_
// type: void __fastcall(_DWORD *, int, int, const void *, int, int, int, int, void *, int)
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Reflection::PropertyDescriptor const*)>::operator()(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_b76c(item: &mut RenderSettingsItem, desc: u32) {
    // IDA 0xb76c (`signal_with_args<1, void(const PropertyDescriptor *)>`
    // `operator()`, decompiled 0xb76c..): fan out to every connected slot
    // with the descriptor. was: boost::signals -> rbx_core::Signal; the host
    // `changed` signal already models the slot list, so firing it with the
    // descriptor id is the call itself.
    item.emit_prop_changed(desc);
}

// 0xb8b0 — __ZNK3RBX15CRenderSettings21getEagerBulkExecutionEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getEagerBulkExecution(void)const")]
pub fn stub_b8b0(settings: &CRenderSettings) -> i32 {
    // IDA 0xb8b0 `LDRB.W R0,[R0,#0x3D]` (disasm 0xb8b0..0xb8b4): byte load, zero-extended.
    i32::from(settings.eager_bulk_execution)
}

// 0xb8b8 — __ZN19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "CRenderSettingsItem::~CRenderSettingsItem()")]
pub fn stub_b8b8() {
    // IDA 0xb8b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xb8bc — __ZN19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "CRenderSettingsItem::~CRenderSettingsItem()")]
pub fn stub_b8bc() {
    // IDA 0xb8bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xb8d0 — __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv")]
pub fn stub_b8d0() -> &'static str {
    // IDA 0xb8d0..0xb8dc: `Creator = static_getCreator()`, then tail-calls
    // `Creator::getClassName` on it — the declared `sRenderSettings` name,
    // `"RenderSettings"` (cf. IDA 0xedfc/0xf1d8 Name::declare path).
    "RenderSettings"
}

// 0xb8e0 — __ZThn32_N19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
pub fn stub_b8e0() {
    // IDA 0xb8e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xb8e8 — __ZThn32_N19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
pub fn stub_b8e8() {
    // IDA 0xb8e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xb900 — __ZThn32_NK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv")]
pub fn stub_b900() {
    // IDA 0xb900: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xb910 — __ZThn36_N19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
pub fn stub_b910() {
    // IDA 0xb910: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xb918 — __ZThn36_N19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
pub fn stub_b918() {
    // IDA 0xb918: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xb930 — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_b930() {
    // IDA 0xb930: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xb934 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()")]
pub fn stub_b934() {
    // IDA 0xb934: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xb938 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()")]
pub fn stub_b938() {
    // IDA 0xb938: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xb94c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::lookup(char const*)const")]
pub fn stub_b94c(desc: &EnumDescModel, name: &str) -> i32 {
    // IDA 0xb94c (decompiled): `Name::lookup` the string (0xb958), then
    // `convertToValue` (0xb966) and `convertToItem` on hit (0xb972), else
    // return 0 (0xb968..0xb978). The returned `Item*` has no host model, so
    // the looked-up value is returned; miss -> 0.
    desc.lookup(name).unwrap_or(0)
}

// 0xb97c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_b97c(desc: &EnumDescModel, value: i32) -> i32 {
    // IDA 0xb97c (decompiled 0xb97c..0xb998): `any_cast<T>` the Variant
    // payload (0xb986), then `convertToItem` (0xb992), which asserts
    // `value >= 0` and `value < size` (enumconverter.h:273-274). The caller
    // passes the already-cast value; the table hit is returned.
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!(
        (value as usize) < desc.pairs.len(),
        "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274"
    );
    desc.pairs[value as usize].0
}

// 0xb99c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_b99c(desc: &EnumDescModel, index: usize, out: &mut i32) -> bool {
    // IDA 0xb99c (disasm 0xb99c..0xb9f4): if `index < [this + 0x28]` (count,
    // 0xb9a4..0xb9a8), store `[[this + 0x90] + index * 4]` into the out-param
    // and return 1 (0xb9ac..0xb9b6). Else the Singleton + `placement_any`
    // fallback (0xb9c0..0xb9ec) resolves outside the pair tables; the host
    // reports failure with `out` untouched.
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
pub fn stub_b9f8(desc: &EnumDescModel, index: usize, out: &mut String) -> bool {
    // IDA 0xb9f8 (decompiled 0xba4c..0xbaaa): if `[a1 + 40] > index`, take
    // `[[a1 + 144] + index]` through the item `convertToString` (0xba5c..0xba66),
    // `assign` into the out string (0xba72) and return 1; else return 0 with
    // `out` untouched.
    if let Some((_, name)) = desc.pairs.get(index) {
        *out = name.clone();
        true
    } else {
        false
    }
}

// 0xbb3c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()")]
pub fn stub_bb3c() {
    // IDA 0xbb3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xbb40 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()")]
pub fn stub_bb40() {
    // IDA 0xbb40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xbb54 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::lookup(char const*)const")]
pub fn stub_bb54() -> ! {
    todo!("0xbb54 RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::lookup(char const*)const")
}

// 0xbb84 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_bb84() -> ! {
    todo!("0xbb84 RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::lookup(RBX::Reflection::Variant const&)const")
}

// 0xbba4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_bba4() -> ! {
    todo!("0xbba4 RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0xbc00 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToString(unsigned long,std::string &)const")]
pub fn stub_bc00() -> ! {
    todo!("0xbc00 RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToString(unsigned long,std::string &)const")
}

// 0xbd44 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::~EnumDesc()")]
pub fn stub_bd44() {
    // IDA 0xbd44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xbd48 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::~EnumDesc()")]
pub fn stub_bd48() {
    // IDA 0xbd48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xbd5c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::lookup(char const*)const")]
pub fn stub_bd5c() -> ! {
    todo!("0xbd5c RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::lookup(char const*)const")
}

// 0xbd8c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_bd8c() -> ! {
    todo!("0xbd8c RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::lookup(RBX::Reflection::Variant const&)const")
}

// 0xbdac — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_bdac() -> ! {
    todo!("0xbdac RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0xbe08 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToString(unsigned long,std::string &)const")]
pub fn stub_be08() -> ! {
    todo!("0xbe08 RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToString(unsigned long,std::string &)const")
}

// 0xbf4c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc()")]
pub fn stub_bf4c() {
    // IDA 0xbf4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xbf50 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc()")]
pub fn stub_bf50() {
    // IDA 0xbf50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xbf64 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::lookup(char const*)const")]
pub fn stub_bf64() -> ! {
    todo!("0xbf64 RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::lookup(char const*)const")
}

// 0xbf94 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_bf94() -> ! {
    todo!("0xbf94 RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::lookup(RBX::Reflection::Variant const&)const")
}

// 0xbfb4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_bfb4() -> ! {
    todo!("0xbfb4 RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0xc010 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToString(unsigned long,std::string &)const")]
pub fn stub_c010() -> ! {
    todo!("0xc010 RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToString(unsigned long,std::string &)const")
}

#[cfg(test)]
mod batch2_tests {
    use super::*;

    #[test]
    fn render_settings_getters_read_image_slots() {
        // IDA slot map: +4/+8/+0xC/+0x10/+0x14/+0x18/+0x1C/+0x20 dwords,
        // +0x28/+0x29/+0x3A/+0x3B/+0x3D zero-extended bytes, +0x40/+0x44 dwords.
        let settings = CRenderSettings {
            graphics_mode: 1,
            antialiasing_mode: 2,
            shadow_mode: 3,
            frame_rate_manager_mode: 4,
            quality_level: 5,
            resolution_preference: 6,
            auto_quality_level: 7,
            max_quality_level: 8,
            debug_show_bounding_boxes: true,
            enable_frm: true,
            show_aggregation: false,
            always_draw_connectors: true,
            eager_bulk_execution: true,
            texture_cache_size: 512,
            mesh_cache_size: 256,
        };
        assert_eq!(stub_b33c(&settings), 1);
        assert_eq!(stub_b444(&settings), 2);
        assert_eq!(stub_b41c(&settings), 3);
        assert_eq!(stub_b364(&settings), 4);
        assert_eq!(stub_b38c(&settings), 5);
        assert_eq!(stub_b4a4(&settings), 6);
        assert_eq!(stub_b474(&settings), 7);
        assert_eq!(stub_b4cc(&settings), 8);
        assert_eq!(stub_b46c(&settings), 1);
        assert_eq!(stub_b49c(&settings), 1);
        assert_eq!(stub_b3e0(&settings), 0);
        assert_eq!(stub_b3b4(&settings), 1);
        assert_eq!(stub_b8b0(&settings), 1);
        assert_eq!(stub_b4f4(&settings), 512);
        assert_eq!(stub_b4f8(&settings), 256);
        // IDA 0xb3e8 reads the `aaSamples` global and ignores `this`.
        AA_SAMPLES.store(9, Ordering::SeqCst);
        assert_eq!(stub_b3e8(&settings), 9);
        AA_SAMPLES.store(0, Ordering::SeqCst);
    }

    #[test]
    fn enum_desc_addpair_lookup_convert_roundtrip() {
        let mut desc = EnumDescModel::default();
        stub_a5bc(&mut desc, 0, "Automatic");
        stub_a91c(&mut desc, 1, "Low");
        stub_ac7c(&mut desc, 2, "High");
        stub_afdc(&mut desc, 3, "Ultra");
        assert_eq!(stub_b94c(&desc, "High"), 2);
        assert_eq!(stub_b94c(&desc, "Missing"), 0);
        assert_eq!(stub_b97c(&desc, 3), 3);
        let mut value = -1;
        assert!(stub_b99c(&desc, 1, &mut value));
        assert_eq!(value, 1);
        assert!(!stub_b99c(&desc, 9, &mut value));
        let mut name = String::new();
        assert!(stub_b9f8(&desc, 0, &mut name));
        assert_eq!(name, "Automatic");
        assert!(!stub_b9f8(&desc, 9, &mut name));
    }

    #[test]
    fn pushback_signal_classname() {
        let mut item = RenderSettingsItem::default();
        stub_b740(&mut item.resolutions, &(800, 600));
        stub_b740(&mut item.resolutions, &(1024, 768));
        assert_eq!(item.resolutions, vec![(800, 600), (1024, 768)]);
        stub_b4fc();
        stub_b76c(&mut item, PROP_GRAPHICS_MODE);
        assert_eq!(item.fired, vec![PROP_GRAPHICS_MODE]);
        assert_eq!(stub_b8d0(), "RenderSettings");
    }
}