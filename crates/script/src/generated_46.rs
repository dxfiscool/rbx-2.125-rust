// Auto-generated skeletons for rbx-script — filler EA-sorted ascending earliest gap (next 100)
// Filter: Lua|Script|Yield|lua (5401 filtered, all already stubbed globally) — filler EA-sorted ascending
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x49ff08..0x4ae418 | existing ~9491 -> ~9591 total (union; filler 0x49ff08 ascending, global remaining 35069 -> 34969)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "RBX::Explosion::signalBlast(std::vector<rbx_core::SharedPtr<RBX::PartInstance>,std::allocator<rbx_core::SharedPtr<RBX::PartInstance>>> const&)")]
pub fn stub_0x49ff08() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PartInstance")
}

#[doc(alias = "void RBX::Explosion::doBlast<RBX::MegaClusterInstance>(RBX::MegaClusterInstance *,std::vector<rbx_core::SharedPtr<RBX::PartInstance>,std::allocator<rbx_core::SharedPtr<RBX::PartInstance>>> const&)")]
pub fn stub_0x4a0a30() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PartInstance")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Explosion> RBX::shared_from<RBX::Explosion>(RBX::Explosion*)")]
pub fn stub_0x4a11c4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Explosion")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E12getClassNameEv")]
pub fn stub_0x4a1340() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Explosion"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E12getClassNameEv")]
pub fn stub_0x4a1358() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Explosion"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_12TimerServiceENS_8InstanceELZNS_13sTimerServiceEES2_E7CreatorD1Ev")]
pub fn stub_0x4a1378() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"TimerService"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7CreatorD1Ev")]
pub fn stub_0x4a137c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Explosion"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E7Creator12getClassNameEv")]
pub fn stub_0x4a1788() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ForceField"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sForceFieldEEEEvv")]
pub fn stub_0x4a18d0() -> crate::slot::PortedFn {
// IDA 0x4a18d0: void RBX::Name::callDoDeclare<RBX::sForceField>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4a18d0, "void RBX::Name::callDoDeclare<RBX::sForceField>()")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E7CreatorC2Ev")]
pub fn stub_0x4a18d8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ForceField"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7CreatorD2Ev")]
pub fn stub_0x4a1b04() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Explosion"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7Creator12getClassNameEv")]
pub fn stub_0x4a1ba0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Explosion"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7Creator6createEv")]
pub fn stub_0x4a1c28() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Explosion"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sExplosionEEEEvv")]
pub fn stub_0x4a1d78() -> crate::slot::PortedFn {
// IDA 0x4a1d78: void RBX::Name::callDoDeclare<RBX::sExplosion>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4a1d78, "void RBX::Name::callDoDeclare<RBX::sExplosion>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sExplosionEEEERKS0_v")]
pub fn stub_0x4a1d7c(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sExplosion>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7CreatorC2Ev")]
pub fn stub_0x4a1e5c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Explosion"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E17static_getCreatorEv")]
pub fn stub_0x4a20a0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Explosion"
}

#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPS8_EENS3_5list2INS3_5valueINS_10shared_ptrINS7_9ExplosionEEEEENSC_IS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x4a2114() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_12TimerServiceENS_8InstanceELZNS_13sTimerServiceEES2_E7Creator12getClassNameEv")]
pub fn stub_0x4a2900() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"TimerService"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_12TimerServiceENS_8InstanceELZNS_13sTimerServiceEES2_E15isNullClassNameEv")]
pub fn stub_0x4a2bd0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::FactoryProduct getter.
cell.get()
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::PartInstance>,std::allocator<rbx_core::SharedPtr<RBX::PartInstance>>>::~vector()")]
pub fn stub_0x4a2c38(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "global constructor keyed to_a_188")]
pub fn stub_0x4a6898() -> crate::slot::PortedFn {
// IDA 0x4a6898: __GLOBAL__I_a_188.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x4a6898, "__GLOBAL__I_a_188")
}

#[doc(alias = "RBX::ExtrudedPartInstance::setVisualTrussStyle(RBX::ExtrudedPartInstance::VisualTrussStyle)")]
pub fn stub_0x4a6e24(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::ExtrudedPartInstance setter.
cell.set(value)
}

#[doc(alias = "RBX::ExtrudedPartInstance::ExtrudedPartInstance(void)")]
pub fn stub_0x4a6e80() -> crate::slot::InstanceHandle {
// RBX::ExtrudedPartInstance ctor.
crate::slot::InstanceHandle::new("RBX::ExtrudedPartInstance")
}

#[doc(alias = "RBX::ExtrudedPartInstance::~ExtrudedPartInstance()")]
pub fn stub_0x4a7184(handle: crate::slot::InstanceHandle) {
// RBX::ExtrudedPartInstance dtor.
drop(handle);
}

#[doc(alias = "RBX::ExtrudedPartInstance::~ExtrudedPartInstance() [0x4a7234]")]
pub fn stub_0x4a7234(handle: crate::slot::InstanceHandle) {
// RBX::ExtrudedPartInstance dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::ExtrudedPartInstance::~ExtrudedPartInstance()")]
pub fn stub_0x4a7244(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::ExtrudedPartInstance::~ExtrudedPartInstance() [0x4a724c]")]
pub fn stub_0x4a724c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::ExtrudedPartInstance::~ExtrudedPartInstance() [0x4a7254]")]
pub fn stub_0x4a7254(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 132, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 132);
}

#[doc(alias = "non-virtual thunk toRBX::ExtrudedPartInstance::~ExtrudedPartInstance() [0x4a725c]")]
pub fn stub_0x4a725c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::ExtrudedPartInstance::~ExtrudedPartInstance() [0x4a7270]")]
pub fn stub_0x4a7270(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::ExtrudedPartInstance::~ExtrudedPartInstance() [0x4a7284]")]
pub fn stub_0x4a7284(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 132, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 132);
}

#[doc(alias = "RBX::ExtrudedPartInstance::getMinimumUiSize(void)const")]
pub fn stub_0x4a7298(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::ExtrudedPartInstance getter.
cell.get()
}

#[doc(alias = "RBX::ExtrudedPartInstance::getResizeIncrement(void)const")]
pub fn stub_0x4a7524(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::ExtrudedPartInstance getter.
cell.get()
}

#[doc(alias = "RBX::ExtrudedPartInstance::getResizeHandleMask(void)const")]
pub fn stub_0x4a7528(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::ExtrudedPartInstance getter.
cell.get()
}

#[doc(alias = "RBX::ExtrudedPartInstance::getVisualTrussStyle(void)const")]
pub fn stub_0x4a772c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::ExtrudedPartInstance getter.
cell.get()
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x4a7758() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ExtrudedPartInstance"
}

#[doc(alias = "RBX::ExtrudedPartInstance::getPartType(void)const")]
pub fn stub_0x4a7768(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::ExtrudedPartInstance getter.
cell.get()
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x4a776c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ExtrudedPartInstance"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED1Ev")]
pub fn stub_0x4a7a94(handle: crate::slot::InstanceHandle) {
// RBX::FactoryProduct dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED0Ev")]
pub fn stub_0x4a7aa8(handle: crate::slot::InstanceHandle) {
// RBX::FactoryProduct dtor.
drop(handle);
}

#[doc(alias = "__ZThn132_N3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED1Ev")]
pub fn stub_0x4a7b58(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 132, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 132);
}

#[doc(alias = "__ZThn132_N3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED0Ev")]
pub fn stub_0x4a7b6c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 132, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 132);
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x4a7b74() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ExtrudedPartInstance"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x4a7b78() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ExtrudedPartInstance"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x4a7c14() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ExtrudedPartInstance"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x4a7c9c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ExtrudedPartInstance"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sExtrudedPartEEEEvv")]
pub fn stub_0x4a8190() -> crate::slot::PortedFn {
// IDA 0x4a8190: void RBX::Name::callDoDeclare<RBX::sExtrudedPart>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4a8190, "void RBX::Name::callDoDeclare<RBX::sExtrudedPart>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sExtrudedPartEEEERKS0_v")]
pub fn stub_0x4a8194(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sExtrudedPart>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x4a8274() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ExtrudedPartInstance"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x4a84b8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ExtrudedPartInstance"
}

#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED1Ev")]
pub fn stub_0x4a852c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED1Ev")]
pub fn stub_0x4a8540(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED0Ev")]
pub fn stub_0x4a8554(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED0Ev")]
pub fn stub_0x4a855c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "global constructor keyed to_a_189")]
pub fn stub_0x4a9168() -> crate::slot::PortedFn {
// IDA 0x4a9168: __GLOBAL__I_a_189.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x4a9168, "__GLOBAL__I_a_189")
}

#[doc(alias = "RBX::FaceInstance::setFace(RBX::NormalId)")]
pub fn stub_0x4a94fc(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::FaceInstance setter.
cell.set(value)
}

#[doc(alias = "RBX::FaceInstance::FaceInstance(void)")]
pub fn stub_0x4a9518() -> crate::slot::InstanceHandle {
// RBX::FaceInstance ctor.
crate::slot::InstanceHandle::new("RBX::FaceInstance")
}

#[doc(alias = "RBX::FaceInstance::getFace(void)const")]
pub fn stub_0x4a9724(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::FaceInstance getter.
cell.get()
}

#[doc(alias = "RBX::FaceInstance::~FaceInstance()")]
pub fn stub_0x4a974c(handle: crate::slot::InstanceHandle) {
// RBX::FaceInstance dtor.
drop(handle);
}

#[doc(alias = "RBX::FaceInstance::~FaceInstance() [0x4a9808]")]
pub fn stub_0x4a9808(handle: crate::slot::InstanceHandle) {
// RBX::FaceInstance dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::FaceInstance::~FaceInstance()")]
pub fn stub_0x4a98d4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::FaceInstance::~FaceInstance() [0x4a998c]")]
pub fn stub_0x4a998c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::FaceInstance::~FaceInstance() [0x4a9a5c]")]
pub fn stub_0x4a9a5c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::FaceInstance::~FaceInstance() [0x4a9b14]")]
pub fn stub_0x4a9b14(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "global constructor keyed to_a_190")]
pub fn stub_0x4aa5e8() -> crate::slot::PortedFn {
// IDA 0x4aa5e8: __GLOBAL__I_a_190.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x4aa5e8, "__GLOBAL__I_a_190")
}

#[doc(alias = "__ZN5boost8functionIFvRSt9exceptionEEaSIPS3_EENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS4_E4typeES8_")]
pub fn stub_0x4ab510(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_19CustomEventReceiverENS_8InstanceELZNS_20sCustomEventReceiverEES2_E7CreatorD1Ev")]
pub fn stub_0x4ab5ec() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"CustomEventReceiver"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_11CustomEventENS_8InstanceELZNS_12sCustomEventEES2_E7CreatorD1Ev")]
pub fn stub_0x4ab5f0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"CustomEvent"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_17BasicPartInstanceENS_14FormFactorPartELZNS_10sBasicPartEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x4ab5f4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BasicPartInstance"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_8SparklesENS_8InstanceELZNS_9sSparklesEES2_E7CreatorD1Ev")]
pub fn stub_0x4ab5f8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Sparkles"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9AnimationENS_8InstanceELZNS_10sAnimationEES2_E7CreatorD1Ev")]
pub fn stub_0x4ab5fc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Animation"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_16BindableFunctionENS_8InstanceELZNS_17sBindableFunctionEES2_E7CreatorD1Ev")]
pub fn stub_0x4ab600() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BindableFunction"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_13BindableEventENS_8InstanceELZNS_14sBindableEventEES2_E7CreatorD1Ev")]
pub fn stub_0x4ab604() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BindableEvent"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_13BindableEventENS_8InstanceELZNS_14sBindableEventEES2_E7CreatorD2Ev")]
pub fn stub_0x4ab608() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BindableEvent"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_13BindableEventENS_8InstanceELZNS_14sBindableEventEES2_E7Creator12getClassNameEv")]
pub fn stub_0x4ab6a4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BindableEvent"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_13BindableEventENS_8InstanceELZNS_14sBindableEventEES2_E7Creator6createEv")]
pub fn stub_0x4ab710() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BindableEvent"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_13BindableEventENS_8InstanceELZNS_14sBindableEventEES2_E12getClassNameEv")]
pub fn stub_0x4aba94() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BindableEvent"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_13BindableEventENS_8InstanceELZNS_14sBindableEventEES2_E12getClassNameEv")]
pub fn stub_0x4abaa4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BindableEvent"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_13BindableEventENS_8InstanceELZNS_14sBindableEventEES2_E17static_getCreatorEv")]
pub fn stub_0x4abab4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BindableEvent"
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sBindableEventEEEERKS0_v")]
pub fn stub_0x4ac23c(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sBindableEvent>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sBindableEventEEEEvv")]
pub fn stub_0x4ac280() -> crate::slot::PortedFn {
// IDA 0x4ac280: void RBX::Name::callDoDeclare<RBX::sBindableEvent>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4ac280, "void RBX::Name::callDoDeclare<RBX::sBindableEvent>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sBindableEventEEEERKS0_v")]
pub fn stub_0x4ac284(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sBindableEvent>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_13BindableEventENS_8InstanceELZNS_14sBindableEventEES2_E7CreatorC2Ev")]
pub fn stub_0x4ac368() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BindableEvent"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_16BindableFunctionENS_8InstanceELZNS_17sBindableFunctionEES2_E7CreatorD2Ev")]
pub fn stub_0x4ac590() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BindableFunction"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_16BindableFunctionENS_8InstanceELZNS_17sBindableFunctionEES2_E7Creator12getClassNameEv")]
pub fn stub_0x4ac62c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BindableFunction"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_16BindableFunctionENS_8InstanceELZNS_17sBindableFunctionEES2_E7Creator6createEv")]
pub fn stub_0x4ac698() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BindableFunction"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_16BindableFunctionENS_8InstanceELZNS_17sBindableFunctionEES2_E12getClassNameEv")]
pub fn stub_0x4aca38() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BindableFunction"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_16BindableFunctionENS_8InstanceELZNS_17sBindableFunctionEES2_E12getClassNameEv")]
pub fn stub_0x4aca48() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BindableFunction"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_16BindableFunctionENS_8InstanceELZNS_17sBindableFunctionEES2_E17static_getCreatorEv")]
pub fn stub_0x4aca58() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BindableFunction"
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_17sBindableFunctionEEEERKS0_v")]
pub fn stub_0x4ada84(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sBindableFunction>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_17sBindableFunctionEEEEvv")]
pub fn stub_0x4adac8() -> crate::slot::PortedFn {
// IDA 0x4adac8: void RBX::Name::callDoDeclare<RBX::sBindableFunction>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4adac8, "void RBX::Name::callDoDeclare<RBX::sBindableFunction>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_17sBindableFunctionEEEERKS0_v")]
pub fn stub_0x4adacc(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sBindableFunction>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_16BindableFunctionENS_8InstanceELZNS_17sBindableFunctionEES2_E7CreatorC2Ev")]
pub fn stub_0x4adbb0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BindableFunction"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9AnimationENS_8InstanceELZNS_10sAnimationEES2_E7CreatorD2Ev")]
pub fn stub_0x4addd8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Animation"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_9AnimationENS_8InstanceELZNS_10sAnimationEES2_E7Creator12getClassNameEv")]
pub fn stub_0x4ade74() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Animation"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_9AnimationENS_8InstanceELZNS_10sAnimationEES2_E7Creator6createEv")]
pub fn stub_0x4adee0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Animation"
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sAnimationEEEERKS0_v")]
pub fn stub_0x4ae3d0(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sAnimation>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sAnimationEEEEvv")]
pub fn stub_0x4ae414() -> crate::slot::PortedFn {
// IDA 0x4ae414: void RBX::Name::callDoDeclare<RBX::sAnimation>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4ae414, "void RBX::Name::callDoDeclare<RBX::sAnimation>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sAnimationEEEERKS0_v")]
pub fn stub_0x4ae418(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sAnimation>() — engine-side; linkage preserved via the alias.
let _ = handle;
}
