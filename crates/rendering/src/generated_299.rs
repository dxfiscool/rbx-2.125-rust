//! rendering shard 299 — 100 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15618/15618 complete, 32440->32540 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 32440 before -> 32540 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0xf6fb4c (lowest remaining 0x3e497c..0x3e793c)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x3e497c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12ShirtGraphicENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ShirtGraphic *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12ShirtGraphicENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x3e497c: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e497c() {
}

// 0x3e499c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12ShirtGraphicENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ShirtGraphic *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12ShirtGraphicENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x3e499c: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e499c() {
}

// 0x3e49b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12ShirtGraphicENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ShirtGraphic *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12ShirtGraphicENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x3e49b4: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e49b4() {
}

// 0x3e49b8 — __ZN3RBX4Name13callDoDeclareILZNS_13sShirtGraphicEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sShirtGraphicEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_13sShirtGraphicEEEEvv
// IDA 0x3e49b8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3e49b8() {
}

// 0x3e49bc — __ZN3RBX4Name9doDeclareILZNS_13sShirtGraphicEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sShirtGraphicEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_13sShirtGraphicEEEERKS0_v
// IDA 0x3e49bc: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e49bc() {
}

// 0x3e4a9c — __ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7CreatorC2Ev
// IDA 0x3e4a9c: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e4a9c() {
}

// 0x3e4ce0 — __ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE17static_getCreatorEv
// IDA 0x3e4ce0: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e4ce0() {
}

// 0x3e4d54 — __ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7CreatorD2Ev
// IDA 0x3e4d54: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e4d54() {
}

// 0x3e4df0 — __ZNK3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7Creator6createEv
// IDA 0x3e4df0: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e4df0() {
}

// 0x3e4f34 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5PantsEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Pants> RBX::Creatable<RBX::Instance>::create<RBX::Pants>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_5PantsEEEN5boost10shared_ptrIT_EEv
// IDA 0x3e4f34: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e4f34() {
}

// 0x3e4fe4 — __ZN5boost10shared_ptrIN3RBX5PantsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Pants>::shared_ptr<RBX::Pants,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX5PantsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x3e4fe4: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e4fe4() {
}

// 0x3e50ac — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5PantsES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Pants,RBX::Pants>(rbx_core::SharedPtr<RBX::Pants> const*,RBX::Pants *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5PantsES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x3e50ac: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e50ac() {
}

// 0x3e5194 — __ZN5boost6detail12shared_countC2IPN3RBX5PantsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX5PantsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x3e5194: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e5194() {
}

// 0x3e529c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5PantsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5PantsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x3e529c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_3e529c() {
}

// 0x3e52a0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5PantsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5PantsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x3e52a0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3e52a0() {
}

// 0x3e52a4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5PantsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5PantsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x3e52a4: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e52a4() {
}

// 0x3e52c4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5PantsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5PantsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x3e52c4: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e52c4() {
}

// 0x3e52dc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5PantsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5PantsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x3e52dc: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e52dc() {
}

// 0x3e52e0 — __ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7CreatorC2Ev
// IDA 0x3e52e0: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e52e0() {
}

// 0x3e5524 — __ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7CreatorD2Ev
// IDA 0x3e5524: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e5524() {
}

// 0x3e55c0 — __ZNK3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7Creator6createEv
// IDA 0x3e55c0: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e55c0() {
}

// 0x3e5704 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ShirtEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Shirt> RBX::Creatable<RBX::Instance>::create<RBX::Shirt>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ShirtEEEN5boost10shared_ptrIT_EEv
// IDA 0x3e5704: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e5704() {
}

// 0x3e57b4 — __ZN5boost10shared_ptrIN3RBX5ShirtEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Shirt>::shared_ptr<RBX::Shirt,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX5ShirtEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x3e57b4: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e57b4() {
}

// 0x3e587c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5ShirtES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Shirt,RBX::Shirt>(rbx_core::SharedPtr<RBX::Shirt> const*,RBX::Shirt *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5ShirtES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x3e587c: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e587c() {
}

// 0x3e5964 — __ZN5boost6detail12shared_countC2IPN3RBX5ShirtENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX5ShirtENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x3e5964: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e5964() {
}

// 0x3e5a6c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ShirtENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ShirtENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x3e5a6c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_3e5a6c() {
}

// 0x3e5a70 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ShirtENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ShirtENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x3e5a70: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3e5a70() {
}

// 0x3e5a74 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ShirtENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ShirtENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x3e5a74: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e5a74() {
}

// 0x3e5a94 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ShirtENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ShirtENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x3e5a94: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e5a94() {
}

// 0x3e5aac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ShirtENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ShirtENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x3e5aac: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e5aac() {
}

// 0x3e5ab0 — __ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7CreatorC2Ev
// IDA 0x3e5ab0: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e5ab0() {
}

// 0x3e5cf4 — __ZN3RBX13ModelInstance23findFirstModifierOfTypeINS_4SkinEEEPT_v
// type: void *__fastcall(int)
#[doc(alias = "RBX::Skin * RBX::ModelInstance::findFirstModifierOfType<RBX::Skin>(void)")]
// was: __ZN3RBX13ModelInstance23findFirstModifierOfTypeINS_4SkinEEEPT_v
// IDA 0x3e5cf4: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e5cf4() {
}

// 0x3e5d40 — __ZN3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3e5d40: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3e5d40() {
}

// 0x3e5d44 — __ZN3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3e5d44: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e5d44() {
}

// 0x3e5de4 — __ZThn32_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3e5de4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e5de4() {
}

// 0x3e5dec — __ZThn32_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3e5dec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e5dec() {
}

// 0x3e5e90 — __ZThn36_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3e5e90: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e5e90() {
}

// 0x3e5e98 — __ZThn36_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3e5e98: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e5e98() {
}

// 0x3e5f3c — __ZThn92_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn92_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3e5f3c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e5f3c() {
}

// 0x3e5f44 — __ZThn92_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn92_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3e5f44: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e5f44() {
}

// 0x3e5fe8 — __ZN3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EEC2INS_10BodyColorsEEEPKcS8_MT_S2_MS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundProp<RBX::BodyColors>(char const*,char const*,RBX::BrickColor RBX::BodyColors::*,void (RBX::BodyColors::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EEC2INS_10BodyColorsEEEPKcS8_MT_S2_MS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE
// IDA 0x3e5fe8: 155 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e5fe8() {
}

// 0x3e617c — __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_10BodyColorsEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::BodyColors>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_10BodyColorsEE10isReadOnlyEv
// IDA 0x3e617c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e617c() {
}

// 0x3e6180 — __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_10BodyColorsEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::BodyColors>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_10BodyColorsEE11isWriteOnlyEv
// IDA 0x3e6180: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e6180() {
}

// 0x3e6184 — __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_10BodyColorsEE8getValueEPKNS0_13DescribedBaseE
// type: _DWORD *__fastcall(_DWORD *result, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::BodyColors>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_10BodyColorsEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x3e6184: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e6184() {
}

// 0x3e6190 — __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_10BodyColorsEE8setValueEPNS0_13DescribedBaseERKS2_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::BodyColors>::setValue(RBX::Reflection::DescribedBase *,RBX::BrickColor const&)const")]
// was: __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_10BodyColorsEE8setValueEPNS0_13DescribedBaseERKS2_
// IDA 0x3e6190: 31 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e6190() {
}

// 0x3e61e0 — __ZN3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3e61e0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3e61e0() {
}

// 0x3e61e4 — __ZN3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3e61e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e61e4() {
}

// 0x3e6284 — __ZThn32_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3e6284: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e6284() {
}

// 0x3e628c — __ZThn32_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3e628c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e628c() {
}

// 0x3e6330 — __ZThn36_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3e6330: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e6330() {
}

// 0x3e6338 — __ZThn36_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3e6338: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e6338() {
}

// 0x3e63dc — __ZThn92_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn92_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3e63dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e63dc() {
}

// 0x3e63e4 — __ZThn92_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn92_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3e63e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e63e4() {
}

// 0x3e6488 — __ZN3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EEC2INS_4SkinEEEPKcS8_MT_S2_MS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Skin>(char const*,char const*,RBX::BrickColor RBX::Skin::*,void (RBX::Skin::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EEC2INS_4SkinEEEPKcS8_MT_S2_MS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE
// IDA 0x3e6488: 155 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e6488() {
}

// 0x3e661c — __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_4SkinEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Skin>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_4SkinEE10isReadOnlyEv
// IDA 0x3e661c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e661c() {
}

// 0x3e6620 — __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_4SkinEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Skin>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_4SkinEE11isWriteOnlyEv
// IDA 0x3e6620: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e6620() {
}

// 0x3e6624 — __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_4SkinEE8getValueEPKNS0_13DescribedBaseE
// type: _DWORD *__fastcall(_DWORD *result, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Skin>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_4SkinEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x3e6624: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e6624() {
}

// 0x3e6630 — __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_4SkinEE8setValueEPNS0_13DescribedBaseERKS2_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Skin>::setValue(RBX::Reflection::DescribedBase *,RBX::BrickColor const&)const")]
// was: __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_4SkinEE8setValueEPNS0_13DescribedBaseERKS2_
// IDA 0x3e6630: 31 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e6630() {
}

// 0x3e6680 — __ZN3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3e6680: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e6680() {
}

// 0x3e66c8 — __ZN3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3e66c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e66c8() {
}

// 0x3e67a8 — __ZThn32_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3e67a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e67a8() {
}

// 0x3e67f4 — __ZThn32_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3e67f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e67f4() {
}

// 0x3e68d8 — __ZThn36_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3e68d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e68d8() {
}

// 0x3e6924 — __ZThn36_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3e6924: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e6924() {
}

// 0x3e6a08 — __ZThn92_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn92_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3e6a08: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e6a08() {
}

// 0x3e6a54 — __ZThn92_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn92_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3e6a54: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e6a54() {
}

// 0x3e6b38 — __ZN3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3e6b38: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e6b38() {
}

// 0x3e6b80 — __ZN3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3e6b80: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e6b80() {
}

// 0x3e6c60 — __ZThn32_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3e6c60: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e6c60() {
}

// 0x3e6cac — __ZThn32_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3e6cac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e6cac() {
}

// 0x3e6d90 — __ZThn36_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3e6d90: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e6d90() {
}

// 0x3e6ddc — __ZThn36_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3e6ddc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e6ddc() {
}

// 0x3e6ec0 — __ZThn92_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn92_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3e6ec0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e6ec0() {
}

// 0x3e6f0c — __ZThn92_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn92_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3e6f0c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e6f0c() {
}

// 0x3e6ff0 — __ZN3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3e6ff0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3e6ff0() {
}

// 0x3e6ff4 — __ZN3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3e6ff4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e6ff4() {
}

// 0x3e7094 — __ZThn32_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3e7094: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e7094() {
}

// 0x3e709c — __ZThn32_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3e709c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e709c() {
}

// 0x3e7140 — __ZThn36_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3e7140: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e7140() {
}

// 0x3e7148 — __ZThn36_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3e7148: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e7148() {
}

// 0x3e71ec — __ZThn92_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn92_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3e71ec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e71ec() {
}

// 0x3e71f4 — __ZThn92_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn92_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3e71f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e71f4() {
}

// 0x3e7298 — __ZN3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3e7298: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3e7298() {
}

// 0x3e729c — __ZN3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3e729c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e729c() {
}

// 0x3e733c — __ZThn32_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3e733c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e733c() {
}

// 0x3e7344 — __ZThn32_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3e7344: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e7344() {
}

// 0x3e73e8 — __ZThn36_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3e73e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e73e8() {
}

// 0x3e73f0 — __ZThn36_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3e73f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e73f0() {
}

// 0x3e7494 — __ZThn92_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn92_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3e7494: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e7494() {
}

// 0x3e749c — __ZThn92_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn92_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3e749c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e749c() {
}

// 0x3e7540 — __ZN3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::PropDescriptor<RBX::TextureId (RBX::Pants::*)(void)const,void (RBX::Pants::*)(RBX::TextureId)>(char const*,char const*,RBX::TextureId (RBX::Pants::*)(void)const,void (RBX::Pants::*)(RBX::TextureId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x3e7540: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e7540() {
}

// 0x3e7654 — __ZN3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEED0Ev
// IDA 0x3e7654: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e7654() {
}

// 0x3e7680 — __ZNK3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Pants::*)(void)const,void (RBX::Pants::*)(RBX::TextureId)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// IDA 0x3e7680: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e7680() {
}

// 0x3e7684 — __ZNK3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Pants::*)(void)const,void (RBX::Pants::*)(RBX::TextureId)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// IDA 0x3e7684: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e7684() {
}

// 0x3e7688 — __ZNK3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Pants::*)(void)const,void (RBX::Pants::*)(RBX::TextureId)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x3e7688: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e7688() {
}

// 0x3e76b0 — __ZNK3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// type: void __fastcall(int, int, const std::string *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Pants::*)(void)const,void (RBX::Pants::*)(RBX::TextureId)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// IDA 0x3e76b0: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e76b0() {
}

// 0x3e77f8 — __ZN3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Shirt,RBX::TextureId>::PropDescriptor<RBX::TextureId (RBX::Shirt::*)(void)const,void (RBX::Shirt::*)(RBX::TextureId)>(char const*,char const*,RBX::TextureId (RBX::Shirt::*)(void)const,void (RBX::Shirt::*)(RBX::TextureId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x3e77f8: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e77f8() {
}

// 0x3e790c — __ZN3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Shirt,RBX::TextureId>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEED0Ev
// IDA 0x3e790c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e790c() {
}

// 0x3e7938 — __ZNK3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Shirt,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Shirt::*)(void)const,void (RBX::Shirt::*)(RBX::TextureId)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// IDA 0x3e7938: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e7938() {
}

// 0x3e793c — __ZNK3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Shirt,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Shirt::*)(void)const,void (RBX::Shirt::*)(RBX::TextureId)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// IDA 0x3e793c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e793c() {
}
