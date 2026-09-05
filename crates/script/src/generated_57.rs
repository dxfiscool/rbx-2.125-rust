// Auto-generated skeletons for rbx-script — filler EA-sorted ascending after 0x57ee60 (next 100)
// Filter: Lua|Script|Yield|lua (5401 filtered, all already stubbed globally) — filler EA-sorted ascending
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x57eefc..0x5a7634 | existing ~10591 -> ~10691 total (union; filler 0x57eefc ascending, global remaining)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x57eefc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ImageLabel"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x57ef84() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ImageLabel"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sImageLabelEEEEvv")]
pub fn stub_0x57f0c8() -> crate::slot::PortedFn {
// IDA 0x57f0c8: void RBX::Name::callDoDeclare<RBX::sImageLabel>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x57f0c8, "void RBX::Name::callDoDeclare<RBX::sImageLabel>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sImageLabelEEEERKS0_v")]
pub fn stub_0x57f0cc(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sImageLabel>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x57f1ac() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ImageLabel"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x57f3f0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ImageLabel"
}

#[doc(alias = "__ZN3RBX18DescribedCreatableINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x580098(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "__ZN3RBX18DescribedCreatableINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x58009c(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x58013c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x580144(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x5801e8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x5801f0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10ImageLabelELZNS_11sImageLabelEENS_14FactoryProductIS2_NS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x580294(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10ImageLabelELZNS_11sImageLabelEENS_14FactoryProductIS2_NS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x580298(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10ImageLabelELZNS_11sImageLabelEENS_14FactoryProductIS2_NS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x580338(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10ImageLabelELZNS_11sImageLabelEENS_14FactoryProductIS2_NS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x580340(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10ImageLabelELZNS_11sImageLabelEENS_14FactoryProductIS2_NS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x5803e4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10ImageLabelELZNS_11sImageLabelEENS_14FactoryProductIS2_NS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x5803ec(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__GLOBAL__I_a_216")]
pub fn stub_0x580490() -> crate::slot::PortedFn {
// IDA 0x580490: __GLOBAL__I_a_216.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x580490, "__GLOBAL__I_a_216")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_13InsertServiceENS_8InstanceELZNS_14sInsertServiceEES2_E12getClassNameEv")]
pub fn stub_0x587248() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"InsertService"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_13InsertServiceENS_8InstanceELZNS_14sInsertServiceEES2_E12getClassNameEv")]
pub fn stub_0x587304() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"InsertService"
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x58cffc(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x58d000(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x58d0a0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x58d0a8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x58d14c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x58d154(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__GLOBAL__I_a_217")]
pub fn stub_0x59e530() -> crate::slot::PortedFn {
// IDA 0x59e530: __GLOBAL__I_a_217.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x59e530, "__GLOBAL__I_a_217")
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEE12getClassNameEv")]
pub fn stub_0x5a3394() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEE12getClassNameEv")]
pub fn stub_0x5a33bc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_5MotorENS_13JointInstanceELZNS_6sMotorEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x5a3488() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Motor"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_5MotorENS_13JointInstanceELZNS_6sMotorEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x5a3544() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Motor"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x5a36a4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Weld"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x5a3760() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Weld"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x5a38c0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ManualSurfaceJointInstance"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x5a397c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ManualSurfaceJointInstance"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x5a3adc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ManualWeld"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x5a3b9c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ManualWeld"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x5a3d00() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ManualGlue"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x5a3dc0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ManualGlue"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x5a3f24() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Motor6D"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x5a3fe0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Motor6D"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_5MotorENS_13JointInstanceELZNS_6sMotorEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x5a409c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Motor"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x5a40a0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Weld"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x5a40a4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ManualSurfaceJointInstance"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x5a40a8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ManualWeld"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x5a40ac() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ManualGlue"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x5a40b0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Motor6D"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x5a4158() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Snap"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x5a4214() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Snap"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x5a4374() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Glue"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x5a4430() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Glue"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x5a4590() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Rotate"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x5a464c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Rotate"
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_13JointInstanceELZNS_14sDynamicRotateEEE12getClassNameEv")]
pub fn stub_0x5a47ac() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"JointInstance"
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_13JointInstanceELZNS_14sDynamicRotateEEE12getClassNameEv")]
pub fn stub_0x5a4880() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"JointInstance"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x5a49f8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"RotateP"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x5a4ab4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"RotateP"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x5a4c14() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"RotateV"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x5a4cd0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"RotateV"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x5a4d8c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"RotateV"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x5a4e00() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"RotateP"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sDynamicRotateEEEEvv")]
pub fn stub_0x5a4e74() -> crate::slot::PortedFn {
// IDA 0x5a4e74: void RBX::Name::callDoDeclare<RBX::sDynamicRotate>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5a4e74, "void RBX::Name::callDoDeclare<RBX::sDynamicRotate>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sDynamicRotateEEEERKS0_v")]
pub fn stub_0x5a4e78(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sDynamicRotate>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x5a4f58() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Rotate"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x5a4fcc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Glue"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x5a5040() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Snap"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x5a50b4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Motor6D"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x5a5150() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Motor6D"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x5a51d8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Motor6D"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_8sMotor6DEEEEvv")]
pub fn stub_0x5a531c() -> crate::slot::PortedFn {
// IDA 0x5a531c: void RBX::Name::callDoDeclare<RBX::sMotor6D>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5a531c, "void RBX::Name::callDoDeclare<RBX::sMotor6D>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sMotor6DEEEERKS0_v")]
pub fn stub_0x5a5320(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sMotor6D>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x5a5400() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Motor6D"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x5a5644() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Motor6D"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x5a56b8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ManualGlue"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x5a5754() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ManualGlue"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x5a57dc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ManualGlue"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sManualGlueEEEEvv")]
pub fn stub_0x5a5ccc() -> crate::slot::PortedFn {
// IDA 0x5a5ccc: void RBX::Name::callDoDeclare<RBX::sManualGlue>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5a5ccc, "void RBX::Name::callDoDeclare<RBX::sManualGlue>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sManualGlueEEEERKS0_v")]
pub fn stub_0x5a5cd0(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sManualGlue>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x5a5db0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ManualGlue"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x5a5ff4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ManualGlue"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x5a6068() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ManualWeld"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x5a6104() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ManualWeld"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x5a618c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ManualWeld"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sManualWeldEEEEvv")]
pub fn stub_0x5a667c() -> crate::slot::PortedFn {
// IDA 0x5a667c: void RBX::Name::callDoDeclare<RBX::sManualWeld>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5a667c, "void RBX::Name::callDoDeclare<RBX::sManualWeld>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sManualWeldEEEERKS0_v")]
pub fn stub_0x5a6680(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sManualWeld>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x5a6760() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ManualWeld"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x5a69a4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ManualWeld"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x5a6a18() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ManualSurfaceJointInstance"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x5a6ab4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ManualSurfaceJointInstance"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x5a6b3c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ManualSurfaceJointInstance"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_27sManualSurfaceJointInstanceEEEEvv")]
pub fn stub_0x5a702c() -> crate::slot::PortedFn {
// IDA 0x5a702c: void RBX::Name::callDoDeclare<RBX::sManualSurfaceJointInstance>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5a702c, "void RBX::Name::callDoDeclare<RBX::sManualSurfaceJointInstance>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_27sManualSurfaceJointInstanceEEEERKS0_v")]
pub fn stub_0x5a7030(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sManualSurfaceJointInstance>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x5a7110() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ManualSurfaceJointInstance"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x5a7354() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ManualSurfaceJointInstance"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x5a73c8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Weld"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x5a7464() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Weld"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x5a74ec() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Weld"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5sWeldEEEEvv")]
pub fn stub_0x5a7630() -> crate::slot::PortedFn {
// IDA 0x5a7630: void RBX::Name::callDoDeclare<RBX::sWeld>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5a7630, "void RBX::Name::callDoDeclare<RBX::sWeld>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sWeldEEEERKS0_v")]
pub fn stub_0x5a7634(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sWeld>() — engine-side; linkage preserved via the alias.
let _ = handle;
}
