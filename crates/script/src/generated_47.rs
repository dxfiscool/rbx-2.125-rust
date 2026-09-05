// Auto-generated skeletons for rbx-script — filler EA-sorted ascending earliest gap (next 100)
// Filter: Lua|Script|Yield|lua (5401 filtered, all already stubbed globally) — filler EA-sorted ascending
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x48867c..0x4d32a8 | existing ~9591 -> ~9691 total (union; filler 0x48867c ascending, global remaining 28558 -> 28458)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::GetSetImpl<RBX::TaskScheduler::ThreadPoolConfig (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::ThreadPoolConfig)>::setValue(RBX::Reflection::DescribedBase *,RBX::TaskScheduler::ThreadPoolConfig const&)const")]
pub fn stub_0x48867c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig> const>::initSingleton(void)")]
pub fn stub_0x4886a0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig> const>::doGetSingleton(void)")]
pub fn stub_0x4886a4(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::BoundFuncDesc(void (RBX::TaskSchedulerSettings::*)(bool,double),char const*,char const*,bool,char const*,double,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x489784() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::TaskSchedulerSettings", "void", 2)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0x4899b4() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::TaskSchedulerSettings", "void", 2)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::~BoundFuncDesc() [0x489a00]")]
pub fn stub_0x489a00(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x489ae0() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::TaskSchedulerSettings", "void", 2)
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::PropDescriptor<double (RBX::TaskSchedulerSettings::*)(void)const,int>(char const*,char const*,double (RBX::TaskSchedulerSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x489cd8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetImpl<double (RBX::TaskSchedulerSettings::*)(void)const>::isReadOnly(void)const")]
pub fn stub_0x489de4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetImpl<double (RBX::TaskSchedulerSettings::*)(void)const>::isWriteOnly(void)const")]
pub fn stub_0x489de8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetImpl<double (RBX::TaskSchedulerSettings::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x489dec(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::GetImpl<double (RBX::TaskSchedulerSettings::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,double const&)const")]
pub fn stub_0x489e0c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::PropDescriptor<unsigned int (RBX::TaskSchedulerSettings::*)(void)const,int>(char const*,char const*,unsigned int (RBX::TaskSchedulerSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x489f2c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::GetImpl<unsigned int (RBX::TaskSchedulerSettings::*)(void)const>::isReadOnly(void)const")]
pub fn stub_0x48a064(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::GetImpl<unsigned int (RBX::TaskSchedulerSettings::*)(void)const>::isWriteOnly(void)const")]
pub fn stub_0x48a068(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::GetImpl<unsigned int (RBX::TaskSchedulerSettings::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x48a06c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::GetImpl<unsigned int (RBX::TaskSchedulerSettings::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
pub fn stub_0x48a08c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<float>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<float>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x48af98() -> crate::slot::InstanceHandle {
// RBX::Reflection::TypedPropertyDescriptor ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::TypedPropertyDescriptor")
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<float>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x48b108(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::TypedPropertyDescriptor<float>::equalValues(RBX::Reflection::DescribedBas~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<std::string>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<std::string>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x48b570() -> crate::slot::InstanceHandle {
// RBX::Reflection::TypedPropertyDescriptor ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::TypedPropertyDescriptor")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9AnimationENS_8InstanceELZNS_10sAnimationEES2_E7CreatorC2Ev")]
pub fn stub_0x4ae4fc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Animation"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_8SparklesENS_8InstanceELZNS_9sSparklesEES2_E7CreatorD2Ev")]
pub fn stub_0x4aee64() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Sparkles"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_8SparklesENS_8InstanceELZNS_9sSparklesEES2_E7Creator12getClassNameEv")]
pub fn stub_0x4aef00() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Sparkles"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_8SparklesENS_8InstanceELZNS_9sSparklesEES2_E7Creator6createEv")]
pub fn stub_0x4aef6c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Sparkles"
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sSparklesEEEERKS0_v")]
pub fn stub_0x4af45c(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sSparkles>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sSparklesEEEEvv")]
pub fn stub_0x4af4a0() -> crate::slot::PortedFn {
// IDA 0x4af4a0: void RBX::Name::callDoDeclare<RBX::sSparkles>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4af4a0, "void RBX::Name::callDoDeclare<RBX::sSparkles>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sSparklesEEEERKS0_v")]
pub fn stub_0x4af4a4(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sSparkles>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_8SparklesENS_8InstanceELZNS_9sSparklesEES2_E7CreatorC2Ev")]
pub fn stub_0x4af588() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Sparkles"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x4af7b0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BasicPartInstance"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x4af84c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BasicPartInstance"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x4af8b8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BasicPartInstance"
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sBasicPartEEEERKS0_v")]
pub fn stub_0x4afdac(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sBasicPart>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sBasicPartEEEEvv")]
pub fn stub_0x4afdf0() -> crate::slot::PortedFn {
// IDA 0x4afdf0: void RBX::Name::callDoDeclare<RBX::sBasicPart>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4afdf0, "void RBX::Name::callDoDeclare<RBX::sBasicPart>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sBasicPartEEEERKS0_v")]
pub fn stub_0x4afdf4(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sBasicPart>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x4afed8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BasicPartInstance"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E7CreatorD2Ev")]
pub fn stub_0x4b0100() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ForceField"
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sForceFieldEEEERKS0_v")]
pub fn stub_0x4b0454(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sForceField>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_11CustomEventENS_8InstanceELZNS_12sCustomEventEES2_E7CreatorD2Ev")]
pub fn stub_0x4b0658() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"CustomEvent"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_11CustomEventENS_8InstanceELZNS_12sCustomEventEES2_E7Creator12getClassNameEv")]
pub fn stub_0x4b06f4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"CustomEvent"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_11CustomEventENS_8InstanceELZNS_12sCustomEventEES2_E7Creator6createEv")]
pub fn stub_0x4b0760() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"CustomEvent"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_11CustomEventENS_8InstanceELZNS_12sCustomEventEES2_E12getClassNameEv")]
pub fn stub_0x4b0e94() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"CustomEvent"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_11CustomEventENS_8InstanceELZNS_12sCustomEventEES2_E12getClassNameEv")]
pub fn stub_0x4b0f50() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"CustomEvent"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_11CustomEventENS_8InstanceELZNS_12sCustomEventEES2_E17static_getCreatorEv")]
pub fn stub_0x4b11d0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"CustomEvent"
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_12sCustomEventEEEERKS0_v")]
pub fn stub_0x4b1c80(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sCustomEvent>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sCustomEventEEEEvv")]
pub fn stub_0x4b1cc4() -> crate::slot::PortedFn {
// IDA 0x4b1cc4: void RBX::Name::callDoDeclare<RBX::sCustomEvent>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4b1cc4, "void RBX::Name::callDoDeclare<RBX::sCustomEvent>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sCustomEventEEEERKS0_v")]
pub fn stub_0x4b1cc8(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sCustomEvent>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_11CustomEventENS_8InstanceELZNS_12sCustomEventEES2_E7CreatorC2Ev")]
pub fn stub_0x4b1dac() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"CustomEvent"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_19CustomEventReceiverENS_8InstanceELZNS_20sCustomEventReceiverEES2_E7CreatorD2Ev")]
pub fn stub_0x4b1fd4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"CustomEventReceiver"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_19CustomEventReceiverENS_8InstanceELZNS_20sCustomEventReceiverEES2_E7Creator12getClassNameEv")]
pub fn stub_0x4b2070() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"CustomEventReceiver"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_19CustomEventReceiverENS_8InstanceELZNS_20sCustomEventReceiverEES2_E7Creator6createEv")]
pub fn stub_0x4b20dc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"CustomEventReceiver"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_19CustomEventReceiverENS_8InstanceELZNS_20sCustomEventReceiverEES2_E12getClassNameEv")]
pub fn stub_0x4b27a8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"CustomEventReceiver"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_19CustomEventReceiverENS_8InstanceELZNS_20sCustomEventReceiverEES2_E12getClassNameEv")]
pub fn stub_0x4b27b8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"CustomEventReceiver"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_19CustomEventReceiverENS_8InstanceELZNS_20sCustomEventReceiverEES2_E17static_getCreatorEv")]
pub fn stub_0x4b27c8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"CustomEventReceiver"
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_20sCustomEventReceiverEEEERKS0_v")]
pub fn stub_0x4b3964(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sCustomEventReceiver>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_20sCustomEventReceiverEEEEvv")]
pub fn stub_0x4b39a8() -> crate::slot::PortedFn {
// IDA 0x4b39a8: void RBX::Name::callDoDeclare<RBX::sCustomEventReceiver>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4b39a8, "void RBX::Name::callDoDeclare<RBX::sCustomEventReceiver>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_20sCustomEventReceiverEEEERKS0_v")]
pub fn stub_0x4b39ac(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sCustomEventReceiver>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_19CustomEventReceiverENS_8InstanceELZNS_20sCustomEventReceiverEES2_E7CreatorC2Ev")]
pub fn stub_0x4b3a90() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"CustomEventReceiver"
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Feature::TopBottom>::singleton(void)")]
pub fn stub_0x4cdf80(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Feature::TopBottom>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Feature::TopBottom>::construct_func(char const*,char *)")]
pub fn stub_0x4cdfec(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Feature::TopBottom>::construct_func(char const*, ch~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Feature::TopBottom>::destruct_func(char *)")]
pub fn stub_0x4cdff8(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Feature::TopBottom>::destruct_func(char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Feature::TopBottom const& rbx::any_cast<RBX::Feature::TopBottom const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x4ce0c8(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::TopBottom>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>> *)")]
pub fn stub_0x4ce234(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Feature::LeftRight>(RBX::Feature::LeftRight const&)")]
pub fn stub_0x4ce958() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Feature::LeftRight>::singleton(void)")]
pub fn stub_0x4ce9a8(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Feature::LeftRight>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Feature::LeftRight>::construct_func(char const*,char *)")]
pub fn stub_0x4cea14(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Feature::LeftRight>::construct_func(char const*, ch~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Feature::LeftRight>::destruct_func(char *)")]
pub fn stub_0x4cea20(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Feature::LeftRight>::destruct_func(char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Feature::LeftRight const& rbx::any_cast<RBX::Feature::LeftRight const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x4ceaf0(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::LeftRight>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>> *)")]
pub fn stub_0x4cec5c(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Feature::InOut>(RBX::Feature::InOut const&)")]
pub fn stub_0x4cf380() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Feature::InOut>::singleton(void)")]
pub fn stub_0x4cf3d0(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Feature::InOut>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Feature::InOut>::construct_func(char const*,char *)")]
pub fn stub_0x4cf43c(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Feature::InOut>::construct_func(char const*, char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Feature::InOut>::destruct_func(char *)")]
pub fn stub_0x4cf448(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Feature::InOut>::destruct_func(char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Feature::InOut const& rbx::any_cast<RBX::Feature::InOut const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x4cf518(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::InOut>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::InOut>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::InOut>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Feature::InOut>> *)")]
pub fn stub_0x4cf684(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::LegacyController::InputType>(RBX::LegacyController::InputType const&)")]
pub fn stub_0x4d08c4() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::LegacyController::InputType>::singleton(void)")]
pub fn stub_0x4d0914(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::LegacyController::InputType>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::LegacyController::InputType>::construct_func(char const*,char *)")]
pub fn stub_0x4d0980(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::LegacyController::InputType>::construct_func(char c~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::LegacyController::InputType>::destruct_func(char *)")]
pub fn stub_0x4d098c(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::LegacyController::InputType>::destruct_func(char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::LegacyController::InputType const& rbx::any_cast<RBX::LegacyController::InputType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x4d0a5c(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::LegacyController::InputType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>> *)")]
pub fn stub_0x4d0bc8(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GuiObject::TweenEasingDirection>(RBX::GuiObject::TweenEasingDirection const&)")]
pub fn stub_0x4d13e0() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiObject::TweenEasingDirection>::singleton(void)")]
pub fn stub_0x4d1430(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::GuiObject::TweenEasingDirection>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiObject::TweenEasingDirection>::construct_func(char const*,char *)")]
pub fn stub_0x4d149c(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::GuiObject::TweenEasingDirection>::construct_func(ch~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiObject::TweenEasingDirection>::destruct_func(char *)")]
pub fn stub_0x4d14a8(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::GuiObject::TweenEasingDirection>::destruct_func(cha~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::GuiObject::TweenEasingDirection const& rbx::any_cast<RBX::GuiObject::TweenEasingDirection const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x4d1578(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>> *)")]
pub fn stub_0x4d16e4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GuiObject::TweenStatus>(RBX::GuiObject::TweenStatus const&)")]
pub fn stub_0x4d1e08() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiObject::TweenStatus>::singleton(void)")]
pub fn stub_0x4d1e58(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::GuiObject::TweenStatus>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiObject::TweenStatus>::construct_func(char const*,char *)")]
pub fn stub_0x4d1ec4(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::GuiObject::TweenStatus>::construct_func(char const*~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiObject::TweenStatus>::destruct_func(char *)")]
pub fn stub_0x4d1ed0(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::GuiObject::TweenStatus>::destruct_func(char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::GuiObject::TweenStatus const& rbx::any_cast<RBX::GuiObject::TweenStatus const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x4d1fa0(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>> *)")]
pub fn stub_0x4d210c(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GuiObject::TweenEasingStyle>(RBX::GuiObject::TweenEasingStyle const&)")]
pub fn stub_0x4d2830() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiObject::TweenEasingStyle>::singleton(void)")]
pub fn stub_0x4d2880(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::GuiObject::TweenEasingStyle>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiObject::TweenEasingStyle>::construct_func(char const*,char *)")]
pub fn stub_0x4d28ec(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::GuiObject::TweenEasingStyle>::construct_func(char c~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiObject::TweenEasingStyle>::destruct_func(char *)")]
pub fn stub_0x4d28f8(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::GuiObject::TweenEasingStyle>::destruct_func(char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::GuiObject::TweenEasingStyle const& rbx::any_cast<RBX::GuiObject::TweenEasingStyle const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x4d29c8(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>> *)")]
pub fn stub_0x4d2b34(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GuiObject::SizeConstraint>(RBX::GuiObject::SizeConstraint const&)")]
pub fn stub_0x4d3258() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiObject::SizeConstraint>::singleton(void)")]
pub fn stub_0x4d32a8(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::GuiObject::SizeConstraint>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}
