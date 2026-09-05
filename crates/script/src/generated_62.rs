// Auto-generated skeletons for rbx-script — filler EA-sorted ascending after 0x62ece0 (next 100)
// Filter: Lua|Script|Yield|lua (5401 filtered, all already stubbed globally) — filler EA-sorted ascending
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x644d30..0x663360 | existing ~11091 -> ~11191 total (union; filler 0x644d30 ascending, global remaining)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "global constructor keyed to_a_262")]
pub fn stub_0x644d30() -> crate::slot::PortedFn {
// IDA 0x644d30: __GLOBAL__I_a_262.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x644d30, "__GLOBAL__I_a_262")
}

#[doc(alias = "ProfilingItem::getTimes(double)")]
pub fn stub_0x649fc8() -> crate::slot::PortedFn {
// IDA 0x649fc8: ProfilingItem::getTimes(double).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x649fc8, "ProfilingItem::getTimes(double)")
}

#[doc(alias = "ProfilingItem::getTimesForFrames(int)")]
pub fn stub_0x64a180() -> crate::slot::PortedFn {
// IDA 0x64a180: ProfilingItem::getTimesForFrames(int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x64a180, "ProfilingItem::getTimesForFrames(int)")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EE15classDescriptorEv")]
pub fn stub_0x64baa0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::Stats::StatsService, RBX::Stats::sStats, RBX::NonFactoryPr~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5Stats10sStatsItemEEEEvv")]
pub fn stub_0x64bed8() -> crate::slot::PortedFn {
// IDA 0x64bed8: void RBX::Name::callDoDeclare<RBX::Stats::sStatsItem>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x64bed8, "void RBX::Name::callDoDeclare<RBX::Stats::sStatsItem>()")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedI26TotalCountTimeIntervalItemLZ27sTotalCountTimeIntervalItemENS_17NonFactoryProductINS_5Stats4ItemELZ27sTotalCountTimeIntervalItemEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x64bedc(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<TotalCountTimeIntervalItem, sTotalCountTimeIntervalItem, RBX::N~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedI13ProfilingItemLZ14sProfilingItemENS_17NonFactoryProductINS_5Stats4ItemELZ14sProfilingItemEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x64bffc(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<ProfilingItem, sProfilingItem, RBX::NonFactoryProduct<RBX::Stat~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedI24RunningAverageItemDoubleLZ25sRunningAverageItemDoubleENS_17NonFactoryProductI18RunningAverageItemIdELZ25sRunningAverageItemDoubleEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x64c118(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RunningAverageItemDouble, sRunningAverageItemDouble, RBX::NonFa~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedI21RunningAverageItemIntLZ22sRunningAverageItemIntENS_17NonFactoryProductI18RunningAverageItemIiELZ22sRunningAverageItemIntEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x64c234(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RunningAverageItemInt, sRunningAverageItemInt, RBX::NonFactoryP~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "ProfilingItem::~ProfilingItem()")]
pub fn stub_0x64e7b4() -> crate::slot::PortedFn {
// IDA 0x64e7b4: ProfilingItem::~ProfilingItem().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x64e7b4, "ProfilingItem::~ProfilingItem()")
}

#[doc(alias = "ProfilingItem::~ProfilingItem() [0x64e7f0]")]
pub fn stub_0x64e7f0() -> crate::slot::PortedFn {
// IDA 0x64e7f0: ProfilingItem::~ProfilingItem().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x64e7f0, "ProfilingItem::~ProfilingItem()")
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_5Stats4ItemELZ14sProfilingItemEE12getClassNameEv")]
pub fn stub_0x64e8c0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Stats::Item"
}

#[doc(alias = "ProfilingItem::update(void)")]
pub fn stub_0x64e8e8() -> crate::slot::PortedFn {
// IDA 0x64e8e8: ProfilingItem::update().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x64e8e8, "ProfilingItem::update()")
}

#[doc(alias = "non-virtual thunk toProfilingItem::~ProfilingItem()")]
pub fn stub_0x64eb20(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toProfilingItem::~ProfilingItem() [0x64eb5c]")]
pub fn stub_0x64eb5c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_5Stats4ItemELZ14sProfilingItemEE12getClassNameEv")]
pub fn stub_0x64ec30() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Stats::Item"
}

#[doc(alias = "non-virtual thunk toProfilingItem::~ProfilingItem() [0x64ec58]")]
pub fn stub_0x64ec58(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toProfilingItem::~ProfilingItem() [0x64ec94]")]
pub fn stub_0x64ec94(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZ14sProfilingItemEEEvv")]
pub fn stub_0x64ed68() -> crate::slot::PortedFn {
// IDA 0x64ed68: void RBX::Name::callDoDeclare<sProfilingItem>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x64ed68, "void RBX::Name::callDoDeclare<sProfilingItem>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZ14sProfilingItemEEERKS0_v")]
pub fn stub_0x64ed6c(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<sProfilingItem>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedI13ProfilingItemLZ14sProfilingItemENS_17NonFactoryProductINS_5Stats4ItemELZ14sProfilingItemEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x64ee4c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedI13ProfilingItemLZ14sProfilingItemENS_17NonFactoryProductINS_5Stats4ItemELZ14sProfilingItemEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x64ee88(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedI13ProfilingItemLZ14sProfilingItemENS_17NonFactoryProductINS_5Stats4ItemELZ14sProfilingItemEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x64ef58(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedI13ProfilingItemLZ14sProfilingItemENS_17NonFactoryProductINS_5Stats4ItemELZ14sProfilingItemEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x64ef94(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedI13ProfilingItemLZ14sProfilingItemENS_17NonFactoryProductINS_5Stats4ItemELZ14sProfilingItemEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x64f068(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedI13ProfilingItemLZ14sProfilingItemENS_17NonFactoryProductINS_5Stats4ItemELZ14sProfilingItemEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x64f0a4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RunningAverageItemDouble::~RunningAverageItemDouble()")]
pub fn stub_0x64f474() -> crate::slot::PortedFn {
// IDA 0x64f474: RunningAverageItemDouble::~RunningAverageItemDouble().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x64f474, "RunningAverageItemDouble::~RunningAverageItemDouble()")
}

#[doc(alias = "RunningAverageItemDouble::~RunningAverageItemDouble() [0x64f4b0]")]
pub fn stub_0x64f4b0() -> crate::slot::PortedFn {
// IDA 0x64f4b0: RunningAverageItemDouble::~RunningAverageItemDouble().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x64f4b0, "RunningAverageItemDouble::~RunningAverageItemDouble()")
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductI18RunningAverageItemIdELZ25sRunningAverageItemDoubleEE12getClassNameEv")]
pub fn stub_0x64f580() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"RunningAverageItem<double>"
}

#[doc(alias = "RunningAverageItem<double>::update(void)")]
pub fn stub_0x64f5a8() -> crate::slot::PortedFn {
// IDA 0x64f5a8: RunningAverageItem<double>::update().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x64f5a8, "RunningAverageItem<double>::update()")
}

#[doc(alias = "non-virtual thunk toRunningAverageItemDouble::~RunningAverageItemDouble()")]
pub fn stub_0x64f718(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRunningAverageItemDouble::~RunningAverageItemDouble() [0x64f754]")]
pub fn stub_0x64f754(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductI18RunningAverageItemIdELZ25sRunningAverageItemDoubleEE12getClassNameEv")]
pub fn stub_0x64f828() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"RunningAverageItem<double>"
}

#[doc(alias = "non-virtual thunk toRunningAverageItemDouble::~RunningAverageItemDouble() [0x64f850]")]
pub fn stub_0x64f850(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRunningAverageItemDouble::~RunningAverageItemDouble() [0x64f88c]")]
pub fn stub_0x64f88c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZ25sRunningAverageItemDoubleEEEvv")]
pub fn stub_0x64f960() -> crate::slot::PortedFn {
// IDA 0x64f960: void RBX::Name::callDoDeclare<sRunningAverageItemDouble>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x64f960, "void RBX::Name::callDoDeclare<sRunningAverageItemDouble>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZ25sRunningAverageItemDoubleEEERKS0_v")]
pub fn stub_0x64f964(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<sRunningAverageItemDouble>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedI24RunningAverageItemDoubleLZ25sRunningAverageItemDoubleENS_17NonFactoryProductI18RunningAverageItemIdELZ25sRunningAverageItemDoubleEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x64fa44(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedI24RunningAverageItemDoubleLZ25sRunningAverageItemDoubleENS_17NonFactoryProductI18RunningAverageItemIdELZ25sRunningAverageItemDoubleEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x64fa80(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedI24RunningAverageItemDoubleLZ25sRunningAverageItemDoubleENS_17NonFactoryProductI18RunningAverageItemIdELZ25sRunningAverageItemDoubleEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x64fb50(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedI24RunningAverageItemDoubleLZ25sRunningAverageItemDoubleENS_17NonFactoryProductI18RunningAverageItemIdELZ25sRunningAverageItemDoubleEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x64fb8c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedI24RunningAverageItemDoubleLZ25sRunningAverageItemDoubleENS_17NonFactoryProductI18RunningAverageItemIdELZ25sRunningAverageItemDoubleEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x64fc60(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedI24RunningAverageItemDoubleLZ25sRunningAverageItemDoubleENS_17NonFactoryProductI18RunningAverageItemIdELZ25sRunningAverageItemDoubleEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x64fc9c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RunningAverageItemInt::~RunningAverageItemInt()")]
pub fn stub_0x65006c() -> crate::slot::PortedFn {
// IDA 0x65006c: RunningAverageItemInt::~RunningAverageItemInt().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x65006c, "RunningAverageItemInt::~RunningAverageItemInt()")
}

#[doc(alias = "RunningAverageItemInt::~RunningAverageItemInt() [0x6500a8]")]
pub fn stub_0x6500a8() -> crate::slot::PortedFn {
// IDA 0x6500a8: RunningAverageItemInt::~RunningAverageItemInt().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x6500a8, "RunningAverageItemInt::~RunningAverageItemInt()")
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductI18RunningAverageItemIiELZ22sRunningAverageItemIntEE12getClassNameEv")]
pub fn stub_0x650178() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"RunningAverageItem<int>"
}

#[doc(alias = "RunningAverageItem<int>::update(void)")]
pub fn stub_0x6501a0() -> crate::slot::PortedFn {
// IDA 0x6501a0: RunningAverageItem<int>::update().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x6501a0, "RunningAverageItem<int>::update()")
}

#[doc(alias = "non-virtual thunk toRunningAverageItemInt::~RunningAverageItemInt()")]
pub fn stub_0x650310(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRunningAverageItemInt::~RunningAverageItemInt() [0x65034c]")]
pub fn stub_0x65034c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductI18RunningAverageItemIiELZ22sRunningAverageItemIntEE12getClassNameEv")]
pub fn stub_0x650420() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"RunningAverageItem<int>"
}

#[doc(alias = "non-virtual thunk toRunningAverageItemInt::~RunningAverageItemInt() [0x650448]")]
pub fn stub_0x650448(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRunningAverageItemInt::~RunningAverageItemInt() [0x650484]")]
pub fn stub_0x650484(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZ22sRunningAverageItemIntEEEvv")]
pub fn stub_0x650558() -> crate::slot::PortedFn {
// IDA 0x650558: void RBX::Name::callDoDeclare<sRunningAverageItemInt>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x650558, "void RBX::Name::callDoDeclare<sRunningAverageItemInt>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZ22sRunningAverageItemIntEEERKS0_v")]
pub fn stub_0x65055c(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<sRunningAverageItemInt>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedI21RunningAverageItemIntLZ22sRunningAverageItemIntENS_17NonFactoryProductI18RunningAverageItemIiELZ22sRunningAverageItemIntEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x65063c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedI21RunningAverageItemIntLZ22sRunningAverageItemIntENS_17NonFactoryProductI18RunningAverageItemIiELZ22sRunningAverageItemIntEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x650678(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedI21RunningAverageItemIntLZ22sRunningAverageItemIntENS_17NonFactoryProductI18RunningAverageItemIiELZ22sRunningAverageItemIntEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x650748(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedI21RunningAverageItemIntLZ22sRunningAverageItemIntENS_17NonFactoryProductI18RunningAverageItemIiELZ22sRunningAverageItemIntEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x650784(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedI21RunningAverageItemIntLZ22sRunningAverageItemIntENS_17NonFactoryProductI18RunningAverageItemIiELZ22sRunningAverageItemIntEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x650858(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedI21RunningAverageItemIntLZ22sRunningAverageItemIntENS_17NonFactoryProductI18RunningAverageItemIiELZ22sRunningAverageItemIntEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x650894(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "TotalCountTimeIntervalItem::~TotalCountTimeIntervalItem()")]
pub fn stub_0x650c64() -> crate::slot::PortedFn {
// IDA 0x650c64: TotalCountTimeIntervalItem::~TotalCountTimeIntervalItem().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x650c64, "TotalCountTimeIntervalItem::~TotalCountTimeIntervalItem()")
}

#[doc(alias = "TotalCountTimeIntervalItem::~TotalCountTimeIntervalItem() [0x650ca0]")]
pub fn stub_0x650ca0() -> crate::slot::PortedFn {
// IDA 0x650ca0: TotalCountTimeIntervalItem::~TotalCountTimeIntervalItem().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x650ca0, "TotalCountTimeIntervalItem::~TotalCountTimeIntervalItem()")
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_5Stats4ItemELZ27sTotalCountTimeIntervalItemEE12getClassNameEv")]
pub fn stub_0x650d70() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Stats::Item"
}

#[doc(alias = "TotalCountTimeIntervalItem::update(void)")]
pub fn stub_0x650d98() -> crate::slot::PortedFn {
// IDA 0x650d98: TotalCountTimeIntervalItem::update().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x650d98, "TotalCountTimeIntervalItem::update()")
}

#[doc(alias = "non-virtual thunk toTotalCountTimeIntervalItem::~TotalCountTimeIntervalItem()")]
pub fn stub_0x650ee8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toTotalCountTimeIntervalItem::~TotalCountTimeIntervalItem() [0x650f24]")]
pub fn stub_0x650f24(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_5Stats4ItemELZ27sTotalCountTimeIntervalItemEE12getClassNameEv")]
pub fn stub_0x650ff8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Stats::Item"
}

#[doc(alias = "non-virtual thunk toTotalCountTimeIntervalItem::~TotalCountTimeIntervalItem() [0x651020]")]
pub fn stub_0x651020(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toTotalCountTimeIntervalItem::~TotalCountTimeIntervalItem() [0x65105c]")]
pub fn stub_0x65105c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZ27sTotalCountTimeIntervalItemEEEvv")]
pub fn stub_0x651130() -> crate::slot::PortedFn {
// IDA 0x651130: void RBX::Name::callDoDeclare<sTotalCountTimeIntervalItem>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x651130, "void RBX::Name::callDoDeclare<sTotalCountTimeIntervalItem>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZ27sTotalCountTimeIntervalItemEEERKS0_v")]
pub fn stub_0x651134(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<sTotalCountTimeIntervalItem>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedI26TotalCountTimeIntervalItemLZ27sTotalCountTimeIntervalItemENS_17NonFactoryProductINS_5Stats4ItemELZ27sTotalCountTimeIntervalItemEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x651248(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedI26TotalCountTimeIntervalItemLZ27sTotalCountTimeIntervalItemENS_17NonFactoryProductINS_5Stats4ItemELZ27sTotalCountTimeIntervalItemEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x651284(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedI26TotalCountTimeIntervalItemLZ27sTotalCountTimeIntervalItemENS_17NonFactoryProductINS_5Stats4ItemELZ27sTotalCountTimeIntervalItemEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x651354(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedI26TotalCountTimeIntervalItemLZ27sTotalCountTimeIntervalItemENS_17NonFactoryProductINS_5Stats4ItemELZ27sTotalCountTimeIntervalItemEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x651390(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedI26TotalCountTimeIntervalItemLZ27sTotalCountTimeIntervalItemENS_17NonFactoryProductINS_5Stats4ItemELZ27sTotalCountTimeIntervalItemEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x651464(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedI26TotalCountTimeIntervalItemLZ27sTotalCountTimeIntervalItemENS_17NonFactoryProductINS_5Stats4ItemELZ27sTotalCountTimeIntervalItemEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x6514a0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "global constructor keyed to_a_263")]
pub fn stub_0x65740c() -> crate::slot::PortedFn {
// IDA 0x65740c: __GLOBAL__I_a_263.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x65740c, "__GLOBAL__I_a_263")
}

#[doc(alias = "global constructor keyed to_a_264")]
pub fn stub_0x6583a0() -> crate::slot::PortedFn {
// IDA 0x6583a0: __GLOBAL__I_a_264.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x6583a0, "__GLOBAL__I_a_264")
}

#[doc(alias = "global constructor keyed to_a_265")]
pub fn stub_0x658744() -> crate::slot::PortedFn {
// IDA 0x658744: __GLOBAL__I_a_265.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x658744, "__GLOBAL__I_a_265")
}

#[doc(alias = "global constructor keyed to_a_266")]
pub fn stub_0x65ff94() -> crate::slot::PortedFn {
// IDA 0x65ff94: __GLOBAL__I_a_266.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x65ff94, "__GLOBAL__I_a_266")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_16SurfaceSelectionENS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x660df4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SurfaceSelection"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_16SurfaceSelectionENS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x6610a8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SurfaceSelection"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_16SurfaceSelectionENS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x66135c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SurfaceSelection"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_16SurfaceSelectionENS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x661360() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SurfaceSelection"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_16SurfaceSelectionENS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x6613fc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SurfaceSelection"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_16SurfaceSelectionENS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x661484() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SurfaceSelection"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_17sSurfaceSelectionEEEEvv")]
pub fn stub_0x661974() -> crate::slot::PortedFn {
// IDA 0x661974: void RBX::Name::callDoDeclare<RBX::sSurfaceSelection>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x661974, "void RBX::Name::callDoDeclare<RBX::sSurfaceSelection>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_17sSurfaceSelectionEEEERKS0_v")]
pub fn stub_0x661978(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sSurfaceSelection>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_16SurfaceSelectionENS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x661a58() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SurfaceSelection"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_16SurfaceSelectionENS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x661c9c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SurfaceSelection"
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_16SurfaceSelectionELZNS_17sSurfaceSelectionEENS_14FactoryProductIS2_NS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x661d10(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_16SurfaceSelectionELZNS_17sSurfaceSelectionEENS_14FactoryProductIS2_NS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x661e58(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_16SurfaceSelectionELZNS_17sSurfaceSelectionEENS_14FactoryProductIS2_NS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x661ef8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_16SurfaceSelectionELZNS_17sSurfaceSelectionEENS_14FactoryProductIS2_NS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x662040(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_16SurfaceSelectionELZNS_17sSurfaceSelectionEENS_14FactoryProductIS2_NS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x66219c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_16SurfaceSelectionELZNS_17sSurfaceSelectionEENS_14FactoryProductIS2_NS_13PartAdornmentELZNS_17sSurfaceSelectionEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x6622e4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "global constructor keyed to_a_267")]
pub fn stub_0x662c48() -> crate::slot::PortedFn {
// IDA 0x662c48: __GLOBAL__I_a_267.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x662c48, "__GLOBAL__I_a_267")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_4TeamENS_8InstanceELZNS_5sTeamEES2_E12getClassNameEv")]
pub fn stub_0x663350() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Team"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_4TeamENS_8InstanceELZNS_5sTeamEES2_E12getClassNameEv")]
pub fn stub_0x663360() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Team"
}
