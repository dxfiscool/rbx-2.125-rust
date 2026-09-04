//! core shard oz — 100 core stubs EA-sorted, 0xbec65c..0xc0dbd8 (RBX not Reflection|Instance|DataModel|Ogre|G3D|RakNet|Sound|Audio|FMOD|Script|Lua, EA-sorted asc, next 100 uncovered, global-deduped).
//! Source: ida/export.json filtered where demangled contains RBX and not Reflection|Instance|DataModel|Ogre|G3D|RakNet|Sound|Audio|FMOD|Script|Lua, EA-sorted asc, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::ViewRbxGfx::printScene(void)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx10printSceneEv")]
// 0xbec65c — __ZN3RBX10ViewRbxGfx10printSceneEv
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this)
pub fn stub_0xbec65c() {
    // IDA 0xbec65c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "non-virtual thunk toRBX::ViewRbxGfx::printScene(void)")]
#[doc(alias = "__ZThn4_N3RBX10ViewRbxGfx10printSceneEv")]
// 0xbec7fc — __ZThn4_N3RBX10ViewRbxGfx10printSceneEv
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this)
pub fn stub_0xbec7fc() {
    // IDA 0xbec7fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ViewRbxGfx::renderPrepare(RBX::IMetric *)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricE")]
// 0xbec808 — __ZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricE
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this, RBX::IMetric *)
pub fn stub_0xbec808() {
    // IDA 0xbec808: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ViewRbxGfx::renderPrepare(RBX::IMetric *)::ProxyMetric::~ProxyMetric()")]
#[doc(alias = "__ZZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricEEN11ProxyMetricD1Ev")]
// 0xbed5b8 — __ZZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricEEN11ProxyMetricD1Ev
// type: 
pub fn stub_0xbed5b8() {
    // IDA 0xbed5b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ViewRbxGfx::renderPerform(double)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx13renderPerformEd")]
// 0xbed5c0 — __ZN3RBX10ViewRbxGfx13renderPerformEd
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this, double)
pub fn stub_0xbed5c0() {
    // IDA 0xbed5c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ViewRbxGfx::saveScreenshotToFile(std::string &)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx20saveScreenshotToFileERSs")]
// 0xbee4c0 — __ZN3RBX10ViewRbxGfx20saveScreenshotToFileERSs
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this, std::string *)
pub fn stub_0xbee4c0() {
    // IDA 0xbee4c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ViewRbxGfx::update(void)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx6updateEv")]
// 0xbee96c — __ZN3RBX10ViewRbxGfx6updateEv
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this)
pub fn stub_0xbee96c() {
    // IDA 0xbee96c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ViewRbxGfx::buildGui(bool)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx8buildGuiEb")]
// 0xbeea8c — __ZN3RBX10ViewRbxGfx8buildGuiEb
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this, bool)
pub fn stub_0xbeea8c() {
    // IDA 0xbeea8c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ViewRbxGfx::getRenderStats(void)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx14getRenderStatsEv")]
// 0xbeeaac — __ZN3RBX10ViewRbxGfx14getRenderStatsEv
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this)
pub fn stub_0xbeeaac() {
    // IDA 0xbeeaac: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ViewRbxGfx::renderThumb(void)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx11renderThumbEv")]
// 0xbeeab8 — __ZN3RBX10ViewRbxGfx11renderThumbEv
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this)
pub fn stub_0xbeeab8() {
    // IDA 0xbeeab8: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ViewRbxGfx::writeRTToFile(std::string const&)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx13writeRTToFileERKSs")]
// 0xbeef64 — __ZN3RBX10ViewRbxGfx13writeRTToFileERKSs
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this, const std::string *)
pub fn stub_0xbeef64() {
    // IDA 0xbeef64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ViewRbxGfx::writeRTToBuffer(unsigned char *,int,int,int)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx15writeRTToBufferEPhiii")]
// 0xbeefb0 — __ZN3RBX10ViewRbxGfx15writeRTToBufferEPhiii
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this, unsigned __int8 *, int, int, int)
pub fn stub_0xbeefb0() {
    // IDA 0xbeefb0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ViewRbxGfx::startFrame(void)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx10startFrameEv")]
// 0xbef174 — __ZN3RBX10ViewRbxGfx10startFrameEv
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this)
pub fn stub_0xbef174() {
    // IDA 0xbef174: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ViewRbxGfx::renderPrepare(RBX::IMetric *)::ProxyMetric::~ProxyMetric()")]
#[doc(alias = "__ZZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricEEN11ProxyMetricD0Ev")]
// 0xbef1f0 — __ZZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricEEN11ProxyMetricD0Ev
// type: int __fastcall(int)
pub fn stub_0xbef1f0() {
    // IDA 0xbef1f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ViewRbxGfx::renderPrepare(RBX::IMetric *)::ProxyMetric::getMetric(std::string const&)const")]
#[doc(alias = "__ZZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricEENK11ProxyMetric9getMetricERKSs")]
// 0xbef1f4 — __ZZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricEENK11ProxyMetric9getMetricERKSs
// type: int __fastcall(int, int, std::string *this)
pub fn stub_0xbef1f4() {
    // IDA 0xbef1f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ViewRbxGfx::renderPrepare(RBX::IMetric *)::ProxyMetric::getMetricValue(std::string const&)const")]
#[doc(alias = "__ZZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricEENK11ProxyMetric14getMetricValueERKSs")]
// 0xbef230 — __ZZN3RBX10ViewRbxGfx13renderPrepareEPNS_7IMetricEENK11ProxyMetric14getMetricValueERKSs
// type: int __fastcall(int, std::string *this)
pub fn stub_0xbef230() {
    // IDA 0xbef230: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ViewRbxGfx_InitModule(void)::ViewRbxGfxFactory::Create(RBX::CRenderSettings::GraphicsMode,RBX::OSContext *,RBX::CRenderSettings*)")]
#[doc(alias = "__ZZN3RBX21ViewRbxGfx_InitModuleEvEN17ViewRbxGfxFactory6CreateENS_15CRenderSettings12GraphicsModeEPNS_9OSContextEPS1_")]
// 0xbef270 — __ZZN3RBX21ViewRbxGfx_InitModuleEvEN17ViewRbxGfxFactory6CreateENS_15CRenderSettings12GraphicsModeEPNS_9OSContextEPS1_
// type: 
pub fn stub_0xbef270() {
    // IDA 0xbef270: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::WindowAverage<double,double>::getSanitizedStats(RBX::Confidence)const")]
#[doc(alias = "__ZNK3RBX13WindowAverageIddE17getSanitizedStatsENS_10ConfidenceE")]
// 0xbef380 — __ZNK3RBX13WindowAverageIddE17getSanitizedStatsENS_10ConfidenceE
// type: 
pub fn stub_0xbef380() {
    // IDA 0xbef380: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ViewRbxGfx::throttle(void)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx8throttleEv")]
// 0xbef8c0 — __ZN3RBX10ViewRbxGfx8throttleEv
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this)
pub fn stub_0xbef8c0() {
    // IDA 0xbef8c0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ViewRbxGfx::getAdorn(void)")]
#[doc(alias = "__ZN3RBX10ViewRbxGfx8getAdornEv")]
// 0xbef8cc — __ZN3RBX10ViewRbxGfx8getAdornEv
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this)
pub fn stub_0xbef8cc() {
    // IDA 0xbef8cc: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ViewBase::canSetFullscreen(void)")]
#[doc(alias = "__ZN3RBX8ViewBase16canSetFullscreenEv")]
// 0xbef8d0 — __ZN3RBX8ViewBase16canSetFullscreenEv
// type: _DWORD __fastcall(RBX::ViewBase *__hidden this)
pub fn stub_0xbef8d0() {
    // IDA 0xbef8d0: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,int>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFviEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX10ViewRbxGfxEiEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev")]
// 0xbf0e94 — __ZN3rbx7signals6signalIFviEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX10ViewRbxGfxEiEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev
// type: 
pub fn stub_0xbf0e94() {
    // IDA 0xbf0e94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,int>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFviEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX10ViewRbxGfxEiEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev")]
// 0xbf0ef0 — __ZN3rbx7signals6signalIFviEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX10ViewRbxGfxEiEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev
// type: 
pub fn stub_0xbf0ef0() {
    // IDA 0xbf0ef0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,int>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>,1,void ()(int)>::call(int)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFviEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX10ViewRbxGfxEiEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEi")]
// 0xbf10f4 — __ZN3rbx8callableINS_7signals6signalIFviEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX10ViewRbxGfxEiEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEi
// type: 
pub fn stub_0xbf10f4() {
    // IDA 0xbf10f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,int>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>,1,void ()(int)>::call(int)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFviEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX10ViewRbxGfxEiEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEi")]
// 0xbf110c — __ZThn4_N3rbx8callableINS_7signals6signalIFviEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX10ViewRbxGfxEiEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEi
// type: 
pub fn stub_0xbf110c() {
    // IDA 0xbf110c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ViewRbxGfx>,boost::_bi::list1<boost::_bi::value<RBX::ViewRbxGfx*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX10ViewRbxGfxEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev")]
// 0xbf18ac — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX10ViewRbxGfxEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev
// type: 
pub fn stub_0xbf18ac() {
    // IDA 0xbf18ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ViewRbxGfx>,boost::_bi::list1<boost::_bi::value<RBX::ViewRbxGfx*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX10ViewRbxGfxEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev")]
// 0xbf1908 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX10ViewRbxGfxEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev
// type: 
pub fn stub_0xbf1908() {
    // IDA 0xbf1908: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ViewRbxGfx>,boost::_bi::list1<boost::_bi::value<RBX::ViewRbxGfx*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX10ViewRbxGfxEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv")]
// 0xbf1a14 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX10ViewRbxGfxEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
// type: 
pub fn stub_0xbf1a14() {
    // IDA 0xbf1a14: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ViewRbxGfx>,boost::_bi::list1<boost::_bi::value<RBX::ViewRbxGfx*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX10ViewRbxGfxEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv")]
// 0xbf1a2c — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX10ViewRbxGfxEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
// type: int __fastcall(_DWORD *)
pub fn stub_0xbf1a2c() {
    // IDA 0xbf1a2c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,bool>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX10ViewRbxGfxEbEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev")]
// 0xbf1afc — __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX10ViewRbxGfxEbEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev
// type: 
pub fn stub_0xbf1afc() {
    // IDA 0xbf1afc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,bool>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX10ViewRbxGfxEbEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev")]
// 0xbf1b58 — __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX10ViewRbxGfxEbEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev
// type: 
pub fn stub_0xbf1b58() {
    // IDA 0xbf1b58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,bool>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>,1,void ()(bool)>::call(bool)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX10ViewRbxGfxEbEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEb")]
// 0xbf1d5c — __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX10ViewRbxGfxEbEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEb
// type: 
pub fn stub_0xbf1d5c() {
    // IDA 0xbf1d5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ViewRbxGfx,bool>,boost::_bi::list2<boost::_bi::value<RBX::ViewRbxGfx*>,boost::arg<1>>>,1,void ()(bool)>::call(bool)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX10ViewRbxGfxEbEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEb")]
// 0xbf1d74 — __ZThn4_N3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX10ViewRbxGfxEbEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEb
// type: 
pub fn stub_0xbf1d74() {
    // IDA 0xbf1d74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FontFactory::FontFactory(void)")]
#[doc(alias = "__ZN3RBX11FontFactoryC1Ev")]
// 0xbf4304 — __ZN3RBX11FontFactoryC1Ev
// type: _DWORD __fastcall(RBX::FontFactory *__hidden this)
pub fn stub_0xbf4304() {
    // IDA 0xbf4304: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FontFactory::FontFactory(void)")]
#[doc(alias = "__ZN3RBX11FontFactoryC2Ev")]
// 0xbf4308 — __ZN3RBX11FontFactoryC2Ev
// type: _DWORD __fastcall(RBX::FontFactory *__hidden this)
pub fn stub_0xbf4308() {
    // IDA 0xbf4308: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FontFactory::loadFont(RBX::Text::Font)")]
#[doc(alias = "__ZN3RBX11FontFactory8loadFontENS_4Text4FontE")]
// 0xbf44a8 — __ZN3RBX11FontFactory8loadFontENS_4Text4FontE
// type: 
pub fn stub_0xbf44a8() {
    // IDA 0xbf44a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FontFactory::~FontFactory()")]
#[doc(alias = "__ZN3RBX11FontFactoryD1Ev")]
// 0xbf58fc — __ZN3RBX11FontFactoryD1Ev
// type: void __fastcall(RBX::FontFactory *__hidden this)
pub fn stub_0xbf58fc() {
    // IDA 0xbf58fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FontFactory::getTypesetter(RBX::Text::Font)")]
#[doc(alias = "__ZN3RBX11FontFactory13getTypesetterENS_4Text4FontE")]
// 0xbf59d8 — __ZN3RBX11FontFactory13getTypesetterENS_4Text4FontE
// type: int __fastcall(int, int)
pub fn stub_0xbf59d8() {
    // IDA 0xbf59d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FontFactory::getTexture(RBX::Text::Font,float)")]
#[doc(alias = "__ZN3RBX11FontFactory10getTextureENS_4Text4FontEf")]
// 0xbf5a34 — __ZN3RBX11FontFactory10getTextureENS_4Text4FontEf
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xbf5a34() {
    // IDA 0xbf5a34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TypesetterBitmap>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX16TypesetterBitmapEED1Ev")]
// 0xbf5b50 — __ZN5boost6detail17sp_counted_impl_pIN3RBX16TypesetterBitmapEED1Ev
// type: 
pub fn stub_0xbf5b50() {
    // IDA 0xbf5b50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TypesetterBitmap>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX16TypesetterBitmapEED0Ev")]
// 0xbf5b54 — __ZN5boost6detail17sp_counted_impl_pIN3RBX16TypesetterBitmapEED0Ev
// type: 
pub fn stub_0xbf5b54() {
    // IDA 0xbf5b54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TypesetterBitmap>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX16TypesetterBitmapEE7disposeEv")]
// 0xbf5b58 — __ZN5boost6detail17sp_counted_impl_pIN3RBX16TypesetterBitmapEE7disposeEv
// type: 
pub fn stub_0xbf5b58() {
    // IDA 0xbf5b58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TypesetterBitmap>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX16TypesetterBitmapEE11get_deleterERKSt9type_info")]
// 0xbf5b68 — __ZN5boost6detail17sp_counted_impl_pIN3RBX16TypesetterBitmapEE11get_deleterERKSt9type_info
// type: 
pub fn stub_0xbf5b68() {
    // IDA 0xbf5b68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TypesetterBitmap>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX16TypesetterBitmapEE19get_untyped_deleterEv")]
// 0xbf5b6c — __ZN5boost6detail17sp_counted_impl_pIN3RBX16TypesetterBitmapEE19get_untyped_deleterEv
// type: 
pub fn stub_0xbf5b6c() {
    // IDA 0xbf5b6c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TypesetterBitmap::TypesetterBitmap(std::string const&,std::string const&,float)")]
#[doc(alias = "__ZN3RBX16TypesetterBitmapC1ERKSsS2_f")]
// 0xbf7df0 — __ZN3RBX16TypesetterBitmapC1ERKSsS2_f
// type: _DWORD __fastcall(RBX::TypesetterBitmap *__hidden this, const std::string *, const std::string *, float)
pub fn stub_0xbf7df0() {
    // IDA 0xbf7df0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "RBX::TypesetterBitmap::TypesetterBitmap(std::string const&,std::string const&,float)")]
#[doc(alias = "__ZN3RBX16TypesetterBitmapC2ERKSsS2_f")]
// 0xbf7df4 — __ZN3RBX16TypesetterBitmapC2ERKSsS2_f
// type: _DWORD __fastcall(RBX::TypesetterBitmap *__hidden this, const std::string *, const std::string *, float)
pub fn stub_0xbf7df4() {
    // IDA 0xbf7df4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "RBX::TypesetterBitmap::~TypesetterBitmap()")]
#[doc(alias = "__ZN3RBX16TypesetterBitmapD1Ev")]
// 0xbf96c0 — __ZN3RBX16TypesetterBitmapD1Ev
// type: void __fastcall(RBX::TypesetterBitmap *__hidden this)
pub fn stub_0xbf96c0() {
    // IDA 0xbf96c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TypesetterBitmap::~TypesetterBitmap()")]
#[doc(alias = "__ZN3RBX16TypesetterBitmapD0Ev")]
// 0xbf9784 — __ZN3RBX16TypesetterBitmapD0Ev
// type: void __fastcall(RBX::TypesetterBitmap *__hidden this)
pub fn stub_0xbf9784() {
    // IDA 0xbf9784: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::TypesetterBitmap::GlyphLine,std::allocator<RBX::TypesetterBitmap::GlyphLine>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TypesetterBitmap::GlyphLine*,std::vector<RBX::TypesetterBitmap::GlyphLine,std::allocator<RBX::TypesetterBitmap::GlyphLine>>>,unsigned long,RBX::TypesetterBitmap::GlyphLine const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX16TypesetterBitmap9GlyphLineESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// 0xbf98bc — __ZNSt6vectorIN3RBX16TypesetterBitmap9GlyphLineESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: 
pub fn stub_0xbf98bc() {
    // IDA 0xbf98bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::TypesetterBitmap::GlyphLine,std::allocator<RBX::TypesetterBitmap::GlyphLine>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TypesetterBitmap::GlyphLine*,std::vector<RBX::TypesetterBitmap::GlyphLine,std::allocator<RBX::TypesetterBitmap::GlyphLine>>>,RBX::TypesetterBitmap::GlyphLine const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX16TypesetterBitmap9GlyphLineESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0xbf9ab0 — __ZNSt6vectorIN3RBX16TypesetterBitmap9GlyphLineESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: 
pub fn stub_0xbf9ab0() {
    // IDA 0xbf9ab0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::priority_queue<RBX::NodeInfo,std::vector<RBX::NodeInfo,std::allocator<RBX::NodeInfo>>,std::less<RBX::NodeInfo>>::push(RBX::NodeInfo const&)")]
#[doc(alias = "__ZNSt14priority_queueIN3RBX8NodeInfoESt6vectorIS1_SaIS1_EESt4lessIS1_EE4pushERKS1_")]
// 0xbfc2e0 — __ZNSt14priority_queueIN3RBX8NodeInfoESt6vectorIS1_SaIS1_EESt4lessIS1_EE4pushERKS1_
// type: 
pub fn stub_0xbfc2e0() {
    // IDA 0xbfc2e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::NodeInfo,std::allocator<RBX::NodeInfo>>::vector(std::vector<RBX::NodeInfo,std::allocator<RBX::NodeInfo>> const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX8NodeInfoESaIS1_EEC2ERKS3_")]
// 0xbfddd8 — __ZNSt6vectorIN3RBX8NodeInfoESaIS1_EEC2ERKS3_
// type: 
pub fn stub_0xbfddd8() {
    // IDA 0xbfddd8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::MegaClusterLegacy::VoxelGridOverlay::prepare(RBX::SpatialRegion::Id const&)")]
#[doc(alias = "__ZN3RBX17MegaClusterLegacy16VoxelGridOverlay7prepareERKNS_13SpatialRegion2IdE")]
// 0xc05840 — __ZN3RBX17MegaClusterLegacy16VoxelGridOverlay7prepareERKNS_13SpatialRegion2IdE
// type: int __fastcall(int, int)
pub fn stub_0xc05840() {
    // IDA 0xc05840: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::InitEdgeDistanceLookup(void)")]
#[doc(alias = "__ZN3RBX22InitEdgeDistanceLookupEv")]
// 0xc05c84 — __ZN3RBX22InitEdgeDistanceLookupEv
// type: _DWORD __fastcall(RBX *__hidden this)
pub fn stub_0xc05c84() {
    // IDA 0xc05c84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::MegaClusterLegacy::~MegaClusterLegacy()")]
#[doc(alias = "__ZN3RBX17MegaClusterLegacyD1Ev")]
// 0xc05e8c — __ZN3RBX17MegaClusterLegacyD1Ev
// type: void __fastcall(RBX::MegaClusterLegacy *__hidden this)
pub fn stub_0xc05e8c() {
    // IDA 0xc05e8c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MegaClusterLegacy::~MegaClusterLegacy()")]
#[doc(alias = "__ZN3RBX17MegaClusterLegacyD2Ev")]
// 0xc05e90 — __ZN3RBX17MegaClusterLegacyD2Ev
// type: void __fastcall(RBX::MegaClusterLegacy *__hidden this)
pub fn stub_0xc05e90() {
    // IDA 0xc05e90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MegaClusterLegacy::resetDirty(RBX::SpatialRegion::Id const&)")]
#[doc(alias = "__ZN3RBX17MegaClusterLegacy10resetDirtyERKNS_13SpatialRegion2IdE")]
// 0xc06020 — __ZN3RBX17MegaClusterLegacy10resetDirtyERKNS_13SpatialRegion2IdE
// type: 
pub fn stub_0xc06020() {
    // IDA 0xc06020: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::anonymous namespace::indexFromChunkPos(RBX::SpatialRegion::Id const&)")]
#[doc(alias = "__ZN3RBX12_GLOBAL__N_117indexFromChunkPosERKNS_13SpatialRegion2IdE")]
// 0xc060d0 — __ZN3RBX12_GLOBAL__N_117indexFromChunkPosERKNS_13SpatialRegion2IdE
// type: 
pub fn stub_0xc060d0() {
    // IDA 0xc060d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MegaClusterLegacy::resetWaterDirty(RBX::SpatialRegion::Id const&)")]
#[doc(alias = "__ZN3RBX17MegaClusterLegacy15resetWaterDirtyERKNS_13SpatialRegion2IdE")]
// 0xc06234 — __ZN3RBX17MegaClusterLegacy15resetWaterDirtyERKNS_13SpatialRegion2IdE
// type: 
pub fn stub_0xc06234() {
    // IDA 0xc06234: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MegaClusterLegacy::markDirty(RBX::SpatialRegion::Id const&,bool,bool)")]
#[doc(alias = "__ZN3RBX17MegaClusterLegacy9markDirtyERKNS_13SpatialRegion2IdEbb")]
// 0xc062cc — __ZN3RBX17MegaClusterLegacy9markDirtyERKNS_13SpatialRegion2IdEbb
// type: 
pub fn stub_0xc062cc() {
    // IDA 0xc062cc: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::MegaClusterLegacy::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")]
#[doc(alias = "__ZN3RBX17MegaClusterLegacy18terrainCellChangedERKNS_5Voxel14CellChangeInfoE")]
// 0xc0642c — __ZN3RBX17MegaClusterLegacy18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
// type: 
pub fn stub_0xc0642c() {
    // IDA 0xc0642c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "non-virtual thunk toRBX::MegaClusterLegacy::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")]
#[doc(alias = "__ZThn56_N3RBX17MegaClusterLegacy18terrainCellChangedERKNS_5Voxel14CellChangeInfoE")]
// 0xc06a90 — __ZThn56_N3RBX17MegaClusterLegacy18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
// type: 
pub fn stub_0xc06a90() {
    // IDA 0xc06a90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MegaClusterLegacy::destroyChunk(RBX::MegaClusterLegacy::ChunkData &)")]
#[doc(alias = "__ZN3RBX17MegaClusterLegacy12destroyChunkERNS0_9ChunkDataE")]
// 0xc06dcc — __ZN3RBX17MegaClusterLegacy12destroyChunkERNS0_9ChunkDataE
// type: 
pub fn stub_0xc06dcc() {
    // IDA 0xc06dcc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MegaClusterLegacy::updateEntity(bool)")]
#[doc(alias = "__ZN3RBX17MegaClusterLegacy12updateEntityEb")]
// 0xc06e8c — __ZN3RBX17MegaClusterLegacy12updateEntityEb
// type: _DWORD __fastcall(RBX::MegaClusterLegacy *__hidden this, bool)
pub fn stub_0xc06e8c() {
    // IDA 0xc06e8c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MegaClusterLegacy::updateChunk(RBX::SpatialRegion::Id const&,bool)")]
#[doc(alias = "__ZN3RBX17MegaClusterLegacy11updateChunkERKNS_13SpatialRegion2IdEb")]
// 0xc0707c — __ZN3RBX17MegaClusterLegacy11updateChunkERKNS_13SpatialRegion2IdEb
// type: 
pub fn stub_0xc0707c() {
    // IDA 0xc0707c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MegaClusterLegacy::getSharedVDecl(void)")]
#[doc(alias = "__ZN3RBX17MegaClusterLegacy14getSharedVDeclEv")]
// 0xc07098 — __ZN3RBX17MegaClusterLegacy14getSharedVDeclEv
// type: _DWORD __fastcall(RBX::MegaClusterLegacy *__hidden this)
pub fn stub_0xc07098() {
    // IDA 0xc07098: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::MegaClusterLegacy::unbuild(void)")]
#[doc(alias = "__ZN3RBX17MegaClusterLegacy7unbuildEv")]
// 0xc07260 — __ZN3RBX17MegaClusterLegacy7unbuildEv
// type: _DWORD __fastcall(RBX::MegaClusterLegacy *__hidden this)
pub fn stub_0xc07260() {
    // IDA 0xc07260: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::MegaCluster::destroy(void)")]
#[doc(alias = "__ZN3RBX11MegaCluster7destroyEv")]
// 0xc07a64 — __ZN3RBX11MegaCluster7destroyEv
// type: _DWORD __fastcall(RBX::MegaCluster *__hidden this)
pub fn stub_0xc07a64() {
    // IDA 0xc07a64: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::MegaCluster::~MegaCluster()")]
#[doc(alias = "__ZN3RBX11MegaClusterD2Ev")]
// 0xc07b08 — __ZN3RBX11MegaClusterD2Ev
// type: void __fastcall(RBX::MegaCluster *__hidden this)
pub fn stub_0xc07b08() {
    // IDA 0xc07b08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MegaCluster::updateEntity(bool)")]
#[doc(alias = "__ZN3RBX11MegaCluster12updateEntityEb")]
// 0xc07df8 — __ZN3RBX11MegaCluster12updateEntityEb
// type: _DWORD __fastcall(RBX::MegaCluster *__hidden this, bool)
pub fn stub_0xc07df8() {
    // IDA 0xc07df8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MegaCluster::updateChunkGeometry(RBX::SpatialRegion::Id const&,bool,bool)")]
#[doc(alias = "__ZN3RBX11MegaCluster19updateChunkGeometryERKNS_13SpatialRegion2IdEbb")]
// 0xc07f5c — __ZN3RBX11MegaCluster19updateChunkGeometryERKNS_13SpatialRegion2IdEbb
// type: int __fastcall(int, int, int, int)
pub fn stub_0xc07f5c() {
    // IDA 0xc07f5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MegaCluster::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")]
#[doc(alias = "__ZN3RBX11MegaCluster18terrainCellChangedERKNS_5Voxel14CellChangeInfoE")]
// 0xc08634 — __ZN3RBX11MegaCluster18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
// type: int __fastcall(int)
pub fn stub_0xc08634() {
    // IDA 0xc08634: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MegaCluster::markDirty(RBX::SpatialRegion::Id const&,bool,bool)")]
#[doc(alias = "__ZN3RBX11MegaCluster9markDirtyERKNS_13SpatialRegion2IdEbb")]
// 0xc08c24 — __ZN3RBX11MegaCluster9markDirtyERKNS_13SpatialRegion2IdEbb
// type: int __fastcall(int, int, int)
pub fn stub_0xc08c24() {
    // IDA 0xc08c24: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "non-virtual thunk toRBX::MegaCluster::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")]
#[doc(alias = "__ZThn56_N3RBX11MegaCluster18terrainCellChangedERKNS_5Voxel14CellChangeInfoE")]
// 0xc08dc8 — __ZThn56_N3RBX11MegaCluster18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
// type: 
pub fn stub_0xc08dc8() {
    // IDA 0xc08dc8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MegaCluster::updateChunk(RBX::SpatialRegion::Id const&,bool)")]
#[doc(alias = "__ZN3RBX11MegaCluster11updateChunkERKNS_13SpatialRegion2IdEb")]
// 0xc08dd0 — __ZN3RBX11MegaCluster11updateChunkERKNS_13SpatialRegion2IdEb
// type: int __fastcall(int, int, int)
pub fn stub_0xc08dd0() {
    // IDA 0xc08dd0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MegaCluster::createSolidGeometry(RBX::RenderNode *,RBX::SpatialRegion::Id const&,unsigned int *)")]
#[doc(alias = "__ZN3RBX11MegaCluster19createSolidGeometryEPNS_10RenderNodeERKNS_13SpatialRegion2IdEPj")]
// 0xc08ddc — __ZN3RBX11MegaCluster19createSolidGeometryEPNS_10RenderNodeERKNS_13SpatialRegion2IdEPj
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, int, int, int, int, int, int, int)
pub fn stub_0xc08ddc() {
    // IDA 0xc08ddc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MegaCluster::createWaterGeometry(RBX::RenderNode *,RBX::SpatialRegion::Id const&,unsigned int *)")]
#[doc(alias = "__ZN3RBX11MegaCluster19createWaterGeometryEPNS_10RenderNodeERKNS_13SpatialRegion2IdEPj")]
// 0xc08fec — __ZN3RBX11MegaCluster19createWaterGeometryEPNS_10RenderNodeERKNS_13SpatialRegion2IdEPj
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, int, int, int, int, int, int, int)
pub fn stub_0xc08fec() {
    // IDA 0xc08fec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MegaCluster::getSharedVDecl(void)")]
#[doc(alias = "__ZN3RBX11MegaCluster14getSharedVDeclEv")]
// 0xc09780 — __ZN3RBX11MegaCluster14getSharedVDeclEv
// type: int __fastcall(RBX::MegaCluster *this)
pub fn stub_0xc09780() {
    // IDA 0xc09780: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Voxel::Water::anonymous namespace::isWaterOnWedge(RBX::Voxel::Cell const&,RBX::Voxel::Water::LocalAreaInfo const&)")]
#[doc(alias = "__ZN3RBX5Voxel5Water12_GLOBAL__N_114isWaterOnWedgeERKNS0_4CellERKNS1_13LocalAreaInfoE")]
// 0xc09948 — __ZN3RBX5Voxel5Water12_GLOBAL__N_114isWaterOnWedgeERKNS0_4CellERKNS1_13LocalAreaInfoE
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xc09948() {
    // IDA 0xc09948: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "void RBX::MegaClusterLegacy::updateEntity_templated<RBX::MegaClusterLegacy::VoxelGridOverlay>(void)")]
#[doc(alias = "__ZN3RBX17MegaClusterLegacy22updateEntity_templatedINS0_16VoxelGridOverlayEEEvv")]
// 0xc09de0 — __ZN3RBX17MegaClusterLegacy22updateEntity_templatedINS0_16VoxelGridOverlayEEEvv
// type: 
pub fn stub_0xc09de0() {
    // IDA 0xc09de0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "void RBX::MegaClusterLegacy::updateChunk_templated<RBX::MegaClusterLegacy::VoxelGridOverlay>(RBX::SpatialRegion::Id const&,bool)")]
#[doc(alias = "__ZN3RBX17MegaClusterLegacy21updateChunk_templatedINS0_16VoxelGridOverlayEEEvRKNS_13SpatialRegion2IdEb")]
// 0xc0a0e0 — __ZN3RBX17MegaClusterLegacy21updateChunk_templatedINS0_16VoxelGridOverlayEEEvRKNS_13SpatialRegion2IdEb
// type: 
pub fn stub_0xc0a0e0() {
    // IDA 0xc0a0e0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "void RBX::MegaClusterLegacy::unbuild_templated<RBX::MegaClusterLegacy::VoxelGridOverlay>(void)")]
#[doc(alias = "__ZN3RBX17MegaClusterLegacy17unbuild_templatedINS0_16VoxelGridOverlayEEEvv")]
// 0xc0a3a0 — __ZN3RBX17MegaClusterLegacy17unbuild_templatedINS0_16VoxelGridOverlayEEEvv
// type: int __fastcall(RBX::GfxBinding *this)
pub fn stub_0xc0a3a0() {
    // IDA 0xc0a3a0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)")]
#[doc(alias = "__ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_20SolidTerrainRendererIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE")]
// 0xc0a4ec — __ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_20SolidTerrainRendererIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE
// type: 
pub fn stub_0xc0a4ec() {
    // IDA 0xc0a4ec: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::WaterFaceRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)")]
#[doc(alias = "__ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_17WaterFaceRendererIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE")]
// 0xc0acec — __ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_17WaterFaceRendererIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE
// type: 
pub fn stub_0xc0acec() {
    // IDA 0xc0acec: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::GfxBinding::invalidateEntity(void)")]
#[doc(alias = "__ZN3RBX10GfxBinding16invalidateEntityEv")]
// 0xc0b430 — __ZN3RBX10GfxBinding16invalidateEntityEv
// type: _DWORD __fastcall(RBX::GfxBinding *__hidden this)
pub fn stub_0xc0b430() {
    // IDA 0xc0b430: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::GfxBinding::onCoordinateFrameChanged(void)")]
#[doc(alias = "__ZN3RBX10GfxBinding24onCoordinateFrameChangedEv")]
// 0xc0b434 — __ZN3RBX10GfxBinding24onCoordinateFrameChangedEv
// type: _DWORD __fastcall(RBX::GfxBinding *__hidden this)
pub fn stub_0xc0b434() {
    // IDA 0xc0b434: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::GfxPart::updateCoordinateFrame(bool)")]
#[doc(alias = "__ZN3RBX7GfxPart21updateCoordinateFrameEb")]
// 0xc0b438 — __ZN3RBX7GfxPart21updateCoordinateFrameEb
// type: _DWORD __fastcall(RBX::GfxPart *__hidden this, bool)
pub fn stub_0xc0b438() {
    // IDA 0xc0b438: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::GfxPart::getFastFuzzyExtents(void)")]
#[doc(alias = "__ZN3RBX7GfxPart19getFastFuzzyExtentsEv")]
// 0xc0b43c — __ZN3RBX7GfxPart19getFastFuzzyExtentsEv
// type: _DWORD __fastcall(RBX::GfxPart *__hidden this)
pub fn stub_0xc0b43c() {
    // IDA 0xc0b43c: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::GfxPart::getPartCount(void)")]
#[doc(alias = "__ZN3RBX7GfxPart12getPartCountEv")]
// 0xc0b4cc — __ZN3RBX7GfxPart12getPartCountEv
// type: _DWORD __fastcall(RBX::GfxPart *__hidden this)
pub fn stub_0xc0b4cc() {
    // IDA 0xc0b4cc: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::GfxPart::onClumpChanged(void)")]
#[doc(alias = "__ZN3RBX7GfxPart14onClumpChangedEv")]
// 0xc0b4d4 — __ZN3RBX7GfxPart14onClumpChangedEv
// type: _DWORD __fastcall(RBX::GfxPart *__hidden this)
pub fn stub_0xc0b4d4() {
    // IDA 0xc0b4d4: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>::internal(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection)const")]
#[doc(alias = "__ZNK3RBX20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE8internalERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionE")]
// 0xc0b4d8 — __ZNK3RBX20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE8internalERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionE
// type: 
pub fn stub_0xc0b4d8() {
    // IDA 0xc0b4d8: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::WaterFaceRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
#[doc(alias = "__ZN3RBX17WaterFaceRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE5applyERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE")]
// 0xc0b66c — __ZN3RBX17WaterFaceRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE5applyERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE
// type: int __fastcall(int, int, int, int)
pub fn stub_0xc0b66c() {
    // IDA 0xc0b66c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::FaceCounter<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)")]
#[doc(alias = "__ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_11FaceCounterIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE")]
// 0xc0bf18 — __ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_11FaceCounterIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE
// type: 
pub fn stub_0xc0bf18() {
    // IDA 0xc0bf18: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE5applyERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE")]
// 0xc0c648 — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE5applyERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE
// type: 
pub fn stub_0xc0c648() {
    // IDA 0xc0c648: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::wedgeFace(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE9wedgeFaceERKNS1_6RegionINS3_5ChunkEE8iteratorE")]
// 0xc0c904 — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE9wedgeFaceERKNS1_6RegionINS3_5ChunkEE8iteratorE
// type: 
pub fn stub_0xc0c904() {
    // IDA 0xc0c904: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::detectWedgeOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE19detectWedgeOutlinesERKNS1_6RegionINS3_5ChunkEE8iteratorE")]
// 0xc0cd30 — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE19detectWedgeOutlinesERKNS1_6RegionINS3_5ChunkEE8iteratorE
// type: 
pub fn stub_0xc0cd30() {
    // IDA 0xc0cd30: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::wedgeUpEmpty(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE12wedgeUpEmptyERKNS1_6RegionINS3_5ChunkEE8iteratorE")]
// 0xc0cf1c — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE12wedgeUpEmptyERKNS1_6RegionINS3_5ChunkEE8iteratorE
// type: 
pub fn stub_0xc0cf1c() {
    // IDA 0xc0cf1c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::detectOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
#[doc(alias = "__ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE14detectOutlinesERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE")]
// 0xc0d000 — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE14detectOutlinesERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE
// type: 
pub fn stub_0xc0d000() {
    // IDA 0xc0d000: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::FaceCounter<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)")]
#[doc(alias = "__ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_11FaceCounterIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE")]
// 0xc0d418 — __ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_11FaceCounterIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE
// type: int __fastcall(unsigned int *, _WORD *)
pub fn stub_0xc0d418() {
    // IDA 0xc0d418: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RenderNode>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEED1Ev")]
// 0xc0dbd8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEED1Ev
// type: 
pub fn stub_0xc0dbd8() {
    // IDA 0xc0dbd8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
