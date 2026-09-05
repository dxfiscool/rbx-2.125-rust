// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield (4818 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x3b99b4..0x3bde94 | script 22902->23002 distinct (filler 0x3b99b4 asc, not-in-script 62643->62543)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12BadgeServiceELZNS_13sBadgeServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sBadgeServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3b99b4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12BadgeServiceELZNS_13sBadgeServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sBadgeServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3b99bc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12BadgeServiceELZNS_13sBadgeServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sBadgeServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3b9a60(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12BadgeServiceELZNS_13sBadgeServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sBadgeServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3b9a68(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::~RemoteEventDesc() [0x3b9b0c]")]
pub fn stub_0x3b9b0c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::BadgeService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x3b9bc0() -> crate::slot::SlotConnection {
// IDA 0x3b9bc0: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::isBroadcast(void)const")]
pub fn stub_0x3b9d2c() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::BadgeService")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::BadgeService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x3b9d34(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescImpl<1, RBX::BadgeService, void (std::string), rbx::remote_signa~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x3b9ed8() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::BadgeService")
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::BadgeService::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x3b9ee8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescBase<RBX::BadgeService, void (std::string), rbx::remote_signal<v~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::BadgeService::*>::EventDesc(rbx::remote_signal<void ()(std::string)> RBX::BadgeService::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x3b9efc() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::BadgeService")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::BadgeService::*>::~EventDesc()")]
pub fn stub_0x3ba080(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::BadgeService::*>::~EventDesc() [0x3ba0a4]")]
pub fn stub_0x3ba0a4(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::BadgeService,void ()(std::string),1>::BoundFuncDesc(void (RBX::BadgeService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x3ba158() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::BadgeService", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::BadgeService,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x3ba2d0() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::BadgeService", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::BadgeService,void ()(std::string),1>::~BoundFuncDesc() [0x3ba300]")]
pub fn stub_0x3ba300(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::BadgeService,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x3ba3cc() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::BadgeService", "void", 1)
}

#[doc(alias = "RBX::Reflection::Call1Helper<RBX::BadgeService,void (RBX::BadgeService::*)(std::string),std::string,void>::call(RBX::BadgeService*,void (RBX::BadgeService::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
pub fn stub_0x3ba508(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Call1Helper<RBX::BadgeService, void (RBX::BadgeService::*)(std::string), ~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::BadgeService,void ()(int),1>::BoundFuncDesc(void (RBX::BadgeService::*)(int),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x3ba638() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::BadgeService", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::BadgeService,void ()(int),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x3ba7b0() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::BadgeService", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::BadgeService,void ()(int),1>::~BoundFuncDesc() [0x3ba7e0]")]
pub fn stub_0x3ba7e0(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::BadgeService,void ()(int),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x3ba8b4() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::BadgeService", "void", 1)
}

#[doc(alias = "rbx::remote_signal<void ()(std::string)>::~remote_signal()")]
pub fn stub_0x3bb204() -> crate::slot::PortedFn {
// IDA 0x3bb204: rbx::remote_signal<void (std::string)>::~remote_signal().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3bb204, "rbx::remote_signal<void (std::string)>::~remote_signal()")
}

#[doc(alias = "RBX::FormFactorPart::setFormFactorUi(RBX::PartInstance::FormFactor)")]
pub fn stub_0x3bb8e8(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::FormFactorPart setter.
cell.set(value)
}

#[doc(alias = "RBX::FormFactorPart::setFormFactorXml(RBX::PartInstance::FormFactor)")]
pub fn stub_0x3bb964(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::FormFactorPart setter.
cell.set(value)
}

#[doc(alias = "RBX::BasicPartInstance::setLegacyPartTypeXml(RBX::BasicPartInstance::LegacyPartType)")]
pub fn stub_0x3bbe50(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::BasicPartInstance setter.
cell.set(value)
}

#[doc(alias = "RBX::BasicPartInstance::setLegacyPartTypeUi(RBX::BasicPartInstance::LegacyPartType)")]
pub fn stub_0x3bbecc(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::BasicPartInstance setter.
cell.set(value)
}

#[doc(alias = "RBX::BasicPartInstance::BasicPartInstance(void)")]
pub fn stub_0x3bbf78() -> crate::slot::InstanceHandle {
// RBX::BasicPartInstance ctor.
crate::slot::InstanceHandle::new("RBX::BasicPartInstance")
}

#[doc(alias = "RBX::BasicPartInstance::BasicPartInstance(void) [0x3bbff0]")]
pub fn stub_0x3bbff0() -> crate::slot::InstanceHandle {
// RBX::BasicPartInstance ctor.
crate::slot::InstanceHandle::new("RBX::BasicPartInstance")
}

#[doc(alias = "RBX::BasicPartInstance::~BasicPartInstance()")]
pub fn stub_0x3bc054(handle: crate::slot::InstanceHandle) {
// RBX::BasicPartInstance dtor.
drop(handle);
}

#[doc(alias = "RBX::BasicPartInstance::~BasicPartInstance() [0x3bc104]")]
pub fn stub_0x3bc104(handle: crate::slot::InstanceHandle) {
// RBX::BasicPartInstance dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::BasicPartInstance::~BasicPartInstance()")]
pub fn stub_0x3bc114(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::BasicPartInstance::~BasicPartInstance() [0x3bc11c]")]
pub fn stub_0x3bc11c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::BasicPartInstance::~BasicPartInstance() [0x3bc124]")]
pub fn stub_0x3bc124(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 132, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 132);
}

#[doc(alias = "RBX::BasicPartInstance::~BasicPartInstance() [0x3bc12c]")]
pub fn stub_0x3bc12c(handle: crate::slot::InstanceHandle) {
// RBX::BasicPartInstance dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::BasicPartInstance::~BasicPartInstance() [0x3bc134]")]
pub fn stub_0x3bc134(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::BasicPartInstance::~BasicPartInstance() [0x3bc148]")]
pub fn stub_0x3bc148(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::BasicPartInstance::~BasicPartInstance() [0x3bc15c]")]
pub fn stub_0x3bc15c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 132, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 132);
}

#[doc(alias = "RBX::BasicPartInstance::validateFormFactor(RBX::PartInstance::FormFactor &)")]
pub fn stub_0x3bc170(handle: &crate::slot::InstanceHandle) {
// RBX::BasicPartInstance::validateFormFactor(RBX::PartInstance::FormFactor&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::BasicPartInstance::getPartType(void)const")]
pub fn stub_0x3bc180(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::BasicPartInstance getter.
cell.get()
}

#[doc(alias = "RBX::BasicPartInstance::hasThreeDimensionalSize(void)")]
pub fn stub_0x3bc1e0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::BasicPartInstance getter.
cell.get()
}

#[doc(alias = "RBX::BasicPartInstance::partNeeds3dAdorn(void)const")]
pub fn stub_0x3bc1ec(handle: &crate::slot::InstanceHandle) {
// RBX::BasicPartInstance::partNeeds3dAdorn() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::~EnumPropDescriptor()")]
pub fn stub_0x3bc1fc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FormFactorPart,RBX::PartInstance::FormFactor>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x3bc220(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::~EnumPropDescriptor()")]
pub fn stub_0x3bc460(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::BasicPartInstance::getLegacyPartType(void)const")]
pub fn stub_0x3bc484(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::BasicPartInstance getter.
cell.get()
}

#[doc(alias = "__ZN3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2Ev")]
pub fn stub_0x3bc48c() -> crate::slot::InstanceHandle {
// RBX::DescribedCreatable ctor.
crate::slot::InstanceHandle::new("RBX::DescribedCreatable")
}

#[doc(alias = "RBX::FormFactorPart::validateFormFactor(RBX::PartInstance::FormFactor &)")]
pub fn stub_0x3bc6d8(handle: &crate::slot::InstanceHandle) {
// RBX::FormFactorPart::validateFormFactor(RBX::PartInstance::FormFactor&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3bc704(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "__ZN3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3bc718(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "__ZThn132_N3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3bc7c8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 132, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 132);
}

#[doc(alias = "__ZThn132_N3RBX21DescribedNonCreatableINS_14FormFactorPartENS_12PartInstanceELZNS_15sFormFactorPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3bc7dc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 132, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 132);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3bc890(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3bc8a4(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn132_N3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3bc954(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 132, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 132);
}

#[doc(alias = "__ZThn132_N3RBX10Reflection9DescribedINS_14FormFactorPartELZNS_15sFormFactorPartEENS_17NonFactoryProductINS_12PartInstanceELZNS_15sFormFactorPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3bc968(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 132, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 132);
}

#[doc(alias = "__ZN3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3bcbc4(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "__ZN3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3bcbd8(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "__ZThn132_N3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3bcc88(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 132, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 132);
}

#[doc(alias = "__ZThn132_N3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3bcc9c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 132, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 132);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_17BasicPartInstanceELZNS_10sBasicPartEENS_14FactoryProductIS2_NS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3bcd50(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_17BasicPartInstanceELZNS_10sBasicPartEENS_14FactoryProductIS2_NS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3bcd64(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn132_N3RBX10Reflection9DescribedINS_17BasicPartInstanceELZNS_10sBasicPartEENS_14FactoryProductIS2_NS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3bce14(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 132, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 132);
}

#[doc(alias = "__ZThn132_N3RBX10Reflection9DescribedINS_17BasicPartInstanceELZNS_10sBasicPartEENS_14FactoryProductIS2_NS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3bce28(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 132, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 132);
}

#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3bd14c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3bd160(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3bd214(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3bd228(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_17BasicPartInstanceELZNS_10sBasicPartEENS_14FactoryProductIS2_NS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3bd2dc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_17BasicPartInstanceELZNS_10sBasicPartEENS_14FactoryProductIS2_NS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3bd2f0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_17BasicPartInstanceELZNS_10sBasicPartEENS_14FactoryProductIS2_NS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3bd3a4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_17BasicPartInstanceELZNS_10sBasicPartEENS_14FactoryProductIS2_NS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3bd3b8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::EnumPropDescriptor<RBX::BasicPartInstance::LegacyPartType (RBX::BasicPartInstance::*)(void)const,void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType)>(char const*,char const*,RBX::BasicPartInstance::LegacyPartType (RBX::BasicPartInstance::*)(void)const,void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x3bd46c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::~EnumPropDescriptor() [0x3bd620]")]
pub fn stub_0x3bd620(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::isReadOnly(void)const")]
pub fn stub_0x3bd64c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::isWriteOnly(void)const")]
pub fn stub_0x3bd65c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x3bd66c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x3bd694(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x3bd6b8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x3bd804(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::hasStringValue(void)const")]
pub fn stub_0x3bd828(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x3bd82c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x3bd850(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x3bd890(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x3bd8b0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x3bdaf0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_0x3bdb0c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x3bdb40(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x3bdb48(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x3bdb94(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_0x3bdbb4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::convertToIndex(RBX::BasicPartInstance::LegacyPartType)const")]
pub fn stub_0x3bdbe8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::convertToIndex(RBX::Bas~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x3bdc58(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::GetSetImpl<RBX::BasicPartInstance::LegacyPartType (RBX::BasicPartInstance::*)(void)const,void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType)>::isReadOnly(void)const")]
pub fn stub_0x3bdc98(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::GetSetImpl<RBX::BasicPartInstance::LegacyPartType (RBX::BasicPartInstance::*)(void)const,void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType)>::isWriteOnly(void)const")]
pub fn stub_0x3bdc9c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::GetSetImpl<RBX::BasicPartInstance::LegacyPartType (RBX::BasicPartInstance::*)(void)const,void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x3bdca0(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::GetSetImpl<RBX::BasicPartInstance::LegacyPartType (RBX::BasicPartInstance::*)(void)const,void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType)>::setValue(RBX::Reflection::DescribedBase *,RBX::BasicPartInstance::LegacyPartType const&)const")]
pub fn stub_0x3bdcc0(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::EnumPropDescriptor<int,void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType)>(char const*,char const*,int,void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x3bdce4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::SetImpl<void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType)>::isReadOnly(void)const")]
pub fn stub_0x3bde90(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BasicPartInstance,RBX::BasicPartInstance::LegacyPartType>::SetImpl<void (RBX::BasicPartInstance::*)(RBX::BasicPartInstance::LegacyPartType)>::isWriteOnly(void)const")]
pub fn stub_0x3bde94(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}
