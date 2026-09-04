#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
//! core watchdog k — 100 core stubs EA-sorted, next gap filler after watchdog_j 0x744db0.
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_core — next 100 uncovered after 0x744db0 (watchdog_j max).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;
/// Batch 1: 24 IDA-grounded ports 0x84e0-0x97d0 — the process entry (`start`),
/// the seven `RBX::CRenderSettings` `EnumDesc<T>` constructor tables (AASamples,
/// GraphicsMode, FrameRateManagerMode, AntialiasingMode, ShadowMode, QualityLevel,
/// ResolutionPreset) and the `CRenderSettingsItem` field setters + ctor. Ports live
/// in `render_settings` under idiomatic names, wired via `stub_84e0`-`stub_97d0`;
/// untouched carriers keep stub bodies.
/// Conventions: `boost::shared_ptr` -> `crate::SharedPtr` (kept via `_SHARED_PTR`
/// carrier); `rbx::signals::signal_with_args<1, ...>::operator()` ->
/// `PropertyChangedSignal::emit` (records the property id, then notifies listeners);
/// `RBX::CRenderSettings::aaSamples` / `RBX::PartInstance::disableInterpolation`
/// globals -> atomics; `std::map` descriptor tables -> insertion-ordered `Vec`s;
/// `__cxa`/throws -> none (every ported path is total). `[INFERENCE]` marks what the
/// binary does not pin down; everything else follows the IDA pseudocode branch-for-branch.
pub mod render_settings {
    use std::os::raw::{c_char, c_int};
    use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

    /// IDA 0x84e0 `start` — raw process entry (`void __fastcall __noreturn start(...)`).
    /// `argv` points at the stacked `[argc, argv0, ..., NULL, envp0, ..., NULL]` block.
    pub mod c_runtime {
        use super::{c_char, c_int};

        /// was: `main(argc, argv, envp)` — the hosted entry, kept as a callee parameter
        /// so the port stays total without linking a real `main`.
        pub type MainFn =
            unsafe extern "C" fn(c_int, *const *const c_char, *const *const c_char) -> c_int;

        /// IDA 0x84e0: `v6 = &argv[argc + 1]; while (*v6++);` then
        /// `v8 = main(argc, argv, &argv[argc + 1]); exit(v8);` (0x84f4-0x8508).
        /// The scan walk is vestigial — `main` receives `&argv[argc + 1]` directly,
        /// never the scanned end — but it is preserved branch-for-branch.
        pub unsafe fn start(argc: c_int, argv: *const *const c_char, main: MainFn) -> ! {
            let mut scan = argv.add(argc as usize + 1);
            while !(*scan).is_null() {
                scan = scan.add(1);
            }
            let _ = scan;
            let code = main(argc, argv, argv.add(argc as usize + 1));
            std::process::exit(code);
        }
    }

    /// was: `RBX::CRenderSettings::AASamples` — IDA 0x850c pairs (1,None),(4,4),(8,8).
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(i32)]
    pub enum AaSamples {
        #[default]
        None = 1,
        Msaa4x = 4,
        Msaa8x = 8,
    }

    /// was: `RBX::CRenderSettings::GraphicsMode` — IDA 0x86d0 pairs + one legacy entry.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(i32)]
    pub enum GraphicsMode {
        #[default]
        Automatic = 1,
        /// IDA 0x880e `addLegacy(2, "OpenGL legacy", 1)` — kept as a regular variant;
        /// the legacy flag lives in the descriptor table, not the value.
        OpenGlLegacy = 2,
        Direct3D = 3,
        OpenGL = 4,
        NoGraphics = 5,
    }

    /// was: `RBX::CRenderSettings::FrameRateManagerMode` — IDA 0x88c4.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(i32)]
    pub enum FrameRateManagerMode {
        #[default]
        Automatic = 0,
        On = 1,
        Off = 2,
    }

    /// was: `RBX::CRenderSettings::AntialiasingMode` — IDA 0x8a88.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(i32)]
    pub enum AntialiasingMode {
        #[default]
        Automatic = 0,
        On = 1,
        Off = 2,
    }

    /// was: `RBX::CRenderSettings::ShadowMode` — IDA 0x8c4c.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(i32)]
    pub enum ShadowMode {
        #[default]
        Automatic = 0,
        All = 1,
        Off = 2,
        CharacterOnly = 3,
    }

    /// was: `RBX::CRenderSettings::QualityLevel` — IDA 0x8e24: Automatic + Level01..Level21.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(i32)]
    pub enum QualityLevel {
        #[default]
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

    /// was: `RBX::CRenderSettings::ResolutionPreset` — IDA 0x9100 pairs 0..18.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    #[repr(i32)]
    pub enum ResolutionPreset {
        #[default]
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

    /// was: `RBX::Reflection::EnumDesc<T>` runtime table — the `std::map<const Name*, T>`
    /// plus legacy/alias maps each ctor fills via `addPair`/`addLegacy`. Insertion-ordered
    /// `Vec`s preserve the IDA declaration order (lookup is linear, matching the tiny
    /// table sizes; `[INFERENCE]` — the original maps are tree-ordered, observable
    /// lookups are identical).
    #[derive(Debug, Clone, Default)]
    pub struct EnumDescData {
        pub desc_name: &'static str,
        /// was: the installed vtable (`*a1 = &off_...` at each ctor tail).
        pub vtable: &'static str,
        pub pairs: Vec<(i32, String)>,
        /// was: `addLegacy(value, name, flag)` entries.
        pub legacy: Vec<(i32, String, i32)>,
        /// was: extra `Name::declare` + `map::operator[]` alias inserts (the " (wide)"
        /// and "Level NN" spellings).
        pub aliases: Vec<(String, i32)>,
    }

    impl EnumDescData {
        /// was: `EnumDesc<T>::addPair(value, name)`.
        pub fn add_pair(&mut self, value: i32, name: &str) {
            self.pairs.push((value, name.to_string()));
        }
        /// was: `EnumDesc<T>::addLegacy(value, name, flag)` (IDA 0x880e).
        pub fn add_legacy(&mut self, value: i32, name: &str, flag: i32) {
            self.legacy.push((value, name.to_string(), flag));
        }
        /// was: `Name::declare` + `map::operator[] = value` alias insert.
        pub fn add_alias(&mut self, name: &str, value: i32) {
            self.aliases.push((name.to_string(), value));
        }
        /// Pair table first, then " (wide)"/spaced aliases — mirrors the two-map lookup.
        pub fn lookup_value(&self, name: &str) -> Option<i32> {
            self.pairs
                .iter()
                .find(|(_, n)| n == name)
                .map(|(v, _)| *v)
                .or_else(|| {
                    self.aliases
                        .iter()
                        .find(|(n, _)| n == name)
                        .map(|(_, v)| *v)
                })
        }
        pub fn lookup_name(&self, value: i32) -> Option<&str> {
            self.pairs
                .iter()
                .find(|(v, _)| *v == value)
                .map(|(_, n)| n.as_str())
        }
    }

    /// IDA 0x850c `EnumDesc<AASamples>::EnumDesc` — base `EnumDescriptor("AASamples")`,
    /// vtable `off_1221308`, empty-table init, then the three `addPair` calls.
    /// (Name-only coverage also exists in `rbx_reflection::enum_desc`; the pair table
    /// here is the complementary detail.)
    pub fn aa_samples_desc() -> EnumDescData {
        let mut d = EnumDescData {
            desc_name: "AASamples",
            vtable: "off_1221308",
            ..Default::default()
        };
        d.add_pair(1, "None");
        d.add_pair(4, "4");
        d.add_pair(8, "8");
        d
    }

    /// IDA 0x86d0 `EnumDesc<GraphicsMode>::EnumDesc` — vtable `off_1221338`.
    pub fn graphics_mode_desc() -> EnumDescData {
        let mut d = EnumDescData {
            desc_name: "GraphicsMode",
            vtable: "off_1221338",
            ..Default::default()
        };
        d.add_pair(1, "Automatic");
        d.add_pair(3, "Direct3D");
        d.add_pair(4, "OpenGL");
        d.add_pair(5, "NoGraphics");
        d.add_legacy(2, "OpenGL legacy", 1);
        d
    }

    /// IDA 0x88c4 `EnumDesc<FrameRateManagerMode>::EnumDesc` — vtable `off_1221368`.
    /// Note the original descriptor string is "FramerateManagerMode" (lowercase r).
    pub fn frame_rate_manager_mode_desc() -> EnumDescData {
        let mut d = EnumDescData {
            desc_name: "FramerateManagerMode",
            vtable: "off_1221368",
            ..Default::default()
        };
        d.add_pair(0, "Automatic");
        d.add_pair(1, "On");
        d.add_pair(2, "Off");
        d
    }

    /// IDA 0x8a88 `EnumDesc<AntialiasingMode>::EnumDesc` — vtable `off_1221398`,
    /// descriptor string "Antialiasing".
    pub fn antialiasing_mode_desc() -> EnumDescData {
        let mut d = EnumDescData {
            desc_name: "Antialiasing",
            vtable: "off_1221398",
            ..Default::default()
        };
        d.add_pair(0, "Automatic");
        d.add_pair(2, "Off");
        d.add_pair(1, "On");
        d
    }

    /// IDA 0x8c4c `EnumDesc<ShadowMode>::EnumDesc` — vtable `off_12213C8`,
    /// descriptor string "Shadow".
    pub fn shadow_mode_desc() -> EnumDescData {
        let mut d = EnumDescData {
            desc_name: "Shadow",
            vtable: "off_12213C8",
            ..Default::default()
        };
        d.add_pair(0, "Automatic");
        d.add_pair(1, "All");
        d.add_pair(3, "CharacterOnly");
        d.add_pair(2, "Off");
        d
    }

    /// IDA 0x8e24 `EnumDesc<QualityLevel>::EnumDesc` — vtable `off_12213F8`.
    /// `(0, "Automatic")`, then `RBX::format("Level%.2d", i)` pairs for 1..=21
    /// (0x8f28 loop), then the spaced-alias loop (0x8ff2): `snprintf("Level 00"+6,
    /// "%2u", i)` + `Name::declare` mapping to the same values.
    pub fn quality_level_desc() -> EnumDescData {
        let mut d = EnumDescData {
            desc_name: "QualityLevel",
            vtable: "off_12213F8",
            ..Default::default()
        };
        d.add_pair(0, "Automatic");
        for i in 1..=21 {
            d.add_pair(i, &format!("Level{i:02}"));
        }
        for i in 1..=21 {
            d.add_alias(&format!("Level {i:2}"), i);
        }
        d
    }

    /// IDA 0x9100 `EnumDesc<ResolutionPreset>::EnumDesc` — vtable `off_1221428`,
    /// descriptor string "Resolution". Pairs 0..18 in order with the " (wide)"
    /// `Name::declare` aliases interleaved exactly as emitted (0x923e-0x9534).
    pub fn resolution_preset_desc() -> EnumDescData {
        let mut d = EnumDescData {
            desc_name: "Resolution",
            vtable: "off_1221428",
            ..Default::default()
        };
        d.add_pair(0, "Automatic");
        d.add_pair(1, "720x526");
        d.add_pair(2, "800x600");
        d.add_pair(3, "1024x600");
        d.add_alias("1024x600 (wide)", 3);
        d.add_pair(4, "1024x768");
        d.add_pair(5, "1280x720");
        d.add_alias("1280x720 (wide)", 5);
        d.add_pair(6, "1280x768");
        d.add_alias("1280x768 (wide)", 6);
        d.add_pair(7, "1152x864");
        d.add_pair(8, "1280x800");
        d.add_alias("1280x800 (wide)", 8);
        d.add_pair(9, "1360x768");
        d.add_alias("1360x768 (wide)", 9);
        d.add_pair(10, "1280x960");
        d.add_pair(11, "1280x1024");
        d.add_pair(12, "1440x900");
        d.add_alias("1440x900 (wide)", 12);
        d.add_pair(13, "1600x900");
        d.add_alias("1600x900 (wide)", 13);
        d.add_pair(14, "1600x1024");
        d.add_alias("1600x1024 (wide)", 14);
        d.add_pair(15, "1600x1200");
        d.add_pair(16, "1680x1050");
        d.add_alias("1680x1050 (wide)", 16);
        d.add_pair(17, "1920x1080");
        d.add_alias("1920x1080 (wide)", 17);
        d.add_pair(18, "1920x1200");
        d.add_alias("1920x1200 (wide)", 18);
        d
    }

    /// was: `rbx::signals::signal_with_args<1, void(PropertyDescriptor const*)>` —
    /// the per-item `propertyChanged` signal at +192. `emit` records the property id
    /// (observable for tests) then notifies listeners in connect order.
    /// (`[INFERENCE]` — listener storage shape; emission order and change-only
    /// gating are per the IDA call sites.)
    #[derive(Default)]
    pub struct PropertyChangedSignal {
        pub emitted: Vec<&'static str>,
        listeners: Vec<Box<dyn Fn(&'static str) + Send + Sync>>,
    }

    impl PropertyChangedSignal {
        pub fn emit(&mut self, prop: &'static str) {
            self.emitted.push(prop);
            for listener in &self.listeners {
                listener(prop);
            }
        }
        pub fn connect(&mut self, f: impl Fn(&'static str) + Send + Sync + 'static) {
            self.listeners.push(Box::new(f));
        }
    }

    /// was: `unk_130C***` property-descriptor ids passed to the signal `operator()`.
    pub const PROP_ALWAYS_DRAW_CONNECTORS: &str = "unk_130C030";
    pub const PROP_SHOW_AGGREGATION: &str = "unk_130C05C";
    pub const PROP_DEBUG_SHOW_BOUNDING_BOXES: &str = "unk_130C0E0";
    pub const PROP_ENABLE_FRM: &str = "unk_130C138";
    pub const PROP_GRAPHICS_MODE: &str = "unk_130C244";
    pub const PROP_FRAME_RATE_MANAGER_MODE: &str = "unk_130C278";
    pub const PROP_QUALITY_LEVEL: &str = "unk_130C2AC";
    pub const PROP_AA_SAMPLES: &str = "unk_130C2E0";
    pub const PROP_SHADOW_MODE: &str = "unk_130C314";
    pub const PROP_ANTIALIASING_MODE: &str = "unk_130C348";
    /// was: `CRenderSettingsItem::prop_resolution` (IDA 0x97bc).
    pub const PROP_RESOLUTION: &str = "CRenderSettingsItem::prop_resolution";

    /// was: `RBX::CRenderSettings::aaSamples` — module-global backing the 0x96d0 setter.
    /// Initial value `[INFERENCE]` (no writer observed before first set).
    pub static AA_SAMPLES: AtomicI32 = AtomicI32::new(AaSamples::None as i32);
    /// was: `RBX::PartInstance::disableInterpolation` — read by 0x9784, written by 0x9794.
    /// Initial value `[INFERENCE]`.
    pub static DISABLE_INTERPOLATION: AtomicBool = AtomicBool::new(false);

    /// IDA 0x993a threshold: `GetDXVideoMemorySize() > 0xF423FF` (15,999,999).
    pub const DX_VIDEO_MEMORY_THRESHOLD: u32 = 0xF423FF;
    /// IDA 0x9926/0x993c stored words at +146 (purpose `[INFERENCE]` — named by site).
    pub const VIDEO_BUDGET_LOW: u32 = 39_322_400;
    pub const VIDEO_BUDGET_HIGH: u32 = 50_332_672;

    /// was: `CRenderSettingsItem` — field offsets from the setter/ctor decompiles.
    /// The `+96` `CRenderSettings` subobject (0x9828 base ctor, not ported here) owns
    /// the +100..+120 words; field defaults below are `[INFERENCE]` except where the
    /// ctor stores them directly (+146 budget, +168 name, +172/174 800x600, +176
    /// resolution list, +189 flag).
    #[derive(Default)]
    pub struct CRenderSettingsItem {
        /// +100, IDA 0x9608.
        pub graphics_mode: GraphicsMode,
        /// +104, IDA 0x971c.
        pub antialiasing_mode: AntialiasingMode,
        /// +108, IDA 0x96fc.
        pub shadow_mode: ShadowMode,
        /// +112, IDA 0x9628.
        pub frame_rate_manager_mode: FrameRateManagerMode,
        /// +116, IDA 0x9648.
        pub quality_level: QualityLevel,
        /// +120, IDA 0x97a4.
        pub resolution_preference: ResolutionPreset,
        /// +136, IDA 0x973c.
        pub debug_show_bounding_boxes: bool,
        /// +137, IDA 0x9760.
        pub enable_frm: bool,
        /// +146, IDA 0x9946 video-memory-derived budget.
        pub video_memory_budget: u32,
        /// +154, IDA 0x96ac.
        pub show_aggregation: bool,
        /// +155, IDA 0x9668.
        pub always_draw_connectors: bool,
        /// +156, IDA 0x9668 second bool (`[INFERENCE]` — purpose unresolvable
        /// from the setter alone; participates in the effective-state compare).
        pub connectors_secondary: bool,
        /// +160, IDA 0x97c0.
        pub texture_cache_size: u32,
        /// +164, IDA 0x97c8.
        pub mesh_cache_size: u32,
        /// +168, IDA 0x98f6/0x9904 `setName("Rendering")`.
        pub name: String,
        /// +172/+174, IDA 0x987e/0x988a: 800x600.
        pub width: u16,
        /// +174.
        pub height: u16,
        /// +176 `std::vector<G3D::Vector2int16>`, seeded by 0x991a push_back.
        pub resolutions: Vec<(u16, u16)>,
        /// +189, IDA 0x98b0 `= 1`.
        pub flag_189: bool,
        /// +192 change signal, IDA 0x98d0 `call_once` init.
        pub changed: PropertyChangedSignal,
    }

    impl CRenderSettingsItem {
        /// IDA 0x97d0 `CRenderSettingsItem::CRenderSettingsItem` — past the base-class
        /// ctors (0x97f0 `GlobalAdvancedSettingsItem`, 0x9828 `CRenderSettings`, vtable
        /// installs `off_11CC118/...`, and the +192 signal `call_once`, all noted not
        /// modeled): name "Rendering", 800x600, one seeded resolution, flag +189 set,
        /// and the `GetDXVideoMemorySize` threshold branch (memory size injected as a
        /// parameter since the platform query lives outside core).
        pub fn new(dx_video_memory_size: u32) -> Self {
            let video_memory_budget = if dx_video_memory_size > DX_VIDEO_MEMORY_THRESHOLD {
                VIDEO_BUDGET_HIGH
            } else {
                VIDEO_BUDGET_LOW
            };
            Self {
                name: "Rendering".to_string(),
                width: 800,
                height: 600,
                resolutions: vec![(800, 600)],
                flag_189: true,
                video_memory_budget,
                ..Default::default()
            }
        }

        /// IDA 0x9608: store +100 when changed, emit `PROP_GRAPHICS_MODE` on +192.
        /// Returns `self` — the original returns `this`.
        pub fn set_graphics_mode(&mut self, mode: GraphicsMode) -> &mut Self {
            if self.graphics_mode != mode {
                self.graphics_mode = mode;
                self.changed.emit(PROP_GRAPHICS_MODE);
            }
            self
        }

        /// IDA 0x9628: store +112 when changed, emit `PROP_FRAME_RATE_MANAGER_MODE`.
        pub fn set_frame_rate_manager_mode(&mut self, mode: FrameRateManagerMode) -> &mut Self {
            if self.frame_rate_manager_mode != mode {
                self.frame_rate_manager_mode = mode;
                self.changed.emit(PROP_FRAME_RATE_MANAGER_MODE);
            }
            self
        }

        /// IDA 0x9648: store +116 when changed, emit `PROP_QUALITY_LEVEL`.
        pub fn set_quality_level(&mut self, level: QualityLevel) -> &mut Self {
            if self.quality_level != level {
                self.quality_level = level;
                self.changed.emit(PROP_QUALITY_LEVEL);
            }
            self
        }

        /// IDA 0x9668: the quirky one. `prev = 1; if (!old155) { prev = old156 != 0; }`;
        /// store 155; when enabling (`a2 == 1`) emit only if `prev == 0`, else compare
        /// `prev` against the new effective state `(155 || 156)` and emit on difference.
        pub fn set_always_draw_connectors(&mut self, value: bool) -> &mut Self {
            let mut prev_effective = 1;
            if !self.always_draw_connectors {
                prev_effective = i32::from(self.connectors_secondary);
                if self.connectors_secondary {
                    prev_effective = 1;
                }
            }
            self.always_draw_connectors = value;
            if value {
                if prev_effective != 0 {
                    return self;
                }
                self.changed.emit(PROP_ALWAYS_DRAW_CONNECTORS);
                return self;
            }
            let mut cur_effective = i32::from(self.connectors_secondary);
            if self.connectors_secondary {
                cur_effective = 1;
            }
            if prev_effective != cur_effective {
                self.changed.emit(PROP_ALWAYS_DRAW_CONNECTORS);
            }
            self
        }

        /// IDA 0x96ac: store byte +154 when changed, emit `PROP_SHOW_AGGREGATION`.
        pub fn set_show_aggregation(&mut self, value: bool) -> &mut Self {
            if value != self.show_aggregation {
                self.show_aggregation = value;
                self.changed.emit(PROP_SHOW_AGGREGATION);
            }
            self
        }

        /// IDA 0x96d0: compares/stores the `RBX::CRenderSettings::aaSamples` global
        /// (not a field), emits `PROP_AA_SAMPLES` on the item signal.
        pub fn set_aa_samples(&mut self, samples: AaSamples) -> &mut Self {
            if AA_SAMPLES.load(Ordering::SeqCst) != samples as i32 {
                AA_SAMPLES.store(samples as i32, Ordering::SeqCst);
                self.changed.emit(PROP_AA_SAMPLES);
            }
            self
        }

        /// IDA 0x96fc: store +108 when changed, emit `PROP_SHADOW_MODE`.
        pub fn set_shadow_mode(&mut self, mode: ShadowMode) -> &mut Self {
            if self.shadow_mode != mode {
                self.shadow_mode = mode;
                self.changed.emit(PROP_SHADOW_MODE);
            }
            self
        }

        /// IDA 0x971c: store +104 when changed, emit `PROP_ANTIALIASING_MODE`.
        pub fn set_antialiasing_mode(&mut self, mode: AntialiasingMode) -> &mut Self {
            if self.antialiasing_mode != mode {
                self.antialiasing_mode = mode;
                self.changed.emit(PROP_ANTIALIASING_MODE);
            }
            self
        }

        /// IDA 0x973c: store byte +136 when changed, emit `PROP_DEBUG_SHOW_BOUNDING_BOXES`.
        pub fn set_debug_show_bounding_boxes(&mut self, value: bool) -> &mut Self {
            if value != self.debug_show_bounding_boxes {
                self.debug_show_bounding_boxes = value;
                self.changed.emit(PROP_DEBUG_SHOW_BOUNDING_BOXES);
            }
            self
        }

        /// IDA 0x9760: store byte +137 when changed, emit `PROP_ENABLE_FRM`.
        pub fn set_enable_frm(&mut self, value: bool) -> &mut Self {
            if value != self.enable_frm {
                self.enable_frm = value;
                self.changed.emit(PROP_ENABLE_FRM);
            }
            self
        }

        /// IDA 0x9784: returns the `RBX::PartInstance::disableInterpolation` global.
        /// (`this` unused.)
        pub fn get_debug_disable_interpolation() -> bool {
            DISABLE_INTERPOLATION.load(Ordering::SeqCst)
        }

        /// IDA 0x9794: stores the global, returns its address — no signal emitted.
        pub fn set_debug_disable_interpolation(value: bool) -> &'static AtomicBool {
            DISABLE_INTERPOLATION.store(value, Ordering::SeqCst);
            &DISABLE_INTERPOLATION
        }

        /// IDA 0x97a4: store +120 when changed, emit `PROP_RESOLUTION`.
        pub fn set_resolution_preference(&mut self, preset: ResolutionPreset) -> &mut Self {
            if self.resolution_preference != preset {
                self.resolution_preference = preset;
                self.changed.emit(PROP_RESOLUTION);
            }
            self
        }

        /// IDA 0x97c0: unconditional store of +160, no signal, returns `this`.
        pub fn set_texture_cache_size(&mut self, size: u32) -> &mut Self {
            self.texture_cache_size = size;
            self
        }

        /// IDA 0x97c8: unconditional store of +164, no signal, returns `this`.
        pub fn set_mesh_cache_size(&mut self, size: u32) -> &mut Self {
            self.mesh_cache_size = size;
            self
        }
    }
}


#[doc(alias = "start")]
// 0x84e0 — start
// type: void __fastcall __noreturn(int, int, int, int, int argc, char *argv)
// IDA 0x84e0: envp scan (vestigial) + `main(argc, argv, envp)` + `exit(code)`.
pub fn stub_84e0(
    argc: std::os::raw::c_int,
    argv: *const *const std::os::raw::c_char,
    main_fn: render_settings::c_runtime::MainFn,
) -> ! {
    unsafe { render_settings::c_runtime::start(argc, argv, main_fn) }
}
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::EnumDesc(void)")]
// 0x850c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEEC2Ev
// type: int __fastcall(int)
// IDA 0x850c: `EnumDescriptor("AASamples")` + vtable off_1221308 + 3 pairs.
pub fn stub_850c() -> render_settings::EnumDescData {
    render_settings::aa_samples_desc()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::EnumDesc(void)")]
// 0x86d0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEEC2Ev
// type: int __fastcall(int)
// IDA 0x86d0: `EnumDescriptor("GraphicsMode")` + vtable off_1221338 + 4 pairs + legacy.
pub fn stub_86d0() -> render_settings::EnumDescData {
    render_settings::graphics_mode_desc()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::EnumDesc(void)")]
// 0x88c4 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEC2Ev
// type: int __fastcall(int)
// IDA 0x88c4: `EnumDescriptor("FramerateManagerMode")` + vtable off_1221368 + 3 pairs.
pub fn stub_88c4() -> render_settings::EnumDescData {
    render_settings::frame_rate_manager_mode_desc()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::EnumDesc(void)")]
// 0x8a88 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEEC2Ev
// type: int __fastcall(int)
// IDA 0x8a88: `EnumDescriptor("Antialiasing")` + vtable off_1221398 + 3 pairs.
pub fn stub_8a88() -> render_settings::EnumDescData {
    render_settings::antialiasing_mode_desc()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::EnumDesc(void)")]
// 0x8c4c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEEC2Ev
// type: int __fastcall(int)
// IDA 0x8c4c: `EnumDescriptor("Shadow")` + vtable off_12213C8 + 4 pairs.
pub fn stub_8c4c() -> render_settings::EnumDescData {
    render_settings::shadow_mode_desc()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::EnumDesc(void)")]
// 0x8e24 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEEC2Ev
// type: RBX::Reflection::EnumDescriptor *__fastcall(RBX::Reflection::EnumDescriptor *)
// IDA 0x8e24: `EnumDescriptor("QualityLevel")` + vtable off_12213F8 + Automatic +
// 21 Level pairs + 21 spaced aliases.
pub fn stub_8e24() -> render_settings::EnumDescData {
    render_settings::quality_level_desc()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::EnumDesc(void)")]
// 0x9100 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEEC2Ev
// type: RBX::Reflection::EnumDescriptor *__fastcall(RBX::Reflection::EnumDescriptor *)
// IDA 0x9100: `EnumDescriptor("Resolution")` + vtable off_1221428 + 19 pairs +
// 11 " (wide)" aliases.
pub fn stub_9100() -> render_settings::EnumDescData {
    render_settings::resolution_preset_desc()
}

#[doc(alias = "CRenderSettingsItem::setGraphicsMode(RBX::CRenderSettings::GraphicsMode)")]
// 0x9608 — __ZN19CRenderSettingsItem15setGraphicsModeEN3RBX15CRenderSettings12GraphicsModeE
// type: int __fastcall(int result, int)
// IDA 0x9608: store +100 when changed, emit on +192. Returns `this`.
pub fn stub_9608(item: &mut render_settings::CRenderSettingsItem, mode: render_settings::GraphicsMode) -> &mut render_settings::CRenderSettingsItem {
    item.set_graphics_mode(mode)
}

#[doc(alias = "CRenderSettingsItem::setFrameRateManagerMode(RBX::CRenderSettings::FrameRateManagerMode)")]
// 0x9628 — __ZN19CRenderSettingsItem23setFrameRateManagerModeEN3RBX15CRenderSettings20FrameRateManagerModeE
// type: int __fastcall(int result, int)
// IDA 0x9628: store +112 when changed, emit on +192. Returns `this`.
pub fn stub_9628(item: &mut render_settings::CRenderSettingsItem, mode: render_settings::FrameRateManagerMode) -> &mut render_settings::CRenderSettingsItem {
    item.set_frame_rate_manager_mode(mode)
}

#[doc(alias = "CRenderSettingsItem::setQualityLevel(RBX::CRenderSettings::QualityLevel)")]
// 0x9648 — __ZN19CRenderSettingsItem15setQualityLevelEN3RBX15CRenderSettings12QualityLevelE
// type: int __fastcall(int result, int)
// IDA 0x9648: store +116 when changed, emit on +192. Returns `this`.
pub fn stub_9648(item: &mut render_settings::CRenderSettingsItem, level: render_settings::QualityLevel) -> &mut render_settings::CRenderSettingsItem {
    item.set_quality_level(level)
}

#[doc(alias = "CRenderSettingsItem::setAlwaysDrawConnectors(bool)")]
// 0x9668 — __ZN19CRenderSettingsItem23setAlwaysDrawConnectorsEb
// type: int __fastcall(int this, int)
// IDA 0x9668: effective-state (+155/+156) compare, emit on +192. Returns `this`.
pub fn stub_9668(item: &mut render_settings::CRenderSettingsItem, value: bool) -> &mut render_settings::CRenderSettingsItem {
    item.set_always_draw_connectors(value)
}

#[doc(alias = "CRenderSettingsItem::setShowAggregation(bool)")]
// 0x96ac — __ZN19CRenderSettingsItem18setShowAggregationEb
// type: int __fastcall(int this, int)
// IDA 0x96ac: store byte +154 when changed, emit on +192. Returns `this`.
pub fn stub_96ac(item: &mut render_settings::CRenderSettingsItem, value: bool) -> &mut render_settings::CRenderSettingsItem {
    item.set_show_aggregation(value)
}

#[doc(alias = "CRenderSettingsItem::setAASamples(RBX::CRenderSettings::AASamples)")]
// 0x96d0 — __ZN19CRenderSettingsItem12setAASamplesEN3RBX15CRenderSettings9AASamplesE
// type: int __fastcall(int result, int)
// IDA 0x96d0: compares/stores the `aaSamples` global, emits on the item signal.
pub fn stub_96d0(item: &mut render_settings::CRenderSettingsItem, samples: render_settings::AaSamples) -> &mut render_settings::CRenderSettingsItem {
    item.set_aa_samples(samples)
}

#[doc(alias = "CRenderSettingsItem::setShadowMode(RBX::CRenderSettings::ShadowMode)")]
// 0x96fc — __ZN19CRenderSettingsItem13setShadowModeEN3RBX15CRenderSettings10ShadowModeE
// type: int __fastcall(int result, int)
// IDA 0x96fc: store +108 when changed, emit on +192. Returns `this`.
pub fn stub_96fc(item: &mut render_settings::CRenderSettingsItem, mode: render_settings::ShadowMode) -> &mut render_settings::CRenderSettingsItem {
    item.set_shadow_mode(mode)
}

#[doc(alias = "CRenderSettingsItem::setAntialiasingMode(RBX::CRenderSettings::AntialiasingMode)")]
// 0x971c — __ZN19CRenderSettingsItem19setAntialiasingModeEN3RBX15CRenderSettings16AntialiasingModeE
// type: int __fastcall(int result, int)
// IDA 0x971c: store +104 when changed, emit on +192. Returns `this`.
pub fn stub_971c(item: &mut render_settings::CRenderSettingsItem, mode: render_settings::AntialiasingMode) -> &mut render_settings::CRenderSettingsItem {
    item.set_antialiasing_mode(mode)
}

#[doc(alias = "CRenderSettingsItem::setDebugShowBoundingBoxes(bool)")]
// 0x973c — __ZN19CRenderSettingsItem25setDebugShowBoundingBoxesEb
// type: int __fastcall(int this, int)
// IDA 0x973c: store byte +136 when changed, emit on +192. Returns `this`.
pub fn stub_973c(item: &mut render_settings::CRenderSettingsItem, value: bool) -> &mut render_settings::CRenderSettingsItem {
    item.set_debug_show_bounding_boxes(value)
}

#[doc(alias = "CRenderSettingsItem::setEnableFRM(bool)")]
// 0x9760 — __ZN19CRenderSettingsItem12setEnableFRMEb
// type: int __fastcall(int this, int)
// IDA 0x9760: store byte +137 when changed, emit on +192. Returns `this`.
pub fn stub_9760(item: &mut render_settings::CRenderSettingsItem, value: bool) -> &mut render_settings::CRenderSettingsItem {
    item.set_enable_frm(value)
}

#[doc(alias = "CRenderSettingsItem::getDebugDisableInterpolation(void)const")]
// 0x9784 — __ZNK19CRenderSettingsItem28getDebugDisableInterpolationEv
// type: int __fastcall(CRenderSettingsItem *this)
// IDA 0x9784: returns the `disableInterpolation` global (`this` unused).
pub fn stub_9784() -> bool {
    render_settings::CRenderSettingsItem::get_debug_disable_interpolation()
}

#[doc(alias = "CRenderSettingsItem::setDebugDisableInterpolation(bool)")]
// 0x9794 — __ZN19CRenderSettingsItem28setDebugDisableInterpolationEb
// type: char *__fastcall(CRenderSettingsItem *this, char)
// IDA 0x9794: stores the global and returns its address; no signal.
pub fn stub_9794(value: bool) -> &'static std::sync::atomic::AtomicBool {
    render_settings::CRenderSettingsItem::set_debug_disable_interpolation(value)
}

#[doc(alias = "CRenderSettingsItem::setResolutionPreference(RBX::CRenderSettings::ResolutionPreset)")]
// 0x97a4 — __ZN19CRenderSettingsItem23setResolutionPreferenceEN3RBX15CRenderSettings16ResolutionPresetE
// type: int __fastcall(int result, int)
// IDA 0x97a4: store +120 when changed, emit `prop_resolution`. Returns `this`.
pub fn stub_97a4(item: &mut render_settings::CRenderSettingsItem, preset: render_settings::ResolutionPreset) -> &mut render_settings::CRenderSettingsItem {
    item.set_resolution_preference(preset)
}

#[doc(alias = "CRenderSettingsItem::setTextureCacheSize(unsigned int)")]
// 0x97c0 — __ZN19CRenderSettingsItem19setTextureCacheSizeEj
// type: int __fastcall(int this, unsigned int)
// IDA 0x97c0: unconditional store of +160, no signal. Returns `this`.
pub fn stub_97c0(item: &mut render_settings::CRenderSettingsItem, size: u32) -> &mut render_settings::CRenderSettingsItem {
    item.set_texture_cache_size(size)
}

#[doc(alias = "CRenderSettingsItem::setMeshCacheSize(unsigned int)")]
// 0x97c8 — __ZN19CRenderSettingsItem16setMeshCacheSizeEj
// type: int __fastcall(int this, unsigned int)
// IDA 0x97c8: unconditional store of +164, no signal. Returns `this`.
pub fn stub_97c8(item: &mut render_settings::CRenderSettingsItem, size: u32) -> &mut render_settings::CRenderSettingsItem {
    item.set_mesh_cache_size(size)
}

#[doc(alias = "CRenderSettingsItem::CRenderSettingsItem(void)")]
// 0x97d0 — __ZN19CRenderSettingsItemC2Ev
// type: void __fastcall(CRenderSettingsItem *this)
// IDA 0x97d0: "Rendering", 800x600, seeded resolution, +189 set, VRAM-budget branch.
// The platform `GetDXVideoMemorySize` query is injected as a parameter.
pub fn stub_97d0(dx_video_memory_size: u32) -> render_settings::CRenderSettingsItem {
    render_settings::CRenderSettingsItem::new(dx_video_memory_size)
}

#[doc(alias = "CRenderSettingsItem::setAutoQualityLevel(int)")]
// 0x9ac8 — __ZN19CRenderSettingsItem19setAutoQualityLevelEi
// type: int __fastcall(int this, int)
pub fn stub_9ac8() -> ! {
    todo!("0x9ac8 __ZN19CRenderSettingsItem19setAutoQualityLevelEi")
}

#[doc(alias = "non-virtual thunk toCRenderSettingsItem::setAutoQualityLevel(int)")]
// 0x9ae8 — __ZThn96_N19CRenderSettingsItem19setAutoQualityLevelEi
// type: int __fastcall(int this, int)
pub fn stub_9ae8() -> ! {
    todo!("0x9ae8 __ZThn96_N19CRenderSettingsItem19setAutoQualityLevelEi")
}

#[doc(alias = "CRenderSettingsItem::setEagerBulkExecution(bool)")]
// 0x9b08 — __ZN19CRenderSettingsItem21setEagerBulkExecutionEb
// type: int __fastcall(int this, int)
pub fn stub_9b08() -> ! {
    todo!("0x9b08 __ZN19CRenderSettingsItem21setEagerBulkExecutionEb")
}

#[doc(alias = "std::length_error::~length_error()")]
// 0x9b2c — __ZNSt12length_errorD1Ev
// type: void __cdecl(std::length_error *__hidden this)
pub fn stub_9b2c() -> ! {
    todo!("0x9b2c __ZNSt12length_errorD1Ev")
}

#[doc(alias = "std::out_of_range::~out_of_range()")]
// 0x9b30 — __ZNSt12out_of_rangeD0Ev
// type: void __cdecl(std::out_of_range *__hidden this)
pub fn stub_9b30() -> ! {
    todo!("0x9b30 __ZNSt12out_of_rangeD0Ev")
}

#[doc(alias = "std::out_of_range::~out_of_range()")]
// 0x9b44 — __ZNSt12out_of_rangeD2Ev
// type: void __cdecl(std::out_of_range *__hidden this)
pub fn stub_9b44() -> ! {
    todo!("0x9b44 __ZNSt12out_of_rangeD2Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::addPair(RBX::CRenderSettings::AASamples,char const*)")]
// 0x9b48 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
pub fn stub_9b48() -> ! {
    todo!("0x9b48 __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE7addPairES3_PKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::addPair(RBX::CRenderSettings::GraphicsMode,char const*)")]
// 0x9ea8 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
pub fn stub_9ea8() -> ! {
    todo!("0x9ea8 __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE7addPairES3_PKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::addLegacy(int,char const*,RBX::CRenderSettings::GraphicsMode)")]
// 0xa208 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE9addLegacyEiPKcS3_
// type: _DWORD *__fastcall(int, unsigned int, int, int)
pub fn stub_a208() -> ! {
    todo!("0xa208 __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE9addLegacyEiPKcS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::addPair(RBX::CRenderSettings::FrameRateManagerMode,char const*)")]
// 0xa25c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
pub fn stub_a25c() -> ! {
    todo!("0xa25c __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE7addPairES3_PKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::addPair(RBX::CRenderSettings::AntialiasingMode,char const*)")]
// 0xa5bc — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
pub fn stub_a5bc() -> ! {
    todo!("0xa5bc __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE7addPairES3_PKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::addPair(RBX::CRenderSettings::ShadowMode,char const*)")]
// 0xa91c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
pub fn stub_a91c() -> ! {
    todo!("0xa91c __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE7addPairES3_PKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::addPair(RBX::CRenderSettings::QualityLevel,char const*)")]
// 0xac7c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
pub fn stub_ac7c() -> ! {
    todo!("0xac7c __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE7addPairES3_PKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::addPair(RBX::CRenderSettings::ResolutionPreset,char const*)")]
// 0xafdc — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
pub fn stub_afdc() -> ! {
    todo!("0xafdc __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE7addPairES3_PKc")
}

#[doc(alias = "RBX::CRenderSettings::getGraphicsMode(void)const")]
// 0xb33c — __ZNK3RBX15CRenderSettings15getGraphicsModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
pub fn stub_b33c() -> ! {
    todo!("0xb33c __ZNK3RBX15CRenderSettings15getGraphicsModeEv")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::~EnumPropDescriptor()")]
// 0xb340 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_b340() -> ! {
    todo!("0xb340 __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEED1Ev")
}

#[doc(alias = "RBX::CRenderSettings::getFrameRateManagerMode(void)const")]
// 0xb364 — __ZNK3RBX15CRenderSettings23getFrameRateManagerModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
pub fn stub_b364() -> ! {
    todo!("0xb364 __ZNK3RBX15CRenderSettings23getFrameRateManagerModeEv")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::~EnumPropDescriptor()")]
// 0xb368 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_b368() -> ! {
    todo!("0xb368 __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEED1Ev")
}

#[doc(alias = "RBX::CRenderSettings::getQualityLevel(void)const")]
// 0xb38c — __ZNK3RBX15CRenderSettings15getQualityLevelEv
// type: int __fastcall(RBX::CRenderSettings *this)
pub fn stub_b38c() -> ! {
    todo!("0xb38c __ZNK3RBX15CRenderSettings15getQualityLevelEv")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::~EnumPropDescriptor()")]
// 0xb390 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_b390() -> ! {
    todo!("0xb390 __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEED1Ev")
}

#[doc(alias = "RBX::CRenderSettings::getAlwaysDrawConnectors(void)const")]
// 0xb3b4 — __ZNK3RBX15CRenderSettings23getAlwaysDrawConnectorsEv
// type: int __fastcall(RBX::CRenderSettings *this)
pub fn stub_b3b4() -> ! {
    todo!("0xb3b4 __ZNK3RBX15CRenderSettings23getAlwaysDrawConnectorsEv")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::~PropDescriptor()")]
// 0xb3bc — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_b3bc() -> ! {
    todo!("0xb3bc __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembED1Ev")
}

#[doc(alias = "RBX::CRenderSettings::getShowAggregation(void)const")]
// 0xb3e0 — __ZNK3RBX15CRenderSettings18getShowAggregationEv
// type: int __fastcall(RBX::CRenderSettings *this)
pub fn stub_b3e0() -> ! {
    todo!("0xb3e0 __ZNK3RBX15CRenderSettings18getShowAggregationEv")
}

#[doc(alias = "RBX::CRenderSettings::getAASamples(void)const")]
// 0xb3e8 — __ZNK3RBX15CRenderSettings12getAASamplesEv
// type: int __fastcall(RBX::CRenderSettings *this)
pub fn stub_b3e8() -> ! {
    todo!("0xb3e8 __ZNK3RBX15CRenderSettings12getAASamplesEv")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::~EnumPropDescriptor()")]
// 0xb3f8 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_b3f8() -> ! {
    todo!("0xb3f8 __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEED1Ev")
}

#[doc(alias = "RBX::CRenderSettings::getShadowMode(void)const")]
// 0xb41c — __ZNK3RBX15CRenderSettings13getShadowModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
pub fn stub_b41c() -> ! {
    todo!("0xb41c __ZNK3RBX15CRenderSettings13getShadowModeEv")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::~EnumPropDescriptor()")]
// 0xb420 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_b420() -> ! {
    todo!("0xb420 __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEED1Ev")
}

#[doc(alias = "RBX::CRenderSettings::getAntialiasingMode(void)const")]
// 0xb444 — __ZNK3RBX15CRenderSettings19getAntialiasingModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
pub fn stub_b444() -> ! {
    todo!("0xb444 __ZNK3RBX15CRenderSettings19getAntialiasingModeEv")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::~EnumPropDescriptor()")]
// 0xb448 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_b448() -> ! {
    todo!("0xb448 __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEED1Ev")
}

#[doc(alias = "RBX::CRenderSettings::getDebugShowBoundingBoxes(void)const")]
// 0xb46c — __ZNK3RBX15CRenderSettings25getDebugShowBoundingBoxesEv
// type: int __fastcall(RBX::CRenderSettings *this)
pub fn stub_b46c() -> ! {
    todo!("0xb46c __ZNK3RBX15CRenderSettings25getDebugShowBoundingBoxesEv")
}

#[doc(alias = "RBX::CRenderSettings::getAutoQualityLevel(void)const")]
// 0xb474 — __ZNK3RBX15CRenderSettings19getAutoQualityLevelEv
// type: int __fastcall(RBX::CRenderSettings *this)
pub fn stub_b474() -> ! {
    todo!("0xb474 __ZNK3RBX15CRenderSettings19getAutoQualityLevelEv")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::~PropDescriptor()")]
// 0xb478 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_b478() -> ! {
    todo!("0xb478 __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiED1Ev")
}

#[doc(alias = "RBX::CRenderSettings::getEnableFRM(void)const")]
// 0xb49c — __ZNK3RBX15CRenderSettings12getEnableFRMEv
// type: int __fastcall(RBX::CRenderSettings *this)
pub fn stub_b49c() -> ! {
    todo!("0xb49c __ZNK3RBX15CRenderSettings12getEnableFRMEv")
}

#[doc(alias = "RBX::CRenderSettings::getResolutionPreference(void)const")]
// 0xb4a4 — __ZNK3RBX15CRenderSettings23getResolutionPreferenceEv
// type: int __fastcall(RBX::CRenderSettings *this)
pub fn stub_b4a4() -> ! {
    todo!("0xb4a4 __ZNK3RBX15CRenderSettings23getResolutionPreferenceEv")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::~EnumPropDescriptor()")]
// 0xb4a8 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_b4a8() -> ! {
    todo!("0xb4a8 __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEED1Ev")
}

#[doc(alias = "RBX::CRenderSettings::getMaxQualityLevel(void)")]
// 0xb4cc — __ZN3RBX15CRenderSettings18getMaxQualityLevelEv
// type: int __fastcall(RBX::CRenderSettings *this)
pub fn stub_b4cc() -> ! {
    todo!("0xb4cc __ZN3RBX15CRenderSettings18getMaxQualityLevelEv")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::~BoundFuncDesc()")]
// 0xb4d0 — __ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_b4d0() -> ! {
    todo!("0xb4d0 __ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EED1Ev")
}

#[doc(alias = "RBX::CRenderSettings::getTextureCacheSize(void)const")]
// 0xb4f4 — __ZNK3RBX15CRenderSettings19getTextureCacheSizeEv
// type: int __fastcall(RBX::CRenderSettings *this)
pub fn stub_b4f4() -> ! {
    todo!("0xb4f4 __ZNK3RBX15CRenderSettings19getTextureCacheSizeEv")
}

#[doc(alias = "RBX::CRenderSettings::getMeshCacheSize(void)const")]
// 0xb4f8 — __ZNK3RBX15CRenderSettings16getMeshCacheSizeEv
// type: int __fastcall(RBX::CRenderSettings *this)
pub fn stub_b4f8() -> ! {
    todo!("0xb4f8 __ZNK3RBX15CRenderSettings16getMeshCacheSizeEv")
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEEC2Ev")]
// 0xb4fc — __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEEC2Ev
// type: RBX::Instance *__fastcall(RBX::Instance *)
pub fn stub_b4fc() -> ! {
    todo!("0xb4fc __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEEC2Ev")
}

#[doc(alias = "std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::push_back(G3D::Vector2int16 const&)")]
// 0xb740 — __ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE9push_backERKS1_
// type: int __fastcall(int result, _DWORD *)
pub fn stub_b740() -> ! {
    todo!("0xb740 __ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE9push_backERKS1_")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Reflection::PropertyDescriptor const*)>::operator()(RBX::Reflection::PropertyDescriptor const*)")]
// 0xb76c — __ZN3rbx7signals16signal_with_argsILi1EFvPKN3RBX10Reflection18PropertyDescriptorEEEclES6_
// type: void __fastcall(_DWORD *, int, int, const void *, int, int, int, int, void *, int)
pub fn stub_b76c() -> ! {
    todo!("0xb76c __ZN3rbx7signals16signal_with_argsILi1EFvPKN3RBX10Reflection18PropertyDescriptorEEEclES6_")
}

#[doc(alias = "RBX::CRenderSettings::getEagerBulkExecution(void)const")]
// 0xb8b0 — __ZNK3RBX15CRenderSettings21getEagerBulkExecutionEv
// type: int __fastcall(RBX::CRenderSettings *this)
pub fn stub_b8b0() -> ! {
    todo!("0xb8b0 __ZNK3RBX15CRenderSettings21getEagerBulkExecutionEv")
}

#[doc(alias = "CRenderSettingsItem::~CRenderSettingsItem()")]
// 0xb8b8 — __ZN19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
pub fn stub_b8b8() -> ! {
    todo!("0xb8b8 __ZN19CRenderSettingsItemD1Ev")
}

#[doc(alias = "CRenderSettingsItem::~CRenderSettingsItem()")]
// 0xb8bc — __ZN19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
pub fn stub_b8bc() -> ! {
    todo!("0xb8bc __ZN19CRenderSettingsItemD0Ev")
}

#[doc(alias = "__ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv")]
// 0xb8d0 — __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv
// type: int()
pub fn stub_b8d0() -> ! {
    todo!("0xb8d0 __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv")
}

#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
// 0xb8e0 — __ZThn32_N19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
pub fn stub_b8e0() -> ! {
    todo!("0xb8e0 __ZThn32_N19CRenderSettingsItemD1Ev")
}

#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
// 0xb8e8 — __ZThn32_N19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
pub fn stub_b8e8() -> ! {
    todo!("0xb8e8 __ZThn32_N19CRenderSettingsItemD0Ev")
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv")]
// 0xb900 — __ZThn32_NK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv
// type: int()
pub fn stub_b900() -> ! {
    todo!("0xb900 __ZThn32_NK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv")
}

#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
// 0xb910 — __ZThn36_N19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
pub fn stub_b910() -> ! {
    todo!("0xb910 __ZThn36_N19CRenderSettingsItemD1Ev")
}

#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
// 0xb918 — __ZThn36_N19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
pub fn stub_b918() -> ! {
    todo!("0xb918 __ZThn36_N19CRenderSettingsItemD0Ev")
}

#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD1Ev")]
// 0xb930 — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD1Ev
// type: int()
pub fn stub_b930() -> ! {
    todo!("0xb930 __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD1Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()")]
// 0xb934 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED1Ev
// type: int()
pub fn stub_b934() -> ! {
    todo!("0xb934 __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED1Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()")]
// 0xb938 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED0Ev
// type: int __fastcall(int)
pub fn stub_b938() -> ! {
    todo!("0xb938 __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED0Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::lookup(char const*)const")]
// 0xb94c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
pub fn stub_b94c() -> ! {
    todo!("0xb94c __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupEPKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::lookup(RBX::Reflection::Variant const&)const")]
// 0xb97c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
pub fn stub_b97c() -> ! {
    todo!("0xb97c __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupERKNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0xb99c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueEmRNS0_7VariantE
pub fn stub_b99c() -> ! {
    todo!("0xb99c __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueEmRNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToString(unsigned long,std::string &)const")]
// 0xb9f8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
pub fn stub_b9f8() -> ! {
    todo!("0xb9f8 __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE15convertToStringEmRSs")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()")]
// 0xbb3c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED1Ev
// type: int()
pub fn stub_bb3c() -> ! {
    todo!("0xbb3c __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED1Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()")]
// 0xbb40 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED0Ev
// type: int __fastcall(int)
pub fn stub_bb40() -> ! {
    todo!("0xbb40 __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED0Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::lookup(char const*)const")]
// 0xbb54 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
pub fn stub_bb54() -> ! {
    todo!("0xbb54 __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE6lookupEPKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::lookup(RBX::Reflection::Variant const&)const")]
// 0xbb84 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
pub fn stub_bb84() -> ! {
    todo!("0xbb84 __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE6lookupERKNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0xbba4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueEmRNS0_7VariantE
pub fn stub_bba4() -> ! {
    todo!("0xbba4 __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueEmRNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToString(unsigned long,std::string &)const")]
// 0xbc00 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
pub fn stub_bc00() -> ! {
    todo!("0xbc00 __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringEmRSs")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::~EnumDesc()")]
// 0xbd44 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED1Ev
// type: int()
pub fn stub_bd44() -> ! {
    todo!("0xbd44 __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED1Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::~EnumDesc()")]
// 0xbd48 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED0Ev
// type: int __fastcall(int)
pub fn stub_bd48() -> ! {
    todo!("0xbd48 __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED0Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::lookup(char const*)const")]
// 0xbd5c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
pub fn stub_bd5c() -> ! {
    todo!("0xbd5c __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupEPKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::lookup(RBX::Reflection::Variant const&)const")]
// 0xbd8c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
pub fn stub_bd8c() -> ! {
    todo!("0xbd8c __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupERKNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0xbdac — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueEmRNS0_7VariantE
pub fn stub_bdac() -> ! {
    todo!("0xbdac __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueEmRNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToString(unsigned long,std::string &)const")]
// 0xbe08 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
pub fn stub_be08() -> ! {
    todo!("0xbe08 __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE15convertToStringEmRSs")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc()")]
// 0xbf4c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED1Ev
// type: int()
pub fn stub_bf4c() -> ! {
    todo!("0xbf4c __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED1Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc()")]
// 0xbf50 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED0Ev
// type: int __fastcall(int)
pub fn stub_bf50() -> ! {
    todo!("0xbf50 __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED0Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::lookup(char const*)const")]
// 0xbf64 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
pub fn stub_bf64() -> ! {
    todo!("0xbf64 __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE6lookupEPKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::lookup(RBX::Reflection::Variant const&)const")]
// 0xbf94 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
pub fn stub_bf94() -> ! {
    todo!("0xbf94 __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE6lookupERKNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0xbfb4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToValueEmRNS0_7VariantE
pub fn stub_bfb4() -> ! {
    todo!("0xbfb4 __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToValueEmRNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToString(unsigned long,std::string &)const")]
// 0xc010 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
pub fn stub_c010() -> ! {
    todo!("0xc010 __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE15convertToStringEmRSs")
}
