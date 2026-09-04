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
pub fn stub_185b0() -> ! {
    todo!("0x185b0 -[Appirater incrementUseCount]")
}

// 0x18878 — -[Appirater incrementSignificantEventCount]
// mangled: -[Appirater incrementSignificantEventCount]
// type: void __cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater incrementSignificantEventCount]")]
pub fn stub_18878() -> ! {
    todo!("0x18878 -[Appirater incrementSignificantEventCount]")
}

// 0x18b18 — -[Appirater incrementAndRate:]
// mangled: -[Appirater incrementAndRate:]
// type: void __cdecl(Appirater *self, SEL, char)
#[doc(alias = "-[Appirater incrementAndRate:]")]
pub fn stub_18b18() -> ! {
    todo!("0x18b18 -[Appirater incrementAndRate:]")
}

// 0x18bb4 — ___30-[Appirater incrementAndRate:]_block_invoke
// mangled: ___30-[Appirater incrementAndRate:]_block_invoke
// type: 
#[doc(alias = "___30-[Appirater incrementAndRate:]_block_invoke")]
pub fn stub_18bb4() -> ! {
    todo!("0x18bb4 ___30-[Appirater incrementAndRate:]_block_invoke")
}

// 0x18bc8 — ___copy_helper_block_125
// mangled: ___copy_helper_block_125
// type: 
#[doc(alias = "___copy_helper_block_125")]
pub fn stub_18bc8() -> ! {
    todo!("0x18bc8 ___copy_helper_block_125")
}

// 0x18bd4 — ___destroy_helper_block_126
// mangled: ___destroy_helper_block_126
// type: 
#[doc(alias = "___destroy_helper_block_126")]
pub fn stub_18bd4() -> ! {
    todo!("0x18bd4 ___destroy_helper_block_126")
}

// 0x18bdc — -[Appirater incrementSignificantEventAndRate:]
// mangled: -[Appirater incrementSignificantEventAndRate:]
// type: void __cdecl(Appirater *self, SEL, char)
#[doc(alias = "-[Appirater incrementSignificantEventAndRate:]")]
pub fn stub_18bdc() -> ! {
    todo!("0x18bdc -[Appirater incrementSignificantEventAndRate:]")
}

// 0x18c78 — ___46-[Appirater incrementSignificantEventAndRate:]_block_invoke
// mangled: ___46-[Appirater incrementSignificantEventAndRate:]_block_invoke
// type: 
#[doc(alias = "___46-[Appirater incrementSignificantEventAndRate:]_block_invoke")]
pub fn stub_18c78() -> ! {
    todo!("0x18c78 ___46-[Appirater incrementSignificantEventAndRate:]_block_invoke")
}

// 0x18c8c — ___copy_helper_block_130
// mangled: ___copy_helper_block_130
// type: 
#[doc(alias = "___copy_helper_block_130")]
pub fn stub_18c8c() -> ! {
    todo!("0x18c8c ___copy_helper_block_130")
}

// 0x18c98 — ___destroy_helper_block_131
// mangled: ___destroy_helper_block_131
// type: 
#[doc(alias = "___destroy_helper_block_131")]
pub fn stub_18c98() -> ! {
    todo!("0x18c98 ___destroy_helper_block_131")
}

// 0x18ca0 — +[Appirater appLaunched]
// mangled: +[Appirater appLaunched]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[Appirater appLaunched]")]
pub fn stub_18ca0() -> ! {
    todo!("0x18ca0 +[Appirater appLaunched]")
}

// 0x18cc0 — +[Appirater appLaunched:]
// mangled: +[Appirater appLaunched:]
// type: void __cdecl(id, SEL, char)
#[doc(alias = "+[Appirater appLaunched:]")]
pub fn stub_18cc0() -> ! {
    todo!("0x18cc0 +[Appirater appLaunched:]")
}

// 0x18d10 — ___25+[Appirater appLaunched:]_block_invoke
// mangled: ___25+[Appirater appLaunched:]_block_invoke
// type: 
#[doc(alias = "___25+[Appirater appLaunched:]_block_invoke")]
pub fn stub_18d10() -> ! {
    todo!("0x18d10 ___25+[Appirater appLaunched:]_block_invoke")
}

// 0x18d4c — -[Appirater hideRatingAlert]
// mangled: -[Appirater hideRatingAlert]
// type: void __cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater hideRatingAlert]")]
pub fn stub_18d4c() -> ! {
    todo!("0x18d4c -[Appirater hideRatingAlert]")
}

// 0x18dbc — +[Appirater appWillResignActive]
// mangled: +[Appirater appWillResignActive]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[Appirater appWillResignActive]")]
pub fn stub_18dbc() -> ! {
    todo!("0x18dbc +[Appirater appWillResignActive]")
}

// 0x18e0c — +[Appirater appEnteredForeground:]
// mangled: +[Appirater appEnteredForeground:]
// type: void __cdecl(id, SEL, char)
#[doc(alias = "+[Appirater appEnteredForeground:]")]
pub fn stub_18e0c() -> ! {
    todo!("0x18e0c +[Appirater appEnteredForeground:]")
}

// 0x18e5c — ___34+[Appirater appEnteredForeground:]_block_invoke
// mangled: ___34+[Appirater appEnteredForeground:]_block_invoke
// type: 
#[doc(alias = "___34+[Appirater appEnteredForeground:]_block_invoke")]
pub fn stub_18e5c() -> ! {
    todo!("0x18e5c ___34+[Appirater appEnteredForeground:]_block_invoke")
}

// 0x18e98 — +[Appirater userDidSignificantEvent:]
// mangled: +[Appirater userDidSignificantEvent:]
// type: void __cdecl(id, SEL, char)
#[doc(alias = "+[Appirater userDidSignificantEvent:]")]
pub fn stub_18e98() -> ! {
    todo!("0x18e98 +[Appirater userDidSignificantEvent:]")
}

// 0x18ee8 — ___37+[Appirater userDidSignificantEvent:]_block_invoke
// mangled: ___37+[Appirater userDidSignificantEvent:]_block_invoke
// type: 
#[doc(alias = "___37+[Appirater userDidSignificantEvent:]_block_invoke")]
pub fn stub_18ee8() -> ! {
    todo!("0x18ee8 ___37+[Appirater userDidSignificantEvent:]_block_invoke")
}

// 0x18f24 — +[Appirater rateApp]
// mangled: +[Appirater rateApp]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[Appirater rateApp]")]
pub fn stub_18f24() -> ! {
    todo!("0x18f24 +[Appirater rateApp]")
}

// 0x19028 — -[Appirater alertView:clickedButtonAtIndex:]
// mangled: -[Appirater alertView:clickedButtonAtIndex:]
// type: void __cdecl(Appirater *self, SEL, id, int)
#[doc(alias = "-[Appirater alertView:clickedButtonAtIndex:]")]
pub fn stub_19028() -> ! {
    todo!("0x19028 -[Appirater alertView:clickedButtonAtIndex:]")
}

// 0x191d4 — -[Appirater ratingAlert]
// mangled: -[Appirater ratingAlert]
// type: UIAlertView *__cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater ratingAlert]")]
pub fn stub_191d4() -> ! {
    todo!("0x191d4 -[Appirater ratingAlert]")
}

// 0x191e4 — -[Appirater setRatingAlert:]
// mangled: -[Appirater setRatingAlert:]
// type: void __cdecl(Appirater *self, SEL, id)
#[doc(alias = "-[Appirater setRatingAlert:]")]
pub fn stub_191e4() -> ! {
    todo!("0x191e4 -[Appirater setRatingAlert:]")
}

// 0x19208 — -[Appirater delegate]
// mangled: -[Appirater delegate]
// type: AppiraterDelegate *__cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater delegate]")]
pub fn stub_19208() -> ! {
    todo!("0x19208 -[Appirater delegate]")
}

// 0x19218 — -[Appirater setDelegate:]
// mangled: -[Appirater setDelegate:]
// type: void __cdecl(Appirater *self, SEL, id)
#[doc(alias = "-[Appirater setDelegate:]")]
pub fn stub_19218() -> ! {
    todo!("0x19218 -[Appirater setDelegate:]")
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
