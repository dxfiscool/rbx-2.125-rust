//! core-C: 150 boost stubs — filtered boost:: namespace.
//! EA-ordered boost stubs (0x48a364..0x4ac008) so `cargo check` stays green.
//! Source: `ida/export.json` filtered where mangled/demangled contains "boost", sorted by EA, next 150 after 4881 already covered.
//! Each stub preserves IDA address, mangled symbol, and demangled spelling; signatures use `rbx_core::SharedPtr` not `boost::`.


#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x48a364 — __ZNK3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFN5boost10shared_ptrIKNS0_5TupleEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// was: RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,boost::shared_ptr<RBX::Reflection::Tuple const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub fn stub_48a364() -> ! {
    todo!("0x48a364 __ZNK3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFN5boost10shared_ptrIKNS0_5TupleEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")
}

#[doc(alias = "RBX::Reflection::Call0Helper<RBX::DebugSettings,rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::DebugSettings::*)(void),rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::call(RBX::DebugSettings*,rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::DebugSettings::*)(void),RBX::Reflection::Variant &)")]
// 0x48a388 — __ZN3RBX10Reflection11Call0HelperINS_13DebugSettingsEMS2_FN5boost10shared_ptrIKNS0_5TupleEEEvES7_E4callEPS2_S9_RNS0_7VariantE
// was: RBX::Reflection::Call0Helper<RBX::DebugSettings,boost::shared_ptr<RBX::Reflection::Tuple const> (RBX::DebugSettings::*)(void),boost::shared_ptr<RBX::Reflection::Tuple const>>::call(RBX::DebugSettings*,boost::shared_ptr<RBX::Reflection::Tuple const> (RBX::DebugSettings::*)(void),RBX::Reflection::Variant &)
pub fn stub_48a388() -> ! {
    todo!("0x48a388 __ZN3RBX10Reflection11Call0HelperINS_13DebugSettingsEMS2_FN5boost10shared_ptrIKNS0_5TupleEEEvES7_E4callEPS2_S9_RNS0_7VariantE")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>(rbx_core::SharedPtr<RBX::Reflection::Tuple const> const&)")]
// 0x48a470 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIN5boost10shared_ptrIKNS1_10Reflection5TupleEEEEERS3_RKT_
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<boost::shared_ptr<RBX::Reflection::Tuple const>>(boost::shared_ptr<RBX::Reflection::Tuple const> const&)
pub fn stub_48a470() -> ! {
    todo!("0x48a470 __ZN3rbx13placement_anyIN3RBX7Region3EEaSIN5boost10shared_ptrIKNS1_10Reflection5TupleEEEEERS3_RKT_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::Tuple const>::operator=(rbx_core::SharedPtr<RBX::Reflection::Tuple const> const&)")]
// 0x48a4d8 — __ZN5boost10shared_ptrIKN3RBX10Reflection5TupleEEaSERKS5_
// was: boost::shared_ptr<RBX::Reflection::Tuple const>::operator=(boost::shared_ptr<RBX::Reflection::Tuple const> const&)
pub fn stub_48a4d8() -> ! {
    todo!("0x48a4d8 __ZN5boost10shared_ptrIKN3RBX10Reflection5TupleEEaSERKS5_")
}

#[doc(alias = "rbx::implementation::typed_holder<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::construct_func(char const*,char *)")]
// 0x48a510 — __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEE14construct_funcEPKcPc
// was: rbx::implementation::typed_holder<boost::shared_ptr<RBX::Reflection::Tuple const>>::construct_func(char const*,char *)
pub fn stub_48a510() -> ! {
    todo!("0x48a510 __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEE14construct_funcEPKcPc")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::BlockCorners,96u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0x48d970 — __ZN5boost14singleton_poolIN3RBX4POLY12BlockCornersELj96ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// was: boost::singleton_pool<RBX::POLY::BlockCorners,96u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)
pub fn stub_48d970() -> ! {
    todo!("0x48d970 __ZN5boost14singleton_poolIN3RBX4POLY12BlockCornersELj96ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Texture> RBX::Creatable<RBX::Instance>::create<RBX::Texture>(void)")]
// 0x491750 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_7TextureEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::Texture> RBX::Creatable<RBX::Instance>::create<RBX::Texture>(void)
pub fn stub_491750() -> ! {
    todo!("0x491750 __ZN3RBX9CreatableINS_8InstanceEE6createINS_7TextureEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Texture>::shared_ptr<RBX::Texture,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x491800 — __ZN5boost10shared_ptrIN3RBX7TextureEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::Texture>::shared_ptr<RBX::Texture,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_491800() -> ! {
    todo!("0x491800 __ZN5boost10shared_ptrIN3RBX7TextureEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Texture,RBX::Texture>(rbx_core::SharedPtr<RBX::Texture> const*,RBX::Texture *)const")]
// 0x4918c8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7TextureES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Texture,RBX::Texture>(boost::shared_ptr<RBX::Texture> const*,RBX::Texture *)const
pub fn stub_4918c8() -> ! {
    todo!("0x4918c8 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7TextureES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4919b0 — __ZN5boost6detail12shared_countC2IPN3RBX7TextureENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4919b0() -> ! {
    todo!("0x4919b0 __ZN5boost6detail12shared_countC2IPN3RBX7TextureENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x491ab8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextureENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_491ab8<T>(block: &mut crate::shared_ptr::ControlBlockPd<T, crate::shared_ptr::CreatableInstanceDeleter>) {
    // IDA 0x491ab8: empty — base class handles release.
    let _ = block;
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x491abc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextureENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_491abc<T>(block: Box<crate::shared_ptr::ControlBlockPd<T, crate::shared_ptr::CreatableInstanceDeleter>>) {
    // IDA 0x491abc (thunk): operator delete(this).
    drop(block);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x491ac0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextureENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_491ac0<T>(block: &mut crate::shared_ptr::ControlBlockPd<T, crate::shared_ptr::CreatableInstanceDeleter>, predelete: impl FnOnce(Option<&T>)) {
    // IDA 0x491ac0: v2 = px; Instance::predelete(v2); if (v2) virtual-delete(v2) via Deleter.
    // `predelete` is the RBX::Instance::predelete hook (datamodel-owned, passed in).
    block.dispose_with(predelete);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x491ae0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextureENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_491ae0<T>(block: &crate::shared_ptr::ControlBlockPd<T, crate::shared_ptr::CreatableInstanceDeleter>, type_name: &str) -> Option<crate::shared_ptr::CreatableInstanceDeleter> {
    // IDA 0x491ae0: if (ti.name != "N3RBX9CreatableINS_8InstanceEE7DeleterE") return 0; return this+16.
    block.get_deleter(type_name)
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x491af8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextureENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_491af8<T>(block: &crate::shared_ptr::ControlBlockPd<T, crate::shared_ptr::CreatableInstanceDeleter>) -> crate::shared_ptr::CreatableInstanceDeleter {
    // IDA 0x491af8: return this+16 — unconditionally the stored deleter.
    block.get_untyped_deleter()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Decal> RBX::Creatable<RBX::Instance>::create<RBX::Decal>(void)")]
// 0x491f20 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5DecalEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::Decal> RBX::Creatable<RBX::Instance>::create<RBX::Decal>(void)
pub fn stub_491f20() -> ! {
    todo!("0x491f20 __ZN3RBX9CreatableINS_8InstanceEE6createINS_5DecalEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Decal>::shared_ptr<RBX::Decal,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Decal *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x491fd0 — __ZN5boost10shared_ptrIN3RBX5DecalEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::Decal>::shared_ptr<RBX::Decal,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Decal *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_491fd0() -> ! {
    todo!("0x491fd0 __ZN5boost10shared_ptrIN3RBX5DecalEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Decal,RBX::Decal>(rbx_core::SharedPtr<RBX::Decal> const*,RBX::Decal *)const")]
// 0x492098 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5DecalES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Decal,RBX::Decal>(boost::shared_ptr<RBX::Decal> const*,RBX::Decal *)const
pub fn stub_492098() -> ! {
    todo!("0x492098 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5DecalES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Decal *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Decal *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x492180 — __ZN5boost6detail12shared_countC2IPN3RBX5DecalENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::Decal *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Decal *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_492180() -> ! {
    todo!("0x492180 __ZN5boost6detail12shared_countC2IPN3RBX5DecalENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Decal *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x492288 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5DecalENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Decal *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_492288() -> ! {
    todo!("0x492288 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5DecalENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Decal *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x49228c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5DecalENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Decal *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_49228c() -> ! {
    todo!("0x49228c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5DecalENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Decal *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x492290 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5DecalENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::Decal *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_492290() -> ! {
    todo!("0x492290 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5DecalENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Decal *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x4922b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5DecalENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::Decal *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_4922b0() -> ! {
    todo!("0x4922b0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5DecalENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Decal *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x4922c8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5DecalENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::Decal *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_4922c8() -> ! {
    todo!("0x4922c8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5DecalENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DialogChoice> RBX::Creatable<RBX::Instance>::create<RBX::DialogChoice>(void)")]
// 0x494564 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12DialogChoiceEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::DialogChoice> RBX::Creatable<RBX::Instance>::create<RBX::DialogChoice>(void)
pub fn stub_494564() -> ! {
    todo!("0x494564 __ZN3RBX9CreatableINS_8InstanceEE6createINS_12DialogChoiceEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DialogChoice>::shared_ptr<RBX::DialogChoice,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DialogChoice *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x494614 — __ZN5boost10shared_ptrIN3RBX12DialogChoiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::DialogChoice>::shared_ptr<RBX::DialogChoice,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DialogChoice *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_494614() -> ! {
    todo!("0x494614 __ZN5boost10shared_ptrIN3RBX12DialogChoiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::DialogChoice,RBX::DialogChoice>(rbx_core::SharedPtr<RBX::DialogChoice> const*,RBX::DialogChoice *)const")]
// 0x4946dc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12DialogChoiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::DialogChoice,RBX::DialogChoice>(boost::shared_ptr<RBX::DialogChoice> const*,RBX::DialogChoice *)const
pub fn stub_4946dc() -> ! {
    todo!("0x4946dc __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12DialogChoiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::DialogChoice *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DialogChoice *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x494804 — __ZN5boost6detail12shared_countC2IPN3RBX12DialogChoiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::DialogChoice *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DialogChoice *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_494804() -> ! {
    todo!("0x494804 __ZN5boost6detail12shared_countC2IPN3RBX12DialogChoiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DialogChoice *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x49490c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12DialogChoiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::DialogChoice *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_49490c() -> ! {
    todo!("0x49490c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12DialogChoiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DialogChoice *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x494910 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12DialogChoiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::DialogChoice *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_494910() -> ! {
    todo!("0x494910 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12DialogChoiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DialogChoice *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x494914 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12DialogChoiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::DialogChoice *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_494914() -> ! {
    todo!("0x494914 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12DialogChoiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DialogChoice *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x494934 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12DialogChoiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::DialogChoice *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_494934() -> ! {
    todo!("0x494934 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12DialogChoiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DialogChoice *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x49494c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12DialogChoiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::DialogChoice *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_49494c() -> ! {
    todo!("0x49494c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12DialogChoiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::DialogRoot::signalDialogChoice(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
// 0x4954e4 — __ZN3RBX10DialogRoot18signalDialogChoiceEN5boost10shared_ptrINS_8InstanceEEES4_
// was: RBX::DialogRoot::signalDialogChoice(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)
pub fn stub_4954e4() -> ! {
    todo!("0x4954e4 __ZN3RBX10DialogRoot18signalDialogChoiceEN5boost10shared_ptrINS_8InstanceEEES4_")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")]
// 0x49622c — __ZN3RBX10Reflection13BoundFuncDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_ELi2EED1Ev
// was: RBX::Reflection::BoundFuncDesc<RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),2>::~BoundFuncDesc()
pub fn stub_49622c() -> ! {
    todo!("0x49622c __ZN3RBX10Reflection13BoundFuncDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_ELi2EED1Ev")
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::~RemoteEventDesc()")]
// 0x496344 — __ZN3RBX10Reflection15RemoteEventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEED1Ev
// was: RBX::Reflection::RemoteEventDesc<RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>>::~RemoteEventDesc()
pub fn stub_496344() -> ! {
    todo!("0x496344 __ZN3RBX10Reflection15RemoteEventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEED1Ev")
}

#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<2,RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::fireAndReplicateEvent(RBX::DialogRoot*,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
// 0x496a28 — __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEE21fireAndReplicateEventEPS2_S6_S6_
// was: RBX::Reflection::RemoteEventDescImpl<2,RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>>::fireAndReplicateEvent(RBX::DialogRoot*,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)
pub fn stub_496a28() -> ! {
    todo!("0x496a28 __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEE21fireAndReplicateEventEPS2_S6_S6_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DialogRoot> RBX::shared_from<RBX::DialogRoot>(RBX::DialogRoot*)")]
// 0x496bd8 — __ZN3RBX11shared_fromINS_10DialogRootEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::DialogRoot> RBX::shared_from<RBX::DialogRoot>(RBX::DialogRoot*)
pub fn stub_496bd8() -> ! {
    todo!("0x496bd8 __ZN3RBX11shared_fromINS_10DialogRootEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DialogRoot> RBX::Creatable<RBX::Instance>::create<RBX::DialogRoot>(void)")]
// 0x4971d0 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10DialogRootEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::DialogRoot> RBX::Creatable<RBX::Instance>::create<RBX::DialogRoot>(void)
pub fn stub_4971d0() -> ! {
    todo!("0x4971d0 __ZN3RBX9CreatableINS_8InstanceEE6createINS_10DialogRootEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DialogRoot>::shared_ptr<RBX::DialogRoot,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DialogRoot *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x497280 — __ZN5boost10shared_ptrIN3RBX10DialogRootEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::DialogRoot>::shared_ptr<RBX::DialogRoot,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DialogRoot *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_497280() -> ! {
    todo!("0x497280 __ZN5boost10shared_ptrIN3RBX10DialogRootEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::DialogRoot,RBX::DialogRoot>(rbx_core::SharedPtr<RBX::DialogRoot> const*,RBX::DialogRoot *)const")]
// 0x497348 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10DialogRootES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::DialogRoot,RBX::DialogRoot>(boost::shared_ptr<RBX::DialogRoot> const*,RBX::DialogRoot *)const
pub fn stub_497348() -> ! {
    todo!("0x497348 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10DialogRootES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::DialogRoot *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DialogRoot *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x497430 — __ZN5boost6detail12shared_countC2IPN3RBX10DialogRootENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::DialogRoot *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DialogRoot *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_497430() -> ! {
    todo!("0x497430 __ZN5boost6detail12shared_countC2IPN3RBX10DialogRootENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DialogRoot *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x497538 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10DialogRootENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::DialogRoot *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_497538() -> ! {
    todo!("0x497538 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10DialogRootENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DialogRoot *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x49753c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10DialogRootENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::DialogRoot *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_49753c() -> ! {
    todo!("0x49753c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10DialogRootENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DialogRoot *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x497540 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10DialogRootENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::DialogRoot *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_497540() -> ! {
    todo!("0x497540 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10DialogRootENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DialogRoot *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x497560 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10DialogRootENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::DialogRoot *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_497560() -> ! {
    todo!("0x497560 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10DialogRootENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DialogRoot *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x497578 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10DialogRootENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::DialogRoot *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_497578() -> ! {
    todo!("0x497578 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10DialogRootENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::DialogRoot::*>::fireEvent(RBX::DialogRoot*,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)const")]
// 0x497918 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPS2_S6_S6_
// was: RBX::Reflection::EventDescImpl<2,RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::DialogRoot::*>::fireEvent(RBX::DialogRoot*,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)const
pub fn stub_497918() -> ! {
    todo!("0x497918 __ZNK3RBX10Reflection13EventDescImplILi2ENS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPS2_S6_S6_")
}

#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<2,RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::replicateEvent(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
// 0x497a38 — __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEE14replicateEventEPNS0_11EventSourceES6_S6_
// was: RBX::Reflection::RemoteEventDescImpl<2,RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>>::replicateEvent(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)
pub fn stub_497a38() -> ! {
    todo!("0x497a38 __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEE14replicateEventEPNS0_11EventSourceES6_S6_")
}

#[doc(alias = "rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::remote_signal(void)")]
// 0x497ba4 — __ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES5_EEC2Ev
// was: rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::remote_signal(void)
pub fn stub_497ba4() -> ! {
    todo!("0x497ba4 __ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES5_EEC2Ev")
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::~RemoteEventDesc()")]
// 0x4988dc — __ZN3RBX10Reflection15RemoteEventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEED0Ev
// was: RBX::Reflection::RemoteEventDesc<RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>>::~RemoteEventDesc()
pub fn stub_4988dc() -> ! {
    todo!("0x4988dc __ZN3RBX10Reflection15RemoteEventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEED0Ev")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::DialogRoot::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// 0x498990 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<2,RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::DialogRoot::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_498990() -> ! {
    todo!("0x498990 __ZNK3RBX10Reflection13EventDescImplILi2ENS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE")
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::isScriptable(void)const")]
// 0x498af4 — __ZNK3RBX10Reflection15RemoteEventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEE12isScriptableEv
// was: RBX::Reflection::RemoteEventDesc<RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>>::isScriptable(void)const
pub fn stub_498af4() -> ! {
    todo!("0x498af4 __ZNK3RBX10Reflection15RemoteEventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEE12isScriptableEv")
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::isBroadcast(void)const")]
// 0x498afc — __ZNK3RBX10Reflection15RemoteEventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEE11isBroadcastEv
// was: RBX::Reflection::RemoteEventDesc<RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>>::isBroadcast(void)const
pub fn stub_498afc() -> ! {
    todo!("0x498afc __ZNK3RBX10Reflection15RemoteEventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEE11isBroadcastEv")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::DialogRoot::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// 0x498b04 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
// was: RBX::Reflection::EventDescImpl<2,RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::DialogRoot::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_498b04() -> ! {
    todo!("0x498b04 __ZNK3RBX10Reflection13EventDescImplILi2ENS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE")
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// 0x498cb4 — __ZNK3RBX10Reflection15RemoteEventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE
// was: RBX::Reflection::RemoteEventDesc<RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_498cb4() -> ! {
    todo!("0x498cb4 __ZNK3RBX10Reflection15RemoteEventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE")
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::DialogRoot::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// 0x498cc4 — __ZNK3RBX10Reflection13EventDescBaseINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
// was: RBX::Reflection::EventDescBase<RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::DialogRoot::*>::disconnectAll(RBX::Reflection::EventSource *)const
pub fn stub_498cc4() -> ! {
    todo!("0x498cc4 __ZNK3RBX10Reflection13EventDescBaseINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::DialogRoot::*>::EventDesc(rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::DialogRoot::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x498cd8 — __ZN3RBX10Reflection9EventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::EventDesc<RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::DialogRoot::*>::EventDesc(rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::DialogRoot::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_498cd8() -> ! {
    todo!("0x498cd8 __ZN3RBX10Reflection9EventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::DialogRoot::*>::~EventDesc()")]
// 0x498ec8 — __ZN3RBX10Reflection9EventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_ED1Ev
// was: RBX::Reflection::EventDesc<RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::DialogRoot::*>::~EventDesc()
pub fn stub_498ec8() -> ! {
    todo!("0x498ec8 __ZN3RBX10Reflection9EventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_ED1Ev")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::DialogRoot::*>::~EventDesc()")]
// 0x498eec — __ZN3RBX10Reflection9EventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_ED0Ev
// was: RBX::Reflection::EventDesc<RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::DialogRoot::*>::~EventDesc()
pub fn stub_498eec() -> ! {
    todo!("0x498eec __ZN3RBX10Reflection9EventDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx13remote_signalIS7_EEMS2_SA_ED0Ev")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),2>::BoundFuncDesc(void (RBX::DialogRoot::*)(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x498fa0 — __ZN3RBX10Reflection13BoundFuncDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_ELi2EEC2EMS2_FvS6_S6_EPKcSC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundFuncDesc<RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),2>::BoundFuncDesc(void (RBX::DialogRoot::*)(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_498fa0() -> ! {
    todo!("0x498fa0 __ZN3RBX10Reflection13BoundFuncDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_ELi2EEC2EMS2_FvS6_S6_EPKcSC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// 0x499170 — __ZN3RBX10Reflection13BoundFuncDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_ELi2EE16declareSignatureEPKcNS0_7VariantESA_SB_
// was: RBX::Reflection::BoundFuncDesc<RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
pub fn stub_499170() -> ! {
    todo!("0x499170 __ZN3RBX10Reflection13BoundFuncDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_ELi2EE16declareSignatureEPKcNS0_7VariantESA_SB_")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")]
// 0x4991bc — __ZN3RBX10Reflection13BoundFuncDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_ELi2EED0Ev
// was: RBX::Reflection::BoundFuncDesc<RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),2>::~BoundFuncDesc()
pub fn stub_4991bc() -> ! {
    todo!("0x4991bc __ZN3RBX10Reflection13BoundFuncDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_ELi2EED0Ev")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x4992e4 — __ZNK3RBX10Reflection13BoundFuncDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_ELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// was: RBX::Reflection::BoundFuncDesc<RBX::DialogRoot,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub fn stub_4992e4() -> ! {
    todo!("0x4992e4 __ZNK3RBX10Reflection13BoundFuncDescINS_10DialogRootEFvN5boost10shared_ptrINS_8InstanceEEES6_ELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")
}

#[doc(alias = "RBX::Reflection::Call2Helper<RBX::DialogRoot,void (RBX::DialogRoot::*)(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::DialogRoot*,void (RBX::DialogRoot::*)(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&)")]
// 0x499408 — __ZN3RBX10Reflection11Call2HelperINS_10DialogRootEMS2_FvN5boost10shared_ptrINS_8InstanceEEES6_ES6_S6_vE4callEPS2_S8_RNS0_7VariantERKS6_SE_
// was: RBX::Reflection::Call2Helper<RBX::DialogRoot,void (RBX::DialogRoot::*)(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,void>::call(RBX::DialogRoot*,void (RBX::DialogRoot::*)(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&)
pub fn stub_499408() -> ! {
    todo!("0x499408 __ZN3RBX10Reflection11Call2HelperINS_10DialogRootEMS2_FvN5boost10shared_ptrINS_8InstanceEEES6_ES6_S6_vE4callEPS2_S8_RNS0_7VariantERKS6_SE_")
}

#[doc(alias = "rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~remote_signal()")]
// 0x49ad94 — __ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES5_EED2Ev
// was: rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::~remote_signal()
pub fn stub_49ad94() -> ! {
    todo!("0x49ad94 __ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES5_EED2Ev")
}

#[doc(alias = "RBX::Explosion::signalBlast(std::vector<rbx_core::SharedPtr<RBX::PartInstance>,std::allocator<rbx_core::SharedPtr<RBX::PartInstance>>> const&)")]
// 0x49ff08 — __ZN3RBX9Explosion11signalBlastERKSt6vectorIN5boost10shared_ptrINS_12PartInstanceEEESaIS5_EE
// was: RBX::Explosion::signalBlast(std::vector<boost::shared_ptr<RBX::PartInstance>,std::allocator<boost::shared_ptr<RBX::PartInstance>>> const&)
pub fn stub_49ff08() -> ! {
    todo!("0x49ff08 __ZN3RBX9Explosion11signalBlastERKSt6vectorIN5boost10shared_ptrINS_12PartInstanceEEESaIS5_EE")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::Explosion,void ()(rbx_core::SharedPtr<RBX::Instance>,float),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> RBX::Explosion::*>::~EventDesc()")]
// 0x4a04b8 — __ZN3RBX10Reflection9EventDescINS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_ED1Ev
// was: RBX::Reflection::EventDesc<RBX::Explosion,void ()(boost::shared_ptr<RBX::Instance>,float),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,float)> RBX::Explosion::*>::~EventDesc()
pub fn stub_4a04b8() -> ! {
    todo!("0x4a04b8 __ZN3RBX10Reflection9EventDescINS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_ED1Ev")
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::operator()(rbx_core::SharedPtr<RBX::Instance>,float)")]
// 0x4a0840 — __ZN3rbx7signals16signal_with_argsILi2EFvN5boost10shared_ptrIN3RBX8InstanceEEEfEEclES6_f
// was: rbx::signals::signal_with_args<2,void ()(boost::shared_ptr<RBX::Instance>,float)>::operator()(boost::shared_ptr<RBX::Instance>,float)
pub fn stub_4a0840() -> ! {
    todo!("0x4a0840 __ZN3rbx7signals16signal_with_argsILi2EFvN5boost10shared_ptrIN3RBX8InstanceEEEfEEclES6_f")
}

#[doc(alias = "void RBX::Explosion::doBlast<RBX::MegaClusterInstance>(RBX::MegaClusterInstance *,std::vector<rbx_core::SharedPtr<RBX::PartInstance>,std::allocator<rbx_core::SharedPtr<RBX::PartInstance>>> const&)")]
// 0x4a0a30 — __ZN3RBX9Explosion7doBlastINS_19MegaClusterInstanceEEEvPT_RKSt6vectorIN5boost10shared_ptrINS_12PartInstanceEEESaIS9_EE
// was: void RBX::Explosion::doBlast<RBX::MegaClusterInstance>(RBX::MegaClusterInstance *,std::vector<boost::shared_ptr<RBX::PartInstance>,std::allocator<boost::shared_ptr<RBX::PartInstance>>> const&)
pub fn stub_4a0a30() -> ! {
    todo!("0x4a0a30 __ZN3RBX9Explosion7doBlastINS_19MegaClusterInstanceEEEvPT_RKSt6vectorIN5boost10shared_ptrINS_12PartInstanceEEESaIS9_EE")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Explosion>,RBX::Instance*>::type> boost::bind<void,RBX::Instance,RBX::Instance*,rbx_core::SharedPtr<RBX::Explosion>,RBX::Instance*>(void (RBX::Instance::*)(RBX::Instance*),rbx_core::SharedPtr<RBX::Explosion>,RBX::Instance*)")]
// 0x4a10a0 — __ZN5boost4bindIvN3RBX8InstanceEPS2_NS_10shared_ptrINS1_9ExplosionEEES3_EENS_3_bi6bind_tIT_NS_4_mfi3mf1IS9_T0_T1_EENS7_9list_av_2IT2_T3_E4typeEEEMSC_FS9_SD_ESG_SH_
// was: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Explosion>,RBX::Instance*>::type> boost::bind<void,RBX::Instance,RBX::Instance*,boost::shared_ptr<RBX::Explosion>,RBX::Instance*>(void (RBX::Instance::*)(RBX::Instance*),boost::shared_ptr<RBX::Explosion>,RBX::Instance*)
pub fn stub_4a10a0() -> ! {
    todo!("0x4a10a0 __ZN5boost4bindIvN3RBX8InstanceEPS2_NS_10shared_ptrINS1_9ExplosionEEES3_EENS_3_bi6bind_tIT_NS_4_mfi3mf1IS9_T0_T1_EENS7_9list_av_2IT2_T3_E4typeEEEMSC_FS9_SD_ESG_SH_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Explosion> RBX::shared_from<RBX::Explosion>(RBX::Explosion*)")]
// 0x4a11c4 — __ZN3RBX11shared_fromINS_9ExplosionEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::Explosion> RBX::shared_from<RBX::Explosion>(RBX::Explosion*)
pub fn stub_4a11c4() -> ! {
    todo!("0x4a11c4 __ZN3RBX11shared_fromINS_9ExplosionEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ForceField> RBX::Creatable<RBX::Instance>::create<RBX::ForceField>(void)")]
// 0x4a17f8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10ForceFieldEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::ForceField> RBX::Creatable<RBX::Instance>::create<RBX::ForceField>(void)
pub fn stub_4a17f8() -> ! {
    todo!("0x4a17f8 __ZN3RBX9CreatableINS_8InstanceEE6createINS_10ForceFieldEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ForceField *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4a18a8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ForceFieldENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::ForceField *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_4a18a8() -> ! {
    todo!("0x4a18a8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ForceFieldENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ForceField *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x4a18b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ForceFieldENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::ForceField *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_4a18b0() -> ! {
    todo!("0x4a18b0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ForceFieldENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ForceField *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x4a18c8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ForceFieldENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::ForceField *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_4a18c8() -> ! {
    todo!("0x4a18c8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ForceFieldENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4a1d70 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ExplosionENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_4a1d70() -> ! {
    todo!("0x4a1d70 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ExplosionENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPS8_EENS3_5list2INS3_5valueINS_10shared_ptrINS7_9ExplosionEEEEENSC_IS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
// 0x4a2114 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPS8_EENS3_5list2INS3_5valueINS_10shared_ptrINS7_9ExplosionEEEEENSC_IS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
// was: __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPS8_EENS3_5list2INS3_5valueINS_10shared_ptrINS7_9ExplosionEEEEENSC_IS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
pub fn stub_4a2114() -> ! {
    todo!("0x4a2114 __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPS8_EENS3_5list2INS3_5valueINS_10shared_ptrINS7_9ExplosionEEEEENSC_IS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>)")]
// 0x4a2200 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPS8_EENS3_5list2INS3_5valueINS_10shared_ptrINS7_9ExplosionEEEEENSC_IS9_EEEEEEEEvT_
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>)
pub fn stub_4a2200() -> ! {
    todo!("0x4a2200 __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPS8_EENS3_5list2INS3_5valueINS_10shared_ptrINS7_9ExplosionEEEEENSC_IS9_EEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x4a22fc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPS8_EENS3_5list2INS3_5valueINS_10shared_ptrINS7_9ExplosionEEEEENSC_IS9_EEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_4a22fc() -> ! {
    todo!("0x4a22fc __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPS8_EENS3_5list2INS3_5valueINS_10shared_ptrINS7_9ExplosionEEEEENSC_IS9_EEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x4a2318 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPS8_EENS3_5list2INS3_5valueINS_10shared_ptrINS7_9ExplosionEEEEENSC_IS9_EEEEEEvE6invokeERNS1_15function_bufferE
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>,void>::invoke(boost::detail::function::function_buffer &)
pub fn stub_4a2318() -> ! {
    todo!("0x4a2318 __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPS8_EENS3_5list2INS3_5valueINS_10shared_ptrINS7_9ExplosionEEEEENSC_IS9_EEEEEEvE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>,boost::detail::function::function_buffer &)const")]
// 0x4a2320 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPSA_EENS5_5list2INS5_5valueINS_10shared_ptrINS9_9ExplosionEEEEENSE_ISB_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>,boost::detail::function::function_buffer &)const
pub fn stub_4a2320() -> ! {
    todo!("0x4a2320 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPSA_EENS5_5list2INS5_5valueINS_10shared_ptrINS9_9ExplosionEEEEENSE_ISB_EEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x4a240c — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPSA_EENS5_5list2INS5_5valueINS_10shared_ptrINS9_9ExplosionEEEEENSE_ISB_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_4a240c() -> ! {
    todo!("0x4a240c __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPSA_EENS5_5list2INS5_5valueINS_10shared_ptrINS9_9ExplosionEEEEENSE_ISB_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x4a24f4 — __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPSA_EENS5_5list2INS5_5valueINS_10shared_ptrINS9_9ExplosionEEEEENSE_ISB_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_4a24f4() -> ! {
    todo!("0x4a24f4 __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPSA_EENS5_5list2INS5_5valueINS_10shared_ptrINS9_9ExplosionEEEEENSE_ISB_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>::operator()(void)")]
// 0x4a25cc — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPS5_EENS0_5list2INS0_5valueINS_10shared_ptrINS4_9ExplosionEEEEENS9_IS6_EEEEEclEv
// was: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>::operator()(void)
pub fn stub_4a25cc() -> ! {
    todo!("0x4a25cc __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPS5_EENS0_5list2INS0_5valueINS_10shared_ptrINS4_9ExplosionEEEEENS9_IS6_EEEEEclEv")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x4a25e4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPS8_EENS3_5list2INS3_5valueINS_10shared_ptrINS7_9ExplosionEEEEENSC_IS9_EEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_4a25e4() -> ! {
    todo!("0x4a25e4 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPS8_EENS3_5list2INS3_5valueINS_10shared_ptrINS7_9ExplosionEEEEENSC_IS9_EEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Explosion>>,boost::_bi::value<RBX::Instance *>>::list2(boost::_bi::value<rbx_core::SharedPtr<RBX::Explosion>>,boost::_bi::value<RBX::Instance *>)")]
// 0x4a2740 — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX9ExplosionEEEEENS2_IPNS4_8InstanceEEEEC2ES7_SA_
// was: boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Explosion>>,boost::_bi::value<RBX::Instance *>>::list2(boost::_bi::value<boost::shared_ptr<RBX::Explosion>>,boost::_bi::value<RBX::Instance *>)
pub fn stub_4a2740() -> ! {
    todo!("0x4a2740 __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX9ExplosionEEEEENS2_IPNS4_8InstanceEEEEC2ES7_SA_")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::Explosion>>,boost::_bi::value<RBX::Instance *>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::Explosion>>,boost::_bi::value<RBX::Instance *>)")]
// 0x4a2818 — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX9ExplosionEEEEENS2_IPNS4_8InstanceEEEEC2ES7_SA_
// was: boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::Explosion>>,boost::_bi::value<RBX::Instance *>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::Explosion>>,boost::_bi::value<RBX::Instance *>)
pub fn stub_4a2818() -> ! {
    todo!("0x4a2818 __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX9ExplosionEEEEENS2_IPNS4_8InstanceEEEEC2ES7_SA_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TimerService> RBX::Creatable<RBX::Instance>::create<RBX::TimerService>(void)")]
// 0x4a2970 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12TimerServiceEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::TimerService> RBX::Creatable<RBX::Instance>::create<RBX::TimerService>(void)
pub fn stub_4a2970() -> ! {
    todo!("0x4a2970 __ZN3RBX9CreatableINS_8InstanceEE6createINS_12TimerServiceEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TimerService>::shared_ptr<RBX::TimerService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TimerService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4a2a20 — __ZN5boost10shared_ptrIN3RBX12TimerServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::TimerService>::shared_ptr<RBX::TimerService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TimerService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4a2a20() -> ! {
    todo!("0x4a2a20 __ZN5boost10shared_ptrIN3RBX12TimerServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TimerService,RBX::TimerService>(rbx_core::SharedPtr<RBX::TimerService> const*,RBX::TimerService *)const")]
// 0x4a2ae8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12TimerServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TimerService,RBX::TimerService>(boost::shared_ptr<RBX::TimerService> const*,RBX::TimerService *)const
pub fn stub_4a2ae8() -> ! {
    todo!("0x4a2ae8 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12TimerServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::PartInstance>,std::allocator<rbx_core::SharedPtr<RBX::PartInstance>>>::~vector()")]
// 0x4a2c38 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX12PartInstanceEEESaIS4_EED2Ev
// was: std::vector<boost::shared_ptr<RBX::PartInstance>,std::allocator<boost::shared_ptr<RBX::PartInstance>>>::~vector()
pub fn stub_4a2c38() -> ! {
    todo!("0x4a2c38 __ZNSt6vectorIN5boost10shared_ptrIN3RBX12PartInstanceEEESaIS4_EED2Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot> &)")]
// 0x4a2d04 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4nextERNS2_13intrusive_ptrINS8_4slotEEE
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::slot> &)
pub fn stub_4a2d04() -> ! {
    todo!("0x4a2d04 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4nextERNS2_13intrusive_ptrINS8_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::on_error(std::exception &)")]
// 0x4a2e64 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE8on_errorERSt9exception
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::on_error(std::exception &)
pub fn stub_4a2e64() -> ! {
    todo!("0x4a2e64 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE8on_errorERSt9exception")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot> const&)")]
// 0x4a2e8c — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEEfEE4slotEEaSERKSB_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::slot> const&)
pub fn stub_4a2e8c() -> ! {
    todo!("0x4a2e8c __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEEfEE4slotEEaSERKSB_")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::safe_static_init_mutex(void)")]
// 0x4a2eb4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE22safe_static_init_mutexEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::safe_static_init_mutex(void)
pub fn stub_4a2eb4() -> ! {
    todo!("0x4a2eb4 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::safe_static_do_get_mutex(void)")]
// 0x4a2eb8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE24safe_static_do_get_mutexEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::safe_static_do_get_mutex(void)
pub fn stub_4a2eb8() -> ! {
    todo!("0x4a2eb8 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::Explosion,void ()(rbx_core::SharedPtr<RBX::Instance>,float),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> RBX::Explosion::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> RBX::Explosion::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x4a38b8 — __ZN3RBX10Reflection9EventDescINS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::EventDesc<RBX::Explosion,void ()(boost::shared_ptr<RBX::Instance>,float),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,float)> RBX::Explosion::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,float)> RBX::Explosion::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_4a38b8() -> ! {
    todo!("0x4a38b8 __ZN3RBX10Reflection9EventDescINS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::Explosion,void ()(rbx_core::SharedPtr<RBX::Instance>,float),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> RBX::Explosion::*>::~EventDesc()")]
// 0x4a3aa8 — __ZN3RBX10Reflection9EventDescINS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_ED0Ev
// was: RBX::Reflection::EventDesc<RBX::Explosion,void ()(boost::shared_ptr<RBX::Instance>,float),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,float)> RBX::Explosion::*>::~EventDesc()
pub fn stub_4a3aa8() -> ! {
    todo!("0x4a3aa8 __ZN3RBX10Reflection9EventDescINS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_ED0Ev")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Explosion,void ()(rbx_core::SharedPtr<RBX::Instance>,float),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> RBX::Explosion::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// 0x4a3b5c — __ZNK3RBX10Reflection13EventDescImplILi2ENS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<2,RBX::Explosion,void ()(boost::shared_ptr<RBX::Instance>,float),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,float)> RBX::Explosion::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_4a3b5c() -> ! {
    todo!("0x4a3b5c __ZNK3RBX10Reflection13EventDescImplILi2ENS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Explosion,void ()(rbx_core::SharedPtr<RBX::Instance>,float),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> RBX::Explosion::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// 0x4a3cb0 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
// was: RBX::Reflection::EventDescImpl<2,RBX::Explosion,void ()(boost::shared_ptr<RBX::Instance>,float),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,float)> RBX::Explosion::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_4a3cb0() -> ! {
    todo!("0x4a3cb0 __ZNK3RBX10Reflection13EventDescImplILi2ENS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE")
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Explosion,void ()(rbx_core::SharedPtr<RBX::Instance>,float),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> RBX::Explosion::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// 0x4a3e20 — __ZNK3RBX10Reflection13EventDescBaseINS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
// was: RBX::Reflection::EventDescBase<RBX::Explosion,void ()(boost::shared_ptr<RBX::Instance>,float),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,float)> RBX::Explosion::*>::disconnectAll(RBX::Reflection::EventSource *)const
pub fn stub_4a3e20() -> ! {
    todo!("0x4a3e20 __ZNK3RBX10Reflection13EventDescBaseINS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::disconnectAll(void)")]
// 0x4a3e34 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE13disconnectAllEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::disconnectAll(void)
pub fn stub_4a3e34() -> ! {
    todo!("0x4a3e34 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE13disconnectAllEv")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(rbx_core::SharedPtr<RBX::Instance> const&,float const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
// 0x4a3fac — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEERKfNS4_IS3_EENS_3argILi1EEENSC_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISH_T0_T1_T2_EENSF_9list_av_3IT3_T4_T5_E4typeEEEMSK_FSH_SL_SM_ESP_SQ_SR_
// was: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,float const&>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,float const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(boost::shared_ptr<RBX::Instance> const&,float const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)
pub fn stub_4a3fac() -> ! {
    todo!("0x4a3fac __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEERKfNS4_IS3_EENS_3argILi1EEENSC_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISH_T0_T1_T2_EENSF_9list_av_3IT3_T4_T5_E4typeEEEMSK_FSH_SL_SM_ESP_SQ_SR_")
}

#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute2<rbx_core::SharedPtr<RBX::Instance>,float>(rbx_core::SharedPtr<RBX::Instance> const&,float const&)")]
// 0x4a40c8 — __ZN3RBX10Reflection18GenericSlotWrapper8execute2IN5boost10shared_ptrINS_8InstanceEEEfEEvRKT_RKT0_
// was: void RBX::Reflection::GenericSlotWrapper::execute2<boost::shared_ptr<RBX::Instance>,float>(boost::shared_ptr<RBX::Instance> const&,float const&)
pub fn stub_4a40c8() -> ! {
    todo!("0x4a40c8 __ZN3RBX10Reflection18GenericSlotWrapper8execute2IN5boost10shared_ptrINS_8InstanceEEEfEEvRKT_RKT0_")
}

#[doc(alias = "boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,float>::clear(void)")]
// 0x4a4230 — __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEfE5clearEv
// was: boost::function2<void,boost::shared_ptr<RBX::Instance>,float>::clear(void)
pub fn stub_4a4230() -> ! {
    todo!("0x4a4230 __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEfE5clearEv")
}

#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEfEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_RKfEENS8_5list3INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
// 0x4a4260 — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEfEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_RKfEENS8_5list3INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
// was: __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEfEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_RKfEENS8_5list3INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
pub fn stub_4a4260() -> ! {
    todo!("0x4a4260 __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEfEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_RKfEENS8_5list3INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEfEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_RKfEENS7_5list3INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
// 0x4a4344 — __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEfEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_RKfEENS7_5list3INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE
// was: __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEfEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_RKfEENS7_5list3INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE
pub fn stub_4a4344() -> ! {
    todo!("0x4a4344 __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEfEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_RKfEENS7_5list3INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")]
// 0x4a442c — __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_RKfEENS7_5list3INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_
// was: void boost::function2<void,boost::shared_ptr<RBX::Instance>,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)
pub fn stub_4a442c() -> ! {
    todo!("0x4a442c __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_RKfEENS7_5list3INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x4a4524 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKfEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_4a4524() -> ! {
    todo!("0x4a4524 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKfEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,rbx_core::SharedPtr<RBX::Instance>,float>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>,float)")]
// 0x4a4540 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKfEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEvSC_fE6invokeERNS1_15function_bufferESC_f
// was: boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,boost::shared_ptr<RBX::Instance>,float>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>,float)
pub fn stub_4a4540() -> ! {
    todo!("0x4a4540 __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKfEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEvSC_fE6invokeERNS1_15function_bufferESC_f")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,rbx_core::SharedPtr<RBX::Instance>,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")]
// 0x4a4554 — __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_RKfEENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable2<void,boost::shared_ptr<RBX::Instance>,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const
pub fn stub_4a4554() -> ! {
    todo!("0x4a4554 __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_RKfEENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,rbx_core::SharedPtr<RBX::Instance>,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x4a463c — __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_RKfEENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable2<void,boost::shared_ptr<RBX::Instance>,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_4a463c() -> ! {
    todo!("0x4a463c __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_RKfEENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,rbx_core::SharedPtr<RBX::Instance>,float>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x4a4720 — __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEfE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_RKfEENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable2<void,boost::shared_ptr<RBX::Instance>,float>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_4a4720() -> ! {
    todo!("0x4a4720 __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEfE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_RKfEENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<rbx_core::SharedPtr<RBX::Instance>,float>(rbx_core::SharedPtr<RBX::Instance> &,float &)")]
// 0x4a47f4 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS4_8InstanceEEERKfEENS0_5list3INS0_5valueINS7_IS6_EEEENS_3argILi1EEENSJ_ILi2EEEEEEclIS9_fEEvRT_RT0_
// was: void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<boost::shared_ptr<RBX::Instance>,float>(boost::shared_ptr<RBX::Instance> &,float &)
pub fn stub_4a47f4() -> ! {
    todo!("0x4a47f4 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS4_8InstanceEEERKfEENS0_5list3INS0_5valueINS7_IS6_EEEENS_3argILi1EEENSJ_ILi2EEEEEEclIS9_fEEvRT_RT0_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x4a4810 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKfEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_4a4810() -> ! {
    todo!("0x4a4810 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKfEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> const&)")]
// 0x4a4968 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_
// was: rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::connect<boost::function<void ()(boost::shared_ptr<RBX::Instance>,float)>>(boost::function<void ()(boost::shared_ptr<RBX::Instance>,float)> const&)
pub fn stub_4a4968() -> ! {
    todo!("0x4a4968 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::insert(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot *)")]
// 0x4a4a5c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE6insertEPNS8_4slotE
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::insert(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::slot *)
pub fn stub_4a4a5c() -> ! {
    todo!("0x4a4a5c __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE6insertEPNS8_4slotE")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot*)")]
// 0x4a4c68 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEEfEE4slotEEaSEPSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::slot>::operator=(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::slot*)
pub fn stub_4a4c68() -> ! {
    todo!("0x4a4c68 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEEfEE4slotEEaSEPSA_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>*>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>*)")]
// 0x4a4c8c — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotENS3_8functionIS8_EELi2ES8_EC2IPS9_EERKSC_T_
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,float)>,2,void ()(boost::shared_ptr<RBX::Instance>,float)>::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>*>(boost::function<void ()(boost::shared_ptr<RBX::Instance>,float)> const&,rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>*)
pub fn stub_4a4c8c() -> ! {
    todo!("0x4a4c8c __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotENS3_8functionIS8_EELi2ES8_EC2IPS9_EERKSC_T_")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>>::~callable_slot()")]
// 0x4a4d88 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE13callable_slotINS2_8functionIS7_EEED1Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::Instance>,float)>>::~callable_slot()
pub fn stub_4a4d88() -> ! {
    todo!("0x4a4d88 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE13callable_slotINS2_8functionIS7_EEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>>::~callable_slot()")]
// 0x4a4e98 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE13callable_slotINS2_8functionIS7_EEED0Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::Instance>,float)>>::~callable_slot()
pub fn stub_4a4e98() -> ! {
    todo!("0x4a4e98 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE13callable_slotINS2_8functionIS7_EEED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot::disconnect(void)")]
// 0x4a4fc8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slot10disconnectEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::slot::disconnect(void)
pub fn stub_4a4fc8() -> ! {
    todo!("0x4a4fc8 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slot10disconnectEv")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot::connected(void)const")]
// 0x4a50d8 — __ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slot9connectedEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::slot::connected(void)const
pub fn stub_4a50d8() -> ! {
    todo!("0x4a50d8 __ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slot9connectedEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::call(rbx_core::SharedPtr<RBX::Instance>,float)")]
// 0x4a50e4 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotENS3_8functionIS8_EELi2ES8_E4callES7_f
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,float)>,2,void ()(boost::shared_ptr<RBX::Instance>,float)>::call(boost::shared_ptr<RBX::Instance>,float)
pub fn stub_4a50e4() -> ! {
    todo!("0x4a50e4 __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotENS3_8functionIS8_EELi2ES8_E4callES7_f")
}

#[doc(alias = "non_virtual_thunk_to rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::call(rbx_core::SharedPtr<RBX::Instance>,float)")]
// 0x4a51bc — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotENS3_8functionIS8_EELi2ES8_E4callES7_f
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,float)>,2,void ()(boost::shared_ptr<RBX::Instance>,float)>::call(boost::shared_ptr<RBX::Instance>,float)
pub fn stub_4a51bc() -> ! {
    todo!("0x4a51bc __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotENS3_8functionIS8_EELi2ES8_E4callES7_f")
}

#[doc(alias = "boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,float>::operator()(rbx_core::SharedPtr<RBX::Instance>,float)const")]
// 0x4a51c4 — __ZNK5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEfEclES4_f
// was: boost::function2<void,boost::shared_ptr<RBX::Instance>,float>::operator()(boost::shared_ptr<RBX::Instance>,float)const
pub fn stub_4a51c4() -> ! {
    todo!("0x4a51c4 __ZNK5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEfEclES4_f")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::remove(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot *)")]
// 0x4a52dc — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE6removeEPNS8_4slotE
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::remove(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::slot *)
pub fn stub_4a52dc() -> ! {
    todo!("0x4a52dc __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE6removeEPNS8_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot::safe_static_init_mutex(void)")]
// 0x4a53cc — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slot22safe_static_init_mutexEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::slot::safe_static_init_mutex(void)
pub fn stub_4a53cc() -> ! {
    todo!("0x4a53cc __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot::safe_static_do_get_mutex(void)")]
// 0x4a53d0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slot24safe_static_do_get_mutexEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::slot::safe_static_do_get_mutex(void)
pub fn stub_4a53d0() -> ! {
    todo!("0x4a53d0 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::~callable()")]
// 0x4a54c4 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotENS3_8functionIS8_EELi2ES8_ED1Ev
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,float)>,2,void ()(boost::shared_ptr<RBX::Instance>,float)>::~callable()
pub fn stub_4a54c4() -> ! {
    todo!("0x4a54c4 __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotENS3_8functionIS8_EELi2ES8_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::~callable()")]
// 0x4a55d4 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotENS3_8functionIS8_EELi2ES8_ED0Ev
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,float)>,2,void ()(boost::shared_ptr<RBX::Instance>,float)>::~callable()
pub fn stub_4a55d4() -> ! {
    todo!("0x4a55d4 __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotENS3_8functionIS8_EELi2ES8_ED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot::~slot()")]
// 0x4a5704 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotD1Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::slot::~slot()
pub fn stub_4a5704() -> ! {
    todo!("0x4a5704 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotD1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot::~slot()")]
// 0x4a5730 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotD0Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,float)>::slot::~slot()
pub fn stub_4a5730() -> ! {
    todo!("0x4a5730 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4slotD0Ev")
}

#[doc(alias = "boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,float>::assign_to_own(boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,float> const&)")]
// 0x4a5804 — __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEfE13assign_to_ownERKS5_
// was: boost::function2<void,boost::shared_ptr<RBX::Instance>,float>::assign_to_own(boost::function2<void,boost::shared_ptr<RBX::Instance>,float> const&)
pub fn stub_4a5804() -> ! {
    todo!("0x4a5804 __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEfE13assign_to_ownERKS5_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ExtrudedPartInstance> RBX::Creatable<RBX::Instance>::create<RBX::ExtrudedPartInstance>(void)")]
// 0x4a7de0 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_20ExtrudedPartInstanceEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::ExtrudedPartInstance> RBX::Creatable<RBX::Instance>::create<RBX::ExtrudedPartInstance>(void)
pub fn stub_4a7de0() -> ! {
    todo!("0x4a7de0 __ZN3RBX9CreatableINS_8InstanceEE6createINS_20ExtrudedPartInstanceEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ExtrudedPartInstance>::shared_ptr<RBX::ExtrudedPartInstance,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ExtrudedPartInstance *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4a7e94 — __ZN5boost10shared_ptrIN3RBX20ExtrudedPartInstanceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::ExtrudedPartInstance>::shared_ptr<RBX::ExtrudedPartInstance,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ExtrudedPartInstance *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4a7e94() -> ! {
    todo!("0x4a7e94 __ZN5boost10shared_ptrIN3RBX20ExtrudedPartInstanceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance>(rbx_core::SharedPtr<RBX::ExtrudedPartInstance> const*,RBX::ExtrudedPartInstance *)const")]
// 0x4a7f5c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_20ExtrudedPartInstanceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance>(boost::shared_ptr<RBX::ExtrudedPartInstance> const*,RBX::ExtrudedPartInstance *)const
pub fn stub_4a7f5c() -> ! {
    todo!("0x4a7f5c __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_20ExtrudedPartInstanceES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ExtrudedPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ExtrudedPartInstance *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4a8044 — __ZN5boost6detail12shared_countC2IPN3RBX20ExtrudedPartInstanceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::ExtrudedPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ExtrudedPartInstance *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4a8044() -> ! {
    todo!("0x4a8044 __ZN5boost6detail12shared_countC2IPN3RBX20ExtrudedPartInstanceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ExtrudedPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4a814c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::ExtrudedPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_4a814c() -> ! {
    todo!("0x4a814c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ExtrudedPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4a8150 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::ExtrudedPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_4a8150() -> ! {
    todo!("0x4a8150 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ExtrudedPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x4a8154 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::ExtrudedPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_4a8154() -> ! {
    todo!("0x4a8154 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ExtrudedPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x4a8174 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::ExtrudedPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_4a8174() -> ! {
    todo!("0x4a8174 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ExtrudedPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x4a818c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::ExtrudedPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_4a818c() -> ! {
    todo!("0x4a818c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ExtrudedPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "__ZN5boost8functionIFvRSt9exceptionEEaSIPS3_EENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS4_E4typeES8_")]
// 0x4ab510 — __ZN5boost8functionIFvRSt9exceptionEEaSIPS3_EENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS4_E4typeES8_
// was: __ZN5boost8functionIFvRSt9exceptionEEaSIPS3_EENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS4_E4typeES8_
pub fn stub_4ab510() -> ! {
    todo!("0x4ab510 __ZN5boost8functionIFvRSt9exceptionEEaSIPS3_EENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS4_E4typeES8_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BindableEvent> RBX::Creatable<RBX::Instance>::create<RBX::BindableEvent>(void)")]
// 0x4ab854 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13BindableEventEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::BindableEvent> RBX::Creatable<RBX::Instance>::create<RBX::BindableEvent>(void)
pub fn stub_4ab854() -> ! {
    todo!("0x4ab854 __ZN3RBX9CreatableINS_8InstanceEE6createINS_13BindableEventEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::safe_static_init_mutex(void)")]
// 0x4abb28 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEEE22safe_static_init_mutexEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>::safe_static_init_mutex(void)
pub fn stub_4abb28() -> ! {
    todo!("0x4abb28 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEEE22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::safe_static_do_get_mutex(void)")]
// 0x4abb2c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEEE24safe_static_do_get_mutexEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>::safe_static_do_get_mutex(void)
pub fn stub_4abb2c() -> ! {
    todo!("0x4abb2c __ZN3rbx7signals6signalIFvN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BindableEvent>::shared_ptr<RBX::BindableEvent,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BindableEvent *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4abf40 — __ZN5boost10shared_ptrIN3RBX13BindableEventEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::BindableEvent>::shared_ptr<RBX::BindableEvent,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BindableEvent *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4abf40() -> ! {
    todo!("0x4abf40 __ZN5boost10shared_ptrIN3RBX13BindableEventEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BindableEvent,RBX::BindableEvent>(rbx_core::SharedPtr<RBX::BindableEvent> const*,RBX::BindableEvent *)const")]
// 0x4ac008 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13BindableEventES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BindableEvent,RBX::BindableEvent>(boost::shared_ptr<RBX::BindableEvent> const*,RBX::BindableEvent *)const
pub fn stub_4ac008() -> ! {
    todo!("0x4ac008 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13BindableEventES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

