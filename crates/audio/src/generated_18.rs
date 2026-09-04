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
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::addPair(RBX::CRenderSettings::AntialiasingMode,char const*)")]
pub fn stub_a5bc() -> ! {
    todo!("0xa5bc RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::addPair(RBX::CRenderSettings::AntialiasingMode,char const*)")
}

// 0xa91c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::addPair(RBX::CRenderSettings::ShadowMode,char const*)")]
pub fn stub_a91c() -> ! {
    todo!("0xa91c RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::addPair(RBX::CRenderSettings::ShadowMode,char const*)")
}

// 0xac7c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::addPair(RBX::CRenderSettings::QualityLevel,char const*)")]
pub fn stub_ac7c() -> ! {
    todo!("0xac7c RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::addPair(RBX::CRenderSettings::QualityLevel,char const*)")
}

// 0xafdc — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::addPair(RBX::CRenderSettings::ResolutionPreset,char const*)")]
pub fn stub_afdc() -> ! {
    todo!("0xafdc RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::addPair(RBX::CRenderSettings::ResolutionPreset,char const*)")
}

// 0xb33c — __ZNK3RBX15CRenderSettings15getGraphicsModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getGraphicsMode(void)const")]
pub fn stub_b33c() -> ! {
    todo!("0xb33c RBX::CRenderSettings::getGraphicsMode(void)const")
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
pub fn stub_b364() -> ! {
    todo!("0xb364 RBX::CRenderSettings::getFrameRateManagerMode(void)const")
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
pub fn stub_b38c() -> ! {
    todo!("0xb38c RBX::CRenderSettings::getQualityLevel(void)const")
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
pub fn stub_b3b4() -> ! {
    todo!("0xb3b4 RBX::CRenderSettings::getAlwaysDrawConnectors(void)const")
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
pub fn stub_b3e0() -> ! {
    todo!("0xb3e0 RBX::CRenderSettings::getShowAggregation(void)const")
}

// 0xb3e8 — __ZNK3RBX15CRenderSettings12getAASamplesEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getAASamples(void)const")]
pub fn stub_b3e8() -> ! {
    todo!("0xb3e8 RBX::CRenderSettings::getAASamples(void)const")
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
pub fn stub_b41c() -> ! {
    todo!("0xb41c RBX::CRenderSettings::getShadowMode(void)const")
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
pub fn stub_b444() -> ! {
    todo!("0xb444 RBX::CRenderSettings::getAntialiasingMode(void)const")
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
pub fn stub_b46c() -> ! {
    todo!("0xb46c RBX::CRenderSettings::getDebugShowBoundingBoxes(void)const")
}

// 0xb474 — __ZNK3RBX15CRenderSettings19getAutoQualityLevelEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getAutoQualityLevel(void)const")]
pub fn stub_b474() -> ! {
    todo!("0xb474 RBX::CRenderSettings::getAutoQualityLevel(void)const")
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
pub fn stub_b49c() -> ! {
    todo!("0xb49c RBX::CRenderSettings::getEnableFRM(void)const")
}

// 0xb4a4 — __ZNK3RBX15CRenderSettings23getResolutionPreferenceEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getResolutionPreference(void)const")]
pub fn stub_b4a4() -> ! {
    todo!("0xb4a4 RBX::CRenderSettings::getResolutionPreference(void)const")
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
pub fn stub_b4cc() -> ! {
    todo!("0xb4cc RBX::CRenderSettings::getMaxQualityLevel(void)")
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
pub fn stub_b4f4() -> ! {
    todo!("0xb4f4 RBX::CRenderSettings::getTextureCacheSize(void)const")
}

// 0xb4f8 — __ZNK3RBX15CRenderSettings16getMeshCacheSizeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getMeshCacheSize(void)const")]
pub fn stub_b4f8() -> ! {
    todo!("0xb4f8 RBX::CRenderSettings::getMeshCacheSize(void)const")
}

// 0xb4fc — __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEEC2Ev
// type: RBX::Instance *__fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEEC2Ev")]
pub fn stub_b4fc() -> ! {
    todo!("0xb4fc __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEEC2Ev")
}

// 0xb740 — __ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE9push_backERKS1_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::push_back(G3D::Vector2int16 const&)")]
pub fn stub_b740() -> ! {
    todo!("0xb740 std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::push_back(G3D::Vector2int16 const&)")
}

// 0xb76c — __ZN3rbx7signals16signal_with_argsILi1EFvPKN3RBX10Reflection18PropertyDescriptorEEEclES6_
// type: void __fastcall(_DWORD *, int, int, const void *, int, int, int, int, void *, int)
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Reflection::PropertyDescriptor const*)>::operator()(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_b76c() -> ! {
    todo!("0xb76c rbx::signals::signal_with_args<1,void ()(RBX::Reflection::PropertyDescriptor const*)>::operator()(RBX::Reflection::PropertyDescriptor const*)")
}

// 0xb8b0 — __ZNK3RBX15CRenderSettings21getEagerBulkExecutionEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getEagerBulkExecution(void)const")]
pub fn stub_b8b0() -> ! {
    todo!("0xb8b0 RBX::CRenderSettings::getEagerBulkExecution(void)const")
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
pub fn stub_b8d0() -> ! {
    todo!("0xb8d0 __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv")
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
pub fn stub_b94c() -> ! {
    todo!("0xb94c RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::lookup(char const*)const")
}

// 0xb97c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_b97c() -> ! {
    todo!("0xb97c RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::lookup(RBX::Reflection::Variant const&)const")
}

// 0xb99c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_b99c() -> ! {
    todo!("0xb99c RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0xb9f8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToString(unsigned long,std::string &)const")]
pub fn stub_b9f8() -> ! {
    todo!("0xb9f8 RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToString(unsigned long,std::string &)const")
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