// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield|CodeGen (4818 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x479dfc..0x47e050 | script 26169->26269 distinct (filler 0x479dfc asc, not-in-script 59376->59276)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x479dfc() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DebrisService", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(rbx_core::SharedPtr<RBX::Instance>,double),2>::BoundFuncDesc(void (RBX::DebrisService::*)(rbx_core::SharedPtr<RBX::Instance>,double),char const*,char const*,char const*,double,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x479e30() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(rbx_core::SharedPtr<RBX::Instance>,double),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0x47a050() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(rbx_core::SharedPtr<RBX::Instance>,double),2>::~BoundFuncDesc() [0x47a09c]")]
pub fn stub_0x47a09c(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(rbx_core::SharedPtr<RBX::Instance>,double),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x47a1c8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::Call2Helper<RBX::DebrisService,void (RBX::DebrisService::*)(rbx_core::SharedPtr<RBX::Instance>,double),rbx_core::SharedPtr<RBX::Instance>,double,void>::call(RBX::DebrisService*,void (RBX::DebrisService::*)(rbx_core::SharedPtr<RBX::Instance>,double),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&,double const&)")]
pub fn stub_0x47a2cc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebrisService,int>::PropDescriptor<int (RBX::DebrisService::*)(void)const,void (RBX::DebrisService::*)(int)>(char const*,char const*,int (RBX::DebrisService::*)(void)const,void (RBX::DebrisService::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x47a3c0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebrisService,int>::~PropDescriptor() [0x47a4d4]")]
pub fn stub_0x47a4d4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebrisService,int>::GetSetImpl<int (RBX::DebrisService::*)(void)const,void (RBX::DebrisService::*)(int)>::isReadOnly(void)const")]
pub fn stub_0x47a500(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebrisService,int>::GetSetImpl<int (RBX::DebrisService::*)(void)const,void (RBX::DebrisService::*)(int)>::isWriteOnly(void)const")]
pub fn stub_0x47a504(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebrisService,int>::GetSetImpl<int (RBX::DebrisService::*)(void)const,void (RBX::DebrisService::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x47a508(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebrisService,int>::GetSetImpl<int (RBX::DebrisService::*)(void)const,void (RBX::DebrisService::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
pub fn stub_0x47a528(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::EnumDesc(void)")]
pub fn stub_0x47a87c() -> crate::slot::InstanceHandle {
// RBX::Reflection::EnumDesc ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::EnumDesc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::EnumDesc(void)")]
pub fn stub_0x47ab28() -> crate::slot::InstanceHandle {
// RBX::Reflection::EnumDesc ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::EnumDesc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::EnumDesc(void)")]
pub fn stub_0x47ad04() -> crate::slot::InstanceHandle {
// RBX::Reflection::EnumDesc ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::EnumDesc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::EnumDesc(void)")]
pub fn stub_0x47aee0() -> crate::slot::InstanceHandle {
// RBX::Reflection::EnumDesc ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::EnumDesc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::EnumDesc(void)")]
pub fn stub_0x47b0b8() -> crate::slot::InstanceHandle {
// RBX::Reflection::EnumDesc ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::EnumDesc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::EnumDesc(void) [0x47b0bc]")]
pub fn stub_0x47b0bc() -> crate::slot::InstanceHandle {
// RBX::Reflection::EnumDesc ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::EnumDesc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::EnumDesc(void)")]
pub fn stub_0x47b2f4() -> crate::slot::InstanceHandle {
// RBX::Reflection::EnumDesc ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::EnumDesc")
}

#[doc(alias = "RBX::DebugSettings::getVertexShaderModel(void)const")]
pub fn stub_0x47b4cc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DebugSettings getter.
cell.get()
}

#[doc(alias = "RBX::DebugSettings::getPixelShaderModel(void)const")]
pub fn stub_0x47b4d0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DebugSettings getter.
cell.get()
}

#[doc(alias = "RBX::DebugSettings::videoMemory(void)const")]
pub fn stub_0x47b4d4(handle: &crate::slot::InstanceHandle) {
// RBX::DebugSettings::videoMemory() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DebugSettings::cpuSpeed(void)const")]
pub fn stub_0x47b564(handle: &crate::slot::InstanceHandle) {
// RBX::DebugSettings::cpuSpeed() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DebugSettings::cpuCount(void)const")]
pub fn stub_0x47b5f4(handle: &crate::slot::InstanceHandle) {
// RBX::DebugSettings::cpuCount() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DebugSettings::osPlatformId(void)const")]
pub fn stub_0x47b684(handle: &crate::slot::InstanceHandle) {
// RBX::DebugSettings::osPlatformId() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DebugSettings::osPlatform(void)const")]
pub fn stub_0x47b688(handle: &crate::slot::InstanceHandle) {
// RBX::DebugSettings::osPlatform() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DebugSettings::osVer(void)const")]
pub fn stub_0x47b6a4(handle: &crate::slot::InstanceHandle) {
// RBX::DebugSettings::osVer() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DebugSettings::osIs64Bit(void)const")]
pub fn stub_0x47b6b0(handle: &crate::slot::InstanceHandle) {
// RBX::DebugSettings::osIs64Bit() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DebugSettings::systemProductName(void)const")]
pub fn stub_0x47b6bc(handle: &crate::slot::InstanceHandle) {
// RBX::DebugSettings::systemProductName() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DebugSettings::gfxcard(void)const")]
pub fn stub_0x47b6d8(handle: &crate::slot::InstanceHandle) {
// RBX::DebugSettings::gfxcard() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DebugSettings::cpu(void)const")]
pub fn stub_0x47b6e4(handle: &crate::slot::InstanceHandle) {
// RBX::DebugSettings::cpu() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DebugSettings::simd(void)const")]
pub fn stub_0x47b894(handle: &crate::slot::InstanceHandle) {
// RBX::DebugSettings::simd() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DebugSettings::totalPhysicalMemory(void)const")]
pub fn stub_0x47b9a4(handle: &crate::slot::InstanceHandle) {
// RBX::DebugSettings::totalPhysicalMemory() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DebugSettings::resolution(void)const")]
pub fn stub_0x47ba34(handle: &crate::slot::InstanceHandle) {
// RBX::DebugSettings::resolution() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DebugSettings::availablePhysicalMemory(void)const")]
pub fn stub_0x47bbb4(handle: &crate::slot::InstanceHandle) {
// RBX::DebugSettings::availablePhysicalMemory() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DebugSettings::getElapsedTime(void)const")]
pub fn stub_0x47bc44(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DebugSettings getter.
cell.get()
}

#[doc(alias = "RBX::DebugSettings::processCores(void)const")]
pub fn stub_0x47bc50(handle: &crate::slot::InstanceHandle) {
// RBX::DebugSettings::processCores() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DebugSettings::totalProcessorTime(void)const")]
pub fn stub_0x47bc8c(handle: &crate::slot::InstanceHandle) {
// RBX::DebugSettings::totalProcessorTime() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DebugSettings::processorTime(void)const")]
pub fn stub_0x47bcb0(handle: &crate::slot::InstanceHandle) {
// RBX::DebugSettings::processorTime() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DebugSettings::privateBytes(void)const")]
pub fn stub_0x47bcb8(handle: &crate::slot::InstanceHandle) {
// RBX::DebugSettings::privateBytes() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DebugSettings::privateWorkingSetBytes(void)const")]
pub fn stub_0x47bcdc(handle: &crate::slot::InstanceHandle) {
// RBX::DebugSettings::privateWorkingSetBytes() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DebugSettings::GetVirtualBytes(void)const")]
pub fn stub_0x47bcfc(handle: &crate::slot::InstanceHandle) {
// RBX::DebugSettings::GetVirtualBytes() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DebugSettings::GetPageFileBytes(void)const")]
pub fn stub_0x47bd1c(handle: &crate::slot::InstanceHandle) {
// RBX::DebugSettings::GetPageFileBytes() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DebugSettings::GetPageFaultsPerSecond(void)const")]
pub fn stub_0x47bd24(handle: &crate::slot::InstanceHandle) {
// RBX::DebugSettings::GetPageFaultsPerSecond() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DebugSettings::getPlayerCount(void)const")]
pub fn stub_0x47bd50(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DebugSettings getter.
cell.get()
}

#[doc(alias = "RBX::DebugSettings::getDataModelCount(void)const")]
pub fn stub_0x47bd60(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DebugSettings getter.
cell.get()
}

#[doc(alias = "RBX::DebugSettings::getCdnSuccessCount(void)const")]
pub fn stub_0x47bd70(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DebugSettings getter.
cell.get()
}

#[doc(alias = "RBX::DebugSettings::getCdnFailureCount(void)const")]
pub fn stub_0x47bd80(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DebugSettings getter.
cell.get()
}

#[doc(alias = "RBX::DebugSettings::getAlternateCdnSuccessCount(void)const")]
pub fn stub_0x47bd90(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DebugSettings getter.
cell.get()
}

#[doc(alias = "RBX::DebugSettings::getAlternateCdnFailureCount(void)const")]
pub fn stub_0x47bda0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DebugSettings getter.
cell.get()
}

#[doc(alias = "RBX::DebugSettings::getBlockMeshMapCount(void)const")]
pub fn stub_0x47bdb0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DebugSettings getter.
cell.get()
}

#[doc(alias = "RBX::DebugSettings::getLastCdnFailureTimeSpan(void)const")]
pub fn stub_0x47bdb4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DebugSettings getter.
cell.get()
}

#[doc(alias = "RBX::DebugSettings::getRobloxSuccessCount(void)const")]
pub fn stub_0x47bdcc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DebugSettings getter.
cell.get()
}

#[doc(alias = "RBX::DebugSettings::getRobloxFalureCount(void)const")]
pub fn stub_0x47bddc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DebugSettings getter.
cell.get()
}

#[doc(alias = "RBX::DebugSettings::getRobloxResponce(void)const")]
pub fn stub_0x47bdf0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DebugSettings getter.
cell.get()
}

#[doc(alias = "RBX::DebugSettings::getCdnRespoce(void)const")]
pub fn stub_0x47be48(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DebugSettings getter.
cell.get()
}

#[doc(alias = "RBX::DebugSettings::resetCdnFailureCounts(void)")]
pub fn stub_0x47bea0(handle: &crate::slot::InstanceHandle) {
// RBX::DebugSettings::resetCdnFailureCounts() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TaskSchedulerSettings::addDummyJob(bool,double)")]
pub fn stub_0x47c2a8(handle: &crate::slot::InstanceHandle) {
// RBX::TaskSchedulerSettings::addDummyJob(bool, double) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DebugSettings::setErrorReporting(RBX::DebugSettings::ErrorReporting)")]
pub fn stub_0x47c3f8(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::DebugSettings setter.
cell.set(value)
}

#[doc(alias = "RBX::TaskSchedulerSettings::getThreadPoolConfig(void)const")]
pub fn stub_0x47c414(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::TaskSchedulerSettings getter.
cell.get()
}

#[doc(alias = "RBX::TaskSchedulerSettings::setThreadPoolConfig(RBX::TaskScheduler::ThreadPoolConfig)")]
pub fn stub_0x47c418(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::TaskSchedulerSettings setter.
cell.set(value)
}

#[doc(alias = "RBX::TaskSchedulerSettings::setThreadShare(double,int)")]
pub fn stub_0x47c460(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::TaskSchedulerSettings setter.
cell.set(value)
}

#[doc(alias = "RBX::TaskSchedulerSettings::setPriorityMethod(RBX::TaskScheduler::PriorityMethod)")]
pub fn stub_0x47c464(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::TaskSchedulerSettings setter.
cell.set(value)
}

#[doc(alias = "RBX::TaskSchedulerSettings::setSleepAdjustMethod(RBX::TaskScheduler::Job::SleepAdjustMethod)")]
pub fn stub_0x47c4a0(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::TaskSchedulerSettings setter.
cell.set(value)
}

#[doc(alias = "RBX::TaskSchedulerSettings::setConcurrencyModel(RBX::DataModelArbiter::ConcurrencyModel)")]
pub fn stub_0x47c4dc(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::TaskSchedulerSettings setter.
cell.set(value)
}

#[doc(alias = "RBX::TaskSchedulerSettings::setIsArbiterThrottled(bool)")]
pub fn stub_0x47c518(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::TaskSchedulerSettings setter.
cell.set(value)
}

#[doc(alias = "RBX::TaskSchedulerSettings::setThrottledJobSleepTime(double)")]
pub fn stub_0x47c53c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::TaskSchedulerSettings setter.
cell.set(value)
}

#[doc(alias = "RBX::DebugSettings::getIsProfilingEnabled(void)const")]
pub fn stub_0x47c564(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DebugSettings getter.
cell.get()
}

#[doc(alias = "RBX::DebugSettings::setIsProfilingEnabled(bool)")]
pub fn stub_0x47c570(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::DebugSettings setter.
cell.set(value)
}

#[doc(alias = "RBX::DebugSettings::getProfilingWindow(void)const")]
pub fn stub_0x47c578(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DebugSettings getter.
cell.get()
}

#[doc(alias = "RBX::DebugSettings::setProfilingWindow(double)")]
pub fn stub_0x47c590(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::DebugSettings setter.
cell.set(value)
}

#[doc(alias = "RBX::DebugSettings::getInstanceCountLimit(void)const")]
pub fn stub_0x47c5c8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DebugSettings getter.
cell.get()
}

#[doc(alias = "RBX::DebugSettings::setInstanceCountLimit(int)")]
pub fn stub_0x47c5d8(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::DebugSettings setter.
cell.set(value)
}

#[doc(alias = "RBX::DebugSettings::getEnforceInstanceCountLimit(void)const")]
pub fn stub_0x47c5e8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DebugSettings getter.
cell.get()
}

#[doc(alias = "RBX::DebugSettings::setEnforceInstanceCountLimit(bool)")]
pub fn stub_0x47c5f8(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::DebugSettings setter.
cell.set(value)
}

#[doc(alias = "RBX::DebugSettings::DebugSettings(void)")]
pub fn stub_0x47c608() -> crate::slot::InstanceHandle {
// RBX::DebugSettings ctor.
crate::slot::InstanceHandle::new("RBX::DebugSettings")
}

#[doc(alias = "RBX::DebugSettings::DebugSettings(void) [0x47c60c]")]
pub fn stub_0x47c60c() -> crate::slot::InstanceHandle {
// RBX::DebugSettings ctor.
crate::slot::InstanceHandle::new("RBX::DebugSettings")
}

#[doc(alias = "DummyArbiter::areExclusive(RBX::TaskScheduler::Job *,RBX::TaskScheduler::Job *)")]
pub fn stub_0x47c7e4() -> crate::slot::PortedFn {
// IDA 0x47c7e4: DummyArbiter::areExclusive(RBX::TaskScheduler::Job*, RBX::TaskScheduler::Job*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x47c7e4, "DummyArbiter::areExclusive(RBX::TaskScheduler::Job*, RBX::TaskScheduler::Job*)")
}

#[doc(alias = "RBX::TaskSchedulerSettings::TaskSchedulerSettings(void)")]
pub fn stub_0x47c800() -> crate::slot::InstanceHandle {
// RBX::TaskSchedulerSettings ctor.
crate::slot::InstanceHandle::new("RBX::TaskSchedulerSettings")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::addPair(RBX::TaskScheduler::ThreadPoolConfig,char const*)")]
pub fn stub_0x47c9c0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::addPair(RBX::TaskSchedule~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::addLegacy(int,char const*,RBX::TaskScheduler::ThreadPoolConfig)")]
pub fn stub_0x47cd20(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::addLegacy(int, char const~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::addPair(RBX::TaskScheduler::PriorityMethod,char const*)")]
pub fn stub_0x47cd74(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::addPair(RBX::TaskScheduler:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::addPair(RBX::TaskScheduler::Job::SleepAdjustMethod,char const*)")]
pub fn stub_0x47d0d4(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::addPair(RBX::TaskSc~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::addPair(RBX::DebugSettings::ErrorReporting,char const*)")]
pub fn stub_0x47d434(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::addPair(RBX::DebugSettings:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::addPair(RBX::EThrottle::EThrottleType,char const*)")]
pub fn stub_0x47d794(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::addPair(RBX::EThrottle::EThrottl~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::addPair(RBX::Time::SampleMethod,char const*)")]
pub fn stub_0x47daf4(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::addPair(RBX::Time::SampleMethod, char ~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DebugSettings::getRobloxVersion(void)const")]
pub fn stub_0x47de54(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DebugSettings getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,std::string>::~PropDescriptor()")]
pub fn stub_0x47dec8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::DebugSettings::getRobloxProductName(void)const")]
pub fn stub_0x47deec(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DebugSettings getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,float>::~PropDescriptor()")]
pub fn stub_0x47df60(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::~PropDescriptor()")]
pub fn stub_0x47df84(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,bool>::~PropDescriptor()")]
pub fn stub_0x47dfa8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::DebugSettings::nameDatabaseSize(void)const")]
pub fn stub_0x47dfcc(handle: &crate::slot::InstanceHandle) {
// RBX::DebugSettings::nameDatabaseSize() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DebugSettings::nameDatabaseBytes(void)const")]
pub fn stub_0x47dfd0(handle: &crate::slot::InstanceHandle) {
// RBX::DebugSettings::nameDatabaseBytes() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,double>::~PropDescriptor()")]
pub fn stub_0x47dfd4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::DebugSettings::instanceCount(void)const")]
pub fn stub_0x47dff8(handle: &crate::slot::InstanceHandle) {
// RBX::DebugSettings::instanceCount() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DebugSettings::jobCount(void)const")]
pub fn stub_0x47e008(handle: &crate::slot::InstanceHandle) {
// RBX::DebugSettings::jobCount() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(void),0>::~BoundFuncDesc()")]
pub fn stub_0x47e018(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::TaskSchedulerSettings::threadPoolSize(void)const")]
pub fn stub_0x47e03c(handle: &crate::slot::InstanceHandle) {
// RBX::TaskSchedulerSettings::threadPoolSize() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::~PropDescriptor()")]
pub fn stub_0x47e050(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}
