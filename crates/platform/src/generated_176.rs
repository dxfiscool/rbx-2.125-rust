//! platform generated_176 — next 100 stubs EA-sorted asc global filler continuation after 0x16640 (global 500->600, rbx_core::SharedPtr not boost)
//! Filter: global EA-sorted asc, rbx_core::SharedPtr not boost
//! Batch: 100 stubs EA-sorted asc | skeleton batch | range 0x16730..0x1aed0 (rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use std::collections::BTreeMap;
use std::sync::LazyLock;
use super::generated_171::{RenderEnumDesc, RenderSettingsItem};
use super::view_controllers::{
    AppDelegate, Appirater, LaunchAction, ObjCId, ViewControllerGraph,
    did_become_active_fetch_settings_block, did_finish_launching_appirater_block,
    did_finish_launching_flurry_block, top_most_controller,
};
use rbx_reflection::generated::{
    Tuple as ReflectionTuple, stub_0x179f4 as tuple_upload_place,
    stub_0x17aac as tuple_shared_ptr_adopt, stub_0x17b80 as tuple_shared_ptr_const_copy,
};

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x16730 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12QualityLevelEEEE13initSingletonEv
// mangled: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12QualityLevelEEEE13initSingletonEv
// type: 
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel> const>::initSingleton(void)")]
pub fn stub_16730() -> &'static RenderEnumDesc {
    // IDA 0x16730 (`Singleton<EnumDesc<QualityLevel> const>::initSingleton`):
    // thunk tail-calling `doGetSingleton` at 0x16734. Same as 0x16548.
    // Family-verified.
    stub_16734()
}

// 0x16734 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12QualityLevelEEEE14doGetSingletonEv
// mangled: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12QualityLevelEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel> const>::doGetSingleton(void)")]
pub fn stub_16734() -> &'static RenderEnumDesc {
    // IDA 0x16734 (`Singleton<EnumDesc<QualityLevel> const>::doGetSingleton`):
    // guard-once construction of the static enum descriptor (0x8e24 ctor).
    // Same shape as the 0x1654c anchor. Family-verified.
    static DESC: LazyLock<RenderEnumDesc> =
        LazyLock::new(super::generated_171::stub_8e24);
    &DESC
}

// 0x16824 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16AntialiasingModeEEEE13initSingletonEv
// mangled: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16AntialiasingModeEEEE13initSingletonEv
// type: 
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode> const>::initSingleton(void)")]
pub fn stub_16824() -> &'static RenderEnumDesc {
    // IDA 0x16824 (`Singleton<EnumDesc<AntialiasingMode>
    // const>::initSingleton`): thunk → `doGetSingleton` at 0x16828. Same
    // as 0x16548. Family-verified.
    stub_16828()
}

// 0x16828 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16AntialiasingModeEEEE14doGetSingletonEv
// mangled: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16AntialiasingModeEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode> const>::doGetSingleton(void)")]
pub fn stub_16828() -> &'static RenderEnumDesc {
    // IDA 0x16828 (`Singleton<EnumDesc<AntialiasingMode>
    // const>::doGetSingleton`): guard-once construction (0x8a88 ctor).
    // Same shape as the 0x1654c anchor. Family-verified.
    static DESC: LazyLock<RenderEnumDesc> =
        LazyLock::new(super::generated_171::stub_8a88);
    &DESC
}

// 0x16918 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEEE13initSingletonEv
// mangled: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEEE13initSingletonEv
// type: 
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode> const>::initSingleton(void)")]
pub fn stub_16918() -> &'static RenderEnumDesc {
    // IDA 0x16918 (`Singleton<EnumDesc<FrameRateManagerMode>
    // const>::initSingleton`): thunk → `doGetSingleton` at 0x1691c. Same
    // as 0x16548. Family-verified.
    stub_1691c()
}

// 0x1691c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEEE14doGetSingletonEv
// mangled: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode> const>::doGetSingleton(void)")]
pub fn stub_1691c() -> &'static RenderEnumDesc {
    // IDA 0x1691c (`Singleton<EnumDesc<FrameRateManagerMode>
    // const>::doGetSingleton`): guard-once construction (0x88c4 ctor).
    // Same shape as the 0x1654c anchor. Family-verified.
    static DESC: LazyLock<RenderEnumDesc> =
        LazyLock::new(super::generated_171::stub_88c4);
    &DESC
}

// 0x16a0c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12GraphicsModeEEEE13initSingletonEv
// mangled: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12GraphicsModeEEEE13initSingletonEv
// type: 
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode> const>::initSingleton(void)")]
pub fn stub_16a0c() -> &'static RenderEnumDesc {
    // IDA 0x16a0c (`Singleton<EnumDesc<GraphicsMode> const>::initSingleton`):
    // thunk → `doGetSingleton` at 0x16a10. Same as 0x16548.
    // Family-verified.
    stub_16a10()
}

// 0x16a10 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12GraphicsModeEEEE14doGetSingletonEv
// mangled: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12GraphicsModeEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode> const>::doGetSingleton(void)")]
pub fn stub_16a10() -> &'static RenderEnumDesc {
    // IDA 0x16a10 (`Singleton<EnumDesc<GraphicsMode> const>::doGetSingleton`):
    // guard-once construction (0x86d0 ctor). Same shape as the 0x1654c
    // anchor. Family-verified.
    static DESC: LazyLock<RenderEnumDesc> =
        LazyLock::new(super::generated_171::stub_86d0);
    &DESC
}

// 0x16b00 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings9AASamplesEEEE13initSingletonEv
// mangled: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings9AASamplesEEEE13initSingletonEv
// type: 
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples> const>::initSingleton(void)")]
pub fn stub_16b00() -> &'static RenderEnumDesc {
    // IDA 0x16b00 (`Singleton<EnumDesc<AASamples> const>::initSingleton`):
    // thunk → `doGetSingleton` at 0x16b04. Same as 0x16548.
    // Family-verified.
    stub_16b04()
}

// 0x16b04 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings9AASamplesEEEE14doGetSingletonEv
// mangled: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings9AASamplesEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples> const>::doGetSingleton(void)")]
pub fn stub_16b04() -> &'static RenderEnumDesc {
    // IDA 0x16b04 (`Singleton<EnumDesc<AASamples> const>::doGetSingleton`):
    // guard-once construction (0x850c ctor). Same shape as the 0x1654c
    // anchor. Family-verified.
    static DESC: LazyLock<RenderEnumDesc> =
        LazyLock::new(super::generated_171::stub_850c);
    &DESC
}

// 0x16bf4 — __ZN19CRenderSettingsItemD2Ev
// mangled: __ZN19CRenderSettingsItemD2Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "CRenderSettingsItem::~CRenderSettingsItem()")]
pub fn stub_16bf4(item: &mut RenderSettingsItem) {
    // IDA 0x16bf4 (`CRenderSettingsItem::D2`): vtable resets (host nop),
    // `property_changed.disconnectAll`, connection release, aux buffer
    // `operator delete` (+176), string teardown (+168), `singE = 0`,
    // `Instance::~Instance`. Host: disconnect the signal and drop the owned
    // heap fields in place (`singE` folds into the host owner, cf. 0xf83c).
    // Verified via IDA decompile.
    item.property_changed.disconnect_all();
    item.aux_string_168.clear();
    item.instance_name.clear();
    item.supported_resolutions.clear();
}

// 0x16d34 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// mangled: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>> *)")]
pub fn stub_16d34(map: &mut BTreeMap<String, i32>) {
    // IDA 0x16d34 (`_Rb_tree<Name const*,ResolutionPreset>::_M_erase`):
    // recursive post-order node destroy + `operator delete` per node. Host
    // has no nodes; whole-subtree drop is `clear`. Verified via IDA
    // decompile.
    map.clear();
}

// 0x16d5c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// mangled: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>> *)")]
pub fn stub_16d5c(map: &mut BTreeMap<String, i32>) {
    // IDA 0x16d5c (`_Rb_tree<Name const*,QualityLevel>::_M_erase`). Same as
    // 0x16d34. Family-verified.
    map.clear();
}

// 0x16d84 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// mangled: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>> *)")]
pub fn stub_16d84(map: &mut BTreeMap<String, i32>) {
    // IDA 0x16d84 (`_Rb_tree<Name const*,ShadowMode>::_M_erase`). Same as
    // 0x16d34. Family-verified.
    map.clear();
}

// 0x16dac — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// mangled: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>> *)")]
pub fn stub_16dac(map: &mut BTreeMap<String, i32>) {
    // IDA 0x16dac (`_Rb_tree<Name const*,AntialiasingMode>::_M_erase`). Same
    // as 0x16d34. Family-verified.
    map.clear();
}

// 0x16dd4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// mangled: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>> *)")]
pub fn stub_16dd4(map: &mut BTreeMap<String, i32>) {
    // IDA 0x16dd4 (`_Rb_tree<Name const*,FrameRateManagerMode>::_M_erase`).
    // Same as 0x16d34. Family-verified.
    map.clear();
}

// 0x16dfc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// mangled: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>> *)")]
pub fn stub_16dfc(map: &mut BTreeMap<String, i32>) {
    // IDA 0x16dfc (`_Rb_tree<Name const*,GraphicsMode>::_M_erase`). Same as
    // 0x16d34. Family-verified.
    map.clear();
}

// 0x16e24 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// mangled: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>> *)")]
pub fn stub_16e24(map: &mut BTreeMap<String, i32>) {
    // IDA 0x16e24 (`_Rb_tree<Name const*,AASamples>::_M_erase`). Same as
    // 0x16d34. Family-verified.
    map.clear();
}

// 0x16e4c — __GLOBAL__I_a
// mangled: __GLOBAL__I_a
// type: 
#[doc(alias = "global constructor keyed to_a")]
pub fn stub_16e4c() {
    // IDA 0x16e4c (`__GLOBAL__I_a`): stores `generic_category()` twice plus
    // `system_category()` (0x16e56..0x16e70), constructs `std::ios_base::Init`
    // with `__cxa_atexit` teardown (0x16e72..0x16e9a), then the guarded
    // `exception_ptr` static objects (`bad_alloc_`/`bad_exception_`, from
    // 0x16e9e). Host statics initialize on use; nothing to run.
}
// 0x179e8 — __ZN3RBX9DataModel10serverSaveEv
// mangled: __ZN3RBX9DataModel10serverSaveEv
// type: void __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::serverSave(void)")]
pub fn stub_179e8() {
    // IDA 0x179e8 (`RBX::DataModel::serverSave`): single `BX LR` — stubbed
    // out in this build. Verified via IDA disasm.
}
// 0x179ec — __ZN3RBX9DataModel17internalSaveAsyncENS_9ContentIdEN5boost8functionIFvbEEE
// mangled: __ZN3RBX9DataModel17internalSaveAsyncENS_9ContentIdEN5boost8functionIFvbEEE
// type: void()
// was: RBX::DataModel::internalSaveAsync(RBX::ContentId,boost::function<void ()(bool)>) (boost::function -> Box<dyn Fn>)
#[doc(alias = "RBX::DataModel::internalSaveAsync(RBX::ContentId,Box<dyn Fn<void ()(bool)>)")]
pub fn stub_179ec() {
    // IDA 0x179ec (`RBX::DataModel::internalSaveAsync`): single `BX LR` —
    // stubbed out in this build. Verified via IDA disasm.
}
// 0x179f0 — __ZN3RBX9DataModel12internalSaveENS_9ContentIdE
// mangled: __ZN3RBX9DataModel12internalSaveENS_9ContentIdE
// type: void()
#[doc(alias = "RBX::DataModel::internalSave(RBX::ContentId)")]
pub fn stub_179f0() {
    // IDA 0x179f0 (`RBX::DataModel::internalSave`): single `BX LR` — stubbed
    // out in this build. Verified via IDA disasm.
}
// 0x179f4 — __ZN3RBX9DataModel11uploadPlaceERKSsNS_8Instance10SaveFilterEN5boost8functionIFvNS5_10shared_ptrIKNS_10Reflection5TupleEEEEEENS6_IFvSsEEE
// mangled: __ZN3RBX9DataModel11uploadPlaceERKSsNS_8Instance10SaveFilterEN5boost8functionIFvNS5_10shared_ptrIKNS_10Reflection5TupleEEEEEENS6_IFvSsEEE
// type: void __fastcall(int)
// was: RBX::DataModel::uploadPlace(std::string const&,RBX::Instance::SaveFilter,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "RBX::DataModel::uploadPlace(std::string const&,RBX::Instance::SaveFilter,Box<dyn Fn<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,Box<dyn Fn<void ()(std::string)>)")]
pub fn stub_179f4(slot: &mut SharedPtr<ReflectionTuple>) {
    // IDA 0x179f4 (`RBX::DataModel::uploadPlace`): holder init, fresh empty
    // `Tuple` shared_ptr (0x17a2a), const-copy into the member (0x17a32),
    // temp release (0x17a64). Same as the `rbx_reflection::generated::stub_0x179f4`
    // anchor. Family-verified.
    tuple_upload_place(slot);
}

// 0x17aac — __ZN5boost10shared_ptrIN3RBX10Reflection5TupleEEC1IS3_EEPT_
// mangled: __ZN5boost10shared_ptrIN3RBX10Reflection5TupleEEC1IS3_EEPT_
// type:
// was: boost::shared_ptr<RBX::Reflection::Tuple>::shared_ptr<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::Tuple>::shared_ptr<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *)")]
pub fn stub_17aac(ptr: SharedPtr<ReflectionTuple>) -> SharedPtr<ReflectionTuple> {
    // IDA 0x17aac (`shared_ptr<Tuple>::shared_ptr` adopt ctor): store + adopt
    // the control block (0x17ada/0x17b08), release the previous (0x17b16).
    // Same as the `rbx_reflection::generated::stub_0x17aac` anchor.
    // Family-verified.
    tuple_shared_ptr_adopt(ptr)
}

// 0x17b80 — __ZN5boost10shared_ptrIKN3RBX10Reflection5TupleEEC2IS3_EERKNS0_IT_EENS_6detail24sp_enable_if_convertibleIS7_S4_E4typeE
// mangled: __ZN5boost10shared_ptrIKN3RBX10Reflection5TupleEEC2IS3_EERKNS0_IT_EENS_6detail24sp_enable_if_convertibleIS7_S4_E4typeE
// type:
// was: boost::shared_ptr<RBX::Reflection::Tuple const>::shared_ptr<RBX::Reflection::Tuple>(boost::shared_ptr<RBX::Reflection::Tuple> const&,boost::detail::sp_enable_if_convertible<RBX::Reflection::Tuple,RBX::Reflection::Tuple const>::type) (boost::shared_ptr -> rbx_core::SharedPtr)
#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::Tuple const>::shared_ptr<RBX::Reflection::Tuple>(rbx_core::SharedPtr<RBX::Reflection::Tuple> const&,boost::detail::sp_enable_if_convertible<RBX::Reflection::Tuple,RBX::Reflection::Tuple const>::type)")]
pub fn stub_17b80(other: &SharedPtr<ReflectionTuple>) -> SharedPtr<ReflectionTuple> {
    // IDA 0x17b80 (`shared_ptr<Tuple const>` copy ctor): copy both words
    // (0x17ba8..0x17bb4), bump the use count under the spinlock pool mutex
    // (0x17bfe..0x17c14) — exactly `Arc` clone. Same as the
    // `rbx_reflection::generated::stub_0x17b80` anchor. Family-verified.
    tuple_shared_ptr_const_copy(other)
}
// 0x17c58 — __GLOBAL__I_a_0
// mangled: __GLOBAL__I_a_0
// type:
#[doc(alias = "global constructor keyed to_a_0")]
pub fn stub_17c58() {
    // IDA 0x17c58 (`__GLOBAL__I_a_0`): same shape as 0x16e4c — category
    // stores (0x17c5c..0x17c76), `ios_base::Init` + `__cxa_atexit`
    // (0x17c78..0x17c9a), guarded `exception_ptr` statics (from 0x17c9e).
    // Host statics initialize on use; nothing to run. Family-verified.
}

// 0x17df0 — +[Appirater setAppId:]
// mangled: +[Appirater setAppId:]
// type: void __cdecl(id, SEL, id)
#[doc(alias = "+[Appirater setAppId:]")]
pub fn stub_17df0(app_id: &str) {
    // IDA 0x17df0 (`+[Appirater setAppId:]`): stores `a3` into
    // `_MergedGlobals243` (0x17dfa). Verified via IDA decompile.
    Appirater::set_app_id(app_id);
}

// 0x17e00 — +[Appirater setDaysUntilPrompt:]
// mangled: +[Appirater setDaysUntilPrompt:]
// type: void __cdecl(id, SEL, double)
#[doc(alias = "+[Appirater setDaysUntilPrompt:]")]
pub fn stub_17e00(days: f64) {
    // IDA 0x17e00 (`+[Appirater setDaysUntilPrompt:]`): stores `a3` into
    // `_daysUntilPrompt` (0x17e0e). Verified via IDA decompile.
    Appirater::set_days_until_prompt(days);
}

// 0x17e14 — +[Appirater setUsesUntilPrompt:]
// mangled: +[Appirater setUsesUntilPrompt:]
// type: void __cdecl(id, SEL, int)
#[doc(alias = "+[Appirater setUsesUntilPrompt:]")]
pub fn stub_17e14(uses: u32) {
    // IDA 0x17e14 (`+[Appirater setUsesUntilPrompt:]`): stores `a3` into
    // `_MergedGlobals` (0x17e1e). Verified via IDA decompile.
    Appirater::set_uses_until_prompt(uses);
}

// 0x17e24 — +[Appirater setSignificantEventsUntilPrompt:]
// mangled: +[Appirater setSignificantEventsUntilPrompt:]
// type: void __cdecl(id, SEL, int)
#[doc(alias = "+[Appirater setSignificantEventsUntilPrompt:]")]
pub fn stub_17e24(count: u32) {
    // IDA 0x17e24 (`+[Appirater setSignificantEventsUntilPrompt:]`): stores
    // `a3` into `dword_122316C` (0x17e2e). Verified via IDA decompile.
    Appirater::set_significant_events_until_prompt(count);
}

// 0x17e34 — +[Appirater setTimeBeforeReminding:]
// mangled: +[Appirater setTimeBeforeReminding:]
// type: void __cdecl(id, SEL, double)
#[doc(alias = "+[Appirater setTimeBeforeReminding:]")]
pub fn stub_17e34(days: f64) {
    // IDA 0x17e34 (`+[Appirater setTimeBeforeReminding:]`): stores `a3` into
    // `_timeBeforeReminding` (0x17e42). Verified via IDA decompile.
    Appirater::set_time_before_reminding(days);
}

// 0x17e48 — +[Appirater setDebug:]
// mangled: +[Appirater setDebug:]
// type: void __cdecl(id, SEL, char)
#[doc(alias = "+[Appirater setDebug:]")]
pub fn stub_17e48(debug: bool) {
    // IDA 0x17e48 (`+[Appirater setDebug:]`): stores `a3` into `_debug`
    // (0x17e52). Verified via IDA decompile.
    Appirater::set_debug(debug);
}

// 0x17e58 — +[Appirater setDelegate:]
// mangled: +[Appirater setDelegate:]
// type: void __cdecl(id, SEL, id)
#[doc(alias = "+[Appirater setDelegate:]")]
pub fn stub_17e58(delegate: ObjCId) {
    // IDA 0x17e58 (`+[Appirater setDelegate:]`): stores `a3` into
    // `dword_130C394` (0x17e62), consumed by the `sharedInstance` block at
    // 0x18036. Verified via IDA decompile.
    Appirater::set_class_delegate(delegate);
}

// 0x17e68 — -[Appirater connectedToNetwork]
// mangled: -[Appirater connectedToNetwork]
// type: char __cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater connectedToNetwork]")]
pub fn stub_17e68(instance: &Appirater) -> bool {
    // IDA 0x17e68 (`-[Appirater connectedToNetwork]`): zeroed `sockaddr`
    // probe via `SCNetworkReachabilityCreateWithAddress`/`GetFlags`
    // (0x17e92..0x17eb8), then a test `NSURLConnection` to apple.com
    // (0x17ede..0x17f3a); reachable flags (`(flags & 6) == 2 || flags & 1`,
    // 0x17f4a) report `connection != nil` (0x17f52), which cannot fail on
    // device. The unreachable-flags path logs and returns 0
    // (0x17f60..0x17f64). Verified via IDA decompile.
    instance.connected_to_network()
}

// 0x17f80 — +[Appirater sharedInstance]
// mangled: +[Appirater sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[Appirater sharedInstance]")]
pub fn stub_17f80() -> &'static Appirater {
    // IDA 0x17f80 (`+[Appirater sharedInstance]`): returns `dword_130C398`,
    // materializing it once via `dispatch_once` on the 0x17fe4 block
    // (0x17fb8..0x17fe0). The host `LazyLock` is that once. Verified via IDA
    // decompile.
    Appirater::shared_instance()
}

// 0x17fe4 — ___27+[Appirater sharedInstance]_block_invoke
// mangled: ___27+[Appirater sharedInstance]_block_invoke
// type: 
#[doc(alias = "___27+[Appirater sharedInstance]_block_invoke")]
pub fn stub_17fe4() -> &'static Appirater {
    // IDA 0x17fe4 (`__27+[Appirater sharedInstance]_block_invoke`):
    // `[[Appirater alloc] init]` into `dword_130C398` (0x18008..0x18030),
    // `setDelegate:` from `dword_130C394` (0x18036), observer for
    // `UIApplicationWillResignActiveNotification` → `appWillResignActive`
    // (0x18052..0x18092). Verified via IDA decompile.
    Appirater::init_shared()
}

// 0x18094 — ___copy_helper_block_
// mangled: ___copy_helper_block_
// type: 
#[doc(alias = "___copy_helper_block_")]
pub fn stub_18094(slot: &mut Option<ObjCId>, src: Option<ObjCId>) {
    // IDA 0x18094 (`__copy_helper_block_`): `_Block_object_assign_shim`
    // retaining the captured `self` (`a1 + 20 <- a2 + 20`, 0x1809a). `Arc`
    // captures retain on clone; the slot copy is the retain. Verified via IDA
    // decompile.
    *slot = src;
}

// 0x180a0 — ___destroy_helper_block_
// mangled: ___destroy_helper_block_
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_")]
pub fn stub_180a0(slot: &mut Option<ObjCId>) {
    // IDA 0x180a0 (`__destroy_helper_block_`): `_Block_object_dispose_shim`
    // releasing the capture (`a1 + 20`, 0x180a4). Dropping the host slot is
    // the release. Verified via IDA decompile.
    *slot = None;
}

// 0x180a8 — -[Appirater showRatingAlert]
// mangled: -[Appirater showRatingAlert]
// type: void __cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater showRatingAlert]")]
pub fn stub_180a8(instance: &Appirater) {
    // IDA 0x180a8 (`-[Appirater showRatingAlert]`): `UIAlertView` built from
    // the `RatingTitle`/`RatingString`/button locals (0x180d0..0x18346),
    // retained into `ratingAlert` (0x18358), shown (0x1836a), then
    // `appiraterDidDisplayAlert:` when the delegate answers it
    // (0x1837e..0x183c6). UIKit strings collapse into the model call.
    // Verified via IDA decompile.
    instance.show_rating_alert();
}

// 0x183d8 — -[Appirater ratingConditionsHaveBeenMet]
// mangled: -[Appirater ratingConditionsHaveBeenMet]
// type: char __cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater ratingConditionsHaveBeenMet]")]
pub fn stub_183d8(instance: &Appirater, now_secs: f64) -> bool {
    // IDA 0x183d8 (`-[Appirater ratingConditionsHaveBeenMet]`): the
    // short-circuit chain over the `kAppirater*` defaults with the initializer
    // dates (0x1841a..0x18594) lives in the model; `now_secs` is `+[NSDate
    // date]` in seconds since 1970. Verified via IDA decompile.
    instance.rating_conditions_have_been_met(now_secs)
}

// 0x185b0 — -[Appirater incrementUseCount]
// mangled: -[Appirater incrementUseCount]
// type: void __cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater incrementUseCount]")]
pub fn stub_185b0(instance: &Appirater, current_version: &str, now_secs: f64) {
    // IDA 0x185b0 (`-[Appirater incrementUseCount]`): version-gated
    // first-use/use-count bookkeeping over the `kAppirater*` defaults
    // (0x185d2..0x1886a) lives in the model; `current_version` is
    // `CFBundleVersion`, `now_secs` is `+[NSDate date]`. Verified via IDA
    // decompile.
    instance.increment_use_count(current_version, now_secs);
}

// 0x18878 — -[Appirater incrementSignificantEventCount]
// mangled: -[Appirater incrementSignificantEventCount]
// type: void __cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater incrementSignificantEventCount]")]
pub fn stub_18878(instance: &Appirater, current_version: &str, now_secs: f64) {
    // IDA 0x18878 (`-[Appirater incrementSignificantEventCount]`): twin of
    // 0x185b0 over `kAppiraterSignificantEventCount` (0x1889a..0x18b08).
    // Same as the 0x185b0 anchor. Family-verified.
    instance.increment_significant_event_count(current_version, now_secs);
}

// 0x18b18 — -[Appirater incrementAndRate:]
// mangled: -[Appirater incrementAndRate:]
// type: void __cdecl(Appirater *self, SEL, char)
#[doc(alias = "-[Appirater incrementAndRate:]")]
pub fn stub_18b18(instance: &Appirater, can_rate: bool, current_version: &str, now_secs: f64) {
    // IDA 0x18b18 (`-[Appirater incrementAndRate:]`): `incrementUseCount`
    // (0x18b30); when `a3` and `ratingConditionsHaveBeenMet` (0x18b48) and
    // `connectedToNetwork` (0x18b60), `dispatch_async` to main of the
    // `showRatingAlert` block (0x18b98..0x18baa). The queue hop collapses;
    // the block is `stub_18bb4`. Verified via IDA decompile.
    instance.increment_use_count(current_version, now_secs);
    if can_rate
        && instance.rating_conditions_have_been_met(now_secs)
        && instance.connected_to_network()
    {
        stub_18bb4(instance);
    }
}

// 0x18bb4 — ___30-[Appirater incrementAndRate:]_block_invoke
// mangled: ___30-[Appirater incrementAndRate:]_block_invoke
// type: 
#[doc(alias = "___30-[Appirater incrementAndRate:]_block_invoke")]
pub fn stub_18bb4(instance: &Appirater) {
    // IDA 0x18bb4 (`__30-[Appirater incrementAndRate:]_block_invoke`): single
    // `objc_msgSend` of `showRatingAlert` to the captured `self`
    // (`a1 + 20`). Verified via IDA decompile.
    instance.show_rating_alert();
}

// 0x18bc8 — ___copy_helper_block_125
// mangled: ___copy_helper_block_125
// type: 
#[doc(alias = "___copy_helper_block_125")]
pub fn stub_18bc8(slot: &mut Option<ObjCId>, src: Option<ObjCId>) {
    // IDA 0x18bc8 (`__copy_helper_block_125`): `_Block_object_assign_shim`
    // retaining the capture (0x18bce). Same as the 0x18094 anchor.
    // Family-verified.
    *slot = src;
}

// 0x18bd4 — ___destroy_helper_block_126
// mangled: ___destroy_helper_block_126
// type: 
#[doc(alias = "___destroy_helper_block_126")]
pub fn stub_18bd4(slot: &mut Option<ObjCId>) {
    // IDA 0x18bd4 (`__destroy_helper_block_126`):
    // `_Block_object_dispose_shim` releasing the capture (0x18bd8). Same as
    // the 0x180a0 anchor. Family-verified.
    *slot = None;
}

// 0x18bdc — -[Appirater incrementSignificantEventAndRate:]
// mangled: -[Appirater incrementSignificantEventAndRate:]
// type: void __cdecl(Appirater *self, SEL, char)
#[doc(alias = "-[Appirater incrementSignificantEventAndRate:]")]
pub fn stub_18bdc(instance: &Appirater, can_rate: bool, current_version: &str, now_secs: f64) {
    // IDA 0x18bdc (`-[Appirater incrementSignificantEventAndRate:]`): twin
    // of 0x18b18 over `incrementSignificantEventCount` (0x18bf4..0x18c6e)
    // with the `showRatingAlert` block at 0x18c68. Same as the 0x18b18
    // anchor. Family-verified.
    instance.increment_significant_event_count(current_version, now_secs);
    if can_rate
        && instance.rating_conditions_have_been_met(now_secs)
        && instance.connected_to_network()
    {
        stub_18c78(instance);
    }
}

// 0x18c78 — ___46-[Appirater incrementSignificantEventAndRate:]_block_invoke
// mangled: ___46-[Appirater incrementSignificantEventAndRate:]_block_invoke
// type: 
#[doc(alias = "___46-[Appirater incrementSignificantEventAndRate:]_block_invoke")]
pub fn stub_18c78(instance: &Appirater) {
    // IDA 0x18c78 (`__46-[Appirater incrementSignificantEventAndRate:]_
    // block_invoke`): `showRatingAlert` to the captured `self`. Same as the
    // 0x18bb4 anchor. Family-verified.
    instance.show_rating_alert();
}

// 0x18c8c — ___copy_helper_block_130
// mangled: ___copy_helper_block_130
// type: 
#[doc(alias = "___copy_helper_block_130")]
pub fn stub_18c8c(slot: &mut Option<ObjCId>, src: Option<ObjCId>) {
    // IDA 0x18c8c (`__copy_helper_block_130`): retain the capture (0x18c92).
    // Same as the 0x18094 anchor. Family-verified.
    *slot = src;
}

// 0x18c98 — ___destroy_helper_block_131
// mangled: ___destroy_helper_block_131
// type: 
#[doc(alias = "___destroy_helper_block_131")]
pub fn stub_18c98(slot: &mut Option<ObjCId>) {
    // IDA 0x18c98 (`__destroy_helper_block_131`): release the capture
    // (0x18c9c). Same as the 0x180a0 anchor. Family-verified.
    *slot = None;
}

// 0x18ca0 — +[Appirater appLaunched]
// mangled: +[Appirater appLaunched]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[Appirater appLaunched]")]
pub fn stub_18ca0(current_version: &str, now_secs: f64) {
    // IDA 0x18ca0 (`+[Appirater appLaunched]`): forwards `YES` to
    // `appLaunched:` (0x18cba). Verified via IDA decompile.
    stub_18cc0(true, current_version, now_secs);
}

// 0x18cc0 — +[Appirater appLaunched:]
// mangled: +[Appirater appLaunched:]
// type: void __cdecl(id, SEL, char)
#[doc(alias = "+[Appirater appLaunched:]")]
pub fn stub_18cc0(first_launch: bool, current_version: &str, now_secs: f64) {
    // IDA 0x18cc0 (`+[Appirater appLaunched:]`): captures `a3` into the
    // stack block and `dispatch_async`es it to a global queue
    // (0x18cd0..0x18d08). The queue hop collapses; the block is `stub_18d10`.
    // Verified via IDA decompile.
    stub_18d10(first_launch, current_version, now_secs);
}

// 0x18d10 — ___25+[Appirater appLaunched:]_block_invoke
// mangled: ___25+[Appirater appLaunched:]_block_invoke
// type: 
#[doc(alias = "___25+[Appirater appLaunched:]_block_invoke")]
pub fn stub_18d10(can_rate: bool, current_version: &str, now_secs: f64) {
    // IDA 0x18d10 (`__25+[Appirater appLaunched:]_block_invoke`):
    // `sharedInstance` (0x18d2e) then `incrementAndRate:` with the captured
    // flag (`a1 + 20`). Verified via IDA decompile.
    stub_18b18(Appirater::shared_instance(), can_rate, current_version, now_secs);
}

// 0x18d4c — -[Appirater hideRatingAlert]
// mangled: -[Appirater hideRatingAlert]
// type: void __cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater hideRatingAlert]")]
pub fn stub_18d4c(instance: &Appirater) -> bool {
    // IDA 0x18d4c (`-[Appirater hideRatingAlert]`): dismisses `ratingAlert`
    // when visible (0x18d62..0x18db8); the `_debug` `NSLog` (0x18d8a..0x18d96)
    // has no host sink. Reports whether an alert was dismissed. Verified via
    // IDA decompile.
    instance.hide_rating_alert()
}

// 0x18dbc — +[Appirater appWillResignActive]
// mangled: +[Appirater appWillResignActive]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[Appirater appWillResignActive]")]
pub fn stub_18dbc() {
    // IDA 0x18dbc (`+[Appirater appWillResignActive]`): `_debug` `NSLog`
    // (0x18dcc..0x18dd8, no host sink), then `hideRatingAlert` on
    // `sharedInstance` (0x18df4..0x18e08). Verified via IDA decompile.
    Appirater::shared_instance().hide_rating_alert();
}

// 0x18e0c — +[Appirater appEnteredForeground:]
// mangled: +[Appirater appEnteredForeground:]
// type: void __cdecl(id, SEL, char)
#[doc(alias = "+[Appirater appEnteredForeground:]")]
pub fn stub_18e0c(entered: bool, current_version: &str, now_secs: f64) {
    // IDA 0x18e0c (`+[Appirater appEnteredForeground:]`): same shape as
    // 0x18cc0 — capture `a3`, `dispatch_async` to a global queue
    // (0x18e1c..0x18e54); the block is `stub_18e5c`. Same as the 0x18cc0
    // anchor. Family-verified.
    stub_18e5c(entered, current_version, now_secs);
}

// 0x18e5c — ___34+[Appirater appEnteredForeground:]_block_invoke
// mangled: ___34+[Appirater appEnteredForeground:]_block_invoke
// type: 
#[doc(alias = "___34+[Appirater appEnteredForeground:]_block_invoke")]
pub fn stub_18e5c(can_rate: bool, current_version: &str, now_secs: f64) {
    // IDA 0x18e5c (`__34+[Appirater appEnteredForeground:]_block_invoke`):
    // `sharedInstance` (0x18e7a) then `incrementAndRate:`. Same as the
    // 0x18d10 anchor. Family-verified.
    stub_18b18(Appirater::shared_instance(), can_rate, current_version, now_secs);
}

// 0x18e98 — +[Appirater userDidSignificantEvent:]
// mangled: +[Appirater userDidSignificantEvent:]
// type: void __cdecl(id, SEL, char)
#[doc(alias = "+[Appirater userDidSignificantEvent:]")]
pub fn stub_18e98(significant: bool, current_version: &str, now_secs: f64) {
    // IDA 0x18e98 (`+[Appirater userDidSignificantEvent:]`): same dispatch
    // shape over `incrementSignificantEventAndRate:` (0x18ea8..0x18ee0);
    // the block is `stub_18ee8`. Same as the 0x18cc0 anchor.
    // Family-verified.
    stub_18ee8(significant, current_version, now_secs);
}

// 0x18ee8 — ___37+[Appirater userDidSignificantEvent:]_block_invoke
// mangled: ___37+[Appirater userDidSignificantEvent:]_block_invoke
// type: 
#[doc(alias = "___37+[Appirater userDidSignificantEvent:]_block_invoke")]
pub fn stub_18ee8(can_rate: bool, current_version: &str, now_secs: f64) {
    // IDA 0x18ee8 (`__37+[Appirater userDidSignificantEvent:]_block_invoke`):
    // `sharedInstance` (0x18f06) then `incrementSignificantEventAndRate:`.
    // Verified via IDA decompile.
    stub_18bdc(Appirater::shared_instance(), can_rate, current_version, now_secs);
}

// 0x18f24 — +[Appirater rateApp]
// mangled: +[Appirater rateApp]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[Appirater rateApp]")]
pub fn stub_18f24() -> String {
    // IDA 0x18f24 (`+[Appirater rateApp]`): review URL from the template with
    // `APP_ID` replaced (0x18f48..0x18fa2), flag
    // `kAppiraterRatedCurrentVersion`, `synchronize`, `openURL:`
    // (0x18fbe..0x19024). Returns the opened URL. Verified via IDA decompile.
    Appirater::shared_instance().rate_app()
}

// 0x19028 — -[Appirater alertView:clickedButtonAtIndex:]
// mangled: -[Appirater alertView:clickedButtonAtIndex:]
// type: void __cdecl(Appirater *self, SEL, id, int)
#[doc(alias = "-[Appirater alertView:clickedButtonAtIndex:]")]
pub fn stub_19028(instance: &Appirater, button_index: i32, now_secs: f64) {
    // IDA 0x19028 (`-[Appirater alertView:clickedButtonAtIndex:]`): the
    // three-way button switch with delegate callbacks (0x19052..0x191ca)
    // lives in the model. Verified via IDA decompile.
    instance.alert_view_clicked_button(button_index, now_secs);
}

// 0x191d4 — -[Appirater ratingAlert]
// mangled: -[Appirater ratingAlert]
// type: UIAlertView *__cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater ratingAlert]")]
pub fn stub_191d4(instance: &Appirater) -> ObjCId {
    // IDA 0x191d4 (`-[Appirater ratingAlert]`): returns the `ratingAlert`
    // ivar (0x191e2). Verified via IDA decompile.
    instance.rating_alert()
}

// 0x191e4 — -[Appirater setRatingAlert:]
// mangled: -[Appirater setRatingAlert:]
// type: void __cdecl(Appirater *self, SEL, id)
#[doc(alias = "-[Appirater setRatingAlert:]")]
pub fn stub_191e4(instance: &Appirater, alert: ObjCId) {
    // IDA 0x191e4 (`-[Appirater setRatingAlert:]`): retained-property store
    // via `objc_setProperty` (0x19200). Verified via IDA decompile.
    instance.set_rating_alert(alert);
}

// 0x19208 — -[Appirater delegate]
// mangled: -[Appirater delegate]
// type: AppiraterDelegate *__cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater delegate]")]
pub fn stub_19208(instance: &Appirater) -> ObjCId {
    // IDA 0x19208 (`-[Appirater delegate]`): returns the `_delegate` ivar
    // (0x19216). Verified via IDA decompile.
    instance.delegate()
}

// 0x19218 — -[Appirater setDelegate:]
// mangled: -[Appirater setDelegate:]
// type: void __cdecl(Appirater *self, SEL, id)
#[doc(alias = "-[Appirater setDelegate:]")]
pub fn stub_19218(instance: &Appirater, delegate: ObjCId) {
    // IDA 0x19218 (`-[Appirater setDelegate:]`): plain ivar store
    // (0x19224). Verified via IDA decompile.
    instance.set_delegate(delegate);
}

// 0x19228 — -[AppDelegate init]
// mangled: -[AppDelegate init]
// type: AppDelegate *__cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate init]")]
pub fn stub_19228() -> AppDelegate {
    // IDA 0x19228 (`-[AppDelegate init]`): only `objc_msgSendSuper2` init
    // (0x19242..0x19252); no ivar stores. Verified via IDA decompile.
    AppDelegate::init()
}

// 0x19254 — -[AppDelegate dealloc]
// mangled: -[AppDelegate dealloc]
// type: void __cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate dealloc]")]
pub fn stub_19254(delegate: AppDelegate) {
    // IDA 0x19254 (`-[AppDelegate dealloc]`): `+[RobloxGoogleAnalytics
    // release]` (0x19276), `-[UIWindow release]` (0x1928a), then super
    // dealloc (0x192ac, runs as drop). Verified via IDA decompile.
    delegate.dealloc();
}

// 0x192b4 — -[AppDelegate application:didFinishLaunchingWithOptions:]
// mangled: -[AppDelegate application:didFinishLaunchingWithOptions:]
// type: char __cdecl(AppDelegate *self, SEL, id, id)
#[doc(alias = "-[AppDelegate application:didFinishLaunchingWithOptions:]")]
pub fn stub_192b4(delegate: &AppDelegate) -> bool {
    // IDA 0x192b4: defaults registration (0x192f8..0x19366), CrashReporter /
    // SessionReporter(7) / GA counters (0x19384..0x193c4), two global-queue
    // blocks (0x193d6..0x193ee -> 0x194ec/0x19514), UpgradeCheck, cookie
    // policy, CurrentPlayer username/password restore (0x1940a..0x194ce),
    // returns 1 (0x194e4). Body lives in the model. Verified via IDA decompile.
    delegate.application_did_finish_launching()
}

// 0x194ec — ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke
// mangled: ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke")]
pub fn stub_194ec() {
    // IDA 0x194ec: `+[Flurry startSession:]` with `FM7DNRW56339NC22K8GR`
    // (0x1950e). Verified via IDA decompile.
    did_finish_launching_flurry_block();
}

// 0x19514 — ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2
// mangled: ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2
// type: void __cdecl(id)
#[doc(alias = "___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2")]
pub fn stub_19514() {
    // IDA 0x19514: Appirater config + `appLaunched:` (0x1953a..0x1959a).
    // Verified via IDA decompile.
    did_finish_launching_appirater_block();
}

// 0x195a0 — -[AppDelegate applicationWillResignActive:]
// mangled: -[AppDelegate applicationWillResignActive:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationWillResignActive:]")]
pub fn stub_195a0(delegate: &AppDelegate) {
    // IDA 0x195a0: begin/end `StandardOut::printf` traces (0x19600/0x1965e)
    // around `disableViewBecauseGoingToBackground` (0x1962e..0x19640).
    // Verified via IDA decompile.
    delegate.application_will_resign_active();
}

// 0x196e4 — -[AppDelegate applicationDidEnterBackground:]
// mangled: -[AppDelegate applicationDidEnterBackground:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationDidEnterBackground:]")]
pub fn stub_196e4(delegate: &AppDelegate) {
    // IDA 0x196e4: `RobloxAppState=tryBackground` + synchronize
    // (0x19742..0x1975c), `leaveGame`, signup/login persistence,
    // `reportSessionFor:1`, page-view tracking, then clears `RobloxAppState`
    // (0x197d4..0x199b6). Body lives in the model. Verified via IDA decompile.
    delegate.application_did_enter_background();
}

// 0x19a30 — -[AppDelegate applicationDidReceiveMemoryWarning:]
// mangled: -[AppDelegate applicationDidReceiveMemoryWarning:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationDidReceiveMemoryWarning:]")]
pub fn stub_19a30(delegate: &AppDelegate) {
    // IDA 0x19a30: OOM `printf` (0x19a90), `stopMemoryBouncer:0`
    // (0x19ac0..0x19ad8); falls through to PlaceLauncher only when it
    // returns false (0x19aee..0x19b00). Verified via IDA decompile.
    delegate.application_did_receive_memory_warning();
}

// 0x19b60 — -[AppDelegate applicationWillEnterForeground:]
// mangled: -[AppDelegate applicationWillEnterForeground:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationWillEnterForeground:]")]
pub fn stub_19b60(delegate: &AppDelegate) {
    // IDA 0x19b60: begin/end traces (0x19bc0/0x19c54), `appEnteredForeground:`
    // (0x19bf0), UpgradeCheck (0x19c0e), page-view tracking (0x19c36).
    // Verified via IDA decompile.
    delegate.application_will_enter_foreground();
}

// 0x19cdc — -[AppDelegate applicationDidBecomeActive:]
// mangled: -[AppDelegate applicationDidBecomeActive:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationDidBecomeActive:]")]
pub fn stub_19cdc(delegate: &AppDelegate) {
    // IDA 0x19cdc: `RobloxAppState=tryForeground` (0x19d3c..0x19d56),
    // `enableViewBecauseGoingToForeground`, `reportSessionFor:0`, global-queue
    // settings block (0x19dce..0x19e22 -> 0x19f34), pending `appPlaceID`
    // launch (0x19e32..0x19e48), then `RobloxAppState=inApp`
    // (0x19ea6..0x19eb8). Verified via IDA decompile.
    delegate.application_did_become_active();
}

// 0x19f34 — ___42-[AppDelegate applicationDidBecomeActive:]_block_invoke
// mangled: ___42-[AppDelegate applicationDidBecomeActive:]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___42-[AppDelegate applicationDidBecomeActive:]_block_invoke")]
pub fn stub_19f34() {
    // IDA 0x19f34: `ClientAppSettings::Initialize` + singleton feed
    // `FetchClientSettingsData("iOSAppSettings", ...)` (0x19f38..0x19f56),
    // then `getiOSSettingsServiceWithForcedReadFromWeb:NO` (0x19f78).
    // Verified via IDA decompile.
    did_become_active_fetch_settings_block();
}

// 0x19f7c — -[AppDelegate applicationWillTerminate:]
// mangled: -[AppDelegate applicationWillTerminate:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationWillTerminate:]")]
pub fn stub_19f7c(delegate: &AppDelegate) {
    // IDA 0x19f7c: logs `RobloxGameState`/`RobloxAppState` (0x19fbc..0x19ff8),
    // sets `RobloxAppState=terminated` (0x1a01e..0x1a038), LoginManager
    // teardown (0x1a054..0x1a064), exit page-view tracking (0x1a092).
    // Verified via IDA decompile.
    delegate.application_will_terminate();
}

// 0x1a098 — __Z18_topMostControllerP16UIViewController
// mangled: __Z18_topMostControllerP16UIViewController
// type: id __fastcall(id)
#[doc(alias = "_topMostController(UIViewController *)")]
pub fn stub_1a098(graph: &ViewControllerGraph, root: ObjCId) -> Option<ObjCId> {
    // IDA 0x1a098: walk `presentedViewController` to the chain end
    // (0x1a0ae..0x1a0ca), resolve a navigation controller to its visible
    // controller (0x1a0e4..0x1a118), nil when nothing sits above the root
    // (0x1a11c..0x1a122). Verified via IDA decompile.
    top_most_controller(graph, root)
}

// 0x1a124 — __Z17topMostControllerv
// mangled: __Z17topMostControllerv
// type: _DWORD __fastcall()
#[doc(alias = "topMostController(void)")]
pub fn stub_1a124(graph: &ViewControllerGraph, key_window_root: ObjCId) -> ObjCId {
    // IDA 0x1a124: `sharedApplication` (0x1a140) -> `keyWindow` (0x1a150) ->
    // `rootViewController` (0x1a160, passed in on the host), then loop
    // `_topMostController` until nil (0x1a164..0x1a16c) and return the last
    // controller (0x1a170). Verified via IDA decompile.
    let mut top = key_window_root;
    while let Some(next) = top_most_controller(graph, top) {
        top = next;
    }
    top
}

// 0x1a174 — -[AppDelegate application:openURL:sourceApplication:annotation:]
// mangled: -[AppDelegate application:openURL:sourceApplication:annotation:]
// type: char __cdecl(AppDelegate *self, SEL, id, id, id, id)
#[doc(alias = "-[AppDelegate application:openURL:sourceApplication:annotation:]")]
pub fn stub_1a174(
    delegate: &AppDelegate,
    url_absolute_string: &str,
    url_host: &str,
    url_path: &str,
    source_application: &str,
    annotation: &str,
) -> bool {
    // IDA 0x1a174: logs the open (0x1a18a), requires the `robloxmobile`
    // prefix (0x1a19c..0x1a1c2), logs host/path (0x1a1d6..0x1a208), stashes
    // `appPlaceID = [host intValue]` (0x1a210..0x1a22e), returns 1 (0x1a230).
    // Verified via IDA decompile.
    delegate.application_open_url(
        url_absolute_string,
        url_host,
        url_path,
        source_application,
        annotation,
    )
}

// 0x1a234 — -[AppDelegate TryLaunchPlace:]
// mangled: -[AppDelegate TryLaunchPlace:]
// type: void __cdecl(AppDelegate *self, SEL, int)
#[doc(alias = "-[AppDelegate TryLaunchPlace:]")]
pub fn stub_1a234(
    delegate: &AppDelegate,
    place_id: i32,
    top_controller_class: &str,
) -> LaunchAction {
    // IDA 0x1a234: window/root + keyWindow trace (0x1a24c..0x1a2f2) feeds the
    // `topMostController` class read (0x1a2fc..0x1a316); dispatch over
    // Login/Home/RobloxNavBar/Game controllers (0x1a334..0x1a488) lives in
    // the model. Verified via IDA decompile.
    delegate.try_launch_place(place_id, top_controller_class)
}

// 0x1a494 — -[AppDelegate bgTask]
// mangled: -[AppDelegate bgTask]
// type: unsigned int __cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate bgTask]")]
pub fn stub_1a494(delegate: &AppDelegate) -> u32 {
    // IDA 0x1a494: `LDR` the `bgTask` ivar (0x1a4a0) + `DMB ISH` (0x1a4a2).
    // Verified via IDA decompile.
    delegate.bg_task()
}

// 0x1a4a8 — -[AppDelegate setBgTask:]
// mangled: -[AppDelegate setBgTask:]
// type: void __cdecl(AppDelegate *self, SEL, unsigned int)
#[doc(alias = "-[AppDelegate setBgTask:]")]
pub fn stub_1a4a8(delegate: &AppDelegate, task: u32) {
    // IDA 0x1a4a8: `DMB ISH` (0x1a4b0), store the `bgTask` ivar (0x1a4b8),
    // `DMB ISH` (0x1a4ba). Verified via IDA decompile.
    delegate.set_bg_task(task);
}

// 0x1a4c0 — -[AppDelegate window]
// mangled: -[AppDelegate window]
// type: UIWindow *__cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate window]")]
pub fn stub_1a4c0(delegate: &AppDelegate) -> Option<ObjCId> {
    // IDA 0x1a4c0: returns `self->_window` (0x1a4ce). Verified via IDA decompile.
    delegate.window()
}

// 0x1a4d0 — -[AppDelegate setWindow:]
// mangled: -[AppDelegate setWindow:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate setWindow:]")]
pub fn stub_1a4d0(delegate: &AppDelegate, window: Option<ObjCId>) {
    // IDA 0x1a4d0: retained-property store via `objc_setProperty`
    // (0x1a4ec). Verified via IDA decompile.
    delegate.set_window(window);
}

// 0x1a4f4 — -[AppDelegate .cxx_destruct]
// mangled: -[AppDelegate .cxx_destruct]
// type: void __cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate .cxx_destruct]")]
pub fn stub_1a4f4(delegate: &AppDelegate) {
    // IDA 0x1a4f4: `connection::disconnect` (0x1a552) + weak-slot release
    // (0x1a558..0x1a560). Verified via IDA decompile.
    delegate.cxx_destruct();
}

// 0x1a5bc — -[AppDelegate .cxx_construct]
// mangled: -[AppDelegate .cxx_construct]
// type: id __cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate .cxx_construct]")]
pub fn stub_1a5bc(delegate: &AppDelegate) {
    // IDA 0x1a5bc: zeroes `messageOutConnection.con.weak_slot.p_` (0x1a5ca),
    // returns self (0x1a5cc; the host returns `()`). Verified via IDA decompile.
    delegate.cxx_construct();
}

// 0x1a5d0 — __GLOBAL__I_a_1
// mangled: __GLOBAL__I_a_1
// type:
#[doc(alias = "global constructor keyed to_a_1")]
pub fn stub_1a5d0() {
    // IDA 0x1a5d0 (`__GLOBAL__I_a_1`): `generic_category()` x2 +
    // `system_category()` stores, `std::ios_base::Init` with `__cxa_atexit`
    // teardown, guarded statics. Host statics initialize on use; nothing to
    // run. Same shape as 0x16e4c. Verified via IDA disasm.
}

// 0x1a768 — _main
// mangled: _main
// type: int __fastcall(int argc, const char **argv, const char **envp)
#[doc(alias = "_main")]
pub fn stub_1a768(delegate: &AppDelegate, argc: i32) -> i32 {
    // IDA 0x1a768: `NSAutoreleasePool` alloc/init (0x1a788..0x1a7a0),
    // `UIApplicationMain(argc, argv, @"UIApplication", @"AppDelegate")`
    // (0x1a7ba), pool release (0x1a7ca), return status (0x1a7d0). The UIKit
    // event loop has no host counterpart; the observable half is the launch
    // sequence, whose YES/NO maps to exit 0/1. Verified via IDA decompile.
    let _ = argc;
    if delegate.application_did_finish_launching() {
        0
    } else {
        1
    }
}
// 0x1a7d4 — __GLOBAL__I_a_2
// mangled: __GLOBAL__I_a_2
// type:
#[doc(alias = "global constructor keyed to_a_2")]
pub fn stub_1a7d4() {
    // IDA 0x1a7d4 (`__GLOBAL__I_a_2`): same `generic_category()` x2 +
    // `system_category()` + `ios_base::Init` shape as 0x1a5d0/0x16e4c. Host
    // statics initialize on use; nothing to run. Verified via IDA disasm.
}
// 0x1a970 — -[DebugSettingsViewController initWithCoder:]
// mangled: -[DebugSettingsViewController initWithCoder:]
// DebugSettingsViewController model — nib-loaded debug panel (IDA 0x1a970..0x1aed0).
// UIKit views have no host counterpart; the model keeps the nib-loaded state
// (`window` frame, `keyboardOffset`, `displayPickerArray`) plus the
// `RBX::GuiBuilder` debug-display mode the panel edits.
#[derive(Debug, Default)]
pub struct DebugSettingsViewController {
    window_frame: parking_lot::Mutex<(f64, f64, f64, f64)>,
    keyboard_offset: std::sync::atomic::AtomicI32,
    display_picker_items: parking_lot::Mutex<Vec<String>>,
    debug_display: std::sync::atomic::AtomicI32,
    view_did_load_calls: std::sync::atomic::AtomicU32,
    animation_runs: std::sync::atomic::AtomicU32,
}
impl DebugSettingsViewController {
    #[doc(alias = "-[DebugSettingsViewController initWithCoder:]")]
    #[doc = "-[DebugSettingsViewController initWithCoder:]"]
    pub fn init_with_coder(
        super_ok: bool,
        idiom_pad: bool,
        screen_bounds: Option<(f64, f64, f64, f64)>,
    ) -> Option<Self> {
        // Super `initWithCoder:` first (0x1a98e..0x1a99c); nil stays nil (0x1a9a0).
        if !super_ok {
            return None; // IDA 0x1ab1c
        }
        // iPad (`userInterfaceIdiom != 0`, 0x1a9f4) gets the fixed
        // 540x508 panel (0x1aa1c..0x1aa1e); otherwise the main-screen bounds
        // (0x1aa4e..0x1aa76). IDA 0x1a9c0..0x1aa76.
        let frame = if idiom_pad {
            (0.0, 0.0, 540.0, 508.0)
        } else {
            screen_bounds.unwrap_or_default()
        };
        let this = Self {
            window_frame: parking_lot::Mutex::new(frame), // IDA 0x1aa76
            keyboard_offset: std::sync::atomic::AtomicI32::new(114), // IDA 0x1aa7a
            display_picker_items: parking_lot::Mutex::new(
                ["None", "FPS", "Summary", "Physics", "PhysicsAndOwner", "Render"]
                    .into_iter()
                    .map(str::to_owned)
                    .collect(),
            ), // IDA 0x1aaa2..0x1ab12
            ..Self::default()
        };
        Some(this) // IDA 0x1ab12
    }
    #[doc(alias = "-[DebugSettingsViewController dealloc]")]
    #[doc = "-[DebugSettingsViewController dealloc]"]
    pub fn dealloc(self) {
        // `-[NSArray release]` the picker array (0x1ab42), then super
        // dealloc (0x1ab5a..0x1ab64, runs as drop).
    }
    #[doc(alias = "-[DebugSettingsViewController reloadOldData]")]
    #[doc = "-[DebugSettingsViewController reloadOldData]"]
    pub fn reload_old_data(&self) {
        // IDA 0x1ab6c: empty body.
    }
    #[doc(alias = "-[DebugSettingsViewController viewDidLoad]")]
    #[doc = "-[DebugSettingsViewController viewDidLoad]"]
    pub fn view_did_load(&self) {
        // Super `viewDidLoad` (0x1ab8c..0x1ab96) then `reloadOldData`
        // (0x1aba8). IDA 0x1ab70.
        self.view_did_load_calls
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.reload_old_data();
    }
    /// `RBX::GuiBuilder::getDebugDisplay` label mapping (IDA 0x1abe6..0x1ac02).
    pub fn display_label(&self) -> &'static str {
        match self.debug_display.load(std::sync::atomic::Ordering::SeqCst) {
            1 => "FPS",       // IDA 0x1ac22
            2 => "Summary",   // IDA 0x1ac38
            3 => "Physics",   // IDA 0x1ac4e
            4 => "PhysicsAndOwner", // IDA 0x1ac64
            5 => "Render",    // IDA 0x1ac7a
            _ => "None",      // IDA 0x1ac02
        }
    }
    #[doc(alias = "-[DebugSettingsViewController setDisplayUI]")]
    #[doc = "-[DebugSettingsViewController setDisplayUI]"]
    pub fn set_display_ui(&self) -> &'static str {
        // `viewWithTag:100` (0x1abc0..0x1abd2) is always present on the host;
        // the switch result is `setText:` (0x1ac0c). IDA 0x1abb0.
        self.display_label()
    }
    #[doc(alias = "-[DebugSettingsViewController displayPickerDoneClicked:]")]
    #[doc = "-[DebugSettingsViewController displayPickerDoneClicked:]"]
    pub fn display_picker_done_clicked(&self, selected_row: i32) -> &'static str {
        // Both tag views present (0x1ac9c..0x1ad0) gates the
        // `animateWithDuration:animations:` dispatch (0x1ad0a..0x1ad34),
        // recorded here; `selectedRowInComponent:0 >= 0` stores the debug
        // display (0x1ad4e..0x1ad50); finishes with `setDisplayUI`
        // (0x1ad62). IDA 0x1ac80.
        self.animation_runs
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        if selected_row >= 0 {
            self.debug_display
                .store(selected_row, std::sync::atomic::Ordering::SeqCst);
        }
        self.set_display_ui()
    }
    #[doc(alias = "___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke")]
    #[doc = "___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke"]
    pub fn display_picker_animation_frame(&self) {
        // Frame shuffle between the picker, self and the toolbar
        // (0x1ad90..0x1ae74): pure UIKit geometry, recorded. IDA 0x1ad78.
        self.animation_runs
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    #[doc(alias = "-[DebugSettingsViewController displayTouchUp:]")]
    #[doc = "-[DebugSettingsViewController displayTouchUp:]"]
    pub fn display_touch_up(&self) {
        // Same tag lookup + animation dispatch as done-clicked
        // (0x1aeec..0x1af86) without the picker read or `setDisplayUI`.
        // IDA 0x1aed0.
        self.animation_runs
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn window_frame(&self) -> (f64, f64, f64, f64) {
        *self.window_frame.lock()
    }
    pub fn keyboard_offset(&self) -> i32 {
        self.keyboard_offset.load(std::sync::atomic::Ordering::SeqCst)
    }
    pub fn animation_run_count(&self) -> u32 {
        self.animation_runs.load(std::sync::atomic::Ordering::SeqCst)
    }
    pub fn view_did_load_count(&self) -> u32 {
        self.view_did_load_calls.load(std::sync::atomic::Ordering::SeqCst)
    }
}
// type: DebugSettingsViewController *__cdecl(DebugSettingsViewController *self, SEL, id)
#[doc(alias = "-[DebugSettingsViewController initWithCoder:]")]
pub fn stub_1a970(
    super_ok: bool,
    idiom_pad: bool,
    screen_bounds: Option<(f64, f64, f64, f64)>,
) -> Option<DebugSettingsViewController> {
    // IDA 0x1a970: super init (0x1a98e..0x1a99c), iPad fixed frame vs screen
    // bounds (0x1a9c0..0x1aa76), `keyboardOffset = 114` (0x1aa7a), six-item
    // picker array (0x1aaa2..0x1ab12). Verified via IDA decompile.
    DebugSettingsViewController::init_with_coder(super_ok, idiom_pad, screen_bounds)
}

// 0x1ab20 — -[DebugSettingsViewController dealloc]
// mangled: -[DebugSettingsViewController dealloc]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController dealloc]")]
pub fn stub_1ab20(controller: DebugSettingsViewController) {
    // IDA 0x1ab20: picker-array release (0x1ab42) + super dealloc
    // (0x1ab5a..0x1ab64, runs as drop). Verified via IDA decompile.
    controller.dealloc();
}

// 0x1ab6c — -[DebugSettingsViewController reloadOldData]
// mangled: -[DebugSettingsViewController reloadOldData]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController reloadOldData]")]
pub fn stub_1ab6c(controller: &DebugSettingsViewController) {
    // IDA 0x1ab6c: empty body. Verified via IDA decompile.
    controller.reload_old_data();
}

// 0x1ab70 — -[DebugSettingsViewController viewDidLoad]
// mangled: -[DebugSettingsViewController viewDidLoad]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController viewDidLoad]")]
pub fn stub_1ab70(controller: &DebugSettingsViewController) {
    // IDA 0x1ab70: super `viewDidLoad` (0x1ab8c..0x1ab96) then
    // `reloadOldData` (0x1aba8). Verified via IDA decompile.
    controller.view_did_load();
}

// 0x1abb0 — -[DebugSettingsViewController setDisplayUI]
// mangled: -[DebugSettingsViewController setDisplayUI]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController setDisplayUI]")]
pub fn stub_1abb0(controller: &DebugSettingsViewController) -> &'static str {
    // IDA 0x1abb0: `viewWithTag:100` (0x1abc0..0x1abd2) then the
    // `getDebugDisplay` switch (0x1abe6..0x1ac02) into `setText:` (0x1ac0c).
    // Returns the label. Verified via IDA decompile.
    controller.set_display_ui()
}

// 0x1ac80 — -[DebugSettingsViewController displayPickerDoneClicked:]
// mangled: -[DebugSettingsViewController displayPickerDoneClicked:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, id)
#[doc(alias = "-[DebugSettingsViewController displayPickerDoneClicked:]")]
pub fn stub_1ac80(controller: &DebugSettingsViewController, selected_row: i32) -> &'static str {
    // IDA 0x1ac80: tag-5012/5011 lookup (0x1ac9c..0x1ad0), animation dispatch
    // (0x1ad0a..0x1ad34), `selectedRowInComponent:0 >= 0` store (0x1ad4e),
    // `setDisplayUI` (0x1ad62). Returns the label. Verified via IDA decompile.
    controller.display_picker_done_clicked(selected_row)
}

// 0x1ad78 — ___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke
// mangled: ___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke")]
pub fn stub_1ad78(controller: &DebugSettingsViewController) {
    // IDA 0x1ad78: `setFrame:` shuffle over the picker/self/toolbar frames
    // (0x1ad90..0x1ae74). Pure UIKit geometry, recorded. Verified via IDA
    // decompile.
    controller.display_picker_animation_frame();
}

// 0x1ae78 — ___copy_helper_block__0
// mangled: ___copy_helper_block__0
// type: void __fastcall(int, const void **)
#[doc(alias = "___copy_helper_block__0")]
pub fn stub_1ae78(
    picker_slot: &mut Option<ObjCId>,
    self_slot: &mut Option<ObjCId>,
    toolbar_slot: &mut Option<ObjCId>,
    picker_src: Option<ObjCId>,
    self_src: Option<ObjCId>,
    toolbar_src: Option<ObjCId>,
) {
    // IDA 0x1ae78: `_Block_object_assign` x2 (0x1ae88..0x1ae94) +
    // `_Block_object_assign` shim (0x1aea4); `Arc` clones retain.
    // Verified via IDA decompile.
    *picker_slot = picker_src;
    *self_slot = self_src;
    *toolbar_slot = toolbar_src;
}

// 0x1aea8 — ___destroy_helper_block__0
// mangled: ___destroy_helper_block__0
// type:
#[doc(alias = "___destroy_helper_block__0")]
pub fn stub_1aea8(
    picker_slot: &mut Option<ObjCId>,
    self_slot: &mut Option<ObjCId>,
    toolbar_slot: &mut Option<ObjCId>,
) {
    // IDA 0x1aea8: `_Block_object_dispose` x2 (0x1aeb2..0x1aeba) + dispose
    // shim (0x1aec6); dropping the host slots is the release. Verified via
    // IDA decompile.
    *picker_slot = None;
    *self_slot = None;
    *toolbar_slot = None;
}

// 0x1aed0 — -[DebugSettingsViewController displayTouchUp:]
// mangled: -[DebugSettingsViewController displayTouchUp:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, id)
#[doc(alias = "-[DebugSettingsViewController displayTouchUp:]")]
pub fn stub_1aed0(controller: &DebugSettingsViewController) {
    // IDA 0x1aed0: same tag lookup + animation dispatch as 0x1ac80
    // (0x1aeec..0x1af86), without the picker store. Verified via IDA decompile.
    controller.display_touch_up();
}

#[cfg(test)]
mod appirater_tests {
    use super::*;
    use crate::view_controllers::NIL_ID;

    #[test]
    fn rating_prompt_chain() {
        // Config setters round-trip (IDA 0x17df0..0x17e58).
        stub_17df0("431946152");
        stub_17e00(3.0);
        stub_17e14(10);
        stub_17e24(5);
        stub_17e34(10.0);
        stub_17e48(false);
        stub_17e58(4242);
        assert_eq!(Appirater::app_id(), "431946152");
        assert_eq!(Appirater::days_until_prompt(), 3.0);
        assert_eq!(Appirater::uses_until_prompt(), 10);
        assert_eq!(Appirater::significant_events_until_prompt(), 5);
        assert_eq!(Appirater::time_before_reminding(), 10.0);
        assert!(!Appirater::is_debug());
        assert_eq!(Appirater::class_delegate(), 4242);
        // `sharedInstance` (0x17f80) + its `dispatch_once` block (0x17fe4)
        let inst = stub_17f80();
        assert_eq!(inst.delegate(), NIL_ID);
        assert_eq!(stub_17fe4().delegate(), 4242);
        let inst = Appirater::shared_instance();
        assert_eq!(inst.delegate(), 4242);
        assert!(Appirater::resign_active_observed());
        // Reachability default (0x17e68).
        assert!(stub_17e68(inst));
        Appirater::set_network_reachable(false);
        assert!(!stub_17e68(inst));
        Appirater::set_network_reachable(true);
        // Debug forces the conditions (0x183d8 fast path).
        stub_17e48(true);
        assert!(stub_183d8(inst, 1_700_000_000.0));
        stub_17e48(false);
        // Fresh counters sit below the configured thresholds.
        assert!(!stub_183d8(inst, 1_700_000_000.0));
        // Loosen every gate, then walk the prompt path end to end.
        stub_17e00(0.0);
        stub_17e14(0);
        stub_17e24(0);
        stub_17e34(0.0);
        let now = 1_700_000_000.0;
        stub_185b0(inst, "2.125", now);
        stub_18878(inst, "2.125", now);
        assert_eq!(inst.use_count(), 1);
        assert_eq!(inst.significant_event_count(), 1);
        assert!(stub_183d8(inst, now + 1.0));
        // `incrementAndRate:YES` (0x18b18) shows the alert on the main-queue
        // block (0x18bb4); the block helpers retain/release the capture.
        let shows = inst.rating_alert_show_count();
        stub_18b18(inst, true, "2.125", now + 1.0);
        assert_eq!(inst.rating_alert_show_count(), shows + 1);
        assert!(inst.is_rating_alert_visible());
        stub_18bb4(inst);
        assert_eq!(inst.rating_alert_show_count(), shows + 2);
        let mut slot: Option<ObjCId> = None;
        stub_18094(&mut slot, Some(7));
        assert_eq!(slot, Some(7));
        stub_18bc8(&mut slot, Some(9));
        assert_eq!(slot, Some(9));
        stub_180a0(&mut slot);
        assert_eq!(slot, None);
        stub_18bd4(&mut slot);
        stub_18c8c(&mut slot, Some(11));
        stub_18c98(&mut slot);
        assert_eq!(slot, None);
        // Significant-event variant gates identically (0x18bdc/0x18c78).
        stub_18bdc(inst, true, "2.125", now + 2.0);
        stub_18c78(inst);
        // Hide reports the dismissal (0x18d4c/0x18dbc).
        assert!(stub_18d4c(inst));
        assert!(!inst.is_rating_alert_visible());
        assert!(!stub_18d4c(inst));
        stub_18dbc();
        // `rateApp` (0x18f24) substitutes the stored app id into the 0x18f6e
        // template and flags the version.
        let url = stub_18f24();
        assert!(url.ends_with("id=431946152"), "{url}");
        assert!(inst.rated_current_version());
        // Rating the version closes the gate again.
        assert!(!stub_183d8(inst, now + 3.0));
        // Alert-button dispatch (0x19028): decline flags, remind stamps.
        stub_19028(inst, 2, now + 4.0);
        assert_eq!(inst.last_alert_button(), 2);
        stub_19028(inst, 0, now + 5.0);
        assert!(inst.declined_to_rate());
        // Ivar accessors (0x191d4..0x19218).
        stub_191e4(inst, 77);
        assert_eq!(stub_191d4(inst), 77);
        stub_19218(inst, 4242);
        assert_eq!(stub_19208(inst), 4242);
        // `appLaunched:`/`appEnteredForeground:`/`userDidSignificantEvent:`
        // chains reach `incrementAndRate:` (0x18ca0..0x18ee8).
        stub_18ca0("2.125", now + 6.0);
        stub_18cc0(true, "2.125", now + 6.0);
        stub_18d10(true, "2.125", now + 6.0);
        stub_18e0c(true, "2.125", now + 6.0);
        stub_18e5c(true, "2.125", now + 6.0);
        stub_18e98(true, "2.125", now + 6.0);
        stub_18ee8(true, "2.125", now + 6.0);
        assert!(inst.use_count() >= 7);
        // Tuple ownership twins (0x179f4/0x17aac/0x17b80).
        let mut slot = SharedPtr::new(ReflectionTuple);
        stub_179f4(&mut slot);
        let adopted = stub_17aac(SharedPtr::new(ReflectionTuple));
        let copied = stub_17b80(&adopted);
        assert!(SharedPtr::ptr_eq(&adopted, &copied));
        // Empty DataModel leaves + global ctors run clean (0x16e4c/0x17c58,
        // 0x179e8..0x179f0).
        stub_16e4c();
        stub_17c58();
        stub_179e8();
        stub_179ec();
        stub_179f0();
    }
}

#[cfg(test)]
mod app_delegate_tests {
    use super::*;
    use crate::view_controllers::APP_PLACE_ID;
    #[test]
    fn app_delegate_lifecycle_chain() {
        // `init` (0x19228) builds without ivar stores; accessors round-trip.
        let delegate = stub_19228();
        assert_eq!(stub_1a494(&delegate), 0);
        stub_1a4a8(&delegate, 7);
        assert_eq!(stub_1a494(&delegate), 7);
        assert_eq!(stub_1a4c0(&delegate), None);
        stub_1a4d0(&delegate, Some(9));
        assert_eq!(stub_1a4c0(&delegate), Some(9));
        stub_1a4d0(&delegate, None);
        // `cxx_construct`/`cxx_destruct` (0x1a5bc/0x1a4f4) run clean.
        stub_1a5bc(&delegate);
        stub_1a4f4(&delegate);
        // Launch blocks (0x194ec/0x19514) configure Flurry + Appirater.
        stub_194ec();
        stub_19514();
        assert_eq!(Appirater::app_id(), "431946152");
        // Full launch (0x192b4) returns YES; `main` (0x1a768) maps it to 0.
        assert!(stub_192b4(&delegate));
        assert_eq!(stub_1a768(&delegate, 1), 0);
        // Lifecycle callbacks run clean.
        stub_195a0(&delegate);
        stub_196e4(&delegate);
        stub_19a30(&delegate);
        stub_19b60(&delegate);
        stub_19cdc(&delegate);
        stub_19f34();
        stub_19f7c(&delegate);
        // `openURL:` (0x1a174) rejects foreign schemes, stashes place ids.
        assert!(!stub_1a174(&delegate, "https://x/y", "x", "/y", "", ""));
        assert!(stub_1a174(
            &delegate,
            "robloxmobile://12345",
            "12345",
            "/",
            "",
            ""
        ));
        assert_eq!(
            APP_PLACE_ID.load(std::sync::atomic::Ordering::SeqCst),
            12345
        );
        APP_PLACE_ID.store(0, std::sync::atomic::Ordering::SeqCst);
        // `TryLaunchPlace:` (0x1a234) dispatches on the top controller class.
        assert_eq!(
            stub_1a234(&delegate, 99, "RobloxNavBarViewController"),
            LaunchAction::GameStarted
        );
        assert_eq!(
            stub_1a234(&delegate, 99, "BogusController"),
            LaunchAction::Unknown
        );
        // `dealloc` (0x19254) consumes.
        stub_19254(stub_19228());
        // Global ctors (0x1a5d0/0x1a7d4) run clean.
        stub_1a5d0();
        stub_1a7d4();
    }
    #[test]
    fn top_most_controller_chain() {
        // Presented chain 1 -> 2 -> 3 resolves to 3 (IDA 0x1a098).
        let graph = ViewControllerGraph::new();
        graph.present(1, 2);
        graph.present(2, 3);
        assert_eq!(stub_1a098(&graph, 1), Some(3));
        // Bare root returns nil.
        assert_eq!(stub_1a098(&graph, 7), None);
        // Navigation controller resolves to its visible controller.
        graph.mark_navigation_controller(10);
        graph.set_visible_view_controller(10, 11);
        graph.present(9, 10);
        assert_eq!(stub_1a098(&graph, 9), Some(11));
        // `topMostController()` (0x1a124) loops to the chain end.
        assert_eq!(stub_1a124(&graph, 1), 3);
        assert_eq!(stub_1a124(&graph, 7), 7);
    }
    #[test]
    fn debug_settings_panel() {
        // Nil super init stays nil (IDA 0x1a9a0..0x1ab1c).
        assert!(stub_1a970(false, true, None).is_none());
        // iPad fixed frame + six picker items (0x1aa1c..0x1ab12).
        let pad = stub_1a970(true, true, None).expect("pad init");
        assert_eq!(pad.window_frame(), (0.0, 0.0, 540.0, 508.0));
        assert_eq!(pad.keyboard_offset(), 114);
        assert_eq!(pad.display_picker_items.lock().len(), 6);
        // Phone takes the main-screen bounds.
        let phone = stub_1a970(true, false, Some((1.0, 2.0, 3.0, 4.0))).expect("phone init");
        assert_eq!(phone.window_frame(), (1.0, 2.0, 3.0, 4.0));
        // `viewDidLoad` (0x1ab70) runs `reloadOldData` (0x1ab6c, empty).
        stub_1ab70(&pad);
        assert_eq!(pad.view_did_load_count(), 1);
        stub_1ab6c(&pad);
        // `setDisplayUI` (0x1abb0) label switch incl. default.
        assert_eq!(stub_1abb0(&pad), "None");
        assert_eq!(stub_1ac80(&pad, 3), "Physics");
        assert_eq!(stub_1abb0(&pad), "Physics");
        // Negative row keeps the stored mode.
        assert_eq!(stub_1ac80(&pad, -1), "Physics");
        assert_eq!(stub_1ac80(&pad, 5), "Render");
        // Animation block (0x1ad78) + touch-up (0x1aed0) record runs.
        let runs = pad.animation_run_count();
        stub_1ad78(&pad);
        stub_1aed0(&pad);
        assert_eq!(pad.animation_run_count(), runs + 2);
        // Block copy/destroy helpers (0x1ae78/0x1aea8) retain/release slots.
        let (mut a, mut b, mut c): (Option<ObjCId>, Option<ObjCId>, Option<ObjCId>) =
            (None, None, None);
        stub_1ae78(&mut a, &mut b, &mut c, Some(1), Some(2), Some(3));
        assert_eq!((a, b, c), (Some(1), Some(2), Some(3)));
        stub_1aea8(&mut a, &mut b, &mut c);
        assert_eq!((a, b, c), (None, None, None));
        // `dealloc` (0x1ab20) consumes.
        stub_1ab20(phone);
        stub_1ab20(pad);
    }
}
