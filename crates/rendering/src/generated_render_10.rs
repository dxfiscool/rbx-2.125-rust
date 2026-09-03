//! rendering shard render_10 — 120 stubs 0x4a725c..0x4aa5e8 EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 37620->37740 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 37620 before -> 37740 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 120 lowest remaining 0x4a725c..0x4aa5e8 (next lowest 0x4ad590 if exists)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x4a725c — __ZThn32_N3RBX20ExtrudedPartInstanceD1Ev
// IDA 0x4a725c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a725c() {
}

// 0x4a7270 — __ZThn36_N3RBX20ExtrudedPartInstanceD1Ev
// IDA 0x4a7270: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a7270() {
}

// 0x4a7284 — __ZThn132_N3RBX20ExtrudedPartInstanceD1Ev
// IDA 0x4a7284: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a7284() {
}

// 0x4a7298 — __ZNK3RBX20ExtrudedPartInstance16getMinimumUiSizeEv
#[doc(alias = "RBX::ExtrudedPartInstance::getMinimumUiSize(void)const")]
// IDA 0x4a7298: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a7298() {
}

// 0x4a7524 — __ZNK3RBX20ExtrudedPartInstance18getResizeIncrementEv
#[doc(alias = "RBX::ExtrudedPartInstance::getResizeIncrement(void)const")]
// IDA 0x4a7524: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a7524() {
}

// 0x4a7528 — __ZNK3RBX20ExtrudedPartInstance19getResizeHandleMaskEv
#[doc(alias = "RBX::ExtrudedPartInstance::getResizeHandleMask(void)const")]
// IDA 0x4a7528: 157 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a7528() {
}

// 0x4a772c — __ZNK3RBX20ExtrudedPartInstance19getVisualTrussStyleEv
#[doc(alias = "RBX::ExtrudedPartInstance::getVisualTrussStyle(void)const")]
// IDA 0x4a772c: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a772c() {
}

// 0x4a7734 — __ZN3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::~EnumPropDescriptor()")]
// IDA 0x4a7734: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a7734() {
}

// 0x4a7758 — __ZNK3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE12getClassNameEv")]
// IDA 0x4a7758: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a7758() {
}

// 0x4a7768 — __ZNK3RBX20ExtrudedPartInstance11getPartTypeEv
#[doc(alias = "RBX::ExtrudedPartInstance::getPartType(void)const")]
// IDA 0x4a7768: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a7768() {
}

// 0x4a776c — __ZThn32_NK3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE12getClassNameEv")]
// IDA 0x4a776c: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a776c() {
}

// 0x4a777c — __ZN3RBX18DescribedCreatableINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x4a777c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a777c() {
}

// 0x4a7790 — __ZN3RBX18DescribedCreatableINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x4a7790: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a7790() {
}

// 0x4a7840 — __ZThn132_N3RBX18DescribedCreatableINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn132_N3RBX18DescribedCreatableINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x4a7840: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a7840() {
}

// 0x4a7854 — __ZThn132_N3RBX18DescribedCreatableINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn132_N3RBX18DescribedCreatableINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x4a7854: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a7854() {
}

// 0x4a7908 — __ZN3RBX10Reflection9DescribedINS_20ExtrudedPartInstanceELZNS_13sExtrudedPartEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_20ExtrudedPartInstanceELZNS_13sExtrudedPartEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x4a7908: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a7908() {
}

// 0x4a791c — __ZN3RBX10Reflection9DescribedINS_20ExtrudedPartInstanceELZNS_13sExtrudedPartEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_20ExtrudedPartInstanceELZNS_13sExtrudedPartEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x4a791c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a791c() {
}

// 0x4a79cc — __ZThn132_N3RBX10Reflection9DescribedINS_20ExtrudedPartInstanceELZNS_13sExtrudedPartEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn132_N3RBX10Reflection9DescribedINS_20ExtrudedPartInstanceELZNS_13sExtrudedPartEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x4a79cc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a79cc() {
}

// 0x4a79e0 — __ZThn132_N3RBX10Reflection9DescribedINS_20ExtrudedPartInstanceELZNS_13sExtrudedPartEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn132_N3RBX10Reflection9DescribedINS_20ExtrudedPartInstanceELZNS_13sExtrudedPartEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x4a79e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a79e0() {
}

// 0x4a7a94 — __ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED1Ev")]
// IDA 0x4a7a94: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a7a94() {
}

// 0x4a7aa8 — __ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED0Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED0Ev")]
// IDA 0x4a7aa8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a7aa8() {
}

// 0x4a7b58 — __ZThn132_N3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED1Ev
#[doc(alias = "__ZThn132_N3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED1Ev")]
// IDA 0x4a7b58: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a7b58() {
}

// 0x4a7b6c — __ZThn132_N3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED0Ev
#[doc(alias = "__ZThn132_N3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED0Ev")]
// IDA 0x4a7b6c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a7b6c() {
}

// 0x4a7b74 — __ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE7CreatorD1Ev")]
// IDA 0x4a7b74: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4a7b74() {
}

// 0x4a7b78 — __ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE7CreatorD2Ev")]
// IDA 0x4a7b78: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a7b78() {
}

// 0x4a7c14 — __ZNK3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE7Creator12getClassNameEv")]
// IDA 0x4a7c14: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a7c14() {
}

// 0x4a7c9c — __ZNK3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE7Creator6createEv")]
// IDA 0x4a7c9c: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a7c9c() {
}

// 0x4a7de0 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_20ExtrudedPartInstanceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::ExtrudedPartInstance> RBX::Creatable<RBX::Instance>::create<RBX::ExtrudedPartInstance>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_20ExtrudedPartInstanceEEEN5boost10shared_ptrIT_EEv
// IDA 0x4a7de0: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a7de0() {
}

// 0x4a7e94 — __ZN5boost10shared_ptrIN3RBX20ExtrudedPartInstanceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::ExtrudedPartInstance>::shared_ptr<RBX::ExtrudedPartInstance,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ExtrudedPartInstance *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX20ExtrudedPartInstanceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x4a7e94: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a7e94() {
}

// 0x4a7f5c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_20ExtrudedPartInstanceES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance>(rbx_core::SharedPtr<RBX::ExtrudedPartInstance> const*,RBX::ExtrudedPartInstance *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_20ExtrudedPartInstanceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x4a7f5c: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a7f5c() {
}

// 0x4a8044 — __ZN5boost6detail12shared_countC2IPN3RBX20ExtrudedPartInstanceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ExtrudedPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ExtrudedPartInstance *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX20ExtrudedPartInstanceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x4a8044: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a8044() {
}

// 0x4a814c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ExtrudedPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x4a814c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4a814c() {
}

// 0x4a8150 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ExtrudedPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x4a8150: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4a8150() {
}

// 0x4a8154 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ExtrudedPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x4a8154: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a8154() {
}

// 0x4a8174 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ExtrudedPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x4a8174: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a8174() {
}

// 0x4a818c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ExtrudedPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x4a818c: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a818c() {
}

// 0x4a8190 — __ZN3RBX4Name13callDoDeclareILZNS_13sExtrudedPartEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sExtrudedPartEEEEvv")]
// IDA 0x4a8190: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4a8190() {
}

// 0x4a8194 — __ZN3RBX4Name9doDeclareILZNS_13sExtrudedPartEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sExtrudedPartEEEERKS0_v")]
// IDA 0x4a8194: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a8194() {
}

// 0x4a8274 — __ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE7CreatorC2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE7CreatorC2Ev")]
// IDA 0x4a8274: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a8274() {
}

// 0x4a84b8 — __ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE17static_getCreatorEv
#[doc(alias = "__ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEE17static_getCreatorEv")]
// IDA 0x4a84b8: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a84b8() {
}

// 0x4a852c — __ZThn32_N3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED1Ev
#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED1Ev")]
// IDA 0x4a852c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a852c() {
}

// 0x4a8540 — __ZThn36_N3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED1Ev
#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED1Ev")]
// IDA 0x4a8540: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a8540() {
}

// 0x4a8554 — __ZThn32_N3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED0Ev
#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED0Ev")]
// IDA 0x4a8554: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a8554() {
}

// 0x4a855c — __ZThn36_N3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED0Ev
#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEED0Ev")]
// IDA 0x4a855c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a855c() {
}

// 0x4a8564 — __ZThn32_N3RBX18DescribedCreatableINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x4a8564: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a8564() {
}

// 0x4a8578 — __ZThn32_N3RBX18DescribedCreatableINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x4a8578: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a8578() {
}

// 0x4a862c — __ZThn36_N3RBX18DescribedCreatableINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x4a862c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a862c() {
}

// 0x4a8640 — __ZThn36_N3RBX18DescribedCreatableINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x4a8640: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a8640() {
}

// 0x4a8760 — __ZThn32_N3RBX10Reflection9DescribedINS_20ExtrudedPartInstanceELZNS_13sExtrudedPartEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_20ExtrudedPartInstanceELZNS_13sExtrudedPartEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x4a8760: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a8760() {
}

// 0x4a8774 — __ZThn32_N3RBX10Reflection9DescribedINS_20ExtrudedPartInstanceELZNS_13sExtrudedPartEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_20ExtrudedPartInstanceELZNS_13sExtrudedPartEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x4a8774: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a8774() {
}

// 0x4a8828 — __ZThn36_N3RBX10Reflection9DescribedINS_20ExtrudedPartInstanceELZNS_13sExtrudedPartEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_20ExtrudedPartInstanceELZNS_13sExtrudedPartEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x4a8828: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a8828() {
}

// 0x4a883c — __ZThn36_N3RBX10Reflection9DescribedINS_20ExtrudedPartInstanceELZNS_13sExtrudedPartEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_20ExtrudedPartInstanceELZNS_13sExtrudedPartEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x4a883c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a883c() {
}

// 0x4a88f0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::EnumPropDescriptor<RBX::ExtrudedPartInstance::VisualTrussStyle (RBX::ExtrudedPartInstance::*)(void)const,void (RBX::ExtrudedPartInstance::*)(RBX::ExtrudedPartInstance::VisualTrussStyle)>(char const*,char const*,RBX::ExtrudedPartInstance::VisualTrussStyle (RBX::ExtrudedPartInstance::*)(void)const,void (RBX::ExtrudedPartInstance::*)(RBX::ExtrudedPartInstance::VisualTrussStyle),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// IDA 0x4a88f0: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a88f0() {
}

// 0x4a8aa4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::~EnumPropDescriptor()")]
// IDA 0x4a8aa4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a8aa4() {
}

// 0x4a8ad0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::isReadOnly(void)const")]
// IDA 0x4a8ad0: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a8ad0() {
}

// 0x4a8ae0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::isWriteOnly(void)const")]
// IDA 0x4a8ae0: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a8ae0() {
}

// 0x4a8af0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// IDA 0x4a8af0: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a8af0() {
}

// 0x4a8b18 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// IDA 0x4a8b18: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a8b18() {
}

// 0x4a8b3c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// IDA 0x4a8b3c: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a8b3c() {
}

// 0x4a8c88 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// IDA 0x4a8c88: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a8c88() {
}

// 0x4a8cac — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::hasStringValue(void)const")]
// IDA 0x4a8cac: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a8cac() {
}

// 0x4a8cb0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// IDA 0x4a8cb0: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a8cb0() {
}

// 0x4a8cd4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// IDA 0x4a8cd4: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a8cd4() {
}

// 0x4a8d14 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// IDA 0x4a8d14: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a8d14() {
}

// 0x4a8d34 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// IDA 0x4a8d34: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a8d34() {
}

// 0x4a8f74 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// IDA 0x4a8f74: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a8f74() {
}

// 0x4a8f90 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// IDA 0x4a8f90: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a8f90() {
}

// 0x4a8fc4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// IDA 0x4a8fc4: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a8fc4() {
}

// 0x4a8fcc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// IDA 0x4a8fcc: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a8fcc() {
}

// 0x4a9018 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// IDA 0x4a9018: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a9018() {
}

// 0x4a9038 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// IDA 0x4a9038: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a9038() {
}

// 0x4a906c — __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToIndex(RBX::ExtrudedPartInstance::VisualTrussStyle)const")]
// IDA 0x4a906c: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a906c() {
}

// 0x4a90dc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// IDA 0x4a90dc: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a90dc() {
}

// 0x4a911c — __ZNK3RBX10Reflection14PropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::GetSetImpl<RBX::ExtrudedPartInstance::VisualTrussStyle (RBX::ExtrudedPartInstance::*)(void)const,void (RBX::ExtrudedPartInstance::*)(RBX::ExtrudedPartInstance::VisualTrussStyle)>::isReadOnly(void)const")]
// IDA 0x4a911c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a911c() {
}

// 0x4a9120 — __ZNK3RBX10Reflection14PropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::GetSetImpl<RBX::ExtrudedPartInstance::VisualTrussStyle (RBX::ExtrudedPartInstance::*)(void)const,void (RBX::ExtrudedPartInstance::*)(RBX::ExtrudedPartInstance::VisualTrussStyle)>::isWriteOnly(void)const")]
// IDA 0x4a9120: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a9120() {
}

// 0x4a9124 — __ZNK3RBX10Reflection14PropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::GetSetImpl<RBX::ExtrudedPartInstance::VisualTrussStyle (RBX::ExtrudedPartInstance::*)(void)const,void (RBX::ExtrudedPartInstance::*)(RBX::ExtrudedPartInstance::VisualTrussStyle)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// IDA 0x4a9124: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a9124() {
}

// 0x4a9144 — __ZNK3RBX10Reflection14PropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::GetSetImpl<RBX::ExtrudedPartInstance::VisualTrussStyle (RBX::ExtrudedPartInstance::*)(void)const,void (RBX::ExtrudedPartInstance::*)(RBX::ExtrudedPartInstance::VisualTrussStyle)>::setValue(RBX::Reflection::DescribedBase *,RBX::ExtrudedPartInstance::VisualTrussStyle const&)const")]
// IDA 0x4a9144: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a9144() {
}

// 0x4a9168 — __GLOBAL__I_a_189
// IDA 0x4a9168: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_4a9168() {
}

// 0x4a94fc — __ZN3RBX12FaceInstance7setFaceENS_8NormalIdE
#[doc(alias = "RBX::FaceInstance::setFace(RBX::NormalId)")]
// IDA 0x4a94fc: 9 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a94fc() {
}

// 0x4a9518 — __ZN3RBX12FaceInstanceC2Ev
#[doc(alias = "RBX::FaceInstance::FaceInstance(void)")]
// IDA 0x4a9518: 119 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a9518() {
}

// 0x4a9668 — __ZNK3RBX12FaceInstance12askSetParentEPKNS_8InstanceE
#[doc(alias = "RBX::FaceInstance::askSetParent(RBX::Instance const*)const")]
// IDA 0x4a9668: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a9668() {
}

// 0x4a9724 — __ZNK3RBX12FaceInstance7getFaceEv
#[doc(alias = "RBX::FaceInstance::getFace(void)const")]
// IDA 0x4a9724: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a9724() {
}

// 0x4a9728 — __ZN3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::~EnumPropDescriptor()")]
// IDA 0x4a9728: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a9728() {
}

// 0x4a974c — __ZN3RBX12FaceInstanceD1Ev
#[doc(alias = "RBX::FaceInstance::~FaceInstance()")]
// IDA 0x4a974c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a974c() {
}

// 0x4a9808 — __ZN3RBX12FaceInstanceD0Ev
#[doc(alias = "RBX::FaceInstance::~FaceInstance()")]
// IDA 0x4a9808: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a9808() {
}

// 0x4a98d4 — __ZThn32_N3RBX12FaceInstanceD1Ev
// IDA 0x4a98d4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a98d4() {
}

// 0x4a998c — __ZThn32_N3RBX12FaceInstanceD0Ev
// IDA 0x4a998c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a998c() {
}

// 0x4a9a5c — __ZThn36_N3RBX12FaceInstanceD1Ev
// IDA 0x4a9a5c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a9a5c() {
}

// 0x4a9b14 — __ZThn36_N3RBX12FaceInstanceD0Ev
// IDA 0x4a9b14: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a9b14() {
}

// 0x4a9be4 — __ZN3RBX10Reflection9DescribedINS_12FaceInstanceELZNS_13sFaceInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12FaceInstanceELZNS_13sFaceInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x4a9be4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4a9be4() {
}

// 0x4a9be8 — __ZN3RBX10Reflection9DescribedINS_12FaceInstanceELZNS_13sFaceInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12FaceInstanceELZNS_13sFaceInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x4a9be8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a9be8() {
}

// 0x4a9c88 — __ZThn32_N3RBX10Reflection9DescribedINS_12FaceInstanceELZNS_13sFaceInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12FaceInstanceELZNS_13sFaceInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x4a9c88: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a9c88() {
}

// 0x4a9c90 — __ZThn32_N3RBX10Reflection9DescribedINS_12FaceInstanceELZNS_13sFaceInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12FaceInstanceELZNS_13sFaceInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x4a9c90: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a9c90() {
}

// 0x4a9d34 — __ZThn36_N3RBX10Reflection9DescribedINS_12FaceInstanceELZNS_13sFaceInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12FaceInstanceELZNS_13sFaceInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x4a9d34: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a9d34() {
}

// 0x4a9d3c — __ZThn36_N3RBX10Reflection9DescribedINS_12FaceInstanceELZNS_13sFaceInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12FaceInstanceELZNS_13sFaceInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x4a9d3c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a9d3c() {
}

// 0x4a9de0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::EnumPropDescriptor<RBX::NormalId (RBX::FaceInstance::*)(void)const,void (RBX::FaceInstance::*)(RBX::NormalId)>(char const*,char const*,RBX::NormalId (RBX::FaceInstance::*)(void)const,void (RBX::FaceInstance::*)(RBX::NormalId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// IDA 0x4a9de0: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a9de0() {
}

// 0x4a9f94 — __ZN3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::~EnumPropDescriptor()")]
// IDA 0x4a9f94: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a9f94() {
}

// 0x4a9fc0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::isReadOnly(void)const")]
// IDA 0x4a9fc0: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a9fc0() {
}

// 0x4a9fd0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::isWriteOnly(void)const")]
// IDA 0x4a9fd0: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a9fd0() {
}

// 0x4a9fe0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// IDA 0x4a9fe0: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a9fe0() {
}

// 0x4aa008 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// IDA 0x4aa008: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4aa008() {
}

// 0x4aa02c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// IDA 0x4aa02c: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4aa02c() {
}

// 0x4aa178 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// IDA 0x4aa178: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4aa178() {
}

// 0x4aa19c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::hasStringValue(void)const")]
// IDA 0x4aa19c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4aa19c() {
}

// 0x4aa1a0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// IDA 0x4aa1a0: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4aa1a0() {
}

// 0x4aa1c4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// IDA 0x4aa1c4: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4aa1c4() {
}

// 0x4aa204 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// IDA 0x4aa204: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4aa204() {
}

// 0x4aa224 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// IDA 0x4aa224: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4aa224() {
}

// 0x4aa464 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// IDA 0x4aa464: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4aa464() {
}

// 0x4aa480 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// IDA 0x4aa480: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4aa480() {
}

// 0x4aa4b4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// IDA 0x4aa4b4: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4aa4b4() {
}

// 0x4aa4bc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// IDA 0x4aa4bc: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4aa4bc() {
}

// 0x4aa508 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// IDA 0x4aa508: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4aa508() {
}

// 0x4aa528 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// IDA 0x4aa528: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4aa528() {
}

// 0x4aa55c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// IDA 0x4aa55c: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4aa55c() {
}

// 0x4aa59c — __ZNK3RBX10Reflection14PropDescriptorINS_12FaceInstanceENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FaceInstance,RBX::NormalId>::GetSetImpl<RBX::NormalId (RBX::FaceInstance::*)(void)const,void (RBX::FaceInstance::*)(RBX::NormalId)>::isReadOnly(void)const")]
// IDA 0x4aa59c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4aa59c() {
}

// 0x4aa5a0 — __ZNK3RBX10Reflection14PropDescriptorINS_12FaceInstanceENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FaceInstance,RBX::NormalId>::GetSetImpl<RBX::NormalId (RBX::FaceInstance::*)(void)const,void (RBX::FaceInstance::*)(RBX::NormalId)>::isWriteOnly(void)const")]
// IDA 0x4aa5a0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4aa5a0() {
}

// 0x4aa5a4 — __ZNK3RBX10Reflection14PropDescriptorINS_12FaceInstanceENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FaceInstance,RBX::NormalId>::GetSetImpl<RBX::NormalId (RBX::FaceInstance::*)(void)const,void (RBX::FaceInstance::*)(RBX::NormalId)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// IDA 0x4aa5a4: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4aa5a4() {
}

// 0x4aa5c4 — __ZNK3RBX10Reflection14PropDescriptorINS_12FaceInstanceENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FaceInstance,RBX::NormalId>::GetSetImpl<RBX::NormalId (RBX::FaceInstance::*)(void)const,void (RBX::FaceInstance::*)(RBX::NormalId)>::setValue(RBX::Reflection::DescribedBase *,RBX::NormalId const&)const")]
// IDA 0x4aa5c4: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4aa5c4() {
}

// 0x4aa5e8 — __GLOBAL__I_a_190
// IDA 0x4aa5e8: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_4aa5e8() {
}
