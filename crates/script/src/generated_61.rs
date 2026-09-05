// Auto-generated skeletons for rbx-script — filler EA-sorted ascending after 0x5f4698 (next 100)
// Filter: Lua|Script|Yield|lua (5401 filtered, all already stubbed globally) — filler EA-sorted ascending
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x61e798..0x62ece0 | existing ~10991 -> ~11091 total (union; filler 0x61e798 ascending, global remaining)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12SelectionBoxELZNS_13sSelectionBoxEENS_14FactoryProductIS2_NS_11PVAdornmentELZNS_13sSelectionBoxEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x61e798(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "global constructor keyed to_a_253")]
pub fn stub_0x61e8f4() -> crate::slot::PortedFn {
// IDA 0x61e8f4: __GLOBAL__I_a_253.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x61e8f4, "__GLOBAL__I_a_253")
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_15sSelectionLassoEEE12getClassNameEv")]
pub fn stub_0x61fb08() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"GuiBase3d"
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_15sSelectionLassoEEE12getClassNameEv")]
pub fn stub_0x61fdd0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"GuiBase3d"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_18SelectionPartLassoENS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x62037c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SelectionPartLasso"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_18SelectionPartLassoENS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x620438() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SelectionPartLasso"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_18SelectionPartLassoENS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x6204f4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SelectionPartLasso"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_19SelectionPointLassoENS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x6204f8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SelectionPointLasso"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_19SelectionPointLassoENS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x6206e0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SelectionPointLasso"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_19SelectionPointLassoENS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x6209a4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SelectionPointLasso"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_19SelectionPointLassoENS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x620c54() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SelectionPointLasso"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_19SelectionPointLassoENS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x620cc8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SelectionPointLasso"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_20sSelectionPointLassoEEEEvv")]
pub fn stub_0x620d50() -> crate::slot::PortedFn {
// IDA 0x620d50: void RBX::Name::callDoDeclare<RBX::sSelectionPointLasso>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x620d50, "void RBX::Name::callDoDeclare<RBX::sSelectionPointLasso>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_20sSelectionPointLassoEEEERKS0_v")]
pub fn stub_0x620d54(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sSelectionPointLasso>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_19SelectionPointLassoENS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x620e34() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SelectionPointLasso"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_19SelectionPointLassoENS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x620ed0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SelectionPointLasso"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_19SelectionPointLassoENS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x6213c0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SelectionPointLasso"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_18SelectionPartLassoENS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x621604() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SelectionPartLasso"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_18SelectionPartLassoENS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x6216a0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SelectionPartLasso"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_18SelectionPartLassoENS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x621728() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SelectionPartLasso"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_19sSelectionPartLassoEEEEvv")]
pub fn stub_0x621c18() -> crate::slot::PortedFn {
// IDA 0x621c18: void RBX::Name::callDoDeclare<RBX::sSelectionPartLasso>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x621c18, "void RBX::Name::callDoDeclare<RBX::sSelectionPartLasso>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_19sSelectionPartLassoEEEERKS0_v")]
pub fn stub_0x621c1c(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sSelectionPartLasso>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_18SelectionPartLassoENS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x621cfc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SelectionPartLasso"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_18SelectionPartLassoENS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x621f40() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SelectionPartLasso"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_15sSelectionLassoEEEEvv")]
pub fn stub_0x621fb4() -> crate::slot::PortedFn {
// IDA 0x621fb4: void RBX::Name::callDoDeclare<RBX::sSelectionLasso>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x621fb4, "void RBX::Name::callDoDeclare<RBX::sSelectionLasso>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sSelectionLassoEEEERKS0_v")]
pub fn stub_0x621fb8(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sSelectionLasso>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19SelectionPointLassoELZNS_20sSelectionPointLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x622098(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19SelectionPointLassoELZNS_20sSelectionPointLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x6221dc(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_19SelectionPointLassoELZNS_20sSelectionPointLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x62227c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_19SelectionPointLassoELZNS_20sSelectionPointLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x6223c0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_19SelectionPointLassoELZNS_20sSelectionPointLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x62251c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_19SelectionPointLassoELZNS_20sSelectionPointLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x622660(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_18SelectionPartLassoELZNS_19sSelectionPartLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x622958(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_18SelectionPartLassoELZNS_19sSelectionPartLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x622a9c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_18SelectionPartLassoELZNS_19sSelectionPartLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x622b3c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_18SelectionPartLassoELZNS_19sSelectionPartLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x622c80(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_18SelectionPartLassoELZNS_19sSelectionPartLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x622ddc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_18SelectionPartLassoELZNS_19sSelectionPartLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x622f20(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14SelectionLassoELZNS_15sSelectionLassoEENS_17NonFactoryProductINS_9GuiBase3dELZNS_15sSelectionLassoEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x623678(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14SelectionLassoELZNS_15sSelectionLassoEENS_17NonFactoryProductINS_9GuiBase3dELZNS_15sSelectionLassoEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x623734(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_14SelectionLassoELZNS_15sSelectionLassoEENS_17NonFactoryProductINS_9GuiBase3dELZNS_15sSelectionLassoEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x623800(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_14SelectionLassoELZNS_15sSelectionLassoEENS_17NonFactoryProductINS_9GuiBase3dELZNS_15sSelectionLassoEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x6238b8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_14SelectionLassoELZNS_15sSelectionLassoEENS_17NonFactoryProductINS_9GuiBase3dELZNS_15sSelectionLassoEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x623988(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_14SelectionLassoELZNS_15sSelectionLassoEENS_17NonFactoryProductINS_9GuiBase3dELZNS_15sSelectionLassoEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x623a40(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "global constructor keyed to_a_254")]
pub fn stub_0x624188() -> crate::slot::PortedFn {
// IDA 0x624188: __GLOBAL__I_a_254.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x624188, "__GLOBAL__I_a_254")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x625424() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SkateboardController"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x6256cc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SkateboardController"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x625c0c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SkateboardController"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x625c10() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SkateboardController"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x625cac() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SkateboardController"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x625d34() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SkateboardController"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_21sSkateboardControllerEEEEvv")]
pub fn stub_0x625e78() -> crate::slot::PortedFn {
// IDA 0x625e78: void RBX::Name::callDoDeclare<RBX::sSkateboardController>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x625e78, "void RBX::Name::callDoDeclare<RBX::sSkateboardController>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_21sSkateboardControllerEEEERKS0_v")]
pub fn stub_0x625e7c(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sSkateboardController>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x625f5c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SkateboardController"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x6261a0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SkateboardController"
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x626290(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x626294(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x626334(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x62633c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x6263e0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x6263e8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x62648c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x626494(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "global constructor keyed to_a_255")]
pub fn stub_0x626cfc() -> crate::slot::PortedFn {
// IDA 0x626cfc: __GLOBAL__I_a_255.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x626cfc, "__GLOBAL__I_a_255")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x62a710() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SkateboardPlatform"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x62a724() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SkateboardPlatform"
}

#[doc(alias = "__ZN3RBX18DescribedCreatableINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x62a738(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "__ZN3RBX18DescribedCreatableINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x62a74c(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "__ZThn132_N3RBX18DescribedCreatableINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x62a804(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 132, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 132);
}

#[doc(alias = "__ZThn132_N3RBX18DescribedCreatableINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x62a818(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 132, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 132);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_18SkateboardPlatformELZNS_19sSkateboardPlatformEENS_14FactoryProductIS2_NS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x62a8cc(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_18SkateboardPlatformELZNS_19sSkateboardPlatformEENS_14FactoryProductIS2_NS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x62a8e0(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn132_N3RBX10Reflection9DescribedINS_18SkateboardPlatformELZNS_19sSkateboardPlatformEENS_14FactoryProductIS2_NS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x62a990(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 132, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 132);
}

#[doc(alias = "__ZThn132_N3RBX10Reflection9DescribedINS_18SkateboardPlatformELZNS_19sSkateboardPlatformEENS_14FactoryProductIS2_NS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x62a9a4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 132, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 132);
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEED1Ev")]
pub fn stub_0x62aa58(handle: crate::slot::InstanceHandle) {
// RBX::FactoryProduct dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEED0Ev")]
pub fn stub_0x62aa6c(handle: crate::slot::InstanceHandle) {
// RBX::FactoryProduct dtor.
drop(handle);
}

#[doc(alias = "__ZThn132_N3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEED1Ev")]
pub fn stub_0x62ab1c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 132, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 132);
}

#[doc(alias = "__ZThn132_N3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEED0Ev")]
pub fn stub_0x62ab30(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 132, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 132);
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x62ad98() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SkateboardPlatform"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x62ad9c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SkateboardPlatform"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x62ae38() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SkateboardPlatform"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x62aec0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SkateboardPlatform"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_19sSkateboardPlatformEEEEvv")]
pub fn stub_0x62b3b4() -> crate::slot::PortedFn {
// IDA 0x62b3b4: void RBX::Name::callDoDeclare<RBX::sSkateboardPlatform>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x62b3b4, "void RBX::Name::callDoDeclare<RBX::sSkateboardPlatform>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_19sSkateboardPlatformEEEERKS0_v")]
pub fn stub_0x62b3b8(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sSkateboardPlatform>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x62b498() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SkateboardPlatform"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x62b6dc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"SkateboardPlatform"
}

#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ModelInstanceEEESB_ENS7_5list2INS7_5valueISB_EESG_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x62cc74() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ModelInstanceEEESA_ENS6_5list2INS6_5valueISA_EESF_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x62ce60() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEED1Ev")]
pub fn stub_0x62e2ec(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEED1Ev")]
pub fn stub_0x62e300(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEED0Ev")]
pub fn stub_0x62e314(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEED0Ev")]
pub fn stub_0x62e31c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_18SkateboardPlatformELZNS_19sSkateboardPlatformEENS_14FactoryProductIS2_NS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2Ev")]
pub fn stub_0x62e860() -> crate::slot::InstanceHandle {
// RBX::Reflection::Described ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::Described")
}

#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x62ea88(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x62ea9c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x62eb50(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x62eb64(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_18SkateboardPlatformELZNS_19sSkateboardPlatformEENS_14FactoryProductIS2_NS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x62ec18(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_18SkateboardPlatformELZNS_19sSkateboardPlatformEENS_14FactoryProductIS2_NS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x62ec2c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_18SkateboardPlatformELZNS_19sSkateboardPlatformEENS_14FactoryProductIS2_NS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x62ece0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}
