//! core wdog watchdog23 — 120 core stubs EA-sorted asc next uncovered distinct not yet in crates/core/src.
//! Source: ida/export.json (85545 funcs) filtered demangled/mangled excludes Reflection|Instance|DataModel|Ogre|G3D|RakNet|FMOD|Lua, EA-sorted asc, next 120 uncovered after 0xf7f220 (prev max), distinctly not yet in crates/core/src.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::Network::Replicator::clusterOutStep(void)")]
// 0xae62ac — __ZN3RBX7Network10Replicator14clusterOutStepEv
// type: void __fastcall(RBX::Network::Replicator *this)
pub fn stub_ae62ac() -> ! {
    todo!("0xae62ac __ZN3RBX7Network10Replicator14clusterOutStepEv")
}

#[doc(alias = "RBX::Network::Replicator::requestDisconnect(void)")]
// 0xae6410 — __ZN3RBX7Network10Replicator17requestDisconnectEv
// type: void __fastcall(RBX::Network::Replicator *this, RBX::Instance *)
pub fn stub_ae6410() -> ! {
    todo!("0xae6410 __ZN3RBX7Network10Replicator17requestDisconnectEv")
}

#[doc(alias = "RBX::Network::Replicator::dataOutStep(void)")]
// 0xae6848 — __ZN3RBX7Network10Replicator11dataOutStepEv
// type: void __fastcall(RBX::Network::Replicator *this)
pub fn stub_ae6848() -> ! {
    todo!("0xae6848 __ZN3RBX7Network10Replicator11dataOutStepEv")
}

#[doc(alias = "RBX::Network::Replicator::isInStreamedRegions(RBX::Extents const&)")]
// 0xae6f38 — __ZN3RBX7Network10Replicator19isInStreamedRegionsERKNS_7ExtentsE
// type: int __fastcall(RBX::Network::Replicator *this, const RBX::Extents *)
pub fn stub_ae6f38() -> ! {
    todo!("0xae6f38 __ZN3RBX7Network10Replicator19isInStreamedRegionsERKNS_7ExtentsE")
}

#[doc(alias = "RBX::Network::Replicator::addTopReplicationContainers(RBX::ServiceProvider *)")]
// 0xae6f50 — __ZN3RBX7Network10Replicator27addTopReplicationContainersEPNS_15ServiceProviderE
// type: void __fastcall(RBX::Network::Replicator *this, RBX::ServiceProvider *)
pub fn stub_ae6f50() -> ! {
    todo!("0xae6f50 __ZN3RBX7Network10Replicator27addTopReplicationContainersEPNS_15ServiceProviderE")
}

#[doc(alias = "RBX::Network::Replicator::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0xae831c — __ZN3RBX7Network10Replicator17onServiceProviderEPNS_15ServiceProviderES3_
// type: int __fastcall(RBX::Network::Replicator *this, struct _Unwind_Exception *, pthread_mutex_t *, int)
pub fn stub_ae831c() -> ! {
    todo!("0xae831c __ZN3RBX7Network10Replicator17onServiceProviderEPNS_15ServiceProviderES3_")
}

#[doc(alias = "RBX::Network::Replicator::updateStatsItem(RBX::Stats::StatsService *)")]
// 0xaebf24 — __ZN3RBX7Network10Replicator15updateStatsItemEPNS_5Stats12StatsServiceE
// type: void __fastcall(RBX::Network::Replicator *this, RBX::Stats::StatsService *)
pub fn stub_aebf24() -> ! {
    todo!("0xaebf24 __ZN3RBX7Network10Replicator15updateStatsItemEPNS_5Stats12StatsServiceE")
}

#[doc(alias = "RBX::Network::Replicator::createPhysicsSender(RBX::NetworkSettings::PhysicsSendMethod)")]
// 0xaece78 — __ZN3RBX7Network10Replicator19createPhysicsSenderENS_15NetworkSettings17PhysicsSendMethodE
// type: void __fastcall(_DWORD *, int, int, int)
pub fn stub_aece78() -> ! {
    todo!("0xaece78 __ZN3RBX7Network10Replicator19createPhysicsSenderENS_15NetworkSettings17PhysicsSendMethodE")
}

#[doc(alias = "RBX::Network::Replicator::incomingPacketsCount(void)const")]
// 0xaed8a8 — __ZNK3RBX7Network10Replicator20incomingPacketsCountEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_aed8a8() -> ! {
    todo!("0xaed8a8 __ZNK3RBX7Network10Replicator20incomingPacketsCountEv")
}

#[doc(alias = "RBX::Network::Replicator::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")]
// 0xaf7c18 — __ZN3RBX7Network10Replicator18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
// type: unsigned int __fastcall(int, int)
pub fn stub_af7c18() -> ! {
    todo!("0xaf7c18 __ZN3RBX7Network10Replicator18terrainCellChangedERKNS_5Voxel14CellChangeInfoE")
}

#[doc(alias = "_non-virtual thunk to_RBX::Network::Replicator::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")]
// 0xaf7ce0 — __ZThn1196_N3RBX7Network10Replicator18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
// type: unsigned int __fastcall(int, int)
pub fn stub_af7ce0() -> ! {
    todo!("0xaf7ce0 __ZThn1196_N3RBX7Network10Replicator18terrainCellChangedERKNS_5Voxel14CellChangeInfoE")
}

#[doc(alias = "RBX::Network::LogError(RBX::Network::Replicator *,std::string const&)")]
// 0xafac40 — __ZN3RBX7Network8LogErrorEPNS0_10ReplicatorERKSs
// type: void __fastcall(RBX::DataModel **this, char **, const std::string *)
pub fn stub_afac40() -> ! {
    todo!("0xafac40 __ZN3RBX7Network8LogErrorEPNS0_10ReplicatorERKSs")
}

#[doc(alias = "RBX::Network::Replicator::processNextIncomingPacket(void)")]
// 0xafb800 — __ZN3RBX7Network10Replicator25processNextIncomingPacketEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_afb800() -> ! {
    todo!("0xafb800 __ZN3RBX7Network10Replicator25processNextIncomingPacketEv")
}

#[doc(alias = "RBX::Network::Replicator::sendItemsPacket(void)")]
// 0xafbb48 — __ZN3RBX7Network10Replicator15sendItemsPacketEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_afbb48() -> ! {
    todo!("0xafbb48 __ZN3RBX7Network10Replicator15sendItemsPacketEv")
}

#[doc(alias = "RBX::Network::Replicator::isInitialDataSent(void)")]
// 0xafbd8c — __ZN3RBX7Network10Replicator17isInitialDataSentEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_afbd8c() -> ! {
    todo!("0xafbd8c __ZN3RBX7Network10Replicator17isInitialDataSentEv")
}

#[doc(alias = "RBX::Network::Replicator::sendClusterChunk(RBX::StreamRegion::Id const&)")]
// 0xafbdb8 — __ZN3RBX7Network10Replicator16sendClusterChunkERKNS_12StreamRegion2IdE
// type: void __fastcall(int, double *, int, int, int, int, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, pthread_mutex_t *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, int, int, int)
pub fn stub_afbdb8() -> ! {
    todo!("0xafbdb8 __ZN3RBX7Network10Replicator16sendClusterChunkERKNS_12StreamRegion2IdE")
}

#[doc(alias = "RBX::Network::Replicator::sendClusterPacket(void)")]
// 0xafc5d8 — __ZN3RBX7Network10Replicator17sendClusterPacketEv
// type: RBX::Network::IdSerializer *__fastcall(RBX::Network::Replicator *this, int, int, const void *)
pub fn stub_afc5d8() -> ! {
    todo!("0xafc5d8 __ZN3RBX7Network10Replicator17sendClusterPacketEv")
}

#[doc(alias = "RBX::Network::Replicator::sendDataPing(void)")]
// 0xb020f8 — __ZN3RBX7Network10Replicator12sendDataPingEv
// type: void __fastcall(RBX::Network::Replicator *this)
pub fn stub_b020f8() -> ! {
    todo!("0xb020f8 __ZN3RBX7Network10Replicator12sendDataPingEv")
}

#[doc(alias = "RBX::Network::Replicator::getPhysicsMtuSize(void)const")]
// 0xb04ad4 — __ZNK3RBX7Network10Replicator17getPhysicsMtuSizeEv
// type: int __fastcall(RBX::Network::Replicator *this, int, int)
pub fn stub_b04ad4() -> ! {
    todo!("0xb04ad4 __ZNK3RBX7Network10Replicator17getPhysicsMtuSizeEv")
}

#[doc(alias = "RBX::Network::Replicator::getMetric(std::string const&)const")]
// 0xb04b48 — __ZNK3RBX7Network10Replicator9getMetricERKSs
// type: void __fastcall(RBX::Network::Replicator *this, const std::string *, std::string *)
pub fn stub_b04b48() -> ! {
    todo!("0xb04b48 __ZNK3RBX7Network10Replicator9getMetricERKSs")
}

#[doc(alias = "_non-virtual thunk to_RBX::Network::Replicator::getMetric(std::string const&)const")]
// 0xb04f70 — __ZThn1192_NK3RBX7Network10Replicator9getMetricERKSs
// type: void __fastcall(RBX::Network::Replicator *this, const std::string *, std::string *)
pub fn stub_b04f70() -> ! {
    todo!("0xb04f70 __ZThn1192_NK3RBX7Network10Replicator9getMetricERKSs")
}

#[doc(alias = "RBX::Network::Replicator::getMetricValue(std::string const&)const")]
// 0xb04f80 — __ZNK3RBX7Network10Replicator14getMetricValueERKSs
// type: double __fastcall(RBX::Network::Replicator *this, const std::string *)
pub fn stub_b04f80() -> ! {
    todo!("0xb04f80 __ZNK3RBX7Network10Replicator14getMetricValueERKSs")
}

#[doc(alias = "_non-virtual thunk to_RBX::Network::Replicator::getMetricValue(std::string const&)const")]
// 0xb05000 — __ZThn1192_NK3RBX7Network10Replicator14getMetricValueERKSs
// type: double __fastcall(RBX::Network::Replicator *this, const std::string *)
pub fn stub_b05000() -> ! {
    todo!("0xb05000 __ZThn1192_NK3RBX7Network10Replicator14getMetricValueERKSs")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Replicator> RBX::shared_from<RBX::Network::Replicator>(RBX::Network::Replicator*)")]
// 0xb05b88 — __ZN3RBX11shared_fromINS_7Network10ReplicatorEEEN5boost10shared_ptrIT_EEPS5_
// type: void __fastcall(int, int)
pub fn stub_b05b88() -> ! {
    todo!("0xb05b88 __ZN3RBX11shared_fromINS_7Network10ReplicatorEEEN5boost10shared_ptrIT_EEPS5_")
}

#[doc(alias = "RBX::Network::Replicator::ClusterReplicationData::ClusterReplicationData(void)")]
// 0xb05e1c — __ZN3RBX7Network10Replicator22ClusterReplicationDataC1Ev
// type: RBX::Network::Replicator::ClusterReplicationData *__fastcall(RBX::Network::Replicator::ClusterReplicationData *this)
pub fn stub_b05e1c() -> ! {
    todo!("0xb05e1c __ZN3RBX7Network10Replicator22ClusterReplicationDataC1Ev")
}

#[doc(alias = "RBX::Network::Replicator::ClusterReplicationData::~ClusterReplicationData()")]
// 0xb060c0 — __ZN3RBX7Network10Replicator22ClusterReplicationDataD1Ev
// type: void __fastcall(RBX::Network::Replicator::ClusterReplicationData *__hidden this)
pub fn stub_b060c0() -> ! {
    todo!("0xb060c0 __ZN3RBX7Network10Replicator22ClusterReplicationDataD1Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::ClusterPacketCache>::reset(void)")]
// 0xb06dd8 — __ZN5boost10shared_ptrIN3RBX7Network18ClusterPacketCacheEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_b06dd8() -> ! {
    todo!("0xb06dd8 __ZN5boost10shared_ptrIN3RBX7Network18ClusterPacketCacheEE5resetEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::PhysicsSender>::reset(void)")]
// 0xb06e78 — __ZN5boost10shared_ptrIN3RBX7Network13PhysicsSenderEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_b06e78() -> ! {
    todo!("0xb06e78 __ZN5boost10shared_ptrIN3RBX7Network13PhysicsSenderEE5resetEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Replicator::SendDataJob>::reset(void)")]
// 0xb071d8 — __ZN5boost10shared_ptrIN3RBX7Network10Replicator11SendDataJobEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_b071d8() -> ! {
    todo!("0xb071d8 __ZN5boost10shared_ptrIN3RBX7Network10Replicator11SendDataJobEE5resetEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Replicator::SendClusterJob>::reset(void)")]
// 0xb07278 — __ZN5boost10shared_ptrIN3RBX7Network10Replicator14SendClusterJobEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_b07278() -> ! {
    todo!("0xb07278 __ZN5boost10shared_ptrIN3RBX7Network10Replicator14SendClusterJobEE5resetEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Replicator::ProcessPacketsJob>::reset(void)")]
// 0xb07318 — __ZN5boost10shared_ptrIN3RBX7Network10Replicator17ProcessPacketsJobEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_b07318() -> ! {
    todo!("0xb07318 __ZN5boost10shared_ptrIN3RBX7Network10Replicator17ProcessPacketsJobEE5resetEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Replicator::PingJob>::reset(void)")]
// 0xb073b8 — __ZN5boost10shared_ptrIN3RBX7Network10Replicator7PingJobEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_b073b8() -> ! {
    todo!("0xb073b8 __ZN5boost10shared_ptrIN3RBX7Network10Replicator7PingJobEE5resetEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::ClusterPacketCache> RBX::shared_from<RBX::Network::ClusterPacketCache>(RBX::Network::ClusterPacketCache*)")]
// 0xb076ec — __ZN3RBX11shared_fromINS_7Network18ClusterPacketCacheEEEN5boost10shared_ptrIT_EEPS5_
// type: void __fastcall(int, int)
pub fn stub_b076ec() -> ! {
    todo!("0xb076ec __ZN3RBX11shared_fromINS_7Network18ClusterPacketCacheEEEN5boost10shared_ptrIT_EEPS5_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::ConcurrentRakPeer>::reset(void)")]
// 0xb07dec — __ZN5boost10shared_ptrIN3RBX7Network17ConcurrentRakPeerEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_b07dec() -> ! {
    todo!("0xb07dec __ZN5boost10shared_ptrIN3RBX7Network17ConcurrentRakPeerEE5resetEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Replicator::StatsItem>::reset(void)")]
// 0xb07e8c — __ZN5boost10shared_ptrIN3RBX7Network10Replicator9StatsItemEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_b07e8c() -> ! {
    todo!("0xb07e8c __ZN5boost10shared_ptrIN3RBX7Network10Replicator9StatsItemEE5resetEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::SharedStringDictionary>::shared_ptr<RBX::Network::SharedStringDictionary>(RBX::Network::SharedStringDictionary *)")]
// 0xb07f30 — __ZN5boost10shared_ptrIN3RBX7Network22SharedStringDictionaryEEC1IS3_EEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, boost::detail::shared_count *, int, int, void *, int)
pub fn stub_b07f30() -> ! {
    todo!("0xb07f30 __ZN5boost10shared_ptrIN3RBX7Network22SharedStringDictionaryEEC1IS3_EEPT_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::SharedStringDictionary>::operator=(rbx_core::SharedPtr<RBX::Network::SharedStringDictionary> const&)")]
// 0xb08350 — __ZN5boost10shared_ptrIN3RBX7Network22SharedStringDictionaryEEaSERKS4_
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
pub fn stub_b08350() -> ! {
    todo!("0xb08350 __ZN5boost10shared_ptrIN3RBX7Network22SharedStringDictionaryEEaSERKS4_")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::Network::Replicator> RBX::weak_from<RBX::Network::Replicator>(RBX::Network::Replicator*)")]
// 0xb0a6e8 — __ZN3RBX9weak_fromINS_7Network10ReplicatorEEEN5boost8weak_ptrIT_EEPS5_
// type: void __fastcall(int, int)
pub fn stub_b0a6e8() -> ! {
    todo!("0xb0a6e8 __ZN3RBX9weak_fromINS_7Network10ReplicatorEEEN5boost8weak_ptrIT_EEPS5_")
}

#[doc(alias = "RBX::Network::Replicator::ProcessPacketsJob::wake(void)")]
// 0xb0c974 — __ZN3RBX7Network10Replicator17ProcessPacketsJob4wakeEv
// type: void __fastcall(RBX::Network::Replicator::ProcessPacketsJob *this)
pub fn stub_b0c974() -> ! {
    todo!("0xb0c974 __ZN3RBX7Network10Replicator17ProcessPacketsJob4wakeEv")
}

#[doc(alias = "RBX::Network::Replicator::getClassName(void)const")]
// 0xb0cc30 — __ZNK3RBX7Network10Replicator12getClassNameEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_b0cc30() -> ! {
    todo!("0xb0cc30 __ZNK3RBX7Network10Replicator12getClassNameEv")
}

#[doc(alias = "RBX::Network::Replicator::requestCharacter(void)")]
// 0xb0cc40 — __ZN3RBX7Network10Replicator16requestCharacterEv
// type: void __fastcall __noreturn(RBX::Network::Replicator *this)
pub fn stub_b0cc40() -> ! {
    todo!("0xb0cc40 __ZN3RBX7Network10Replicator16requestCharacterEv")
}

#[doc(alias = "RBX::Network::Replicator::postProcessPacket(void)")]
// 0xb0ce80 — __ZN3RBX7Network10Replicator17postProcessPacketEv
// type: void __fastcall(RBX::Network::Replicator *this)
pub fn stub_b0ce80() -> ! {
    todo!("0xb0ce80 __ZN3RBX7Network10Replicator17postProcessPacketEv")
}

#[doc(alias = "RBX::Network::Replicator::onSentMarker(long)")]
// 0xb0cea8 — __ZN3RBX7Network10Replicator12onSentMarkerEl
// type: void __fastcall(RBX::Network::Replicator *this, int)
pub fn stub_b0cea8() -> ! {
    todo!("0xb0cea8 __ZN3RBX7Network10Replicator12onSentMarkerEl")
}

#[doc(alias = "RBX::Network::Replicator::processSendStats(unsigned int)")]
// 0xb0ceb0 — __ZN3RBX7Network10Replicator16processSendStatsEj
// type: void __fastcall(RBX::Network::Replicator *this, unsigned int)
pub fn stub_b0ceb0() -> ! {
    todo!("0xb0ceb0 __ZN3RBX7Network10Replicator16processSendStatsEj")
}

#[doc(alias = "_non-virtual thunk to_RBX::Network::Replicator::getClassName(void)const")]
// 0xb0cec0 — __ZThn32_NK3RBX7Network10Replicator12getClassNameEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_b0cec0() -> ! {
    todo!("0xb0cec0 __ZThn32_NK3RBX7Network10Replicator12getClassNameEv")
}

#[doc(alias = "_non-virtual thunk to_RBX::Network::Replicator::UsesReliabilityLayer(void)const")]
// 0xb0cef0 — __ZThn1180_NK3RBX7Network10Replicator20UsesReliabilityLayerEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_b0cef0() -> ! {
    todo!("0xb0cef0 __ZThn1180_NK3RBX7Network10Replicator20UsesReliabilityLayerEv")
}

#[doc(alias = "RBX::Network::Replicator::SendDataJob::~SendDataJob()")]
// 0xb0cf08 — __ZN3RBX7Network10Replicator11SendDataJobD1Ev
// type: void __fastcall(RBX::Network::Replicator::SendDataJob *__hidden this)
pub fn stub_b0cf08() -> ! {
    todo!("0xb0cf08 __ZN3RBX7Network10Replicator11SendDataJobD1Ev")
}

#[doc(alias = "RBX::Network::Replicator::SendDataJob::~SendDataJob()")]
// 0xb0cfd4 — __ZN3RBX7Network10Replicator11SendDataJobD0Ev
// type: void __fastcall(RBX::Network::Replicator::SendDataJob *__hidden this)
pub fn stub_b0cfd4() -> ! {
    todo!("0xb0cfd4 __ZN3RBX7Network10Replicator11SendDataJobD0Ev")
}

#[doc(alias = "RBX::Network::Replicator::SendDataJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
// 0xb0d0b4 — __ZN3RBX7Network10Replicator11SendDataJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::Replicator::SendDataJob *this, const RBX::TaskScheduler::Job::Stats *, double)
pub fn stub_b0d0b4() -> ! {
    todo!("0xb0d0b4 __ZN3RBX7Network10Replicator11SendDataJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::Network::Replicator::SendClusterJob::~SendClusterJob()")]
// 0xb0db10 — __ZN3RBX7Network10Replicator14SendClusterJobD1Ev
// type: void __fastcall(RBX::Network::Replicator::SendClusterJob *__hidden this)
pub fn stub_b0db10() -> ! {
    todo!("0xb0db10 __ZN3RBX7Network10Replicator14SendClusterJobD1Ev")
}

#[doc(alias = "RBX::Network::Replicator::SendClusterJob::~SendClusterJob()")]
// 0xb0dbdc — __ZN3RBX7Network10Replicator14SendClusterJobD0Ev
// type: void __fastcall(RBX::Network::Replicator::SendClusterJob *__hidden this)
pub fn stub_b0dbdc() -> ! {
    todo!("0xb0dbdc __ZN3RBX7Network10Replicator14SendClusterJobD0Ev")
}

#[doc(alias = "RBX::Network::Replicator::SendClusterJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
// 0xb0dcbc — __ZN3RBX7Network10Replicator14SendClusterJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::Replicator::SendClusterJob *this, const RBX::TaskScheduler::Job::Stats *, double)
pub fn stub_b0dcbc() -> ! {
    todo!("0xb0dcbc __ZN3RBX7Network10Replicator14SendClusterJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::Network::ReplicatorJob::~ReplicatorJob()")]
// 0xb0dfd8 — __ZN3RBX7Network13ReplicatorJobD1Ev
// type: void __fastcall(RBX::Network::ReplicatorJob *__hidden this)
pub fn stub_b0dfd8() -> ! {
    todo!("0xb0dfd8 __ZN3RBX7Network13ReplicatorJobD1Ev")
}

#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::Network::Marker>,std::allocator<rbx_core::SharedPtr<RBX::Network::Marker>>>::_M_push_back_aux(rbx_core::SharedPtr<RBX::Network::Marker> const&)")]
// 0xb139fc — __ZNSt5dequeIN5boost10shared_ptrIN3RBX7Network6MarkerEEESaIS5_EE16_M_push_back_auxERKS5_
// type: void __fastcall(_DWORD *, int *, int, int, int, int, int, int, void *, int)
pub fn stub_b139fc() -> ! {
    todo!("0xb139fc __ZNSt5dequeIN5boost10shared_ptrIN3RBX7Network6MarkerEEESaIS5_EE16_M_push_back_auxERKS5_")
}

#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::Network::Marker>,std::allocator<rbx_core::SharedPtr<RBX::Network::Marker>>>::_M_reallocate_map(unsigned long,bool)")]
// 0xb13d44 — __ZNSt5dequeIN5boost10shared_ptrIN3RBX7Network6MarkerEEESaIS5_EE17_M_reallocate_mapEmb
// type: char *__fastcall(void **, unsigned int, int)
pub fn stub_b13d44() -> ! {
    todo!("0xb13d44 __ZNSt5dequeIN5boost10shared_ptrIN3RBX7Network6MarkerEEESaIS5_EE17_M_reallocate_mapEmb")
}

#[doc(alias = "boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>>::list1(boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>)")]
// 0xb14fe0 — __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEEEC2ES8_
// type: _DWORD *__fastcall(_DWORD *, unsigned int *, int, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, pthread_mutex_t *, int, int, int, int)
pub fn stub_b14fe0() -> ! {
    todo!("0xb14fe0 __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEEEC2ES8_")
}

#[doc(alias = "boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>::list4(boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>)")]
// 0xb1c5cc — __ZN5boost3_bi5list4INS0_5valueINS_8weak_ptrIN3RBX7Network10ReplicatorEEEEENS2_IPNS6_15ReplicationDataEEENS_3argILi1EEENSC_ILi2EEEEC2ES8_SB_SD_SE_
// type: int __fastcall(int, int *, int)
pub fn stub_b1c5cc() -> ! {
    todo!("0xb1c5cc __ZN5boost3_bi5list4INS0_5valueINS_8weak_ptrIN3RBX7Network10ReplicatorEEEEENS2_IPNS6_15ReplicationDataEEENS_3argILi1EEENSC_ILi2EEEEC2ES8_SB_SD_SE_")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>)")]
// 0xb1c790 — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX7Network10ReplicatorEEEEENS2_IPNS6_15ReplicationDataEEENS_3argILi1EEENSC_ILi2EEEEC2ES8_SB_SD_SE_
// type: int __fastcall(int, int *, int)
pub fn stub_b1c790() -> ! {
    todo!("0xb1c790 __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX7Network10ReplicatorEEEEENS2_IPNS6_15ReplicationDataEEENS_3argILi1EEENSC_ILi2EEEEC2ES8_SB_SD_SE_")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>)")]
// 0xb1c954 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX7Network10ReplicatorEEEEENS2_IPNS6_15ReplicationDataEEENS_3argILi1EEEEC2ES8_SB_SD_
// type: int __fastcall(int, int *, int, int)
pub fn stub_b1c954() -> ! {
    todo!("0xb1c954 __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX7Network10ReplicatorEEEEENS2_IPNS6_15ReplicationDataEEENS_3argILi1EEEEC2ES8_SB_SD_")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>)")]
// 0xb1cb18 — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX7Network10ReplicatorEEEEENS2_IPNS6_15ReplicationDataEEEEC2ES8_SB_
// type: _DWORD *__fastcall(_DWORD *, unsigned int *, int, int, int, pthread_mutex_t *, int, int, int, int)
pub fn stub_b1cb18() -> ! {
    todo!("0xb1cb18 __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX7Network10ReplicatorEEEEENS2_IPNS6_15ReplicationDataEEEEC2ES8_SB_")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::SharedStringProtectedDictionary,RBX::Network::SharedStringProtectedDictionary>(rbx_core::SharedPtr<RBX::Network::SharedStringProtectedDictionary> *,RBX::Network::SharedStringProtectedDictionary *,boost::detail::shared_count &)")]
// 0xb2058c — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network31SharedStringProtectedDictionaryES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
pub fn stub_b2058c() -> ! {
    todo!("0xb2058c __ZN5boost6detail20sp_pointer_constructIN3RBX7Network31SharedStringProtectedDictionaryES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::SharedStringProtectedDictionary>::~sp_counted_impl_p()")]
// 0xb20850 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network31SharedStringProtectedDictionaryEED1Ev
// type: void()
pub fn stub_b20850() -> ! {
    todo!("0xb20850 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network31SharedStringProtectedDictionaryEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::SharedStringProtectedDictionary>::~sp_counted_impl_p()")]
// 0xb20854 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network31SharedStringProtectedDictionaryEED0Ev
// type: void __fastcall(void *)
pub fn stub_b20854() -> ! {
    todo!("0xb20854 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network31SharedStringProtectedDictionaryEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::SharedStringProtectedDictionary>::dispose(void)")]
// 0xb20860 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network31SharedStringProtectedDictionaryEE7disposeEv
// type: void __fastcall(int)
pub fn stub_b20860() -> ! {
    todo!("0xb20860 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network31SharedStringProtectedDictionaryEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::SharedStringProtectedDictionary>::get_deleter(std::type_info const&)")]
// 0xb209b0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network31SharedStringProtectedDictionaryEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_b209b0() -> ! {
    todo!("0xb209b0 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network31SharedStringProtectedDictionaryEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::SharedStringProtectedDictionary>::get_untyped_deleter(void)")]
// 0xb209b4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network31SharedStringProtectedDictionaryEE19get_untyped_deleterEv
// type: int()
pub fn stub_b209b4() -> ! {
    todo!("0xb209b4 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network31SharedStringProtectedDictionaryEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::SharedStringDictionary>::~sp_counted_impl_p()")]
// 0xb20d10 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22SharedStringDictionaryEED1Ev
// type: void()
pub fn stub_b20d10() -> ! {
    todo!("0xb20d10 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22SharedStringDictionaryEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::SharedStringDictionary>::~sp_counted_impl_p()")]
// 0xb20d14 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22SharedStringDictionaryEED0Ev
// type: void __fastcall(void *)
pub fn stub_b20d14() -> ! {
    todo!("0xb20d14 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22SharedStringDictionaryEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::SharedStringDictionary>::dispose(void)")]
// 0xb20d20 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22SharedStringDictionaryEE7disposeEv
// type: void __fastcall(int)
pub fn stub_b20d20() -> ! {
    todo!("0xb20d20 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22SharedStringDictionaryEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::SharedStringDictionary>::get_deleter(std::type_info const&)")]
// 0xb20e64 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22SharedStringDictionaryEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_b20e64() -> ! {
    todo!("0xb20e64 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22SharedStringDictionaryEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::SharedStringDictionary>::get_untyped_deleter(void)")]
// 0xb20e68 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22SharedStringDictionaryEE19get_untyped_deleterEv
// type: int()
pub fn stub_b20e68() -> ! {
    todo!("0xb20e68 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22SharedStringDictionaryEE19get_untyped_deleterEv")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsSender,RBX::Network::TopNErrorsPhysicsSender>(rbx_core::SharedPtr<RBX::Network::PhysicsSender> *,RBX::Network::TopNErrorsPhysicsSender *,boost::detail::shared_count &)")]
// 0xb20e6c — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSenderENS3_23TopNErrorsPhysicsSenderEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
pub fn stub_b20e6c() -> ! {
    todo!("0xb20e6c __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSenderENS3_23TopNErrorsPhysicsSenderEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::TopNErrorsPhysicsSender>::~sp_counted_impl_p()")]
// 0xb21004 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEED1Ev
// type: void()
pub fn stub_b21004() -> ! {
    todo!("0xb21004 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::TopNErrorsPhysicsSender>::~sp_counted_impl_p()")]
// 0xb21008 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEED0Ev
// type: void __fastcall(void *)
pub fn stub_b21008() -> ! {
    todo!("0xb21008 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::TopNErrorsPhysicsSender>::dispose(void)")]
// 0xb21014 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEE7disposeEv
// type: int __fastcall(int)
pub fn stub_b21014() -> ! {
    todo!("0xb21014 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::TopNErrorsPhysicsSender>::get_deleter(std::type_info const&)")]
// 0xb21028 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_b21028() -> ! {
    todo!("0xb21028 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::TopNErrorsPhysicsSender>::get_untyped_deleter(void)")]
// 0xb2102c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEE19get_untyped_deleterEv
// type: int()
pub fn stub_b2102c() -> ! {
    todo!("0xb2102c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEE19get_untyped_deleterEv")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsSender,RBX::Network::RoundRobinPhysicsSender>(rbx_core::SharedPtr<RBX::Network::PhysicsSender> *,RBX::Network::RoundRobinPhysicsSender *,boost::detail::shared_count &)")]
// 0xb21030 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSenderENS3_23RoundRobinPhysicsSenderEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
pub fn stub_b21030() -> ! {
    todo!("0xb21030 __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSenderENS3_23RoundRobinPhysicsSenderEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::RoundRobinPhysicsSender>::~sp_counted_impl_p()")]
// 0xb211c8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEED1Ev
// type: void()
pub fn stub_b211c8() -> ! {
    todo!("0xb211c8 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::RoundRobinPhysicsSender>::~sp_counted_impl_p()")]
// 0xb211cc — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEED0Ev
// type: void __fastcall(void *)
pub fn stub_b211cc() -> ! {
    todo!("0xb211cc __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::RoundRobinPhysicsSender>::dispose(void)")]
// 0xb211d8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEE7disposeEv
// type: int __fastcall(int)
pub fn stub_b211d8() -> ! {
    todo!("0xb211d8 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::RoundRobinPhysicsSender>::get_deleter(std::type_info const&)")]
// 0xb211ec — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_b211ec() -> ! {
    todo!("0xb211ec __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::RoundRobinPhysicsSender>::get_untyped_deleter(void)")]
// 0xb211f0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEE19get_untyped_deleterEv
// type: int()
pub fn stub_b211f0() -> ! {
    todo!("0xb211f0 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEE19get_untyped_deleterEv")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsSender,RBX::Network::ErrorCompPhysicsSender2>(rbx_core::SharedPtr<RBX::Network::PhysicsSender> *,RBX::Network::ErrorCompPhysicsSender2 *,boost::detail::shared_count &)")]
// 0xb211f4 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSenderENS3_23ErrorCompPhysicsSender2EEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
pub fn stub_b211f4() -> ! {
    todo!("0xb211f4 __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSenderENS3_23ErrorCompPhysicsSender2EEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender2>::~sp_counted_impl_p()")]
// 0xb2138c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EED1Ev
// type: void()
pub fn stub_b2138c() -> ! {
    todo!("0xb2138c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender2>::~sp_counted_impl_p()")]
// 0xb21390 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EED0Ev
// type: void __fastcall(void *)
pub fn stub_b21390() -> ! {
    todo!("0xb21390 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender2>::dispose(void)")]
// 0xb2139c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EE7disposeEv
// type: int __fastcall(int)
pub fn stub_b2139c() -> ! {
    todo!("0xb2139c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender2>::get_deleter(std::type_info const&)")]
// 0xb213b0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EE11get_deleterERKSt9type_info
// type: int()
pub fn stub_b213b0() -> ! {
    todo!("0xb213b0 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender2>::get_untyped_deleter(void)")]
// 0xb213b4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EE19get_untyped_deleterEv
// type: int()
pub fn stub_b213b4() -> ! {
    todo!("0xb213b4 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EE19get_untyped_deleterEv")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsSender,RBX::Network::ErrorCompPhysicsSender>(rbx_core::SharedPtr<RBX::Network::PhysicsSender> *,RBX::Network::ErrorCompPhysicsSender *,boost::detail::shared_count &)")]
// 0xb213b8 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSenderENS3_22ErrorCompPhysicsSenderEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
pub fn stub_b213b8() -> ! {
    todo!("0xb213b8 __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSenderENS3_22ErrorCompPhysicsSenderEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::~sp_counted_impl_p()")]
// 0xb21550 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEED1Ev
// type: void()
pub fn stub_b21550() -> ! {
    todo!("0xb21550 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::~sp_counted_impl_p()")]
// 0xb21554 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEED0Ev
// type: void __fastcall(void *)
pub fn stub_b21554() -> ! {
    todo!("0xb21554 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::dispose(void)")]
// 0xb21560 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEE7disposeEv
// type: int __fastcall(int)
pub fn stub_b21560() -> ! {
    todo!("0xb21560 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::get_deleter(std::type_info const&)")]
// 0xb21574 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_b21574() -> ! {
    todo!("0xb21574 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::get_untyped_deleter(void)")]
// 0xb21578 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEE19get_untyped_deleterEv
// type: int()
pub fn stub_b21578() -> ! {
    todo!("0xb21578 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::list5(boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")]
// 0xb221c8 — __ZN5boost3_bi5list5INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEC2ES8_SA_SB_SC_SD_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int)
pub fn stub_b221c8() -> ! {
    todo!("0xb221c8 __ZN5boost3_bi5list5INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEC2ES8_SA_SB_SC_SD_")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::storage4(boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
// 0xb22618 — __ZN5boost3_bi8storage4INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEC2ES8_SA_SB_SC_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int)
pub fn stub_b22618() -> ! {
    todo!("0xb22618 __ZN5boost3_bi8storage4INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEC2ES8_SA_SB_SC_")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>,boost::arg<1>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>,boost::arg<1>)")]
// 0xb22a68 — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEENS_3argILi1EEEEC2ES8_SA_
// type: _DWORD *__fastcall(_DWORD *, unsigned int *, int, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, pthread_mutex_t *, int, int, int, int)
pub fn stub_b22a68() -> ! {
    todo!("0xb22a68 __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEENS_3argILi1EEEEC2ES8_SA_")
}

#[doc(alias = "RBX::Network::ClusterPacketCache * RBX::ServiceProvider::find<RBX::Network::ClusterPacketCache>(void)const")]
// 0xb22cb8 — __ZNK3RBX15ServiceProvider4findINS_7Network18ClusterPacketCacheEEEPT_v
// type: __guard *__fastcall(_DWORD *, int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
pub fn stub_b22cb8() -> ! {
    todo!("0xb22cb8 __ZNK3RBX15ServiceProvider4findINS_7Network18ClusterPacketCacheEEEPT_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7Network19sClusterPacketCacheEEEEvv")]
// 0xb2332c — __ZN3RBX4Name13callDoDeclareILZNS_7Network19sClusterPacketCacheEEEEvv
// type: void()
pub fn stub_b2332c() -> ! {
    todo!("0xb2332c __ZN3RBX4Name13callDoDeclareILZNS_7Network19sClusterPacketCacheEEEEvv")
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::Network::ClusterPacketCache>(void)")]
// 0xb23400 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_7Network18ClusterPacketCacheEEEvv
// type: void()
pub fn stub_b23400() -> ! {
    todo!("0xb23400 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_7Network18ClusterPacketCacheEEEvv")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::Replicator::PingJob,RBX::Network::Replicator::PingJob>(rbx_core::SharedPtr<RBX::Network::Replicator::PingJob> *,RBX::Network::Replicator::PingJob *,boost::detail::shared_count &)")]
// 0xb23cd8 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator7PingJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
pub fn stub_b23cd8() -> ! {
    todo!("0xb23cd8 __ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator7PingJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::Replicator::PingJob,RBX::Network::Replicator::PingJob>(rbx_core::SharedPtr<RBX::Network::Replicator::PingJob> const*,RBX::Network::Replicator::PingJob *)const")]
// 0xb23e88 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator7PingJobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
pub fn stub_b23e88() -> ! {
    todo!("0xb23e88 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator7PingJobES8_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::PingJob>::~sp_counted_impl_p()")]
// 0xb24134 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEED1Ev
// type: void()
pub fn stub_b24134() -> ! {
    todo!("0xb24134 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::PingJob>::~sp_counted_impl_p()")]
// 0xb24138 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEED0Ev
// type: void __fastcall(void *)
pub fn stub_b24138() -> ! {
    todo!("0xb24138 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::PingJob>::dispose(void)")]
// 0xb24144 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEE7disposeEv
// type: int __fastcall(int)
pub fn stub_b24144() -> ! {
    todo!("0xb24144 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::PingJob>::get_deleter(std::type_info const&)")]
// 0xb24158 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_b24158() -> ! {
    todo!("0xb24158 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::PingJob>::get_untyped_deleter(void)")]
// 0xb2415c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEE19get_untyped_deleterEv
// type: int()
pub fn stub_b2415c() -> ! {
    todo!("0xb2415c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEE19get_untyped_deleterEv")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::Replicator::ProcessPacketsJob,RBX::Network::Replicator::ProcessPacketsJob>(rbx_core::SharedPtr<RBX::Network::Replicator::ProcessPacketsJob> *,RBX::Network::Replicator::ProcessPacketsJob *,boost::detail::shared_count &)")]
// 0xb24160 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator17ProcessPacketsJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
pub fn stub_b24160() -> ! {
    todo!("0xb24160 __ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator17ProcessPacketsJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::Replicator::ProcessPacketsJob,RBX::Network::Replicator::ProcessPacketsJob>(rbx_core::SharedPtr<RBX::Network::Replicator::ProcessPacketsJob> const*,RBX::Network::Replicator::ProcessPacketsJob *)const")]
// 0xb24310 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator17ProcessPacketsJobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
pub fn stub_b24310() -> ! {
    todo!("0xb24310 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator17ProcessPacketsJobES8_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::ProcessPacketsJob>::~sp_counted_impl_p()")]
// 0xb245bc — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEED1Ev
// type: void()
pub fn stub_b245bc() -> ! {
    todo!("0xb245bc __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::ProcessPacketsJob>::~sp_counted_impl_p()")]
// 0xb245c0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEED0Ev
// type: void __fastcall(void *)
pub fn stub_b245c0() -> ! {
    todo!("0xb245c0 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::ProcessPacketsJob>::dispose(void)")]
// 0xb245cc — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEE7disposeEv
// type: int __fastcall(int)
pub fn stub_b245cc() -> ! {
    todo!("0xb245cc __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::ProcessPacketsJob>::get_deleter(std::type_info const&)")]
// 0xb245e0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_b245e0() -> ! {
    todo!("0xb245e0 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::ProcessPacketsJob>::get_untyped_deleter(void)")]
// 0xb245e4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEE19get_untyped_deleterEv
// type: int()
pub fn stub_b245e4() -> ! {
    todo!("0xb245e4 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEE19get_untyped_deleterEv")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::Replicator::SendClusterJob,RBX::Network::Replicator::SendClusterJob>(rbx_core::SharedPtr<RBX::Network::Replicator::SendClusterJob> *,RBX::Network::Replicator::SendClusterJob *,boost::detail::shared_count &)")]
// 0xb245e8 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator14SendClusterJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
pub fn stub_b245e8() -> ! {
    todo!("0xb245e8 __ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator14SendClusterJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::Replicator::SendClusterJob,RBX::Network::Replicator::SendClusterJob>(rbx_core::SharedPtr<RBX::Network::Replicator::SendClusterJob> const*,RBX::Network::Replicator::SendClusterJob *)const")]
// 0xb24798 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator14SendClusterJobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
pub fn stub_b24798() -> ! {
    todo!("0xb24798 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator14SendClusterJobES8_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendClusterJob>::~sp_counted_impl_p()")]
// 0xb24a44 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEED1Ev
// type: void()
pub fn stub_b24a44() -> ! {
    todo!("0xb24a44 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendClusterJob>::~sp_counted_impl_p()")]
// 0xb24a48 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEED0Ev
// type: void __fastcall(void *)
pub fn stub_b24a48() -> ! {
    todo!("0xb24a48 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendClusterJob>::dispose(void)")]
// 0xb24a54 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEE7disposeEv
// type: int __fastcall(int)
pub fn stub_b24a54() -> ! {
    todo!("0xb24a54 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEE7disposeEv")
}
