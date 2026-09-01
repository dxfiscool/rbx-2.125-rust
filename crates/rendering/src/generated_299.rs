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
pub fn stub_3e497c() -> ! {
    todo!("0x3e497c boost::detail::sp_counted_impl_pd<RBX::ShirtGraphic *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x3e499c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12ShirtGraphicENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ShirtGraphic *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12ShirtGraphicENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_3e499c() -> ! {
    todo!("0x3e499c boost::detail::sp_counted_impl_pd<RBX::ShirtGraphic *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x3e49b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12ShirtGraphicENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ShirtGraphic *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12ShirtGraphicENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_3e49b4() -> ! {
    todo!("0x3e49b4 boost::detail::sp_counted_impl_pd<RBX::ShirtGraphic *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x3e49b8 — __ZN3RBX4Name13callDoDeclareILZNS_13sShirtGraphicEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sShirtGraphicEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_13sShirtGraphicEEEEvv
pub fn stub_3e49b8() -> ! {
    todo!("0x3e49b8 __ZN3RBX4Name13callDoDeclareILZNS_13sShirtGraphicEEEEvv")
}

// 0x3e49bc — __ZN3RBX4Name9doDeclareILZNS_13sShirtGraphicEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sShirtGraphicEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_13sShirtGraphicEEEERKS0_v
pub fn stub_3e49bc() -> ! {
    todo!("0x3e49bc __ZN3RBX4Name9doDeclareILZNS_13sShirtGraphicEEEERKS0_v")
}

// 0x3e4a9c — __ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7CreatorC2Ev
pub fn stub_3e4a9c() -> ! {
    todo!("0x3e4a9c __ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7CreatorC2Ev")
}

// 0x3e4ce0 — __ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE17static_getCreatorEv
pub fn stub_3e4ce0() -> ! {
    todo!("0x3e4ce0 __ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE17static_getCreatorEv")
}

// 0x3e4d54 — __ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7CreatorD2Ev
pub fn stub_3e4d54() -> ! {
    todo!("0x3e4d54 __ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7CreatorD2Ev")
}

// 0x3e4df0 — __ZNK3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7Creator6createEv
pub fn stub_3e4df0() -> ! {
    todo!("0x3e4df0 __ZNK3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7Creator6createEv")
}

// 0x3e4f34 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5PantsEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "boost::shared_ptr<RBX::Pants> RBX::Creatable<RBX::Instance>::create<RBX::Pants>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_5PantsEEEN5boost10shared_ptrIT_EEv
pub fn stub_3e4f34() -> ! {
    todo!("0x3e4f34 boost::shared_ptr<RBX::Pants> RBX::Creatable<RBX::Instance>::create<RBX::Pants>(void)")
}

// 0x3e4fe4 — __ZN5boost10shared_ptrIN3RBX5PantsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::Pants>::shared_ptr<RBX::Pants,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX5PantsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_3e4fe4() -> ! {
    todo!("0x3e4fe4 boost::shared_ptr<RBX::Pants>::shared_ptr<RBX::Pants,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x3e50ac — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5PantsES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Pants,RBX::Pants>(boost::shared_ptr<RBX::Pants> const*,RBX::Pants *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5PantsES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_3e50ac() -> ! {
    todo!("0x3e50ac void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Pants,RBX::Pants>(boost::shared_ptr<RBX::Pants> const*,RBX::Pants *)const")
}

// 0x3e5194 — __ZN5boost6detail12shared_countC2IPN3RBX5PantsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX5PantsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_3e5194() -> ! {
    todo!("0x3e5194 boost::detail::shared_count::shared_count<RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x3e529c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5PantsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5PantsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_3e529c() -> ! {
    todo!("0x3e529c boost::detail::sp_counted_impl_pd<RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x3e52a0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5PantsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5PantsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_3e52a0() -> ! {
    todo!("0x3e52a0 boost::detail::sp_counted_impl_pd<RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x3e52a4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5PantsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5PantsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_3e52a4() -> ! {
    todo!("0x3e52a4 boost::detail::sp_counted_impl_pd<RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x3e52c4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5PantsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5PantsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_3e52c4() -> ! {
    todo!("0x3e52c4 boost::detail::sp_counted_impl_pd<RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x3e52dc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5PantsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5PantsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_3e52dc() -> ! {
    todo!("0x3e52dc boost::detail::sp_counted_impl_pd<RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x3e52e0 — __ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7CreatorC2Ev
pub fn stub_3e52e0() -> ! {
    todo!("0x3e52e0 __ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7CreatorC2Ev")
}

// 0x3e5524 — __ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7CreatorD2Ev
pub fn stub_3e5524() -> ! {
    todo!("0x3e5524 __ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7CreatorD2Ev")
}

// 0x3e55c0 — __ZNK3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7Creator6createEv
pub fn stub_3e55c0() -> ! {
    todo!("0x3e55c0 __ZNK3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7Creator6createEv")
}

// 0x3e5704 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ShirtEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "boost::shared_ptr<RBX::Shirt> RBX::Creatable<RBX::Instance>::create<RBX::Shirt>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ShirtEEEN5boost10shared_ptrIT_EEv
pub fn stub_3e5704() -> ! {
    todo!("0x3e5704 boost::shared_ptr<RBX::Shirt> RBX::Creatable<RBX::Instance>::create<RBX::Shirt>(void)")
}

// 0x3e57b4 — __ZN5boost10shared_ptrIN3RBX5ShirtEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::Shirt>::shared_ptr<RBX::Shirt,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX5ShirtEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_3e57b4() -> ! {
    todo!("0x3e57b4 boost::shared_ptr<RBX::Shirt>::shared_ptr<RBX::Shirt,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x3e587c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5ShirtES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Shirt,RBX::Shirt>(boost::shared_ptr<RBX::Shirt> const*,RBX::Shirt *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5ShirtES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_3e587c() -> ! {
    todo!("0x3e587c void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Shirt,RBX::Shirt>(boost::shared_ptr<RBX::Shirt> const*,RBX::Shirt *)const")
}

// 0x3e5964 — __ZN5boost6detail12shared_countC2IPN3RBX5ShirtENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX5ShirtENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_3e5964() -> ! {
    todo!("0x3e5964 boost::detail::shared_count::shared_count<RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x3e5a6c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ShirtENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ShirtENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_3e5a6c() -> ! {
    todo!("0x3e5a6c boost::detail::sp_counted_impl_pd<RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x3e5a70 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ShirtENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ShirtENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_3e5a70() -> ! {
    todo!("0x3e5a70 boost::detail::sp_counted_impl_pd<RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x3e5a74 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ShirtENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ShirtENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_3e5a74() -> ! {
    todo!("0x3e5a74 boost::detail::sp_counted_impl_pd<RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x3e5a94 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ShirtENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ShirtENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_3e5a94() -> ! {
    todo!("0x3e5a94 boost::detail::sp_counted_impl_pd<RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x3e5aac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ShirtENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ShirtENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_3e5aac() -> ! {
    todo!("0x3e5aac boost::detail::sp_counted_impl_pd<RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x3e5ab0 — __ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7CreatorC2Ev
pub fn stub_3e5ab0() -> ! {
    todo!("0x3e5ab0 __ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7CreatorC2Ev")
}

// 0x3e5cf4 — __ZN3RBX13ModelInstance23findFirstModifierOfTypeINS_4SkinEEEPT_v
// type: void *__fastcall(int)
#[doc(alias = "RBX::Skin * RBX::ModelInstance::findFirstModifierOfType<RBX::Skin>(void)")]
// was: __ZN3RBX13ModelInstance23findFirstModifierOfTypeINS_4SkinEEEPT_v
pub fn stub_3e5cf4() -> ! {
    todo!("0x3e5cf4 RBX::Skin * RBX::ModelInstance::findFirstModifierOfType<RBX::Skin>(void)")
}

// 0x3e5d40 — __ZN3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3e5d40() -> ! {
    todo!("0x3e5d40 __ZN3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3e5d44 — __ZN3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3e5d44() -> ! {
    todo!("0x3e5d44 __ZN3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3e5de4 — __ZThn32_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3e5de4() -> ! {
    todo!("0x3e5de4 __ZThn32_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3e5dec — __ZThn32_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3e5dec() -> ! {
    todo!("0x3e5dec __ZThn32_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3e5e90 — __ZThn36_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3e5e90() -> ! {
    todo!("0x3e5e90 __ZThn36_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3e5e98 — __ZThn36_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3e5e98() -> ! {
    todo!("0x3e5e98 __ZThn36_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3e5f3c — __ZThn92_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn92_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3e5f3c() -> ! {
    todo!("0x3e5f3c __ZThn92_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3e5f44 — __ZThn92_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn92_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3e5f44() -> ! {
    todo!("0x3e5f44 __ZThn92_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3e5fe8 — __ZN3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EEC2INS_10BodyColorsEEEPKcS8_MT_S2_MS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundProp<RBX::BodyColors>(char const*,char const*,RBX::BrickColor RBX::BodyColors::*,void (RBX::BodyColors::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EEC2INS_10BodyColorsEEEPKcS8_MT_S2_MS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE
pub fn stub_3e5fe8() -> ! {
    todo!("0x3e5fe8 RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundProp<RBX::BodyColors>(char const*,char const*,RBX::BrickColor RBX::BodyColors::*,void (RBX::BodyColors::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x3e617c — __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_10BodyColorsEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::BodyColors>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_10BodyColorsEE10isReadOnlyEv
pub fn stub_3e617c() -> ! {
    todo!("0x3e617c RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::BodyColors>::isReadOnly(void)const")
}

// 0x3e6180 — __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_10BodyColorsEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::BodyColors>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_10BodyColorsEE11isWriteOnlyEv
pub fn stub_3e6180() -> ! {
    todo!("0x3e6180 RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::BodyColors>::isWriteOnly(void)const")
}

// 0x3e6184 — __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_10BodyColorsEE8getValueEPKNS0_13DescribedBaseE
// type: _DWORD *__fastcall(_DWORD *result, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::BodyColors>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_10BodyColorsEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_3e6184() -> ! {
    todo!("0x3e6184 RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::BodyColors>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x3e6190 — __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_10BodyColorsEE8setValueEPNS0_13DescribedBaseERKS2_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::BodyColors>::setValue(RBX::Reflection::DescribedBase *,RBX::BrickColor const&)const")]
// was: __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_10BodyColorsEE8setValueEPNS0_13DescribedBaseERKS2_
pub fn stub_3e6190() -> ! {
    todo!("0x3e6190 RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::BodyColors>::setValue(RBX::Reflection::DescribedBase *,RBX::BrickColor const&)const")
}

// 0x3e61e0 — __ZN3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3e61e0() -> ! {
    todo!("0x3e61e0 __ZN3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3e61e4 — __ZN3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3e61e4() -> ! {
    todo!("0x3e61e4 __ZN3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3e6284 — __ZThn32_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3e6284() -> ! {
    todo!("0x3e6284 __ZThn32_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3e628c — __ZThn32_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3e628c() -> ! {
    todo!("0x3e628c __ZThn32_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3e6330 — __ZThn36_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3e6330() -> ! {
    todo!("0x3e6330 __ZThn36_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3e6338 — __ZThn36_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3e6338() -> ! {
    todo!("0x3e6338 __ZThn36_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3e63dc — __ZThn92_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn92_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3e63dc() -> ! {
    todo!("0x3e63dc __ZThn92_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3e63e4 — __ZThn92_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn92_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3e63e4() -> ! {
    todo!("0x3e63e4 __ZThn92_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3e6488 — __ZN3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EEC2INS_4SkinEEEPKcS8_MT_S2_MS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Skin>(char const*,char const*,RBX::BrickColor RBX::Skin::*,void (RBX::Skin::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EEC2INS_4SkinEEEPKcS8_MT_S2_MS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE
pub fn stub_3e6488() -> ! {
    todo!("0x3e6488 RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Skin>(char const*,char const*,RBX::BrickColor RBX::Skin::*,void (RBX::Skin::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x3e661c — __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_4SkinEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Skin>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_4SkinEE10isReadOnlyEv
pub fn stub_3e661c() -> ! {
    todo!("0x3e661c RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Skin>::isReadOnly(void)const")
}

// 0x3e6620 — __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_4SkinEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Skin>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_4SkinEE11isWriteOnlyEv
pub fn stub_3e6620() -> ! {
    todo!("0x3e6620 RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Skin>::isWriteOnly(void)const")
}

// 0x3e6624 — __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_4SkinEE8getValueEPKNS0_13DescribedBaseE
// type: _DWORD *__fastcall(_DWORD *result, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Skin>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_4SkinEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_3e6624() -> ! {
    todo!("0x3e6624 RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Skin>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x3e6630 — __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_4SkinEE8setValueEPNS0_13DescribedBaseERKS2_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Skin>::setValue(RBX::Reflection::DescribedBase *,RBX::BrickColor const&)const")]
// was: __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_4SkinEE8setValueEPNS0_13DescribedBaseERKS2_
pub fn stub_3e6630() -> ! {
    todo!("0x3e6630 RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Skin>::setValue(RBX::Reflection::DescribedBase *,RBX::BrickColor const&)const")
}

// 0x3e6680 — __ZN3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3e6680() -> ! {
    todo!("0x3e6680 __ZN3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3e66c8 — __ZN3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3e66c8() -> ! {
    todo!("0x3e66c8 __ZN3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3e67a8 — __ZThn32_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3e67a8() -> ! {
    todo!("0x3e67a8 __ZThn32_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3e67f4 — __ZThn32_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3e67f4() -> ! {
    todo!("0x3e67f4 __ZThn32_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3e68d8 — __ZThn36_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3e68d8() -> ! {
    todo!("0x3e68d8 __ZThn36_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3e6924 — __ZThn36_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3e6924() -> ! {
    todo!("0x3e6924 __ZThn36_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3e6a08 — __ZThn92_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn92_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3e6a08() -> ! {
    todo!("0x3e6a08 __ZThn92_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3e6a54 — __ZThn92_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn92_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3e6a54() -> ! {
    todo!("0x3e6a54 __ZThn92_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3e6b38 — __ZN3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3e6b38() -> ! {
    todo!("0x3e6b38 __ZN3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3e6b80 — __ZN3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3e6b80() -> ! {
    todo!("0x3e6b80 __ZN3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3e6c60 — __ZThn32_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3e6c60() -> ! {
    todo!("0x3e6c60 __ZThn32_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3e6cac — __ZThn32_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3e6cac() -> ! {
    todo!("0x3e6cac __ZThn32_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3e6d90 — __ZThn36_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3e6d90() -> ! {
    todo!("0x3e6d90 __ZThn36_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3e6ddc — __ZThn36_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3e6ddc() -> ! {
    todo!("0x3e6ddc __ZThn36_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3e6ec0 — __ZThn92_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn92_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3e6ec0() -> ! {
    todo!("0x3e6ec0 __ZThn92_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3e6f0c — __ZThn92_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn92_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3e6f0c() -> ! {
    todo!("0x3e6f0c __ZThn92_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3e6ff0 — __ZN3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3e6ff0() -> ! {
    todo!("0x3e6ff0 __ZN3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3e6ff4 — __ZN3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3e6ff4() -> ! {
    todo!("0x3e6ff4 __ZN3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3e7094 — __ZThn32_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3e7094() -> ! {
    todo!("0x3e7094 __ZThn32_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3e709c — __ZThn32_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3e709c() -> ! {
    todo!("0x3e709c __ZThn32_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3e7140 — __ZThn36_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3e7140() -> ! {
    todo!("0x3e7140 __ZThn36_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3e7148 — __ZThn36_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3e7148() -> ! {
    todo!("0x3e7148 __ZThn36_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3e71ec — __ZThn92_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn92_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3e71ec() -> ! {
    todo!("0x3e71ec __ZThn92_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3e71f4 — __ZThn92_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn92_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3e71f4() -> ! {
    todo!("0x3e71f4 __ZThn92_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3e7298 — __ZN3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3e7298() -> ! {
    todo!("0x3e7298 __ZN3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3e729c — __ZN3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3e729c() -> ! {
    todo!("0x3e729c __ZN3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3e733c — __ZThn32_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3e733c() -> ! {
    todo!("0x3e733c __ZThn32_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3e7344 — __ZThn32_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3e7344() -> ! {
    todo!("0x3e7344 __ZThn32_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3e73e8 — __ZThn36_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3e73e8() -> ! {
    todo!("0x3e73e8 __ZThn36_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3e73f0 — __ZThn36_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3e73f0() -> ! {
    todo!("0x3e73f0 __ZThn36_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3e7494 — __ZThn92_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn92_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3e7494() -> ! {
    todo!("0x3e7494 __ZThn92_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3e749c — __ZThn92_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn92_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3e749c() -> ! {
    todo!("0x3e749c __ZThn92_N3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3e7540 — __ZN3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::PropDescriptor<RBX::TextureId (RBX::Pants::*)(void)const,void (RBX::Pants::*)(RBX::TextureId)>(char const*,char const*,RBX::TextureId (RBX::Pants::*)(void)const,void (RBX::Pants::*)(RBX::TextureId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_3e7540() -> ! {
    todo!("0x3e7540 RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::PropDescriptor<RBX::TextureId (RBX::Pants::*)(void)const,void (RBX::Pants::*)(RBX::TextureId)>(char const*,char const*,RBX::TextureId (RBX::Pants::*)(void)const,void (RBX::Pants::*)(RBX::TextureId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x3e7654 — __ZN3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEED0Ev
pub fn stub_3e7654() -> ! {
    todo!("0x3e7654 RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::~PropDescriptor()")
}

// 0x3e7680 — __ZNK3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Pants::*)(void)const,void (RBX::Pants::*)(RBX::TextureId)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
pub fn stub_3e7680() -> ! {
    todo!("0x3e7680 RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Pants::*)(void)const,void (RBX::Pants::*)(RBX::TextureId)>::isReadOnly(void)const")
}

// 0x3e7684 — __ZNK3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Pants::*)(void)const,void (RBX::Pants::*)(RBX::TextureId)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
pub fn stub_3e7684() -> ! {
    todo!("0x3e7684 RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Pants::*)(void)const,void (RBX::Pants::*)(RBX::TextureId)>::isWriteOnly(void)const")
}

// 0x3e7688 — __ZNK3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Pants::*)(void)const,void (RBX::Pants::*)(RBX::TextureId)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
pub fn stub_3e7688() -> ! {
    todo!("0x3e7688 RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Pants::*)(void)const,void (RBX::Pants::*)(RBX::TextureId)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x3e76b0 — __ZNK3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// type: void __fastcall(int, int, const std::string *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Pants::*)(void)const,void (RBX::Pants::*)(RBX::TextureId)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
pub fn stub_3e76b0() -> ! {
    todo!("0x3e76b0 RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Pants::*)(void)const,void (RBX::Pants::*)(RBX::TextureId)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const")
}

// 0x3e77f8 — __ZN3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Shirt,RBX::TextureId>::PropDescriptor<RBX::TextureId (RBX::Shirt::*)(void)const,void (RBX::Shirt::*)(RBX::TextureId)>(char const*,char const*,RBX::TextureId (RBX::Shirt::*)(void)const,void (RBX::Shirt::*)(RBX::TextureId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_3e77f8() -> ! {
    todo!("0x3e77f8 RBX::Reflection::PropDescriptor<RBX::Shirt,RBX::TextureId>::PropDescriptor<RBX::TextureId (RBX::Shirt::*)(void)const,void (RBX::Shirt::*)(RBX::TextureId)>(char const*,char const*,RBX::TextureId (RBX::Shirt::*)(void)const,void (RBX::Shirt::*)(RBX::TextureId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x3e790c — __ZN3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Shirt,RBX::TextureId>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEED0Ev
pub fn stub_3e790c() -> ! {
    todo!("0x3e790c RBX::Reflection::PropDescriptor<RBX::Shirt,RBX::TextureId>::~PropDescriptor()")
}

// 0x3e7938 — __ZNK3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Shirt,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Shirt::*)(void)const,void (RBX::Shirt::*)(RBX::TextureId)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
pub fn stub_3e7938() -> ! {
    todo!("0x3e7938 RBX::Reflection::PropDescriptor<RBX::Shirt,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Shirt::*)(void)const,void (RBX::Shirt::*)(RBX::TextureId)>::isReadOnly(void)const")
}

// 0x3e793c — __ZNK3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Shirt,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Shirt::*)(void)const,void (RBX::Shirt::*)(RBX::TextureId)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
pub fn stub_3e793c() -> ! {
    todo!("0x3e793c RBX::Reflection::PropDescriptor<RBX::Shirt,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Shirt::*)(void)const,void (RBX::Shirt::*)(RBX::TextureId)>::isWriteOnly(void)const")
}
