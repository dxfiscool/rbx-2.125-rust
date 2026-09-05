// Auto-generated skeletons for rbx-script — filler EA-sorted ascending after 0x567f8c (next 100)
// Filter: Lua|Script|Yield|lua (5401 filtered, all already stubbed globally) — filler EA-sorted ascending
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x568028..0x57ee60 | existing ~10491 -> ~10591 total (union; filler 0x568028 ascending, global remaining)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "__ZNK3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x568028() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Handles"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x5680b0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Handles"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_8sHandlesEEEEvv")]
pub fn stub_0x5685a4() -> crate::slot::PortedFn {
// IDA 0x5685a4: void RBX::Name::callDoDeclare<RBX::sHandles>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5685a4, "void RBX::Name::callDoDeclare<RBX::sHandles>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sHandlesEEEERKS0_v")]
pub fn stub_0x5685a8(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sHandles>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x568688() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Handles"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_7HandlesENS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x5688cc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Handles"
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7HandlesELZNS_8sHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x56b1b0(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7HandlesELZNS_8sHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x56b1b4(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7HandlesELZNS_8sHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x56b254(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7HandlesELZNS_8sHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x56b25c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7HandlesELZNS_8sHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x56b300(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7HandlesELZNS_8sHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x56b308(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "global constructor keyed to_a_210")]
pub fn stub_0x56f3f0() -> crate::slot::PortedFn {
// IDA 0x56f3f0: __GLOBAL__I_a_210.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x56f3f0, "__GLOBAL__I_a_210")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11HandlesBaseELZNS_12sHandlesBaseEENS_17NonFactoryProductINS_13PartAdornmentELZNS_12sHandlesBaseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x5707dc(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11HandlesBaseELZNS_12sHandlesBaseEENS_17NonFactoryProductINS_13PartAdornmentELZNS_12sHandlesBaseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x570924(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11HandlesBaseELZNS_12sHandlesBaseEENS_17NonFactoryProductINS_13PartAdornmentELZNS_12sHandlesBaseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x5709c4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11HandlesBaseELZNS_12sHandlesBaseEENS_17NonFactoryProductINS_13PartAdornmentELZNS_12sHandlesBaseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x570b0c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11HandlesBaseELZNS_12sHandlesBaseEENS_17NonFactoryProductINS_13PartAdornmentELZNS_12sHandlesBaseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x570c68(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11HandlesBaseELZNS_12sHandlesBaseEENS_17NonFactoryProductINS_13PartAdornmentELZNS_12sHandlesBaseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x570db0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "global constructor keyed to_a_211")]
pub fn stub_0x570f0c() -> crate::slot::PortedFn {
// IDA 0x570f0c: __GLOBAL__I_a_211.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x570f0c, "__GLOBAL__I_a_211")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_9HopperBinENS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x573b00() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"HopperBin"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_9HopperBinENS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x573bd4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"HopperBin"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_11StarterGearENS_8InstanceELZNS_12sStarterGearEES2_E12getClassNameEv")]
pub fn stub_0x573d38() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"StarterGear"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_11StarterGearENS_8InstanceELZNS_12sStarterGearEES2_E12getClassNameEv")]
pub fn stub_0x573df4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"StarterGear"
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEE12getClassNameEv")]
pub fn stub_0x574128() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Widget"
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEE12getClassNameEv")]
pub fn stub_0x5743d0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Widget"
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEE12getClassNameEv")]
pub fn stub_0x57490c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Hopper"
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEE12getClassNameEv")]
pub fn stub_0x5749e0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Hopper"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9HopperBinENS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x574ab4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"HopperBin"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_11StarterGearENS_8InstanceELZNS_12sStarterGearEES2_E7CreatorD1Ev")]
pub fn stub_0x574ab8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"StarterGear"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sBackpackItemEEEEvv")]
pub fn stub_0x574abc() -> crate::slot::PortedFn {
// IDA 0x574abc: void RBX::Name::callDoDeclare<RBX::sBackpackItem>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x574abc, "void RBX::Name::callDoDeclare<RBX::sBackpackItem>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sBackpackItemEEEERKS0_v")]
pub fn stub_0x574ac0(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sBackpackItem>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_11StarterGearENS_8InstanceELZNS_12sStarterGearEES2_E7CreatorD2Ev")]
pub fn stub_0x574ba0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"StarterGear"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_11StarterGearENS_8InstanceELZNS_12sStarterGearEES2_E7Creator12getClassNameEv")]
pub fn stub_0x574c3c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"StarterGear"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_11StarterGearENS_8InstanceELZNS_12sStarterGearEES2_E7Creator6createEv")]
pub fn stub_0x574cc4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"StarterGear"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sStarterGearEEEEvv")]
pub fn stub_0x575088() -> crate::slot::PortedFn {
// IDA 0x575088: void RBX::Name::callDoDeclare<RBX::sStarterGear>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x575088, "void RBX::Name::callDoDeclare<RBX::sStarterGear>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sStarterGearEEEERKS0_v")]
pub fn stub_0x57508c(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sStarterGear>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_11StarterGearENS_8InstanceELZNS_12sStarterGearEES2_E7CreatorC2Ev")]
pub fn stub_0x57516c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"StarterGear"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_11StarterGearENS_8InstanceELZNS_12sStarterGearEES2_E17static_getCreatorEv")]
pub fn stub_0x5753b0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"StarterGear"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9HopperBinENS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x575424() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"HopperBin"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_9HopperBinENS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x5754c0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"HopperBin"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_9HopperBinENS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x575548() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"HopperBin"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sHopperBinEEEEvv")]
pub fn stub_0x575a40() -> crate::slot::PortedFn {
// IDA 0x575a40: void RBX::Name::callDoDeclare<RBX::sHopperBin>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x575a40, "void RBX::Name::callDoDeclare<RBX::sHopperBin>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sHopperBinEEEERKS0_v")]
pub fn stub_0x575a44(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sHopperBin>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9HopperBinENS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x575b24() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"HopperBin"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9HopperBinENS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x575d68() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"HopperBin"
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_18StarterPackServiceELZNS_19sStarterPackServiceEENS_17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x575ddc(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_18StarterPackServiceELZNS_19sStarterPackServiceEENS_17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x575de0(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_18StarterPackServiceELZNS_19sStarterPackServiceEENS_17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x575e80(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_18StarterPackServiceELZNS_19sStarterPackServiceEENS_17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x575e88(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_18StarterPackServiceELZNS_19sStarterPackServiceEENS_17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x575f2c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_18StarterPackServiceELZNS_19sStarterPackServiceEENS_17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x575f34(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9HopperBinELZNS_10sHopperBinEENS_14FactoryProductIS2_NS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x576944(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9HopperBinELZNS_10sHopperBinEENS_14FactoryProductIS2_NS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x576a78(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9HopperBinELZNS_10sHopperBinEENS_14FactoryProductIS2_NS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x576bbc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9HopperBinELZNS_10sHopperBinEENS_14FactoryProductIS2_NS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x576cec(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9HopperBinELZNS_10sHopperBinEENS_14FactoryProductIS2_NS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x576e30(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9HopperBinELZNS_10sHopperBinEENS_14FactoryProductIS2_NS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x576f60(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12BackpackItemELZNS_13sBackpackItemEENS_17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x5770a4(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12BackpackItemELZNS_13sBackpackItemEENS_17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x5770a8(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12BackpackItemELZNS_13sBackpackItemEENS_17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x577148(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12BackpackItemELZNS_13sBackpackItemEENS_17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x577150(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12BackpackItemELZNS_13sBackpackItemEENS_17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x5771f4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12BackpackItemELZNS_13sBackpackItemEENS_17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x5771fc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11StarterGearELZNS_12sStarterGearEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sStarterGearEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x5772a0(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11StarterGearELZNS_12sStarterGearEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sStarterGearEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x5772a4(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11StarterGearELZNS_12sStarterGearEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sStarterGearEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x577344(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11StarterGearELZNS_12sStarterGearEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sStarterGearEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x57734c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11StarterGearELZNS_12sStarterGearEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sStarterGearEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x5773f0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11StarterGearELZNS_12sStarterGearEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sStarterGearEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x5773f8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "global constructor keyed to_a_212")]
pub fn stub_0x5799c4() -> crate::slot::PortedFn {
// IDA 0x5799c4: __GLOBAL__I_a_212.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x5799c4, "__GLOBAL__I_a_212")
}

#[doc(alias = "global constructor keyed to_a_213")]
pub fn stub_0x57bd94() -> crate::slot::PortedFn {
// IDA 0x57bd94: __GLOBAL__I_a_213.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x57bd94, "__GLOBAL__I_a_213")
}

#[doc(alias = "global constructor keyed to_a_214")]
pub fn stub_0x57c3d4() -> crate::slot::PortedFn {
// IDA 0x57c3d4: __GLOBAL__I_a_214.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x57c3d4, "__GLOBAL__I_a_214")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x57cf64() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"GuiImageButton"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x57d178() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"GuiImageButton"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x57d38c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"GuiImageButton"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x57d390() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"GuiImageButton"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x57d42c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"GuiImageButton"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x57d4b4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"GuiImageButton"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_15sGuiImageButtonEEEEvv")]
pub fn stub_0x57d6ac() -> crate::slot::PortedFn {
// IDA 0x57d6ac: void RBX::Name::callDoDeclare<RBX::sGuiImageButton>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x57d6ac, "void RBX::Name::callDoDeclare<RBX::sGuiImageButton>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sGuiImageButtonEEEERKS0_v")]
pub fn stub_0x57d6b0(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sGuiImageButton>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x57d790() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"GuiImageButton"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x57d9d4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"GuiImageButton"
}

#[doc(alias = "__ZN3RBX18DescribedCreatableINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x57dd10(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "__ZN3RBX18DescribedCreatableINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x57dd14(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x57ddb4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x57ddbc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x57de60(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x57de68(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x57df0c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x57df10(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x57dfb0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x57dfb8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x57e05c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x57e064(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "global constructor keyed to_a_215")]
pub fn stub_0x57e108() -> crate::slot::PortedFn {
// IDA 0x57e108: __GLOBAL__I_a_215.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x57e108, "__GLOBAL__I_a_215")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x57ea30() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ImageLabel"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x57ec48() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ImageLabel"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x57ee5c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ImageLabel"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x57ee60() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ImageLabel"
}
