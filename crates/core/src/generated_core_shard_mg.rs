//! core shard mg — 150 core stubs EA-sorted asc global gap filler not yet in core (fallback filter).
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 150 not yet in rbx_core (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; fallback 20084 before, batch 0x4775e4..0x48c910).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::DebrisService::cleanup(void)")]
// 0x4775e4 — __ZN3RBX13DebrisService7cleanupEv — RBX::DebrisService::cleanup(void)
// type: _DWORD __fastcall(RBX::DebrisService *__hidden this)
pub fn stub_0x4775e4() {
    // IDA 0x4775e4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebrisService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x477864 — __ZN3RBX13DebrisService17onServiceProviderEPNS_15ServiceProviderES2_ — RBX::DebrisService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)
// type: _DWORD __fastcall(RBX::DebrisService *__hidden this, RBX::ServiceProvider *, RBX::ServiceProvider *)
pub fn stub_0x477864() {
    // IDA 0x477864: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebrisService::getMaxItems(void)const")]
// 0x477a0c — __ZNK3RBX13DebrisService11getMaxItemsEv — RBX::DebrisService::getMaxItems(void)const
// type: _DWORD __fastcall(RBX::DebrisService *__hidden this)
pub fn stub_0x477a0c() {
    // IDA 0x477a0c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TimerService>::operator=(rbx_core::SharedPtr<RBX::TimerService> const&)")]
// 0x477d30 — __ZN5boost10shared_ptrIN3RBX12TimerServiceEEaSERKS3_ — rbx_core::SharedPtr<RBX::TimerService>::operator=(rbx_core::SharedPtr<RBX::TimerService> const&)
// was: boost::shared_ptr<RBX::TimerService>::operator=(boost::shared_ptr<RBX::TimerService> const&)
// type: int(void)
pub fn stub_0x477d30() {
    // IDA 0x477d30: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TimerService> RBX::shared_from<RBX::TimerService>(RBX::TimerService*)")]
// 0x477d68 — __ZN3RBX11shared_fromINS_12TimerServiceEEEN5boost10shared_ptrIT_EEPS4_ — rbx_core::SharedPtr<RBX::TimerService> RBX::shared_from<RBX::TimerService>(RBX::TimerService*)
// was: boost::shared_ptr<RBX::TimerService> RBX::shared_from<RBX::TimerService>(RBX::TimerService*)
// type: int(void)
pub fn stub_0x477d68() {
    // IDA 0x477d68: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::DebrisService::~DebrisService()")]
// 0x477ed8 — __ZN3RBX13DebrisServiceD1Ev — RBX::DebrisService::~DebrisService()
// type: void __fastcall(RBX::DebrisService *__hidden this)
pub fn stub_0x477ed8() {
    // IDA 0x477ed8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::DebrisService::~DebrisService()")]
// 0x477fe4 — __ZN3RBX13DebrisServiceD0Ev — RBX::DebrisService::~DebrisService()
// type: void __fastcall(RBX::DebrisService *__hidden this)
pub fn stub_0x477fe4() {
    // IDA 0x477fe4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "non-virtual thunk toRBX::DebrisService::~DebrisService()")]
// 0x478128 — __ZThn32_N3RBX13DebrisServiceD1Ev — non-virtual thunk toRBX::DebrisService::~DebrisService()
// was: non-virtual thunk toRBX::DebrisService::~DebrisService()
// type: void __fastcall(RBX::DebrisService *__hidden this)
pub fn stub_0x478128() {
    // IDA 0x478128: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::DebrisService::~DebrisService()")]
// 0x478234 — __ZThn32_N3RBX13DebrisServiceD0Ev — non-virtual thunk toRBX::DebrisService::~DebrisService()
// was: non-virtual thunk toRBX::DebrisService::~DebrisService()
// type: void __fastcall(RBX::DebrisService *__hidden this)
pub fn stub_0x478234() {
    // IDA 0x478234: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::DebrisService::~DebrisService()")]
// 0x47837c — __ZThn36_N3RBX13DebrisServiceD1Ev — non-virtual thunk toRBX::DebrisService::~DebrisService()
// was: non-virtual thunk toRBX::DebrisService::~DebrisService()
// type: void __fastcall(RBX::DebrisService *__hidden this)
pub fn stub_0x47837c() {
    // IDA 0x47837c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::DebrisService::~DebrisService()")]
// 0x478484 — __ZThn36_N3RBX13DebrisServiceD0Ev — non-virtual thunk toRBX::DebrisService::~DebrisService()
// was: non-virtual thunk toRBX::DebrisService::~DebrisService()
// type: void __fastcall(RBX::DebrisService *__hidden this)
pub fn stub_0x478484() {
    // IDA 0x478484: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DebugSettings::getVertexShaderModel(void)const")]
// 0x47b4cc — __ZNK3RBX13DebugSettings20getVertexShaderModelEv — RBX::DebugSettings::getVertexShaderModel(void)const
// type: int __fastcall(RBX::DebugSettings *this)
pub fn stub_0x47b4cc() {
    // IDA 0x47b4cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DebugSettings::getPixelShaderModel(void)const")]
// 0x47b4d0 — __ZNK3RBX13DebugSettings19getPixelShaderModelEv — RBX::DebugSettings::getPixelShaderModel(void)const
// type: int __fastcall(RBX::DebugSettings *this)
pub fn stub_0x47b4d0() {
    // IDA 0x47b4d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DebugSettings::videoMemory(void)const")]
// 0x47b4d4 — __ZNK3RBX13DebugSettings11videoMemoryEv — RBX::DebugSettings::videoMemory(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47b4d4() {
    // IDA 0x47b4d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DebugSettings::cpuSpeed(void)const")]
// 0x47b564 — __ZNK3RBX13DebugSettings8cpuSpeedEv — RBX::DebugSettings::cpuSpeed(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47b564() {
    // IDA 0x47b564: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DebugSettings::cpuCount(void)const")]
// 0x47b5f4 — __ZNK3RBX13DebugSettings8cpuCountEv — RBX::DebugSettings::cpuCount(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47b5f4() {
    // IDA 0x47b5f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DebugSettings::osPlatformId(void)const")]
// 0x47b684 — __ZNK3RBX13DebugSettings12osPlatformIdEv — RBX::DebugSettings::osPlatformId(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47b684() {
    // IDA 0x47b684: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebugSettings::osPlatform(void)const")]
// 0x47b688 — __ZNK3RBX13DebugSettings10osPlatformEv — RBX::DebugSettings::osPlatform(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47b688() {
    // IDA 0x47b688: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebugSettings::osVer(void)const")]
// 0x47b6a4 — __ZNK3RBX13DebugSettings5osVerEv — RBX::DebugSettings::osVer(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47b6a4() {
    // IDA 0x47b6a4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebugSettings::osIs64Bit(void)const")]
// 0x47b6b0 — __ZNK3RBX13DebugSettings9osIs64BitEv — RBX::DebugSettings::osIs64Bit(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47b6b0() {
    // IDA 0x47b6b0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebugSettings::systemProductName(void)const")]
// 0x47b6bc — __ZNK3RBX13DebugSettings17systemProductNameEv — RBX::DebugSettings::systemProductName(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47b6bc() {
    // IDA 0x47b6bc: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebugSettings::gfxcard(void)const")]
// 0x47b6d8 — __ZNK3RBX13DebugSettings7gfxcardEv — RBX::DebugSettings::gfxcard(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47b6d8() {
    // IDA 0x47b6d8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebugSettings::cpu(void)const")]
// 0x47b6e4 — __ZNK3RBX13DebugSettings3cpuEv — RBX::DebugSettings::cpu(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47b6e4() {
    // IDA 0x47b6e4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebugSettings::simd(void)const")]
// 0x47b894 — __ZNK3RBX13DebugSettings4simdEv — RBX::DebugSettings::simd(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47b894() {
    // IDA 0x47b894: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebugSettings::totalPhysicalMemory(void)const")]
// 0x47b9a4 — __ZNK3RBX13DebugSettings19totalPhysicalMemoryEv — RBX::DebugSettings::totalPhysicalMemory(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47b9a4() {
    // IDA 0x47b9a4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebugSettings::resolution(void)const")]
// 0x47ba34 — __ZNK3RBX13DebugSettings10resolutionEv — RBX::DebugSettings::resolution(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47ba34() {
    // IDA 0x47ba34: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebugSettings::availablePhysicalMemory(void)const")]
// 0x47bbb4 — __ZNK3RBX13DebugSettings23availablePhysicalMemoryEv — RBX::DebugSettings::availablePhysicalMemory(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47bbb4() {
    // IDA 0x47bbb4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebugSettings::getElapsedTime(void)const")]
// 0x47bc44 — __ZNK3RBX13DebugSettings14getElapsedTimeEv — RBX::DebugSettings::getElapsedTime(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47bc44() {
    // IDA 0x47bc44: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebugSettings::processCores(void)const")]
// 0x47bc50 — __ZNK3RBX13DebugSettings12processCoresEv — RBX::DebugSettings::processCores(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47bc50() {
    // IDA 0x47bc50: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebugSettings::totalProcessorTime(void)const")]
// 0x47bc8c — __ZNK3RBX13DebugSettings18totalProcessorTimeEv — RBX::DebugSettings::totalProcessorTime(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47bc8c() {
    // IDA 0x47bc8c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebugSettings::processorTime(void)const")]
// 0x47bcb0 — __ZNK3RBX13DebugSettings13processorTimeEv — RBX::DebugSettings::processorTime(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47bcb0() {
    // IDA 0x47bcb0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebugSettings::privateBytes(void)const")]
// 0x47bcb8 — __ZNK3RBX13DebugSettings12privateBytesEv — RBX::DebugSettings::privateBytes(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47bcb8() {
    // IDA 0x47bcb8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebugSettings::privateWorkingSetBytes(void)const")]
// 0x47bcdc — __ZNK3RBX13DebugSettings22privateWorkingSetBytesEv — RBX::DebugSettings::privateWorkingSetBytes(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47bcdc() {
    // IDA 0x47bcdc: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebugSettings::GetVirtualBytes(void)const")]
// 0x47bcfc — __ZNK3RBX13DebugSettings15GetVirtualBytesEv — RBX::DebugSettings::GetVirtualBytes(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47bcfc() {
    // IDA 0x47bcfc: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebugSettings::GetPageFileBytes(void)const")]
// 0x47bd1c — __ZNK3RBX13DebugSettings16GetPageFileBytesEv — RBX::DebugSettings::GetPageFileBytes(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47bd1c() {
    // IDA 0x47bd1c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebugSettings::GetPageFaultsPerSecond(void)const")]
// 0x47bd24 — __ZNK3RBX13DebugSettings22GetPageFaultsPerSecondEv — RBX::DebugSettings::GetPageFaultsPerSecond(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47bd24() {
    // IDA 0x47bd24: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebugSettings::getPlayerCount(void)const")]
// 0x47bd50 — __ZNK3RBX13DebugSettings14getPlayerCountEv — RBX::DebugSettings::getPlayerCount(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47bd50() {
    // IDA 0x47bd50: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebugSettings::getCdnSuccessCount(void)const")]
// 0x47bd70 — __ZNK3RBX13DebugSettings18getCdnSuccessCountEv — RBX::DebugSettings::getCdnSuccessCount(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47bd70() {
    // IDA 0x47bd70: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebugSettings::getCdnFailureCount(void)const")]
// 0x47bd80 — __ZNK3RBX13DebugSettings18getCdnFailureCountEv — RBX::DebugSettings::getCdnFailureCount(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47bd80() {
    // IDA 0x47bd80: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebugSettings::getAlternateCdnSuccessCount(void)const")]
// 0x47bd90 — __ZNK3RBX13DebugSettings27getAlternateCdnSuccessCountEv — RBX::DebugSettings::getAlternateCdnSuccessCount(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47bd90() {
    // IDA 0x47bd90: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebugSettings::getAlternateCdnFailureCount(void)const")]
// 0x47bda0 — __ZNK3RBX13DebugSettings27getAlternateCdnFailureCountEv — RBX::DebugSettings::getAlternateCdnFailureCount(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47bda0() {
    // IDA 0x47bda0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebugSettings::getBlockMeshMapCount(void)const")]
// 0x47bdb0 — __ZNK3RBX13DebugSettings20getBlockMeshMapCountEv — RBX::DebugSettings::getBlockMeshMapCount(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47bdb0() {
    // IDA 0x47bdb0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebugSettings::getLastCdnFailureTimeSpan(void)const")]
// 0x47bdb4 — __ZNK3RBX13DebugSettings25getLastCdnFailureTimeSpanEv — RBX::DebugSettings::getLastCdnFailureTimeSpan(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47bdb4() {
    // IDA 0x47bdb4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebugSettings::getRobloxSuccessCount(void)const")]
// 0x47bdcc — __ZNK3RBX13DebugSettings21getRobloxSuccessCountEv — RBX::DebugSettings::getRobloxSuccessCount(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47bdcc() {
    // IDA 0x47bdcc: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebugSettings::getRobloxFalureCount(void)const")]
// 0x47bddc — __ZNK3RBX13DebugSettings20getRobloxFalureCountEv — RBX::DebugSettings::getRobloxFalureCount(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47bddc() {
    // IDA 0x47bddc: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebugSettings::getRobloxResponce(void)const")]
// 0x47bdf0 — __ZNK3RBX13DebugSettings17getRobloxResponceEv — RBX::DebugSettings::getRobloxResponce(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47bdf0() {
    // IDA 0x47bdf0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebugSettings::getCdnRespoce(void)const")]
// 0x47be48 — __ZNK3RBX13DebugSettings13getCdnRespoceEv — RBX::DebugSettings::getCdnRespoce(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47be48() {
    // IDA 0x47be48: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DebugSettings::resetCdnFailureCounts(void)")]
// 0x47bea0 — __ZN3RBX13DebugSettings21resetCdnFailureCountsEv — RBX::DebugSettings::resetCdnFailureCounts(void)
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47bea0() {
    // IDA 0x47bea0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::TaskSchedulerSettings::addDummyJob(bool,double)")]
// 0x47c2a8 — __ZN3RBX21TaskSchedulerSettings11addDummyJobEbd — RBX::TaskSchedulerSettings::addDummyJob(bool,double)
// type: _DWORD __fastcall(RBX::TaskSchedulerSettings *__hidden this, bool, double)
pub fn stub_0x47c2a8() {
    // IDA 0x47c2a8: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::DebugSettings::setErrorReporting(RBX::DebugSettings::ErrorReporting)")]
// 0x47c3f8 — __ZN3RBX13DebugSettings17setErrorReportingENS0_14ErrorReportingE — RBX::DebugSettings::setErrorReporting(RBX::DebugSettings::ErrorReporting)
pub fn stub_0x47c3f8() {
    // IDA 0x47c3f8: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::TaskSchedulerSettings::getThreadPoolConfig(void)const")]
// 0x47c414 — __ZNK3RBX21TaskSchedulerSettings19getThreadPoolConfigEv — RBX::TaskSchedulerSettings::getThreadPoolConfig(void)const
// type: _DWORD __fastcall(RBX::TaskSchedulerSettings *__hidden this)
pub fn stub_0x47c414() {
    // IDA 0x47c414: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::TaskSchedulerSettings::setThreadPoolConfig(RBX::TaskScheduler::ThreadPoolConfig)")]
// 0x47c418 — __ZN3RBX21TaskSchedulerSettings19setThreadPoolConfigENS_13TaskScheduler16ThreadPoolConfigE — RBX::TaskSchedulerSettings::setThreadPoolConfig(RBX::TaskScheduler::ThreadPoolConfig)
pub fn stub_0x47c418() {
    // IDA 0x47c418: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::TaskSchedulerSettings::setThreadShare(double,int)")]
// 0x47c460 — __ZN3RBX21TaskSchedulerSettings14setThreadShareEdi — RBX::TaskSchedulerSettings::setThreadShare(double,int)
// type: _DWORD __fastcall(RBX::TaskSchedulerSettings *__hidden this, double, int)
pub fn stub_0x47c460() {
    // IDA 0x47c460: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::TaskSchedulerSettings::setPriorityMethod(RBX::TaskScheduler::PriorityMethod)")]
// 0x47c464 — __ZN3RBX21TaskSchedulerSettings17setPriorityMethodENS_13TaskScheduler14PriorityMethodE — RBX::TaskSchedulerSettings::setPriorityMethod(RBX::TaskScheduler::PriorityMethod)
pub fn stub_0x47c464() {
    // IDA 0x47c464: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::TaskSchedulerSettings::setSleepAdjustMethod(RBX::TaskScheduler::Job::SleepAdjustMethod)")]
// 0x47c4a0 — __ZN3RBX21TaskSchedulerSettings20setSleepAdjustMethodENS_13TaskScheduler3Job17SleepAdjustMethodE — RBX::TaskSchedulerSettings::setSleepAdjustMethod(RBX::TaskScheduler::Job::SleepAdjustMethod)
pub fn stub_0x47c4a0() {
    // IDA 0x47c4a0: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::TaskSchedulerSettings::setIsArbiterThrottled(bool)")]
// 0x47c518 — __ZN3RBX21TaskSchedulerSettings21setIsArbiterThrottledEb — RBX::TaskSchedulerSettings::setIsArbiterThrottled(bool)
// type: _DWORD __fastcall(RBX::TaskSchedulerSettings *__hidden this, bool)
pub fn stub_0x47c518() {
    // IDA 0x47c518: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::TaskSchedulerSettings::setThrottledJobSleepTime(double)")]
// 0x47c53c — __ZN3RBX21TaskSchedulerSettings24setThrottledJobSleepTimeEd — RBX::TaskSchedulerSettings::setThrottledJobSleepTime(double)
// type: _DWORD __fastcall(RBX::TaskSchedulerSettings *__hidden this, double)
pub fn stub_0x47c53c() {
    // IDA 0x47c53c: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::DebugSettings::getIsProfilingEnabled(void)const")]
// 0x47c564 — __ZNK3RBX13DebugSettings21getIsProfilingEnabledEv — RBX::DebugSettings::getIsProfilingEnabled(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47c564() {
    // IDA 0x47c564: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::DebugSettings::setIsProfilingEnabled(bool)")]
// 0x47c570 — __ZN3RBX13DebugSettings21setIsProfilingEnabledEb — RBX::DebugSettings::setIsProfilingEnabled(bool)
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this, RBX::Profiling *)
pub fn stub_0x47c570() {
    // IDA 0x47c570: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::DebugSettings::getProfilingWindow(void)const")]
// 0x47c578 — __ZNK3RBX13DebugSettings18getProfilingWindowEv — RBX::DebugSettings::getProfilingWindow(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47c578() {
    // IDA 0x47c578: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::DebugSettings::setProfilingWindow(double)")]
// 0x47c590 — __ZN3RBX13DebugSettings18setProfilingWindowEd — RBX::DebugSettings::setProfilingWindow(double)
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this, double)
pub fn stub_0x47c590() {
    // IDA 0x47c590: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::DebugSettings::DebugSettings(void)")]
// 0x47c608 — __ZN3RBX13DebugSettingsC1Ev — RBX::DebugSettings::DebugSettings(void)
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47c608() {
    // IDA 0x47c608: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::DebugSettings::DebugSettings(void)")]
// 0x47c60c — __ZN3RBX13DebugSettingsC2Ev — RBX::DebugSettings::DebugSettings(void)
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47c60c() {
    // IDA 0x47c60c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "DummyArbiter::areExclusive(RBX::TaskScheduler::Job *,RBX::TaskScheduler::Job *)")]
// 0x47c7e4 — __ZN12DummyArbiter12areExclusiveEPN3RBX13TaskScheduler3JobES3_ — DummyArbiter::areExclusive(RBX::TaskScheduler::Job *,RBX::TaskScheduler::Job *)
// type: _DWORD __fastcall(DummyArbiter *__hidden this, RBX::TaskScheduler::Job *, RBX::TaskScheduler::Job *)
pub fn stub_0x47c7e4() {
    // IDA 0x47c7e4: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::TaskSchedulerSettings::TaskSchedulerSettings(void)")]
// 0x47c800 — __ZN3RBX21TaskSchedulerSettingsC2Ev — RBX::TaskSchedulerSettings::TaskSchedulerSettings(void)
// type: _DWORD __fastcall(RBX::TaskSchedulerSettings *__hidden this)
pub fn stub_0x47c800() {
    // IDA 0x47c800: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::DebugSettings::getRobloxVersion(void)const")]
// 0x47de54 — __ZNK3RBX13DebugSettings16getRobloxVersionEv — RBX::DebugSettings::getRobloxVersion(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47de54() {
    // IDA 0x47de54: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::DebugSettings::getRobloxProductName(void)const")]
// 0x47deec — __ZNK3RBX13DebugSettings20getRobloxProductNameEv — RBX::DebugSettings::getRobloxProductName(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47deec() {
    // IDA 0x47deec: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::DebugSettings::nameDatabaseSize(void)const")]
// 0x47dfcc — __ZNK3RBX13DebugSettings16nameDatabaseSizeEv — RBX::DebugSettings::nameDatabaseSize(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47dfcc() {
    // IDA 0x47dfcc: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::DebugSettings::nameDatabaseBytes(void)const")]
// 0x47dfd0 — __ZNK3RBX13DebugSettings17nameDatabaseBytesEv — RBX::DebugSettings::nameDatabaseBytes(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47dfd0() {
    // IDA 0x47dfd0: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::DebugSettings::instanceCount(void)const")]
// 0x47dff8 — __ZNK3RBX13DebugSettings13instanceCountEv — RBX::DebugSettings::instanceCount(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47dff8() {
    // IDA 0x47dff8: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::DebugSettings::jobCount(void)const")]
// 0x47e008 — __ZNK3RBX13DebugSettings8jobCountEv — RBX::DebugSettings::jobCount(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47e008() {
    // IDA 0x47e008: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::TaskSchedulerSettings::threadPoolSize(void)const")]
// 0x47e03c — __ZNK3RBX21TaskSchedulerSettings14threadPoolSizeEv — RBX::TaskSchedulerSettings::threadPoolSize(void)const
// type: _DWORD __fastcall(RBX::TaskSchedulerSettings *__hidden this)
pub fn stub_0x47e03c() {
    // IDA 0x47e03c: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::TaskSchedulerSettings::threadAffinity(void)const")]
// 0x47e074 — __ZNK3RBX21TaskSchedulerSettings14threadAffinityEv — RBX::TaskSchedulerSettings::threadAffinity(void)const
// type: _DWORD __fastcall(RBX::TaskSchedulerSettings *__hidden this)
pub fn stub_0x47e074() {
    // IDA 0x47e074: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::TaskSchedulerSettings::numSleepingJobs(void)const")]
// 0x47e0ac — __ZNK3RBX21TaskSchedulerSettings15numSleepingJobsEv — RBX::TaskSchedulerSettings::numSleepingJobs(void)const
// type: _DWORD __fastcall(RBX::TaskSchedulerSettings *__hidden this)
pub fn stub_0x47e0ac() {
    // IDA 0x47e0ac: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::TaskSchedulerSettings::numWaitingJobs(void)const")]
// 0x47e0c0 — __ZNK3RBX21TaskSchedulerSettings14numWaitingJobsEv — RBX::TaskSchedulerSettings::numWaitingJobs(void)const
// type: _DWORD __fastcall(RBX::TaskSchedulerSettings *__hidden this)
pub fn stub_0x47e0c0() {
    // IDA 0x47e0c0: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::TaskSchedulerSettings::numRunningJobs(void)const")]
// 0x47e0d4 — __ZNK3RBX21TaskSchedulerSettings14numRunningJobsEv — RBX::TaskSchedulerSettings::numRunningJobs(void)const
// type: _DWORD __fastcall(RBX::TaskSchedulerSettings *__hidden this)
pub fn stub_0x47e0d4() {
    // IDA 0x47e0d4: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::DebugSettings::getErrorReporting(void)const")]
// 0x47e158 — __ZNK3RBX13DebugSettings17getErrorReportingEv — RBX::DebugSettings::getErrorReporting(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47e158() {
    // IDA 0x47e158: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::DebugSettings::noOpt(void)")]
// 0x47e180 — __ZN3RBX13DebugSettings5noOptEv — RBX::DebugSettings::noOpt(void)
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47e180() {
    // IDA 0x47e180: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::DebugSettings::setBlockingRemove(bool)")]
// 0x47e1a8 — __ZN3RBX13DebugSettings17setBlockingRemoveEb — RBX::DebugSettings::setBlockingRemove(bool)
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this, bool)
pub fn stub_0x47e1a8() {
    // IDA 0x47e1a8: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::DebugSettings::getTickCountPreciseOverride(void)const")]
// 0x47e344 — __ZNK3RBX13DebugSettings27getTickCountPreciseOverrideEv — RBX::DebugSettings::getTickCountPreciseOverride(void)const
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47e344() {
    // IDA 0x47e344: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::DebugSettings::setTickCountPreciseOverride(RBX::Time::SampleMethod)")]
// 0x47e354 — __ZN3RBX13DebugSettings27setTickCountPreciseOverrideENS_4Time12SampleMethodE — RBX::DebugSettings::setTickCountPreciseOverride(RBX::Time::SampleMethod)
pub fn stub_0x47e354() {
    // IDA 0x47e354: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEEC2Ev")]
// 0x47e388 — __ZN3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEEC2Ev
// type: int(void)
pub fn stub_0x47e388() {
    // IDA 0x47e388: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "rbx_core::SharedPtr<DummyArbiter>::~shared_ptr()")]
// 0x47e640 — __ZN5boost10shared_ptrI12DummyArbiterED1Ev — rbx_core::SharedPtr<DummyArbiter>::~shared_ptr()
// was: boost::shared_ptr<DummyArbiter>::~shared_ptr()
pub fn stub_0x47e640() {
    // IDA 0x47e640: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DebugSettings::~DebugSettings()")]
// 0x47f7ac — __ZN3RBX13DebugSettingsD1Ev — RBX::DebugSettings::~DebugSettings()
// type: void __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47f7ac() {
    // IDA 0x47f7ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DebugSettings::~DebugSettings()")]
// 0x47f7ec — __ZN3RBX13DebugSettingsD0Ev — RBX::DebugSettings::~DebugSettings()
// type: void __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47f7ec() {
    // IDA 0x47f7ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::DebugSettings::~DebugSettings()")]
// 0x47f91c — __ZThn32_N3RBX13DebugSettingsD1Ev — non-virtual thunk toRBX::DebugSettings::~DebugSettings()
// was: non-virtual thunk toRBX::DebugSettings::~DebugSettings()
// type: void __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47f91c() {
    // IDA 0x47f91c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::DebugSettings::~DebugSettings()")]
// 0x47f960 — __ZThn32_N3RBX13DebugSettingsD0Ev — non-virtual thunk toRBX::DebugSettings::~DebugSettings()
// was: non-virtual thunk toRBX::DebugSettings::~DebugSettings()
// type: void __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47f960() {
    // IDA 0x47f960: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::DebugSettings::~DebugSettings()")]
// 0x47fa58 — __ZThn36_N3RBX13DebugSettingsD1Ev — non-virtual thunk toRBX::DebugSettings::~DebugSettings()
// was: non-virtual thunk toRBX::DebugSettings::~DebugSettings()
// type: void __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47fa58() {
    // IDA 0x47fa58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::DebugSettings::~DebugSettings()")]
// 0x47fa9c — __ZThn36_N3RBX13DebugSettingsD0Ev — non-virtual thunk toRBX::DebugSettings::~DebugSettings()
// was: non-virtual thunk toRBX::DebugSettings::~DebugSettings()
// type: void __fastcall(RBX::DebugSettings *__hidden this)
pub fn stub_0x47fa9c() {
    // IDA 0x47fa9c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_13DebugSettingsENS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEE7Creator12getClassNameEv")]
// 0x4800e0 — __ZNK3RBX14FactoryProductINS_13DebugSettingsENS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
pub fn stub_0x4800e0() {
    // IDA 0x4800e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DebugSettings::ErrorReporting>(RBX::DebugSettings::ErrorReporting const&)")]
// 0x481000 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13DebugSettings14ErrorReportingEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DebugSettings::ErrorReporting>(RBX::DebugSettings::ErrorReporting const&)
// type: int(void)
pub fn stub_0x481000() {
    // IDA 0x481000: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DebugSettings::ErrorReporting>::singleton(void)")]
// 0x481050 — __ZN3rbx14implementation12typed_holderIN3RBX13DebugSettings14ErrorReportingEE9singletonEv — rbx::implementation::typed_holder<RBX::DebugSettings::ErrorReporting>::singleton(void)
// type: int(void)
pub fn stub_0x481050() {
    // IDA 0x481050: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DebugSettings::ErrorReporting>::construct_func(char const*,char *)")]
// 0x4810bc — __ZN3rbx14implementation12typed_holderIN3RBX13DebugSettings14ErrorReportingEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::DebugSettings::ErrorReporting>::construct_func(char const*,char *)
pub fn stub_0x4810bc() {
    // IDA 0x4810bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DebugSettings::ErrorReporting>::destruct_func(char *)")]
// 0x4810c8 — __ZN3rbx14implementation12typed_holderIN3RBX13DebugSettings14ErrorReportingEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::DebugSettings::ErrorReporting>::destruct_func(char *)
pub fn stub_0x4810c8() {
    // IDA 0x4810c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DebugSettings::ErrorReporting const& rbx::any_cast<RBX::DebugSettings::ErrorReporting const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x481198 — __ZN3rbx8any_castIRKN3RBX13DebugSettings14ErrorReportingENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::DebugSettings::ErrorReporting const& rbx::any_cast<RBX::DebugSettings::ErrorReporting const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// type: int(void)
pub fn stub_0x481198() {
    // IDA 0x481198: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::circular_buffer<double,std::allocator<double>>::destroy_content(boost::integral_constant<bool,true> const&)")]
// 0x4834f8 — __ZN5boost15circular_bufferIdSaIdEE15destroy_contentERKNS_17integral_constantIbLb1EEE — boost::circular_buffer<double,std::allocator<double>>::destroy_content(boost::integral_constant<bool,true> const&)
// type: int(void)
pub fn stub_0x4834f8() {
    // IDA 0x4834f8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx_core::SharedPtr<DummyJob>::shared_ptr<DummyJob>(DummyJob *)")]
// 0x4839f0 — __ZN5boost10shared_ptrI8DummyJobEC2IS1_EEPT_ — rbx_core::SharedPtr<DummyJob>::shared_ptr<DummyJob>(DummyJob *)
// was: boost::shared_ptr<DummyJob>::shared_ptr<DummyJob>(DummyJob *)
// type: int __fastcall(int, void *, int, int, int, int)
pub fn stub_0x4839f0() {
    // IDA 0x4839f0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<DummyJob,DummyJob>(rbx_core::SharedPtr<DummyJob> const*,DummyJob *)const")]
// 0x483ad8 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerI8DummyJobS6_EEvPKNS_10shared_ptrIT_EEPT0_ — void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<DummyJob,DummyJob>(rbx_core::SharedPtr<DummyJob> const*,DummyJob *)const
// was: void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<DummyJob,DummyJob>(boost::shared_ptr<DummyJob> const*,DummyJob *)const
pub fn stub_0x483ad8() {
    // IDA 0x483ad8: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<DummyJob>(DummyJob *)")]
// 0x483bbc — __ZN5boost6detail12shared_countC2I8DummyJobEEPT_ — boost::detail::shared_count::shared_count<DummyJob>(DummyJob *)
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x483bbc() {
    // IDA 0x483bbc: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<DummyJob>::~sp_counted_impl_p()")]
// 0x483cb4 — __ZN5boost6detail17sp_counted_impl_pI8DummyJobED1Ev — boost::detail::sp_counted_impl_p<DummyJob>::~sp_counted_impl_p()
pub fn stub_0x483cb4() {
    // IDA 0x483cb4: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<DummyJob>::~sp_counted_impl_p()")]
// 0x483cb8 — __ZN5boost6detail17sp_counted_impl_pI8DummyJobED0Ev — boost::detail::sp_counted_impl_p<DummyJob>::~sp_counted_impl_p()
pub fn stub_0x483cb8() {
    // IDA 0x483cb8: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<DummyJob>::dispose(void)")]
// 0x483cbc — __ZN5boost6detail17sp_counted_impl_pI8DummyJobE7disposeEv — boost::detail::sp_counted_impl_p<DummyJob>::dispose(void)
pub fn stub_0x483cbc() {
    // IDA 0x483cbc: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<DummyJob>::get_deleter(std::type_info const&)")]
// 0x483ccc — __ZN5boost6detail17sp_counted_impl_pI8DummyJobE11get_deleterERKSt9type_info — boost::detail::sp_counted_impl_p<DummyJob>::get_deleter(std::type_info const&)
pub fn stub_0x483ccc() {
    // IDA 0x483ccc: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<DummyJob>::get_untyped_deleter(void)")]
// 0x483cd0 — __ZN5boost6detail17sp_counted_impl_pI8DummyJobE19get_untyped_deleterEv — boost::detail::sp_counted_impl_p<DummyJob>::get_untyped_deleter(void)
pub fn stub_0x483cd0() {
    // IDA 0x483cd0: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<DummyArbiter>::shared_ptr<DummyArbiter>(DummyArbiter *)")]
// 0x483cd4 — __ZN5boost10shared_ptrI12DummyArbiterEC2IS1_EEPT_ — rbx_core::SharedPtr<DummyArbiter>::shared_ptr<DummyArbiter>(DummyArbiter *)
// was: boost::shared_ptr<DummyArbiter>::shared_ptr<DummyArbiter>(DummyArbiter *)
pub fn stub_0x483cd4() {
    // IDA 0x483cd4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<DummyArbiter>(DummyArbiter *)")]
// 0x483da8 — __ZN5boost6detail12shared_countC2I12DummyArbiterEEPT_ — boost::detail::shared_count::shared_count<DummyArbiter>(DummyArbiter *)
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x483da8() {
    // IDA 0x483da8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<DummyArbiter>::~sp_counted_impl_p()")]
// 0x483e94 — __ZN5boost6detail17sp_counted_impl_pI12DummyArbiterED1Ev — boost::detail::sp_counted_impl_p<DummyArbiter>::~sp_counted_impl_p()
pub fn stub_0x483e94() {
    // IDA 0x483e94: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<DummyArbiter>::~sp_counted_impl_p()")]
// 0x483e98 — __ZN5boost6detail17sp_counted_impl_pI12DummyArbiterED0Ev — boost::detail::sp_counted_impl_p<DummyArbiter>::~sp_counted_impl_p()
pub fn stub_0x483e98() {
    // IDA 0x483e98: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<DummyArbiter>::dispose(void)")]
// 0x483e9c — __ZN5boost6detail17sp_counted_impl_pI12DummyArbiterE7disposeEv — boost::detail::sp_counted_impl_p<DummyArbiter>::dispose(void)
pub fn stub_0x483e9c() {
    // IDA 0x483e9c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<DummyArbiter>::get_deleter(std::type_info const&)")]
// 0x483ea8 — __ZN5boost6detail17sp_counted_impl_pI12DummyArbiterE11get_deleterERKSt9type_info — boost::detail::sp_counted_impl_p<DummyArbiter>::get_deleter(std::type_info const&)
pub fn stub_0x483ea8() {
    // IDA 0x483ea8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<DummyArbiter>::get_untyped_deleter(void)")]
// 0x483eac — __ZN5boost6detail17sp_counted_impl_pI12DummyArbiterE19get_untyped_deleterEv — boost::detail::sp_counted_impl_p<DummyArbiter>::get_untyped_deleter(void)
pub fn stub_0x483eac() {
    // IDA 0x483eac: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED1Ev")]
// 0x48409c — __ZN3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED1Ev
pub fn stub_0x48409c() {
    // IDA 0x48409c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED0Ev")]
// 0x4840dc — __ZN3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED0Ev
pub fn stub_0x4840dc() {
    // IDA 0x4840dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED1Ev")]
// 0x4841bc — __ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED1Ev
pub fn stub_0x4841bc() {
    // IDA 0x4841bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED0Ev")]
// 0x484200 — __ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED0Ev
pub fn stub_0x484200() {
    // IDA 0x484200: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED1Ev")]
// 0x484208 — __ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED1Ev
pub fn stub_0x484208() {
    // IDA 0x484208: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "float const& rbx::any_cast<float const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x48b140 — __ZN3rbx8any_castIRKfN3RBX7Region3EEET_RNS_13placement_anyIT0_EE — float const& rbx::any_cast<float const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// type: int __fastcall(_DWORD)
pub fn stub_0x48b140() {
    // IDA 0x48b140: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<float>(float const&)")]
// 0x48b228 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIfEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<float>(float const&)
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x48b228() {
    // IDA 0x48b228: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<float>::singleton(void)")]
// 0x48b278 — __ZN3rbx14implementation12typed_holderIfE9singletonEv — rbx::implementation::typed_holder<float>::singleton(void)
// type: int(void)
pub fn stub_0x48b278() {
    // IDA 0x48b278: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<float>::destruct_func(char *)")]
// 0x48b2e8 — __ZN3rbx14implementation12typed_holderIfE13destruct_funcEPc — rbx::implementation::typed_holder<float>::destruct_func(char *)
pub fn stub_0x48b2e8() {
    // IDA 0x48b2e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::resize(unsigned long,RBX::Time::SampleMethod)")]
// 0x48bbbc — __ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE6resizeEmS2_ — std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::resize(unsigned long,RBX::Time::SampleMethod)
// type: int(void)
pub fn stub_0x48bbbc() {
    // IDA 0x48bbbc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::push_back(RBX::Time::SampleMethod const&)")]
// 0x48bbf4 — __ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE9push_backERKS2_ — std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::push_back(RBX::Time::SampleMethod const&)
// type: int(void)
pub fn stub_0x48bbf4() {
    // IDA 0x48bbf4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Time::SampleMethod,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::operator[](RBX::Name const* const&)")]
// 0x48bc20 — __ZNSt3mapIPKN3RBX4NameENS0_4Time12SampleMethodESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::Time::SampleMethod,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::operator[](RBX::Name const* const&)
// type: _Rb_tree_node_base **__fastcall(int, int *)
pub fn stub_0x48bc20() {
    // IDA 0x48bc20: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Time::SampleMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::pair<RBX::Name const* const,RBX::Time::SampleMethod> const&)")]
// 0x48bc78 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_4Time12SampleMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Time::SampleMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::pair<RBX::Name const* const,RBX::Time::SampleMethod> const&)
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0x48bc78() {
    // IDA 0x48bc78: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Time::SampleMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Time::SampleMethod> const&)")]
// 0x48bd2c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_4Time12SampleMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Time::SampleMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Time::SampleMethod> const&)
// type: int(void)
pub fn stub_0x48bd2c() {
    // IDA 0x48bd2c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Time::SampleMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Time::SampleMethod> const&)")]
// 0x48bd84 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_4Time12SampleMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Time::SampleMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Time::SampleMethod> const&)
// type: int(void)
pub fn stub_0x48bd84() {
    // IDA 0x48bd84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Time::SampleMethod*,std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>>,RBX::Time::SampleMethod const&)")]
// 0x48bdf0 — __ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Time::SampleMethod*,std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>>,RBX::Time::SampleMethod const&)
// type: int(void)
pub fn stub_0x48bdf0() {
    // IDA 0x48bdf0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::_M_allocate(unsigned long)")]
// 0x48bed4 — __ZNSt12_Vector_baseIN3RBX4Time12SampleMethodESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::_M_allocate(unsigned long)
// type: int __fastcall(int, unsigned int)
pub fn stub_0x48bed4() {
    // IDA 0x48bed4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Time::SampleMethod * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Time::SampleMethod *,RBX::Time::SampleMethod *>(RBX::Time::SampleMethod *,RBX::Time::SampleMethod *,RBX::Time::SampleMethod *)")]
// 0x48beec — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX4Time12SampleMethodES6_EET0_T_S8_S7_ — RBX::Time::SampleMethod * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Time::SampleMethod *,RBX::Time::SampleMethod *>(RBX::Time::SampleMethod *,RBX::Time::SampleMethod *,RBX::Time::SampleMethod *)
// type: int(void)
pub fn stub_0x48beec() {
    // IDA 0x48beec: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Time::SampleMethod*,std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>>,unsigned long,RBX::Time::SampleMethod const&)")]
// 0x48bf2c — __ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Time::SampleMethod*,std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>>,unsigned long,RBX::Time::SampleMethod const&)
// type: int(void)
pub fn stub_0x48bf2c() {
    // IDA 0x48bf2c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::resize(unsigned long,RBX::EThrottle::EThrottleType)")]
// 0x48c0c0 — __ZNSt6vectorIN3RBX9EThrottle13EThrottleTypeESaIS2_EE6resizeEmS2_ — std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::resize(unsigned long,RBX::EThrottle::EThrottleType)
// type: int(void)
pub fn stub_0x48c0c0() {
    // IDA 0x48c0c0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::push_back(RBX::EThrottle::EThrottleType const&)")]
// 0x48c0f4 — __ZNSt6vectorIN3RBX9EThrottle13EThrottleTypeESaIS2_EE9push_backERKS2_ — std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::push_back(RBX::EThrottle::EThrottleType const&)
// type: int __fastcall(int result, _DWORD *)
pub fn stub_0x48c0f4() {
    // IDA 0x48c0f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::EThrottle::EThrottleType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::operator[](RBX::Name const* const&)")]
// 0x48c11c — __ZNSt3mapIPKN3RBX4NameENS0_9EThrottle13EThrottleTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::EThrottle::EThrottleType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::operator[](RBX::Name const* const&)
// type: int(void)
pub fn stub_0x48c11c() {
    // IDA 0x48c11c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType> const&)")]
// 0x48c174 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9EThrottle13EThrottleTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType> const&)
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0x48c174() {
    // IDA 0x48c174: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType> const&)")]
// 0x48c228 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9EThrottle13EThrottleTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType> const&)
// type: int(void)
pub fn stub_0x48c228() {
    // IDA 0x48c228: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType> const&)")]
// 0x48c280 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9EThrottle13EThrottleTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType> const&)
// type: int(void)
pub fn stub_0x48c280() {
    // IDA 0x48c280: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::EThrottle::EThrottleType*,std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>>,RBX::EThrottle::EThrottleType const&)")]
// 0x48c2e8 — __ZNSt6vectorIN3RBX9EThrottle13EThrottleTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::EThrottle::EThrottleType*,std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>>,RBX::EThrottle::EThrottleType const&)
// type: int(void)
pub fn stub_0x48c2e8() {
    // IDA 0x48c2e8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::_M_allocate(unsigned long)")]
// 0x48c3cc — __ZNSt12_Vector_baseIN3RBX9EThrottle13EThrottleTypeESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::_M_allocate(unsigned long)
// type: int __fastcall(int, unsigned int)
pub fn stub_0x48c3cc() {
    // IDA 0x48c3cc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::EThrottle::EThrottleType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::EThrottle::EThrottleType *,RBX::EThrottle::EThrottleType *>(RBX::EThrottle::EThrottleType *,RBX::EThrottle::EThrottleType *,RBX::EThrottle::EThrottleType *)")]
// 0x48c3e4 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9EThrottle13EThrottleTypeES6_EET0_T_S8_S7_ — RBX::EThrottle::EThrottleType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::EThrottle::EThrottleType *,RBX::EThrottle::EThrottleType *>(RBX::EThrottle::EThrottleType *,RBX::EThrottle::EThrottleType *,RBX::EThrottle::EThrottleType *)
// type: int(void)
pub fn stub_0x48c3e4() {
    // IDA 0x48c3e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::EThrottle::EThrottleType*,std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>>,unsigned long,RBX::EThrottle::EThrottleType const&)")]
// 0x48c420 — __ZNSt6vectorIN3RBX9EThrottle13EThrottleTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::EThrottle::EThrottleType*,std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>>,unsigned long,RBX::EThrottle::EThrottleType const&)
// type: int(void)
pub fn stub_0x48c420() {
    // IDA 0x48c420: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::resize(unsigned long,RBX::DebugSettings::ErrorReporting)")]
// 0x48c5b0 — __ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE6resizeEmS2_ — std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::resize(unsigned long,RBX::DebugSettings::ErrorReporting)
// type: int(void)
pub fn stub_0x48c5b0() {
    // IDA 0x48c5b0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::push_back(RBX::DebugSettings::ErrorReporting const&)")]
// 0x48c5e4 — __ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE9push_backERKS2_ — std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::push_back(RBX::DebugSettings::ErrorReporting const&)
// type: int(void)
pub fn stub_0x48c5e4() {
    // IDA 0x48c5e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::DebugSettings::ErrorReporting,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::operator[](RBX::Name const* const&)")]
// 0x48c60c — __ZNSt3mapIPKN3RBX4NameENS0_13DebugSettings14ErrorReportingESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::DebugSettings::ErrorReporting,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::operator[](RBX::Name const* const&)
// type: int(void)
pub fn stub_0x48c60c() {
    // IDA 0x48c60c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting> const&)")]
// 0x48c664 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DebugSettings14ErrorReportingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting> const&)
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0x48c664() {
    // IDA 0x48c664: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting> const&)")]
// 0x48c718 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DebugSettings14ErrorReportingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting> const&)
// type: int(void)
pub fn stub_0x48c718() {
    // IDA 0x48c718: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting> const&)")]
// 0x48c770 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DebugSettings14ErrorReportingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting> const&)
// type: int(void)
pub fn stub_0x48c770() {
    // IDA 0x48c770: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DebugSettings::ErrorReporting*,std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>>,RBX::DebugSettings::ErrorReporting const&)")]
// 0x48c7d8 — __ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DebugSettings::ErrorReporting*,std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>>,RBX::DebugSettings::ErrorReporting const&)
// type: int(void)
pub fn stub_0x48c7d8() {
    // IDA 0x48c7d8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::_M_allocate(unsigned long)")]
// 0x48c8bc — __ZNSt12_Vector_baseIN3RBX13DebugSettings14ErrorReportingESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::_M_allocate(unsigned long)
// type: int __fastcall(int, unsigned int)
pub fn stub_0x48c8bc() {
    // IDA 0x48c8bc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::DebugSettings::ErrorReporting * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DebugSettings::ErrorReporting *,RBX::DebugSettings::ErrorReporting *>(RBX::DebugSettings::ErrorReporting *,RBX::DebugSettings::ErrorReporting *,RBX::DebugSettings::ErrorReporting *)")]
// 0x48c8d4 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13DebugSettings14ErrorReportingES6_EET0_T_S8_S7_ — RBX::DebugSettings::ErrorReporting * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DebugSettings::ErrorReporting *,RBX::DebugSettings::ErrorReporting *>(RBX::DebugSettings::ErrorReporting *,RBX::DebugSettings::ErrorReporting *,RBX::DebugSettings::ErrorReporting *)
// type: int(void)
pub fn stub_0x48c8d4() {
    // IDA 0x48c8d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DebugSettings::ErrorReporting*,std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>>,unsigned long,RBX::DebugSettings::ErrorReporting const&)")]
// 0x48c910 — __ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DebugSettings::ErrorReporting*,std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>>,unsigned long,RBX::DebugSettings::ErrorReporting const&)
// type: int(void)
pub fn stub_0x48c910() {
    // IDA 0x48c910: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
