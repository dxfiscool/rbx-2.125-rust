//! core shard FA — 100 core stubs EA-sorted, lowest uncovered 0xeb23b8..0xf26524 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after EZ 0xeb2280).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xeb2280.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::IndexArray<RBX::IAdornable,&RBX::IAdornable::indexFunc2d>::fastRemove(RBX::IAdornable*)")]
// 0xeb23b8 — __ZN3RBX10IndexArrayINS_10IAdornableEXadL_ZNS1_11indexFunc2dEvEEE10fastRemoveEPS1_
pub fn stub_eb23b8() -> ! {
    todo!("0xeb23b8 __ZN3RBX10IndexArrayINS_10IAdornableEXadL_ZNS1_11indexFunc2dEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::IndexArray<RBX::IAdornable,&RBX::IAdornable::indexFunc3d>::fastRemove(RBX::IAdornable*)")]
// 0xeb248c — __ZN3RBX10IndexArrayINS_10IAdornableEXadL_ZNS1_11indexFunc3dEvEEE10fastRemoveEPS1_
pub fn stub_eb248c() -> ! {
    todo!("0xeb248c __ZN3RBX10IndexArrayINS_10IAdornableEXadL_ZNS1_11indexFunc3dEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::IndexArray<RBX::IAdornable,&RBX::IAdornable::indexFunc3dSorted>::fastRemove(RBX::IAdornable*)")]
// 0xeb2560 — __ZN3RBX10IndexArrayINS_10IAdornableEXadL_ZNS1_17indexFunc3dSortedEvEEE10fastRemoveEPS1_
pub fn stub_eb2560() -> ! {
    todo!("0xeb2560 __ZN3RBX10IndexArrayINS_10IAdornableEXadL_ZNS1_17indexFunc3dSortedEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,RBX::IAdornable * const&)")]
// 0xeb2634 — __ZNSt6vectorIPN3RBX10IAdornableESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_eb2634() -> ! {
    todo!("0xeb2634 __ZNSt6vectorIPN3RBX10IAdornableESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "RBX::RenderCaps::RenderCaps(std::string,unsigned long)")]
// 0xeb2e10 — __ZN3RBX10RenderCapsC1ESsm
pub fn stub_eb2e10() -> ! {
    todo!("0xeb2e10 __ZN3RBX10RenderCapsC1ESsm")
}

#[doc(alias = "RBX::RenderCaps::setSoftwareVertexProcessing(bool)")]
// 0xeb2e40 — __ZN3RBX10RenderCaps27setSoftwareVertexProcessingEb
pub fn stub_eb2e40() -> ! {
    todo!("0xeb2e40 __ZN3RBX10RenderCaps27setSoftwareVertexProcessingEb")
}

#[doc(alias = "RBX::RenderCaps::getMaxTextureResolution(void)const")]
// 0xeb2e44 — __ZNK3RBX10RenderCaps23getMaxTextureResolutionEv
pub fn stub_eb2e44() -> ! {
    todo!("0xeb2e44 __ZNK3RBX10RenderCaps23getMaxTextureResolutionEv")
}

#[doc(alias = "RBX::CRenderSettings::CRenderSettings(void)")]
// 0xeb2e90 — __ZN3RBX15CRenderSettingsC2Ev
pub fn stub_eb2e90() -> ! {
    todo!("0xeb2e90 __ZN3RBX15CRenderSettingsC2Ev")
}

#[doc(alias = "RBX::CRenderSettings::setAutoQualityLevel(int)")]
// 0xeb2ef8 — __ZN3RBX15CRenderSettings19setAutoQualityLevelEi
pub fn stub_eb2ef8() -> ! {
    todo!("0xeb2ef8 __ZN3RBX15CRenderSettings19setAutoQualityLevelEi")
}

#[doc(alias = "RBX::RenderStats::RenderStats(void)")]
// 0xeb2f48 — __ZN3RBX11RenderStatsC1Ev
pub fn stub_eb2f48() -> ! {
    todo!("0xeb2f48 __ZN3RBX11RenderStatsC1Ev")
}

#[doc(alias = "RBX::RenderStats::RenderStats(void)")]
// 0xeb2f54 — __ZN3RBX11RenderStatsC2Ev
pub fn stub_eb2f54() -> ! {
    todo!("0xeb2f54 __ZN3RBX11RenderStatsC2Ev")
}

#[doc(alias = "RBX::RenderStats::~RenderStats()")]
// 0xeb351c — __ZN3RBX11RenderStatsD1Ev
pub fn stub_eb351c() -> ! {
    todo!("0xeb351c __ZN3RBX11RenderStatsD1Ev")
}

#[doc(alias = "RBX::RenderStats::~RenderStats()")]
// 0xeb3528 — __ZN3RBX11RenderStatsD2Ev
pub fn stub_eb3528() -> ! {
    todo!("0xeb3528 __ZN3RBX11RenderStatsD2Ev")
}

#[doc(alias = "RBX::ViewBase::CreateView(RBX::CRenderSettings::GraphicsMode,RBX::OSContext *,RBX::CRenderSettings*)")]
// 0xeb390c — __ZN3RBX8ViewBase10CreateViewENS_15CRenderSettings12GraphicsModeEPNS_9OSContextEPS1_
pub fn stub_eb390c() -> ! {
    todo!("0xeb390c __ZN3RBX8ViewBase10CreateViewENS_15CRenderSettings12GraphicsModeEPNS_9OSContextEPS1_")
}

#[doc(alias = "RBX::ViewBase::RegisterFactory(RBX::CRenderSettings::GraphicsMode,RBX::IViewBaseFactory *)")]
// 0xeb39a4 — __ZN3RBX8ViewBase15RegisterFactoryENS_15CRenderSettings12GraphicsModeEPNS_16IViewBaseFactoryE
pub fn stub_eb39a4() -> ! {
    todo!("0xeb39a4 __ZN3RBX8ViewBase15RegisterFactoryENS_15CRenderSettings12GraphicsModeEPNS_16IViewBaseFactoryE")
}

#[doc(alias = "RBX::ViewBase::render(RBX::IMetric *,double)")]
// 0xeb3a10 — __ZN3RBX8ViewBase6renderEPNS_7IMetricEd
pub fn stub_eb3a10() -> ! {
    todo!("0xeb3a10 __ZN3RBX8ViewBase6renderEPNS_7IMetricEd")
}

#[doc(alias = "RBX::ViewBase::InitPluginModules(void)")]
// 0xeb3a50 — __ZN3RBX8ViewBase17InitPluginModulesEv
pub fn stub_eb3a50() -> ! {
    todo!("0xeb3a50 __ZN3RBX8ViewBase17InitPluginModulesEv")
}

#[doc(alias = "RBX::ViewBase::setFullscreen(bool,int,int)")]
// 0xeb3a5c — __ZN3RBX8ViewBase13setFullscreenEbii
pub fn stub_eb3a5c() -> ! {
    todo!("0xeb3a5c __ZN3RBX8ViewBase13setFullscreenEbii")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::ShadowMode>(RBX::CRenderSettings::ShadowMode const&)")]
// 0xf25d44 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings10ShadowModeEEERS3_RKT_
pub fn stub_f25d44() -> ! {
    todo!("0xf25d44 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings10ShadowModeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::GraphicsMode>(RBX::CRenderSettings::GraphicsMode const&)")]
// 0xf25d54 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings12GraphicsModeEEERS3_RKT_
pub fn stub_f25d54() -> ! {
    todo!("0xf25d54 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings12GraphicsModeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::QualityLevel>(RBX::CRenderSettings::QualityLevel const&)")]
// 0xf25d64 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings12QualityLevelEEERS3_RKT_
pub fn stub_f25d64() -> ! {
    todo!("0xf25d64 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings12QualityLevelEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::AntialiasingMode>(RBX::CRenderSettings::AntialiasingMode const&)")]
// 0xf25d74 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings16AntialiasingModeEEERS3_RKT_
pub fn stub_f25d74() -> ! {
    todo!("0xf25d74 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings16AntialiasingModeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::ResolutionPreset>(RBX::CRenderSettings::ResolutionPreset const&)")]
// 0xf25d84 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings16ResolutionPresetEEERS3_RKT_
pub fn stub_f25d84() -> ! {
    todo!("0xf25d84 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings16ResolutionPresetEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::FrameRateManagerMode>(RBX::CRenderSettings::FrameRateManagerMode const&)")]
// 0xf25d94 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings20FrameRateManagerModeEEERS3_RKT_
pub fn stub_f25d94() -> ! {
    todo!("0xf25d94 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings20FrameRateManagerModeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::AASamples>(RBX::CRenderSettings::AASamples const&)")]
// 0xf25da4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings9AASamplesEEERS3_RKT_
pub fn stub_f25da4() -> ! {
    todo!("0xf25da4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings9AASamplesEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ShadowMode>::singleton(void)")]
// 0xf25db4 — j___ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings10ShadowModeEE9singletonEv
pub fn stub_f25db4() -> ! {
    todo!("0xf25db4 j___ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings10ShadowModeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::GraphicsMode>::singleton(void)")]
// 0xf25dc4 — j___ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12GraphicsModeEE9singletonEv
pub fn stub_f25dc4() -> ! {
    todo!("0xf25dc4 j___ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12GraphicsModeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::QualityLevel>::singleton(void)")]
// 0xf25dd4 — j___ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12QualityLevelEE9singletonEv
pub fn stub_f25dd4() -> ! {
    todo!("0xf25dd4 j___ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12QualityLevelEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AntialiasingMode>::singleton(void)")]
// 0xf25de4 — j___ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16AntialiasingModeEE9singletonEv
pub fn stub_f25de4() -> ! {
    todo!("0xf25de4 j___ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16AntialiasingModeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ResolutionPreset>::singleton(void)")]
// 0xf25df4 — j___ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16ResolutionPresetEE9singletonEv
pub fn stub_f25df4() -> ! {
    todo!("0xf25df4 j___ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16ResolutionPresetEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::FrameRateManagerMode>::singleton(void)")]
// 0xf25e04 — j___ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings20FrameRateManagerModeEE9singletonEv
pub fn stub_f25e04() -> ! {
    todo!("0xf25e04 j___ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings20FrameRateManagerModeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::singleton(void)")]
// 0xf25e14 — j___ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings9AASamplesEE9singletonEv
pub fn stub_f25e14() -> ! {
    todo!("0xf25e14 j___ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings9AASamplesEE9singletonEv")
}

#[doc(alias = "RBX::CRenderSettings::ShadowMode const& rbx::any_cast<RBX::CRenderSettings::ShadowMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf25e54 — j___ZN3rbx8any_castIRKN3RBX15CRenderSettings10ShadowModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f25e54() -> ! {
    todo!("0xf25e54 j___ZN3rbx8any_castIRKN3RBX15CRenderSettings10ShadowModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::CRenderSettings::GraphicsMode const& rbx::any_cast<RBX::CRenderSettings::GraphicsMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf25e64 — j___ZN3rbx8any_castIRKN3RBX15CRenderSettings12GraphicsModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f25e64() -> ! {
    todo!("0xf25e64 j___ZN3rbx8any_castIRKN3RBX15CRenderSettings12GraphicsModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::CRenderSettings::QualityLevel const& rbx::any_cast<RBX::CRenderSettings::QualityLevel const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf25e74 — j___ZN3rbx8any_castIRKN3RBX15CRenderSettings12QualityLevelENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f25e74() -> ! {
    todo!("0xf25e74 j___ZN3rbx8any_castIRKN3RBX15CRenderSettings12QualityLevelENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::CRenderSettings::AntialiasingMode const& rbx::any_cast<RBX::CRenderSettings::AntialiasingMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf25e84 — j___ZN3rbx8any_castIRKN3RBX15CRenderSettings16AntialiasingModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f25e84() -> ! {
    todo!("0xf25e84 j___ZN3rbx8any_castIRKN3RBX15CRenderSettings16AntialiasingModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::CRenderSettings::ResolutionPreset const& rbx::any_cast<RBX::CRenderSettings::ResolutionPreset const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf25e94 — j___ZN3rbx8any_castIRKN3RBX15CRenderSettings16ResolutionPresetENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f25e94() -> ! {
    todo!("0xf25e94 j___ZN3rbx8any_castIRKN3RBX15CRenderSettings16ResolutionPresetENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::CRenderSettings::FrameRateManagerMode const& rbx::any_cast<RBX::CRenderSettings::FrameRateManagerMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf25ea4 — j___ZN3rbx8any_castIRKN3RBX15CRenderSettings20FrameRateManagerModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f25ea4() -> ! {
    todo!("0xf25ea4 j___ZN3rbx8any_castIRKN3RBX15CRenderSettings20FrameRateManagerModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::CRenderSettings::AASamples const& rbx::any_cast<RBX::CRenderSettings::AASamples const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf25eb4 — j___ZN3rbx8any_castIRKN3RBX15CRenderSettings9AASamplesENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f25eb4() -> ! {
    todo!("0xf25eb4 j___ZN3rbx8any_castIRKN3RBX15CRenderSettings9AASamplesENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "boost::exception_detail::refcount_ptr<boost::exception_detail::error_info_container>::~refcount_ptr()")]
// 0xf25ed4 — j___ZN5boost16exception_detail12refcount_ptrINS0_20error_info_containerEED2Ev
pub fn stub_f25ed4() -> ! {
    todo!("0xf25ed4 j___ZN5boost16exception_detail12refcount_ptrINS0_20error_info_containerEED2Ev")
}

#[doc(alias = "boost::detail::sp_counted_base::use_count(void)const")]
// 0xf26134 — j___ZNK5boost6detail15sp_counted_base9use_countEv
pub fn stub_f26134() -> ! {
    todo!("0xf26134 j___ZNK5boost6detail15sp_counted_base9use_countEv")
}

#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_allocate(unsigned long)")]
// 0xf26154 — j___ZNSt12_Vector_baseIN3RBX15CRenderSettings10ShadowModeESaIS2_EE11_M_allocateEm
pub fn stub_f26154() -> ! {
    todo!("0xf26154 j___ZNSt12_Vector_baseIN3RBX15CRenderSettings10ShadowModeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_allocate(unsigned long)")]
// 0xf26164 — j___ZNSt12_Vector_baseIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE11_M_allocateEm
pub fn stub_f26164() -> ! {
    todo!("0xf26164 j___ZNSt12_Vector_baseIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_allocate(unsigned long)")]
// 0xf26174 — j___ZNSt12_Vector_baseIN3RBX15CRenderSettings12QualityLevelESaIS2_EE11_M_allocateEm
pub fn stub_f26174() -> ! {
    todo!("0xf26174 j___ZNSt12_Vector_baseIN3RBX15CRenderSettings12QualityLevelESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_allocate(unsigned long)")]
// 0xf26184 — j___ZNSt12_Vector_baseIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE11_M_allocateEm
pub fn stub_f26184() -> ! {
    todo!("0xf26184 j___ZNSt12_Vector_baseIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_allocate(unsigned long)")]
// 0xf26194 — j___ZNSt12_Vector_baseIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE11_M_allocateEm
pub fn stub_f26194() -> ! {
    todo!("0xf26194 j___ZNSt12_Vector_baseIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_allocate(unsigned long)")]
// 0xf261a4 — j___ZNSt12_Vector_baseIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE11_M_allocateEm
pub fn stub_f261a4() -> ! {
    todo!("0xf261a4 j___ZNSt12_Vector_baseIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_allocate(unsigned long)")]
// 0xf261b4 — j___ZNSt12_Vector_baseIN3RBX15CRenderSettings9AASamplesESaIS2_EE11_M_allocateEm
pub fn stub_f261b4() -> ! {
    todo!("0xf261b4 j___ZNSt12_Vector_baseIN3RBX15CRenderSettings9AASamplesESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::CRenderSettings::ShadowMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *>(RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *)")]
// 0xf261d4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings10ShadowModeES6_EET0_T_S8_S7_
pub fn stub_f261d4() -> ! {
    todo!("0xf261d4 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings10ShadowModeES6_EET0_T_S8_S7_")
}

#[doc(alias = "RBX::CRenderSettings::GraphicsMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *>(RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *)")]
// 0xf261e4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings12GraphicsModeES6_EET0_T_S8_S7_
pub fn stub_f261e4() -> ! {
    todo!("0xf261e4 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings12GraphicsModeES6_EET0_T_S8_S7_")
}

#[doc(alias = "RBX::CRenderSettings::QualityLevel * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *>(RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *)")]
// 0xf261f4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings12QualityLevelES6_EET0_T_S8_S7_
pub fn stub_f261f4() -> ! {
    todo!("0xf261f4 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings12QualityLevelES6_EET0_T_S8_S7_")
}

#[doc(alias = "RBX::CRenderSettings::AntialiasingMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *>(RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *)")]
// 0xf26204 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings16AntialiasingModeES6_EET0_T_S8_S7_
pub fn stub_f26204() -> ! {
    todo!("0xf26204 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings16AntialiasingModeES6_EET0_T_S8_S7_")
}

#[doc(alias = "RBX::CRenderSettings::ResolutionPreset * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *>(RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *)")]
// 0xf26214 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings16ResolutionPresetES6_EET0_T_S8_S7_
pub fn stub_f26214() -> ! {
    todo!("0xf26214 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings16ResolutionPresetES6_EET0_T_S8_S7_")
}

#[doc(alias = "RBX::CRenderSettings::FrameRateManagerMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *>(RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *)")]
// 0xf26224 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings20FrameRateManagerModeES6_EET0_T_S8_S7_
pub fn stub_f26224() -> ! {
    todo!("0xf26224 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings20FrameRateManagerModeES6_EET0_T_S8_S7_")
}

#[doc(alias = "RBX::CRenderSettings::AASamples * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *>(RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *)")]
// 0xf26234 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings9AASamplesES6_EET0_T_S8_S7_
pub fn stub_f26234() -> ! {
    todo!("0xf26234 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings9AASamplesES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::ShadowMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::operator[](RBX::Name const* const&)")]
// 0xf26244 — j___ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings10ShadowModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f26244() -> ! {
    todo!("0xf26244 j___ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings10ShadowModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::GraphicsMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::operator[](RBX::Name const* const&)")]
// 0xf26254 — j___ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings12GraphicsModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f26254() -> ! {
    todo!("0xf26254 j___ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings12GraphicsModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::QualityLevel,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::operator[](RBX::Name const* const&)")]
// 0xf26264 — j___ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings12QualityLevelESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f26264() -> ! {
    todo!("0xf26264 j___ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings12QualityLevelESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::AntialiasingMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::operator[](RBX::Name const* const&)")]
// 0xf26274 — j___ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings16AntialiasingModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f26274() -> ! {
    todo!("0xf26274 j___ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings16AntialiasingModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::ResolutionPreset,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::operator[](RBX::Name const* const&)")]
// 0xf26284 — j___ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings16ResolutionPresetESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f26284() -> ! {
    todo!("0xf26284 j___ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings16ResolutionPresetESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::FrameRateManagerMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::operator[](RBX::Name const* const&)")]
// 0xf26294 — j___ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings20FrameRateManagerModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f26294() -> ! {
    todo!("0xf26294 j___ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings20FrameRateManagerModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::AASamples,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::operator[](RBX::Name const* const&)")]
// 0xf262a4 — j___ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings9AASamplesESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f262a4() -> ! {
    todo!("0xf262a4 j___ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings9AASamplesESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ShadowMode*,std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>>,RBX::CRenderSettings::ShadowMode const&)")]
// 0xf262d4 — j___ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f262d4() -> ! {
    todo!("0xf262d4 j___ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ShadowMode*,std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>>,unsigned long,RBX::CRenderSettings::ShadowMode const&)")]
// 0xf262e4 — j___ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f262e4() -> ! {
    todo!("0xf262e4 j___ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::resize(unsigned long,RBX::CRenderSettings::ShadowMode)")]
// 0xf262f4 — j___ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE6resizeEmS2_
pub fn stub_f262f4() -> ! {
    todo!("0xf262f4 j___ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::push_back(RBX::CRenderSettings::ShadowMode const&)")]
// 0xf26304 — j___ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE9push_backERKS2_
pub fn stub_f26304() -> ! {
    todo!("0xf26304 j___ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::GraphicsMode*,std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>>,RBX::CRenderSettings::GraphicsMode const&)")]
// 0xf26314 — j___ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f26314() -> ! {
    todo!("0xf26314 j___ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::GraphicsMode*,std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>>,unsigned long,RBX::CRenderSettings::GraphicsMode const&)")]
// 0xf26324 — j___ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f26324() -> ! {
    todo!("0xf26324 j___ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::resize(unsigned long,RBX::CRenderSettings::GraphicsMode)")]
// 0xf26334 — j___ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE6resizeEmS2_
pub fn stub_f26334() -> ! {
    todo!("0xf26334 j___ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::push_back(RBX::CRenderSettings::GraphicsMode const&)")]
// 0xf26344 — j___ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE9push_backERKS2_
pub fn stub_f26344() -> ! {
    todo!("0xf26344 j___ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::QualityLevel*,std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>>,RBX::CRenderSettings::QualityLevel const&)")]
// 0xf26354 — j___ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f26354() -> ! {
    todo!("0xf26354 j___ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::QualityLevel*,std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>>,unsigned long,RBX::CRenderSettings::QualityLevel const&)")]
// 0xf26364 — j___ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f26364() -> ! {
    todo!("0xf26364 j___ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::resize(unsigned long,RBX::CRenderSettings::QualityLevel)")]
// 0xf26374 — j___ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE6resizeEmS2_
pub fn stub_f26374() -> ! {
    todo!("0xf26374 j___ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::push_back(RBX::CRenderSettings::QualityLevel const&)")]
// 0xf26384 — j___ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE9push_backERKS2_
pub fn stub_f26384() -> ! {
    todo!("0xf26384 j___ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AntialiasingMode*,std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>>,RBX::CRenderSettings::AntialiasingMode const&)")]
// 0xf26394 — j___ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f26394() -> ! {
    todo!("0xf26394 j___ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AntialiasingMode*,std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>>,unsigned long,RBX::CRenderSettings::AntialiasingMode const&)")]
// 0xf263a4 — j___ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f263a4() -> ! {
    todo!("0xf263a4 j___ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::resize(unsigned long,RBX::CRenderSettings::AntialiasingMode)")]
// 0xf263b4 — j___ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE6resizeEmS2_
pub fn stub_f263b4() -> ! {
    todo!("0xf263b4 j___ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::push_back(RBX::CRenderSettings::AntialiasingMode const&)")]
// 0xf263c4 — j___ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE9push_backERKS2_
pub fn stub_f263c4() -> ! {
    todo!("0xf263c4 j___ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ResolutionPreset*,std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>>,RBX::CRenderSettings::ResolutionPreset const&)")]
// 0xf263d4 — j___ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f263d4() -> ! {
    todo!("0xf263d4 j___ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ResolutionPreset*,std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>>,unsigned long,RBX::CRenderSettings::ResolutionPreset const&)")]
// 0xf263e4 — j___ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f263e4() -> ! {
    todo!("0xf263e4 j___ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::resize(unsigned long,RBX::CRenderSettings::ResolutionPreset)")]
// 0xf263f4 — j___ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE6resizeEmS2_
pub fn stub_f263f4() -> ! {
    todo!("0xf263f4 j___ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::push_back(RBX::CRenderSettings::ResolutionPreset const&)")]
// 0xf26404 — j___ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE9push_backERKS2_
pub fn stub_f26404() -> ! {
    todo!("0xf26404 j___ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::FrameRateManagerMode*,std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>>,RBX::CRenderSettings::FrameRateManagerMode const&)")]
// 0xf26414 — j___ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f26414() -> ! {
    todo!("0xf26414 j___ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::FrameRateManagerMode*,std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>>,unsigned long,RBX::CRenderSettings::FrameRateManagerMode const&)")]
// 0xf26424 — j___ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f26424() -> ! {
    todo!("0xf26424 j___ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::resize(unsigned long,RBX::CRenderSettings::FrameRateManagerMode)")]
// 0xf26434 — j___ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE6resizeEmS2_
pub fn stub_f26434() -> ! {
    todo!("0xf26434 j___ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::push_back(RBX::CRenderSettings::FrameRateManagerMode const&)")]
// 0xf26444 — j___ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE9push_backERKS2_
pub fn stub_f26444() -> ! {
    todo!("0xf26444 j___ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AASamples*,std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>>,RBX::CRenderSettings::AASamples const&)")]
// 0xf26454 — j___ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f26454() -> ! {
    todo!("0xf26454 j___ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AASamples*,std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>>,unsigned long,RBX::CRenderSettings::AASamples const&)")]
// 0xf26464 — j___ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f26464() -> ! {
    todo!("0xf26464 j___ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::resize(unsigned long,RBX::CRenderSettings::AASamples)")]
// 0xf26474 — j___ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE6resizeEmS2_
pub fn stub_f26474() -> ! {
    todo!("0xf26474 j___ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::push_back(RBX::CRenderSettings::AASamples const&)")]
// 0xf26484 — j___ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE9push_backERKS2_
pub fn stub_f26484() -> ! {
    todo!("0xf26484 j___ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)")]
// 0xf26494 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f26494() -> ! {
    todo!("0xf26494 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)")]
// 0xf264a4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f264a4() -> ! {
    todo!("0xf264a4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>> *)")]
// 0xf264b4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f264b4() -> ! {
    todo!("0xf264b4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)")]
// 0xf264c4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f264c4() -> ! {
    todo!("0xf264c4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)")]
// 0xf264d4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f264d4() -> ! {
    todo!("0xf264d4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)")]
// 0xf264e4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f264e4() -> ! {
    todo!("0xf264e4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>> *)")]
// 0xf264f4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f264f4() -> ! {
    todo!("0xf264f4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)")]
// 0xf26504 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f26504() -> ! {
    todo!("0xf26504 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)")]
// 0xf26514 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f26514() -> ! {
    todo!("0xf26514 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)")]
// 0xf26524 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f26524() -> ! {
    todo!("0xf26524 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}
