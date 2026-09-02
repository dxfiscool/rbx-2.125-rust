//! core watchdog e — 100 core stubs EA-sorted, gap filler after 0x3e3084 (watchdog_d max).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in core — next 100 uncovered after 0x3e3084.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "rbx_core::SharedPtr<RBX::BodyColors> RBX::Creatable<RBX::Instance>::create<RBX::BodyColors>(void)")]
// 0x3e31c8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10BodyColorsEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
pub fn stub_3e31c8() -> ! {
    todo!("0x3e31c8 __ZN3RBX9CreatableINS_8InstanceEE6createINS_10BodyColorsEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BodyColors>::shared_ptr<RBX::BodyColors,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyColors *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x3e3278 — __ZN5boost10shared_ptrIN3RBX10BodyColorsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
pub fn stub_3e3278() -> ! {
    todo!("0x3e3278 __ZN5boost10shared_ptrIN3RBX10BodyColorsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BodyColors,RBX::BodyColors>(rbx_core::SharedPtr<RBX::BodyColors> const*,RBX::BodyColors *)const")]
// 0x3e3340 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10BodyColorsES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
pub fn stub_3e3340() -> ! {
    todo!("0x3e3340 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10BodyColorsES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BodyColors *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyColors *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x3e3428 — __ZN5boost6detail12shared_countC2IPN3RBX10BodyColorsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
pub fn stub_3e3428() -> ! {
    todo!("0x3e3428 __ZN5boost6detail12shared_countC2IPN3RBX10BodyColorsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyColors *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x3e3530 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10BodyColorsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
pub fn stub_3e3530() -> ! {
    todo!("0x3e3530 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10BodyColorsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyColors *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x3e3534 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10BodyColorsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
pub fn stub_3e3534() -> ! {
    todo!("0x3e3534 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10BodyColorsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyColors *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x3e3538 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10BodyColorsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
pub fn stub_3e3538() -> ! {
    todo!("0x3e3538 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10BodyColorsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyColors *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x3e3558 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10BodyColorsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
pub fn stub_3e3558() -> ! {
    todo!("0x3e3558 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10BodyColorsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyColors *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x3e3570 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10BodyColorsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
pub fn stub_3e3570() -> ! {
    todo!("0x3e3570 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10BodyColorsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sBodyColorsEEEEvv")]
// 0x3e3574 — __ZN3RBX4Name13callDoDeclareILZNS_11sBodyColorsEEEEvv
pub fn stub_3e3574() -> ! {
    todo!("0x3e3574 __ZN3RBX4Name13callDoDeclareILZNS_11sBodyColorsEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sBodyColorsEEEERKS0_v")]
// 0x3e3578 — __ZN3RBX4Name9doDeclareILZNS_11sBodyColorsEEEERKS0_v
// type: int()
pub fn stub_3e3578() -> ! {
    todo!("0x3e3578 __ZN3RBX4Name9doDeclareILZNS_11sBodyColorsEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7CreatorC2Ev")]
// 0x3e3658 — __ZN3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
pub fn stub_3e3658() -> ! {
    todo!("0x3e3658 __ZN3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7CreatorC2Ev")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE17static_getCreatorEv")]
// 0x3e389c — __ZN3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE17static_getCreatorEv
// type: void *()
pub fn stub_3e389c() -> ! {
    todo!("0x3e389c __ZN3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE17static_getCreatorEv")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE7CreatorD2Ev")]
// 0x3e3910 — __ZN3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3e3910() -> ! {
    todo!("0x3e3910 __ZN3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE7CreatorD2Ev")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE7Creator12getClassNameEv")]
// 0x3e39ac — __ZNK3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE7Creator12getClassNameEv
pub fn stub_3e39ac() -> ! {
    todo!("0x3e39ac __ZNK3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE7Creator12getClassNameEv")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE7Creator6createEv")]
// 0x3e3a34 — __ZNK3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *)
pub fn stub_3e3a34() -> ! {
    todo!("0x3e3a34 __ZNK3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE7Creator6createEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Skin> RBX::Creatable<RBX::Instance>::create<RBX::Skin>(void)")]
// 0x3e3b78 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_4SkinEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
pub fn stub_3e3b78() -> ! {
    todo!("0x3e3b78 __ZN3RBX9CreatableINS_8InstanceEE6createINS_4SkinEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Skin>::shared_ptr<RBX::Skin,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Skin *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x3e3c28 — __ZN5boost10shared_ptrIN3RBX4SkinEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
pub fn stub_3e3c28() -> ! {
    todo!("0x3e3c28 __ZN5boost10shared_ptrIN3RBX4SkinEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Skin,RBX::Skin>(rbx_core::SharedPtr<RBX::Skin> const*,RBX::Skin *)const")]
// 0x3e3cf0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4SkinES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
pub fn stub_3e3cf0() -> ! {
    todo!("0x3e3cf0 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4SkinES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Skin *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Skin *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x3e3dd8 — __ZN5boost6detail12shared_countC2IPN3RBX4SkinENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
pub fn stub_3e3dd8() -> ! {
    todo!("0x3e3dd8 __ZN5boost6detail12shared_countC2IPN3RBX4SkinENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Skin *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x3e3ee0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SkinENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
pub fn stub_3e3ee0() -> ! {
    todo!("0x3e3ee0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SkinENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Skin *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x3e3ee4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SkinENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
pub fn stub_3e3ee4() -> ! {
    todo!("0x3e3ee4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SkinENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Skin *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x3e3ee8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SkinENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
pub fn stub_3e3ee8() -> ! {
    todo!("0x3e3ee8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SkinENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Skin *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x3e3f08 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SkinENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
pub fn stub_3e3f08() -> ! {
    todo!("0x3e3f08 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SkinENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Skin *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x3e3f20 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SkinENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
pub fn stub_3e3f20() -> ! {
    todo!("0x3e3f20 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SkinENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5sSkinEEEEvv")]
// 0x3e3f24 — __ZN3RBX4Name13callDoDeclareILZNS_5sSkinEEEEvv
pub fn stub_3e3f24() -> ! {
    todo!("0x3e3f24 __ZN3RBX4Name13callDoDeclareILZNS_5sSkinEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sSkinEEEERKS0_v")]
// 0x3e3f28 — __ZN3RBX4Name9doDeclareILZNS_5sSkinEEEERKS0_v
// type: int()
pub fn stub_3e3f28() -> ! {
    todo!("0x3e3f28 __ZN3RBX4Name9doDeclareILZNS_5sSkinEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE7CreatorC2Ev")]
// 0x3e4008 — __ZN3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
pub fn stub_3e4008() -> ! {
    todo!("0x3e4008 __ZN3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE7CreatorC2Ev")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE17static_getCreatorEv")]
// 0x3e424c — __ZN3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE17static_getCreatorEv
// type: void *()
pub fn stub_3e424c() -> ! {
    todo!("0x3e424c __ZN3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE17static_getCreatorEv")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sClothingEEEEvv")]
// 0x3e42c0 — __ZN3RBX4Name13callDoDeclareILZNS_9sClothingEEEEvv
pub fn stub_3e42c0() -> ! {
    todo!("0x3e42c0 __ZN3RBX4Name13callDoDeclareILZNS_9sClothingEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sClothingEEEERKS0_v")]
// 0x3e42c4 — __ZN3RBX4Name9doDeclareILZNS_9sClothingEEEERKS0_v
// type: int()
pub fn stub_3e42c4() -> ! {
    todo!("0x3e42c4 __ZN3RBX4Name9doDeclareILZNS_9sClothingEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7CreatorD2Ev")]
// 0x3e43a4 — __ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3e43a4() -> ! {
    todo!("0x3e43a4 __ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7CreatorD2Ev")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7Creator12getClassNameEv")]
// 0x3e4440 — __ZNK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7Creator12getClassNameEv
pub fn stub_3e4440() -> ! {
    todo!("0x3e4440 __ZNK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7Creator12getClassNameEv")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7Creator6createEv")]
// 0x3e44c8 — __ZNK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *)
pub fn stub_3e44c8() -> ! {
    todo!("0x3e44c8 __ZNK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7Creator6createEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ShirtGraphic> RBX::Creatable<RBX::Instance>::create<RBX::ShirtGraphic>(void)")]
// 0x3e460c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12ShirtGraphicEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
pub fn stub_3e460c() -> ! {
    todo!("0x3e460c __ZN3RBX9CreatableINS_8InstanceEE6createINS_12ShirtGraphicEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ShirtGraphic>::shared_ptr<RBX::ShirtGraphic,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ShirtGraphic *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x3e46bc — __ZN5boost10shared_ptrIN3RBX12ShirtGraphicEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
pub fn stub_3e46bc() -> ! {
    todo!("0x3e46bc __ZN5boost10shared_ptrIN3RBX12ShirtGraphicEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ShirtGraphic,RBX::ShirtGraphic>(rbx_core::SharedPtr<RBX::ShirtGraphic> const*,RBX::ShirtGraphic *)const")]
// 0x3e4784 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12ShirtGraphicES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
pub fn stub_3e4784() -> ! {
    todo!("0x3e4784 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12ShirtGraphicES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ShirtGraphic *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ShirtGraphic *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x3e486c — __ZN5boost6detail12shared_countC2IPN3RBX12ShirtGraphicENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
pub fn stub_3e486c() -> ! {
    todo!("0x3e486c __ZN5boost6detail12shared_countC2IPN3RBX12ShirtGraphicENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ShirtGraphic *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x3e4974 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12ShirtGraphicENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
pub fn stub_3e4974() -> ! {
    todo!("0x3e4974 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12ShirtGraphicENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ShirtGraphic *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x3e4978 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12ShirtGraphicENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
pub fn stub_3e4978() -> ! {
    todo!("0x3e4978 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12ShirtGraphicENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ShirtGraphic *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x3e497c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12ShirtGraphicENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
pub fn stub_3e497c() -> ! {
    todo!("0x3e497c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12ShirtGraphicENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ShirtGraphic *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x3e499c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12ShirtGraphicENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
pub fn stub_3e499c() -> ! {
    todo!("0x3e499c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12ShirtGraphicENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ShirtGraphic *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x3e49b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12ShirtGraphicENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
pub fn stub_3e49b4() -> ! {
    todo!("0x3e49b4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12ShirtGraphicENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sShirtGraphicEEEEvv")]
// 0x3e49b8 — __ZN3RBX4Name13callDoDeclareILZNS_13sShirtGraphicEEEEvv
pub fn stub_3e49b8() -> ! {
    todo!("0x3e49b8 __ZN3RBX4Name13callDoDeclareILZNS_13sShirtGraphicEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sShirtGraphicEEEERKS0_v")]
// 0x3e49bc — __ZN3RBX4Name9doDeclareILZNS_13sShirtGraphicEEEERKS0_v
// type: int()
pub fn stub_3e49bc() -> ! {
    todo!("0x3e49bc __ZN3RBX4Name9doDeclareILZNS_13sShirtGraphicEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7CreatorC2Ev")]
// 0x3e4a9c — __ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
pub fn stub_3e4a9c() -> ! {
    todo!("0x3e4a9c __ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7CreatorC2Ev")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE17static_getCreatorEv")]
// 0x3e4ce0 — __ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE17static_getCreatorEv
// type: void *()
pub fn stub_3e4ce0() -> ! {
    todo!("0x3e4ce0 __ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE17static_getCreatorEv")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7CreatorD2Ev")]
// 0x3e4d54 — __ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3e4d54() -> ! {
    todo!("0x3e4d54 __ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7CreatorD2Ev")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7Creator6createEv")]
// 0x3e4df0 — __ZNK3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *)
pub fn stub_3e4df0() -> ! {
    todo!("0x3e4df0 __ZNK3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7Creator6createEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Pants> RBX::Creatable<RBX::Instance>::create<RBX::Pants>(void)")]
// 0x3e4f34 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5PantsEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
pub fn stub_3e4f34() -> ! {
    todo!("0x3e4f34 __ZN3RBX9CreatableINS_8InstanceEE6createINS_5PantsEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Pants>::shared_ptr<RBX::Pants,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x3e4fe4 — __ZN5boost10shared_ptrIN3RBX5PantsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
pub fn stub_3e4fe4() -> ! {
    todo!("0x3e4fe4 __ZN5boost10shared_ptrIN3RBX5PantsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Pants,RBX::Pants>(rbx_core::SharedPtr<RBX::Pants> const*,RBX::Pants *)const")]
// 0x3e50ac — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5PantsES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
pub fn stub_3e50ac() -> ! {
    todo!("0x3e50ac __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5PantsES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x3e5194 — __ZN5boost6detail12shared_countC2IPN3RBX5PantsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
pub fn stub_3e5194() -> ! {
    todo!("0x3e5194 __ZN5boost6detail12shared_countC2IPN3RBX5PantsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x3e529c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5PantsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
pub fn stub_3e529c() -> ! {
    todo!("0x3e529c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5PantsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x3e52a0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5PantsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
pub fn stub_3e52a0() -> ! {
    todo!("0x3e52a0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5PantsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x3e52a4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5PantsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
pub fn stub_3e52a4() -> ! {
    todo!("0x3e52a4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5PantsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x3e52c4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5PantsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
pub fn stub_3e52c4() -> ! {
    todo!("0x3e52c4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5PantsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x3e52dc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5PantsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
pub fn stub_3e52dc() -> ! {
    todo!("0x3e52dc __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5PantsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7CreatorC2Ev")]
// 0x3e52e0 — __ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
pub fn stub_3e52e0() -> ! {
    todo!("0x3e52e0 __ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7CreatorC2Ev")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7CreatorD2Ev")]
// 0x3e5524 — __ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3e5524() -> ! {
    todo!("0x3e5524 __ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7CreatorD2Ev")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7Creator6createEv")]
// 0x3e55c0 — __ZNK3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *)
pub fn stub_3e55c0() -> ! {
    todo!("0x3e55c0 __ZNK3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7Creator6createEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Shirt> RBX::Creatable<RBX::Instance>::create<RBX::Shirt>(void)")]
// 0x3e5704 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ShirtEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
pub fn stub_3e5704() -> ! {
    todo!("0x3e5704 __ZN3RBX9CreatableINS_8InstanceEE6createINS_5ShirtEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Shirt>::shared_ptr<RBX::Shirt,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x3e57b4 — __ZN5boost10shared_ptrIN3RBX5ShirtEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
pub fn stub_3e57b4() -> ! {
    todo!("0x3e57b4 __ZN5boost10shared_ptrIN3RBX5ShirtEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Shirt,RBX::Shirt>(rbx_core::SharedPtr<RBX::Shirt> const*,RBX::Shirt *)const")]
// 0x3e587c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5ShirtES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
pub fn stub_3e587c() -> ! {
    todo!("0x3e587c __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5ShirtES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x3e5964 — __ZN5boost6detail12shared_countC2IPN3RBX5ShirtENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
pub fn stub_3e5964() -> ! {
    todo!("0x3e5964 __ZN5boost6detail12shared_countC2IPN3RBX5ShirtENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x3e5a6c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ShirtENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
pub fn stub_3e5a6c() -> ! {
    todo!("0x3e5a6c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ShirtENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x3e5a70 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ShirtENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
pub fn stub_3e5a70() -> ! {
    todo!("0x3e5a70 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ShirtENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x3e5a74 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ShirtENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
pub fn stub_3e5a74() -> ! {
    todo!("0x3e5a74 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ShirtENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x3e5a94 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ShirtENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
pub fn stub_3e5a94() -> ! {
    todo!("0x3e5a94 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ShirtENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x3e5aac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ShirtENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
pub fn stub_3e5aac() -> ! {
    todo!("0x3e5aac __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5ShirtENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7CreatorC2Ev")]
// 0x3e5ab0 — __ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
pub fn stub_3e5ab0() -> ! {
    todo!("0x3e5ab0 __ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7CreatorC2Ev")
}

#[doc(alias = "RBX::Skin * RBX::ModelInstance::findFirstModifierOfType<RBX::Skin>(void)")]
// 0x3e5cf4 — __ZN3RBX13ModelInstance23findFirstModifierOfTypeINS_4SkinEEEPT_v
// type: void *__fastcall(int)
pub fn stub_3e5cf4() -> ! {
    todo!("0x3e5cf4 __ZN3RBX13ModelInstance23findFirstModifierOfTypeINS_4SkinEEEPT_v")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3e5d40 — __ZN3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
pub fn stub_3e5d40() -> ! {
    todo!("0x3e5d40 __ZN3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3e5d44 — __ZN3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
pub fn stub_3e5d44() -> ! {
    todo!("0x3e5d44 __ZN3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3e5de4 — __ZThn32_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_3e5de4() -> ! {
    todo!("0x3e5de4 __ZThn32_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3e5dec — __ZThn32_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub fn stub_3e5dec() -> ! {
    todo!("0x3e5dec __ZThn32_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3e5e90 — __ZThn36_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_3e5e90() -> ! {
    todo!("0x3e5e90 __ZThn36_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3e5e98 — __ZThn36_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub fn stub_3e5e98() -> ! {
    todo!("0x3e5e98 __ZThn36_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3e5f3c — __ZThn92_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_3e5f3c() -> ! {
    todo!("0x3e5f3c __ZThn92_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3e5f44 — __ZThn92_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub fn stub_3e5f44() -> ! {
    todo!("0x3e5f44 __ZThn92_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundProp<RBX::BodyColors>(char const*,char const*,RBX::BrickColor RBX::BodyColors::*,void (RBX::BodyColors::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x3e5fe8 — __ZN3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EEC2INS_10BodyColorsEEEPKcS8_MT_S2_MS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
pub fn stub_3e5fe8() -> ! {
    todo!("0x3e5fe8 __ZN3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EEC2INS_10BodyColorsEEEPKcS8_MT_S2_MS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE")
}

#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::BodyColors>::isReadOnly(void)const")]
// 0x3e617c — __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_10BodyColorsEE10isReadOnlyEv
// type: int()
pub fn stub_3e617c() -> ! {
    todo!("0x3e617c __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_10BodyColorsEE10isReadOnlyEv")
}

#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::BodyColors>::isWriteOnly(void)const")]
// 0x3e6180 — __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_10BodyColorsEE11isWriteOnlyEv
// type: int()
pub fn stub_3e6180() -> ! {
    todo!("0x3e6180 __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_10BodyColorsEE11isWriteOnlyEv")
}

#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::BodyColors>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x3e6184 — __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_10BodyColorsEE8getValueEPKNS0_13DescribedBaseE
// type: _DWORD *__fastcall(_DWORD *result, int, int)
pub fn stub_3e6184() -> ! {
    todo!("0x3e6184 __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_10BodyColorsEE8getValueEPKNS0_13DescribedBaseE")
}

#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::BodyColors>::setValue(RBX::Reflection::DescribedBase *,RBX::BrickColor const&)const")]
// 0x3e6190 — __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_10BodyColorsEE8setValueEPNS0_13DescribedBaseERKS2_
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_3e6190() -> ! {
    todo!("0x3e6190 __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_10BodyColorsEE8setValueEPNS0_13DescribedBaseERKS2_")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3e61e0 — __ZN3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
pub fn stub_3e61e0() -> ! {
    todo!("0x3e61e0 __ZN3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3e61e4 — __ZN3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
pub fn stub_3e61e4() -> ! {
    todo!("0x3e61e4 __ZN3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3e6284 — __ZThn32_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_3e6284() -> ! {
    todo!("0x3e6284 __ZThn32_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3e628c — __ZThn32_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub fn stub_3e628c() -> ! {
    todo!("0x3e628c __ZThn32_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3e6330 — __ZThn36_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_3e6330() -> ! {
    todo!("0x3e6330 __ZThn36_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3e6338 — __ZThn36_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub fn stub_3e6338() -> ! {
    todo!("0x3e6338 __ZThn36_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3e63dc — __ZThn92_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_3e63dc() -> ! {
    todo!("0x3e63dc __ZThn92_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3e63e4 — __ZThn92_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub fn stub_3e63e4() -> ! {
    todo!("0x3e63e4 __ZThn92_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Skin>(char const*,char const*,RBX::BrickColor RBX::Skin::*,void (RBX::Skin::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x3e6488 — __ZN3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EEC2INS_4SkinEEEPKcS8_MT_S2_MS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
pub fn stub_3e6488() -> ! {
    todo!("0x3e6488 __ZN3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EEC2INS_4SkinEEEPKcS8_MT_S2_MS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE")
}

#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Skin>::isReadOnly(void)const")]
// 0x3e661c — __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_4SkinEE10isReadOnlyEv
// type: int()
pub fn stub_3e661c() -> ! {
    todo!("0x3e661c __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_4SkinEE10isReadOnlyEv")
}

#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Skin>::isWriteOnly(void)const")]
// 0x3e6620 — __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_4SkinEE11isWriteOnlyEv
// type: int()
pub fn stub_3e6620() -> ! {
    todo!("0x3e6620 __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_4SkinEE11isWriteOnlyEv")
}

#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Skin>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x3e6624 — __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_4SkinEE8getValueEPKNS0_13DescribedBaseE
// type: _DWORD *__fastcall(_DWORD *result, int, int)
pub fn stub_3e6624() -> ! {
    todo!("0x3e6624 __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_4SkinEE8getValueEPKNS0_13DescribedBaseE")
}

#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Skin>::setValue(RBX::Reflection::DescribedBase *,RBX::BrickColor const&)const")]
// 0x3e6630 — __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_4SkinEE8setValueEPNS0_13DescribedBaseERKS2_
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_3e6630() -> ! {
    todo!("0x3e6630 __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_4SkinEE8setValueEPNS0_13DescribedBaseERKS2_")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3e6680 — __ZN3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_3e6680() -> ! {
    todo!("0x3e6680 __ZN3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3e66c8 — __ZN3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3e66c8() -> ! {
    todo!("0x3e66c8 __ZN3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}
