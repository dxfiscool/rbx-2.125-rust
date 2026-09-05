// Auto-generated skeletons for rbx-script — filler EA-sorted ascending after 0x4efbb0 (next 100)
// Filter: Lua|Script|Yield|lua (5401 filtered, all already stubbed globally) — filler EA-sorted ascending
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x4efc6c..0x4f5bfc | existing ~9541 -> ~9641 total (union; filler 0x4efc6c ascending, global remaining 55736 -> 55636)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


#[doc(alias = "non-virtual thunk toRBX::Fire::~Fire() [0x4efc6c]")]
pub fn stub_0x4efc6c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Fire::~Fire() [0x4efc74]")]
pub fn stub_0x4efc74(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::Fire::~Fire() [0x4efc7c]")]
pub fn stub_0x4efc7c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "RBX::Fire::getClampedSize(void)const")]
pub fn stub_0x4efc84(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Fire getter.
cell.get()
}

#[doc(alias = "RBX::Fire::getClampedHeat(void)const")]
pub fn stub_0x4efca4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Fire getter.
cell.get()
}

#[doc(alias = "RBX::Fire::getColor(void)const")]
pub fn stub_0x4efcc4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Fire getter.
cell.get()
}

#[doc(alias = "RBX::Fire::getSecondaryColor(void)const")]
pub fn stub_0x4efcf8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Fire getter.
cell.get()
}

#[doc(alias = "RBX::Fire::getSizeRaw(void)const")]
pub fn stub_0x4efd08(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Fire getter.
cell.get()
}

#[doc(alias = "RBX::Fire::getHeatRaw(void)const")]
pub fn stub_0x4efd30(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Fire getter.
cell.get()
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_4FireENS_8InstanceELZNS_5sFireEES2_E12getClassNameEv")]
pub fn stub_0x4efd78() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Fire"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_4FireENS_8InstanceELZNS_5sFireEES2_E12getClassNameEv")]
pub fn stub_0x4efd88() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Fire"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_4FireENS_8InstanceELZNS_5sFireEES2_E7CreatorD1Ev")]
pub fn stub_0x4efd98() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Fire"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_4FireENS_8InstanceELZNS_5sFireEES2_E7CreatorD2Ev")]
pub fn stub_0x4efd9c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Fire"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_4FireENS_8InstanceELZNS_5sFireEES2_E7Creator12getClassNameEv")]
pub fn stub_0x4efe38() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Fire"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_4FireENS_8InstanceELZNS_5sFireEES2_E7Creator6createEv")]
pub fn stub_0x4efec0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Fire"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5sFireEEEEvv")]
pub fn stub_0x4f03b0() -> crate::slot::PortedFn {
// IDA 0x4f03b0: void RBX::Name::callDoDeclare<RBX::sFire>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4f03b0, "void RBX::Name::callDoDeclare<RBX::sFire>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sFireEEEERKS0_v")]
pub fn stub_0x4f03b4(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sFire>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_4FireENS_8InstanceELZNS_5sFireEES2_E7CreatorC2Ev")]
pub fn stub_0x4f0494() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Fire"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_4FireENS_8InstanceELZNS_5sFireEES2_E17static_getCreatorEv")]
pub fn stub_0x4f06d8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Fire"
}

#[doc(alias = "global constructor keyed to_a_195")]
pub fn stub_0x4f1070() -> crate::slot::PortedFn {
// IDA 0x4f1070: __GLOBAL__I_a_195.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x4f1070, "__GLOBAL__I_a_195")
}

#[doc(alias = "RBX::Flag::getTeamColor(void)const")]
pub fn stub_0x4f15f0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Flag getter.
cell.get()
}

#[doc(alias = "RBX::Flag::setTeamColor(RBX::BrickColor)")]
pub fn stub_0x4f15f8(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Flag setter.
cell.set(value)
}

#[doc(alias = "RBX::Flag::Flag(void)")]
pub fn stub_0x4f1610() -> crate::slot::InstanceHandle {
// RBX::Flag ctor.
crate::slot::InstanceHandle::new("RBX::Flag")
}

#[doc(alias = "RBX::Flag::~Flag()")]
pub fn stub_0x4f192c(handle: crate::slot::InstanceHandle) {
// RBX::Flag dtor.
drop(handle);
}

#[doc(alias = "RBX::Flag::~Flag() [0x4f19cc]")]
pub fn stub_0x4f19cc(handle: crate::slot::InstanceHandle) {
// RBX::Flag dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::Flag::~Flag()")]
pub fn stub_0x4f19d0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Flag::~Flag() [0x4f19d8]")]
pub fn stub_0x4f19d8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::Flag::~Flag() [0x4f19e0]")]
pub fn stub_0x4f19e0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 292, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 292);
}

#[doc(alias = "RBX::Flag::~Flag() [0x4f19e8]")]
pub fn stub_0x4f19e8(handle: crate::slot::InstanceHandle) {
// RBX::Flag dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::Flag::~Flag() [0x4f1b84]")]
pub fn stub_0x4f1b84(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Flag::~Flag() [0x4f1b8c]")]
pub fn stub_0x4f1b8c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::Flag::~Flag() [0x4f1b94]")]
pub fn stub_0x4f1b94(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 292, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 292);
}

#[doc(alias = "RBX::Flag::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_0x4f1b9c(handle: &crate::slot::InstanceHandle) {
// RBX::Flag::onServiceProvider(RBX::ServiceProvider*, RBX::ServiceProvider*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Flag::canBePickedUpByPlayer(RBX::Network::Player *)")]
pub fn stub_0x4f1df8(handle: &crate::slot::InstanceHandle) {
// RBX::Flag::canBePickedUpByPlayer(RBX::Network::Player*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Flag::getJoinedStand(void)")]
pub fn stub_0x4f1e14(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Flag getter.
cell.get()
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_4FlagENS_4ToolELZNS_5sFlagEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x4f202c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Flag"
}

#[doc(alias = "RBX::Flag::canUnequip(void)")]
pub fn stub_0x4f203c(handle: &crate::slot::InstanceHandle) {
// RBX::Flag::canUnequip() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_4FlagENS_4ToolELZNS_5sFlagEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x4f2040() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Flag"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_4FlagENS_4ToolELZNS_5sFlagEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x4f2050() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Flag"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_4FlagENS_4ToolELZNS_5sFlagEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x4f2054() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Flag"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_4FlagENS_4ToolELZNS_5sFlagEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x4f20f0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Flag"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_4FlagENS_4ToolELZNS_5sFlagEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x4f2178() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Flag"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5sFlagEEEEvv")]
pub fn stub_0x4f266c() -> crate::slot::PortedFn {
// IDA 0x4f266c: void RBX::Name::callDoDeclare<RBX::sFlag>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4f266c, "void RBX::Name::callDoDeclare<RBX::sFlag>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sFlagEEEERKS0_v")]
pub fn stub_0x4f2670(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sFlag>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_4FlagENS_4ToolELZNS_5sFlagEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x4f2750() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Flag"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_4FlagENS_4ToolELZNS_5sFlagEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x4f2994() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Flag"
}

#[doc(alias = "global constructor keyed to_a_196")]
pub fn stub_0x4f3080() -> crate::slot::PortedFn {
// IDA 0x4f3080: __GLOBAL__I_a_196.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x4f3080, "__GLOBAL__I_a_196")
}

#[doc(alias = "RBX::FlagStand::getTeamColor(void)const")]
pub fn stub_0x4f33c4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::FlagStand getter.
cell.get()
}

#[doc(alias = "RBX::FlagStand::setTeamColor(RBX::BrickColor)")]
pub fn stub_0x4f33cc(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::FlagStand setter.
cell.set(value)
}

#[doc(alias = "RBX::FlagStand::FlagStand(void)")]
pub fn stub_0x4f33e4() -> crate::slot::InstanceHandle {
// RBX::FlagStand ctor.
crate::slot::InstanceHandle::new("RBX::FlagStand")
}

#[doc(alias = "RBX::FlagStand::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_0x4f3740(handle: &crate::slot::InstanceHandle) {
// RBX::FlagStand::onServiceProvider(RBX::ServiceProvider*, RBX::ServiceProvider*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::FlagStand::onStepped(void)")]
pub fn stub_0x4f3b68(handle: &crate::slot::InstanceHandle) {
// RBX::FlagStand::onStepped() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::FlagStand::getJoinedFlag(void)")]
pub fn stub_0x4f3d50(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::FlagStand getter.
cell.get()
}

#[doc(alias = "RBX::FlagStandService::affixFlagToRandomEmptyStand(RBX::Flag *)")]
pub fn stub_0x4f3e10(handle: &crate::slot::InstanceHandle) {
// RBX::FlagStandService::affixFlagToRandomEmptyStand(RBX::Flag*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::FlagStand::affixFlag(RBX::Flag *)")]
pub fn stub_0x4f3e2c(handle: &crate::slot::InstanceHandle) {
// RBX::FlagStand::affixFlag(RBX::Flag*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::FlagStandService::FlagStandService(void)")]
pub fn stub_0x4f3ef8() -> crate::slot::InstanceHandle {
// RBX::FlagStandService ctor.
crate::slot::InstanceHandle::new("RBX::FlagStandService")
}

#[doc(alias = "RBX::FlagStandService::~FlagStandService()")]
pub fn stub_0x4f418c(handle: crate::slot::InstanceHandle) {
// RBX::FlagStandService dtor.
drop(handle);
}

#[doc(alias = "RBX::FlagStandService::~FlagStandService() [0x4f422c]")]
pub fn stub_0x4f422c(handle: crate::slot::InstanceHandle) {
// RBX::FlagStandService dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::FlagStandService::~FlagStandService()")]
pub fn stub_0x4f4230(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::FlagStandService::~FlagStandService() [0x4f4238]")]
pub fn stub_0x4f4238(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::FlagStandService::~FlagStandService() [0x4f4240]")]
pub fn stub_0x4f4240(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "RBX::FlagStandService::~FlagStandService() [0x4f4248]")]
pub fn stub_0x4f4248(handle: crate::slot::InstanceHandle) {
// RBX::FlagStandService dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::FlagStandService::~FlagStandService() [0x4f439c]")]
pub fn stub_0x4f439c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::FlagStandService::~FlagStandService() [0x4f43a4]")]
pub fn stub_0x4f43a4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::FlagStandService::~FlagStandService() [0x4f43ac]")]
pub fn stub_0x4f43ac(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "RBX::FlagStandService::onStepped(RBX::Stepped const&)")]
pub fn stub_0x4f43b4(handle: &crate::slot::InstanceHandle) {
// RBX::FlagStandService::onStepped(RBX::Stepped const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::FlagStandService::onStepped(RBX::Stepped const&)")]
pub fn stub_0x4f43d0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "RBX::FlagStandService::findRandomEmptyStandForFlag(RBX::Flag *)")]
pub fn stub_0x4f43ec(handle: &crate::slot::InstanceHandle) {
// RBX::FlagStandService::findRandomEmptyStandForFlag(RBX::Flag*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::FlagStandService::FindStandWithFlag(RBX::Flag *)")]
pub fn stub_0x4f4528(handle: &crate::slot::InstanceHandle) {
// RBX::FlagStandService::FindStandWithFlag(RBX::Flag*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Flag>::operator=(rbx_core::SharedPtr<RBX::Flag> const&)")]
pub fn stub_0x4f4934(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Flag> RBX::shared_from<RBX::Flag>(RBX::Flag*)")]
pub fn stub_0x4f496c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Flag")
}

#[doc(alias = "std::list<RBX::FlagStand *,std::allocator<RBX::FlagStand *>>::remove(RBX::FlagStand * const&)")]
pub fn stub_0x4f4af4(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// IndexArray::fastRemove — swap-with-last.
vec.fast_remove(index)
}

#[doc(alias = "std::vector<RBX::FlagStand *,std::allocator<RBX::FlagStand *>>::push_back(RBX::FlagStand * const&)")]
pub fn stub_0x4f4b2c(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "RBX::FlagStandService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_0x4f4b58(handle: &crate::slot::InstanceHandle) {
// RBX::FlagStandService::onServiceProvider(RBX::ServiceProvider*, RBX::ServiceProvider*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_17sFlagStandServiceEEE12getClassNameEv")]
pub fn stub_0x4f4b60() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_17sFlagStandServiceEEE12getClassNameEv")]
pub fn stub_0x4f4b88() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "RBX::FlagStand::~FlagStand()")]
pub fn stub_0x4f4bb0(handle: crate::slot::InstanceHandle) {
// RBX::FlagStand dtor.
drop(handle);
}

#[doc(alias = "RBX::FlagStand::~FlagStand() [0x4f4bc0]")]
pub fn stub_0x4f4bc0(handle: crate::slot::InstanceHandle) {
// RBX::FlagStand dtor.
drop(handle);
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x4f4c6c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"FlagStand"
}

#[doc(alias = "non-virtual thunk toRBX::FlagStand::~FlagStand()")]
pub fn stub_0x4f4c7c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::FlagStand::~FlagStand() [0x4f4c8c]")]
pub fn stub_0x4f4c8c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x4f4d38() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"FlagStand"
}

#[doc(alias = "non-virtual thunk toRBX::FlagStand::~FlagStand() [0x4f4d48]")]
pub fn stub_0x4f4d48(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::FlagStand::~FlagStand() [0x4f4d58]")]
pub fn stub_0x4f4d58(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::FlagStand::~FlagStand() [0x4f4e04]")]
pub fn stub_0x4f4e04(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 132, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 132);
}

#[doc(alias = "non-virtual thunk toRBX::FlagStand::~FlagStand() [0x4f4e14]")]
pub fn stub_0x4f4e14(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 132, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 132);
}

#[doc(alias = "__ZThn132_N3RBX10Reflection9DescribedINS_9FlagStandELZNS_10sFlagStandEENS_14FactoryProductIS2_NS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4f5110(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 132, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 132);
}

#[doc(alias = "__ZThn132_N3RBX10Reflection9DescribedINS_9FlagStandELZNS_10sFlagStandEENS_14FactoryProductIS2_NS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4f5124(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 132, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 132);
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEED1Ev")]
pub fn stub_0x4f51d8(handle: crate::slot::InstanceHandle) {
// RBX::FactoryProduct dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEED0Ev")]
pub fn stub_0x4f51ec(handle: crate::slot::InstanceHandle) {
// RBX::FactoryProduct dtor.
drop(handle);
}

#[doc(alias = "__ZThn132_N3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEED1Ev")]
pub fn stub_0x4f529c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 132, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 132);
}

#[doc(alias = "__ZThn132_N3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEED0Ev")]
pub fn stub_0x4f52b0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 132, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 132);
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x4f52b8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"FlagStand"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x4f52bc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"FlagStand"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x4f5358() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"FlagStand"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x4f53e0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"FlagStand"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sFlagStandEEEEvv")]
pub fn stub_0x4f58d4() -> crate::slot::PortedFn {
// IDA 0x4f58d4: void RBX::Name::callDoDeclare<RBX::sFlagStand>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4f58d4, "void RBX::Name::callDoDeclare<RBX::sFlagStand>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sFlagStandEEEERKS0_v")]
pub fn stub_0x4f58d8(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sFlagStand>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x4f59b8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"FlagStand"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x4f5bfc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"FlagStand"
}
