// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: global gap filler EA-sorted asc next 100 not yet in datamodel (datamodel 18726/85545, global 0 missing)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0x1654c..0x1b14c | total filtered 10215, remaining 0 after batch; local 18726->18826 distinct, 66819->66719 not in datamodel (0 global missing)
// Shard: bg_1 EA-sorted asc next 100 low-EA global gap filler after 0x1654b not yet in datamodel (filtered exhausted, 66819 missing before -> 66719 after)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use crate::generated_189::{CRenderSettingsItem, RenderEnumDesc, resolution_preset_enum_desc};
use crate::generated_191::{
    aa_samples_enum_desc, antialiasing_mode_enum_desc, frame_rate_manager_mode_enum_desc,
    quality_level_enum_desc, shadow_mode_enum_desc,
};
use crate::generated_next_b::graphics_mode_enum_desc;
use std::collections::BTreeMap;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x1654c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings10ShadowModeEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode> const>::doGetSingleton(void)")]
pub fn stub_1654c() -> &'static RenderEnumDesc {
    // IDA 0x1654c (decompiled): `Singleton<EnumDesc<ShadowMode>>::doGetSingleton` —
    // `__cxa_guard_acquire` once-init around the function-local `EnumDesc<ShadowMode> s`
    // (0x165a8), in-place `EnumDesc` ctor (0x165c2), `__cxa_atexit` dtor registration
    // (0x165e0), return `&s` (0x16610). The guard collapses into the table
    // singleton; same object as `shadow_mode_enum_desc()` (built by 0x8c4c).
    // Same shape as the 0x16548 `initSingleton` touch in generated_next_b.
    shadow_mode_enum_desc()
}

// 0x1663c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16ResolutionPresetEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset> const>::initSingleton(void)")]
pub fn stub_1663c() {
    // IDA 0x1663c (decompiled, thunk): `Singleton<EnumDesc<ResolutionPreset>>::initSingleton`
    // tail-branches to `doGetSingleton` (0x16640, disasm `B.W`). The once-init
    // collapses into the table singleton touch; same treatment as stub_16548.
    let _ = stub_16640();
}

// 0x16640 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16ResolutionPresetEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset> const>::doGetSingleton(void)")]
pub fn stub_16640() -> &'static RenderEnumDesc {
    // IDA 0x16640 (decompiled): `Singleton<EnumDesc<ResolutionPreset>>::doGetSingleton` —
    // guard once-init (0x1669c), in-place `EnumDesc` ctor (0x166b6), `__cxa_atexit`
    // (0x166d4), return `&s` (0x16704). Collapses into the table singleton;
    // same object as `resolution_preset_enum_desc()`. Same shape as 0x1654c.
    resolution_preset_enum_desc()
}

// 0x16730 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12QualityLevelEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel> const>::initSingleton(void)")]
pub fn stub_16730() {
    // IDA 0x16730 (decompiled, thunk): `Singleton<EnumDesc<QualityLevel>>::initSingleton`
    // tail-branches to `doGetSingleton` (0x16734). Same touch-collapse as stub_16548.
    let _ = stub_16734();
}

// 0x16734 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12QualityLevelEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel> const>::doGetSingleton(void)")]
pub fn stub_16734() -> &'static RenderEnumDesc {
    // IDA 0x16734 (decompiled): `Singleton<EnumDesc<QualityLevel>>::doGetSingleton` —
    // guard once-init (0x16790), in-place `EnumDesc` ctor (0x167aa), `__cxa_atexit`
    // (0x167c8), return `&s` (0x167f8). Same object as `quality_level_enum_desc()`.
    // Same shape as 0x1654c.
    quality_level_enum_desc()
}

// 0x16824 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16AntialiasingModeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode> const>::initSingleton(void)")]
pub fn stub_16824() {
    // IDA 0x16824 (decompiled, thunk): `Singleton<EnumDesc<AntialiasingMode>>::initSingleton`
    // tail-branches to `doGetSingleton` (0x16828). Same touch-collapse as stub_16548.
    let _ = stub_16828();
}

// 0x16828 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16AntialiasingModeEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode> const>::doGetSingleton(void)")]
pub fn stub_16828() -> &'static RenderEnumDesc {
    // IDA 0x16828 (decompiled): `Singleton<EnumDesc<AntialiasingMode>>::doGetSingleton` —
    // guard once-init (0x16884), in-place `EnumDesc` ctor (0x1689e), `__cxa_atexit`
    // (0x168bc), return `&s` (0x168ec). Same object as `antialiasing_mode_enum_desc()`.
    // Same shape as 0x1654c.
    antialiasing_mode_enum_desc()
}

// 0x16918 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode> const>::initSingleton(void)")]
pub fn stub_16918() {
    // IDA 0x16918 (decompiled, thunk): `Singleton<EnumDesc<FrameRateManagerMode>>::initSingleton`
    // tail-branches to `doGetSingleton` (0x1691c). Same touch-collapse as stub_16548.
    let _ = stub_1691c();
}

// 0x1691c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode> const>::doGetSingleton(void)")]
pub fn stub_1691c() -> &'static RenderEnumDesc {
    // IDA 0x1691c (decompiled): `Singleton<EnumDesc<FrameRateManagerMode>>::doGetSingleton` —
    // guard once-init (0x16978), in-place `EnumDesc` ctor (0x16992), `__cxa_atexit`
    // (0x169b0), return `&s` (0x169e0). Same object as `frame_rate_manager_mode_enum_desc()`.
    // Same shape as 0x1654c.
    frame_rate_manager_mode_enum_desc()
}

// 0x16a0c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12GraphicsModeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode> const>::initSingleton(void)")]
pub fn stub_16a0c() {
    // IDA 0x16a0c (decompiled, thunk): `Singleton<EnumDesc<GraphicsMode>>::initSingleton`
    // tail-branches to `doGetSingleton` (0x16a10). Same touch-collapse as stub_16548.
    let _ = stub_16a10();
}

// 0x16a10 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12GraphicsModeEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode> const>::doGetSingleton(void)")]
pub fn stub_16a10() -> &'static RenderEnumDesc {
    // IDA 0x16a10 (decompiled): `Singleton<EnumDesc<GraphicsMode>>::doGetSingleton` —
    // guard once-init (0x16a6c), in-place `EnumDesc` ctor (0x16a86), `__cxa_atexit`
    // (0x16aa4), return `&s` (0x16ad4). Same object as `graphics_mode_enum_desc()`
    // (generated_next_b, built by 0x86d0). Same shape as 0x1654c.
    graphics_mode_enum_desc()
}

// 0x16b00 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings9AASamplesEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples> const>::initSingleton(void)")]
pub fn stub_16b00() {
    // IDA 0x16b00 (decompiled, thunk): `Singleton<EnumDesc<AASamples>>::initSingleton`
    // tail-branches to `doGetSingleton` (0x16b04). Same touch-collapse as stub_16548.
    let _ = stub_16b04();
}

// 0x16b04 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings9AASamplesEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples> const>::doGetSingleton(void)")]
pub fn stub_16b04() -> &'static RenderEnumDesc {
    // IDA 0x16b04 (decompiled): `Singleton<EnumDesc<AASamples>>::doGetSingleton` —
    // guard once-init (0x16b60), in-place `EnumDesc` ctor (0x16b7a), `__cxa_atexit`
    // (0x16b98), return `&s` (0x16bc8). Same object as `aa_samples_enum_desc()`.
    // Same shape as 0x1654c.
    aa_samples_enum_desc()
}

// 0x16bf4 — __ZN19CRenderSettingsItemD2Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "CRenderSettingsItem::~CRenderSettingsItem()")]
pub fn stub_16bf4(_item: *mut CRenderSettingsItem) {
    // IDA 0x16bf4 (decompiled): `CRenderSettingsItem::~CRenderSettingsItem` D2 —
    // vtable resets (0x16c28..0x16c42), `signal::disconnectAll` on +192 (0x16c74),
    // `intrusive_ptr_release` on the slot (0x16c7a..0x16c82), `operator delete`
    // on +176 (0x16c88..0x16c90), `std::string::~string` on +168 (0x16c98), base
    // vtables (0x16cb8..0x16cca), `sing = 0` (0x16cce), `Instance::~Instance`
    // (0x16cd2). Rust Drop glue covers the teardown; the signal/slot and string
    // drops run with the value. Same drop-glue shape as stub_1335c in generated_next_b.
}

// 0x16d34 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>> *)")]
pub fn stub_16d34(map: &mut BTreeMap<*const (), i32>) {
    // IDA 0x16d34 (decompiled): `_Rb_tree<Name const*, pair<Name const*, ResolutionPreset>>::_M_erase` —
    // null check (0x16d3e), then loop: recurse into the right child (`v2[3]`, 0x16d46),
    // spill the left link (`v2[2]`, 0x16d4c), `operator delete` the node (0x16d4e),
    // continue with the left (0x16d52..0x16d56). Post-order subtree delete; from the
    // map dtor/clear path the subtree is the whole tree, so the BTreeMap drops the
    // same nodes via `clear`. Same collapse as the `_M_insert` twins in generated_next_b.
    map.clear();
}

// 0x16d5c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>> *)")]
pub fn stub_16d5c(map: &mut BTreeMap<*const (), i32>) {
    // IDA 0x16d5c (decompiled): `_Rb_tree<Name const*, pair<Name const*, QualityLevel>>::_M_erase` —
    // same null-check + recurse-right (0x16d6e) + spill-left + delete + loop shape as
    // 0x16d34, over the QualityLevel value word. Same `clear` collapse.
    map.clear();
}

// 0x16d84 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>> *)")]
pub fn stub_16d84(map: &mut BTreeMap<*const (), i32>) {
    // IDA 0x16d84 (decompiled): `_Rb_tree<Name const*, pair<Name const*, ShadowMode>>::_M_erase` —
    // same recurse-right (0x16d96) + spill-left + delete + loop shape as 0x16d34,
    // over the ShadowMode value word. Same `clear` collapse.
    map.clear();
}

// 0x16dac — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>> *)")]
pub fn stub_16dac(map: &mut BTreeMap<*const (), i32>) {
    // IDA 0x16dac (decompiled): `_Rb_tree<Name const*, pair<Name const*, AntialiasingMode>>::_M_erase` —
    // same recurse-right + spill-left + delete + loop shape as 0x16d34, over the
    // AntialiasingMode value word. Same `clear` collapse.
    map.clear();
}

// 0x16dd4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>> *)")]
pub fn stub_16dd4(map: &mut BTreeMap<*const (), i32>) {
    // IDA 0x16dd4 (decompiled): `_Rb_tree<Name const*, pair<Name const*, FrameRateManagerMode>>::_M_erase` —
    // same recurse-right + spill-left + delete + loop shape as 0x16d34, over the
    // FrameRateManagerMode value word. Same `clear` collapse.
    map.clear();
}

// 0x16dfc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>> *)")]
pub fn stub_16dfc(map: &mut BTreeMap<*const (), i32>) {
    // IDA 0x16dfc (decompiled): `_Rb_tree<Name const*, pair<Name const*, GraphicsMode>>::_M_erase` —
    // same recurse-right + spill-left + delete + loop shape as 0x16d34, over the
    // GraphicsMode value word. Same `clear` collapse.
    map.clear();
}

// 0x16e24 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>> *)")]
pub fn stub_16e24(map: &mut BTreeMap<*const (), i32>) {
    // IDA 0x16e24 (decompiled): `_Rb_tree<Name const*, pair<Name const*, AASamples>>::_M_erase` —
    // same recurse-right (0x16e36) + spill-left + delete + loop shape as 0x16d34,
    // over the AASamples value word. Same `clear` collapse.
    map.clear();
}

// 0x16e4c — __GLOBAL__I_a
#[doc(alias = "global constructor keyed to_a")]
pub fn stub_16e4c() {
    // IDA 0x16e4c (`__GLOBAL__I_a`, disasm 0x16e4c..): stores
    // `boost::system::generic_category()` / `system_category()` into the
    // `__MergedGlobals_33` slots (dword_130C024 et al.). Process-static
    // error-category init; the `__cxa_guard` once-init collapses into static
    // init (cf. instance.rs `INSTANCE_SIGNAL_MUTEX`). No observable body remains.
}

// 0x17aac — __ZN5boost10shared_ptrIN3RBX10Reflection5TupleEEC1IS3_EEPT_
#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::Tuple>::shared_ptr<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *)")]
// was: boost::shared_ptr<RBX::Reflection::Tuple>::shared_ptr<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *)
pub use rbx_reflection::generated::stub_0x17aac as stub_17aac;

// 0x17b80 — __ZN5boost10shared_ptrIKN3RBX10Reflection5TupleEEC2IS3_EERKNS0_IT_EENS_6detail24sp_enable_if_convertibleIS7_S4_E4typeE
#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::Tuple const>::shared_ptr<RBX::Reflection::Tuple>(rbx_core::SharedPtr<RBX::Reflection::Tuple> const&,boost::detail::sp_enable_if_convertible<RBX::Reflection::Tuple,RBX::Reflection::Tuple const>::type)")]
// was: boost::shared_ptr<RBX::Reflection::Tuple const>::shared_ptr<RBX::Reflection::Tuple>(boost::shared_ptr<RBX::Reflection::Tuple> const&,boost::detail::sp_enable_if_convertible<RBX::Reflection::Tuple,RBX::Reflection::Tuple const>::type)
pub use rbx_reflection::generated::stub_0x17b80 as stub_17b80;

// 0x17c58 — __GLOBAL__I_a_0
#[doc(alias = "global constructor keyed to_a_0")]
pub fn stub_17c58() {
    // IDA 0x17c58 (`__GLOBAL__I_a_0`, disasm 0x17c58..): stores
    // `boost::system::generic_category()` / `system_category()` into the
    // `__MergedGlobals_34` slots (dword_130C380 et al.). Same static-init
    // collapse as 0x16e4c; no observable body remains.
}

// 0x17df0 — +[Appirater setAppId:]
// type: void __cdecl(id, SEL, id)
#[doc(alias = "+[Appirater setAppId:]")]
pub use rbx_reflection::generated_bg_2::stub_0x17df0 as stub_17df0;

// 0x17e00 — +[Appirater setDaysUntilPrompt:]
// type: void __cdecl(id, SEL, double)
#[doc(alias = "+[Appirater setDaysUntilPrompt:]")]
pub use rbx_reflection::generated_bg_2::stub_0x17e00 as stub_17e00;

// 0x17e14 — +[Appirater setUsesUntilPrompt:]
// type: void __cdecl(id, SEL, int)
#[doc(alias = "+[Appirater setUsesUntilPrompt:]")]
pub use rbx_reflection::generated_bg_2::stub_0x17e14 as stub_17e14;

// 0x17e24 — +[Appirater setSignificantEventsUntilPrompt:]
// type: void __cdecl(id, SEL, int)
#[doc(alias = "+[Appirater setSignificantEventsUntilPrompt:]")]
pub use rbx_reflection::generated_bg_2::stub_0x17e24 as stub_17e24;

// 0x17e34 — +[Appirater setTimeBeforeReminding:]
// type: void __cdecl(id, SEL, double)
#[doc(alias = "+[Appirater setTimeBeforeReminding:]")]
pub use rbx_reflection::generated_bg_2::stub_0x17e34 as stub_17e34;

// 0x17e48 — +[Appirater setDebug:]
// type: void __cdecl(id, SEL, char)
#[doc(alias = "+[Appirater setDebug:]")]
pub use rbx_reflection::generated_bg_2::stub_0x17e48 as stub_17e48;

// 0x17e58 — +[Appirater setDelegate:]
// type: void __cdecl(id, SEL, id)
#[doc(alias = "+[Appirater setDelegate:]")]
pub use rbx_reflection::generated_bg_2::stub_0x17e58 as stub_17e58;

// 0x17e68 — -[Appirater connectedToNetwork]
// type: char __cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater connectedToNetwork]")]
pub use rbx_reflection::generated_bg_2::stub_0x17e68 as stub_17e68;

// 0x18094 — ___copy_helper_block_
#[doc(alias = "___copy_helper_block_")]
pub use rbx_reflection::generated_bg_2::stub_0x18094 as stub_18094;

// 0x180a0 — ___destroy_helper_block_
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_")]
pub use rbx_reflection::generated_bg_2::stub_0x180a0 as stub_180a0;

// 0x180a8 — -[Appirater showRatingAlert]
// type: void __cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater showRatingAlert]")]
pub use rbx_reflection::generated_bg_2::stub_0x180a8 as stub_180a8;

// 0x183d8 — -[Appirater ratingConditionsHaveBeenMet]
// type: char __cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater ratingConditionsHaveBeenMet]")]
pub use rbx_reflection::generated_bg_2::stub_0x183d8 as stub_183d8;

// 0x185b0 — -[Appirater incrementUseCount]
// type: void __cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater incrementUseCount]")]
pub use rbx_reflection::generated_bg_2::stub_0x185b0 as stub_185b0;

// 0x18878 — -[Appirater incrementSignificantEventCount]
// type: void __cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater incrementSignificantEventCount]")]
pub use rbx_reflection::generated_bg_2::stub_0x18878 as stub_18878;

// 0x18b18 — -[Appirater incrementAndRate:]
// type: void __cdecl(Appirater *self, SEL, char)
#[doc(alias = "-[Appirater incrementAndRate:]")]
pub use rbx_reflection::generated_bg_2::stub_0x18b18 as stub_18b18;

// 0x18bb4 — ___30-[Appirater incrementAndRate:]_block_invoke
#[doc(alias = "___30-[Appirater incrementAndRate:]_block_invoke")]
pub use rbx_reflection::generated_bg_2::stub_0x18bb4 as stub_18bb4;

// 0x18bc8 — ___copy_helper_block_125
#[doc(alias = "___copy_helper_block_125")]
pub use rbx_reflection::generated_bg_2::stub_0x18bc8 as stub_18bc8;

// 0x18bd4 — ___destroy_helper_block_126
#[doc(alias = "___destroy_helper_block_126")]
pub use rbx_reflection::generated_bg_2::stub_0x18bd4 as stub_18bd4;

// 0x18bdc — -[Appirater incrementSignificantEventAndRate:]
// type: void __cdecl(Appirater *self, SEL, char)
#[doc(alias = "-[Appirater incrementSignificantEventAndRate:]")]
pub use rbx_reflection::generated_bg_2::stub_0x18bdc as stub_18bdc;

// 0x18c78 — ___46-[Appirater incrementSignificantEventAndRate:]_block_invoke
#[doc(alias = "___46-[Appirater incrementSignificantEventAndRate:]_block_invoke")]
pub use rbx_reflection::generated_bg_2::stub_0x18c78 as stub_18c78;

// 0x18c8c — ___copy_helper_block_130
#[doc(alias = "___copy_helper_block_130")]
pub use rbx_reflection::generated_bg_2::stub_0x18c8c as stub_18c8c;

// 0x18c98 — ___destroy_helper_block_131
#[doc(alias = "___destroy_helper_block_131")]
pub use rbx_reflection::generated_bg_2::stub_0x18c98 as stub_18c98;

// 0x18ca0 — +[Appirater appLaunched]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[Appirater appLaunched]")]
pub use rbx_reflection::generated_bg_2::stub_0x18ca0 as stub_18ca0;

// 0x18cc0 — +[Appirater appLaunched:]
// type: void __cdecl(id, SEL, char)
#[doc(alias = "+[Appirater appLaunched:]")]
pub use rbx_reflection::generated_bg_2::stub_0x18cc0 as stub_18cc0;

// 0x18d10 — ___25+[Appirater appLaunched:]_block_invoke
#[doc(alias = "___25+[Appirater appLaunched:]_block_invoke")]
pub use rbx_reflection::generated_bg_2::stub_0x18d10 as stub_18d10;

// 0x18d4c — -[Appirater hideRatingAlert]
// type: void __cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater hideRatingAlert]")]
pub use rbx_reflection::generated_bg_2::stub_0x18d4c as stub_18d4c;

// 0x18dbc — +[Appirater appWillResignActive]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[Appirater appWillResignActive]")]
pub use rbx_reflection::generated_bg_2::stub_0x18dbc as stub_18dbc;

// 0x18e0c — +[Appirater appEnteredForeground:]
// type: void __cdecl(id, SEL, char)
#[doc(alias = "+[Appirater appEnteredForeground:]")]
pub use rbx_reflection::generated_bg_2::stub_0x18e0c as stub_18e0c;

// 0x18e5c — ___34+[Appirater appEnteredForeground:]_block_invoke
#[doc(alias = "___34+[Appirater appEnteredForeground:]_block_invoke")]
pub use rbx_reflection::generated_bg_2::stub_0x18e5c as stub_18e5c;

// 0x18e98 — +[Appirater userDidSignificantEvent:]
// type: void __cdecl(id, SEL, char)
#[doc(alias = "+[Appirater userDidSignificantEvent:]")]
pub use rbx_reflection::generated_bg_2::stub_0x18e98 as stub_18e98;

// 0x18ee8 — ___37+[Appirater userDidSignificantEvent:]_block_invoke
#[doc(alias = "___37+[Appirater userDidSignificantEvent:]_block_invoke")]
pub use rbx_reflection::generated_bg_2::stub_0x18ee8 as stub_18ee8;

// 0x18f24 — +[Appirater rateApp]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[Appirater rateApp]")]
pub use rbx_reflection::generated_bg_2::stub_0x18f24 as stub_18f24;

// 0x19028 — -[Appirater alertView:clickedButtonAtIndex:]
// type: void __cdecl(Appirater *self, SEL, id, int)
#[doc(alias = "-[Appirater alertView:clickedButtonAtIndex:]")]
pub use rbx_reflection::generated_bg_2::stub_0x19028 as stub_19028;

// 0x191d4 — -[Appirater ratingAlert]
// type: UIAlertView *__cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater ratingAlert]")]
pub use rbx_reflection::generated_bg_2::stub_0x191d4 as stub_191d4;

// 0x191e4 — -[Appirater setRatingAlert:]
// type: void __cdecl(Appirater *self, SEL, id)
#[doc(alias = "-[Appirater setRatingAlert:]")]
pub use rbx_reflection::generated_bg_2::stub_0x191e4 as stub_191e4;

// 0x19208 — -[Appirater delegate]
// type: AppiraterDelegate *__cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater delegate]")]
pub use rbx_reflection::generated_bg_2::stub_0x19208 as stub_19208;

// 0x19218 — -[Appirater setDelegate:]
// type: void __cdecl(Appirater *self, SEL, id)
#[doc(alias = "-[Appirater setDelegate:]")]
pub use rbx_reflection::generated_bg_2::stub_0x19218 as stub_19218;

// 0x19228 — -[AppDelegate init]
// type: AppDelegate *__cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate init]")]
pub use rbx_reflection::generated_bg_2::stub_0x19228 as stub_19228;

// 0x19254 — -[AppDelegate dealloc]
// type: void __cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate dealloc]")]
pub use rbx_reflection::generated_bg_2::stub_0x19254 as stub_19254;

// 0x192b4 — -[AppDelegate application:didFinishLaunchingWithOptions:]
// type: char __cdecl(AppDelegate *self, SEL, id, id)
#[doc(alias = "-[AppDelegate application:didFinishLaunchingWithOptions:]")]
pub use rbx_reflection::generated_bg_2::stub_0x192b4 as stub_192b4;

// 0x194ec — ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke")]
pub use rbx_reflection::generated_bg_3::stub_0x194ec as stub_194ec;

// 0x19514 — ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2
// type: void __cdecl(id)
#[doc(alias = "___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2")]
pub use rbx_reflection::generated_bg_3::stub_0x19514 as stub_19514;

// 0x195a0 — -[AppDelegate applicationWillResignActive:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationWillResignActive:]")]
pub use rbx_reflection::generated_bg_3::stub_0x195a0 as stub_195a0;

// 0x196e4 — -[AppDelegate applicationDidEnterBackground:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationDidEnterBackground:]")]
pub use rbx_reflection::generated_bg_3::stub_0x196e4 as stub_196e4;

// 0x19a30 — -[AppDelegate applicationDidReceiveMemoryWarning:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationDidReceiveMemoryWarning:]")]
pub use rbx_reflection::generated_bg_3::stub_0x19a30 as stub_19a30;

// 0x19b60 — -[AppDelegate applicationWillEnterForeground:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationWillEnterForeground:]")]
pub use rbx_reflection::generated_bg_3::stub_0x19b60 as stub_19b60;

// 0x19cdc — -[AppDelegate applicationDidBecomeActive:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationDidBecomeActive:]")]
pub use rbx_reflection::generated_bg_3::stub_0x19cdc as stub_19cdc;

// 0x19f34 — ___42-[AppDelegate applicationDidBecomeActive:]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___42-[AppDelegate applicationDidBecomeActive:]_block_invoke")]
pub use rbx_reflection::generated_bg_3::stub_0x19f34 as stub_19f34;

// 0x19f7c — -[AppDelegate applicationWillTerminate:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationWillTerminate:]")]
pub use rbx_reflection::generated_bg_3::stub_0x19f7c as stub_19f7c;

// 0x1a098 — __Z18_topMostControllerP16UIViewController
// type: id __fastcall(id)
#[doc(alias = "_topMostController(UIViewController *)")]
pub use rbx_reflection::generated_bg_3::stub_0x1a098 as stub_1a098;

// 0x1a124 — __Z17topMostControllerv
// type: _DWORD __fastcall()
#[doc(alias = "topMostController(void)")]
pub use rbx_reflection::generated_bg_3::stub_0x1a124 as stub_1a124;

// 0x1a174 — -[AppDelegate application:openURL:sourceApplication:annotation:]
// type: char __cdecl(AppDelegate *self, SEL, id, id, id, id)
#[doc(alias = "-[AppDelegate application:openURL:sourceApplication:annotation:]")]
pub use rbx_reflection::generated_bg_3::stub_0x1a174 as stub_1a174;

// 0x1a234 — -[AppDelegate TryLaunchPlace:]
// type: void __cdecl(AppDelegate *self, SEL, int)
#[doc(alias = "-[AppDelegate TryLaunchPlace:]")]
pub use rbx_reflection::generated_bg_3::stub_0x1a234 as stub_1a234;

// 0x1a494 — -[AppDelegate bgTask]
// type: unsigned int __cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate bgTask]")]
pub use rbx_reflection::generated_bg_3::stub_0x1a494 as stub_1a494;

// 0x1a4a8 — -[AppDelegate setBgTask:]
// type: void __cdecl(AppDelegate *self, SEL, unsigned int)
#[doc(alias = "-[AppDelegate setBgTask:]")]
pub use rbx_reflection::generated_bg_3::stub_0x1a4a8 as stub_1a4a8;

// 0x1a4c0 — -[AppDelegate window]
// type: UIWindow *__cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate window]")]
pub use rbx_reflection::generated_bg_3::stub_0x1a4c0 as stub_1a4c0;

// 0x1a4d0 — -[AppDelegate setWindow:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate setWindow:]")]
pub use rbx_reflection::generated_bg_3::stub_0x1a4d0 as stub_1a4d0;

// 0x1a4f4 — -[AppDelegate .cxx_destruct]
// type: void __cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate .cxx_destruct]")]
pub use rbx_reflection::generated_bg_3::stub_0x1a4f4 as stub_1a4f4;

// 0x1a5bc — -[AppDelegate .cxx_construct]
// type: id __cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate .cxx_construct]")]
pub use rbx_reflection::generated_bg_3::stub_0x1a5bc as stub_1a5bc;

// 0x1a5d0 — __GLOBAL__I_a_1
#[doc(alias = "global constructor keyed to_a_1")]
pub use rbx_reflection::generated_bg_3::stub_0x1a5d0 as stub_1a5d0;

// 0x1a768 — _main
// type: int __fastcall(int argc, const char **argv, const char **envp)
#[doc(alias = "_main")]
pub use rbx_reflection::generated_bg_3::stub_0x1a768 as stub_1a768;

// 0x1a7d4 — __GLOBAL__I_a_2
#[doc(alias = "global constructor keyed to_a_2")]
pub use rbx_reflection::generated_bg_3::stub_0x1a7d4 as stub_1a7d4;

// 0x1a970 — -[DebugSettingsViewController initWithCoder:]
// type: DebugSettingsViewController *__cdecl(DebugSettingsViewController *self, SEL, id)
#[doc(alias = "-[DebugSettingsViewController initWithCoder:]")]
pub use rbx_reflection::generated_bg_3::stub_0x1a970 as stub_1a970;

// 0x1ab20 — -[DebugSettingsViewController dealloc]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController dealloc]")]
pub use rbx_reflection::generated_bg_3::stub_0x1ab20 as stub_1ab20;

// 0x1ab6c — -[DebugSettingsViewController reloadOldData]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController reloadOldData]")]
pub use rbx_reflection::generated_bg_3::stub_0x1ab6c as stub_1ab6c;

// 0x1ab70 — -[DebugSettingsViewController viewDidLoad]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController viewDidLoad]")]
pub use rbx_reflection::generated_bg_3::stub_0x1ab70 as stub_1ab70;

// 0x1abb0 — -[DebugSettingsViewController setDisplayUI]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController setDisplayUI]")]
pub use rbx_reflection::generated_bg_3::stub_0x1abb0 as stub_1abb0;

// 0x1ac80 — -[DebugSettingsViewController displayPickerDoneClicked:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, id)
#[doc(alias = "-[DebugSettingsViewController displayPickerDoneClicked:]")]
pub use rbx_reflection::generated_bg_3::stub_0x1ac80 as stub_1ac80;

// 0x1ad78 — ___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke")]
pub use rbx_reflection::generated_bg_3::stub_0x1ad78 as stub_1ad78;

// 0x1ae78 — ___copy_helper_block__0
// type: void __fastcall(int, const void **)
#[doc(alias = "___copy_helper_block__0")]
pub use rbx_reflection::generated_bg_3::stub_0x1ae78 as stub_1ae78;

// 0x1aea8 — ___destroy_helper_block__0
#[doc(alias = "___destroy_helper_block__0")]
pub use rbx_reflection::generated_bg_3::stub_0x1aea8 as stub_1aea8;

// 0x1aed0 — -[DebugSettingsViewController displayTouchUp:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, id)
#[doc(alias = "-[DebugSettingsViewController displayTouchUp:]")]
pub use rbx_reflection::generated_bg_3::stub_0x1aed0 as stub_1aed0;

// 0x1afa0 — ___46-[DebugSettingsViewController displayTouchUp:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___46-[DebugSettingsViewController displayTouchUp:]_block_invoke")]
pub use rbx_reflection::generated_bg_3::stub_0x1afa0 as stub_1afa0;

// 0x1b11c — ___copy_helper_block_66
#[doc(alias = "___copy_helper_block_66")]
pub use rbx_reflection::generated_bg_3::stub_0x1b11c as stub_1b11c;

// 0x1b14c — ___destroy_helper_block_67
#[doc(alias = "___destroy_helper_block_67")]
pub use rbx_reflection::generated_bg_3::stub_0x1b14c as stub_1b14c;

#[cfg(test)]
mod bg_1_singleton_tests {
    use super::*;

    #[test]
    fn singleton_tables_match_enum_desc() {
        assert_eq!(stub_1654c().enum_name, shadow_mode_enum_desc().enum_name);
        assert_eq!(stub_16640().enum_name, resolution_preset_enum_desc().enum_name);
        assert_eq!(stub_16734().enum_name, quality_level_enum_desc().enum_name);
        assert_eq!(stub_16828().enum_name, antialiasing_mode_enum_desc().enum_name);
        assert_eq!(stub_1691c().enum_name, frame_rate_manager_mode_enum_desc().enum_name);
        assert_eq!(stub_16a10().enum_name, graphics_mode_enum_desc().enum_name);
        assert_eq!(stub_16b04().enum_name, aa_samples_enum_desc().enum_name);
        assert!(stub_1654c().lookup_value("Off").is_some());
        assert!(stub_16734().lookup_value("Level01").is_some());
    }

    #[test]
    fn init_thunks_touch_singletons() {
        stub_1663c();
        stub_16730();
        stub_16824();
        stub_16918();
        stub_16a0c();
        stub_16b00();
        stub_16e4c();
        stub_17c58();
    }

    #[test]
    fn erase_clears_name_maps() {
        let mut map: BTreeMap<*const (), i32> = BTreeMap::new();
        let k = 0x7000 as *const ();
        map.insert(k, 3);
        stub_16d34(&mut map);
        assert!(map.is_empty());
        map.insert(k, 5);
        stub_16d5c(&mut map);
        stub_16d84(&mut map);
        stub_16dac(&mut map);
        stub_16dd4(&mut map);
        stub_16dfc(&mut map);
        stub_16e24(&mut map);
        assert!(map.is_empty());
    }

    #[test]
    fn shared_ptr_tuple_adopt_and_convert() {
        let ptr = rbx_reflection::generated::stub_0x17aac(SharedPtr::new(
            rbx_reflection::generated::Tuple,
        ));
        let converted = stub_17b80(&ptr);
        assert_eq!(SharedPtr::strong_count(&converted), 2);
        let _ = stub_17aac(SharedPtr::new(rbx_reflection::generated::Tuple));
    }
}
