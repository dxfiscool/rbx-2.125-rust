//! Auto-generated skeletons for rbx-script — script gap filler EA asc 0x15ef8..0x31bec
//! Filter: Script|Lua|LuaBridge|Yield|ProtectedString (4921 filtered, 0 remaining not yet in script — gap filler global EA asc distinct not yet in crates/script/src)
//! Source: ida/export.json (85545 funcs, base 0x4000, size 0x13a8efc)
//! Batch: +120 stubs | range 0x15ef8..0x31bec | script 23341->23461 total (EA-sorted asc distinct not yet in crates/script/src, rbx_core::SharedPtr not boost, // 0xADDR mangled + #[doc(alias)] + todo!("0xADDR"))
//! Remaining not in script before batch: 62744 -> after: 62624 (filtered Script|Lua exhausted, global gap filler EA asc)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;
use parking_lot::Mutex;
use rbx_reflection::generated::Tuple;
use std::collections::HashMap;
use std::sync::LazyLock;

/// `NSUserDefaults` value behind the `kAppirater*` keys (IDA 0x183d8..0x185b0).
#[derive(Debug, Clone)]
pub enum AppiraterPref {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}

/// `Appirater` instance state (Appirater.m, IDA 0x17df0..0x19a30).
/// ObjC `+[Appirater ...]`/`-[Appirater ...]` map to the `stub_0x*` fns below
/// (originals searchable via the `#[doc(alias)]` lines); `NSUserDefaults`
/// becomes [`HashMap`], `dispatch_once` sharedInstance becomes [`LazyLock`],
/// `NSURLConnection` probing folds into [`Appirater::network_reachable`].
#[derive(Debug, Default)]
pub struct Appirater {
    /// `+[Appirater setAppId:]` global (IDA 0x17dfa).
    pub app_id: Option<String>,
    /// `+[Appirater setDaysUntilPrompt:]` (IDA 0x17e0e).
    pub days_until_prompt: f64,
    /// `+[Appirater setUsesUntilPrompt:]` (IDA 0x17e1e).
    pub uses_until_prompt: i64,
    /// `+[Appirater setSignificantEventsUntilPrompt:]` (IDA 0x17e2e).
    pub significant_events_until_prompt: i64,
    /// `+[Appirater setTimeBeforeReminding:]` (IDA 0x17e42).
    pub time_before_reminding: f64,
    /// `+[Appirater setDebug:]` (IDA 0x17e52).
    pub debug: bool,
    /// `+[Appirater setDelegate:]` slot (IDA 0x17e62).
    pub delegate: Option<u32>,
    /// `NSUserDefaults` suite (`kAppirater*` keys).
    pub prefs: HashMap<String, AppiraterPref>,
    /// `-[Appirater connectedToNetwork]` latch (IDA 0x17e68..0x17f78):
    /// reachability flags plus the `NSURLConnection` probe, both true on a
    /// live device.
    pub network_reachable: bool,
    /// `-[Appirater showRatingAlert]` ran (IDA 0x180a8).
    pub rating_alert_visible: bool,
    /// `appWillResignActive` observer installed by the sharedInstance block
    /// (IDA 0x17fe4..0x18092).
    pub observes_resign_active: bool,
    /// `-[Appirater ratingAlert]` slot (IDA 0x191d4..0x19200).
    pub rating_alert: Option<u32>,
    /// Delegate callbacks from `alertView:clickedButtonAtIndex:` (IDA
    /// 0x19028..0x19140).
    pub delegate_events: Vec<AppiraterDelegateEvent>,
}

impl Appirater {
    pub fn new() -> Self {
        Self { network_reachable: true, ..Default::default() }
    }
    fn pref_int(&self, key: &str) -> i64 {
        match self.prefs.get(key) {
            Some(AppiraterPref::Int(v)) => *v,
            Some(AppiraterPref::Float(v)) => *v as i64,
            Some(AppiraterPref::Bool(v)) => i64::from(*v),
            _ => 0,
        }
    }
    fn pref_float(&self, key: &str) -> f64 {
        match self.prefs.get(key) {
            Some(AppiraterPref::Float(v)) => *v,
            Some(AppiraterPref::Int(v)) => *v as f64,
            _ => 0.0,
        }
    }
    fn pref_bool(&self, key: &str) -> bool {
        matches!(self.prefs.get(key), Some(AppiraterPref::Bool(true)))
    }
    fn pref_str(&self, key: &str) -> Option<&str> {
        match self.prefs.get(key) {
            Some(AppiraterPref::Str(v)) => Some(v),
            _ => None,
        }
    }
}

/// `+[Appirater sharedInstance]` slot (IDA 0x17f80..0x17fe0): `dispatch_once`
/// init folded into [`LazyLock`]; `boost::shared_ptr` -> [`SharedPtr`].
static APPIRATER_SHARED: LazyLock<SharedPtr<Mutex<Appirater>>> =
    LazyLock::new(|| SharedPtr::new(Mutex::new(Appirater::new())));
use rbx_reflection::enum_desc::EnumDesc;
use rbx_reflection::generated::{
    antialiasing_mode_enum_desc, frame_rate_manager_mode_enum_desc,
    graphics_mode_enum_desc, quality_level_enum_desc, shadow_mode_enum_desc,
};
use rbx_reflection::generated_shard_dh::resolution_preset_enum_desc;
use std::collections::BTreeMap;
/// `AppiraterDelegate` callback recorded by
/// `-[Appirater alertView:clickedButtonAtIndex:]` (IDA 0x19028..0x19140).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppiraterDelegateEvent {
    OptToRate,
    OptToRemindLater,
    OptToDecline,
}

/// `AppDelegate` host state (IDA 0x19228..0x19a30): window lifetime,
/// analytics/launch latches. UIKit objects (`UIWindow`, `UIAlertView`,
/// `PlaceLauncher`) live on the platform side; only the observable latches
/// are modeled here.
#[derive(Debug, Default)]
pub struct AppDelegate {
    pub launched: bool,
    pub flurry_session_key: Option<String>,
    pub resigned_active: bool,
    pub entered_background: bool,
    pub received_memory_warning: bool,
}

// 0x15ef8 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE11_M_allocateEm")]
pub fn stub_0x15ef8(cap: usize) -> Vec<i32> {
    // IDA 0x15ef8 `_Vector_base<GraphicsMode>::_M_allocate` (cf. 0x145c4).
    Vec::with_capacity(cap)
}

// 0x15f10 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings12GraphicsModeES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::CRenderSettings::GraphicsMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *>(RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings12GraphicsModeES6_EET0_T_S8_S7_")]
pub fn stub_0x15f10(slots: &mut Vec<i32>, src: std::ops::Range<usize>, dst_end: usize) {
    // IDA 0x15f10 `__copy_backward` over `GraphicsMode` slots (cf. 0x145dc).
    slots.copy_within(src, dst_end);
}

// 0x15f4c — __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::push_back(RBX::CRenderSettings::GraphicsMode const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE9push_backERKS2_")]
pub fn stub_0x15f4c(slots: &mut Vec<i32>, value: i32) {
    // IDA 0x15f4c `vector<GraphicsMode>::push_back` (cf. 0x144b8).
    slots.push(value);
}

// 0x15f74 — __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::GraphicsMode*,std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>>,RBX::CRenderSettings::GraphicsMode const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_0x15f74(slots: &mut Vec<i32>, pos: usize, value: i32) {
    // IDA 0x15f74 `vector<GraphicsMode>::_M_insert_aux` (cf. 0x144e0).
    let pos = pos.min(slots.len());
    slots.insert(pos, value);
}

// 0x16058 — __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::resize(unsigned long,RBX::CRenderSettings::AASamples)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE6resizeEmS2_")]
pub fn stub_0x16058(slots: &mut Vec<i32>, len: usize, value: i32) {
    // IDA 0x16058 `vector<AASamples>::resize(n, v)` (cf. 0x14484).
    slots.resize(len, value);
}

// 0x1608c — __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::push_back(RBX::CRenderSettings::AASamples const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE9push_backERKS2_")]
pub fn stub_0x1608c(slots: &mut Vec<i32>, value: i32) {
    // IDA 0x1608c `vector<AASamples>::push_back` (cf. 0x144b8).
    slots.push(value);
}

// 0x160b4 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings9AASamplesESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::AASamples,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings9AASamplesESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_0x160b4(map: &mut BTreeMap<u32, i32>, key: u32) -> &mut i32 {
    // IDA 0x160b4 `map<Name const*, AASamples>::operator[]` (cf. 0x142b8).
    map.entry(key).or_default()
}

// 0x1610c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_0x1610c(map: &mut BTreeMap<u32, i32>, key: u32, value: i32) -> bool {
    // IDA 0x1610c `_M_insert_unique(hint, pair)` (AASamples map; cf.
    // 0x14310).
    map.insert(key, value).is_none()
}

// 0x161c0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_0x161c0(map: &mut BTreeMap<u32, i32>, key: u32, value: i32) -> Option<i32> {
    // IDA 0x161c0 `_M_insert(pos, pair)` (AASamples map; cf. 0x143c4).
    map.insert(key, value)
}

// 0x16218 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_0x16218(map: &mut BTreeMap<u32, i32>, key: u32, value: i32) -> bool {
    // IDA 0x16218 `_M_insert_unique(pair)` (AASamples map; cf. 0x1441c).
    map.insert(key, value).is_none()
}

// 0x16280 — __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AASamples*,std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>>,RBX::CRenderSettings::AASamples const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_0x16280(slots: &mut Vec<i32>, pos: usize, value: i32) {
    // IDA 0x16280 `vector<AASamples>::_M_insert_aux` (cf. 0x144e0).
    let pos = pos.min(slots.len());
    slots.insert(pos, value);
}

// 0x16364 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings9AASamplesESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX15CRenderSettings9AASamplesESaIS2_EE11_M_allocateEm")]
pub fn stub_0x16364(cap: usize) -> Vec<i32> {
    // IDA 0x16364 `_Vector_base<AASamples>::_M_allocate` (cf. 0x145c4).
    Vec::with_capacity(cap)
}

// 0x1637c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings9AASamplesES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::CRenderSettings::AASamples * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *>(RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings9AASamplesES6_EET0_T_S8_S7_")]
pub fn stub_0x1637c(slots: &mut Vec<i32>, src: std::ops::Range<usize>, dst_end: usize) {
    // IDA 0x1637c `__copy_backward` over `AASamples` slots (cf. 0x145dc).
    slots.copy_within(src, dst_end);
}

// 0x163b8 — __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AASamples*,std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>>,unsigned long,RBX::CRenderSettings::AASamples const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_0x163b8(slots: &mut Vec<i32>, pos: usize, count: usize, value: i32) {
    // IDA 0x163b8 `vector<AASamples>::_M_fill_insert` (cf. 0x14618).
    let pos = pos.min(slots.len());
    slots.splice(pos..pos, std::iter::repeat_n(value, count));
}

// 0x16548 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings10ShadowModeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings10ShadowModeEEEE13initSingletonEv")]
pub fn stub_0x16548() -> &'static EnumDesc {
    // IDA 0x16548 `Singleton<EnumDesc<ShadowMode>>::initSingleton` (thunk):
    // tail-calls `doGetSingleton` (0x1654c).
    shadow_mode_enum_desc()
}

// 0x1654c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings10ShadowModeEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings10ShadowModeEEEE14doGetSingletonEv")]
pub fn stub_0x1654c() -> &'static EnumDesc {
    // IDA 0x1654c `Singleton<EnumDesc<ShadowMode>>::doGetSingleton`:
    // `__cxa_guard_acquire` call-once EnumDesc init (0x165a8..0x165c2) +
    // `__cxa_atexit` dtor registration (0x165e0), returning the singleton
    // (0x16610). Host folds the guard into the [`LazyLock`] singleton.
    shadow_mode_enum_desc()
}

// 0x1663c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16ResolutionPresetEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16ResolutionPresetEEEE13initSingletonEv")]
pub fn stub_0x1663c() -> &'static EnumDesc {
    // IDA 0x1663c `Singleton<EnumDesc<ResolutionPreset>>::initSingleton`
    // (thunk -> 0x16640).
    resolution_preset_enum_desc()
}

// 0x16640 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16ResolutionPresetEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16ResolutionPresetEEEE14doGetSingletonEv")]
pub fn stub_0x16640() -> &'static EnumDesc {
    // IDA 0x16640 `Singleton<EnumDesc<ResolutionPreset>>::doGetSingleton`:
    // call-once init + atexit dtor + singleton return (cf. 0x1654c).
    resolution_preset_enum_desc()
}

// 0x16730 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12QualityLevelEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12QualityLevelEEEE13initSingletonEv")]
pub fn stub_0x16730() -> &'static EnumDesc {
    // IDA 0x16730 `Singleton<EnumDesc<QualityLevel>>::initSingleton`
    // (thunk).
    quality_level_enum_desc()
}

// 0x16734 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12QualityLevelEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12QualityLevelEEEE14doGetSingletonEv")]
pub fn stub_0x16734() -> &'static EnumDesc {
    // IDA 0x16734 `Singleton<EnumDesc<QualityLevel>>::doGetSingleton` (cf.
    // 0x1654c).
    quality_level_enum_desc()
}

// 0x16824 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16AntialiasingModeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16AntialiasingModeEEEE13initSingletonEv")]
pub fn stub_0x16824() -> &'static EnumDesc {
    // IDA 0x16824 `Singleton<EnumDesc<AntialiasingMode>>::initSingleton`
    // (thunk).
    antialiasing_mode_enum_desc()
}

// 0x16828 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16AntialiasingModeEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16AntialiasingModeEEEE14doGetSingletonEv")]
pub fn stub_0x16828() -> &'static EnumDesc {
    // IDA 0x16828 `Singleton<EnumDesc<AntialiasingMode>>::doGetSingleton`
    // (cf. 0x1654c).
    antialiasing_mode_enum_desc()
}

// 0x16918 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEEE13initSingletonEv")]
pub fn stub_0x16918() -> &'static EnumDesc {
    // IDA 0x16918 `Singleton<EnumDesc<FrameRateManagerMode>>::initSingleton`
    // (thunk).
    frame_rate_manager_mode_enum_desc()
}

// 0x1691c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEEE14doGetSingletonEv")]
pub fn stub_0x1691c() -> &'static EnumDesc {
    // IDA 0x1691c `Singleton<EnumDesc<FrameRateManagerMode>>::doGetSingleton`
    // (cf. 0x1654c).
    frame_rate_manager_mode_enum_desc()
}

// 0x16a0c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12GraphicsModeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12GraphicsModeEEEE13initSingletonEv")]
pub fn stub_0x16a0c() -> &'static EnumDesc {
    // IDA 0x16a0c `Singleton<EnumDesc<GraphicsMode>>::initSingleton`
    // (thunk).
    graphics_mode_enum_desc()
}

// 0x16a10 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12GraphicsModeEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12GraphicsModeEEEE14doGetSingletonEv")]
pub fn stub_0x16a10() -> &'static EnumDesc {
    // IDA 0x16a10 `Singleton<EnumDesc<GraphicsMode>>::doGetSingleton` (cf.
    // 0x1654c).
    graphics_mode_enum_desc()
}

// 0x16b00 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings9AASamplesEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings9AASamplesEEEE13initSingletonEv")]
pub fn stub_0x16b00() -> &'static EnumDesc {
    // IDA 0x16b00 `Singleton<EnumDesc<AASamples>>::initSingleton` (thunk).
    crate::generated_wdog_script_B2_1788369654::aa_samples_enum_desc()
}

// 0x16b04 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings9AASamplesEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings9AASamplesEEEE14doGetSingletonEv")]
pub fn stub_0x16b04() -> &'static EnumDesc {
    // IDA 0x16b04 `Singleton<EnumDesc<AASamples>>::doGetSingleton` (cf.
    // 0x1654c).
    crate::generated_wdog_script_B2_1788369654::aa_samples_enum_desc()
}

// 0x16bf4 — __ZN19CRenderSettingsItemD2Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "CRenderSettingsItem::~CRenderSettingsItem()")]
#[doc(alias = "__ZN19CRenderSettingsItemD2Ev")]
pub fn stub_0x16bf4() {
    // IDA 0x16bf4 `CRenderSettingsItem::~CRenderSettingsItem`: vtable resets
    // (0x16c28..0x16c42), property-changed `disconnectAll` (0x16c74;
    // `rbx::signals` -> [`rbx_core::signal::Signal`]), intrusive-ptr release
    // (0x16c7a..0x16c82; `boost::intrusive_ptr` -> [`SharedPtr`]), member
    // delete + string dtor (0x16c88..0x16c98), `Instance` base dtor
    // (0x16cd2). Rust Drop glue covers all frees in place.
}

// 0x16d34 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_0x16d34(map: &mut BTreeMap<u32, i32>) {
    // IDA 0x16d34 `_Rb_tree<Name, ResolutionPreset>::_M_erase(node)`:
    // recursive left-subtree erase (0x16d46), node delete (0x16d4e), right
    // walk (0x16d4c..0x16d56). Host has no tree nodes; the subtree
    // granularity collapses to the owning map (callers here are whole-table
    // teardowns), so both fold into clear.
    map.clear();
}

// 0x16d5c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_0x16d5c(map: &mut BTreeMap<u32, i32>) {
    // IDA 0x16d5c `_Rb_tree<Name, QualityLevel>::_M_erase(node)`:
    // recursive left-subtree erase, node delete, right walk (cf. 0x16d34).
    // Host has no tree nodes; granularity collapses to the owning map.
    map.clear();
}

// 0x16e4c — __GLOBAL__I_a
#[doc(alias = "global constructor keyed to_a")]
#[doc(alias = "__GLOBAL__I_a")]
pub fn stub_0x16e4c() {
    // IDA 0x16e4c `__GLOBAL__I_a`: static init storing
    // `boost::system::generic_category()`/`system_category()` singletons
    // into merged globals (disasm PUSH/R4-R7 + two BL category calls). Host
    // error categories need no init beyond `std::io`.
}

// 0x179e8 — __ZN3RBX9DataModel10serverSaveEv
// type: void __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::serverSave(void)")]
#[doc(alias = "__ZN3RBX9DataModel10serverSaveEv")]
pub fn stub_0x179e8() {
    // IDA 0x179e8 `RBX::DataModel::serverSave`: empty body — no-op.
}

// 0x179ec — __ZN3RBX9DataModel17internalSaveAsyncENS_9ContentIdEN5boost8functionIFvbEEE
// type: void()
#[doc(alias = "RBX::DataModel::internalSaveAsync(RBX::ContentId,boost::function<void ()(bool)>)")]
#[doc(alias = "__ZN3RBX9DataModel17internalSaveAsyncENS_9ContentIdEN5boost8functionIFvbEEE")]
pub fn stub_0x179ec() {
    // IDA 0x179ec `RBX::DataModel::internalSaveAsync`: empty body — no-op.
}

// 0x179f0 — __ZN3RBX9DataModel12internalSaveENS_9ContentIdE
// type: void()
#[doc(alias = "RBX::DataModel::internalSave(RBX::ContentId)")]
#[doc(alias = "__ZN3RBX9DataModel12internalSaveENS_9ContentIdE")]
pub fn stub_0x179f0() {
    // IDA 0x179f0 `RBX::DataModel::internalSave`: empty body — no-op.
}

// 0x179f4 — __ZN3RBX9DataModel11uploadPlaceERKSsNS_8Instance10SaveFilterEN5boost8functionIFvNS5_10shared_ptrIKNS_10Reflection5TupleEEEEEENS6_IFvSsEEE
// type: void __fastcall(int)
#[doc(alias = "RBX::DataModel::uploadPlace(std::string const&,RBX::Instance::SaveFilter,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")]
#[doc(alias = "__ZN3RBX9DataModel11uploadPlaceERKSsNS_8Instance10SaveFilterEN5boost8functionIFvNS5_10shared_ptrIKNS_10Reflection5TupleEEEEEENS6_IFvSsEEE")]
pub fn stub_0x179f4() {
    // IDA 0x179f4 `RBX::DataModel::uploadPlace`: empty Tuple
    // `shared_ptr` + aliasing `shared_ptr(a1)` (0x17a2a..0x17a32), both
    // released on return (0x17a64..0x17a6c). Net effect is nil;
    // `boost::shared_ptr` -> [`SharedPtr`] per AGENTS.md.
    let _keep = SharedPtr::new(Tuple);
    let _alias = SharedPtr::clone(&_keep);
}

// 0x17aac — __ZN5boost10shared_ptrIN3RBX10Reflection5TupleEEC1IS3_EEPT_
#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::Tuple>::shared_ptr<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10Reflection5TupleEEC1IS3_EEPT_")]
pub fn stub_0x17aac(shared: &SharedPtr<Tuple>) -> SharedPtr<Tuple> {
    // IDA 0x17aac `shared_ptr<Tuple>::shared_ptr(ptr, args)`: pointer store
    // (0x17ada) + `shared_count` attach (0x17b08) with old-count release
    // (0x17b10..0x17b1c). Host folds both into the [`SharedPtr`] clone.
    SharedPtr::clone(shared)
}

// 0x17b80 — __ZN5boost10shared_ptrIKN3RBX10Reflection5TupleEEC2IS3_EERKNS0_IT_EENS_6detail24sp_enable_if_convertibleIS7_S4_E4typeE
#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::Tuple const>::shared_ptr<RBX::Reflection::Tuple>(rbx_core::SharedPtr<RBX::Reflection::Tuple> const&,boost::detail::sp_enable_if_convertible<RBX::Reflection::Tuple,RBX::Reflection::Tuple const>::type)")]
#[doc(alias = "__ZN5boost10shared_ptrIKN3RBX10Reflection5TupleEEC2IS3_EERKNS0_IT_EENS_6detail24sp_enable_if_convertibleIS7_S4_E4typeE")]
pub fn stub_0x17b80(shared: &SharedPtr<Tuple>) -> SharedPtr<Tuple> {
    // IDA 0x17b80 `shared_ptr<Tuple const>::shared_ptr(copy)`: payload copy
    // (0x17ba8) + spinlock-guarded count bump (0x17bfe..0x17c14). Host folds
    // both into the [`SharedPtr`] clone.
    SharedPtr::clone(shared)
}

// 0x17c58 — __GLOBAL__I_a_0
#[doc(alias = "global constructor keyed to_a_0")]
#[doc(alias = "__GLOBAL__I_a_0")]
pub fn stub_0x17c58() {
    // IDA 0x17c58 `__GLOBAL__I_a`: static init storing
    // `boost::system::generic_category()` (x2) + `system_category()` into
    // merged globals (disasm 0x17c5c..0x17c76). Host categories need no
    // init (cf. 0x16e4c).
}

// 0x17df0 — +[Appirater setAppId:]
// type: void __cdecl(id, SEL, id)
#[doc(alias = "+[Appirater setAppId:]")]
pub fn stub_0x17df0(state: &mut Appirater, app_id: &str) {
    // IDA 0x17df0 `+[Appirater setAppId:]`: global store (0x17dfa).
    state.app_id = Some(app_id.to_owned());
}

// 0x17e00 — +[Appirater setDaysUntilPrompt:]
// type: void __cdecl(id, SEL, double)
#[doc(alias = "+[Appirater setDaysUntilPrompt:]")]
pub fn stub_0x17e00(state: &mut Appirater, days: f64) {
    // IDA 0x17e00 `+[Appirater setDaysUntilPrompt:]`: global store
    // (0x17e0e).
    state.days_until_prompt = days;
}

// 0x17e14 — +[Appirater setUsesUntilPrompt:]
// type: void __cdecl(id, SEL, int)
#[doc(alias = "+[Appirater setUsesUntilPrompt:]")]
pub fn stub_0x17e14(state: &mut Appirater, uses: i64) {
    // IDA 0x17e14 `+[Appirater setUsesUntilPrompt:]`: global store
    // (0x17e1e).
    state.uses_until_prompt = uses;
}

// 0x17e24 — +[Appirater setSignificantEventsUntilPrompt:]
// type: void __cdecl(id, SEL, int)
#[doc(alias = "+[Appirater setSignificantEventsUntilPrompt:]")]
pub fn stub_0x17e24(state: &mut Appirater, events: i64) {
    // IDA 0x17e24 `+[Appirater setSignificantEventsUntilPrompt:]`: global
    // store (0x17e2e).
    state.significant_events_until_prompt = events;
}

// 0x17e34 — +[Appirater setTimeBeforeReminding:]
// type: void __cdecl(id, SEL, double)
#[doc(alias = "+[Appirater setTimeBeforeReminding:]")]
pub fn stub_0x17e34(state: &mut Appirater, days: f64) {
    // IDA 0x17e34 `+[Appirater setTimeBeforeReminding:]`: global store
    // (0x17e42).
    state.time_before_reminding = days;
}

// 0x17e48 — +[Appirater setDebug:]
// type: void __cdecl(id, SEL, char)
#[doc(alias = "+[Appirater setDebug:]")]
pub fn stub_0x17e48(state: &mut Appirater, debug: bool) {
    // IDA 0x17e48 `+[Appirater setDebug:]`: global store (0x17e52).
    state.debug = debug;
}

// 0x17e58 — +[Appirater setDelegate:]
// type: void __cdecl(id, SEL, id)
#[doc(alias = "+[Appirater setDelegate:]")]
pub fn stub_0x17e58(state: &mut Appirater, delegate: Option<u32>) {
    // IDA 0x17e58 `+[Appirater setDelegate:]`: global slot store (0x17e62).
    state.delegate = delegate;
}

// 0x17e68 — -[Appirater connectedToNetwork]
// type: char __cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater connectedToNetwork]")]
pub fn stub_0x17e68(state: &Appirater) -> bool {
    // IDA 0x17e68 `-[Appirater connectedToNetwork]`:
    // `SCNetworkReachability` flags probe (0x17ea8..0x17eb8), then an
    // `NSURLConnection` probe to apple.com (0x17ede..0x17f3a); reachable
    // when `(flags & 6) == 2 || (flags & 1) != 0` and the connection is
    // non-null (0x17f4a..0x17f52), else 0 with an NSLog (0x17f60..0x17f64).
    // Host has no reachability API here; both fold into the latch.
    state.network_reachable
}

// 0x17f80 — +[Appirater sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[Appirater sharedInstance]")]
pub fn stub_0x17f80() -> SharedPtr<Mutex<Appirater>> {
    // IDA 0x17f80 `+[Appirater sharedInstance]`: nil-checked
    // `dispatch_once` alloc (0x17f92..0x17fdc). Host folds the once-guard
    // into [`LazyLock`].
    SharedPtr::clone(&APPIRATER_SHARED)
}

// 0x17fe4 — ___27+[Appirater sharedInstance]_block_invoke
#[doc(alias = "___27+[Appirater sharedInstance]_block_invoke")]
pub fn stub_0x17fe4(state: &mut Appirater, delegate: Option<u32>) {
    // IDA 0x17fe4 `__27+[Appirater sharedInstance]_block_invoke`: alloc+init
    // (0x18008..0x18030), `setDelegate:` (0x18036), `addObserver` for
    // `appWillResignActive` (0x18052..0x18092).
    *state = Appirater::new();
    state.delegate = delegate;
    state.observes_resign_active = true;
}

// 0x18094 — ___copy_helper_block_
#[doc(alias = "___copy_helper_block_")]
pub fn stub_0x18094(dst: &mut Option<SharedPtr<Mutex<Appirater>>>, src: &Option<SharedPtr<Mutex<Appirater>>>) {
    // IDA 0x18094 `__copy_helper_block_`: `_Block_object_assign` retain of
    // the captured self (0x1809a). Host folds the retain into the clone.
    *dst = src.clone();
}

// 0x180a0 — ___destroy_helper_block_
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_")]
pub fn stub_0x180a0(slot: &mut Option<SharedPtr<Mutex<Appirater>>>) {
    // IDA 0x180a0 `__destroy_helper_block_`: `_Block_object_dispose`
    // release of the captured self (0x180a4).
    *slot = None;
}

// 0x180a8 — -[Appirater showRatingAlert]
// type: void __cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater showRatingAlert]")]
pub fn stub_0x180a8(state: &mut Appirater) -> bool {
    // IDA 0x180a8 `-[Appirater showRatingAlert]`: localized `UIAlertView`
    // build from the main bundle (0x180d0..0x1812c+) gated on the delegate,
    // then shown. The view hierarchy lives on the platform side; the model
    // keeps the shown latch (delegate refusal = hidden).
    if state.delegate.is_some() || state.app_id.is_some() {
        state.rating_alert_visible = true;
        true
    } else {
        false
    }
}

// 0x183d8 — -[Appirater ratingConditionsHaveBeenMet]
// type: char __cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater ratingConditionsHaveBeenMet]")]
pub fn stub_0x183d8(state: &Appirater, now_epoch: f64) -> bool {
    // IDA 0x183d8 `-[Appirater ratingConditionsHaveBeenMet]`: `_debug`
    // short-circuits true (0x183ea..0x183f6); otherwise all of: days since
    // `kAppiraterFirstUseDate` >= `daysUntilPrompt` (0x1841a..0x184aa),
    // `kAppiraterUseCount` > `usesUntilPrompt` (0x184d0..0x184dc),
    // `kAppiraterSignificantEventCount` > threshold (0x184f2..0x184f6), not
    // `kAppiraterDeclinedToRate` (0x18516..0x18518), not
    // `kAppiraterRatedCurrentVersion` (0x18530..0x18532), days since
    // `kAppiraterReminderRequestDate` >= `timeBeforeReminding`
    // (0x18552..0x18594).
    if state.debug {
        return true;
    }
    if now_epoch - state.pref_float("kAppiraterFirstUseDate")
        < state.days_until_prompt * 86400.0
    {
        return false;
    }
    if state.pref_int("kAppiraterUseCount") <= state.uses_until_prompt {
        return false;
    }
    if state.pref_int("kAppiraterSignificantEventCount")
        <= state.significant_events_until_prompt
    {
        return false;
    }
    if state.pref_bool("kAppiraterDeclinedToRate") {
        return false;
    }
    if state.pref_bool("kAppiraterRatedCurrentVersion") {
        return false;
    }
    now_epoch - state.pref_float("kAppiraterReminderRequestDate")
        >= state.time_before_reminding * 86400.0
}

// 0x185b0 — -[Appirater incrementUseCount]
// type: void __cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater incrementUseCount]")]
pub fn stub_0x185b0(state: &mut Appirater, bundle_version: &str, now_epoch: f64) {
    // IDA 0x185b0 `-[Appirater incrementUseCount]`: stores
    // `kAppiraterCurrentVersion` when unset (0x18640..0x18662); when it
    // matches the bundle version (0x18694), stamps `kAppiraterFirstUseDate`
    // when unset (0x186b8..0x1870a) and bumps `kAppiraterUseCount`
    // (0x18730..0x18740); on a version change resets all four keys
    // (0x1877a..0x187fe+).
    if state.pref_str("kAppiraterCurrentVersion").is_none() {
        state.prefs.insert(
            "kAppiraterCurrentVersion".to_owned(),
            AppiraterPref::Str(bundle_version.to_owned()),
        );
    }
    if state.pref_str("kAppiraterCurrentVersion") == Some(bundle_version) {
        if state.pref_float("kAppiraterFirstUseDate") == 0.0 {
            state.prefs.insert(
                "kAppiraterFirstUseDate".to_owned(),
                AppiraterPref::Float(now_epoch),
            );
        }
        let next = state.pref_int("kAppiraterUseCount") + 1;
        state.prefs.insert("kAppiraterUseCount".to_owned(), AppiraterPref::Int(next));
    } else {
        state.prefs.insert(
            "kAppiraterCurrentVersion".to_owned(),
            AppiraterPref::Str(bundle_version.to_owned()),
        );
        state.prefs.insert(
            "kAppiraterFirstUseDate".to_owned(),
            AppiraterPref::Float(now_epoch),
        );
        state.prefs.insert("kAppiraterUseCount".to_owned(), AppiraterPref::Int(1));
        state.prefs.insert(
            "kAppiraterSignificantEventCount".to_owned(),
            AppiraterPref::Int(0),
        );
    }
}

// 0x18878 — -[Appirater incrementSignificantEventCount]
// type: void __cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater incrementSignificantEventCount]")]
pub fn stub_0x18878(state: &mut Appirater, bundle_version: &str, now_epoch: f64) {
    // IDA 0x18878 `-[Appirater incrementSignificantEventCount]`: same
    // version-gate shape as `incrementUseCount` (0x1889a..0x18990), but bumps
    // `kAppiraterSignificantEventCount` (0x189f8+).
    if state.pref_str("kAppiraterCurrentVersion").is_none() {
        state.prefs.insert(
            "kAppiraterCurrentVersion".to_owned(),
            AppiraterPref::Str(bundle_version.to_owned()),
        );
    }
    if state.pref_str("kAppiraterCurrentVersion") == Some(bundle_version) {
        if state.pref_float("kAppiraterFirstUseDate") == 0.0 {
            state.prefs.insert(
                "kAppiraterFirstUseDate".to_owned(),
                AppiraterPref::Float(now_epoch),
            );
        }
        let next = state.pref_int("kAppiraterSignificantEventCount") + 1;
        state.prefs.insert(
            "kAppiraterSignificantEventCount".to_owned(),
            AppiraterPref::Int(next),
        );
    }
}

// 0x18b18 — -[Appirater incrementAndRate:]
// type: void __cdecl(Appirater *self, SEL, char)
#[doc(alias = "-[Appirater incrementAndRate:]")]
pub fn stub_0x18b18(state: &mut Appirater, bundle_version: &str, now_epoch: f64, can_rate: bool) {
    // IDA 0x18b18 `-[Appirater incrementAndRate:]`: `incrementUseCount`
    // (0x18b30); when `canRate` and conditions hold (0x18b48) and the
    // network is up (0x18b60), the alert block runs on the main queue
    // (0x18b98..0x18baa -> 0x18bb4). The queue hop is synchronous here.
    stub_0x185b0(state, bundle_version, now_epoch);
    if can_rate && stub_0x183d8(state, now_epoch) && stub_0x17e68(state) {
        stub_0x18bb4(state);
    }
}

// 0x18bb4 — ___30-[Appirater incrementAndRate:]_block_invoke
#[doc(alias = "___30-[Appirater incrementAndRate:]_block_invoke")]
pub fn stub_0x18bb4(state: &mut Appirater) -> bool {
    // IDA 0x18bb4 `__30-[Appirater incrementAndRate:]_block_invoke`:
    // `showRatingAlert` shim (single `objc_msgSend`).
    stub_0x180a8(state)
}

// 0x18bc8 — ___copy_helper_block_125
#[doc(alias = "___copy_helper_block_125")]
pub fn stub_0x18bc8(
    dst: &mut Option<SharedPtr<Mutex<Appirater>>>,
    src: &Option<SharedPtr<Mutex<Appirater>>>,
) {
    // IDA 0x18bc8 `__copy_helper_block_125`: `_Block_object_assign` retain
    // of the captured self (0x18bce; cf. 0x18094).
    *dst = src.clone();
}

// 0x18bd4 — ___destroy_helper_block_126
#[doc(alias = "___destroy_helper_block_126")]
pub fn stub_0x18bd4(slot: &mut Option<SharedPtr<Mutex<Appirater>>>) {
    // IDA 0x18bd4 `__destroy_helper_block_126`: `_Block_object_dispose`
    // release of the captured self (0x18bd8; cf. 0x180a0).
    *slot = None;
}

// 0x18bdc — -[Appirater incrementSignificantEventAndRate:]
// type: void __cdecl(Appirater *self, SEL, char)
#[doc(alias = "-[Appirater incrementSignificantEventAndRate:]")]
pub fn stub_0x18bdc(state: &mut Appirater, bundle_version: &str, now_epoch: f64, can_rate: bool) {
    // IDA 0x18bdc `-[Appirater incrementSignificantEventAndRate:]`:
    // `incrementSignificantEventCount` (0x18bf4); when `canRate` and
    // conditions hold (0x18c0c) and the network is up (0x18c24), the alert
    // block runs on the main queue (0x18c5c..0x18c6e -> 0x18c78). The queue
    // hop is synchronous here.
    stub_0x18878(state, bundle_version, now_epoch);
    if can_rate && stub_0x183d8(state, now_epoch) && stub_0x17e68(state) {
        stub_0x18c78(state);
    }
}

// 0x18c78 — ___46-[Appirater incrementSignificantEventAndRate:]_block_invoke
#[doc(alias = "___46-[Appirater incrementSignificantEventAndRate:]_block_invoke")]
pub fn stub_0x18c78(state: &mut Appirater) -> bool {
    // IDA 0x18c78 `__46-[Appirater incrementSignificantEventAndRate:]_
    // block_invoke`: `showRatingAlert` shim (single `objc_msgSend`; cf.
    // 0x18bb4).
    stub_0x180a8(state)
}

// 0x18c8c — ___copy_helper_block_130
#[doc(alias = "___copy_helper_block_130")]
pub fn stub_0x18c8c(
    dst: &mut Option<SharedPtr<Mutex<Appirater>>>,
    src: &Option<SharedPtr<Mutex<Appirater>>>,
) {
    // IDA 0x18c8c `__copy_helper_block_130`: `_Block_object_assign` retain
    // of the captured self (0x18c92; cf. 0x18094).
    *dst = src.clone();
}

// 0x18c98 — ___destroy_helper_block_131
#[doc(alias = "___destroy_helper_block_131")]
pub fn stub_0x18c98(slot: &mut Option<SharedPtr<Mutex<Appirater>>>) {
    // IDA 0x18c98 `__destroy_helper_block_131`: `_Block_object_dispose`
    // release of the captured self (0x18c9c; cf. 0x180a0).
    *slot = None;
}

// 0x18ca0 — +[Appirater appLaunched]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[Appirater appLaunched]")]
pub fn stub_0x18ca0(state: &mut Appirater, bundle_version: &str, now_epoch: f64) {
    // IDA 0x18ca0 `+[Appirater appLaunched]`: forwards to `appLaunched:`
    // with 1 (0x18cba).
    stub_0x18cc0(state, bundle_version, now_epoch, true);
}

// 0x18cc0 — +[Appirater appLaunched:]
// type: void __cdecl(id, SEL, char)
#[doc(alias = "+[Appirater appLaunched:]")]
pub fn stub_0x18cc0(state: &mut Appirater, bundle_version: &str, now_epoch: f64, can_rate: bool) {
    // IDA 0x18cc0 `+[Appirater appLaunched:]`: builds the
    // `__25_appLaunched_block_invoke` block capturing `canRate` (0x18cf2..
    // 0x18d04) and runs it on a global queue (0x18cd0..0x18d08 -> 0x18d10).
    // The queue hop is synchronous here.
    stub_0x18d10(state, bundle_version, now_epoch, can_rate);
}

// 0x18d10 — ___25+[Appirater appLaunched:]_block_invoke
#[doc(alias = "___25+[Appirater appLaunched:]_block_invoke")]
pub fn stub_0x18d10(state: &mut Appirater, bundle_version: &str, now_epoch: f64, can_rate: bool) {
    // IDA 0x18d10 `__25+[Appirater appLaunched:]_block_invoke`:
    // `sharedInstance` (0x18d2e) + `incrementAndRate:` with the captured
    // flag. The shared slot already exists; the call lands on `state`.
    stub_0x18b18(state, bundle_version, now_epoch, can_rate);
}

// 0x18d4c — -[Appirater hideRatingAlert]
// type: void __cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater hideRatingAlert]")]
pub fn stub_0x18d4c(state: &mut Appirater) {
    // IDA 0x18d4c `-[Appirater hideRatingAlert]`: when the alert `isVisible`
    // (0x18d62..0x18d72), `dismissWithClickedButtonIndex:-1` (0x18d9e..
    // 0x18db8).
    if state.rating_alert_visible {
        state.rating_alert_visible = false;
    }
}

// 0x18dbc — +[Appirater appWillResignActive]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[Appirater appWillResignActive]")]
pub fn stub_0x18dbc(state: &mut Appirater) {
    // IDA 0x18dbc `+[Appirater appWillResignActive]`: `sharedInstance`
    // (0x18df4) + `hideRatingAlert` (0x18e08).
    stub_0x18d4c(state);
}

// 0x18e0c — +[Appirater appEnteredForeground:]
// type: void __cdecl(id, SEL, char)
#[doc(alias = "+[Appirater appEnteredForeground:]")]
pub fn stub_0x18e0c(state: &mut Appirater, bundle_version: &str, now_epoch: f64, can_rate: bool) {
    // IDA 0x18e0c `+[Appirater appEnteredForeground:]`: builds the
    // `__34_appEnteredForeground_block_invoke` block (0x18e3e..0x18e50) and
    // runs it on a global queue (0x18e1c..0x18e54 -> 0x18e5c). Synchronous
    // here.
    stub_0x18e5c(state, bundle_version, now_epoch, can_rate);
}

// 0x18e5c — ___34+[Appirater appEnteredForeground:]_block_invoke
#[doc(alias = "___34+[Appirater appEnteredForeground:]_block_invoke")]
pub fn stub_0x18e5c(state: &mut Appirater, bundle_version: &str, now_epoch: f64, can_rate: bool) {
    // IDA 0x18e5c `__34+[Appirater appEnteredForeground:]_block_invoke`:
    // `sharedInstance` (0x18e7a) + `incrementAndRate:` (cf. 0x18d10).
    stub_0x18b18(state, bundle_version, now_epoch, can_rate);
}

// 0x18e98 — +[Appirater userDidSignificantEvent:]
// type: void __cdecl(id, SEL, char)
#[doc(alias = "+[Appirater userDidSignificantEvent:]")]
pub fn stub_0x18e98(state: &mut Appirater, bundle_version: &str, now_epoch: f64, can_rate: bool) {
    // IDA 0x18e98 `+[Appirater userDidSignificantEvent:]`: builds the
    // `__37_userDidSignificantEvent_block_invoke` block (0x18eca..0x18edc)
    // and runs it on a global queue (0x18ea8..0x18ee0 -> 0x18ee8).
    // Synchronous here.
    stub_0x18ee8(state, bundle_version, now_epoch, can_rate);
}

// 0x18ee8 — ___37+[Appirater userDidSignificantEvent:]_block_invoke
#[doc(alias = "___37+[Appirater userDidSignificantEvent:]_block_invoke")]
pub fn stub_0x18ee8(state: &mut Appirater, bundle_version: &str, now_epoch: f64, can_rate: bool) {
    // IDA 0x18ee8 `__37+[Appirater userDidSignificantEvent:]_block_invoke`:
    // `sharedInstance` (0x18f06) + `incrementSignificantEventAndRate:`.
    stub_0x18bdc(state, bundle_version, now_epoch, can_rate);
}

// 0x18f24 — +[Appirater rateApp]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[Appirater rateApp]")]
pub fn stub_0x18f24(state: &mut Appirater) {
    // IDA 0x18f24 `+[Appirater rateApp]`: formats `templateReviewURL` with
    // the app id (`APP_ID` substitution, 0x18f6e..0x18fa2), sets
    // `kAppiraterRatedCurrentVersion` (0x18fbe), synchronizes (0x18fd0),
    // and opens the URL (0x18ff0..0x19024). The template text and the open
    // live on the platform side; the rated latch is modeled here.
    state.prefs.insert(
        "kAppiraterRatedCurrentVersion".to_owned(),
        AppiraterPref::Bool(true),
    );
}

// 0x19028 — -[Appirater alertView:clickedButtonAtIndex:]
// type: void __cdecl(Appirater *self, SEL, id, int)
#[doc(alias = "-[Appirater alertView:clickedButtonAtIndex:]")]
pub fn stub_0x19028(state: &mut Appirater, button: i32, now_epoch: f64) {
    // IDA 0x19028 `-[Appirater alertView:clickedButtonAtIndex:]`: button 2
    // stamps `kAppiraterReminderRequestDate` (0x190c4..0x190f6) + delegate
    // `appiraterDidOptToRemindLater` (0x19122..0x19140); button 1 runs
    // `rateApp` (0x19070) + delegate `appiraterDidOptToRate`
    // (0x1908a..0x190aa); button 0 sets `kAppiraterDeclinedToRate` +
    // delegate `appiraterDidDeclineToRate` (classic Appirater tail).
    match button {
        2 => {
            state.prefs.insert(
                "kAppiraterReminderRequestDate".to_owned(),
                AppiraterPref::Float(now_epoch),
            );
            if state.delegate.is_some() {
                state.delegate_events.push(AppiraterDelegateEvent::OptToRemindLater);
            }
        }
        1 => {
            stub_0x18f24(state);
            if state.delegate.is_some() {
                state.delegate_events.push(AppiraterDelegateEvent::OptToRate);
            }
        }
        _ => {
            state.prefs.insert(
                "kAppiraterDeclinedToRate".to_owned(),
                AppiraterPref::Bool(true),
            );
            if state.delegate.is_some() {
                state.delegate_events.push(AppiraterDelegateEvent::OptToDecline);
            }
        }
    }
}

// 0x191d4 — -[Appirater ratingAlert]
// type: UIAlertView *__cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater ratingAlert]")]
pub fn stub_0x191d4(state: &Appirater) -> Option<u32> {
    // IDA 0x191d4 `-[Appirater ratingAlert]`: `self->ratingAlert` load
    // (0x191e2).
    state.rating_alert
}

// 0x191e4 — -[Appirater setRatingAlert:]
// type: void __cdecl(Appirater *self, SEL, id)
#[doc(alias = "-[Appirater setRatingAlert:]")]
pub fn stub_0x191e4(state: &mut Appirater, alert: Option<u32>) {
    // IDA 0x191e4 `-[Appirater setRatingAlert:]`: `objc_setProperty` retain
    // into slot +4 (0x19200).
    state.rating_alert = alert;
}

// 0x19208 — -[Appirater delegate]
// type: AppiraterDelegate *__cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater delegate]")]
pub fn stub_0x19208(state: &Appirater) -> Option<u32> {
    // IDA 0x19208 `-[Appirater delegate]`: `_delegate` load (0x19216).
    state.delegate
}

// 0x19218 — -[Appirater setDelegate:]
// type: void __cdecl(Appirater *self, SEL, id)
#[doc(alias = "-[Appirater setDelegate:]")]
pub fn stub_0x19218(state: &mut Appirater, delegate: Option<u32>) {
    // IDA 0x19218 `-[Appirater setDelegate:]`: `_delegate` store (0x19224).
    state.delegate = delegate;
}

// 0x19228 — -[AppDelegate init]
// type: AppDelegate *__cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate init]")]
pub fn stub_0x19228() -> AppDelegate {
    // IDA 0x19228 `-[AppDelegate init]`: super-init only (0x19242..0x19252).
    AppDelegate::default()
}

// 0x19254 — -[AppDelegate dealloc]
// type: void __cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate dealloc]")]
pub fn stub_0x19254(_app: AppDelegate) {
    // IDA 0x19254 `-[AppDelegate dealloc]`: analytics release + window
    // release (0x19276..0x1928a), then super-dealloc (0x192a2..0x192ac).
    // Consuming `app` models the releases; Drop glue covers the rest.
}

// 0x192b4 — -[AppDelegate application:didFinishLaunchingWithOptions:]
// type: char __cdecl(AppDelegate *self, SEL, id, id)
#[doc(alias = "-[AppDelegate application:didFinishLaunchingWithOptions:]")]
pub fn stub_0x192b4(app: &mut AppDelegate, state: &mut Appirater, bundle_version: &str, now_epoch: f64) {
    // IDA 0x192b4 `-[AppDelegate application:didFinishLaunchingWithOptions:]`:
    // defaults registration + reporter/flurry bootstrap on background
    // queues; the Appirater leg is the `appLaunched:` block (0x19514),
    // applied here synchronously.
    app.launched = true;
    stub_0x19514(state, bundle_version, now_epoch);
}

// 0x194ec — ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke")]
pub fn stub_0x194ec(app: &mut AppDelegate) {
    // IDA 0x194ec `__57_didFinishLaunching_block_invoke`: `+[Flurry
    // startSession:]` with the app key (0x1950e).
    app.flurry_session_key = Some("FM7DNRW56339NC22K8GR".to_owned());
}

// 0x19514 — ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2
// type: void __cdecl(id)
#[doc(alias = "___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2")]
pub fn stub_0x19514(state: &mut Appirater, _bundle_version: &str, now_epoch: f64) {
    // IDA 0x19514 `__57_didFinishLaunching_block_invoke_2`: the launch-time
    // Appirater config — `setAppId:@"431946152"` (0x1953a),
    // `setDaysUntilPrompt:3.0` (0x19554), `setUsesUntilPrompt:10` (0x19568),
    // `setTimeBeforeReminding:10.0` (0x19582), `appLaunched:1` (0x1959a).
    // `_bundle_version` is unused: the original keys off the running bundle,
    // which the platform side supplies; `now_epoch` stands in for `NSDate`.
    stub_0x17df0(state, "431946152");
    stub_0x17e00(state, 3.0);
    stub_0x17e14(state, 10);
    stub_0x17e34(state, 10.0);
    stub_0x18cc0(state, _bundle_version, now_epoch, true);
}

// 0x195a0 — -[AppDelegate applicationWillResignActive:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationWillResignActive:]")]
pub fn stub_0x195a0(app: &mut AppDelegate) {
    // IDA 0x195a0 `-[AppDelegate applicationWillResignActive:]`: log +
    // `PlaceLauncher disableViewBecauseGoingToBackground` (0x195be..0x19640;
    // view work lives on the platform side); the latch is modeled here.
    app.resigned_active = true;
}

// 0x196e4 — -[AppDelegate applicationDidEnterBackground:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationDidEnterBackground:]")]
pub fn stub_0x196e4(app: &mut AppDelegate) {
    // IDA 0x196e4 `-[AppDelegate applicationDidEnterBackground:]`:
    // `NSUserDefaults` writes + session reporting (0x1971c+); view/service
    // work lives on the platform side; the latch is modeled here.
    app.entered_background = true;
}

// 0x19a30 — -[AppDelegate applicationDidReceiveMemoryWarning:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationDidReceiveMemoryWarning:]")]
pub fn stub_0x19a30(app: &mut AppDelegate) {
    // IDA 0x19a30 `-[AppDelegate applicationDidReceiveMemoryWarning:]`: OOM
    // log + `stopMemoryBouncer` + `PlaceLauncher` purge (0x19a4e..0x19aee;
    // service work lives on the platform side); the latch is modeled here.
    app.received_memory_warning = true;
}

// 0x30b1c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x30b1c() {
    // IDA 0x30b1c: functor_manager thunk; closure buffer ops fold into Box<dyn Fn> — carrier no-op.
}

// 0x30b38 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEvE6invokeERNS1_15function_bufferE")]
pub fn stub_0x30b38() {
    // IDA 0x30b38: invoker thunk; dispatches a stored closure — carrier no-op.
}

// 0x30b40 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIPKcEENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIPKcEENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_0x30b40() {
    // IDA 0x30b40: vtable assign thunk; closure buffer ops fold into Box<dyn Fn> — carrier no-op.
}

// 0x30c28 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIPKcEENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIPKcEENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_0x30c28() {
    // IDA 0x30c28: vtable assign thunk; closure buffer ops fold into Box<dyn Fn> — carrier no-op.
}

// 0x30d3c — __ZN5boost3_bi5list2INS0_5valueIPKcEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFvRKSsS9_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: int(void)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::operator()<void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>) &,boost::_bi::list0 &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueIPKcEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFvRKSsS9_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x30d3c() {
    // IDA 0x30d3c: bind argument-list dispatch; captures fold into closures — carrier no-op.
}

// 0x30eac — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_0x30eac() {
    // IDA 0x30eac: functor_manager thunk; closure buffer ops fold into Box<dyn Fn> — carrier no-op.
}

// 0x30fe0 — __ZN5boost3_bi5list2INS0_5valueIPKcEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES5_SA_
#[doc(alias = "boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::list2(boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueIPKcEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES5_SA_")]
pub fn stub_0x30fe0() {
    // IDA 0x30fe0: bind argument-list plumbing; captures fold into closures — carrier no-op.
}

// 0x310a8 — __ZN5boost6detail12shared_countC2IN3RBX16SecurePlayerGameEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::SecurePlayerGame>(RBX::SecurePlayerGame *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX16SecurePlayerGameEEEPT_")]
pub fn stub_0x310a8() {
    // IDA 0x310a8: shared_count ctor thunk; refcount owned by SharedPtr (Arc) — carrier no-op.
}

// 0x3119c — __ZN5boost6detail17sp_counted_impl_pIN3RBX16SecurePlayerGameEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX16SecurePlayerGameEED1Ev")]
pub fn stub_0x3119c() {
    // IDA 0x3119c: sp_counted dtor thunk; refcount owned by SharedPtr (Arc) — carrier no-op.
}

// 0x311a0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX16SecurePlayerGameEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::~sp_counted_impl_p() [0x311a0]")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX16SecurePlayerGameEED0Ev")]
pub fn stub_0x311a0() {
    // IDA 0x311a0: sp_counted dtor thunk; refcount owned by SharedPtr (Arc) — carrier no-op.
}

// 0x311a4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX16SecurePlayerGameEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX16SecurePlayerGameEE7disposeEv")]
pub fn stub_0x311a4() {
    // IDA 0x311a4: sp_counted dispose thunk; refcount owned by SharedPtr (Arc) — carrier no-op.
}

// 0x311b4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX16SecurePlayerGameEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX16SecurePlayerGameEE11get_deleterERKSt9type_info")]
pub fn stub_0x311b4() {
    // IDA 0x311b4: sp_counted get_deleter thunk; refcount owned by SharedPtr (Arc) — carrier no-op.
}

// 0x311b8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX16SecurePlayerGameEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX16SecurePlayerGameEE19get_untyped_deleterEv")]
pub fn stub_0x311b8() {
    // IDA 0x311b8: sp_counted get_untyped_deleter thunk; refcount owned by SharedPtr (Arc) — carrier no-op.
}

// 0x311bc — __ZN5boost6detail12shared_countC2IN3RBX19UnsecuredStudioGameEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::UnsecuredStudioGame>(RBX::UnsecuredStudioGame *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX19UnsecuredStudioGameEEEPT_")]
pub fn stub_0x311bc() {
    // IDA 0x311bc: shared_count ctor thunk; refcount owned by SharedPtr (Arc) — carrier no-op.
}

// 0x312b0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX19UnsecuredStudioGameEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX19UnsecuredStudioGameEED1Ev")]
pub fn stub_0x312b0() {
    // IDA 0x312b0: sp_counted dtor thunk; refcount owned by SharedPtr (Arc) — carrier no-op.
}

// 0x312b4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX19UnsecuredStudioGameEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::~sp_counted_impl_p() [0x312b4]")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX19UnsecuredStudioGameEED0Ev")]
pub fn stub_0x312b4() {
    // IDA 0x312b4: sp_counted dtor thunk; refcount owned by SharedPtr (Arc) — carrier no-op.
}

// 0x312b8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX19UnsecuredStudioGameEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX19UnsecuredStudioGameEE7disposeEv")]
pub fn stub_0x312b8() {
    // IDA 0x312b8: sp_counted dispose thunk; refcount owned by SharedPtr (Arc) — carrier no-op.
}

// 0x312c8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX19UnsecuredStudioGameEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX19UnsecuredStudioGameEE11get_deleterERKSt9type_info")]
pub fn stub_0x312c8() {
    // IDA 0x312c8: sp_counted get_deleter thunk; refcount owned by SharedPtr (Arc) — carrier no-op.
}

// 0x312cc — __ZN5boost6detail17sp_counted_impl_pIN3RBX19UnsecuredStudioGameEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::UnsecuredStudioGame>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX19UnsecuredStudioGameEE19get_untyped_deleterEv")]
pub fn stub_0x312cc() {
    // IDA 0x312cc: sp_counted get_untyped_deleter thunk; refcount owned by SharedPtr (Arc) — carrier no-op.
}

// 0x312d0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewaPN3RBX18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSD_IaEENSD_IS9_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,signed char,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<signed char>,boost::_bi::value<RBX::FunctionMarshaller *>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewaPN3RBX18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSD_IaEENSD_IS9_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x312d0() {
    // IDA 0x312d0: functor_manager thunk; closure buffer ops fold into Box<dyn Fn> — carrier no-op.
}

// 0x31348 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP10RobloxViewaPN3RBX18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSD_IaEENSD_IS9_EEEEEEvPNS7_9DataModelEE6invokeERNS1_15function_bufferESK_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RobloxView *,signed char,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<signed char>,boost::_bi::value<RBX::FunctionMarshaller *>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP10RobloxViewaPN3RBX18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSD_IaEENSD_IS9_EEEEEEvPNS7_9DataModelEE6invokeERNS1_15function_bufferESK_")]
pub fn stub_0x31348() {
    // IDA 0x31348: invoker thunk; dispatches a stored closure — carrier no-op.
}

// 0x31358 — __ZNK3RBX15ServiceProvider6createINS_12LoginServiceEEEPT_v
// type: int __fastcall(pthread_mutex_t *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::LoginService * RBX::ServiceProvider::create<RBX::LoginService>(void)const")]
#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_12LoginServiceEEEPT_v")]
pub fn stub_0x31358() -> ! {
    todo!("0x31358 RBX::LoginService * RBX::ServiceProvider::create<RBX::LoginService>(void)const")
}

// 0x3151c — __ZNK3RBX15ServiceProvider4findINS_12LoginServiceEEEPT_v
// type: int __fastcall(pthread_mutex_t *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::LoginService * RBX::ServiceProvider::find<RBX::LoginService>(void)const")]
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_12LoginServiceEEEPT_v")]
pub fn stub_0x3151c() -> ! {
    todo!("0x3151c RBX::LoginService * RBX::ServiceProvider::find<RBX::LoginService>(void)const")
}

// 0x31678 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12LoginServiceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::LoginService> RBX::Creatable<RBX::Instance>::create<RBX::LoginService>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_12LoginServiceEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_0x31678() -> ! {
    todo!("0x31678 rbx_core::SharedPtr<RBX::LoginService> RBX::Creatable<RBX::Instance>::create<RBX::LoginService>(void)")
}

// 0x31728 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_12LoginServiceEEERS3_RKNS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::LoginService>(rbx_core::SharedPtr<RBX::LoginService> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_12LoginServiceEEERS3_RKNS0_IT_EE")]
pub fn stub_0x31728() -> ! {
    todo!("0x31728 rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::LoginService>(rbx_core::SharedPtr<RBX::LoginService> const&)")
}

// 0x317e4 — __ZN3RBX4Name7declareILZNS_13sLoginServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_13sLoginServiceEEEERKS0_v")]
pub fn stub_0x317e4() -> ! {
    todo!("0x317e4 __ZN3RBX4Name7declareILZNS_13sLoginServiceEEEERKS0_v")
}

// 0x31828 — __ZN3RBX4Name13callDoDeclareILZNS_13sLoginServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sLoginServiceEEEEvv")]
pub fn stub_0x31828() -> ! {
    todo!("0x31828 __ZN3RBX4Name13callDoDeclareILZNS_13sLoginServiceEEEEvv")
}

// 0x3182c — __ZN3RBX4Name9doDeclareILZNS_13sLoginServiceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sLoginServiceEEEERKS0_v")]
pub fn stub_0x3182c() -> ! {
    todo!("0x3182c __ZN3RBX4Name9doDeclareILZNS_13sLoginServiceEEEERKS0_v")
}

// 0x31910 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_12LoginServiceEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::LoginService>(void)")]
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_12LoginServiceEEEvv")]
pub fn stub_0x31910() -> ! {
    todo!("0x31910 void RBX::ServiceProvider::callDoGetClassIndex<RBX::LoginService>(void)")
}

// 0x31914 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_12LoginServiceEEEmv
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::LoginService>(void)")]
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_12LoginServiceEEEmv")]
pub fn stub_0x31914() -> ! {
    todo!("0x31914 unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::LoginService>(void)")
}

// 0x319ec — __ZN5boost10shared_ptrIN3RBX12LoginServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::LoginService>::shared_ptr<RBX::LoginService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX12LoginServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0x319ec() -> ! {
    todo!("0x319ec rbx_core::SharedPtr<RBX::LoginService>::shared_ptr<RBX::LoginService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x31a10 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12LoginServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LoginService,RBX::LoginService>(rbx_core::SharedPtr<RBX::LoginService> const*,RBX::LoginService *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12LoginServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x31a10() -> ! {
    todo!("0x31a10 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LoginService,RBX::LoginService>(rbx_core::SharedPtr<RBX::LoginService> const*,RBX::LoginService *)const")
}

// 0x31aec — __ZN5boost6detail12shared_countC2IPN3RBX12LoginServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX12LoginServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0x31aec() -> ! {
    todo!("0x31aec boost::detail::shared_count::shared_count<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x31bec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0x31bec() -> ! {
    todo!("0x31bec boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

#[cfg(test)]
mod bind_carrier_tests {
    use super::*;

    #[test]
    fn boost_carriers_are_noops() {
        stub_0x30b1c();
        stub_0x30b38();
        stub_0x30b40();
        stub_0x30c28();
        stub_0x30d3c();
        stub_0x30eac();
        stub_0x30fe0();
        stub_0x310a8();
        stub_0x3119c();
        stub_0x311a0();
        stub_0x311a4();
        stub_0x311b4();
        stub_0x311b8();
        stub_0x311bc();
        stub_0x312b0();
        stub_0x312b4();
        stub_0x312b8();
        stub_0x312c8();
        stub_0x312cc();
        stub_0x312d0();
        stub_0x31348();
    }
}
