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
    use std::collections::HashMap;
    use std::os::raw::{c_char, c_int};
    use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

    /// Batch 2: IDA 0x9b48 `ReleaseAssert((int)value<=2304)`, enumconverter.h:211.
    /// (The `value>=0` twin at line 210 is asserted at the same site.)
    pub const MAX_ENUM_VALUE: i32 = 2304;

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
    /// was: `RBX::Reflection::EnumDescriptor::Item` heap box (0x1C bytes, IDA 0x9b7e):
    /// `Descriptor(name)` + vtable `off_1270CA8` + owner/value/index words.
    #[derive(Debug, Clone)]
    pub struct EnumItemEntry {
        pub name: String,
        pub value: i32,
        /// was: `a1[10]` snapshot at insert (IDA 0x9be0 `*((_DWORD *)v38 + 6) = v30`).
        pub index: u32,
    }

    #[derive(Debug, Clone, Default)]
    pub struct EnumDescData {
        pub desc_name: &'static str,
        /// was: the installed vtable (`*a1 = &off_...` at each ctor tail).
        pub vtable: &'static str,
        pub pairs: Vec<(i32, String)>,
        /// was: `Item*` vector at +7 (IDA 0x9bee `push_back(a1 + 7, ...)`).
        pub items: Vec<EnumItemEntry>,
        /// was: `vector<T>` at +33, `-1`-filled (IDA 0x9c14 resize, then `[a2] = a2`).
        pub value_vec: Vec<i32>,
        /// was: `vector<unsigned long>` at +39 holding the insert counter per value.
        pub index_vec: Vec<u32>,
        /// was: `vector<T>` at +36, values in declaration order (IDA 0x9d08).
        pub ordered: Vec<i32>,
        /// was: `vector<const Name*>` at +24, NullName-filled (IDA 0x9d2e).
        /// `[INFERENCE]` — names stored owned here; null slots are `None`.
        pub name_slots: Vec<Option<String>>,
        /// was: `vector<string>` at +27, empty-filled then assigned (IDA 0x9d98).
        pub name_strings: Vec<String>,
        /// was: `vector<const Item*>` at +30, null-filled (IDA 0x9db8).
        pub item_index: Vec<Option<usize>>,
        /// was: `map<const Name*, T>` at +12 (IDA 0x9dd4 `operator[]`).
        pub by_name: HashMap<String, i32>,
        /// was: `a1[10]` insert counter (IDA 0x9dde).
        pub counter: u32,
        /// was: `a1[11] = floor(log2(counter))` (IDA 0x9de4 shift loop).
        pub log_bits: i32,
        /// was: `vector<T>` at +132 for legacy remap, `-1`-filled (IDA 0xa234).
        pub legacy_remap: Vec<i32>,
        /// was: `addLegacy(value, name, flag)` entries.
        pub legacy: Vec<(i32, String, i32)>,
        /// was: extra `Name::declare` + `map::operator[]` alias inserts (the " (wide)"
        /// and "Level NN" spellings).
        pub aliases: Vec<(String, i32)>,
    }

    impl EnumDescData {
        /// Batch 2 full port of `EnumDesc<T>::addPair(value, name)` (IDA 0x9b48 body;
        /// the 0x9ea8/0xa25c/0xa5bc/0xa91c/0xac7c/0xafdc instantiations are identical
        /// apart from the type word — verified by diff): Item box alloc (0x9b7e),
        /// owner/value/index words (0x9bd4-0x9be0), items push (0x9bee), `-1`-fill
        /// resize of the value vector (0x9c14) with `[value] = value` (0x9c22), the
        /// `value>=0` / `(int)value<=2304` ReleaseAsserts (enumconverter.h:210-211,
        /// panics here), counter snapshot into the index vector (0x9cfc), ordered
        /// push (0x9d08), NullName-fill resize of the name-pointer vector (0x9d2e),
        /// empty-fill resize + assign of the name-string vector (0x9d68/0x9d98),
        /// null-fill resize of the item-pointer vector (0x9db8), the name->value map
        /// insert (0x9dd4), and the counter/log-bits update (0x9dde-0x9df6).
        /// The declaration-order `pairs` list is kept alongside for stable iteration.
        pub fn add_pair(&mut self, value: i32, name: &str) {
            assert!(value >= 0, "value>=0 file: ../App/include/reflection/enumconverter.h line: 210");
            assert!(
                value <= MAX_ENUM_VALUE,
                "(int)value<=2304 file: ../App/include/reflection/enumconverter.h line: 211"
            );
            let index = self.counter;
            self.items.push(EnumItemEntry { name: name.to_string(), value, index });
            if self.value_vec.len() <= value as usize {
                self.value_vec.resize(value as usize + 1, -1);
            }
            self.value_vec[value as usize] = value;
            if self.index_vec.len() <= value as usize {
                self.index_vec.resize(value as usize + 1, u32::MAX);
            }
            self.index_vec[value as usize] = index;
            self.ordered.push(value);
            if self.name_slots.len() <= value as usize {
                self.name_slots.resize(value as usize + 1, None);
            }
            self.name_slots[value as usize] = Some(name.to_string());
            if self.name_strings.len() <= value as usize {
                self.name_strings.resize(value as usize + 1, String::new());
            }
            self.name_strings[value as usize] = name.to_string();
            if self.item_index.len() <= value as usize {
                self.item_index.resize(value as usize + 1, None);
            }
            self.item_index[value as usize] = Some(self.items.len() - 1);
            self.by_name.insert(name.to_string(), value);
            self.pairs.push((value, name.to_string()));
            self.counter = index + 1;
            self.log_bits = 31 - self.counter.leading_zeros() as i32;
        }
        /// Batch 2 full port of `EnumDesc<GraphicsMode>::addLegacy` (IDA 0xa208):
        /// `-1`-fill resize of the +132 remap vector when `len <= index` (0xa22a),
        /// `remap[index] = value` (0xa23a), then `Name::declare` + `map[+72] = value`
        /// (0xa244/0xa24c). The `(index, name, value)` triple is kept in `legacy`
        /// alongside; the ctor-time alias also lands in `aliases` via `add_alias`.
        pub fn add_legacy(&mut self, index: i32, name: &str, value: i32) {
            assert!(index >= 0, "value>=0 file: ../App/include/reflection/enumconverter.h line: 210");
            if self.legacy_remap.len() <= index as usize {
                self.legacy_remap.resize(index as usize + 1, -1);
            }
            self.legacy_remap[index as usize] = value;
            self.legacy.push((index, name.to_string(), value));
            self.add_alias(name, value);
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

    /// Batch 3: was: `FLog::SignalPrints` — gates the `"Signal with 1 arg executed"`
    /// trace in the 1-arg emit walk (IDA 0xb7ce). Off in this port by default.
    pub static SIGNAL_PRINTS: AtomicBool = AtomicBool::new(false);

    /// was: a connected `signal::slot` — the `*(v22 + 12)` connected word (IDA 0xb7e6)
    /// decides whether the functor at `*(v22 + 4)` runs on each emission.
    pub struct SignalSlot {
        pub id: u64,
        pub connected: bool,
        pub callback: Box<dyn Fn(&'static str) + Send + Sync>,
    }

    /// was: `rbx::signals::signal_with_args<1, void(PropertyDescriptor const*)>` —
    /// the per-item `propertyChanged` signal at +192. `emit` records the property id
    /// (observable for tests) then notifies still-connected slots in connect order,
    /// mirroring the `next()` walk (IDA 0xb80a) + `*(v22 + 12)` guard (0xb7e6).
    /// (`[INFERENCE]` — listener storage shape; emission order, change-only gating,
    /// and the connected guard are per the IDA call sites.)
    #[derive(Default)]
    pub struct PropertyChangedSignal {
        pub emitted: Vec<&'static str>,
        slots: Vec<SignalSlot>,
        next_id: u64,
    }

    impl PropertyChangedSignal {
        pub fn emit(&mut self, prop: &'static str) {
            self.emitted.push(prop);
            for slot in &self.slots {
                if slot.connected {
                    (slot.callback)(prop);
                }
            }
        }
        /// Batch 3: traced emit — IDA 0xb7ce/0xb7e0 fast-log when `SIGNAL_PRINTS`
        /// is set, then the same walk. The `next()` iterator ref and its terminal
        /// `intrusive_ptr_release` (0xb80c) are `Arc` drops here.
        pub fn emit_traced(&mut self, prop: &'static str) {
            if SIGNAL_PRINTS.load(Ordering::SeqCst) {
                eprintln!("Signal with 1 arg executed");
            }
            self.emit(prop);
        }
        pub fn connect(&mut self, f: impl Fn(&'static str) + Send + Sync + 'static) -> u64 {
            let id = self.next_id;
            self.next_id += 1;
            self.slots.push(SignalSlot { id, connected: true, callback: Box::new(f) });
            id
        }
        pub fn disconnect(&mut self, id: u64) {
            if let Some(slot) = self.slots.iter_mut().find(|s| s.id == id) {
                slot.connected = false;
            }
        }
    }

    /// Batch 2: was: `unk_130C2AC` — shared by quality-level (0x9648) and
    /// auto-quality-level (0x9ac8) setters alike.
    pub const PROP_AUTO_QUALITY_LEVEL: &str = "unk_130C2AC";
    /// Batch 2: was: `unk_130C1E8` for eager bulk execution (IDA 0x9b26).
    pub const PROP_EAGER_BULK_EXECUTION: &str = "unk_130C1E8";
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
        /// Batch 3: +128, IDA 0xb4cc max quality level (`[INFERENCE]` — no writer
        /// observed in this batch; default 0 until the FRM-side port lands).
        pub max_quality_level: i32,
        /// Batch 2: +124, IDA 0x9ac8 auto quality level.
        pub auto_quality_level: i32,
        /// Batch 2: +157, IDA 0x9b08 eager bulk execution flag.
        pub eager_bulk_execution: bool,
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

        /// Batch 2, IDA 0x9ac8: store +124 when changed, emit `PROP_AUTO_QUALITY_LEVEL`
        /// (same property id as the quality-level setter). Returns `this`.
        pub fn set_auto_quality_level(&mut self, level: i32) -> &mut Self {
            if self.auto_quality_level != level {
                self.auto_quality_level = level;
                self.changed.emit(PROP_AUTO_QUALITY_LEVEL);
            }
            self
        }

        /// Batch 2, IDA 0x9ae8 non-virtual thunk to `setAutoQualityLevel`: the incoming
        /// `this` is the +96 `CRenderSettings` subobject, so the original biases every
        /// access by -96 (`v2 = this - 96`, 0x9af4) — a no-op in this flat layout, where
        /// the thunk delegates straight to the method.
        pub fn set_auto_quality_level_thunk(&mut self, level: i32) -> &mut Self {
            self.set_auto_quality_level(level)
        }

        /// Batch 2, IDA 0x9b08: store byte +157 when changed, emit
        /// `PROP_EAGER_BULK_EXECUTION`. Returns `this`.
        pub fn set_eager_bulk_execution(&mut self, value: bool) -> &mut Self {
            if value != self.eager_bulk_execution {
                self.eager_bulk_execution = value;
                self.changed.emit(PROP_EAGER_BULK_EXECUTION);
            }
            self
        }

        /// Batch 2 getters. Note the original `this` is the +96 `CRenderSettings`
        /// subobject, so its getter offsets are biased by -96 against the setters:
        /// +4 -> item+100 (0xb33e), +16 -> item+112 (0xb366), +20 -> item+116
        /// (0xb38e), +59 -> item+155 (0xb3b8), +58 -> item+154 (0xb3e4).
        /// Batch 2, IDA 0xb33c `CRenderSettings::getGraphicsMode`.
        pub fn graphics_mode(&self) -> GraphicsMode {
            self.graphics_mode
        }
        /// Batch 2, IDA 0xb364 `CRenderSettings::getFrameRateManagerMode`.
        pub fn frame_rate_manager_mode(&self) -> FrameRateManagerMode {
            self.frame_rate_manager_mode
        }
        /// Batch 2, IDA 0xb38c `CRenderSettings::getQualityLevel`.
        pub fn quality_level(&self) -> QualityLevel {
            self.quality_level
        }
        /// Batch 2, IDA 0xb3b4 `CRenderSettings::getAlwaysDrawConnectors`.
        pub fn always_draw_connectors(&self) -> bool {
            self.always_draw_connectors
        }
        /// Batch 2, IDA 0xb3e0 `CRenderSettings::getShowAggregation`.
        pub fn show_aggregation(&self) -> bool {
            self.show_aggregation
        }
        /// Batch 2, IDA 0xb3e8 `CRenderSettings::getAASamples` — reads the global.
        pub fn get_aa_samples() -> i32 {
            AA_SAMPLES.load(Ordering::SeqCst)
        }
        /// Batch 3, IDA 0xb41c `getShadowMode` (subobject +12 -> item+108).
        pub fn shadow_mode(&self) -> ShadowMode {
            self.shadow_mode
        }
        /// Batch 3, IDA 0xb444 `getAntialiasingMode` (subobject +8 -> item+104).
        pub fn antialiasing_mode(&self) -> AntialiasingMode {
            self.antialiasing_mode
        }
        /// Batch 3, IDA 0xb46c `getDebugShowBoundingBoxes` (subobject +40 -> item+136).
        pub fn debug_show_bounding_boxes(&self) -> bool {
            self.debug_show_bounding_boxes
        }
        /// Batch 3, IDA 0xb474 `getAutoQualityLevel` (subobject +28 -> item+124).
        pub fn auto_quality_level(&self) -> i32 {
            self.auto_quality_level
        }
        /// Batch 3, IDA 0xb49c `getEnableFRM` (subobject +41 -> item+137).
        pub fn enable_frm(&self) -> bool {
            self.enable_frm
        }
        /// Batch 3, IDA 0xb4a4 `getResolutionPreference` (subobject +24 -> item+120).
        pub fn resolution_preference(&self) -> ResolutionPreset {
            self.resolution_preference
        }
        /// Batch 3, IDA 0xb4cc `getMaxQualityLevel` (subobject +32 -> item+128).
        pub fn max_quality_level(&self) -> i32 {
            self.max_quality_level
        }
        /// Batch 3, IDA 0xb4f4 `getTextureCacheSize` (subobject +64 -> item+160).
        pub fn texture_cache_size(&self) -> u32 {
            self.texture_cache_size
        }
        /// Batch 3, IDA 0xb4f8 `getMeshCacheSize` (subobject +68 -> item+164).
        pub fn mesh_cache_size(&self) -> u32 {
            self.mesh_cache_size
        }
        /// Batch 3, IDA 0xb8b0 `getEagerBulkExecution` (subobject +61 -> item+157).
        pub fn eager_bulk_execution(&self) -> bool {
            self.eager_bulk_execution
        }

        /// Batch 3: models `CRenderSettingsItem::~CRenderSettingsItem` D2 member
        /// teardown (reached via the 0xb8b8/0xb8e0 thunks; the full D2 body lives
        /// outside this batch). Heap members are released in C++ order — name,
        /// resolution list, change signal — with the vtable restores and base-class
        /// dtors left to Rust `Drop`. Returns nothing, like the original.
        pub fn destroy_d2(&mut self) {
            self.name = String::new();
            self.resolutions.clear();
            self.changed.slots.clear();
        }
        /// Batch 3, IDA 0xb8b8 D1 — thunk to D2.
        pub fn destroy_d1(&mut self) {
            self.destroy_d2();
        }
    }

    /// Batch 3: was: `std::vector<G3D::Vector2int16>::push_back` (IDA 0xb740) —
    /// fast path writes `*a2` at `finish` and bumps it when `finish != end`
    /// (0xb74c-0xb75c); full storage falls into `_M_insert_aux` (0xb766), which
    /// grows. `Vec::push` is exactly that split; the `finish == 0` residue of the
    /// original (`v4 = 0` when begin is null) cannot occur for a valid vector and
    /// is noted, not modeled.
    pub fn vector2int16_push_back(list: &mut Vec<(u16, u16)>, x: u16, y: u16) {
        list.push((x, y));
    }

    /// Batch 3: was: `GlobalAdvancedSettingsItem<CRenderSettingsItem>` base state —
    /// `Instance::Instance(a1, 0)` + vtable installs (`off_1221C68/...` then
    /// `off_1221B98/...` after the classDescriptor call), `registrar++` (0xb5b2),
    /// byte +92 set (0xb5ba), and `setName("RenderSettings")` (0xb5ec/0xb5f8).
    /// The singleton throw (0xb688-0xb6b4) becomes a `Result` so the port stays total.
    #[derive(Debug, Default)]
    pub struct GlobalAdvancedSettingsBase {
        /// was: `+92 = 1` (IDA 0xb5ba).
        pub initialized: bool,
        /// was: the `setName("RenderSettings")` name (IDA 0xb5f8).
        pub name: String,
    }

    /// was: `GlobalAdvancedSettingsItem<...>::singE` guard (IDA 0xb622/0xb626).
    pub static SINGLETON_CLAIMED: AtomicBool = AtomicBool::new(false);

    /// Batch 3, IDA 0xb4fc base ctor tail — claims the singleton or reports the
    /// `singleton %s already exists` runtime_error (0xb692) as `Err`.
    pub fn claim_render_settings_singleton() -> Result<GlobalAdvancedSettingsBase, String> {
        if SINGLETON_CLAIMED.swap(true, Ordering::SeqCst) {
            return Err(format!("singleton {} already exists", "RenderSettings"));
        }
        Ok(GlobalAdvancedSettingsBase { initialized: true, name: "RenderSettings".to_string() })
    }

    /// Batch 3, IDA 0xb8d0 `FactoryProduct<...>::getClassName` — hops through
    /// `static_getCreator` to `Creator::getClassName` (0xb8d4/shim). The hop is
    /// collapsed; the literal is `[INFERENCE]` from the item type.
    pub fn render_settings_item_class_name() -> &'static str {
        "CRenderSettingsItem"
    }

    /// Batch 4: was: `RBX::Reflection::Variant` — the placement-any (`Region3`-sized)
    /// that `convertToValue` fills via `Singleton::initSingleton` + `placement_any::=`
    /// (IDA 0xb9c8-0xb9ec). Collapsed to the payload this batch observes; the
    /// singleton/type-tag hop is a no-op here since the payload carries its type.
    /// Unknown payloads stay `Empty` (`[INFERENCE]` — only `I32` is observed).
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub enum Variant {
        #[default]
        Empty,
        I32(i32),
    }

    impl EnumDescData {
        /// Batch 4 shared port of `EnumDesc<T>::convertToValue(index, out)` (IDA 0xb99c
        /// disasm; the 0xbba4/0xbdac/0xbfb4 instantiations are the same template):
        /// `if (*(a1 + 40) > index) { out = *(*(a1 + 144) + 4 * index); return 1; }
        /// return 0;` (0xb9a4-0xb9b6) then the singleton/placement-assign of the
        /// variant (0xb9c8-0xb9ec, collapsed into the `Variant` write).
        /// `[INFERENCE]` — which table word +40 sizes and whether +144 aliases one
        /// of the addPair tables; behavior matches on every IDA-observed input:
        /// in-range slots resolve (holes propagate `-1`, exactly like the unchecked
        /// `STRHI`), out-of-range fails.
        pub fn convert_to_value(&self, index: usize, out: &mut Variant) -> bool {
            if index < self.value_vec.len() {
                *out = Variant::I32(self.value_vec[index]);
                true
            } else {
                false
            }
        }
        /// Batch 4 shared port of `EnumDesc<T>::convertToString(value, out)`
        /// (IDA 0xb9f8/0xbc00/0xbe08/0xc010): resolve the per-value name slot and
        /// assign it out, returning 1 — else return 0 with `out` untouched. The inner
        /// base-class stringify call (0xba66) is the identity on names here.
        pub fn convert_to_string(&self, value: usize, out: &mut String) -> bool {
            if value < self.value_vec.len() && self.value_vec[value] >= 0 {
                if let Some(Some(name)) = self.name_slots.get(value) {
                    *out = name.clone();
                    return true;
                }
            }
            false
        }
        /// Batch 4: was: `EnumDesc<T>::convertToItem(value)` — the item-pointer
        /// table read both `lookup` overloads funnel into (IDA 0xb972/0xb998).
        /// Null (miss) becomes `None`.
        pub fn convert_to_item(&self, value: usize) -> Option<usize> {
            self.item_index.get(value).copied().flatten()
        }
        /// Batch 4 shared port of `EnumDesc<T>::lookup(name)` (IDA 0xb94c/0xbb54/
        /// 0xbd5c/0xbf64): `Name::lookup` (the `by_name` map here), then
        /// `convertToValue`, then `convertToItem` — null/`0` on any miss (0xb968).
        pub fn lookup_by_name(&self, name: &str) -> Option<usize> {
            let value = *self.by_name.get(name)?;
            if value < 0 {
                return None;
            }
            self.convert_to_item(value as usize)
        }
        /// Batch 4 shared port of `EnumDesc<T>::lookup(variant)` (IDA 0xb97c/0xbb84/
        /// 0xbd8c/0xbf94): `any_cast<const T&>` reads the payload (the `*(a2 + 4)`
        /// word, 0xb98e), then `convertToItem`. Non-`I32` payloads miss.
        pub fn lookup_by_value(&self, variant: &Variant) -> Option<usize> {
            match *variant {
                Variant::I32(v) if v >= 0 => self.convert_to_item(v as usize),
                _ => None,
            }
        }
        /// Batch 4: models `EnumDesc<T>::~EnumDesc() D2` member teardown (reached via
        /// the per-enum D1 thunks; full D2 bodies live outside this batch): every
        /// table the ctor/`addPair`/`addLegacy` fills is released and the counter
        /// reset, with base-class dtors left to Rust `Drop`.
        pub fn destroy_d2(&mut self) {
            self.pairs.clear();
            self.items.clear();
            self.value_vec.clear();
            self.index_vec.clear();
            self.ordered.clear();
            self.name_slots.clear();
            self.name_strings.clear();
            self.item_index.clear();
            self.by_name.clear();
            self.legacy.clear();
            self.aliases.clear();
            self.legacy_remap.clear();
            self.counter = 0;
            self.log_bits = 0;
        }
        /// Batch 4: D1 delegates to D2 (per the 0xb934-style thunks).
        pub fn destroy_d1(&mut self) {
            self.destroy_d2();
        }
    }

    /// Batch 4, IDA 0xb930: `FactoryProduct<...>::Creator D1` — thunk to D2
    /// (whose body is outside this batch); teardown collapses to a no-op with
    /// members left to `Drop`.
    pub fn render_settings_creator_d2() {}
    /// Batch 4, IDA 0xb930 thunk target wrapper.
    pub fn render_settings_creator_d1() {
        render_settings_creator_d2();
    }

    /// Batch 2: was: `RBX::Reflection::{Enum,}PropDescriptor` dtor core —
    /// `*a1 = <base vtable>; if (slot) operator delete(slot); return a1;`
    /// (IDA 0xb354/0xb37c/0xb3a4/0xb3d0). The owned slot (a1[11]/+44 for the enum
    /// descriptors, a1[10]/+40 for the bool one) is an `Option` here; destruction
    /// restores the base vtable and drops it. Rust `Drop` would run implicitly —
    /// this models the explicit base-restore the binary performs.
    #[derive(Debug, Default)]
    pub struct PropDescriptorBox {
        pub vtable: &'static str,
        pub owned: Option<Box<[u8]>>,
    }

    impl PropDescriptorBox {
        pub fn destroy(&mut self, base_vtable: &'static str) {
            self.vtable = base_vtable;
            self.owned = None;
        }
    }

    /// Batch 2, IDA 0xb340 `~EnumPropDescriptor<CRenderSettingsItem, GraphicsMode>`:
    /// restore `off_12228E8`, free +44.
    pub fn enum_prop_descriptor_graphics_mode_dtor(b: &mut PropDescriptorBox) {
        b.destroy("off_12228E8");
    }
    /// Batch 2, IDA 0xb368 `~EnumPropDescriptor<CRenderSettingsItem, FrameRateManagerMode>`:
    /// restore `off_1222848`, free +44.
    pub fn enum_prop_descriptor_frame_rate_manager_mode_dtor(b: &mut PropDescriptorBox) {
        b.destroy("off_1222848");
    }
    /// Batch 2, IDA 0xb390 `~EnumPropDescriptor<CRenderSettingsItem, QualityLevel>`:
    /// restore `off_12227A8`, free +44.
    pub fn enum_prop_descriptor_quality_level_dtor(b: &mut PropDescriptorBox) {
        b.destroy("off_12227A8");
    }
    /// Batch 3, IDA 0xb3f8 `~EnumPropDescriptor<Item, AASamples>` — off_1222658, +44.
    pub fn enum_prop_descriptor_aa_samples_dtor(b: &mut PropDescriptorBox) {
        b.destroy("off_1222658");
    }
    /// Batch 3, IDA 0xb420 `~EnumPropDescriptor<Item, ShadowMode>` — off_12224C8, +44.
    pub fn enum_prop_descriptor_shadow_mode_dtor(b: &mut PropDescriptorBox) {
        b.destroy("off_12224C8");
    }
    /// Batch 3, IDA 0xb448 `~EnumPropDescriptor<Item, AntialiasingMode>` — off_1222428, +44.
    pub fn enum_prop_descriptor_antialiasing_mode_dtor(b: &mut PropDescriptorBox) {
        b.destroy("off_1222428");
    }
    /// Batch 3, IDA 0xb478 `~PropDescriptor<Item, int>` — restore `off_1222178`, free +40.
    pub fn prop_descriptor_int_dtor(b: &mut PropDescriptorBox) {
        b.destroy("off_1222178");
    }
    /// Batch 3, IDA 0xb4a8 `~EnumPropDescriptor<Item, ResolutionPreset>` — off_1222268, +44.
    pub fn enum_prop_descriptor_resolution_preset_dtor(b: &mut PropDescriptorBox) {
        b.destroy("off_1222268");
    }
    /// Batch 3: was: `BoundFuncDesc<Item, int(), 0>` box for the 0xb4d0 dtor —
    /// restores `off_1222248` then `_M_clear`s the signature-item list at +8 (0xb4ec).
    #[derive(Debug, Default)]
    pub struct BoundFuncDescBox {
        pub vtable: &'static str,
        pub signatures: Vec<String>,
    }
    /// Batch 3, IDA 0xb4d0 `~BoundFuncDesc<Item, int(), 0>`.
    pub fn bound_func_desc_dtor(b: &mut BoundFuncDescBox) {
        b.vtable = "off_1222248";
        b.signatures.clear();
    }
    /// Batch 2, IDA 0xb3bc `~PropDescriptor<CRenderSettingsItem, bool>`:
    /// restore `off_1222378`, free +40.
    pub fn prop_descriptor_bool_dtor(b: &mut PropDescriptorBox) {
        b.destroy("off_1222378");
    }

    /// Batch 2: was: `std::logic_error`/`length_error`/`out_of_range` dtors.
    /// Rust `Drop` replaces the exception-object lifetime; these model the exact
    /// thunk/delete split: D1/D2 run the base dtor only, D0 additionally frees.
    pub mod std_exceptions {
        use std::ffi::c_void;

        /// IDA 0x9b2c/0x9b44 target `std::logic_error::~logic_error` — base-class
        /// teardown; no owned state in this port.
        pub unsafe fn logic_error_dtor(_this: *mut c_void) {}
        /// IDA 0x9b2c `std::length_error::~length_error() D1` — thunk to the base
        /// dtor, no delete.
        pub unsafe fn length_error_d1(this: *mut c_void) {
            logic_error_dtor(this);
        }
        /// IDA 0x9b44 `std::out_of_range::~out_of_range() D2` — base dtor, no delete.
        pub unsafe fn out_of_range_d2(this: *mut c_void) {
            logic_error_dtor(this);
        }
        /// IDA 0x9b30/0x9b36-0x9b40 `~out_of_range() D0` — base dtor plus
        /// `operator delete(this)`; the caller hands ownership over.
        pub unsafe fn out_of_range_d0(this: *mut c_void) {
            logic_error_dtor(this);
            drop(Box::from_raw(this as *mut u8));
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
// IDA 0x9ac8: store +124 when changed, emit `unk_130C2AC`. Returns `this`.
pub fn stub_9ac8(item: &mut render_settings::CRenderSettingsItem, level: i32) -> &mut render_settings::CRenderSettingsItem {
    item.set_auto_quality_level(level)
}

#[doc(alias = "non-virtual thunk toCRenderSettingsItem::setAutoQualityLevel(int)")]
// 0x9ae8 — __ZThn96_N19CRenderSettingsItem19setAutoQualityLevelEi
// type: int __fastcall(int this, int)
// IDA 0x9ae8: non-virtual thunk, `this` biased -96 (subobject); same store+emit.
pub fn stub_9ae8(item: &mut render_settings::CRenderSettingsItem, level: i32) -> &mut render_settings::CRenderSettingsItem {
    item.set_auto_quality_level_thunk(level)
}

#[doc(alias = "CRenderSettingsItem::setEagerBulkExecution(bool)")]
// 0x9b08 — __ZN19CRenderSettingsItem21setEagerBulkExecutionEb
// type: int __fastcall(int this, int)
// IDA 0x9b08: store byte +157 when changed, emit `unk_130C1E8`. Returns `this`.
pub fn stub_9b08(item: &mut render_settings::CRenderSettingsItem, value: bool) -> &mut render_settings::CRenderSettingsItem {
    item.set_eager_bulk_execution(value)
}

#[doc(alias = "std::length_error::~length_error()")]
// 0x9b2c — __ZNSt12length_errorD1Ev
// type: void __cdecl(std::length_error *__hidden this)
// IDA 0x9b2c: `length_error::~length_error() D1` thunk to the base dtor.
pub fn stub_9b2c(this: *mut std::ffi::c_void) {
    unsafe { render_settings::std_exceptions::length_error_d1(this) }
}

#[doc(alias = "std::out_of_range::~out_of_range()")]
// 0x9b30 — __ZNSt12out_of_rangeD0Ev
// type: void __cdecl(std::out_of_range *__hidden this)
// IDA 0x9b30: `out_of_range::~out_of_range() D0` — dtor plus `operator delete`.
// The caller hands ownership over, mirroring the deleting-destructor contract.
pub fn stub_9b30(this: *mut std::ffi::c_void) {
    unsafe { render_settings::std_exceptions::out_of_range_d0(this) }
}

#[doc(alias = "std::out_of_range::~out_of_range()")]
// 0x9b44 — __ZNSt12out_of_rangeD2Ev
// type: void __cdecl(std::out_of_range *__hidden this)
// IDA 0x9b44: `out_of_range::~out_of_range() D2` thunk to the base dtor.
pub fn stub_9b44(this: *mut std::ffi::c_void) {
    unsafe { render_settings::std_exceptions::out_of_range_d2(this) }
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::addPair(RBX::CRenderSettings::AASamples,char const*)")]
// 0x9b48 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
// IDA 0x9b48: `aa_samples_desc` `addPair` instantiation — shared template body,
// ported once on `EnumDescData::add_pair`; this records the per-type call.
pub fn stub_9b48(desc: &mut render_settings::EnumDescData, value: i32, name: &str) {
    desc.add_pair(value, name)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::addPair(RBX::CRenderSettings::GraphicsMode,char const*)")]
// 0x9ea8 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
// IDA 0x9ea8: `graphics_mode_desc` `addPair` instantiation — shared template body,
// ported once on `EnumDescData::add_pair`; this records the per-type call.
pub fn stub_9ea8(desc: &mut render_settings::EnumDescData, value: i32, name: &str) {
    desc.add_pair(value, name)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::addLegacy(int,char const*,RBX::CRenderSettings::GraphicsMode)")]
// 0xa208 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE9addLegacyEiPKcS3_
// type: _DWORD *__fastcall(int, unsigned int, int, int)
// IDA 0xa208: `addLegacy(index, name, value)` — remap resize plus alias insert.
pub fn stub_a208(desc: &mut render_settings::EnumDescData, index: i32, name: &str, value: i32) {
    desc.add_legacy(index, name, value)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::addPair(RBX::CRenderSettings::FrameRateManagerMode,char const*)")]
// 0xa25c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
// IDA 0xa25c: `frame_rate_manager_mode_desc` `addPair` instantiation — shared template body,
// ported once on `EnumDescData::add_pair`; this records the per-type call.
pub fn stub_a25c(desc: &mut render_settings::EnumDescData, value: i32, name: &str) {
    desc.add_pair(value, name)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::addPair(RBX::CRenderSettings::AntialiasingMode,char const*)")]
// 0xa5bc — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
// IDA 0xa5bc: `antialiasing_mode_desc` `addPair` instantiation — shared template body,
// ported once on `EnumDescData::add_pair`; this records the per-type call.
pub fn stub_a5bc(desc: &mut render_settings::EnumDescData, value: i32, name: &str) {
    desc.add_pair(value, name)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::addPair(RBX::CRenderSettings::ShadowMode,char const*)")]
// 0xa91c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
// IDA 0xa91c: `shadow_mode_desc` `addPair` instantiation — shared template body,
// ported once on `EnumDescData::add_pair`; this records the per-type call.
pub fn stub_a91c(desc: &mut render_settings::EnumDescData, value: i32, name: &str) {
    desc.add_pair(value, name)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::addPair(RBX::CRenderSettings::QualityLevel,char const*)")]
// 0xac7c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
// IDA 0xac7c: `quality_level_desc` `addPair` instantiation — shared template body,
// ported once on `EnumDescData::add_pair`; this records the per-type call.
pub fn stub_ac7c(desc: &mut render_settings::EnumDescData, value: i32, name: &str) {
    desc.add_pair(value, name)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::addPair(RBX::CRenderSettings::ResolutionPreset,char const*)")]
// 0xafdc — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
// IDA 0xafdc: `resolution_preset_desc` `addPair` instantiation — shared template body,
// ported once on `EnumDescData::add_pair`; this records the per-type call.
pub fn stub_afdc(desc: &mut render_settings::EnumDescData, value: i32, name: &str) {
    desc.add_pair(value, name)
}

#[doc(alias = "RBX::CRenderSettings::getGraphicsMode(void)const")]
// 0xb33c — __ZNK3RBX15CRenderSettings15getGraphicsModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
// IDA 0xb33c: `getGraphicsMode` reads item+100 (subobject +4).
pub fn stub_b33c(item: &render_settings::CRenderSettingsItem) -> render_settings::GraphicsMode {
    item.graphics_mode()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::~EnumPropDescriptor()")]
// 0xb340 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
// IDA 0xb340: `~EnumPropDescriptor<Item, GraphicsMode>` — restore off_12228E8, free +44.
pub fn stub_b340(b: &mut render_settings::PropDescriptorBox) {
    render_settings::enum_prop_descriptor_graphics_mode_dtor(b)
}

#[doc(alias = "RBX::CRenderSettings::getFrameRateManagerMode(void)const")]
// 0xb364 — __ZNK3RBX15CRenderSettings23getFrameRateManagerModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
// IDA 0xb364: `getFrameRateManagerMode` reads item+112 (subobject +16).
pub fn stub_b364(item: &render_settings::CRenderSettingsItem) -> render_settings::FrameRateManagerMode {
    item.frame_rate_manager_mode()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::~EnumPropDescriptor()")]
// 0xb368 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
// IDA 0xb368: `~EnumPropDescriptor<Item, FrameRateManagerMode>` — off_1222848, free +44.
pub fn stub_b368(b: &mut render_settings::PropDescriptorBox) {
    render_settings::enum_prop_descriptor_frame_rate_manager_mode_dtor(b)
}

#[doc(alias = "RBX::CRenderSettings::getQualityLevel(void)const")]
// 0xb38c — __ZNK3RBX15CRenderSettings15getQualityLevelEv
// type: int __fastcall(RBX::CRenderSettings *this)
// IDA 0xb38c: `getQualityLevel` reads item+116 (subobject +20).
pub fn stub_b38c(item: &render_settings::CRenderSettingsItem) -> render_settings::QualityLevel {
    item.quality_level()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::~EnumPropDescriptor()")]
// 0xb390 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
// IDA 0xb390: `~EnumPropDescriptor<Item, QualityLevel>` — off_12227A8, free +44.
pub fn stub_b390(b: &mut render_settings::PropDescriptorBox) {
    render_settings::enum_prop_descriptor_quality_level_dtor(b)
}

#[doc(alias = "RBX::CRenderSettings::getAlwaysDrawConnectors(void)const")]
// 0xb3b4 — __ZNK3RBX15CRenderSettings23getAlwaysDrawConnectorsEv
// type: int __fastcall(RBX::CRenderSettings *this)
// IDA 0xb3b4: `getAlwaysDrawConnectors` reads byte item+155 (subobject +59).
pub fn stub_b3b4(item: &render_settings::CRenderSettingsItem) -> bool {
    item.always_draw_connectors()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::~PropDescriptor()")]
// 0xb3bc — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembED1Ev
// type: _DWORD *__fastcall(_DWORD *)
// IDA 0xb3bc: `~PropDescriptor<Item, bool>` — restore off_1222378, free +40.
pub fn stub_b3bc(b: &mut render_settings::PropDescriptorBox) {
    render_settings::prop_descriptor_bool_dtor(b)
}

#[doc(alias = "RBX::CRenderSettings::getShowAggregation(void)const")]
// 0xb3e0 — __ZNK3RBX15CRenderSettings18getShowAggregationEv
// type: int __fastcall(RBX::CRenderSettings *this)
// IDA 0xb3e0: `getShowAggregation` reads byte item+154 (subobject +58).
pub fn stub_b3e0(item: &render_settings::CRenderSettingsItem) -> bool {
    item.show_aggregation()
}

#[doc(alias = "RBX::CRenderSettings::getAASamples(void)const")]
// 0xb3e8 — __ZNK3RBX15CRenderSettings12getAASamplesEv
// type: int __fastcall(RBX::CRenderSettings *this)
// IDA 0xb3e8: `getAASamples` reads the `aaSamples` global.
pub fn stub_b3e8() -> i32 {
    render_settings::CRenderSettingsItem::get_aa_samples()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::~EnumPropDescriptor()")]
// 0xb3f8 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
// IDA 0xb3f8: `~EnumPropDescriptor<Item, AASamples>` — off_1222658, free +44.
pub fn stub_b3f8(b: &mut render_settings::PropDescriptorBox) {
    render_settings::enum_prop_descriptor_aa_samples_dtor(b)
}

#[doc(alias = "RBX::CRenderSettings::getShadowMode(void)const")]
// 0xb41c — __ZNK3RBX15CRenderSettings13getShadowModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
// IDA 0xb41c: `getShadowMode` reads item+108 (subobject +12).
pub fn stub_b41c(item: &render_settings::CRenderSettingsItem) -> render_settings::ShadowMode {
    item.shadow_mode()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::~EnumPropDescriptor()")]
// 0xb420 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
// IDA 0xb420: `~EnumPropDescriptor<Item, ShadowMode>` — off_12224C8, free +44.
pub fn stub_b420(b: &mut render_settings::PropDescriptorBox) {
    render_settings::enum_prop_descriptor_shadow_mode_dtor(b)
}

#[doc(alias = "RBX::CRenderSettings::getAntialiasingMode(void)const")]
// 0xb444 — __ZNK3RBX15CRenderSettings19getAntialiasingModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
// IDA 0xb444: `getAntialiasingMode` reads item+104 (subobject +8).
pub fn stub_b444(item: &render_settings::CRenderSettingsItem) -> render_settings::AntialiasingMode {
    item.antialiasing_mode()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::~EnumPropDescriptor()")]
// 0xb448 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
// IDA 0xb448: `~EnumPropDescriptor<Item, AntialiasingMode>` — off_1222428, free +44.
pub fn stub_b448(b: &mut render_settings::PropDescriptorBox) {
    render_settings::enum_prop_descriptor_antialiasing_mode_dtor(b)
}

#[doc(alias = "RBX::CRenderSettings::getDebugShowBoundingBoxes(void)const")]
// 0xb46c — __ZNK3RBX15CRenderSettings25getDebugShowBoundingBoxesEv
// type: int __fastcall(RBX::CRenderSettings *this)
// IDA 0xb46c: `getDebugShowBoundingBoxes` reads byte item+136 (subobject +40).
pub fn stub_b46c(item: &render_settings::CRenderSettingsItem) -> bool {
    item.debug_show_bounding_boxes()
}

#[doc(alias = "RBX::CRenderSettings::getAutoQualityLevel(void)const")]
// 0xb474 — __ZNK3RBX15CRenderSettings19getAutoQualityLevelEv
// type: int __fastcall(RBX::CRenderSettings *this)
// IDA 0xb474: `getAutoQualityLevel` reads item+124 (subobject +28).
pub fn stub_b474(item: &render_settings::CRenderSettingsItem) -> i32 {
    item.auto_quality_level()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::~PropDescriptor()")]
// 0xb478 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiED1Ev
// type: _DWORD *__fastcall(_DWORD *)
// IDA 0xb478: `~PropDescriptor<Item, int>` — off_1222178, free +40.
pub fn stub_b478(b: &mut render_settings::PropDescriptorBox) {
    render_settings::prop_descriptor_int_dtor(b)
}

#[doc(alias = "RBX::CRenderSettings::getEnableFRM(void)const")]
// 0xb49c — __ZNK3RBX15CRenderSettings12getEnableFRMEv
// type: int __fastcall(RBX::CRenderSettings *this)
// IDA 0xb49c: `getEnableFRM` reads byte item+137 (subobject +41).
pub fn stub_b49c(item: &render_settings::CRenderSettingsItem) -> bool {
    item.enable_frm()
}

#[doc(alias = "RBX::CRenderSettings::getResolutionPreference(void)const")]
// 0xb4a4 — __ZNK3RBX15CRenderSettings23getResolutionPreferenceEv
// type: int __fastcall(RBX::CRenderSettings *this)
// IDA 0xb4a4: `getResolutionPreference` reads item+120 (subobject +24).
pub fn stub_b4a4(item: &render_settings::CRenderSettingsItem) -> render_settings::ResolutionPreset {
    item.resolution_preference()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::~EnumPropDescriptor()")]
// 0xb4a8 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
// IDA 0xb4a8: `~EnumPropDescriptor<Item, ResolutionPreset>` — off_1222268, free +44.
pub fn stub_b4a8(b: &mut render_settings::PropDescriptorBox) {
    render_settings::enum_prop_descriptor_resolution_preset_dtor(b)
}

#[doc(alias = "RBX::CRenderSettings::getMaxQualityLevel(void)")]
// 0xb4cc — __ZN3RBX15CRenderSettings18getMaxQualityLevelEv
// type: int __fastcall(RBX::CRenderSettings *this)
// IDA 0xb4cc: `getMaxQualityLevel` reads item+128 (subobject +32).
pub fn stub_b4cc(item: &render_settings::CRenderSettingsItem) -> i32 {
    item.max_quality_level()
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::~BoundFuncDesc()")]
// 0xb4d0 — __ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
// IDA 0xb4d0: `~BoundFuncDesc<Item, int(), 0>` — off_1222248 plus signature-list clear.
pub fn stub_b4d0(b: &mut render_settings::BoundFuncDescBox) {
    render_settings::bound_func_desc_dtor(b)
}

#[doc(alias = "RBX::CRenderSettings::getTextureCacheSize(void)const")]
// 0xb4f4 — __ZNK3RBX15CRenderSettings19getTextureCacheSizeEv
// type: int __fastcall(RBX::CRenderSettings *this)
// IDA 0xb4f4: `getTextureCacheSize` reads item+160 (subobject +64).
pub fn stub_b4f4(item: &render_settings::CRenderSettingsItem) -> u32 {
    item.texture_cache_size()
}

#[doc(alias = "RBX::CRenderSettings::getMeshCacheSize(void)const")]
// 0xb4f8 — __ZNK3RBX15CRenderSettings16getMeshCacheSizeEv
// type: int __fastcall(RBX::CRenderSettings *this)
// IDA 0xb4f8: `getMeshCacheSize` reads item+164 (subobject +68).
pub fn stub_b4f8(item: &render_settings::CRenderSettingsItem) -> u32 {
    item.mesh_cache_size()
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEEC2Ev")]
// 0xb4fc — __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEEC2Ev
// type: RBX::Instance *__fastcall(RBX::Instance *)
// IDA 0xb4fc: `GlobalAdvancedSettingsItem` base ctor — Instance base, vtable installs,
// classDescriptor + registrar, +92 set, name "RenderSettings", singleton claim
// (the already-exists `runtime_error` throw is `Err`).
pub fn stub_b4fc() -> Result<render_settings::GlobalAdvancedSettingsBase, String> {
    render_settings::claim_render_settings_singleton()
}

#[doc(alias = "std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::push_back(G3D::Vector2int16 const&)")]
// 0xb740 — __ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE9push_backERKS1_
// type: int __fastcall(int result, _DWORD *)
// IDA 0xb740: `vector<Vector2int16>::push_back` — fast write+bump, grow on full.
pub fn stub_b740(list: &mut Vec<(u16, u16)>, x: u16, y: u16) {
    render_settings::vector2int16_push_back(list, x, y)
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Reflection::PropertyDescriptor const*)>::operator()(RBX::Reflection::PropertyDescriptor const*)")]
// 0xb76c — __ZN3rbx7signals16signal_with_argsILi1EFvPKN3RBX10Reflection18PropertyDescriptorEEEclES6_
// type: void __fastcall(_DWORD *, int, int, const void *, int, int, int, int, void *, int)
// IDA 0xb76c: 1-arg `signal::operator()` — null-head no-op, SignalPrints trace,
// connected-slot walk with the emitted property, terminal release. Delegates to
// the traced emit; per-slot connection state lives on the signal.
pub fn stub_b76c(sig: &mut render_settings::PropertyChangedSignal, prop: &'static str) {
    sig.emit_traced(prop)
}

#[doc(alias = "RBX::CRenderSettings::getEagerBulkExecution(void)const")]
// 0xb8b0 — __ZNK3RBX15CRenderSettings21getEagerBulkExecutionEv
// type: int __fastcall(RBX::CRenderSettings *this)
// IDA 0xb8b0: `getEagerBulkExecution` reads byte item+157 (subobject +61).
pub fn stub_b8b0(item: &render_settings::CRenderSettingsItem) -> bool {
    item.eager_bulk_execution()
}

#[doc(alias = "CRenderSettingsItem::~CRenderSettingsItem()")]
// 0xb8b8 — __ZN19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
// IDA 0xb8b8: `~CRenderSettingsItem() D1` — thunk to the D2 member teardown.
pub fn stub_b8b8(item: &mut render_settings::CRenderSettingsItem) {
    item.destroy_d1()
}

#[doc(alias = "CRenderSettingsItem::~CRenderSettingsItem()")]
// 0xb8bc — __ZN19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
// IDA 0xb8bc: `~CRenderSettingsItem() D0` — D2 teardown plus `operator delete`.
// Ownership moves in, mirroring the deleting-destructor contract.
pub fn stub_b8bc(item: Box<render_settings::CRenderSettingsItem>) {
    let mut item = item;
    item.destroy_d2();
}

#[doc(alias = "__ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv")]
// 0xb8d0 — __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv
// type: int()
// IDA 0xb8d0: `FactoryProduct::getClassName` via the static creator hop (collapsed).
pub fn stub_b8d0() -> &'static str {
    render_settings::render_settings_item_class_name()
}

#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
// 0xb8e0 — __ZThn32_N19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
// IDA 0xb8e0: non-virtual thunk to D1 — incoming `this` biased -32 (subobject vec
// entry), a no-op in the flat layout; delegates to the D1 teardown.
pub fn stub_b8e0(item: &mut render_settings::CRenderSettingsItem) {
    item.destroy_d1()
}

#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
// 0xb8e8 — __ZThn32_N19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
// IDA 0xb8e8: non-virtual thunk to D0 — same -32 bias, then D2 plus free.
pub fn stub_b8e8(item: Box<render_settings::CRenderSettingsItem>) {
    stub_b8bc(item)
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv")]
// 0xb900 — __ZThn32_NK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv
// type: int()
// IDA 0xb900: Thn32 to `FactoryProduct::getClassName` — same static-creator hop,
// collapsed like 0xb8d0 (the `this`-bias of the thunk is a no-op here).
pub fn stub_b900() -> &'static str {
    render_settings::render_settings_item_class_name()
}

#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
// 0xb910 — __ZThn36_N19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
// IDA 0xb910: non-virtual thunk to D1 — incoming `this` biased -36 (0xb912),
// a no-op in the flat layout; delegates to the D1 teardown.
pub fn stub_b910(item: &mut render_settings::CRenderSettingsItem) {
    item.destroy_d1()
}

#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
// 0xb918 — __ZThn36_N19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
// IDA 0xb918: non-virtual thunk to D0 — same -36 bias, then D2 plus free.
pub fn stub_b918(item: Box<render_settings::CRenderSettingsItem>) {
    stub_b8bc(item)
}

#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD1Ev")]
// 0xb930 — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD1Ev
// type: int()
// IDA 0xb930: `FactoryProduct::Creator D1` — thunk to D2.
pub fn stub_b930() {
    render_settings::render_settings_creator_d1()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()")]
// 0xb934 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED1Ev
// type: int()
// IDA 0xb934: `EnumDesc::D1` — thunk to the D2 member teardown.
pub fn stub_b934(desc: &mut render_settings::EnumDescData) {
    desc.destroy_d1()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()")]
// 0xb938 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED0Ev
// type: int __fastcall(int)
// IDA 0xb938: `EnumDesc::D0` — D2 teardown plus `operator delete`.
// Ownership moves in, mirroring the deleting-destructor contract.
pub fn stub_b938(desc: Box<render_settings::EnumDescData>) {
    let mut desc = desc;
    desc.destroy_d2()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::lookup(char const*)const")]
// 0xb94c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
// IDA 0xb94c: `lookup(name)` — `Name::lookup`, `convertToValue`,
// `convertToItem`; miss yields null (`None` here).
pub fn stub_b94c(desc: &render_settings::EnumDescData, name: &str) -> Option<usize> {
    desc.lookup_by_name(name)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::lookup(RBX::Reflection::Variant const&)const")]
// 0xb97c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
// IDA 0xb97c: `lookup(variant)` — `any_cast` payload, `convertToItem`.
pub fn stub_b97c(desc: &render_settings::EnumDescData, variant: &render_settings::Variant) -> Option<usize> {
    desc.lookup_by_value(variant)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0xb99c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueEmRNS0_7VariantE
// IDA 0xb99c: `convertToValue(index, out)` — per-value table read with
// the `+40`-bound check; `false` leaves `out` untouched.
pub fn stub_b99c(desc: &render_settings::EnumDescData, index: usize, out: &mut render_settings::Variant) -> bool {
    desc.convert_to_value(index, out)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToString(unsigned long,std::string &)const")]
// 0xb9f8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
// IDA 0xb9f8: `convertToString(value, out)` — name-slot assign on hit,
// `out` untouched and `false` on miss.
pub fn stub_b9f8(desc: &render_settings::EnumDescData, value: usize, out: &mut String) -> bool {
    desc.convert_to_string(value, out)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()")]
// 0xbb3c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED1Ev
// type: int()
// IDA 0xbb3c: `EnumDesc::D1` — thunk to the D2 member teardown.
pub fn stub_bb3c(desc: &mut render_settings::EnumDescData) {
    desc.destroy_d1()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()")]
// 0xbb40 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED0Ev
// type: int __fastcall(int)
// IDA 0xbb40: `EnumDesc::D0` — D2 teardown plus `operator delete`.
// Ownership moves in, mirroring the deleting-destructor contract.
pub fn stub_bb40(desc: Box<render_settings::EnumDescData>) {
    let mut desc = desc;
    desc.destroy_d2()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::lookup(char const*)const")]
// 0xbb54 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
// IDA 0xbb54: `lookup(name)` — `Name::lookup`, `convertToValue`,
// `convertToItem`; miss yields null (`None` here).
pub fn stub_bb54(desc: &render_settings::EnumDescData, name: &str) -> Option<usize> {
    desc.lookup_by_name(name)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::lookup(RBX::Reflection::Variant const&)const")]
// 0xbb84 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
// IDA 0xbb84: `lookup(variant)` — `any_cast` payload, `convertToItem`.
pub fn stub_bb84(desc: &render_settings::EnumDescData, variant: &render_settings::Variant) -> Option<usize> {
    desc.lookup_by_value(variant)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0xbba4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueEmRNS0_7VariantE
// IDA 0xbba4: `convertToValue(index, out)` — per-value table read with
// the `+40`-bound check; `false` leaves `out` untouched.
pub fn stub_bba4(desc: &render_settings::EnumDescData, index: usize, out: &mut render_settings::Variant) -> bool {
    desc.convert_to_value(index, out)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToString(unsigned long,std::string &)const")]
// 0xbc00 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
// IDA 0xbc00: `convertToString(value, out)` — name-slot assign on hit,
// `out` untouched and `false` on miss.
pub fn stub_bc00(desc: &render_settings::EnumDescData, value: usize, out: &mut String) -> bool {
    desc.convert_to_string(value, out)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::~EnumDesc()")]
// 0xbd44 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED1Ev
// type: int()
// IDA 0xbd44: `EnumDesc::D1` — thunk to the D2 member teardown.
pub fn stub_bd44(desc: &mut render_settings::EnumDescData) {
    desc.destroy_d1()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::~EnumDesc()")]
// 0xbd48 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED0Ev
// type: int __fastcall(int)
// IDA 0xbd48: `EnumDesc::D0` — D2 teardown plus `operator delete`.
// Ownership moves in, mirroring the deleting-destructor contract.
pub fn stub_bd48(desc: Box<render_settings::EnumDescData>) {
    let mut desc = desc;
    desc.destroy_d2()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::lookup(char const*)const")]
// 0xbd5c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
// IDA 0xbd5c: `lookup(name)` — `Name::lookup`, `convertToValue`,
// `convertToItem`; miss yields null (`None` here).
pub fn stub_bd5c(desc: &render_settings::EnumDescData, name: &str) -> Option<usize> {
    desc.lookup_by_name(name)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::lookup(RBX::Reflection::Variant const&)const")]
// 0xbd8c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
// IDA 0xbd8c: `lookup(variant)` — `any_cast` payload, `convertToItem`.
pub fn stub_bd8c(desc: &render_settings::EnumDescData, variant: &render_settings::Variant) -> Option<usize> {
    desc.lookup_by_value(variant)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0xbdac — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueEmRNS0_7VariantE
// IDA 0xbdac: `convertToValue(index, out)` — per-value table read with
// the `+40`-bound check; `false` leaves `out` untouched.
pub fn stub_bdac(desc: &render_settings::EnumDescData, index: usize, out: &mut render_settings::Variant) -> bool {
    desc.convert_to_value(index, out)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToString(unsigned long,std::string &)const")]
// 0xbe08 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
// IDA 0xbe08: `convertToString(value, out)` — name-slot assign on hit,
// `out` untouched and `false` on miss.
pub fn stub_be08(desc: &render_settings::EnumDescData, value: usize, out: &mut String) -> bool {
    desc.convert_to_string(value, out)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc()")]
// 0xbf4c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED1Ev
// type: int()
// IDA 0xbf4c: `EnumDesc::D1` — thunk to the D2 member teardown.
pub fn stub_bf4c(desc: &mut render_settings::EnumDescData) {
    desc.destroy_d1()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc()")]
// 0xbf50 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED0Ev
// type: int __fastcall(int)
// IDA 0xbf50: `EnumDesc::D0` — D2 teardown plus `operator delete`.
// Ownership moves in, mirroring the deleting-destructor contract.
pub fn stub_bf50(desc: Box<render_settings::EnumDescData>) {
    let mut desc = desc;
    desc.destroy_d2()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::lookup(char const*)const")]
// 0xbf64 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
// IDA 0xbf64: `lookup(name)` — `Name::lookup`, `convertToValue`,
// `convertToItem`; miss yields null (`None` here).
pub fn stub_bf64(desc: &render_settings::EnumDescData, name: &str) -> Option<usize> {
    desc.lookup_by_name(name)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::lookup(RBX::Reflection::Variant const&)const")]
// 0xbf94 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
// IDA 0xbf94: `lookup(variant)` — `any_cast` payload, `convertToItem`.
pub fn stub_bf94(desc: &render_settings::EnumDescData, variant: &render_settings::Variant) -> Option<usize> {
    desc.lookup_by_value(variant)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0xbfb4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToValueEmRNS0_7VariantE
// IDA 0xbfb4: `convertToValue(index, out)` — per-value table read with
// the `+40`-bound check; `false` leaves `out` untouched.
pub fn stub_bfb4(desc: &render_settings::EnumDescData, index: usize, out: &mut render_settings::Variant) -> bool {
    desc.convert_to_value(index, out)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToString(unsigned long,std::string &)const")]
// 0xc010 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
// IDA 0xc010: `convertToString(value, out)` — name-slot assign on hit,
// `out` untouched and `false` on miss.
pub fn stub_c010(desc: &render_settings::EnumDescData, value: usize, out: &mut String) -> bool {
    desc.convert_to_string(value, out)
}
