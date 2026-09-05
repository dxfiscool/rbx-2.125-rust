// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 150 not yet in script
// Filter: Script|Lua|Yield|lua (5401 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +150 stubs | range 0x396c00..0x39d684 | script 22202->22352 distinct (filler 0x396c00 asc, not-in-script 63343->63193)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "RBX::Animation::isEmbeddedAsset(void)const")]
pub fn stub_0x396c00(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Animation getter.
cell.get()
}

#[doc(alias = "RBX::Animation::getKeyframeSequence(RBX::Instance const*)const")]
pub fn stub_0x396c40(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Animation getter.
cell.get()
}

#[doc(alias = "RBX::Animation::getAssetId(void)const")]
pub fn stub_0x396e08(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Animation getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Animation,RBX::AnimationId>::~PropDescriptor()")]
pub fn stub_0x396e20(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::KeyframeSequenceProvider * RBX::ServiceProvider::create<RBX::KeyframeSequenceProvider>(RBX::Instance const*)")]
pub fn stub_0x396e44() -> crate::slot::ServiceHandle {
// ServiceProvider::create — constructs + registers.
crate::slot::ServiceHandle::new("RBX::KeyframeSequenceProvider")
}

#[doc(alias = "RBX::Animation::~Animation()")]
pub fn stub_0x396e5c(handle: crate::slot::InstanceHandle) {
// RBX::Animation dtor.
drop(handle);
}

#[doc(alias = "RBX::Animation::~Animation() [0x396f40]")]
pub fn stub_0x396f40(handle: crate::slot::InstanceHandle) {
// RBX::Animation dtor.
drop(handle);
}

#[doc(alias = "RBX::Animation::getPersistentDataCost(void)const")]
pub fn stub_0x397038(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Animation getter.
cell.get()
}

#[doc(alias = "RBX::Animation::askSetParent(RBX::Instance const*)const")]
pub fn stub_0x3970bc(handle: &crate::slot::InstanceHandle) {
// RBX::Animation::askSetParent(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_9AnimationENS_8InstanceELZNS_10sAnimationEES2_E12getClassNameEv")]
pub fn stub_0x3970c0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Animation"
}

#[doc(alias = "non-virtual thunk toRBX::Animation::~Animation()")]
pub fn stub_0x3970d0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Animation::~Animation() [0x3971b4]")]
pub fn stub_0x3971b4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_9AnimationENS_8InstanceELZNS_10sAnimationEES2_E12getClassNameEv")]
pub fn stub_0x3972ac() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Animation"
}

#[doc(alias = "non-virtual thunk toRBX::Animation::~Animation() [0x3972bc]")]
pub fn stub_0x3972bc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::Animation::~Animation() [0x3973a0]")]
pub fn stub_0x3973a0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9AnimationENS_8InstanceELZNS_10sAnimationEES2_E17static_getCreatorEv")]
pub fn stub_0x397498() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Animation"
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9AnimationELZNS_10sAnimationEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sAnimationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x39750c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9AnimationELZNS_10sAnimationEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sAnimationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x397510(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9AnimationELZNS_10sAnimationEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sAnimationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3975b0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9AnimationELZNS_10sAnimationEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sAnimationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3975b8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9AnimationELZNS_10sAnimationEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sAnimationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x39765c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9AnimationELZNS_10sAnimationEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sAnimationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x397664(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Animation,RBX::AnimationId>::PropDescriptor<RBX::AnimationId (RBX::Animation::*)(void)const,void (RBX::Animation::*)(RBX::AnimationId)>(char const*,char const*,RBX::AnimationId (RBX::Animation::*)(void)const,void (RBX::Animation::*)(RBX::AnimationId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x397708(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x39781c() -> crate::slot::InstanceHandle {
// RBX::Reflection::TypedPropertyDescriptor ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::TypedPropertyDescriptor")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Animation,RBX::AnimationId>::~PropDescriptor() [0x397940]")]
pub fn stub_0x397940(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::isReadOnly(void)const")]
pub fn stub_0x39796c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Reflection::TypedPropertyDescriptor getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::isWriteOnly(void)const")]
pub fn stub_0x39797c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Reflection::TypedPropertyDescriptor getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x39798c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::equalValues(RBX::Reflection::D~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x397b38(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Reflection::TypedPropertyDescriptor getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x397c64(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Reflection::TypedPropertyDescriptor setter.
cell.set(value)
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x397e60(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::copyValue(RBX::Reflection::Des~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AnimationId const& rbx::any_cast<RBX::AnimationId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x397f88(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::~TypedPropertyDescriptor()")]
pub fn stub_0x398078(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::TypedPropertyDescriptor dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::~TypedPropertyDescriptor() [0x39809c]")]
pub fn stub_0x39809c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::TypedPropertyDescriptor dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Animation,RBX::AnimationId>::GetSetImpl<RBX::AnimationId (RBX::Animation::*)(void)const,void (RBX::Animation::*)(RBX::AnimationId)>::isReadOnly(void)const")]
pub fn stub_0x3980c8(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Animation,RBX::AnimationId>::GetSetImpl<RBX::AnimationId (RBX::Animation::*)(void)const,void (RBX::Animation::*)(RBX::AnimationId)>::isWriteOnly(void)const")]
pub fn stub_0x3980cc(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Animation,RBX::AnimationId>::GetSetImpl<RBX::AnimationId (RBX::Animation::*)(void)const,void (RBX::Animation::*)(RBX::AnimationId)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x3980d0(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Animation,RBX::AnimationId>::GetSetImpl<RBX::AnimationId (RBX::Animation::*)(void)const,void (RBX::Animation::*)(RBX::AnimationId)>::setValue(RBX::Reflection::DescribedBase *,RBX::AnimationId const&)const")]
pub fn stub_0x3980f8(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "global constructor keyed to_a_155")]
pub fn stub_0x398240() -> crate::slot::PortedFn {
// IDA 0x398240: __GLOBAL__I_a_155.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x398240, "__GLOBAL__I_a_155")
}

#[doc(alias = "RBX::AnimationTrack::play(float,float,float)")]
pub fn stub_0x398554(handle: &crate::slot::InstanceHandle) {
// RBX::AnimationTrack::play(float, float, float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AnimationTrack::stop(float)")]
pub fn stub_0x398694(handle: &crate::slot::InstanceHandle) {
// RBX::AnimationTrack::stop(float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AnimationTrack::adjustWeight(float,float)")]
pub fn stub_0x39869c(handle: &crate::slot::InstanceHandle) {
// RBX::AnimationTrack::adjustWeight(float, float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AnimationTrack::adjustSpeed(float)")]
pub fn stub_0x3987cc(handle: &crate::slot::InstanceHandle) {
// RBX::AnimationTrack::adjustSpeed(float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AnimationTrack::AnimationTrack(rbx_core::SharedPtr<RBX::AnimationTrackState>,rbx_core::Weak<RBX::Animator>)")]
pub fn stub_0x3988f0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AnimationTrackState")
}

#[doc(alias = "RBX::AnimationTrack::AnimationTrack(rbx_core::SharedPtr<RBX::AnimationTrackState>,rbx_core::Weak<RBX::Animator>) [0x3988f4]")]
pub fn stub_0x3988f4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AnimationTrackState")
}

#[doc(alias = "RBX::AnimationTrack::forwardKeyframeReached(std::string)")]
pub fn stub_0x398d64(handle: &crate::slot::InstanceHandle) {
// RBX::AnimationTrack::forwardKeyframeReached(std::string) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AnimationTrack::~AnimationTrack()")]
pub fn stub_0x398e80(handle: crate::slot::InstanceHandle) {
// RBX::AnimationTrack dtor.
drop(handle);
}

#[doc(alias = "RBX::AnimationTrack::~AnimationTrack() [0x398f20]")]
pub fn stub_0x398f20(handle: crate::slot::InstanceHandle) {
// RBX::AnimationTrack dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::AnimationTrack::~AnimationTrack()")]
pub fn stub_0x398f24(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::AnimationTrack::~AnimationTrack() [0x398f2c]")]
pub fn stub_0x398f2c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::AnimationTrack::~AnimationTrack() [0x398f34]")]
pub fn stub_0x398f34(handle: crate::slot::InstanceHandle) {
// RBX::AnimationTrack dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::AnimationTrack::~AnimationTrack() [0x3991a8]")]
pub fn stub_0x3991a8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::AnimationTrack::~AnimationTrack() [0x3991b0]")]
pub fn stub_0x3991b0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float,float),3>::~BoundFuncDesc()")]
pub fn stub_0x3991b8(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float),1>::~BoundFuncDesc()")]
pub fn stub_0x39920c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float),2>::~BoundFuncDesc()")]
pub fn stub_0x39924c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrack,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::AnimationTrack::*>::~EventDesc()")]
pub fn stub_0x399294(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrack,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::AnimationTrack::*>::~EventDesc()")]
pub fn stub_0x3992b8(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEE12getClassNameEv")]
pub fn stub_0x3992dc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEE12getClassNameEv")]
pub fn stub_0x399304() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_15sAnimationTrackEEEEvv")]
pub fn stub_0x39932c() -> crate::slot::PortedFn {
// IDA 0x39932c: void RBX::Name::callDoDeclare<RBX::sAnimationTrack>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x39932c, "void RBX::Name::callDoDeclare<RBX::sAnimationTrack>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sAnimationTrackEEEERKS0_v")]
pub fn stub_0x399330(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sAnimationTrack>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Animator>::shared_ptr<RBX::Animator>(rbx_core::Weak<RBX::Animator> const&,boost::detail::sp_nothrow_tag)")]
pub fn stub_0x399410() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Animator")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>> const&)")]
pub fn stub_0x39948c() -> crate::slot::SlotConnection {
// IDA 0x39948c: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_0x399500(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>>>::~callable_slot() [0x39952c]")]
pub fn stub_0x39952c(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>>,1,void ()(std::string)>::call(std::string)")]
pub fn stub_0x399600(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x399600: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>>,1,void ()(std::string)>::call(std::string)")]
pub fn stub_0x39961c(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x39961c: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list1<std::string &>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string> &,boost::_bi::list1<std::string &> &,int)")]
pub fn stub_0x399638(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x399638: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>::operator()(RBX::AnimationTrack*,std::string)const")]
pub fn stub_0x399758() -> crate::slot::BindPiece {
// boost::bind fragment (mf1) composing a host BoundCall.
crate::slot::BindPiece::new("mf1")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>>,1,void ()(std::string)>::~callable()")]
pub fn stub_0x39988c(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x39988c: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>>,1,void ()(std::string)>::~callable() [0x3998b8]")]
pub fn stub_0x3998b8(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3998b8: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x39998c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x399990(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x399a30(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x399a38(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x399adc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x399ae4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrack,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::AnimationTrack::*>::~EventDesc() [0x399b88]")]
pub fn stub_0x399b88(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::AnimationTrack,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::AnimationTrack::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x399c3c() -> crate::slot::SlotConnection {
// IDA 0x399c3c: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::AnimationTrack,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::AnimationTrack::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x399e40(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescImpl<0, RBX::AnimationTrack, void (), rbx::signal<void ()>, rbx:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::AnimationTrack,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::AnimationTrack::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x399eb4(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescBase<RBX::AnimationTrack, void (), rbx::signal<void ()>, rbx::si~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrack,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::AnimationTrack::*>::EventDesc(rbx::signal<void ()(std::string)> RBX::AnimationTrack::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x399ec8() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::AnimationTrack")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrack,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::AnimationTrack::*>::~EventDesc() [0x39a04c]")]
pub fn stub_0x39a04c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::AnimationTrack,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::AnimationTrack::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x39a100() -> crate::slot::SlotConnection {
// IDA 0x39a100: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::AnimationTrack,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::AnimationTrack::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x39a254(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescImpl<1, RBX::AnimationTrack, void (std::string), rbx::signal<voi~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::AnimationTrack,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::AnimationTrack::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x39a3f8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescBase<RBX::AnimationTrack, void (std::string), rbx::signal<void (~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float),2>::BoundFuncDesc(void (RBX::AnimationTrack::*)(float,float),char const*,char const*,float,char const*,float,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x39a40c() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::AnimationTrack", "void", 2)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0x39a648() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::AnimationTrack", "void", 2)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float),2>::~BoundFuncDesc() [0x39a694]")]
pub fn stub_0x39a694(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x39a774() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::AnimationTrack", "void", 2)
}

#[doc(alias = "float RBX::Reflection::ArgHelper::getArg<float,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<float> const&,boost::disable_if<boost::is_same<float,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_0x39a7d4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float),1>::BoundFuncDesc(void (RBX::AnimationTrack::*)(float),char const*,char const*,float,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x39a978() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::AnimationTrack", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x39ab30() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::AnimationTrack", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float),1>::~BoundFuncDesc() [0x39ab60]")]
pub fn stub_0x39ab60(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x39ac34() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::AnimationTrack", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float,float),3>::BoundFuncDesc(void (RBX::AnimationTrack::*)(float,float,float),char const*,char const*,float,char const*,float,char const*,float,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x39ac70() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::AnimationTrack", "void", 3)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float,float),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0x39af34() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::AnimationTrack", "void", 3)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float,float),3>::~BoundFuncDesc() [0x39af9c]")]
pub fn stub_0x39af9c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float,float),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x39b088() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::AnimationTrack", "void", 3)
}

#[doc(alias = "global constructor keyed to_a_156")]
pub fn stub_0x39b100() -> crate::slot::PortedFn {
// IDA 0x39b100: __GLOBAL__I_a_156.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x39b100, "__GLOBAL__I_a_156")
}

#[doc(alias = "RBX::AnimationTrackState::AnimationTrackState(rbx_core::SharedPtr<RBX::KeyframeSequence const>,rbx_core::Weak<RBX::Animator const>)")]
pub fn stub_0x39b490() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::KeyframeSequence const")
}

#[doc(alias = "RBX::AnimationTrackState::AnimationTrackState(rbx_core::SharedPtr<RBX::KeyframeSequence const>,rbx_core::Weak<RBX::Animator const>) [0x39b494]")]
pub fn stub_0x39b494() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::KeyframeSequence const")
}

#[doc(alias = "RBX::AnimationTrackState::onPlay(float,float,float,float)")]
pub fn stub_0x39b950(handle: &crate::slot::InstanceHandle) {
// RBX::AnimationTrackState::onPlay(float, float, float, float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AnimationTrackState::onStop(float,float)")]
pub fn stub_0x39b9ac(handle: &crate::slot::InstanceHandle) {
// RBX::AnimationTrackState::onStop(float, float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AnimationTrackState::onAdjustWeight(float,float,float)")]
pub fn stub_0x39b9f4(handle: &crate::slot::InstanceHandle) {
// RBX::AnimationTrackState::onAdjustWeight(float, float, float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AnimationTrackState::onAdjustSpeed(float,float)")]
pub fn stub_0x39ba40(handle: &crate::slot::InstanceHandle) {
// RBX::AnimationTrackState::onAdjustSpeed(float, float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AnimationTrackState::isStopped(double)")]
pub fn stub_0x39ba88(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::AnimationTrackState getter.
cell.get()
}

#[doc(alias = "RBX::AnimationTrackState::getGameTime(void)")]
pub fn stub_0x39bb00(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::AnimationTrackState getter.
cell.get()
}

#[doc(alias = "RBX::AnimationTrackState::getWeightAtTime(double)")]
pub fn stub_0x39bc00(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::AnimationTrackState getter.
cell.get()
}

#[doc(alias = "RBX::AnimationTrackState::play(float,float,float)")]
pub fn stub_0x39bc5c(handle: &crate::slot::InstanceHandle) {
// RBX::AnimationTrackState::play(float, float, float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AnimationTrackState::stop(float)")]
pub fn stub_0x39bcbc(handle: &crate::slot::InstanceHandle) {
// RBX::AnimationTrackState::stop(float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AnimationTrackState::adjustWeight(float,float)")]
pub fn stub_0x39bd0c(handle: &crate::slot::InstanceHandle) {
// RBX::AnimationTrackState::adjustWeight(float, float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AnimationTrackState::adjustSpeed(float)")]
pub fn stub_0x39bd64(handle: &crate::slot::InstanceHandle) {
// RBX::AnimationTrackState::adjustSpeed(float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AnimationTrackState::triggerKeyframeReachedSignal(rbx_core::SharedPtr<RBX::Instance> const&,double,double)")]
pub fn stub_0x39bdb4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::AnimationTrackState::step(std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> &,double)")]
pub fn stub_0x39bf44(handle: &crate::slot::InstanceHandle) {
// RBX::AnimationTrackState::step(std::vector<RBX::PoseAccumulator, std::allocator<RBX::PoseA~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>>::~RemoteEventDesc()")]
pub fn stub_0x39c124(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>>::~RemoteEventDesc()")]
pub fn stub_0x39c148(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>>::~RemoteEventDesc()")]
pub fn stub_0x39c16c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::~RemoteEventDesc()")]
pub fn stub_0x39c190(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<1,RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::fireAndReplicateEvent(RBX::AnimationTrackState*,std::string)")]
pub fn stub_0x39c1b4(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::RemoteEventDescImpl<1, RBX::AnimationTrackState, void (std::string), rbx:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AnimationTrackState::~AnimationTrackState()")]
pub fn stub_0x39c370(handle: crate::slot::InstanceHandle) {
// RBX::AnimationTrackState dtor.
drop(handle);
}

#[doc(alias = "RBX::AnimationTrackState::~AnimationTrackState() [0x39c374]")]
pub fn stub_0x39c374(handle: crate::slot::InstanceHandle) {
// RBX::AnimationTrackState dtor.
drop(handle);
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEE12getClassNameEv")]
pub fn stub_0x39c414() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "non-virtual thunk toRBX::AnimationTrackState::~AnimationTrackState()")]
pub fn stub_0x39c43c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::AnimationTrackState::~AnimationTrackState() [0x39c444]")]
pub fn stub_0x39c444(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEE12getClassNameEv")]
pub fn stub_0x39c44c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "non-virtual thunk toRBX::AnimationTrackState::~AnimationTrackState() [0x39c474]")]
pub fn stub_0x39c474(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::AnimationTrackState::~AnimationTrackState() [0x39c47c]")]
pub fn stub_0x39c47c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::AnimationTrackState::~AnimationTrackState() [0x39c484]")]
pub fn stub_0x39c484(handle: crate::slot::InstanceHandle) {
// RBX::AnimationTrackState dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_20sAnimationTrackStateEEEEvv")]
pub fn stub_0x39c640() -> crate::slot::PortedFn {
// IDA 0x39c640: void RBX::Name::callDoDeclare<RBX::sAnimationTrackState>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x39c640, "void RBX::Name::callDoDeclare<RBX::sAnimationTrackState>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_20sAnimationTrackStateEEEERKS0_v")]
pub fn stub_0x39c644(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sAnimationTrackState>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::fireEvent(RBX::AnimationTrackState*,std::string)const")]
pub fn stub_0x39c724(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescImpl<1, RBX::AnimationTrackState, void (std::string), rbx::remot~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<1,RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::replicateEvent(RBX::Reflection::EventSource *,std::string)")]
pub fn stub_0x39c840(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::RemoteEventDescImpl<1, RBX::AnimationTrackState, void (std::string), rbx:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<3,RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>>::replicateEvent(RBX::Reflection::EventSource *,float,float,float)")]
pub fn stub_0x39c98c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::RemoteEventDescImpl<3, RBX::AnimationTrackState, void (float, float, floa~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(float,float,float)>::operator()(float,float,float)")]
pub fn stub_0x39cb28(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal_with_args<3, void (float, float, float)>::operator()(float, float, fl~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(float,float,float)>::slot> &)")]
pub fn stub_0x39cc88() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("rbx::signals::signal<void (float, float, float)>::slot")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::on_error(std::exception &)")]
pub fn stub_0x39cde8(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(float,float,float)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(float,float,float)>::slot> const&)")]
pub fn stub_0x39ce10(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::safe_static_init_mutex(void)")]
pub fn stub_0x39ce34(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (float, float, float)>::safe_static_init_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::safe_static_do_get_mutex(void)")]
pub fn stub_0x39ce38(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (float, float, float)>::safe_static_do_get_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<2,RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>>::replicateEvent(RBX::Reflection::EventSource *,float,float)")]
pub fn stub_0x39cf30(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::RemoteEventDescImpl<2, RBX::AnimationTrackState, void (float, float), rbx~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<4,RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>>::replicateEvent(RBX::Reflection::EventSource *,float,float,float,float)")]
pub fn stub_0x39d09c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::RemoteEventDescImpl<4, RBX::AnimationTrackState, void (float, float, floa~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal_with_args<4,void ()(float,float,float,float)>::operator()(float,float,float,float)")]
pub fn stub_0x39d260(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal_with_args<4, void (float, float, float, float)>::operator()(float, fl~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(float,float,float,float)>::slot> &)")]
pub fn stub_0x39d3dc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("rbx::signals::signal<void (float, float, float, float)>::slot")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::on_error(std::exception &)")]
pub fn stub_0x39d53c(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(float,float,float,float)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(float,float,float,float)>::slot> const&)")]
pub fn stub_0x39d564(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::safe_static_init_mutex(void)")]
pub fn stub_0x39d588(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (float, float, float, float)>::safe_static_init_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::safe_static_do_get_mutex(void)")]
pub fn stub_0x39d58c(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (float, float, float, float)>::safe_static_do_get_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Animator const>::shared_ptr<RBX::Animator const>(rbx_core::Weak<RBX::Animator const> const&,boost::detail::sp_nothrow_tag)")]
pub fn stub_0x39d684() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Animator const")
}
