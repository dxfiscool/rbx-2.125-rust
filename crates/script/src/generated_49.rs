// Auto-generated skeletons for rbx-script — filler EA-sorted ascending after 0x4e5fc8 (next 100)
// Filter: Lua|Script|Yield|lua (5401 filtered, all already stubbed globally) — filler EA-sorted ascending
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x4e5fd0..0x4e9740 | existing ~5071 -> ~5171 total (union; filler 0x4e5fd0 ascending, global remaining 63165 -> 63065)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "RBX::VelocityMotor::~VelocityMotor() [0x4e5fd0]")]
pub fn stub_0x4e5fd0(handle: crate::slot::InstanceHandle) {
// RBX::VelocityMotor dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::VelocityMotor::~VelocityMotor() [0x4e6114]")]
pub fn stub_0x4e6114(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::VelocityMotor::~VelocityMotor() [0x4e611c]")]
pub fn stub_0x4e611c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::VelocityMotor::setPart(int,RBX::Feature *)")]
pub fn stub_0x4e6124(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::VelocityMotor setter.
cell.set(value)
}

#[doc(alias = "RBX::VelocityMotor::onAncestorChanged(RBX::AncestorChanged const&)")]
pub fn stub_0x4e61b0(handle: &crate::slot::InstanceHandle) {
// RBX::VelocityMotor::onAncestorChanged(RBX::AncestorChanged const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::VelocityMotor::onEvent_HoleAncestorChanged(void)")]
pub fn stub_0x4e62ec(handle: &crate::slot::InstanceHandle) {
// RBX::VelocityMotor::onEvent_HoleAncestorChanged() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Feature::getFaceId(void)const")]
pub fn stub_0x4e6dc4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Feature getter.
cell.get()
}

#[doc(alias = "RBX::Feature::getTopBottom(void)const")]
pub fn stub_0x4e6dec(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Feature getter.
cell.get()
}

#[doc(alias = "RBX::Feature::getLeftRight(void)const")]
pub fn stub_0x4e6e14(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Feature getter.
cell.get()
}

#[doc(alias = "RBX::Feature::getInOut(void)const")]
pub fn stub_0x4e6e3c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Feature getter.
cell.get()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::VelocityMotor> RBX::Creatable<RBX::Instance>::create<RBX::VelocityMotor>(void)")]
pub fn stub_0x4e6e68() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::VelocityMotor")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Hole>::operator=(rbx_core::SharedPtr<RBX::Hole> const&)")]
pub fn stub_0x4e6f18(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Hole> RBX::shared_from<RBX::Hole>(RBX::Hole*)")]
pub fn stub_0x4e6f50() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Hole")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::VelocityMotor>,boost::_bi::list1<boost::_bi::value<RBX::VelocityMotor*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::VelocityMotor>,boost::_bi::list1<boost::_bi::value<RBX::VelocityMotor*>>> const&)")]
pub fn stub_0x4e70c0() -> crate::slot::SlotConnection {
// IDA 0x4e70c0: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::Feature::askSetParent(RBX::Instance const*)const")]
pub fn stub_0x4e7134(handle: &crate::slot::InstanceHandle) {
// RBX::Feature::askSetParent(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_8sFeatureEEE12getClassNameEv")]
pub fn stub_0x4e7138() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "RBX::Feature::shouldRender3dAdorn(void)const")]
pub fn stub_0x4e7160(handle: &crate::slot::InstanceHandle) {
// RBX::Feature::shouldRender3dAdorn() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Feature::getCoordOrientation(void)const")]
pub fn stub_0x4e7164(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Feature getter.
cell.get()
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_8sFeatureEEE12getClassNameEv")]
pub fn stub_0x4e7168() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "non-virtual thunk toRBX::Feature::shouldRender3dAdorn(void)const")]
pub fn stub_0x4e7190(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "RBX::Hole::~Hole()")]
pub fn stub_0x4e7194(handle: crate::slot::InstanceHandle) {
// RBX::Hole dtor.
drop(handle);
}

#[doc(alias = "RBX::Hole::~Hole() [0x4e7198]")]
pub fn stub_0x4e7198(handle: crate::slot::InstanceHandle) {
// RBX::Hole dtor.
drop(handle);
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_4HoleENS_7FeatureELZNS_5sHoleEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x4e7238() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Hole"
}

#[doc(alias = "RBX::Hole::getCoordOrientation(void)const")]
pub fn stub_0x4e7248(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Hole getter.
cell.get()
}

#[doc(alias = "non-virtual thunk toRBX::Hole::~Hole()")]
pub fn stub_0x4e724c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Hole::~Hole() [0x4e7254]")]
pub fn stub_0x4e7254(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_4HoleENS_7FeatureELZNS_5sHoleEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x4e72f8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Hole"
}

#[doc(alias = "non-virtual thunk toRBX::Hole::~Hole() [0x4e7308]")]
pub fn stub_0x4e7308(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::Hole::~Hole() [0x4e7310]")]
pub fn stub_0x4e7310(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::MotorFeature::~MotorFeature()")]
pub fn stub_0x4e73b4(handle: crate::slot::InstanceHandle) {
// RBX::MotorFeature dtor.
drop(handle);
}

#[doc(alias = "RBX::MotorFeature::~MotorFeature() [0x4e73b8]")]
pub fn stub_0x4e73b8(handle: crate::slot::InstanceHandle) {
// RBX::MotorFeature dtor.
drop(handle);
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_12MotorFeatureENS_7FeatureELZNS_13sMotorFeatureEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x4e7458() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"MotorFeature"
}

#[doc(alias = "non-virtual thunk toRBX::MotorFeature::~MotorFeature()")]
pub fn stub_0x4e7468(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::MotorFeature::~MotorFeature() [0x4e7470]")]
pub fn stub_0x4e7470(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_12MotorFeatureENS_7FeatureELZNS_13sMotorFeatureEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x4e7514() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"MotorFeature"
}

#[doc(alias = "non-virtual thunk toRBX::MotorFeature::~MotorFeature() [0x4e7524]")]
pub fn stub_0x4e7524(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::MotorFeature::~MotorFeature() [0x4e752c]")]
pub fn stub_0x4e752c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::JointInstance::getPersistentDataCost(void)const")]
pub fn stub_0x4e75d0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::JointInstance getter.
cell.get()
}

#[doc(alias = "RBX::VelocityMotor::askSetParent(RBX::Instance const*)const")]
pub fn stub_0x4e75dc(handle: &crate::slot::InstanceHandle) {
// RBX::VelocityMotor::askSetParent(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_13VelocityMotorENS_13JointInstanceELZNS_14sVelocityMotorEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x4e75e0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"VelocityMotor"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_13VelocityMotorENS_13JointInstanceELZNS_14sVelocityMotorEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x4e75f0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"VelocityMotor"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_4HoleENS_7FeatureELZNS_5sHoleEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x4e7600() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Hole"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_12MotorFeatureENS_7FeatureELZNS_13sMotorFeatureEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x4e7604() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"MotorFeature"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_13VelocityMotorENS_13JointInstanceELZNS_14sVelocityMotorEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x4e7608() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"VelocityMotor"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_13VelocityMotorENS_13JointInstanceELZNS_14sVelocityMotorEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x4e760c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"VelocityMotor"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_13VelocityMotorENS_13JointInstanceELZNS_14sVelocityMotorEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x4e76a8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"VelocityMotor"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_13VelocityMotorENS_13JointInstanceELZNS_14sVelocityMotorEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x4e7730() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"VelocityMotor"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sVelocityMotorEEEEvv")]
pub fn stub_0x4e7874() -> crate::slot::PortedFn {
// IDA 0x4e7874: void RBX::Name::callDoDeclare<RBX::sVelocityMotor>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4e7874, "void RBX::Name::callDoDeclare<RBX::sVelocityMotor>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sVelocityMotorEEEERKS0_v")]
pub fn stub_0x4e7878(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sVelocityMotor>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_13VelocityMotorENS_13JointInstanceELZNS_14sVelocityMotorEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x4e7958() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"VelocityMotor"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_13VelocityMotorENS_13JointInstanceELZNS_14sVelocityMotorEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x4e7b9c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"VelocityMotor"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_12MotorFeatureENS_7FeatureELZNS_13sMotorFeatureEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x4e7c10() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"MotorFeature"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_12MotorFeatureENS_7FeatureELZNS_13sMotorFeatureEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x4e7cac() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"MotorFeature"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_12MotorFeatureENS_7FeatureELZNS_13sMotorFeatureEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x4e7d34() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"MotorFeature"
}

#[doc(alias = "rbx_core::SharedPtr<RBX::MotorFeature> RBX::Creatable<RBX::Instance>::create<RBX::MotorFeature>(void)")]
pub fn stub_0x4e7e78() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::MotorFeature")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::MotorFeature>::shared_ptr<RBX::MotorFeature,RBX::Creatable<RBX::Instance>::Deleter>(RBX::MotorFeature *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x4e7f28() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::MotorFeature")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::MotorFeature *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::MotorFeature *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x4e80d8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MotorFeature *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x4e81e0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MotorFeature *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x4e81e4]")]
pub fn stub_0x4e81e4(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MotorFeature *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x4e81e8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MotorFeature *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x4e8208() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MotorFeature *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x4e8220() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sMotorFeatureEEEEvv")]
pub fn stub_0x4e8224() -> crate::slot::PortedFn {
// IDA 0x4e8224: void RBX::Name::callDoDeclare<RBX::sMotorFeature>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4e8224, "void RBX::Name::callDoDeclare<RBX::sMotorFeature>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sMotorFeatureEEEERKS0_v")]
pub fn stub_0x4e8228(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sMotorFeature>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_12MotorFeatureENS_7FeatureELZNS_13sMotorFeatureEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x4e8308() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"MotorFeature"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_12MotorFeatureENS_7FeatureELZNS_13sMotorFeatureEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x4e854c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"MotorFeature"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_4HoleENS_7FeatureELZNS_5sHoleEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x4e85c0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Hole"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_4HoleENS_7FeatureELZNS_5sHoleEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x4e865c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Hole"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_4HoleENS_7FeatureELZNS_5sHoleEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x4e86e4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Hole"
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Hole> RBX::Creatable<RBX::Instance>::create<RBX::Hole>(void)")]
pub fn stub_0x4e8828() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Hole")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Hole>::shared_ptr<RBX::Hole,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hole *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x4e88d8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Hole")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Hole *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hole *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x4e8a88() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hole *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x4e8b90(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hole *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x4e8b94]")]
pub fn stub_0x4e8b94(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hole *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x4e8b98() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hole *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x4e8bb8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hole *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x4e8bd0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5sHoleEEEEvv")]
pub fn stub_0x4e8bd4() -> crate::slot::PortedFn {
// IDA 0x4e8bd4: void RBX::Name::callDoDeclare<RBX::sHole>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4e8bd4, "void RBX::Name::callDoDeclare<RBX::sHole>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sHoleEEEERKS0_v")]
pub fn stub_0x4e8bd8(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sHole>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_4HoleENS_7FeatureELZNS_5sHoleEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x4e8cb8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Hole"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_4HoleENS_7FeatureELZNS_5sHoleEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x4e8efc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Hole"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_8sFeatureEEEEvv")]
pub fn stub_0x4e8f70() -> crate::slot::PortedFn {
// IDA 0x4e8f70: void RBX::Name::callDoDeclare<RBX::sFeature>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4e8f70, "void RBX::Name::callDoDeclare<RBX::sFeature>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sFeatureEEEERKS0_v")]
pub fn stub_0x4e8f74(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sFeature>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::VelocityMotor>,boost::_bi::list1<boost::_bi::value<RBX::VelocityMotor*>>>>::~callable_slot()")]
pub fn stub_0x4e9054(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::VelocityMotor>,boost::_bi::list1<boost::_bi::value<RBX::VelocityMotor*>>>>::~callable_slot() [0x4e9080]")]
pub fn stub_0x4e9080(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::VelocityMotor>,boost::_bi::list1<boost::_bi::value<RBX::VelocityMotor*>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x4e9154(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x4e9154: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::VelocityMotor>,boost::_bi::list1<boost::_bi::value<RBX::VelocityMotor*>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x4e916c(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x4e916c: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::VelocityMotor>,boost::_bi::list1<boost::_bi::value<RBX::VelocityMotor*>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
pub fn stub_0x4e9184(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x4e9184: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::VelocityMotor>,boost::_bi::list1<boost::_bi::value<RBX::VelocityMotor*>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~callable() [0x4e91b0]")]
pub fn stub_0x4e91b0(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x4e91b0: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13VelocityMotorELZNS_14sVelocityMotorEENS_14FactoryProductIS2_NS_13JointInstanceELZNS_14sVelocityMotorEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4e9284(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13VelocityMotorELZNS_14sVelocityMotorEENS_14FactoryProductIS2_NS_13JointInstanceELZNS_14sVelocityMotorEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4e9288(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13VelocityMotorELZNS_14sVelocityMotorEENS_14FactoryProductIS2_NS_13JointInstanceELZNS_14sVelocityMotorEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4e9328(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13VelocityMotorELZNS_14sVelocityMotorEENS_14FactoryProductIS2_NS_13JointInstanceELZNS_14sVelocityMotorEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4e9330(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13VelocityMotorELZNS_14sVelocityMotorEENS_14FactoryProductIS2_NS_13JointInstanceELZNS_14sVelocityMotorEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4e93d4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13VelocityMotorELZNS_14sVelocityMotorEENS_14FactoryProductIS2_NS_13JointInstanceELZNS_14sVelocityMotorEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4e93dc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::VelocityMotor>::shared_ptr<RBX::VelocityMotor,RBX::Creatable<RBX::Instance>::Deleter>(RBX::VelocityMotor *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x4e9480() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::VelocityMotor")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::VelocityMotor *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::VelocityMotor *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x4e9630() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::VelocityMotor *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x4e9738(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::VelocityMotor *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x4e973c]")]
pub fn stub_0x4e973c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::VelocityMotor *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x4e9740() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}
