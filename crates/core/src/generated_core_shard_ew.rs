//! core shard EW — 100 core stubs EA-sorted, lowest uncovered 0xbeef64..0xc08fec (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after EV 0xbeeab8).
//! Source: `ida/export.json` filtered where demangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xbeeab8.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::ViewRbxGfx::writeRTToFile(std::string const&)")]
// 0xbeef64 — __ZN3RBX10ViewRbxGfx13writeRTToFileERKSs
pub fn stub_beef64() -> ! {
    todo!("0xbeef64 __ZN3RBX10ViewRbxGfx13writeRTToFileERKSs")
}

#[doc(alias = "RBX::ViewRbxGfx::writeRTToBuffer(unsigned char *,int,int,int)")]
// 0xbeefb0 — __ZN3RBX10ViewRbxGfx15writeRTToBufferEPhiii
pub fn stub_beefb0() -> ! {
    todo!("0xbeefb0 __ZN3RBX10ViewRbxGfx15writeRTToBufferEPhiii")
}

#[doc(alias = "RBX::ViewRbxGfx::startFrame(void)")]
// 0xbef174 — __ZN3RBX10ViewRbxGfx10startFrameEv
pub fn stub_bef174() -> ! {
    todo!("0xbef174 __ZN3RBX10ViewRbxGfx10startFrameEv")
}

#[doc(alias = "RBX::ViewRbxGfx::renderPrepare(RBX::IMetric *)::ProxyMetric::~ProxyMetric()")]
// 0xbef1f0 — __ZZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricEEN11ProxyMetricD0Ev
pub fn stub_bef1f0() -> ! {
    todo!("0xbef1f0 __ZZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricEEN11ProxyMetricD0Ev")
}

#[doc(alias = "RBX::ViewRbxGfx::renderPrepare(RBX::IMetric *)::ProxyMetric::getMetric(std::string const&)const")]
// 0xbef1f4 — __ZZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricEENK11ProxyMetric9getMetricERKSs
pub fn stub_bef1f4() -> ! {
    todo!("0xbef1f4 __ZZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricEENK11ProxyMetric9getMetricERKSs")
}

#[doc(alias = "RBX::ViewRbxGfx::renderPrepare(RBX::IMetric *)::ProxyMetric::getMetricValue(std::string const&)const")]
// 0xbef230 — __ZZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricEENK11ProxyMetric14getMetricValueERKSs
pub fn stub_bef230() -> ! {
    todo!("0xbef230 __ZZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricEENK11ProxyMetric14getMetricValueERKSs")
}

#[doc(alias = "RBX::ViewRbxGfx_InitModule(void)::ViewRbxGfxFactory::Create(RBX::CRenderSettings::GraphicsMode,RBX::OSContext *,RBX::CRenderSettings*)")]
// 0xbef270 — __ZZN3RBX21ViewRbxGfx_InitModuleEvEN17ViewRbxGfxFactory6CreateENS_15CRenderSettings12GraphicsModeEPNS_9OSContextEPS1_
pub fn stub_bef270() -> ! {
    todo!("0xbef270 __ZZN3RBX21ViewRbxGfx_InitModuleEvEN17ViewRbxGfxFactory6CreateENS_15CRenderSettings12GraphicsModeEPNS_9OSContextEPS1_")
}

#[doc(alias = "std::invalid_argument::~invalid_argument()")]
// 0xbef320 — __ZNSt16invalid_argumentD2Ev
pub fn stub_bef320() -> ! {
    todo!("0xbef320 __ZNSt16invalid_argumentD2Ev")
}

#[doc(alias = "RBX::WindowAverage<double,double>::getSanitizedStats(RBX::Confidence)const")]
// 0xbef380 — __ZNK3RBX13WindowAverageIddE17getSanitizedStatsENS_10ConfidenceE
pub fn stub_bef380() -> ! {
    todo!("0xbef380 __ZNK3RBX13WindowAverageIddE17getSanitizedStatsENS_10ConfidenceE")
}

#[doc(alias = "RBX::ViewRbxGfx::throttle(void)")]
// 0xbef8c0 — __ZN3RBX10ViewRbxGfx8throttleEv
pub fn stub_bef8c0() -> ! {
    todo!("0xbef8c0 __ZN3RBX10ViewRbxGfx8throttleEv")
}

#[doc(alias = "RBX::ViewRbxGfx::getAdorn(void)")]
// 0xbef8cc — __ZN3RBX10ViewRbxGfx8getAdornEv
pub fn stub_bef8cc() -> ! {
    todo!("0xbef8cc __ZN3RBX10ViewRbxGfx8getAdornEv")
}

#[doc(alias = "RBX::ViewBase::canSetFullscreen(void)")]
// 0xbef8d0 — __ZN3RBX8ViewBase16canSetFullscreenEv
pub fn stub_bef8d0() -> ! {
    todo!("0xbef8d0 __ZN3RBX8ViewBase16canSetFullscreenEv")
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::insert(rbx::signals::signal<void ()(int)>::slot *)")]
// 0xbf0af8 — __ZN3rbx7signals6signalIFviEE6insertEPNS3_4slotE
pub fn stub_bf0af8() -> ! {
    todo!("0xbf0af8 __ZN3rbx7signals6signalIFviEE6insertEPNS3_4slotE")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(int)>::slot>::operator=(rbx::signals::signal<void ()(int)>::slot*)")]
// 0xbf0dd8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviEE4slotEEaSEPS6_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(int)>::slot>::operator=(rbx::signals::signal<void ()(int)>::slot*)
pub fn stub_bf0dd8() -> ! {
    todo!("0xbf0dd8 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviEE4slotEEaSEPS6_")
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::safe_static_init_mutex(void)")]
// 0xbf0e90 — __ZN3rbx7signals6signalIFviEE22safe_static_init_mutexEv
pub fn stub_bf0e90() -> ! {
    todo!("0xbf0e90 __ZN3rbx7signals6signalIFviEE22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,int>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>>::~callable_slot()")]
// 0xbf0e94 — __ZN3rbx7signals6signalIFviEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX10ViewRbxGfxEiEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev
pub fn stub_bf0e94() -> ! {
    todo!("0xbf0e94 __ZN3rbx7signals6signalIFviEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX10ViewRbxGfxEiEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,int>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>>::~callable_slot()")]
// 0xbf0ef0 — __ZN3rbx7signals6signalIFviEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX10ViewRbxGfxEiEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev
pub fn stub_bf0ef0() -> ! {
    todo!("0xbf0ef0 __ZN3rbx7signals6signalIFviEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX10ViewRbxGfxEiEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::slot::disconnect(void)")]
// 0xbf0ff8 — __ZN3rbx7signals6signalIFviEE4slot10disconnectEv
pub fn stub_bf0ff8() -> ! {
    todo!("0xbf0ff8 __ZN3rbx7signals6signalIFviEE4slot10disconnectEv")
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::slot::connected(void)const")]
// 0xbf10e8 — __ZNK3rbx7signals6signalIFviEE4slot9connectedEv
pub fn stub_bf10e8() -> ! {
    todo!("0xbf10e8 __ZNK3rbx7signals6signalIFviEE4slot9connectedEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,int>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>,1,void ()(int)>::call(int)")]
// 0xbf10f4 — __ZN3rbx8callableINS_7signals6signalIFviEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX10ViewRbxGfxEiEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEi
pub fn stub_bf10f4() -> ! {
    todo!("0xbf10f4 __ZN3rbx8callableINS_7signals6signalIFviEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX10ViewRbxGfxEiEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEi")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,int>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>,1,void ()(int)>::call(int)")]
// 0xbf110c — __ZThn4_N3rbx8callableINS_7signals6signalIFviEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX10ViewRbxGfxEiEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEi
// was: `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,int>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>,1,void ()(int)>::call(int)
pub fn stub_bf110c() -> ! {
    todo!("0xbf110c __ZThn4_N3rbx8callableINS_7signals6signalIFviEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX10ViewRbxGfxEiEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEi")
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::slot::mutex(void)")]
// 0xbf1124 — __ZN3rbx7signals6signalIFviEE4slot5mutexEv
pub fn stub_bf1124() -> ! {
    todo!("0xbf1124 __ZN3rbx7signals6signalIFviEE4slot5mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::remove(rbx::signals::signal<void ()(int)>::slot *)")]
// 0xbf1230 — __ZN3rbx7signals6signalIFviEE6removeEPNS3_4slotE
pub fn stub_bf1230() -> ! {
    todo!("0xbf1230 __ZN3rbx7signals6signalIFviEE6removeEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::slot::safe_static_init_mutex(void)")]
// 0xbf1320 — __ZN3rbx7signals6signalIFviEE4slot22safe_static_init_mutexEv
pub fn stub_bf1320() -> ! {
    todo!("0xbf1320 __ZN3rbx7signals6signalIFviEE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::slot::~slot()")]
// 0xbf1408 — __ZN3rbx7signals6signalIFviEE4slotD1Ev
pub fn stub_bf1408() -> ! {
    todo!("0xbf1408 __ZN3rbx7signals6signalIFviEE4slotD1Ev")
}

#[doc(alias = "RBX::RenderHooksService * RBX::ServiceProvider::find<RBX::RenderHooksService>(void)const")]
// 0xbf1468 — __ZNK3RBX15ServiceProvider4findINS_18RenderHooksServiceEEEPT_v
pub fn stub_bf1468() -> ! {
    todo!("0xbf1468 __ZNK3RBX15ServiceProvider4findINS_18RenderHooksServiceEEEPT_v")
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ViewRbxGfx>,boost::_bi::list1<boost::_bi::value<RBX::ViewRbxGfx*>>>>::~callable_slot()")]
// 0xbf18ac — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX10ViewRbxGfxEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev
pub fn stub_bf18ac() -> ! {
    todo!("0xbf18ac __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX10ViewRbxGfxEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ViewRbxGfx>,boost::_bi::list1<boost::_bi::value<RBX::ViewRbxGfx*>>>>::~callable_slot()")]
// 0xbf1908 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX10ViewRbxGfxEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev
pub fn stub_bf1908() -> ! {
    todo!("0xbf1908 __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX10ViewRbxGfxEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ViewRbxGfx>,boost::_bi::list1<boost::_bi::value<RBX::ViewRbxGfx*>>>,0,void ()(void)>::call(void)")]
// 0xbf1a14 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX10ViewRbxGfxEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
pub fn stub_bf1a14() -> ! {
    todo!("0xbf1a14 __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX10ViewRbxGfxEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ViewRbxGfx>,boost::_bi::list1<boost::_bi::value<RBX::ViewRbxGfx*>>>,0,void ()(void)>::call(void)")]
// 0xbf1a2c — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX10ViewRbxGfxEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
// was: `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ViewRbxGfx>,boost::_bi::list1<boost::_bi::value<RBX::ViewRbxGfx*>>>,0,void ()(void)>::call(void)
pub fn stub_bf1a2c() -> ! {
    todo!("0xbf1a2c __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX10ViewRbxGfxEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(bool)>::slot>::operator=(rbx::signals::signal<void ()(bool)>::slot*)")]
// 0xbf1a48 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbEE4slotEEaSEPS6_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(bool)>::slot>::operator=(rbx::signals::signal<void ()(bool)>::slot*)
pub fn stub_bf1a48() -> ! {
    todo!("0xbf1a48 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbEE4slotEEaSEPS6_")
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,bool>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>>::~callable_slot()")]
// 0xbf1afc — __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX10ViewRbxGfxEbEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev
pub fn stub_bf1afc() -> ! {
    todo!("0xbf1afc __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX10ViewRbxGfxEbEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,bool>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>>::~callable_slot()")]
// 0xbf1b58 — __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX10ViewRbxGfxEbEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev
pub fn stub_bf1b58() -> ! {
    todo!("0xbf1b58 __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX10ViewRbxGfxEbEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::slot::disconnect(void)")]
// 0xbf1c60 — __ZN3rbx7signals6signalIFvbEE4slot10disconnectEv
pub fn stub_bf1c60() -> ! {
    todo!("0xbf1c60 __ZN3rbx7signals6signalIFvbEE4slot10disconnectEv")
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::slot::connected(void)const")]
// 0xbf1d50 — __ZNK3rbx7signals6signalIFvbEE4slot9connectedEv
pub fn stub_bf1d50() -> ! {
    todo!("0xbf1d50 __ZNK3rbx7signals6signalIFvbEE4slot9connectedEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,bool>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>,1,void ()(bool)>::call(bool)")]
// 0xbf1d5c — __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX10ViewRbxGfxEbEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEb
pub fn stub_bf1d5c() -> ! {
    todo!("0xbf1d5c __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX10ViewRbxGfxEbEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEb")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,bool>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>,1,void ()(bool)>::call(bool)")]
// 0xbf1d74 — __ZThn4_N3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX10ViewRbxGfxEbEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEb
// was: `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,bool>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>,1,void ()(bool)>::call(bool)
pub fn stub_bf1d74() -> ! {
    todo!("0xbf1d74 __ZThn4_N3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX10ViewRbxGfxEbEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEb")
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::slot::mutex(void)")]
// 0xbf1d8c — __ZN3rbx7signals6signalIFvbEE4slot5mutexEv
pub fn stub_bf1d8c() -> ! {
    todo!("0xbf1d8c __ZN3rbx7signals6signalIFvbEE4slot5mutexEv")
}

#[doc(alias = "RBX::TextService * RBX::ServiceProvider::create<RBX::TextService>(void)const")]
// 0xbf23c0 — __ZNK3RBX15ServiceProvider6createINS_11TextServiceEEEPT_v
pub fn stub_bf23c0() -> ! {
    todo!("0xbf23c0 __ZNK3RBX15ServiceProvider6createINS_11TextServiceEEEPT_v")
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::TextService>(void)")]
// 0xbf2770 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_11TextServiceEEEvv
pub fn stub_bf2770() -> ! {
    todo!("0xbf2770 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_11TextServiceEEEvv")
}

#[doc(alias = "boost::xtime_get(boost::xtime *,int)")]
// 0xbf2a9c — __ZN5boost9xtime_getEPNS_5xtimeEi
pub fn stub_bf2a9c() -> ! {
    todo!("0xbf2a9c __ZN5boost9xtime_getEPNS_5xtimeEi")
}

#[doc(alias = "void boost::this_thread::sleep<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>(boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll> const&)")]
// 0xbf2b84 — __ZN5boost11this_thread5sleepINS_9date_time18subsecond_durationINS_10posix_time13time_durationELx1000EEEEEvRKT_
pub fn stub_bf2b84() -> ! {
    todo!("0xbf2b84 __ZN5boost11this_thread5sleepINS_9date_time18subsecond_durationINS_10posix_time13time_durationELx1000EEEEEvRKT_")
}

#[doc(alias = "RBX::TextureContentProvider * RBX::ServiceProvider::create<RBX::TextureContentProvider>(void)const")]
// 0xbf2dc0 — __ZNK3RBX15ServiceProvider6createINS_22TextureContentProviderEEEPT_v
pub fn stub_bf2dc0() -> ! {
    todo!("0xbf2dc0 __ZNK3RBX15ServiceProvider6createINS_22TextureContentProviderEEEPT_v")
}

#[doc(alias = "RBX::TextureContentProvider * RBX::ServiceProvider::find<RBX::TextureContentProvider>(void)const")]
// 0xbf304c — __ZNK3RBX15ServiceProvider4findINS_22TextureContentProviderEEEPT_v
pub fn stub_bf304c() -> ! {
    todo!("0xbf304c __ZNK3RBX15ServiceProvider4findINS_22TextureContentProviderEEEPT_v")
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::TextureContentProvider>(void)")]
// 0xbf36b0 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_22TextureContentProviderEEEvv
pub fn stub_bf36b0() -> ! {
    todo!("0xbf36b0 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_22TextureContentProviderEEEvv")
}

#[doc(alias = "rbx::signals::scoped_connection::~scoped_connection()")]
// 0xbf3908 — __ZN3rbx7signals17scoped_connectionD2Ev
pub fn stub_bf3908() -> ! {
    todo!("0xbf3908 __ZN3rbx7signals17scoped_connectionD2Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::safe_static_init_mutex(void)")]
// 0xbf3ac8 — __ZN3rbx7signals6signalIFvSsEE22safe_static_init_mutexEv
pub fn stub_bf3ac8() -> ! {
    todo!("0xbf3ac8 __ZN3rbx7signals6signalIFvSsEE22safe_static_init_mutexEv")
}

#[doc(alias = "RBX::FontFactory::FontFactory(void)")]
// 0xbf4304 — __ZN3RBX11FontFactoryC1Ev
pub fn stub_bf4304() -> ! {
    todo!("0xbf4304 __ZN3RBX11FontFactoryC1Ev")
}

#[doc(alias = "RBX::FontFactory::FontFactory(void)")]
// 0xbf4308 — __ZN3RBX11FontFactoryC2Ev
pub fn stub_bf4308() -> ! {
    todo!("0xbf4308 __ZN3RBX11FontFactoryC2Ev")
}

#[doc(alias = "RBX::FontFactory::loadFont(RBX::Text::Font)")]
// 0xbf44a8 — __ZN3RBX11FontFactory8loadFontENS_4Text4FontE
pub fn stub_bf44a8() -> ! {
    todo!("0xbf44a8 __ZN3RBX11FontFactory8loadFontENS_4Text4FontE")
}

#[doc(alias = "RBX::FontFactory::~FontFactory()")]
// 0xbf58fc — __ZN3RBX11FontFactoryD1Ev
pub fn stub_bf58fc() -> ! {
    todo!("0xbf58fc __ZN3RBX11FontFactoryD1Ev")
}

#[doc(alias = "RBX::FontFactory::getTypesetter(RBX::Text::Font)")]
// 0xbf59d8 — __ZN3RBX11FontFactory13getTypesetterENS_4Text4FontE
pub fn stub_bf59d8() -> ! {
    todo!("0xbf59d8 __ZN3RBX11FontFactory13getTypesetterENS_4Text4FontE")
}

#[doc(alias = "RBX::FontFactory::getTexture(RBX::Text::Font,float)")]
// 0xbf5a34 — __ZN3RBX11FontFactory10getTextureENS_4Text4FontEf
pub fn stub_bf5a34() -> ! {
    todo!("0xbf5a34 __ZN3RBX11FontFactory10getTextureENS_4Text4FontEf")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TypesetterBitmap>::~sp_counted_impl_p()")]
// 0xbf5b50 — __ZN5boost6detail17sp_counted_impl_pIN3RBX16TypesetterBitmapEED1Ev
pub fn stub_bf5b50() -> ! {
    todo!("0xbf5b50 __ZN5boost6detail17sp_counted_impl_pIN3RBX16TypesetterBitmapEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TypesetterBitmap>::~sp_counted_impl_p()")]
// 0xbf5b54 — __ZN5boost6detail17sp_counted_impl_pIN3RBX16TypesetterBitmapEED0Ev
pub fn stub_bf5b54() -> ! {
    todo!("0xbf5b54 __ZN5boost6detail17sp_counted_impl_pIN3RBX16TypesetterBitmapEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TypesetterBitmap>::dispose(void)")]
// 0xbf5b58 — __ZN5boost6detail17sp_counted_impl_pIN3RBX16TypesetterBitmapEE7disposeEv
pub fn stub_bf5b58() -> ! {
    todo!("0xbf5b58 __ZN5boost6detail17sp_counted_impl_pIN3RBX16TypesetterBitmapEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TypesetterBitmap>::get_deleter(std::type_info const&)")]
// 0xbf5b68 — __ZN5boost6detail17sp_counted_impl_pIN3RBX16TypesetterBitmapEE11get_deleterERKSt9type_info
pub fn stub_bf5b68() -> ! {
    todo!("0xbf5b68 __ZN5boost6detail17sp_counted_impl_pIN3RBX16TypesetterBitmapEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TypesetterBitmap>::get_untyped_deleter(void)")]
// 0xbf5b6c — __ZN5boost6detail17sp_counted_impl_pIN3RBX16TypesetterBitmapEE19get_untyped_deleterEv
pub fn stub_bf5b6c() -> ! {
    todo!("0xbf5b6c __ZN5boost6detail17sp_counted_impl_pIN3RBX16TypesetterBitmapEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::TypesetterBitmap::TypesetterBitmap(std::string const&,std::string const&,float)")]
// 0xbf7df0 — __ZN3RBX16TypesetterBitmapC1ERKSsS2_f
pub fn stub_bf7df0() -> ! {
    todo!("0xbf7df0 __ZN3RBX16TypesetterBitmapC1ERKSsS2_f")
}

#[doc(alias = "RBX::TypesetterBitmap::TypesetterBitmap(std::string const&,std::string const&,float)")]
// 0xbf7df4 — __ZN3RBX16TypesetterBitmapC2ERKSsS2_f
pub fn stub_bf7df4() -> ! {
    todo!("0xbf7df4 __ZN3RBX16TypesetterBitmapC2ERKSsS2_f")
}

#[doc(alias = "RBX::TypesetterBitmap::~TypesetterBitmap()")]
// 0xbf96c0 — __ZN3RBX16TypesetterBitmapD1Ev
pub fn stub_bf96c0() -> ! {
    todo!("0xbf96c0 __ZN3RBX16TypesetterBitmapD1Ev")
}

#[doc(alias = "RBX::TypesetterBitmap::~TypesetterBitmap()")]
// 0xbf9784 — __ZN3RBX16TypesetterBitmapD0Ev
pub fn stub_bf9784() -> ! {
    todo!("0xbf9784 __ZN3RBX16TypesetterBitmapD0Ev")
}

#[doc(alias = "std::vector<RBX::TypesetterBitmap::GlyphLine,std::allocator<RBX::TypesetterBitmap::GlyphLine>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TypesetterBitmap::GlyphLine*,std::vector<RBX::TypesetterBitmap::GlyphLine,std::allocator<RBX::TypesetterBitmap::GlyphLine>>>,unsigned long,RBX::TypesetterBitmap::GlyphLine const&)")]
// 0xbf98bc — __ZNSt6vectorIN3RBX16TypesetterBitmap9GlyphLineESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_bf98bc() -> ! {
    todo!("0xbf98bc __ZNSt6vectorIN3RBX16TypesetterBitmap9GlyphLineESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::TypesetterBitmap::GlyphLine,std::allocator<RBX::TypesetterBitmap::GlyphLine>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TypesetterBitmap::GlyphLine*,std::vector<RBX::TypesetterBitmap::GlyphLine,std::allocator<RBX::TypesetterBitmap::GlyphLine>>>,RBX::TypesetterBitmap::GlyphLine const&)")]
// 0xbf9ab0 — __ZNSt6vectorIN3RBX16TypesetterBitmap9GlyphLineESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_bf9ab0() -> ! {
    todo!("0xbf9ab0 __ZNSt6vectorIN3RBX16TypesetterBitmap9GlyphLineESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::priority_queue<RBX::NodeInfo,std::vector<RBX::NodeInfo,std::allocator<RBX::NodeInfo>>,std::less<RBX::NodeInfo>>::push(RBX::NodeInfo const&)")]
// 0xbfc2e0 — __ZNSt14priority_queueIN3RBX8NodeInfoESt6vectorIS1_SaIS1_EESt4lessIS1_EE4pushERKS1_
pub fn stub_bfc2e0() -> ! {
    todo!("0xbfc2e0 __ZNSt14priority_queueIN3RBX8NodeInfoESt6vectorIS1_SaIS1_EESt4lessIS1_EE4pushERKS1_")
}

#[doc(alias = "std::vector<RBX::NodeInfo,std::allocator<RBX::NodeInfo>>::vector(std::vector<RBX::NodeInfo,std::allocator<RBX::NodeInfo>> const&)")]
// 0xbfddd8 — __ZNSt6vectorIN3RBX8NodeInfoESaIS1_EEC2ERKS3_
pub fn stub_bfddd8() -> ! {
    todo!("0xbfddd8 __ZNSt6vectorIN3RBX8NodeInfoESaIS1_EEC2ERKS3_")
}

#[doc(alias = "ResourceGroupHelper::resourceGroupExist(std::string const&)")]
// 0xc039f4 — __ZN19ResourceGroupHelper18resourceGroupExistERKSs
pub fn stub_c039f4() -> ! {
    todo!("0xc039f4 __ZN19ResourceGroupHelper18resourceGroupExistERKSs")
}

#[doc(alias = "ResourceGroupHelper::getLatestModificationTime(std::string const&)")]
// 0xc046d4 — __ZN19ResourceGroupHelper25getLatestModificationTimeERKSs
pub fn stub_c046d4() -> ! {
    todo!("0xc046d4 __ZN19ResourceGroupHelper25getLatestModificationTimeERKSs")
}

#[doc(alias = "ResourceGroupHelper::checkTimeAndReloadIfNeeded(std::string const&,std::string &,bool)")]
// 0xc04800 — __ZN19ResourceGroupHelper26checkTimeAndReloadIfNeededERKSsRSsb
pub fn stub_c04800() -> ! {
    todo!("0xc04800 __ZN19ResourceGroupHelper26checkTimeAndReloadIfNeededERKSsRSsb")
}

#[doc(alias = "std::map<std::string,long,std::less<std::string>,std::allocator<std::pair<std::string const,long>>>::operator[](std::string const&)")]
// 0xc04acc — __ZNSt3mapISslSt4lessISsESaISt4pairIKSslEEEixERS3_
pub fn stub_c04acc() -> ! {
    todo!("0xc04acc __ZNSt3mapISslSt4lessISsESaISt4pairIKSslEEEixERS3_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,long>,std::_Select1st<std::pair<std::string const,long>>,std::less<std::string>,std::allocator<std::pair<std::string const,long>>>::find(std::string const&)")]
// 0xc04c90 — __ZNSt8_Rb_treeISsSt4pairIKSslESt10_Select1stIS2_ESt4lessISsESaIS2_EE4findERS1_
pub fn stub_c04c90() -> ! {
    todo!("0xc04c90 __ZNSt8_Rb_treeISsSt4pairIKSslESt10_Select1stIS2_ESt4lessISsESaIS2_EE4findERS1_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,long>,std::_Select1st<std::pair<std::string const,long>>,std::less<std::string>,std::allocator<std::pair<std::string const,long>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,long>>,std::pair<std::string const,long> const&)")]
// 0xc04d34 — __ZNSt8_Rb_treeISsSt4pairIKSslESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
pub fn stub_c04d34() -> ! {
    todo!("0xc04d34 __ZNSt8_Rb_treeISsSt4pairIKSslESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,long>,std::_Select1st<std::pair<std::string const,long>>,std::less<std::string>,std::allocator<std::pair<std::string const,long>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,long> const&)")]
// 0xc04f14 — __ZNSt8_Rb_treeISsSt4pairIKSslESt10_Select1stIS2_ESt4lessISsESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
pub fn stub_c04f14() -> ! {
    todo!("0xc04f14 __ZNSt8_Rb_treeISsSt4pairIKSslESt10_Select1stIS2_ESt4lessISsESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,long>,std::_Select1st<std::pair<std::string const,long>>,std::less<std::string>,std::allocator<std::pair<std::string const,long>>>::_M_insert_unique(std::pair<std::string const,long> const&)")]
// 0xc0505c — __ZNSt8_Rb_treeISsSt4pairIKSslESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueERKS2_
pub fn stub_c0505c() -> ! {
    todo!("0xc0505c __ZNSt8_Rb_treeISsSt4pairIKSslESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueERKS2_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,long>,std::_Select1st<std::pair<std::string const,long>>,std::less<std::string>,std::allocator<std::pair<std::string const,long>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,long>> *)")]
// 0xc05140 — __ZNSt8_Rb_treeISsSt4pairIKSslESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
pub fn stub_c05140() -> ! {
    todo!("0xc05140 __ZNSt8_Rb_treeISsSt4pairIKSslESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")
}

#[doc(alias = "RBX::MegaClusterLegacy::VoxelGridOverlay::prepare(RBX::SpatialRegion::Id const&)")]
// 0xc05840 — __ZN3RBX17MegaClusterLegacy16VoxelGridOverlay7prepareERKNS_13SpatialRegion2IdE
pub fn stub_c05840() -> ! {
    todo!("0xc05840 __ZN3RBX17MegaClusterLegacy16VoxelGridOverlay7prepareERKNS_13SpatialRegion2IdE")
}

#[doc(alias = "RBX::InitEdgeDistanceLookup(void)")]
// 0xc05c84 — __ZN3RBX22InitEdgeDistanceLookupEv
pub fn stub_c05c84() -> ! {
    todo!("0xc05c84 __ZN3RBX22InitEdgeDistanceLookupEv")
}

#[doc(alias = "RBX::MegaClusterLegacy::~MegaClusterLegacy()")]
// 0xc05e8c — __ZN3RBX17MegaClusterLegacyD1Ev
pub fn stub_c05e8c() -> ! {
    todo!("0xc05e8c __ZN3RBX17MegaClusterLegacyD1Ev")
}

#[doc(alias = "RBX::MegaClusterLegacy::~MegaClusterLegacy()")]
// 0xc05e90 — __ZN3RBX17MegaClusterLegacyD2Ev
pub fn stub_c05e90() -> ! {
    todo!("0xc05e90 __ZN3RBX17MegaClusterLegacyD2Ev")
}

#[doc(alias = "RBX::MegaClusterLegacy::resetDirty(RBX::SpatialRegion::Id const&)")]
// 0xc06020 — __ZN3RBX17MegaClusterLegacy10resetDirtyERKNS_13SpatialRegion2IdE
pub fn stub_c06020() -> ! {
    todo!("0xc06020 __ZN3RBX17MegaClusterLegacy10resetDirtyERKNS_13SpatialRegion2IdE")
}

#[doc(alias = "RBX::anonymous namespace::indexFromChunkPos(RBX::SpatialRegion::Id const&)")]
// 0xc060d0 — __ZN3RBX12_GLOBAL__N_117indexFromChunkPosERKNS_13SpatialRegion2IdE
// was: RBX::`anonymous namespace'::indexFromChunkPos(RBX::SpatialRegion::Id const&)
pub fn stub_c060d0() -> ! {
    todo!("0xc060d0 __ZN3RBX12_GLOBAL__N_117indexFromChunkPosERKNS_13SpatialRegion2IdE")
}

#[doc(alias = "RBX::MegaClusterLegacy::resetWaterDirty(RBX::SpatialRegion::Id const&)")]
// 0xc06234 — __ZN3RBX17MegaClusterLegacy15resetWaterDirtyERKNS_13SpatialRegion2IdE
pub fn stub_c06234() -> ! {
    todo!("0xc06234 __ZN3RBX17MegaClusterLegacy15resetWaterDirtyERKNS_13SpatialRegion2IdE")
}

#[doc(alias = "RBX::MegaClusterLegacy::markDirty(RBX::SpatialRegion::Id const&,bool,bool)")]
// 0xc062cc — __ZN3RBX17MegaClusterLegacy9markDirtyERKNS_13SpatialRegion2IdEbb
pub fn stub_c062cc() -> ! {
    todo!("0xc062cc __ZN3RBX17MegaClusterLegacy9markDirtyERKNS_13SpatialRegion2IdEbb")
}

#[doc(alias = "RBX::MegaClusterLegacy::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")]
// 0xc0642c — __ZN3RBX17MegaClusterLegacy18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
pub fn stub_c0642c() -> ! {
    todo!("0xc0642c __ZN3RBX17MegaClusterLegacy18terrainCellChangedERKNS_5Voxel14CellChangeInfoE")
}

#[doc(alias = "non-virtual thunk toRBX::MegaClusterLegacy::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")]
// 0xc06a90 — __ZThn56_N3RBX17MegaClusterLegacy18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
// was: `non-virtual thunk to'RBX::MegaClusterLegacy::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)
pub fn stub_c06a90() -> ! {
    todo!("0xc06a90 __ZThn56_N3RBX17MegaClusterLegacy18terrainCellChangedERKNS_5Voxel14CellChangeInfoE")
}

#[doc(alias = "RBX::MegaClusterLegacy::destroyChunk(RBX::MegaClusterLegacy::ChunkData &)")]
// 0xc06dcc — __ZN3RBX17MegaClusterLegacy12destroyChunkERNS0_9ChunkDataE
pub fn stub_c06dcc() -> ! {
    todo!("0xc06dcc __ZN3RBX17MegaClusterLegacy12destroyChunkERNS0_9ChunkDataE")
}

#[doc(alias = "RBX::MegaClusterLegacy::updateEntity(bool)")]
// 0xc06e8c — __ZN3RBX17MegaClusterLegacy12updateEntityEb
pub fn stub_c06e8c() -> ! {
    todo!("0xc06e8c __ZN3RBX17MegaClusterLegacy12updateEntityEb")
}

#[doc(alias = "RBX::MegaClusterLegacy::updateChunk(RBX::SpatialRegion::Id const&,bool)")]
// 0xc0707c — __ZN3RBX17MegaClusterLegacy11updateChunkERKNS_13SpatialRegion2IdEb
pub fn stub_c0707c() -> ! {
    todo!("0xc0707c __ZN3RBX17MegaClusterLegacy11updateChunkERKNS_13SpatialRegion2IdEb")
}

#[doc(alias = "RBX::MegaClusterLegacy::getSharedVDecl(void)")]
// 0xc07098 — __ZN3RBX17MegaClusterLegacy14getSharedVDeclEv
pub fn stub_c07098() -> ! {
    todo!("0xc07098 __ZN3RBX17MegaClusterLegacy14getSharedVDeclEv")
}

#[doc(alias = "RBX::MegaClusterLegacy::unbuild(void)")]
// 0xc07260 — __ZN3RBX17MegaClusterLegacy7unbuildEv
pub fn stub_c07260() -> ! {
    todo!("0xc07260 __ZN3RBX17MegaClusterLegacy7unbuildEv")
}

#[doc(alias = "RBX::MegaCluster::destroy(void)")]
// 0xc07a64 — __ZN3RBX11MegaCluster7destroyEv
pub fn stub_c07a64() -> ! {
    todo!("0xc07a64 __ZN3RBX11MegaCluster7destroyEv")
}

#[doc(alias = "RBX::MegaCluster::~MegaCluster()")]
// 0xc07b08 — __ZN3RBX11MegaClusterD2Ev
pub fn stub_c07b08() -> ! {
    todo!("0xc07b08 __ZN3RBX11MegaClusterD2Ev")
}

#[doc(alias = "RBX::MegaCluster::updateEntity(bool)")]
// 0xc07df8 — __ZN3RBX11MegaCluster12updateEntityEb
pub fn stub_c07df8() -> ! {
    todo!("0xc07df8 __ZN3RBX11MegaCluster12updateEntityEb")
}

#[doc(alias = "RBX::MegaCluster::updateChunkGeometry(RBX::SpatialRegion::Id const&,bool,bool)")]
// 0xc07f5c — __ZN3RBX11MegaCluster19updateChunkGeometryERKNS_13SpatialRegion2IdEbb
pub fn stub_c07f5c() -> ! {
    todo!("0xc07f5c __ZN3RBX11MegaCluster19updateChunkGeometryERKNS_13SpatialRegion2IdEbb")
}

#[doc(alias = "RBX::MegaCluster::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")]
// 0xc08634 — __ZN3RBX11MegaCluster18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
pub fn stub_c08634() -> ! {
    todo!("0xc08634 __ZN3RBX11MegaCluster18terrainCellChangedERKNS_5Voxel14CellChangeInfoE")
}

#[doc(alias = "RBX::MegaCluster::markDirty(RBX::SpatialRegion::Id const&,bool,bool)")]
// 0xc08c24 — __ZN3RBX11MegaCluster9markDirtyERKNS_13SpatialRegion2IdEbb
pub fn stub_c08c24() -> ! {
    todo!("0xc08c24 __ZN3RBX11MegaCluster9markDirtyERKNS_13SpatialRegion2IdEbb")
}

#[doc(alias = "non-virtual thunk toRBX::MegaCluster::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")]
// 0xc08dc8 — __ZThn56_N3RBX11MegaCluster18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
// was: `non-virtual thunk to'RBX::MegaCluster::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)
pub fn stub_c08dc8() -> ! {
    todo!("0xc08dc8 __ZThn56_N3RBX11MegaCluster18terrainCellChangedERKNS_5Voxel14CellChangeInfoE")
}

#[doc(alias = "RBX::MegaCluster::updateChunk(RBX::SpatialRegion::Id const&,bool)")]
// 0xc08dd0 — __ZN3RBX11MegaCluster11updateChunkERKNS_13SpatialRegion2IdEb
pub fn stub_c08dd0() -> ! {
    todo!("0xc08dd0 __ZN3RBX11MegaCluster11updateChunkERKNS_13SpatialRegion2IdEb")
}

#[doc(alias = "RBX::MegaCluster::createSolidGeometry(RBX::RenderNode *,RBX::SpatialRegion::Id const&,unsigned int *)")]
// 0xc08ddc — __ZN3RBX11MegaCluster19createSolidGeometryEPNS_10RenderNodeERKNS_13SpatialRegion2IdEPj
pub fn stub_c08ddc() -> ! {
    todo!("0xc08ddc __ZN3RBX11MegaCluster19createSolidGeometryEPNS_10RenderNodeERKNS_13SpatialRegion2IdEPj")
}

#[doc(alias = "RBX::MegaCluster::createWaterGeometry(RBX::RenderNode *,RBX::SpatialRegion::Id const&,unsigned int *)")]
// 0xc08fec — __ZN3RBX11MegaCluster19createWaterGeometryEPNS_10RenderNodeERKNS_13SpatialRegion2IdEPj
pub fn stub_c08fec() -> ! {
    todo!("0xc08fec __ZN3RBX11MegaCluster19createWaterGeometryEPNS_10RenderNodeERKNS_13SpatialRegion2IdEPj")
}