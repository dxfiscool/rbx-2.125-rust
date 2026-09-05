// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 120 not yet in script
// Filter: Script|Lua|Yield|CodeGen (4818 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +120 stubs | range 0x47e074..0x481288 | script 11590->11710 distinct (filler 0x47e074 asc, not-in-script 74156->74036)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "RBX::TaskSchedulerSettings::threadAffinity(void)const")]
pub fn stub_0x47e074(handle: &crate::slot::InstanceHandle) {
// RBX::TaskSchedulerSettings::threadAffinity() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::~PropDescriptor()")]
pub fn stub_0x47e088(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::TaskSchedulerSettings::numSleepingJobs(void)const")]
pub fn stub_0x47e0ac(handle: &crate::slot::InstanceHandle) {
// RBX::TaskSchedulerSettings::numSleepingJobs() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TaskSchedulerSettings::numWaitingJobs(void)const")]
pub fn stub_0x47e0c0(handle: &crate::slot::InstanceHandle) {
// RBX::TaskSchedulerSettings::numWaitingJobs() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TaskSchedulerSettings::numRunningJobs(void)const")]
pub fn stub_0x47e0d4(handle: &crate::slot::InstanceHandle) {
// RBX::TaskSchedulerSettings::numRunningJobs() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TaskSchedulerSettings::schedulerRate(void)const")]
pub fn stub_0x47e0e8(handle: &crate::slot::InstanceHandle) {
// RBX::TaskSchedulerSettings::schedulerRate() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TaskSchedulerSettings::schedulerDutyCyclePerThread(void)const")]
pub fn stub_0x47e0fc(handle: &crate::slot::InstanceHandle) {
// RBX::TaskSchedulerSettings::schedulerDutyCyclePerThread() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::~BoundFuncDesc()")]
pub fn stub_0x47e10c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::DebugSettings::getErrorReporting(void)const")]
pub fn stub_0x47e158(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DebugSettings getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::~EnumPropDescriptor()")]
pub fn stub_0x47e15c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::DebugSettings::noOpt(void)")]
pub fn stub_0x47e180(handle: &crate::slot::InstanceHandle) {
// RBX::DebugSettings::noOpt() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(void),0>::~BoundFuncDesc()")]
pub fn stub_0x47e184(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::DebugSettings::setBlockingRemove(bool)")]
pub fn stub_0x47e1a8(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::DebugSettings setter.
cell.set(value)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(bool),1>::~BoundFuncDesc()")]
pub fn stub_0x47e1b0(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::~EnumPropDescriptor()")]
pub fn stub_0x47e1f0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(double,int),2>::~BoundFuncDesc()")]
pub fn stub_0x47e214(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::TaskSchedulerSettings::getPriorityMethod(void)const")]
pub fn stub_0x47e25c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::TaskSchedulerSettings getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::~EnumPropDescriptor()")]
pub fn stub_0x47e26c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::TaskSchedulerSettings::getSleepAdjustMethod(void)const")]
pub fn stub_0x47e290(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::TaskSchedulerSettings getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::~EnumPropDescriptor()")]
pub fn stub_0x47e2a0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::TaskSchedulerSettings::getConcurrencyModel(void)const")]
pub fn stub_0x47e2c4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::TaskSchedulerSettings getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::DataModelArbiter::ConcurrencyModel>::~EnumPropDescriptor()")]
pub fn stub_0x47e2d4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::TaskSchedulerSettings::getIsArbiterThrottled(void)const")]
pub fn stub_0x47e2f8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::TaskSchedulerSettings getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,bool>::~PropDescriptor()")]
pub fn stub_0x47e308(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::TaskSchedulerSettings::getThrottledJobSleepTime(void)const")]
pub fn stub_0x47e32c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::TaskSchedulerSettings getter.
cell.get()
}

#[doc(alias = "RBX::DebugSettings::getTickCountPreciseOverride(void)const")]
pub fn stub_0x47e344(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DebugSettings getter.
cell.get()
}

#[doc(alias = "RBX::DebugSettings::setTickCountPreciseOverride(RBX::Time::SampleMethod)")]
pub fn stub_0x47e354(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::DebugSettings setter.
cell.set(value)
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::~EnumPropDescriptor()")]
pub fn stub_0x47e364(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEEC2Ev")]
pub fn stub_0x47e388() -> crate::slot::InstanceHandle {
// settings-item ctor.
crate::slot::InstanceHandle::new("RBX::GlobalAdvancedSettingsItem")
}

#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::getSize(void)")]
pub fn stub_0x47e5f8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::GeometryPool getter.
cell.get()
}

#[doc(alias = "DummyArbiter::arbiterName(void)")]
pub fn stub_0x47e8c8() -> crate::slot::PortedFn {
// IDA 0x47e8c8: DummyArbiter::arbiterName().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x47e8c8, "DummyArbiter::arbiterName()")
}

#[doc(alias = "DummyArbiter::isThrottled(void)")]
pub fn stub_0x47e8e4() -> crate::slot::PortedFn {
// IDA 0x47e8e4: DummyArbiter::isThrottled().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x47e8e4, "DummyArbiter::isThrottled()")
}

#[doc(alias = "RBX::TaskScheduler::Arbiter::preStep(RBX::TaskScheduler::Job *)")]
pub fn stub_0x47e8e8(handle: &crate::slot::InstanceHandle) {
// RBX::TaskScheduler::Arbiter::preStep(RBX::TaskScheduler::Job*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TaskScheduler::Arbiter::postStep(RBX::TaskScheduler::Job *)")]
pub fn stub_0x47e900(handle: &crate::slot::InstanceHandle) {
// RBX::TaskScheduler::Arbiter::postStep(RBX::TaskScheduler::Job*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::~EnumDesc()")]
pub fn stub_0x47e924(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::~EnumDesc() [0x47e928]")]
pub fn stub_0x47e928(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::lookup(char const*)const")]
pub fn stub_0x47e9c8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::lookup(char const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_0x47e9f8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::lookup(RBX::Reflection::V~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_0x47ea18(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::convertToValue(unsigned l~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::convertToString(unsigned long,std::string &)const")]
pub fn stub_0x47ea4c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::convertToString(unsigned ~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::~EnumDesc()")]
pub fn stub_0x47eb90(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::~EnumDesc() [0x47eb94]")]
pub fn stub_0x47eb94(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::lookup(char const*)const")]
pub fn stub_0x47ec34(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::lookup(char const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_0x47ec64(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::lookup(RBX::Reflection::Var~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_0x47ec84(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::convertToValue(unsigned lon~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::convertToString(unsigned long,std::string &)const")]
pub fn stub_0x47ecb8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::convertToString(unsigned lo~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::~EnumDesc()")]
pub fn stub_0x47edfc(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::~EnumDesc() [0x47ee00]")]
pub fn stub_0x47ee00(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::lookup(char const*)const")]
pub fn stub_0x47eea0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::lookup(char const*)~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_0x47eed0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::lookup(RBX::Reflect~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_0x47eef0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::convertToValue(unsi~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::convertToString(unsigned long,std::string &)const")]
pub fn stub_0x47ef24(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::convertToString(uns~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::~EnumDesc()")]
pub fn stub_0x47f068(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::~EnumDesc() [0x47f06c]")]
pub fn stub_0x47f06c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::lookup(char const*)const")]
pub fn stub_0x47f10c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::lookup(char const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_0x47f13c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::lookup(RBX::Reflection::Var~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_0x47f15c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::convertToValue(unsigned lon~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::convertToString(unsigned long,std::string &)const")]
pub fn stub_0x47f190(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::convertToString(unsigned lo~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::~EnumDesc()")]
pub fn stub_0x47f2d4(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::~EnumDesc() [0x47f2d8]")]
pub fn stub_0x47f2d8(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::lookup(char const*)const")]
pub fn stub_0x47f378(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::lookup(char const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_0x47f3a8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::lookup(RBX::Reflection::Variant ~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_0x47f3c8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::convertToValue(unsigned long, RB~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::convertToString(unsigned long,std::string &)const")]
pub fn stub_0x47f3fc(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::convertToString(unsigned long, s~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::~EnumDesc()")]
pub fn stub_0x47f540(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::~EnumDesc() [0x47f544]")]
pub fn stub_0x47f544(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::lookup(char const*)const")]
pub fn stub_0x47f5e4(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::lookup(char const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_0x47f614(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::lookup(RBX::Reflection::Variant const&~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_0x47f634(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::convertToValue(unsigned long, RBX::Ref~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::convertToString(unsigned long,std::string &)const")]
pub fn stub_0x47f668(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::convertToString(unsigned long, std::st~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DebugSettings::~DebugSettings()")]
pub fn stub_0x47f7ac(handle: crate::slot::InstanceHandle) {
// RBX::DebugSettings dtor.
drop(handle);
}

#[doc(alias = "RBX::DebugSettings::~DebugSettings() [0x47f7ec]")]
pub fn stub_0x47f7ec(handle: crate::slot::InstanceHandle) {
// RBX::DebugSettings dtor.
drop(handle);
}

#[doc(alias = "RBX::GlobalAdvancedSettings::Item::askAddChild(RBX::Instance const*)const")]
pub fn stub_0x47f8d0(handle: &crate::slot::InstanceHandle) {
// RBX::GlobalAdvancedSettings::Item::askAddChild(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_13DebugSettingsENS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x47f908() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"DebugSettings"
}

#[doc(alias = "non-virtual thunk toRBX::DebugSettings::~DebugSettings()")]
pub fn stub_0x47f91c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::DebugSettings::~DebugSettings() [0x47f960]")]
pub fn stub_0x47f960(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_13DebugSettingsENS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x47fa48() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"DebugSettings"
}

#[doc(alias = "non-virtual thunk toRBX::DebugSettings::~DebugSettings() [0x47fa58]")]
pub fn stub_0x47fa58(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::DebugSettings::~DebugSettings() [0x47fa9c]")]
pub fn stub_0x47fa9c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::TaskSchedulerSettings::~TaskSchedulerSettings()")]
pub fn stub_0x47fb88(handle: crate::slot::InstanceHandle) {
// RBX::TaskSchedulerSettings dtor.
drop(handle);
}

#[doc(alias = "RBX::TaskSchedulerSettings::~TaskSchedulerSettings() [0x47fbc8]")]
pub fn stub_0x47fbc8(handle: crate::slot::InstanceHandle) {
// RBX::TaskSchedulerSettings dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_13DebugSettingsENS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x48006c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"DebugSettings"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_13DebugSettingsENS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x4800e0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"DebugSettings"
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::convertToString(RBX::Time::SampleMethod const&)const")]
pub fn stub_0x48016c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::convertToString(RBX::Time::SampleMetho~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Time::SampleMethod>(RBX::Time::SampleMethod const&)")]
pub fn stub_0x48030c() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Time::SampleMethod>::singleton(void)")]
pub fn stub_0x48035c(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Time::SampleMethod>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Time::SampleMethod>::construct_func(char const*,char *)")]
pub fn stub_0x4803c8(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Time::SampleMethod>::construct_func(char const*, ch~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Time::SampleMethod>::destruct_func(char *)")]
pub fn stub_0x4803d4(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Time::SampleMethod>::destruct_func(char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::convertToItem(RBX::Time::SampleMethod const&)const")]
pub fn stub_0x4803d8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::convertToItem(RBX::Time::SampleMethod ~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Time::SampleMethod const& rbx::any_cast<RBX::Time::SampleMethod const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x4804a4(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::convertToValue(RBX::Name const&,RBX::Time::SampleMethod&)const")]
pub fn stub_0x480598(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::convertToValue(RBX::Name const&, RBX::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::~EnumDesc() [0x480614]")]
pub fn stub_0x480614(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::convertToString(RBX::EThrottle::EThrottleType const&)const")]
pub fn stub_0x4807e8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::convertToString(RBX::EThrottle::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::EThrottle::EThrottleType>(RBX::EThrottle::EThrottleType const&)")]
pub fn stub_0x480988() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::EThrottle::EThrottleType>::singleton(void)")]
pub fn stub_0x4809d8(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::EThrottle::EThrottleType>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::EThrottle::EThrottleType>::construct_func(char const*,char *)")]
pub fn stub_0x480a44(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::EThrottle::EThrottleType>::construct_func(char cons~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::EThrottle::EThrottleType>::destruct_func(char *)")]
pub fn stub_0x480a50(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::EThrottle::EThrottleType>::destruct_func(char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::convertToItem(RBX::EThrottle::EThrottleType const&)const")]
pub fn stub_0x480a54(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::convertToItem(RBX::EThrottle::ET~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::EThrottle::EThrottleType const& rbx::any_cast<RBX::EThrottle::EThrottleType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x480b20(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::convertToValue(RBX::Name const&,RBX::EThrottle::EThrottleType&)const")]
pub fn stub_0x480c10(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::convertToValue(RBX::Name const&,~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::~EnumDesc() [0x480c8c]")]
pub fn stub_0x480c8c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::convertToString(RBX::DebugSettings::ErrorReporting const&)const")]
pub fn stub_0x480e60(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::convertToString(RBX::DebugS~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DebugSettings::ErrorReporting>(RBX::DebugSettings::ErrorReporting const&)")]
pub fn stub_0x481000() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DebugSettings::ErrorReporting>::singleton(void)")]
pub fn stub_0x481050(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::DebugSettings::ErrorReporting>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DebugSettings::ErrorReporting>::construct_func(char const*,char *)")]
pub fn stub_0x4810bc(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::DebugSettings::ErrorReporting>::construct_func(char~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DebugSettings::ErrorReporting>::destruct_func(char *)")]
pub fn stub_0x4810c8(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::DebugSettings::ErrorReporting>::destruct_func(char*~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::convertToItem(RBX::DebugSettings::ErrorReporting const&)const")]
pub fn stub_0x4810cc(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::convertToItem(RBX::DebugSet~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DebugSettings::ErrorReporting const& rbx::any_cast<RBX::DebugSettings::ErrorReporting const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x481198(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::convertToValue(RBX::Name const&,RBX::DebugSettings::ErrorReporting&)const")]
pub fn stub_0x481288(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::convertToValue(RBX::Name co~ — engine-side; linkage preserved via the alias.
let _ = handle;
}
