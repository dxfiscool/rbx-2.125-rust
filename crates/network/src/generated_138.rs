//! Auto-generated skeletons for rbx-network — filler global ascending EA-sorted
//! Filter: RakNet|RBX::Network|Replicator (case-insensitive) -> 4797 funcs, 4797 already stubbed (0 remaining before batch); filler global ascending
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +100 stubs | range 0x84e0..0xf6fb4c | existing 16229 -> 16329 total (filler global ascending EA-sorted, rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use rbx_core::signal::Signal;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

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

// 0x9ae8 — __ZThn96_N19CRenderSettingsItem19setAutoQualityLevelEi
// type: int __fastcall(int this, int)
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::setAutoQualityLevel(int)")]
pub fn stub_9ae8() -> ! {
    todo!("0x9ae8 __ZThn96_N19CRenderSettingsItem19setAutoQualityLevelEi")
}

// 0x9b08 — __ZN19CRenderSettingsItem21setEagerBulkExecutionEb
// type: int __fastcall(int this, int)
#[doc(alias = "CRenderSettingsItem::setEagerBulkExecution(bool)")]
pub fn stub_9b08() -> ! {
    todo!("0x9b08 __ZN19CRenderSettingsItem21setEagerBulkExecutionEb")
}

// 0x9b2c — __ZNSt12length_errorD1Ev
// type: void __cdecl(std::length_error *__hidden this)
#[doc(alias = "std::length_error::~length_error()")]
pub fn stub_9b2c() -> ! {
    todo!("0x9b2c __ZNSt12length_errorD1Ev")
}

// 0x9b30 — __ZNSt12out_of_rangeD0Ev
// type: void __cdecl(std::out_of_range *__hidden this)
#[doc(alias = "std::out_of_range::~out_of_range()")]
pub fn stub_9b30() -> ! {
    todo!("0x9b30 __ZNSt12out_of_rangeD0Ev")
}

// 0x9b44 — __ZNSt12out_of_rangeD2Ev
// type: void __cdecl(std::out_of_range *__hidden this)
#[doc(alias = "std::out_of_range::~out_of_range()")]
pub fn stub_9b44() -> ! {
    todo!("0x9b44 __ZNSt12out_of_rangeD2Ev")
}

// 0x9b48 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::addPair(RBX::CRenderSettings::AASamples,char const*)")]
pub fn stub_9b48() -> ! {
    todo!("0x9b48 __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE7addPairES3_PKc")
}

// 0x9ea8 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::addPair(RBX::CRenderSettings::GraphicsMode,char const*)")]
pub fn stub_9ea8() -> ! {
    todo!("0x9ea8 __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE7addPairES3_PKc")
}

// 0xa208 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE9addLegacyEiPKcS3_
// type: _DWORD *__fastcall(int, unsigned int, int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::addLegacy(int,char const*,RBX::CRenderSettings::GraphicsMode)")]
pub fn stub_a208() -> ! {
    todo!("0xa208 __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE9addLegacyEiPKcS3_")
}

// 0xa25c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::addPair(RBX::CRenderSettings::FrameRateManagerMode,char const*)")]
pub fn stub_a25c() -> ! {
    todo!("0xa25c __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE7addPairES3_PKc")
}

// 0xa5bc — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::addPair(RBX::CRenderSettings::AntialiasingMode,char const*)")]
pub fn stub_a5bc() -> ! {
    todo!("0xa5bc __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE7addPairES3_PKc")
}

// 0xa91c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::addPair(RBX::CRenderSettings::ShadowMode,char const*)")]
pub fn stub_a91c() -> ! {
    todo!("0xa91c __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE7addPairES3_PKc")
}

// 0xac7c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::addPair(RBX::CRenderSettings::QualityLevel,char const*)")]
pub fn stub_ac7c() -> ! {
    todo!("0xac7c __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE7addPairES3_PKc")
}

// 0xafdc — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::addPair(RBX::CRenderSettings::ResolutionPreset,char const*)")]
pub fn stub_afdc() -> ! {
    todo!("0xafdc __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE7addPairES3_PKc")
}

// 0xb33c — __ZNK3RBX15CRenderSettings15getGraphicsModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getGraphicsMode(void)const")]
pub fn stub_b33c() -> ! {
    todo!("0xb33c __ZNK3RBX15CRenderSettings15getGraphicsModeEv")
}

// 0xb340 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::~EnumPropDescriptor()")]
pub fn stub_b340() -> ! {
    todo!("0xb340 __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEED1Ev")
}

// 0xb364 — __ZNK3RBX15CRenderSettings23getFrameRateManagerModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getFrameRateManagerMode(void)const")]
pub fn stub_b364() -> ! {
    todo!("0xb364 __ZNK3RBX15CRenderSettings23getFrameRateManagerModeEv")
}

// 0xb368 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::~EnumPropDescriptor()")]
pub fn stub_b368() -> ! {
    todo!("0xb368 __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEED1Ev")
}

// 0xb38c — __ZNK3RBX15CRenderSettings15getQualityLevelEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getQualityLevel(void)const")]
pub fn stub_b38c() -> ! {
    todo!("0xb38c __ZNK3RBX15CRenderSettings15getQualityLevelEv")
}

// 0xb390 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::~EnumPropDescriptor()")]
pub fn stub_b390() -> ! {
    todo!("0xb390 __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEED1Ev")
}

// 0xb3b4 — __ZNK3RBX15CRenderSettings23getAlwaysDrawConnectorsEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getAlwaysDrawConnectors(void)const")]
pub fn stub_b3b4() -> ! {
    todo!("0xb3b4 __ZNK3RBX15CRenderSettings23getAlwaysDrawConnectorsEv")
}

// 0xb3bc — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::~PropDescriptor()")]
pub fn stub_b3bc() -> ! {
    todo!("0xb3bc __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembED1Ev")
}

// 0xb3e0 — __ZNK3RBX15CRenderSettings18getShowAggregationEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getShowAggregation(void)const")]
pub fn stub_b3e0() -> ! {
    todo!("0xb3e0 __ZNK3RBX15CRenderSettings18getShowAggregationEv")
}

// 0xb3e8 — __ZNK3RBX15CRenderSettings12getAASamplesEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getAASamples(void)const")]
pub fn stub_b3e8() -> ! {
    todo!("0xb3e8 __ZNK3RBX15CRenderSettings12getAASamplesEv")
}

// 0xb3f8 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::~EnumPropDescriptor()")]
pub fn stub_b3f8() -> ! {
    todo!("0xb3f8 __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEED1Ev")
}

// 0xb41c — __ZNK3RBX15CRenderSettings13getShadowModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getShadowMode(void)const")]
pub fn stub_b41c() -> ! {
    todo!("0xb41c __ZNK3RBX15CRenderSettings13getShadowModeEv")
}

// 0xb420 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::~EnumPropDescriptor()")]
pub fn stub_b420() -> ! {
    todo!("0xb420 __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEED1Ev")
}

// 0xb444 — __ZNK3RBX15CRenderSettings19getAntialiasingModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getAntialiasingMode(void)const")]
pub fn stub_b444() -> ! {
    todo!("0xb444 __ZNK3RBX15CRenderSettings19getAntialiasingModeEv")
}

// 0xb448 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::~EnumPropDescriptor()")]
pub fn stub_b448() -> ! {
    todo!("0xb448 __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEED1Ev")
}

// 0xb46c — __ZNK3RBX15CRenderSettings25getDebugShowBoundingBoxesEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getDebugShowBoundingBoxes(void)const")]
pub fn stub_b46c() -> ! {
    todo!("0xb46c __ZNK3RBX15CRenderSettings25getDebugShowBoundingBoxesEv")
}

// 0xb474 — __ZNK3RBX15CRenderSettings19getAutoQualityLevelEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getAutoQualityLevel(void)const")]
pub fn stub_b474() -> ! {
    todo!("0xb474 __ZNK3RBX15CRenderSettings19getAutoQualityLevelEv")
}

// 0xb478 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::~PropDescriptor()")]
pub fn stub_b478() -> ! {
    todo!("0xb478 __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiED1Ev")
}

// 0xb49c — __ZNK3RBX15CRenderSettings12getEnableFRMEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getEnableFRM(void)const")]
pub fn stub_b49c() -> ! {
    todo!("0xb49c __ZNK3RBX15CRenderSettings12getEnableFRMEv")
}

// 0xb4a4 — __ZNK3RBX15CRenderSettings23getResolutionPreferenceEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getResolutionPreference(void)const")]
pub fn stub_b4a4() -> ! {
    todo!("0xb4a4 __ZNK3RBX15CRenderSettings23getResolutionPreferenceEv")
}

// 0xb4a8 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::~EnumPropDescriptor()")]
pub fn stub_b4a8() -> ! {
    todo!("0xb4a8 __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEED1Ev")
}

// 0xb4cc — __ZN3RBX15CRenderSettings18getMaxQualityLevelEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getMaxQualityLevel(void)")]
pub fn stub_b4cc() -> ! {
    todo!("0xb4cc __ZN3RBX15CRenderSettings18getMaxQualityLevelEv")
}

// 0xf6f978 — sub_F6F978
#[doc(alias = "sub_F6F978")]
pub fn stub_f6f978() -> ! {
    todo!("0xf6f978 sub_F6F978")
}

// 0xf6f984 — sub_F6F984
#[doc(alias = "sub_F6F984")]
pub fn stub_f6f984() -> ! {
    todo!("0xf6f984 sub_F6F984")
}

// 0xf6f990 — sub_F6F990
#[doc(alias = "sub_F6F990")]
pub fn stub_f6f990() -> ! {
    todo!("0xf6f990 sub_F6F990")
}

// 0xf6f99c — sub_F6F99C
#[doc(alias = "sub_F6F99C")]
pub fn stub_f6f99c() -> ! {
    todo!("0xf6f99c sub_F6F99C")
}

// 0xf6f9a8 — sub_F6F9A8
#[doc(alias = "sub_F6F9A8")]
pub fn stub_f6f9a8() -> ! {
    todo!("0xf6f9a8 sub_F6F9A8")
}

// 0xf6f9b4 — sub_F6F9B4
#[doc(alias = "sub_F6F9B4")]
pub fn stub_f6f9b4() -> ! {
    todo!("0xf6f9b4 sub_F6F9B4")
}

// 0xf6f9c0 — sub_F6F9C0
#[doc(alias = "sub_F6F9C0")]
pub fn stub_f6f9c0() -> ! {
    todo!("0xf6f9c0 sub_F6F9C0")
}

// 0xf6f9cc — sub_F6F9CC
#[doc(alias = "sub_F6F9CC")]
pub fn stub_f6f9cc() -> ! {
    todo!("0xf6f9cc sub_F6F9CC")
}

// 0xf6f9d8 — sub_F6F9D8
#[doc(alias = "sub_F6F9D8")]
pub fn stub_f6f9d8() -> ! {
    todo!("0xf6f9d8 sub_F6F9D8")
}

// 0xf6f9e4 — sub_F6F9E4
#[doc(alias = "sub_F6F9E4")]
pub fn stub_f6f9e4() -> ! {
    todo!("0xf6f9e4 sub_F6F9E4")
}

// 0xf6f9f0 — sub_F6F9F0
#[doc(alias = "sub_F6F9F0")]
pub fn stub_f6f9f0() -> ! {
    todo!("0xf6f9f0 sub_F6F9F0")
}

// 0xf6f9fc — sub_F6F9FC
#[doc(alias = "sub_F6F9FC")]
pub fn stub_f6f9fc() -> ! {
    todo!("0xf6f9fc sub_F6F9FC")
}

// 0xf6fa08 — sub_F6FA08
#[doc(alias = "sub_F6FA08")]
pub fn stub_f6fa08() -> ! {
    todo!("0xf6fa08 sub_F6FA08")
}

// 0xf6fa14 — sub_F6FA14
#[doc(alias = "sub_F6FA14")]
pub fn stub_f6fa14() -> ! {
    todo!("0xf6fa14 sub_F6FA14")
}

// 0xf6fa20 — sub_F6FA20
#[doc(alias = "sub_F6FA20")]
pub fn stub_f6fa20() -> ! {
    todo!("0xf6fa20 sub_F6FA20")
}

// 0xf6fa2c — sub_F6FA2C
#[doc(alias = "sub_F6FA2C")]
pub fn stub_f6fa2c() -> ! {
    todo!("0xf6fa2c sub_F6FA2C")
}

// 0xf6fa38 — sub_F6FA38
#[doc(alias = "sub_F6FA38")]
pub fn stub_f6fa38() -> ! {
    todo!("0xf6fa38 sub_F6FA38")
}

// 0xf6fa44 — sub_F6FA44
#[doc(alias = "sub_F6FA44")]
pub fn stub_f6fa44() -> ! {
    todo!("0xf6fa44 sub_F6FA44")
}

// 0xf6fa50 — sub_F6FA50
#[doc(alias = "sub_F6FA50")]
pub fn stub_f6fa50() -> ! {
    todo!("0xf6fa50 sub_F6FA50")
}

// 0xf6fa5c — sub_F6FA5C
#[doc(alias = "sub_F6FA5C")]
pub fn stub_f6fa5c() -> ! {
    todo!("0xf6fa5c sub_F6FA5C")
}

// 0xf6fa68 — sub_F6FA68
#[doc(alias = "sub_F6FA68")]
pub fn stub_f6fa68() -> ! {
    todo!("0xf6fa68 sub_F6FA68")
}

// 0xf6fa74 — sub_F6FA74
#[doc(alias = "sub_F6FA74")]
pub fn stub_f6fa74() -> ! {
    todo!("0xf6fa74 sub_F6FA74")
}

// 0xf6fa80 — sub_F6FA80
#[doc(alias = "sub_F6FA80")]
pub fn stub_f6fa80() -> ! {
    todo!("0xf6fa80 sub_F6FA80")
}

// 0xf6fa8c — sub_F6FA8C
#[doc(alias = "sub_F6FA8C")]
pub fn stub_f6fa8c() -> ! {
    todo!("0xf6fa8c sub_F6FA8C")
}

// 0xf6fa98 — sub_F6FA98
#[doc(alias = "sub_F6FA98")]
pub fn stub_f6fa98() -> ! {
    todo!("0xf6fa98 sub_F6FA98")
}

// 0xf6faa4 — sub_F6FAA4
#[doc(alias = "sub_F6FAA4")]
pub fn stub_f6faa4() -> ! {
    todo!("0xf6faa4 sub_F6FAA4")
}

// 0xf6fab0 — sub_F6FAB0
#[doc(alias = "sub_F6FAB0")]
pub fn stub_f6fab0() -> ! {
    todo!("0xf6fab0 sub_F6FAB0")
}

// 0xf6fabc — sub_F6FABC
#[doc(alias = "sub_F6FABC")]
pub fn stub_f6fabc() -> ! {
    todo!("0xf6fabc sub_F6FABC")
}

// 0xf6fac8 — sub_F6FAC8
#[doc(alias = "sub_F6FAC8")]
pub fn stub_f6fac8() -> ! {
    todo!("0xf6fac8 sub_F6FAC8")
}

// 0xf6fad4 — sub_F6FAD4
#[doc(alias = "sub_F6FAD4")]
pub fn stub_f6fad4() -> ! {
    todo!("0xf6fad4 sub_F6FAD4")
}

// 0xf6fae0 — sub_F6FAE0
#[doc(alias = "sub_F6FAE0")]
pub fn stub_f6fae0() -> ! {
    todo!("0xf6fae0 sub_F6FAE0")
}

// 0xf6faec — sub_F6FAEC
#[doc(alias = "sub_F6FAEC")]
pub fn stub_f6faec() -> ! {
    todo!("0xf6faec sub_F6FAEC")
}

// 0xf6faf8 — sub_F6FAF8
#[doc(alias = "sub_F6FAF8")]
pub fn stub_f6faf8() -> ! {
    todo!("0xf6faf8 sub_F6FAF8")
}

// 0xf6fb04 — sub_F6FB04
#[doc(alias = "sub_F6FB04")]
pub fn stub_f6fb04() -> ! {
    todo!("0xf6fb04 sub_F6FB04")
}

// 0xf6fb10 — sub_F6FB10
#[doc(alias = "sub_F6FB10")]
pub fn stub_f6fb10() -> ! {
    todo!("0xf6fb10 sub_F6FB10")
}

// 0xf6fb1c — sub_F6FB1C
#[doc(alias = "sub_F6FB1C")]
pub fn stub_f6fb1c() -> ! {
    todo!("0xf6fb1c sub_F6FB1C")
}

// 0xf6fb28 — sub_F6FB28
#[doc(alias = "sub_F6FB28")]
pub fn stub_f6fb28() -> ! {
    todo!("0xf6fb28 sub_F6FB28")
}

// 0xf6fb34 — sub_F6FB34
#[doc(alias = "sub_F6FB34")]
pub fn stub_f6fb34() -> ! {
    todo!("0xf6fb34 sub_F6FB34")
}

// 0xf6fb40 — sub_F6FB40
#[doc(alias = "sub_F6FB40")]
pub fn stub_f6fb40() -> ! {
    todo!("0xf6fb40 sub_F6FB40")
}

// 0xf6fb4c — sub_F6FB4C
// type: int()
#[doc(alias = "sub_F6FB4C")]
pub fn stub_f6fb4c() -> ! {
    todo!("0xf6fb4c sub_F6FB4C")
}
