//! platform generated_176 — next 100 stubs EA-sorted asc global filler continuation after 0x16640 (global 500->600, rbx_core::SharedPtr not boost)
//! Filter: global EA-sorted asc, rbx_core::SharedPtr not boost
//! Batch: 100 stubs EA-sorted asc | skeleton batch | range 0x16730..0x1aed0 (rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use std::collections::BTreeMap;
use std::sync::LazyLock;
use super::generated_171::{RenderEnumDesc, RenderSettingsItem};
use super::view_controllers::{Appirater, ObjCId};
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
pub fn stub_19228() -> ! {
    todo!("0x19228 -[AppDelegate init]")
}

// 0x19254 — -[AppDelegate dealloc]
// mangled: -[AppDelegate dealloc]
// type: void __cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate dealloc]")]
pub fn stub_19254() -> ! {
    todo!("0x19254 -[AppDelegate dealloc]")
}

// 0x192b4 — -[AppDelegate application:didFinishLaunchingWithOptions:]
// mangled: -[AppDelegate application:didFinishLaunchingWithOptions:]
// type: char __cdecl(AppDelegate *self, SEL, id, id)
#[doc(alias = "-[AppDelegate application:didFinishLaunchingWithOptions:]")]
pub fn stub_192b4() -> ! {
    todo!("0x192b4 -[AppDelegate application:didFinishLaunchingWithOptions:]")
}

// 0x194ec — ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke
// mangled: ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke")]
pub fn stub_194ec() -> ! {
    todo!("0x194ec ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke")
}

// 0x19514 — ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2
// mangled: ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2
// type: void __cdecl(id)
#[doc(alias = "___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2")]
pub fn stub_19514() -> ! {
    todo!("0x19514 ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2")
}

// 0x195a0 — -[AppDelegate applicationWillResignActive:]
// mangled: -[AppDelegate applicationWillResignActive:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationWillResignActive:]")]
pub fn stub_195a0() -> ! {
    todo!("0x195a0 -[AppDelegate applicationWillResignActive:]")
}

// 0x196e4 — -[AppDelegate applicationDidEnterBackground:]
// mangled: -[AppDelegate applicationDidEnterBackground:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationDidEnterBackground:]")]
pub fn stub_196e4() -> ! {
    todo!("0x196e4 -[AppDelegate applicationDidEnterBackground:]")
}

// 0x19a30 — -[AppDelegate applicationDidReceiveMemoryWarning:]
// mangled: -[AppDelegate applicationDidReceiveMemoryWarning:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationDidReceiveMemoryWarning:]")]
pub fn stub_19a30() -> ! {
    todo!("0x19a30 -[AppDelegate applicationDidReceiveMemoryWarning:]")
}

// 0x19b60 — -[AppDelegate applicationWillEnterForeground:]
// mangled: -[AppDelegate applicationWillEnterForeground:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationWillEnterForeground:]")]
pub fn stub_19b60() -> ! {
    todo!("0x19b60 -[AppDelegate applicationWillEnterForeground:]")
}

// 0x19cdc — -[AppDelegate applicationDidBecomeActive:]
// mangled: -[AppDelegate applicationDidBecomeActive:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationDidBecomeActive:]")]
pub fn stub_19cdc() -> ! {
    todo!("0x19cdc -[AppDelegate applicationDidBecomeActive:]")
}

// 0x19f34 — ___42-[AppDelegate applicationDidBecomeActive:]_block_invoke
// mangled: ___42-[AppDelegate applicationDidBecomeActive:]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___42-[AppDelegate applicationDidBecomeActive:]_block_invoke")]
pub fn stub_19f34() -> ! {
    todo!("0x19f34 ___42-[AppDelegate applicationDidBecomeActive:]_block_invoke")
}

// 0x19f7c — -[AppDelegate applicationWillTerminate:]
// mangled: -[AppDelegate applicationWillTerminate:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate applicationWillTerminate:]")]
pub fn stub_19f7c() -> ! {
    todo!("0x19f7c -[AppDelegate applicationWillTerminate:]")
}

// 0x1a098 — __Z18_topMostControllerP16UIViewController
// mangled: __Z18_topMostControllerP16UIViewController
// type: id __fastcall(id)
#[doc(alias = "_topMostController(UIViewController *)")]
pub fn stub_1a098() -> ! {
    todo!("0x1a098 _topMostController(UIViewController *)")
}

// 0x1a124 — __Z17topMostControllerv
// mangled: __Z17topMostControllerv
// type: _DWORD __fastcall()
#[doc(alias = "topMostController(void)")]
pub fn stub_1a124() -> ! {
    todo!("0x1a124 topMostController(void)")
}

// 0x1a174 — -[AppDelegate application:openURL:sourceApplication:annotation:]
// mangled: -[AppDelegate application:openURL:sourceApplication:annotation:]
// type: char __cdecl(AppDelegate *self, SEL, id, id, id, id)
#[doc(alias = "-[AppDelegate application:openURL:sourceApplication:annotation:]")]
pub fn stub_1a174() -> ! {
    todo!("0x1a174 -[AppDelegate application:openURL:sourceApplication:annotation:]")
}

// 0x1a234 — -[AppDelegate TryLaunchPlace:]
// mangled: -[AppDelegate TryLaunchPlace:]
// type: void __cdecl(AppDelegate *self, SEL, int)
#[doc(alias = "-[AppDelegate TryLaunchPlace:]")]
pub fn stub_1a234() -> ! {
    todo!("0x1a234 -[AppDelegate TryLaunchPlace:]")
}

// 0x1a494 — -[AppDelegate bgTask]
// mangled: -[AppDelegate bgTask]
// type: unsigned int __cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate bgTask]")]
pub fn stub_1a494() -> ! {
    todo!("0x1a494 -[AppDelegate bgTask]")
}

// 0x1a4a8 — -[AppDelegate setBgTask:]
// mangled: -[AppDelegate setBgTask:]
// type: void __cdecl(AppDelegate *self, SEL, unsigned int)
#[doc(alias = "-[AppDelegate setBgTask:]")]
pub fn stub_1a4a8() -> ! {
    todo!("0x1a4a8 -[AppDelegate setBgTask:]")
}

// 0x1a4c0 — -[AppDelegate window]
// mangled: -[AppDelegate window]
// type: UIWindow *__cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate window]")]
pub fn stub_1a4c0() -> ! {
    todo!("0x1a4c0 -[AppDelegate window]")
}

// 0x1a4d0 — -[AppDelegate setWindow:]
// mangled: -[AppDelegate setWindow:]
// type: void __cdecl(AppDelegate *self, SEL, id)
#[doc(alias = "-[AppDelegate setWindow:]")]
pub fn stub_1a4d0() -> ! {
    todo!("0x1a4d0 -[AppDelegate setWindow:]")
}

// 0x1a4f4 — -[AppDelegate .cxx_destruct]
// mangled: -[AppDelegate .cxx_destruct]
// type: void __cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate .cxx_destruct]")]
pub fn stub_1a4f4() -> ! {
    todo!("0x1a4f4 -[AppDelegate .cxx_destruct]")
}

// 0x1a5bc — -[AppDelegate .cxx_construct]
// mangled: -[AppDelegate .cxx_construct]
// type: id __cdecl(AppDelegate *self, SEL)
#[doc(alias = "-[AppDelegate .cxx_construct]")]
pub fn stub_1a5bc() -> ! {
    todo!("0x1a5bc -[AppDelegate .cxx_construct]")
}

// 0x1a5d0 — __GLOBAL__I_a_1
// mangled: __GLOBAL__I_a_1
// type: 
#[doc(alias = "global constructor keyed to_a_1")]
pub fn stub_1a5d0() -> ! {
    todo!("0x1a5d0 global constructor keyed to_a_1")
}

// 0x1a768 — _main
// mangled: _main
// type: int __fastcall(int argc, const char **argv, const char **envp)
#[doc(alias = "_main")]
pub fn stub_1a768() -> ! {
    todo!("0x1a768 _main")
}

// 0x1a7d4 — __GLOBAL__I_a_2
// mangled: __GLOBAL__I_a_2
// type: 
#[doc(alias = "global constructor keyed to_a_2")]
pub fn stub_1a7d4() -> ! {
    todo!("0x1a7d4 global constructor keyed to_a_2")
}

// 0x1a970 — -[DebugSettingsViewController initWithCoder:]
// mangled: -[DebugSettingsViewController initWithCoder:]
// type: DebugSettingsViewController *__cdecl(DebugSettingsViewController *self, SEL, id)
#[doc(alias = "-[DebugSettingsViewController initWithCoder:]")]
pub fn stub_1a970() -> ! {
    todo!("0x1a970 -[DebugSettingsViewController initWithCoder:]")
}

// 0x1ab20 — -[DebugSettingsViewController dealloc]
// mangled: -[DebugSettingsViewController dealloc]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController dealloc]")]
pub fn stub_1ab20() -> ! {
    todo!("0x1ab20 -[DebugSettingsViewController dealloc]")
}

// 0x1ab6c — -[DebugSettingsViewController reloadOldData]
// mangled: -[DebugSettingsViewController reloadOldData]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController reloadOldData]")]
pub fn stub_1ab6c() -> ! {
    todo!("0x1ab6c -[DebugSettingsViewController reloadOldData]")
}

// 0x1ab70 — -[DebugSettingsViewController viewDidLoad]
// mangled: -[DebugSettingsViewController viewDidLoad]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController viewDidLoad]")]
pub fn stub_1ab70() -> ! {
    todo!("0x1ab70 -[DebugSettingsViewController viewDidLoad]")
}

// 0x1abb0 — -[DebugSettingsViewController setDisplayUI]
// mangled: -[DebugSettingsViewController setDisplayUI]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
#[doc(alias = "-[DebugSettingsViewController setDisplayUI]")]
pub fn stub_1abb0() -> ! {
    todo!("0x1abb0 -[DebugSettingsViewController setDisplayUI]")
}

// 0x1ac80 — -[DebugSettingsViewController displayPickerDoneClicked:]
// mangled: -[DebugSettingsViewController displayPickerDoneClicked:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, id)
#[doc(alias = "-[DebugSettingsViewController displayPickerDoneClicked:]")]
pub fn stub_1ac80() -> ! {
    todo!("0x1ac80 -[DebugSettingsViewController displayPickerDoneClicked:]")
}

// 0x1ad78 — ___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke
// mangled: ___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke")]
pub fn stub_1ad78() -> ! {
    todo!("0x1ad78 ___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke")
}

// 0x1ae78 — ___copy_helper_block__0
// mangled: ___copy_helper_block__0
// type: void __fastcall(int, const void **)
#[doc(alias = "___copy_helper_block__0")]
pub fn stub_1ae78() -> ! {
    todo!("0x1ae78 ___copy_helper_block__0")
}

// 0x1aea8 — ___destroy_helper_block__0
// mangled: ___destroy_helper_block__0
// type: 
#[doc(alias = "___destroy_helper_block__0")]
pub fn stub_1aea8() -> ! {
    todo!("0x1aea8 ___destroy_helper_block__0")
}

// 0x1aed0 — -[DebugSettingsViewController displayTouchUp:]
// mangled: -[DebugSettingsViewController displayTouchUp:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, id)
#[doc(alias = "-[DebugSettingsViewController displayTouchUp:]")]
pub fn stub_1aed0() -> ! {
    todo!("0x1aed0 -[DebugSettingsViewController displayTouchUp:]")
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
