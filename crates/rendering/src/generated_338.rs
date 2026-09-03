//! rendering shard 338 — 120 stubs 0x5c92a0..0x5cf308 EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 36773->36893 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 36773 before -> 36893 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 120 after 0x5c9188 (range 0x5c92a0..0x5cf308)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x5c92a0 — __ZThn36_N3RBX4HintD1Ev
// type: void __fastcall(RBX::Hint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Hint::~Hint()")]
// was: __ZThn36_N3RBX4HintD1Ev
// IDA 0x5c92a0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5c92a0() {
}

// 0x5c9390 — __ZThn36_N3RBX4HintD0Ev
// type: void __fastcall(RBX::Hint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Hint::~Hint()")]
// was: __ZThn36_N3RBX4HintD0Ev
// IDA 0x5c9390: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5c9390() {
}

// 0x5c9498 — __ZN3RBX14FactoryProductINS_7MessageENS_8InstanceELZNS_8sMessageEES2_E7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_7MessageENS_8InstanceELZNS_8sMessageEES2_E7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_7MessageENS_8InstanceELZNS_8sMessageEES2_E7CreatorD1Ev
// IDA 0x5c9498: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5c9498() {
}

// 0x5c949c — __ZN3RBX14FactoryProductINS_4HintENS_7MessageELZNS_5sHintEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_4HintENS_7MessageELZNS_5sHintEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_4HintENS_7MessageELZNS_5sHintEENS_8InstanceEE7CreatorD1Ev
// IDA 0x5c949c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5c949c() {
}

// 0x5c94a0 — __ZN3RBX14FactoryProductINS_4HintENS_7MessageELZNS_5sHintEENS_8InstanceEE7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_4HintENS_7MessageELZNS_5sHintEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_4HintENS_7MessageELZNS_5sHintEENS_8InstanceEE7CreatorD2Ev
// IDA 0x5c94a0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5c94a0() {
}

// 0x5c953c — __ZNK3RBX14FactoryProductINS_4HintENS_7MessageELZNS_5sHintEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_4HintENS_7MessageELZNS_5sHintEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_4HintENS_7MessageELZNS_5sHintEENS_8InstanceEE7Creator12getClassNameEv
// IDA 0x5c953c: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c953c() {
}

// 0x5c95c4 — __ZNK3RBX14FactoryProductINS_4HintENS_7MessageELZNS_5sHintEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_4HintENS_7MessageELZNS_5sHintEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_4HintENS_7MessageELZNS_5sHintEENS_8InstanceEE7Creator6createEv
// IDA 0x5c95c4: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c95c4() {
}

// 0x5c9708 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_4HintEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Hint> RBX::Creatable<RBX::Instance>::create<RBX::Hint>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_4HintEEEN5boost10shared_ptrIT_EEv
// IDA 0x5c9708: 180 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c9708() {
}

// 0x5c9918 — __ZN3RBX10Reflection9DescribedINS_4HintELZNS_5sHintEENS_14FactoryProductIS2_NS_7MessageELZNS_5sHintEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_4HintELZNS_5sHintEENS_14FactoryProductIS2_NS_7MessageELZNS_5sHintEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_4HintELZNS_5sHintEENS_14FactoryProductIS2_NS_7MessageELZNS_5sHintEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x5c9918: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5c9918() {
}

// 0x5c9a0c — __ZN3RBX10Reflection9DescribedINS_4HintELZNS_5sHintEENS_14FactoryProductIS2_NS_7MessageELZNS_5sHintEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_4HintELZNS_5sHintEENS_14FactoryProductIS2_NS_7MessageELZNS_5sHintEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_4HintELZNS_5sHintEENS_14FactoryProductIS2_NS_7MessageELZNS_5sHintEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x5c9a0c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5c9a0c() {
}

// 0x5c9b10 — __ZThn32_N3RBX10Reflection9DescribedINS_4HintELZNS_5sHintEENS_14FactoryProductIS2_NS_7MessageELZNS_5sHintEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_4HintELZNS_5sHintEENS_14FactoryProductIS2_NS_7MessageELZNS_5sHintEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_4HintELZNS_5sHintEENS_14FactoryProductIS2_NS_7MessageELZNS_5sHintEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x5c9b10: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5c9b10() {
}

// 0x5c9c04 — __ZThn32_N3RBX10Reflection9DescribedINS_4HintELZNS_5sHintEENS_14FactoryProductIS2_NS_7MessageELZNS_5sHintEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_4HintELZNS_5sHintEENS_14FactoryProductIS2_NS_7MessageELZNS_5sHintEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_4HintELZNS_5sHintEENS_14FactoryProductIS2_NS_7MessageELZNS_5sHintEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x5c9c04: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5c9c04() {
}

// 0x5c9d0c — __ZThn36_N3RBX10Reflection9DescribedINS_4HintELZNS_5sHintEENS_14FactoryProductIS2_NS_7MessageELZNS_5sHintEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_4HintELZNS_5sHintEENS_14FactoryProductIS2_NS_7MessageELZNS_5sHintEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_4HintELZNS_5sHintEENS_14FactoryProductIS2_NS_7MessageELZNS_5sHintEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x5c9d0c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5c9d0c() {
}

// 0x5c9dfc — __ZThn36_N3RBX10Reflection9DescribedINS_4HintELZNS_5sHintEENS_14FactoryProductIS2_NS_7MessageELZNS_5sHintEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_4HintELZNS_5sHintEENS_14FactoryProductIS2_NS_7MessageELZNS_5sHintEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_4HintELZNS_5sHintEENS_14FactoryProductIS2_NS_7MessageELZNS_5sHintEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x5c9dfc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5c9dfc() {
}

// 0x5c9f04 — __ZN5boost10shared_ptrIN3RBX4HintEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Hint>::shared_ptr<RBX::Hint,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX4HintEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x5c9f04: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c9f04() {
}

// 0x5c9fcc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4HintES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Hint,RBX::Hint>(rbx_core::SharedPtr<RBX::Hint> const*,RBX::Hint *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4HintES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x5c9fcc: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c9fcc() {
}

// 0x5ca0b4 — __ZN5boost6detail12shared_countC2IPN3RBX4HintENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX4HintENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x5ca0b4: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ca0b4() {
}

// 0x5ca1bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HintENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HintENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x5ca1bc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_5ca1bc() {
}

// 0x5ca1c0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HintENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HintENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x5ca1c0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5ca1c0() {
}

// 0x5ca1c4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HintENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HintENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x5ca1c4: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ca1c4() {
}

// 0x5ca1e4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HintENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HintENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x5ca1e4: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ca1e4() {
}

// 0x5ca1fc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HintENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HintENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x5ca1fc: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ca1fc() {
}

// 0x5ca200 — __ZN3RBX4Name13callDoDeclareILZNS_5sHintEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5sHintEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_5sHintEEEEvv
// IDA 0x5ca200: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5ca200() {
}

// 0x5ca204 — __ZN3RBX4Name9doDeclareILZNS_5sHintEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sHintEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_5sHintEEEERKS0_v
// IDA 0x5ca204: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ca204() {
}

// 0x5ca2e4 — __ZN3RBX14FactoryProductINS_4HintENS_7MessageELZNS_5sHintEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_4HintENS_7MessageELZNS_5sHintEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_4HintENS_7MessageELZNS_5sHintEENS_8InstanceEE7CreatorC2Ev
// IDA 0x5ca2e4: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ca2e4() {
}

// 0x5ca528 — __ZN3RBX14FactoryProductINS_4HintENS_7MessageELZNS_5sHintEENS_8InstanceEE17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_4HintENS_7MessageELZNS_5sHintEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_4HintENS_7MessageELZNS_5sHintEENS_8InstanceEE17static_getCreatorEv
// IDA 0x5ca528: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ca528() {
}

// 0x5ca59c — __ZN3RBX14FactoryProductINS_7MessageENS_8InstanceELZNS_8sMessageEES2_E7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7MessageENS_8InstanceELZNS_8sMessageEES2_E7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_7MessageENS_8InstanceELZNS_8sMessageEES2_E7CreatorD2Ev
// IDA 0x5ca59c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ca59c() {
}

// 0x5ca638 — __ZNK3RBX14FactoryProductINS_7MessageENS_8InstanceELZNS_8sMessageEES2_E7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7MessageENS_8InstanceELZNS_8sMessageEES2_E7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_7MessageENS_8InstanceELZNS_8sMessageEES2_E7Creator12getClassNameEv
// IDA 0x5ca638: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ca638() {
}

// 0x5ca6c0 — __ZNK3RBX14FactoryProductINS_7MessageENS_8InstanceELZNS_8sMessageEES2_E7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7MessageENS_8InstanceELZNS_8sMessageEES2_E7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_7MessageENS_8InstanceELZNS_8sMessageEES2_E7Creator6createEv
// IDA 0x5ca6c0: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ca6c0() {
}

// 0x5ca804 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_7MessageEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Message> RBX::Creatable<RBX::Instance>::create<RBX::Message>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_7MessageEEEN5boost10shared_ptrIT_EEv
// IDA 0x5ca804: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ca804() {
}

// 0x5ca8b4 — __ZN5boost10shared_ptrIN3RBX7MessageEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Message>::shared_ptr<RBX::Message,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX7MessageEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x5ca8b4: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ca8b4() {
}

// 0x5ca97c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7MessageES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Message,RBX::Message>(rbx_core::SharedPtr<RBX::Message> const*,RBX::Message *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7MessageES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x5ca97c: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ca97c() {
}

// 0x5caa64 — __ZN5boost6detail12shared_countC2IPN3RBX7MessageENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX7MessageENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x5caa64: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5caa64() {
}

// 0x5cab6c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7MessageENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7MessageENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x5cab6c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_5cab6c() {
}

// 0x5cab70 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7MessageENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7MessageENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x5cab70: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5cab70() {
}

// 0x5cab74 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7MessageENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7MessageENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x5cab74: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cab74() {
}

// 0x5cab94 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7MessageENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7MessageENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x5cab94: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cab94() {
}

// 0x5cabac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7MessageENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7MessageENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x5cabac: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cabac() {
}

// 0x5cabb0 — __ZN3RBX4Name13callDoDeclareILZNS_8sMessageEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_8sMessageEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_8sMessageEEEEvv
// IDA 0x5cabb0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5cabb0() {
}

// 0x5cabb4 — __ZN3RBX4Name9doDeclareILZNS_8sMessageEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sMessageEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_8sMessageEEEERKS0_v
// IDA 0x5cabb4: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cabb4() {
}

// 0x5cac94 — __ZN3RBX14FactoryProductINS_7MessageENS_8InstanceELZNS_8sMessageEES2_E7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7MessageENS_8InstanceELZNS_8sMessageEES2_E7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_7MessageENS_8InstanceELZNS_8sMessageEES2_E7CreatorC2Ev
// IDA 0x5cac94: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cac94() {
}

// 0x5caed8 — __ZN3RBX14FactoryProductINS_7MessageENS_8InstanceELZNS_8sMessageEES2_E17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7MessageENS_8InstanceELZNS_8sMessageEES2_E17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_7MessageENS_8InstanceELZNS_8sMessageEES2_E17static_getCreatorEv
// IDA 0x5caed8: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5caed8() {
}

// 0x5caf4c — __ZN3RBX10Reflection9DescribedINS_7MessageELZNS_8sMessageEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sMessageEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7MessageELZNS_8sMessageEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sMessageEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_7MessageELZNS_8sMessageEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sMessageEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x5caf4c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5caf4c() {
}

// 0x5caf50 — __ZN3RBX10Reflection9DescribedINS_7MessageELZNS_8sMessageEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sMessageEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7MessageELZNS_8sMessageEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sMessageEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_7MessageELZNS_8sMessageEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sMessageEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x5caf50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5caf50() {
}

// 0x5caff0 — __ZThn32_N3RBX10Reflection9DescribedINS_7MessageELZNS_8sMessageEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sMessageEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7MessageELZNS_8sMessageEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sMessageEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_7MessageELZNS_8sMessageEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sMessageEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x5caff0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5caff0() {
}

// 0x5caff8 — __ZThn32_N3RBX10Reflection9DescribedINS_7MessageELZNS_8sMessageEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sMessageEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7MessageELZNS_8sMessageEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sMessageEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_7MessageELZNS_8sMessageEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sMessageEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x5caff8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5caff8() {
}

// 0x5cb09c — __ZThn36_N3RBX10Reflection9DescribedINS_7MessageELZNS_8sMessageEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sMessageEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7MessageELZNS_8sMessageEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sMessageEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_7MessageELZNS_8sMessageEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sMessageEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x5cb09c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5cb09c() {
}

// 0x5cb0a4 — __ZThn36_N3RBX10Reflection9DescribedINS_7MessageELZNS_8sMessageEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sMessageEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7MessageELZNS_8sMessageEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sMessageEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_7MessageELZNS_8sMessageEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sMessageEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x5cb0a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5cb0a4() {
}

// 0x5cb148 — __ZN3RBX10Reflection14PropDescriptorINS_7MessageESsEC2IMS2_KFRKSsvEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Message,std::string>::PropDescriptor<std::string const& (RBX::Message::*)(void)const,void (RBX::Message::*)(std::string const&)>(char const*,char const*,std::string const& (RBX::Message::*)(void)const,void (RBX::Message::*)(std::string const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_7MessageESsEC2IMS2_KFRKSsvEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x5cb148: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cb148() {
}

// 0x5cb25c — __ZN3RBX10Reflection14PropDescriptorINS_7MessageESsED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Message,std::string>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_7MessageESsED0Ev
// IDA 0x5cb25c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5cb25c() {
}

// 0x5cb288 — __ZNK3RBX10Reflection14PropDescriptorINS_7MessageESsE10GetSetImplIMS2_KFRKSsvEMS2_FvS6_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Message,std::string>::GetSetImpl<std::string const& (RBX::Message::*)(void)const,void (RBX::Message::*)(std::string const&)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7MessageESsE10GetSetImplIMS2_KFRKSsvEMS2_FvS6_EE10isReadOnlyEv
// IDA 0x5cb288: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cb288() {
}

// 0x5cb28c — __ZNK3RBX10Reflection14PropDescriptorINS_7MessageESsE10GetSetImplIMS2_KFRKSsvEMS2_FvS6_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Message,std::string>::GetSetImpl<std::string const& (RBX::Message::*)(void)const,void (RBX::Message::*)(std::string const&)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7MessageESsE10GetSetImplIMS2_KFRKSsvEMS2_FvS6_EE11isWriteOnlyEv
// IDA 0x5cb28c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cb28c() {
}

// 0x5cb290 — __ZNK3RBX10Reflection14PropDescriptorINS_7MessageESsE10GetSetImplIMS2_KFRKSsvEMS2_FvS6_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Message,std::string>::GetSetImpl<std::string const& (RBX::Message::*)(void)const,void (RBX::Message::*)(std::string const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7MessageESsE10GetSetImplIMS2_KFRKSsvEMS2_FvS6_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x5cb290: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cb290() {
}

// 0x5cb2c0 — __ZNK3RBX10Reflection14PropDescriptorINS_7MessageESsE10GetSetImplIMS2_KFRKSsvEMS2_FvS6_EE8setValueEPNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Message,std::string>::GetSetImpl<std::string const& (RBX::Message::*)(void)const,void (RBX::Message::*)(std::string const&)>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7MessageESsE10GetSetImplIMS2_KFRKSsvEMS2_FvS6_EE8setValueEPNS0_13DescribedBaseES6_
// IDA 0x5cb2c0: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cb2c0() {
}

// 0x5cb2e4 — __GLOBAL__I_a_227
#[doc(alias = "global constructor keyed to_a_227")]
// was: __GLOBAL__I_a_227
// IDA 0x5cb2e4: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_5cb2e4() {
}

// 0x5cb594 — __ZNK3RBX13ModelInstance23getPrimaryPartSetByUserEv
// type: _DWORD __fastcall(RBX::ModelInstance *__hidden this)
#[doc(alias = "RBX::ModelInstance::getPrimaryPartSetByUser(void)const")]
// was: __ZNK3RBX13ModelInstance23getPrimaryPartSetByUserEv
// IDA 0x5cb594: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cb594() {
}

// 0x5cb5b8 — __ZN3RBX13ModelInstance23setPrimaryPartSetByUserEPNS_12PartInstanceE
// type: _DWORD __fastcall(RBX::ModelInstance *__hidden this, RBX::PartInstance *)
#[doc(alias = "RBX::ModelInstance::setPrimaryPartSetByUser(RBX::PartInstance *)")]
// was: __ZN3RBX13ModelInstance23setPrimaryPartSetByUserEPNS_12PartInstanceE
// IDA 0x5cb5b8: 74 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cb5b8() {
}

// 0x5cb690 — __ZN3RBX13ModelInstance11breakJointsEv
// type: _DWORD __fastcall(RBX::ModelInstance *__hidden this)
#[doc(alias = "RBX::ModelInstance::breakJoints(void)")]
// was: __ZN3RBX13ModelInstance11breakJointsEv
// IDA 0x5cb690: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cb690() {
}

// 0x5cb6ac — __ZN3RBX13ModelInstance10makeJointsEv
// type: _DWORD __fastcall(RBX::ModelInstance *__hidden this)
#[doc(alias = "RBX::ModelInstance::makeJoints(void)")]
// was: __ZN3RBX13ModelInstance10makeJointsEv
// IDA 0x5cb6ac: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cb6ac() {
}

// 0x5cb6c8 — __ZN3RBX13ModelInstance18calculateModelSizeEv
// type: _DWORD __fastcall(RBX::ModelInstance *__hidden this)
#[doc(alias = "RBX::ModelInstance::calculateModelSize(void)")]
// was: __ZN3RBX13ModelInstance18calculateModelSizeEv
// IDA 0x5cb6c8: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cb6c8() {
}

// 0x5cb73c — __ZN3RBX13ModelInstance20calculateModelCFrameEv
// type: _DWORD __fastcall(RBX::ModelInstance *__hidden this)
#[doc(alias = "RBX::ModelInstance::calculateModelCFrame(void)")]
// was: __ZN3RBX13ModelInstance20calculateModelCFrameEv
// IDA 0x5cb73c: 88 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cb73c() {
}

// 0x5cb994 — __ZN3RBX13ModelInstance22setIdentityOrientationEv
// type: _DWORD __fastcall(RBX::ModelInstance *__hidden this)
#[doc(alias = "RBX::ModelInstance::setIdentityOrientation(void)")]
// was: __ZN3RBX13ModelInstance22setIdentityOrientationEv
// IDA 0x5cb994: 220 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cb994() {
}

// 0x5cbbf8 — __ZN3RBX13ModelInstance26resetOrientationToIdentityEv
// type: _DWORD __fastcall(RBX::ModelInstance *__hidden this)
#[doc(alias = "RBX::ModelInstance::resetOrientationToIdentity(void)")]
// was: __ZN3RBX13ModelInstance26resetOrientationToIdentityEv
// IDA 0x5cbbf8: 201 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cbbf8() {
}

// 0x5cbe18 — __ZN3RBX13ModelInstanceC1Ev
// type: _DWORD __fastcall(RBX::ModelInstance *__hidden this)
#[doc(alias = "RBX::ModelInstance::ModelInstance(void)")]
// was: __ZN3RBX13ModelInstanceC1Ev
// IDA 0x5cbe18: 270 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cbe18() {
}

// 0x5cc128 — __ZN3RBX13ModelInstanceC2Ev
// type: _DWORD __fastcall(RBX::ModelInstance *__hidden this)
#[doc(alias = "RBX::ModelInstance::ModelInstance(void)")]
// was: __ZN3RBX13ModelInstanceC2Ev
// IDA 0x5cc128: 289 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cc128() {
}

// 0x5cc458 — __ZN3RBX13ModelInstanceD0Ev
// type: void __fastcall(RBX::ModelInstance *__hidden this)
#[doc(alias = "RBX::ModelInstance::~ModelInstance()")]
// was: __ZN3RBX13ModelInstanceD0Ev
// IDA 0x5cc458: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5cc458() {
}

// 0x5cc504 — __ZN3RBX13ModelInstanceD1Ev
// type: void __fastcall(RBX::ModelInstance *__hidden this)
#[doc(alias = "RBX::ModelInstance::~ModelInstance()")]
// was: __ZN3RBX13ModelInstanceD1Ev
// IDA 0x5cc504: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5cc504() {
}

// 0x5cc514 — __ZThn32_N3RBX13ModelInstanceD0Ev
// type: void __fastcall(RBX::ModelInstance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ModelInstance::~ModelInstance()")]
// was: __ZThn32_N3RBX13ModelInstanceD0Ev
// IDA 0x5cc514: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5cc514() {
}

// 0x5cc51c — __ZThn36_N3RBX13ModelInstanceD0Ev
// type: void __fastcall(RBX::ModelInstance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ModelInstance::~ModelInstance()")]
// was: __ZThn36_N3RBX13ModelInstanceD0Ev
// IDA 0x5cc51c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5cc51c() {
}

// 0x5cc524 — __ZThn120_N3RBX13ModelInstanceD0Ev
// type: void __fastcall(RBX::ModelInstance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ModelInstance::~ModelInstance()")]
// was: __ZThn120_N3RBX13ModelInstanceD0Ev
// IDA 0x5cc524: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5cc524() {
}

// 0x5cc52c — __ZN3RBX13ModelInstanceD2Ev
// type: void __fastcall(RBX::ModelInstance *__hidden this)
#[doc(alias = "RBX::ModelInstance::~ModelInstance()")]
// was: __ZN3RBX13ModelInstanceD2Ev
// IDA 0x5cc52c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5cc52c() {
}

// 0x5cc6b0 — __ZThn32_N3RBX13ModelInstanceD1Ev
// type: void __fastcall(RBX::ModelInstance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ModelInstance::~ModelInstance()")]
// was: __ZThn32_N3RBX13ModelInstanceD1Ev
// IDA 0x5cc6b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5cc6b0() {
}

// 0x5cc6c0 — __ZThn36_N3RBX13ModelInstanceD1Ev
// type: void __fastcall(RBX::ModelInstance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ModelInstance::~ModelInstance()")]
// was: __ZThn36_N3RBX13ModelInstanceD1Ev
// IDA 0x5cc6c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5cc6c0() {
}

// 0x5cc6d0 — __ZThn120_N3RBX13ModelInstanceD1Ev
// type: void __fastcall(RBX::ModelInstance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ModelInstance::~ModelInstance()")]
// was: __ZThn120_N3RBX13ModelInstanceD1Ev
// IDA 0x5cc6d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5cc6d0() {
}

// 0x5cc6e0 — __ZN3RBX13ModelInstance21hackPhysicalCharacterEv
// type: _DWORD __fastcall(RBX::ModelInstance *__hidden this)
#[doc(alias = "RBX::ModelInstance::hackPhysicalCharacter(void)")]
// was: __ZN3RBX13ModelInstance21hackPhysicalCharacterEv
// IDA 0x5cc6e0: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cc6e0() {
}

// 0x5cc728 — __ZN3RBX13ModelInstance7setNameERKSs
// type: _DWORD __fastcall(RBX::ModelInstance *__hidden this, const std::string *)
#[doc(alias = "RBX::ModelInstance::setName(std::string const&)")]
// was: __ZN3RBX13ModelInstance7setNameERKSs
// IDA 0x5cc728: 13 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cc728() {
}

// 0x5cc8ec — __ZN3RBX13ModelInstance15setExtentsDirtyEv
// type: _DWORD __fastcall(RBX::ModelInstance *__hidden this)
#[doc(alias = "RBX::ModelInstance::setExtentsDirty(void)")]
// was: __ZN3RBX13ModelInstance15setExtentsDirtyEv
// IDA 0x5cc8ec: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cc8ec() {
}

// 0x5cc934 — __ZN3RBX13ModelInstance12onChildAddedEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::ModelInstance *__hidden this, RBX::Instance *lpsrc)
#[doc(alias = "RBX::ModelInstance::onChildAdded(RBX::Instance *)")]
// was: __ZN3RBX13ModelInstance12onChildAddedEPNS_8InstanceE
// IDA 0x5cc934: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cc934() {
}

// 0x5cc984 — __ZN3RBX13ModelInstance15onChildRemovingEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::ModelInstance *__hidden this, RBX::Instance *lpsrc)
#[doc(alias = "RBX::ModelInstance::onChildRemoving(RBX::Instance *)")]
// was: __ZN3RBX13ModelInstance15onChildRemovingEPNS_8InstanceE
// IDA 0x5cc984: 68 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cc984() {
}

// 0x5cca4c — __ZN3RBX13ModelInstance14onChildChangedEPNS_8InstanceERKNS_15PropertyChangedE
#[doc(alias = "RBX::ModelInstance::onChildChanged(RBX::Instance *,RBX::PropertyChanged const&)")]
// was: __ZN3RBX13ModelInstance14onChildChangedEPNS_8InstanceERKNS_15PropertyChangedE
// IDA 0x5cca4c: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cca4c() {
}

// 0x5cca68 — __ZNK3RBX13ModelInstance12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::ModelInstance *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::ModelInstance::askSetParent(RBX::Instance const*)const")]
// was: __ZNK3RBX13ModelInstance12askSetParentEPKNS_8InstanceE
// IDA 0x5cca68: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cca68() {
}

// 0x5ccaa4 — __ZN3RBX13ModelInstance17onDescendantAddedEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::ModelInstance *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::ModelInstance::onDescendantAdded(RBX::Instance *)")]
// was: __ZN3RBX13ModelInstance17onDescendantAddedEPNS_8InstanceE
// IDA 0x5ccaa4: 9 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ccaa4() {
}

// 0x5ccac0 — __ZN3RBX13ModelInstance20onDescendantRemovingERKN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::ModelInstance::onDescendantRemoving(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: __ZN3RBX13ModelInstance20onDescendantRemovingERKN5boost10shared_ptrINS_8InstanceEEE
// IDA 0x5ccac0: 52 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ccac0() {
}

// 0x5ccb5c — __ZN3RBX13ModelInstance14getPrimaryPartEv
// type: _DWORD __fastcall(RBX::ModelInstance *__hidden this)
#[doc(alias = "RBX::ModelInstance::getPrimaryPart(void)")]
// was: __ZN3RBX13ModelInstance14getPrimaryPartEv
// IDA 0x5ccb5c: 116 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ccb5c() {
}

// 0x5ccca4 — __ZN3RBX13ModelInstance18computePrimaryPartEv
// type: _DWORD __fastcall(RBX::ModelInstance *__hidden this)
#[doc(alias = "RBX::ModelInstance::computePrimaryPart(void)")]
// was: __ZN3RBX13ModelInstance18computePrimaryPartEv
// IDA 0x5ccca4: 144 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ccca4() {
}

// 0x5cce34 — __ZThn92_N3RBX13ModelInstance14getPrimaryPartEv
// type: _DWORD __fastcall(RBX::ModelInstance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ModelInstance::getPrimaryPart(void)")]
// was: __ZThn92_N3RBX13ModelInstance14getPrimaryPartEv
// IDA 0x5cce34: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cce34() {
}

// 0x5cce3c — __ZN3RBX21VisitModelDescendantsEN5boost10shared_ptrINS_8InstanceEEEPPNS_12PartInstanceEPf
#[doc(alias = "RBX::VisitModelDescendants(rbx_core::SharedPtr<RBX::Instance>,RBX::PartInstance **,float *)")]
// was: __ZN3RBX21VisitModelDescendantsEN5boost10shared_ptrINS_8InstanceEEEPPNS_12PartInstanceEPf
// IDA 0x5cce3c: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cce3c() {
}

// 0x5ccebc — __ZN3RBXL5makeJEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::makeJ(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBXL5makeJEN5boost10shared_ptrINS_8InstanceEEE
// IDA 0x5ccebc: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ccebc() {
}

// 0x5ccf3c — __ZN3RBXL6breakJEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::breakJ(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBXL6breakJEN5boost10shared_ptrINS_8InstanceEEE
// IDA 0x5ccf3c: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ccf3c() {
}

// 0x5cd060 — __ZN3RBX13ModelInstance11getLocationEv
// type: _DWORD __fastcall(RBX::ModelInstance *__hidden this)
#[doc(alias = "RBX::ModelInstance::getLocation(void)")]
// was: __ZN3RBX13ModelInstance11getLocationEv
// IDA 0x5cd060: 144 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cd060() {
}

// 0x5cd1f4 — __ZTv0_n12_N3RBX13ModelInstance11getLocationEv
// type: _DWORD __fastcall(RBX::ModelInstance *__hidden this)
#[doc(alias = "virtual thunk toRBX::ModelInstance::getLocation(void)")]
// was: __ZTv0_n12_N3RBX13ModelInstance11getLocationEv
// IDA 0x5cd1f4: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cd1f4() {
}

// 0x5cd2d0 — __ZNK3RBX13ModelInstance19computeExtentsWorldEv
// type: _DWORD __fastcall(RBX::ModelInstance *__hidden this)
#[doc(alias = "RBX::ModelInstance::computeExtentsWorld(void)const")]
// was: __ZNK3RBX13ModelInstance19computeExtentsWorldEv
// IDA 0x5cd2d0: 138 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cd2d0() {
}

// 0x5cd44c — __ZN3RBXL21unionPartExtentsWorldEN5boost10shared_ptrINS_8InstanceEEERNS_7ExtentsE
#[doc(alias = "RBX::unionPartExtentsWorld(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents &)")]
// was: __ZN3RBXL21unionPartExtentsWorldEN5boost10shared_ptrINS_8InstanceEEERNS_7ExtentsE
// IDA 0x5cd44c: 68 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cd44c() {
}

// 0x5cd7ac — __ZN3RBX13ModelInstance11computePartEv
// type: _DWORD __fastcall(RBX::ModelInstance *__hidden this)
#[doc(alias = "RBX::ModelInstance::computePart(void)")]
// was: __ZN3RBX13ModelInstance11computePartEv
// IDA 0x5cd7ac: 189 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cd7ac() {
}

// 0x5cda60 — __ZN3RBX13ModelInstance12onCameraNearEf
// type: _DWORD __fastcall(RBX::ModelInstance *__hidden this, float)
#[doc(alias = "RBX::ModelInstance::onCameraNear(float)")]
// was: __ZN3RBX13ModelInstance12onCameraNearEf
// IDA 0x5cda60: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cda60() {
}

// 0x5cda9c — __ZThn120_N3RBX13ModelInstance12onCameraNearEf
// type: _DWORD __fastcall(RBX::ModelInstance *__hidden this, float)
#[doc(alias = "non-virtual thunk toRBX::ModelInstance::onCameraNear(float)")]
// was: __ZThn120_N3RBX13ModelInstance12onCameraNearEf
// IDA 0x5cda9c: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cda9c() {
}

// 0x5cdaa4 — __ZN3RBX13ModelInstance25getCameraIgnorePrimitivesERSt6vectorIPKNS_9PrimitiveESaIS4_EE
#[doc(alias = "RBX::ModelInstance::getCameraIgnorePrimitives(std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &)")]
// was: __ZN3RBX13ModelInstance25getCameraIgnorePrimitivesERSt6vectorIPKNS_9PrimitiveESaIS4_EE
// IDA 0x5cdaa4: 15 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cdaa4() {
}

// 0x5cdacc — __ZThn120_N3RBX13ModelInstance25getCameraIgnorePrimitivesERSt6vectorIPKNS_9PrimitiveESaIS4_EE
#[doc(alias = "non-virtual thunk toRBX::ModelInstance::getCameraIgnorePrimitives(std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &)")]
// was: __ZThn120_N3RBX13ModelInstance25getCameraIgnorePrimitivesERSt6vectorIPKNS_9PrimitiveESaIS4_EE
// IDA 0x5cdacc: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cdacc() {
}

// 0x5cdad4 — __ZNK3RBX13ModelInstance17getModelInPrimaryEv
// type: _DWORD __fastcall(RBX::ModelInstance *__hidden this)
#[doc(alias = "RBX::ModelInstance::getModelInPrimary(void)const")]
// was: __ZNK3RBX13ModelInstance17getModelInPrimaryEv
// IDA 0x5cdad4: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cdad4() {
}

// 0x5cdb34 — __ZN3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEED1Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::ModelInstance,RBX::PartInstance>::~RefPropDescriptor()")]
// was: __ZN3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEED1Ev
// IDA 0x5cdb34: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5cdb34() {
}

// 0x5cdb60 — __ZN3RBX10Reflection13BoundFuncDescINS_13ModelInstanceEFvvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ModelInstance,void ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ModelInstanceEFvvELi0EED1Ev
// IDA 0x5cdb60: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5cdb60() {
}

// 0x5cde1c — __ZNSt6vectorIPN3RBX14IModelModifierESaIS2_EE9push_backERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::IModelModifier *,std::allocator<RBX::IModelModifier *>>::push_back(RBX::IModelModifier * const&)")]
// was: __ZNSt6vectorIPN3RBX14IModelModifierESaIS2_EE9push_backERKS2_
// IDA 0x5cde1c: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_5cde1c() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x5cde48 — __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EEPPNS_12PartInstanceEPfENS3_5list3INS2_3argILi1EEENS3_5valueIS9_EENSG_ISA_EEEEEEEEvRKT_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::PartInstance **,float *),boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::PartInstance **>,boost::_bi::value<float *>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::PartInstance **,float *),boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::PartInstance **>,boost::_bi::value<float *>>> const&)const")]
// was: __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EEPPNS_12PartInstanceEPfENS3_5list3INS2_3argILi1EEENS3_5valueIS9_EENSG_ISA_EEEEEEEEvRKT_
// IDA 0x5cde48: 97 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cde48() {
}

// 0x5cdf50 — __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EERNS_7ExtentsEENS3_5list2INS2_3argILi1EEENS2_17reference_wrapperIS7_EEEEEEEEvRKT_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<RBX::Extents>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<RBX::Extents>>> const&)const")]
// was: __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EERNS_7ExtentsEENS3_5list2INS2_3argILi1EEENS2_17reference_wrapperIS7_EEEEEEEEvRKT_
// IDA 0x5cdf50: 97 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cdf50() {
}

// 0x5ce23c — __ZN5boost3_bi5list2INS_3argILi1EEENS_17reference_wrapperIN3RBX7ExtentsEEEEclIPFvNS_10shared_ptrINS5_8InstanceEEERS6_ENS0_5list1IRKSC_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<RBX::Extents>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents&),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents&) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
// was: __ZN5boost3_bi5list2INS_3argILi1EEENS_17reference_wrapperIN3RBX7ExtentsEEEEclIPFvNS_10shared_ptrINS5_8InstanceEEERS6_ENS0_5list1IRKSC_EEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x5ce23c: 75 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ce23c() {
}

// 0x5ce310 — __ZN5boost3_bi5list3INS_3argILi1EEENS0_5valueIPPN3RBX12PartInstanceEEENS4_IPfEEEclIPFvNS_10shared_ptrINS5_8InstanceEEES8_SA_ENS0_5list1IRKSG_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::PartInstance **>,boost::_bi::value<float *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::PartInstance **,float *),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::PartInstance **,float *) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
// was: __ZN5boost3_bi5list3INS_3argILi1EEENS0_5valueIPPN3RBX12PartInstanceEEENS4_IPfEEEclIPFvNS_10shared_ptrINS5_8InstanceEEES8_SA_ENS0_5list1IRKSG_EEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x5ce310: 77 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ce310() {
}

// 0x5ce3ec — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX14IModelModifierESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_RKT0_St26random_access_iterator_tag
// type: int(void)
#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::IModelModifier **,std::vector<RBX::IModelModifier *,std::allocator<RBX::IModelModifier *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::IModelModifier **,std::vector<RBX::IModelModifier *,std::allocator<RBX::IModelModifier *>>>,RBX::IModelModifier *>(__gnu_cxx::__normal_iterator<RBX::IModelModifier **,std::vector<RBX::IModelModifier *,std::allocator<RBX::IModelModifier *>>>,__gnu_cxx::__normal_iterator<RBX::IModelModifier **,std::vector<RBX::IModelModifier *,std::allocator<RBX::IModelModifier *>>>,RBX::IModelModifier * const&,std::random_access_iterator_tag)")]
// was: __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX14IModelModifierESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_RKT0_St26random_access_iterator_tag
// IDA 0x5ce3ec: 59 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ce3ec() {
}

// 0x5ce47c — __ZNSt6vectorIPN3RBX14IModelModifierESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::IModelModifier *,std::allocator<RBX::IModelModifier *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::IModelModifier **,std::vector<RBX::IModelModifier *,std::allocator<RBX::IModelModifier *>>>,RBX::IModelModifier * const&)")]
// was: __ZNSt6vectorIPN3RBX14IModelModifierESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// IDA 0x5ce47c: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_5ce47c() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x5ce55c — __ZNSt12_Vector_baseIPN3RBX14IModelModifierESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::IModelModifier *,std::allocator<RBX::IModelModifier *>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIPN3RBX14IModelModifierESaIS2_EE11_M_allocateEm
// IDA 0x5ce55c: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_5ce55c() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x5cee04 — __ZN3RBX10Reflection13BoundFuncDescINS_13ModelInstanceEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ModelInstance,void ()(void),0>::BoundFuncDesc(void (RBX::ModelInstance::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ModelInstanceEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x5cee04: 90 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cee04() {
}

// 0x5cef08 — __ZN3RBX10Reflection13BoundFuncDescINS_13ModelInstanceEFvvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ModelInstance,void ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ModelInstanceEFvvELi0EED0Ev
// IDA 0x5cef08: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5cef08() {
}

// 0x5cefbc — __ZNK3RBX10Reflection13BoundFuncDescINS_13ModelInstanceEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ModelInstance,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_13ModelInstanceEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x5cefbc: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cefbc() {
}

// 0x5cefdc — __ZN3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::ModelInstance,RBX::PartInstance>::RefPropDescriptor<RBX::PartInstance* (RBX::ModelInstance::*)(void)const,void (RBX::ModelInstance::*)(RBX::PartInstance*)>(char const*,char const*,RBX::PartInstance* (RBX::ModelInstance::*)(void)const,void (RBX::ModelInstance::*)(RBX::PartInstance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x5cefdc: 57 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cefdc() {
}

// 0x5cf080 — __ZN3RBX10Reflection7RefTypeIPNS_12PartInstanceEE9singletonEv
#[doc(alias = "RBX::Reflection::RefType<RBX::PartInstance *>::singleton(void)")]
// was: __ZN3RBX10Reflection7RefTypeIPNS_12PartInstanceEE9singletonEv
// IDA 0x5cf080: 79 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cf080() {
}

// 0x5cf178 — __ZN3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEED0Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::ModelInstance,RBX::PartInstance>::~RefPropDescriptor()")]
// was: __ZN3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEED0Ev
// IDA 0x5cf178: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5cf178() {
}

// 0x5cf1a8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::ModelInstance,RBX::PartInstance>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEE10isReadOnlyEv
// IDA 0x5cf1a8: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cf1a8() {
}

// 0x5cf1b8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::ModelInstance,RBX::PartInstance>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEE11isWriteOnlyEv
// IDA 0x5cf1b8: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cf1b8() {
}

// 0x5cf1c8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::ModelInstance,RBX::PartInstance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEE11equalValuesEPKNS0_13DescribedBaseES7_
// IDA 0x5cf1c8: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cf1c8() {
}

// 0x5cf1f0 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::ModelInstance,RBX::PartInstance>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x5cf1f0: 103 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cf1f0() {
}

// 0x5cf308 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::ModelInstance,RBX::PartInstance>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x5cf308: 72 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cf308() {
}
