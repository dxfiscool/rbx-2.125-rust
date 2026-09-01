//! core-A: 150 boost stubs — filtered boost:: namespace.
//! First half addresses < 0x30000 are already fully covered (84 funcs, 84/84 in boost_skeletons.rs).
//! This batch continues the next 150 boost stubs in EA order (0x452f48..0x461d6c) so `cargo check` stays green.
//! Source: `ida/export.json` filtered where mangled/demangled contains "boost".
//! Each stub preserves IDA address, mangled symbol, and demangled spelling; signatures use `rbx_core::SharedPtr` not `boost::`.

#[doc(alias = "rbx_core::SharedPtr<RBX::RunService> RBX::Creatable<RBX::Instance>::create<RBX::RunService>(void)")]
// 0x452f48 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10RunServiceEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::RunService> RBX::Creatable<RBX::Instance>::create<RBX::RunService>(void)
pub fn stub_452f48() -> ! {
    todo!("0x452f48 __ZN3RBX9CreatableINS_8InstanceEE6createINS_10RunServiceEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::RunService>(rbx_core::SharedPtr<RBX::RunService> const&)")]
// 0x452ff8 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_10RunServiceEEERS3_RKNS0_IT_EE
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::RunService>(boost::shared_ptr<RBX::RunService> const&)
pub fn stub_452ff8() -> ! {
    todo!("0x452ff8 __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_10RunServiceEEERS3_RKNS0_IT_EE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CoreGuiService>::shared_ptr<RBX::CoreGuiService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CoreGuiService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x453040 — __ZN5boost10shared_ptrIN3RBX14CoreGuiServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::CoreGuiService>::shared_ptr<RBX::CoreGuiService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CoreGuiService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_453040() -> ! {
    todo!("0x453040 __ZN5boost10shared_ptrIN3RBX14CoreGuiServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CoreGuiService,RBX::CoreGuiService>(rbx_core::SharedPtr<RBX::CoreGuiService> const*,RBX::CoreGuiService *)const")]
// 0x453108 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_14CoreGuiServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CoreGuiService,RBX::CoreGuiService>(boost::shared_ptr<RBX::CoreGuiService> const*,RBX::CoreGuiService *)const
pub fn stub_453108() -> ! {
    todo!("0x453108 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_14CoreGuiServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CoreGuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4531f8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14CoreGuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::CoreGuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_4531f8() -> ! {
    todo!("0x4531f8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14CoreGuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::StarterGuiService> RBX::Creatable<RBX::Instance>::create<RBX::StarterGuiService>(void)")]
// 0x453374 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_17StarterGuiServiceEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::StarterGuiService> RBX::Creatable<RBX::Instance>::create<RBX::StarterGuiService>(void)
pub fn stub_453374() -> ! {
    todo!("0x453374 __ZN3RBX9CreatableINS_8InstanceEE6createINS_17StarterGuiServiceEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::StarterGuiService>(rbx_core::SharedPtr<RBX::StarterGuiService> const&)")]
// 0x453424 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_17StarterGuiServiceEEERS3_RKNS0_IT_EE
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::StarterGuiService>(boost::shared_ptr<RBX::StarterGuiService> const&)
pub fn stub_453424() -> ! {
    todo!("0x453424 __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_17StarterGuiServiceEEERS3_RKNS0_IT_EE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::StarterGuiService>::shared_ptr<RBX::StarterGuiService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x453660 — __ZN5boost10shared_ptrIN3RBX17StarterGuiServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::StarterGuiService>::shared_ptr<RBX::StarterGuiService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_453660() -> ! {
    todo!("0x453660 __ZN5boost10shared_ptrIN3RBX17StarterGuiServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::StarterGuiService,RBX::StarterGuiService>(rbx_core::SharedPtr<RBX::StarterGuiService> const*,RBX::StarterGuiService *)const")]
// 0x453728 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17StarterGuiServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::StarterGuiService,RBX::StarterGuiService>(boost::shared_ptr<RBX::StarterGuiService> const*,RBX::StarterGuiService *)const
pub fn stub_453728() -> ! {
    todo!("0x453728 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17StarterGuiServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x453814 — __ZN5boost6detail12shared_countC2IPN3RBX17StarterGuiServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_453814() -> ! {
    todo!("0x453814 __ZN5boost6detail12shared_countC2IPN3RBX17StarterGuiServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x453920 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17StarterGuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_453920() -> ! {
    todo!("0x453920 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17StarterGuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x453924 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17StarterGuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_453924() -> ! {
    todo!("0x453924 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17StarterGuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x45393c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17StarterGuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_45393c() -> ! {
    todo!("0x45393c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17StarterGuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::StarterPackService>::shared_ptr<RBX::StarterPackService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StarterPackService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x453d60 — __ZN5boost10shared_ptrIN3RBX18StarterPackServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::StarterPackService>::shared_ptr<RBX::StarterPackService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StarterPackService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_453d60() -> ! {
    todo!("0x453d60 __ZN5boost10shared_ptrIN3RBX18StarterPackServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::StarterPackService,RBX::StarterPackService>(rbx_core::SharedPtr<RBX::StarterPackService> const*,RBX::StarterPackService *)const")]
// 0x453e28 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_18StarterPackServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::StarterPackService,RBX::StarterPackService>(boost::shared_ptr<RBX::StarterPackService> const*,RBX::StarterPackService *)const
pub fn stub_453e28() -> ! {
    todo!("0x453e28 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_18StarterPackServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterPackService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x453f18 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18StarterPackServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::StarterPackService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_453f18() -> ! {
    todo!("0x453f18 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18StarterPackServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::PlayerHUD>::shared_ptr<RBX::PlayerHUD,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x453fc0 — __ZN5boost10shared_ptrIN3RBX9PlayerHUDEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::PlayerHUD>::shared_ptr<RBX::PlayerHUD,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_453fc0() -> ! {
    todo!("0x453fc0 __ZN5boost10shared_ptrIN3RBX9PlayerHUDEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PlayerHUD,RBX::PlayerHUD>(rbx_core::SharedPtr<RBX::PlayerHUD> const*,RBX::PlayerHUD *)const")]
// 0x454088 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9PlayerHUDES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PlayerHUD,RBX::PlayerHUD>(boost::shared_ptr<RBX::PlayerHUD> const*,RBX::PlayerHUD *)const
pub fn stub_454088() -> ! {
    todo!("0x454088 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9PlayerHUDES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x454174 — __ZN5boost6detail12shared_countC2IPN3RBX9PlayerHUDENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_454174() -> ! {
    todo!("0x454174 __ZN5boost6detail12shared_countC2IPN3RBX9PlayerHUDENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x45427c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerHUDENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_45427c() -> ! {
    todo!("0x45427c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerHUDENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x454280 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerHUDENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_454280() -> ! {
    todo!("0x454280 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerHUDENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x454284 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerHUDENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_454284() -> ! {
    todo!("0x454284 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerHUDENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x4542a4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerHUDENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_4542a4() -> ! {
    todo!("0x4542a4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerHUDENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x4542bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerHUDENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_4542bc() -> ! {
    todo!("0x4542bc __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerHUDENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LocalBackpack> RBX::Creatable<RBX::Instance>::create<RBX::LocalBackpack>(void)")]
// 0x454434 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13LocalBackpackEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::LocalBackpack> RBX::Creatable<RBX::Instance>::create<RBX::LocalBackpack>(void)
pub fn stub_454434() -> ! {
    todo!("0x454434 __ZN3RBX9CreatableINS_8InstanceEE6createINS_13LocalBackpackEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::LocalBackpack>(rbx_core::SharedPtr<RBX::LocalBackpack> const&)")]
// 0x4544e4 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13LocalBackpackEEERS3_RKNS0_IT_EE
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::LocalBackpack>(boost::shared_ptr<RBX::LocalBackpack> const&)
pub fn stub_4544e4() -> ! {
    todo!("0x4544e4 __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13LocalBackpackEEERS3_RKNS0_IT_EE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LocalBackpack>::shared_ptr<RBX::LocalBackpack,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x454720 — __ZN5boost10shared_ptrIN3RBX13LocalBackpackEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::LocalBackpack>::shared_ptr<RBX::LocalBackpack,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_454720() -> ! {
    todo!("0x454720 __ZN5boost10shared_ptrIN3RBX13LocalBackpackEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LocalBackpack,RBX::LocalBackpack>(rbx_core::SharedPtr<RBX::LocalBackpack> const*,RBX::LocalBackpack *)const")]
// 0x4547e8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13LocalBackpackES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LocalBackpack,RBX::LocalBackpack>(boost::shared_ptr<RBX::LocalBackpack> const*,RBX::LocalBackpack *)const
pub fn stub_4547e8() -> ! {
    todo!("0x4547e8 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13LocalBackpackES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4548d4 — __ZN5boost6detail12shared_countC2IPN3RBX13LocalBackpackENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4548d4() -> ! {
    todo!("0x4548d4 __ZN5boost6detail12shared_countC2IPN3RBX13LocalBackpackENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4549dc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LocalBackpackENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_4549dc() -> ! {
    todo!("0x4549dc __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LocalBackpackENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4549e0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LocalBackpackENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_4549e0() -> ! {
    todo!("0x4549e0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LocalBackpackENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x4549e4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LocalBackpackENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_4549e4() -> ! {
    todo!("0x4549e4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LocalBackpackENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x454a04 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LocalBackpackENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_454a04() -> ! {
    todo!("0x454a04 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LocalBackpackENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x454a1c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LocalBackpackENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_454a1c() -> ! {
    todo!("0x454a1c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LocalBackpackENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::MarketplaceService> RBX::Creatable<RBX::Instance>::create<RBX::MarketplaceService>(void)")]
// 0x454ca4 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_18MarketplaceServiceEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::MarketplaceService> RBX::Creatable<RBX::Instance>::create<RBX::MarketplaceService>(void)
pub fn stub_454ca4() -> ! {
    todo!("0x454ca4 __ZN3RBX9CreatableINS_8InstanceEE6createINS_18MarketplaceServiceEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x454d58 — __ZN5boost6detail12shared_countC2IPN3RBX18MarketplaceServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_454d58() -> ! {
    todo!("0x454d58 __ZN5boost6detail12shared_countC2IPN3RBX18MarketplaceServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x454e60 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MarketplaceServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_454e60() -> ! {
    todo!("0x454e60 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MarketplaceServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x454e64 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MarketplaceServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_454e64() -> ! {
    todo!("0x454e64 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MarketplaceServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x454e68 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MarketplaceServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_454e68() -> ! {
    todo!("0x454e68 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MarketplaceServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x454e88 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MarketplaceServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_454e88() -> ! {
    todo!("0x454e88 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MarketplaceServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x454ea0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MarketplaceServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_454ea0() -> ! {
    todo!("0x454ea0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MarketplaceServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ChatService> RBX::Creatable<RBX::Instance>::create<RBX::ChatService>(void)")]
// 0x45562c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_11ChatServiceEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::ChatService> RBX::Creatable<RBX::Instance>::create<RBX::ChatService>(void)
pub fn stub_45562c() -> ! {
    todo!("0x45562c __ZN3RBX9CreatableINS_8InstanceEE6createINS_11ChatServiceEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ChatService>::shared_ptr<RBX::ChatService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4556dc — __ZN5boost10shared_ptrIN3RBX11ChatServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::ChatService>::shared_ptr<RBX::ChatService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4556dc() -> ! {
    todo!("0x4556dc __ZN5boost10shared_ptrIN3RBX11ChatServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChatService,RBX::ChatService>(rbx_core::SharedPtr<RBX::ChatService> const*,RBX::ChatService *)const")]
// 0x4557a4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11ChatServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChatService,RBX::ChatService>(boost::shared_ptr<RBX::ChatService> const*,RBX::ChatService *)const
pub fn stub_4557a4() -> ! {
    todo!("0x4557a4 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11ChatServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x455890 — __ZN5boost6detail12shared_countC2IPN3RBX11ChatServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_455890() -> ! {
    todo!("0x455890 __ZN5boost6detail12shared_countC2IPN3RBX11ChatServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x455998 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_455998() -> ! {
    todo!("0x455998 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x45599c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_45599c() -> ! {
    todo!("0x45599c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x4559a0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_4559a0() -> ! {
    todo!("0x4559a0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x4559c0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_4559c0() -> ! {
    todo!("0x4559c0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x4559d8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_4559d8() -> ! {
    todo!("0x4559d8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ChatService>(rbx_core::SharedPtr<RBX::ChatService> const&)")]
// 0x455ea4 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_11ChatServiceEEERS3_RKNS0_IT_EE
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::ChatService>(boost::shared_ptr<RBX::ChatService> const&)
pub fn stub_455ea4() -> ! {
    todo!("0x455ea4 __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_11ChatServiceEEERS3_RKNS0_IT_EE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GuiService> RBX::Creatable<RBX::Instance>::create<RBX::GuiService>(void)")]
// 0x456090 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10GuiServiceEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::GuiService> RBX::Creatable<RBX::Instance>::create<RBX::GuiService>(void)
pub fn stub_456090() -> ! {
    todo!("0x456090 __ZN3RBX9CreatableINS_8InstanceEE6createINS_10GuiServiceEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::GuiService>(rbx_core::SharedPtr<RBX::GuiService> const&)")]
// 0x456140 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_10GuiServiceEEERS3_RKNS0_IT_EE
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::GuiService>(boost::shared_ptr<RBX::GuiService> const&)
pub fn stub_456140() -> ! {
    todo!("0x456140 __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_10GuiServiceEEERS3_RKNS0_IT_EE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GuiService>::shared_ptr<RBX::GuiService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x456174 — __ZN5boost10shared_ptrIN3RBX10GuiServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::GuiService>::shared_ptr<RBX::GuiService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_456174() -> ! {
    todo!("0x456174 __ZN5boost10shared_ptrIN3RBX10GuiServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiService,RBX::GuiService>(rbx_core::SharedPtr<RBX::GuiService> const*,RBX::GuiService *)const")]
// 0x45623c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10GuiServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiService,RBX::GuiService>(boost::shared_ptr<RBX::GuiService> const*,RBX::GuiService *)const
pub fn stub_45623c() -> ! {
    todo!("0x45623c __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10GuiServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x456328 — __ZN5boost6detail12shared_countC2IPN3RBX10GuiServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_456328() -> ! {
    todo!("0x456328 __ZN5boost6detail12shared_countC2IPN3RBX10GuiServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x456430 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_456430() -> ! {
    todo!("0x456430 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x456434 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_456434() -> ! {
    todo!("0x456434 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x456438 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_456438() -> ! {
    todo!("0x456438 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x456458 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_456458() -> ! {
    todo!("0x456458 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x456470 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_456470() -> ! {
    todo!("0x456470 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::KeyframeSequenceProvider> RBX::Creatable<RBX::Instance>::create<RBX::KeyframeSequenceProvider>(void)")]
// 0x4565e8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_24KeyframeSequenceProviderEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::KeyframeSequenceProvider> RBX::Creatable<RBX::Instance>::create<RBX::KeyframeSequenceProvider>(void)
pub fn stub_4565e8() -> ! {
    todo!("0x4565e8 __ZN3RBX9CreatableINS_8InstanceEE6createINS_24KeyframeSequenceProviderEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::KeyframeSequenceProvider>(rbx_core::SharedPtr<RBX::KeyframeSequenceProvider> const&)")]
// 0x456698 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_24KeyframeSequenceProviderEEERS3_RKNS0_IT_EE
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::KeyframeSequenceProvider>(boost::shared_ptr<RBX::KeyframeSequenceProvider> const&)
pub fn stub_456698() -> ! {
    todo!("0x456698 __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_24KeyframeSequenceProviderEEERS3_RKNS0_IT_EE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::KeyframeSequenceProvider>::shared_ptr<RBX::KeyframeSequenceProvider,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4567f4 — __ZN5boost10shared_ptrIN3RBX24KeyframeSequenceProviderEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::KeyframeSequenceProvider>::shared_ptr<RBX::KeyframeSequenceProvider,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4567f4() -> ! {
    todo!("0x4567f4 __ZN5boost10shared_ptrIN3RBX24KeyframeSequenceProviderEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::KeyframeSequenceProvider,RBX::KeyframeSequenceProvider>(rbx_core::SharedPtr<RBX::KeyframeSequenceProvider> const*,RBX::KeyframeSequenceProvider *)const")]
// 0x4568bc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_24KeyframeSequenceProviderES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::KeyframeSequenceProvider,RBX::KeyframeSequenceProvider>(boost::shared_ptr<RBX::KeyframeSequenceProvider> const*,RBX::KeyframeSequenceProvider *)const
pub fn stub_4568bc() -> ! {
    todo!("0x4568bc __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_24KeyframeSequenceProviderES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4569a8 — __ZN5boost6detail12shared_countC2IPN3RBX24KeyframeSequenceProviderENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4569a8() -> ! {
    todo!("0x4569a8 __ZN5boost6detail12shared_countC2IPN3RBX24KeyframeSequenceProviderENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x456ab0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_456ab0() -> ! {
    todo!("0x456ab0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x456ab4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_456ab4() -> ! {
    todo!("0x456ab4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x456ab8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_456ab8() -> ! {
    todo!("0x456ab8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x456ad8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_456ad8() -> ! {
    todo!("0x456ad8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x456af0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_456af0() -> ! {
    todo!("0x456af0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ContentFilter> RBX::Creatable<RBX::Instance>::create<RBX::ContentFilter>(void)")]
// 0x456d08 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13ContentFilterEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::ContentFilter> RBX::Creatable<RBX::Instance>::create<RBX::ContentFilter>(void)
pub fn stub_456d08() -> ! {
    todo!("0x456d08 __ZN3RBX9CreatableINS_8InstanceEE6createINS_13ContentFilterEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ContentFilter>(rbx_core::SharedPtr<RBX::ContentFilter> const&)")]
// 0x456db8 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13ContentFilterEEERS3_RKNS0_IT_EE
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::ContentFilter>(boost::shared_ptr<RBX::ContentFilter> const&)
pub fn stub_456db8() -> ! {
    todo!("0x456db8 __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13ContentFilterEEERS3_RKNS0_IT_EE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ContentFilter>::shared_ptr<RBX::ContentFilter,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x456ff4 — __ZN5boost10shared_ptrIN3RBX13ContentFilterEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::ContentFilter>::shared_ptr<RBX::ContentFilter,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_456ff4() -> ! {
    todo!("0x456ff4 __ZN5boost10shared_ptrIN3RBX13ContentFilterEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ContentFilter,RBX::ContentFilter>(rbx_core::SharedPtr<RBX::ContentFilter> const*,RBX::ContentFilter *)const")]
// 0x4570bc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13ContentFilterES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ContentFilter,RBX::ContentFilter>(boost::shared_ptr<RBX::ContentFilter> const*,RBX::ContentFilter *)const
pub fn stub_4570bc() -> ! {
    todo!("0x4570bc __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13ContentFilterES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4571a8 — __ZN5boost6detail12shared_countC2IPN3RBX13ContentFilterENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4571a8() -> ! {
    todo!("0x4571a8 __ZN5boost6detail12shared_countC2IPN3RBX13ContentFilterENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4572b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_4572b0() -> ! {
    todo!("0x4572b0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4572b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_4572b4() -> ! {
    todo!("0x4572b4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x4572b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_4572b8() -> ! {
    todo!("0x4572b8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x4572d8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_4572d8() -> ! {
    todo!("0x4572d8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x4572f0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_4572f0() -> ! {
    todo!("0x4572f0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::delete_buckets(void)")]
// 0x457398 — __ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE14delete_bucketsEv
// was: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::delete_buckets(void)
pub fn stub_457398() -> ! {
    todo!("0x457398 __ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE14delete_bucketsEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::table(unsigned long,boost::hash<unsigned int> const&,std::equal_to<unsigned int> const&,std::allocator<boost::unordered::detail::ptr_node<unsigned int>> const&)")]
// 0x4573e4 — __ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEEC2EmRKS6_RKS8_RKSaINS1_8ptr_nodeIjEEE
// was: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::table(unsigned long,boost::hash<unsigned int> const&,std::equal_to<unsigned int> const&,std::allocator<boost::unordered::detail::ptr_node<unsigned int>> const&)
pub fn stub_4573e4() -> ! {
    todo!("0x4573e4 __ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEEC2EmRKS6_RKS8_RKSaINS1_8ptr_nodeIjEEE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GuiRoot>::shared_ptr<RBX::GuiRoot,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x457c98 — __ZN5boost10shared_ptrIN3RBX7GuiRootEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::GuiRoot>::shared_ptr<RBX::GuiRoot,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_457c98() -> ! {
    todo!("0x457c98 __ZN5boost10shared_ptrIN3RBX7GuiRootEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiRoot,RBX::GuiRoot>(rbx_core::SharedPtr<RBX::GuiRoot> const*,RBX::GuiRoot *)const")]
// 0x457d60 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7GuiRootES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiRoot,RBX::GuiRoot>(boost::shared_ptr<RBX::GuiRoot> const*,RBX::GuiRoot *)const
pub fn stub_457d60() -> ! {
    todo!("0x457d60 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7GuiRootES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x457e4c — __ZN5boost6detail12shared_countC2IPN3RBX7GuiRootENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_457e4c() -> ! {
    todo!("0x457e4c __ZN5boost6detail12shared_countC2IPN3RBX7GuiRootENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x457f54 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7GuiRootENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_457f54() -> ! {
    todo!("0x457f54 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7GuiRootENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x457f58 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7GuiRootENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_457f58() -> ! {
    todo!("0x457f58 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7GuiRootENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x457f5c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7GuiRootENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_457f5c() -> ! {
    todo!("0x457f5c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7GuiRootENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x457f7c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7GuiRootENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_457f7c() -> ! {
    todo!("0x457f7c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7GuiRootENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x457f94 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7GuiRootENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_457f94() -> ! {
    todo!("0x457f94 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7GuiRootENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Workspace>::shared_ptr<RBX::Workspace,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x457f98 — __ZN5boost10shared_ptrIN3RBX9WorkspaceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::Workspace>::shared_ptr<RBX::Workspace,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_457f98() -> ! {
    todo!("0x457f98 __ZN5boost10shared_ptrIN3RBX9WorkspaceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Workspace,RBX::Workspace>(rbx_core::SharedPtr<RBX::Workspace> const*,RBX::Workspace *)const")]
// 0x458060 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9WorkspaceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Workspace,RBX::Workspace>(boost::shared_ptr<RBX::Workspace> const*,RBX::Workspace *)const
pub fn stub_458060() -> ! {
    todo!("0x458060 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9WorkspaceES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x45814c — __ZN5boost6detail12shared_countC2IPN3RBX9WorkspaceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_45814c() -> ! {
    todo!("0x45814c __ZN5boost6detail12shared_countC2IPN3RBX9WorkspaceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x458254 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9WorkspaceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_458254() -> ! {
    todo!("0x458254 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9WorkspaceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x458258 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9WorkspaceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_458258() -> ! {
    todo!("0x458258 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9WorkspaceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x45825c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9WorkspaceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_45825c() -> ! {
    todo!("0x45825c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9WorkspaceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x45827c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9WorkspaceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_45827c() -> ! {
    todo!("0x45827c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9WorkspaceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x458294 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9WorkspaceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_458294() -> ! {
    todo!("0x458294 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9WorkspaceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ChangeHistoryService> RBX::Creatable<RBX::Instance>::create<RBX::ChangeHistoryService>(void)")]
// 0x45847c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_20ChangeHistoryServiceEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::ChangeHistoryService> RBX::Creatable<RBX::Instance>::create<RBX::ChangeHistoryService>(void)
pub fn stub_45847c() -> ! {
    todo!("0x45847c __ZN3RBX9CreatableINS_8InstanceEE6createINS_20ChangeHistoryServiceEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ChangeHistoryService>::shared_ptr<RBX::ChangeHistoryService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x45852c — __ZN5boost10shared_ptrIN3RBX20ChangeHistoryServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::ChangeHistoryService>::shared_ptr<RBX::ChangeHistoryService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_45852c() -> ! {
    todo!("0x45852c __ZN5boost10shared_ptrIN3RBX20ChangeHistoryServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChangeHistoryService,RBX::ChangeHistoryService>(rbx_core::SharedPtr<RBX::ChangeHistoryService> const*,RBX::ChangeHistoryService *)const")]
// 0x4585f4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_20ChangeHistoryServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChangeHistoryService,RBX::ChangeHistoryService>(boost::shared_ptr<RBX::ChangeHistoryService> const*,RBX::ChangeHistoryService *)const
pub fn stub_4585f4() -> ! {
    todo!("0x4585f4 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_20ChangeHistoryServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4586e0 — __ZN5boost6detail12shared_countC2IPN3RBX20ChangeHistoryServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4586e0() -> ! {
    todo!("0x4586e0 __ZN5boost6detail12shared_countC2IPN3RBX20ChangeHistoryServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4587e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ChangeHistoryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_4587e8() -> ! {
    todo!("0x4587e8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ChangeHistoryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x4587f0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ChangeHistoryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_4587f0() -> ! {
    todo!("0x4587f0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ChangeHistoryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x458810 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ChangeHistoryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_458810() -> ! {
    todo!("0x458810 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ChangeHistoryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x458828 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ChangeHistoryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_458828() -> ! {
    todo!("0x458828 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ChangeHistoryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>::~vector()")]
// 0x458e44 — __ZNSt6vectorIN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESaIS6_EED2Ev
// was: std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>::~vector()
pub fn stub_458e44() -> ! {
    todo!("0x458e44 __ZNSt6vectorIN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESaIS6_EED2Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel>::shared_ptr<RBX::DataModel,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x458f10 — __ZN5boost10shared_ptrIN3RBX9DataModelEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::DataModel>::shared_ptr<RBX::DataModel,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_458f10() -> ! {
    todo!("0x458f10 __ZN5boost10shared_ptrIN3RBX9DataModelEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::DataModel,RBX::DataModel>(rbx_core::SharedPtr<RBX::DataModel> const*,RBX::DataModel *)const")]
// 0x458fd8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9DataModelES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::DataModel,RBX::DataModel>(boost::shared_ptr<RBX::DataModel> const*,RBX::DataModel *)const
pub fn stub_458fd8() -> ! {
    todo!("0x458fd8 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9DataModelES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4590c4 — __ZN5boost6detail12shared_countC2IPN3RBX9DataModelENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4590c4() -> ! {
    todo!("0x4590c4 __ZN5boost6detail12shared_countC2IPN3RBX9DataModelENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4591cc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9DataModelENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_4591cc() -> ! {
    todo!("0x4591cc __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9DataModelENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4591d0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9DataModelENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_4591d0() -> ! {
    todo!("0x4591d0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9DataModelENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x4591d4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9DataModelENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_4591d4() -> ! {
    todo!("0x4591d4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9DataModelENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x4591f4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9DataModelENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_4591f4() -> ! {
    todo!("0x4591f4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9DataModelENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x45920c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9DataModelENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_45920c() -> ! {
    todo!("0x45920c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9DataModelENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel::GenericJob>::shared_ptr<RBX::DataModel::GenericJob>(RBX::DataModel::GenericJob *)")]
// 0x459210 — __ZN5boost10shared_ptrIN3RBX9DataModel10GenericJobEEC2IS3_EEPT_
// was: boost::shared_ptr<RBX::DataModel::GenericJob>::shared_ptr<RBX::DataModel::GenericJob>(RBX::DataModel::GenericJob *)
pub fn stub_459210() -> ! {
    todo!("0x459210 __ZN5boost10shared_ptrIN3RBX9DataModel10GenericJobEEC2IS3_EEPT_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::DataModel::GenericJob,RBX::DataModel::GenericJob>(rbx_core::SharedPtr<RBX::DataModel::GenericJob> const*,RBX::DataModel::GenericJob *)const")]
// 0x4592f8 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_9DataModel10GenericJobES7_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::DataModel::GenericJob,RBX::DataModel::GenericJob>(boost::shared_ptr<RBX::DataModel::GenericJob> const*,RBX::DataModel::GenericJob *)const
pub fn stub_4592f8() -> ! {
    todo!("0x4592f8 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_9DataModel10GenericJobES7_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::DataModel::GenericJob>(RBX::DataModel::GenericJob *)")]
// 0x4593dc — __ZN5boost6detail12shared_countC2IN3RBX9DataModel10GenericJobEEEPT_
// was: boost::detail::shared_count::shared_count<RBX::DataModel::GenericJob>(RBX::DataModel::GenericJob *)
pub fn stub_4593dc() -> ! {
    todo!("0x4593dc __ZN5boost6detail12shared_countC2IN3RBX9DataModel10GenericJobEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::~sp_counted_impl_p()")]
// 0x4594d4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10GenericJobEED1Ev
// was: boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::~sp_counted_impl_p()
pub fn stub_4594d4() -> ! {
    todo!("0x4594d4 __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10GenericJobEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::~sp_counted_impl_p()")]
// 0x4594d8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10GenericJobEED0Ev
// was: boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::~sp_counted_impl_p()
pub fn stub_4594d8() -> ! {
    todo!("0x4594d8 __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10GenericJobEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::dispose(void)")]
// 0x4594dc — __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10GenericJobEE7disposeEv
// was: boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::dispose(void)
pub fn stub_4594dc() -> ! {
    todo!("0x4594dc __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10GenericJobEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::get_deleter(std::type_info const&)")]
// 0x4594ec — __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10GenericJobEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::get_deleter(std::type_info const&)
pub fn stub_4594ec() -> ! {
    todo!("0x4594ec __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10GenericJobEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::get_untyped_deleter(void)")]
// 0x4594f0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10GenericJobEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::get_untyped_deleter(void)
pub fn stub_4594f0() -> ! {
    todo!("0x4594f0 __ZN5boost6detail17sp_counted_impl_pIN3RBX9DataModel10GenericJobEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Visit> RBX::Creatable<RBX::Instance>::create<RBX::Visit>(void)")]
// 0x459b9c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5VisitEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::Visit> RBX::Creatable<RBX::Instance>::create<RBX::Visit>(void)
pub fn stub_459b9c() -> ! {
    todo!("0x459b9c __ZN3RBX9CreatableINS_8InstanceEE6createINS_5VisitEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Visit>::shared_ptr<RBX::Visit,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x459c4c — __ZN5boost10shared_ptrIN3RBX5VisitEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::Visit>::shared_ptr<RBX::Visit,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_459c4c() -> ! {
    todo!("0x459c4c __ZN5boost10shared_ptrIN3RBX5VisitEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Visit,RBX::Visit>(rbx_core::SharedPtr<RBX::Visit> const*,RBX::Visit *)const")]
// 0x459d14 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5VisitES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Visit,RBX::Visit>(boost::shared_ptr<RBX::Visit> const*,RBX::Visit *)const
pub fn stub_459d14() -> ! {
    todo!("0x459d14 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5VisitES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x459e00 — __ZN5boost6detail12shared_countC2IPN3RBX5VisitENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_459e00() -> ! {
    todo!("0x459e00 __ZN5boost6detail12shared_countC2IPN3RBX5VisitENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x459f08 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5VisitENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_459f08() -> ! {
    todo!("0x459f08 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5VisitENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x459f0c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5VisitENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_459f0c() -> ! {
    todo!("0x459f0c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5VisitENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x459f10 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5VisitENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_459f10() -> ! {
    todo!("0x459f10 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5VisitENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x459f30 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5VisitENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_459f30() -> ! {
    todo!("0x459f30 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5VisitENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x459f48 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5VisitENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_459f48() -> ! {
    todo!("0x459f48 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5VisitENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::DataModel,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::DataModel::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// 0x45cf14 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9DataModelEFvbEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<1,RBX::DataModel,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::DataModel::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_45cf14() -> ! {
    todo!("0x45cf14 __ZNK3RBX10Reflection13EventDescImplILi1ENS_9DataModelEFvbEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,bool const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(bool const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")]
// 0x45d108 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKbNS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISC_T0_T1_EENSA_9list_av_2IT2_T3_E4typeEEEMSF_FSC_SG_ESJ_SK_
// was: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,bool const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(bool const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)
pub fn stub_45d108() -> ! {
    todo!("0x45d108 __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKbNS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISC_T0_T1_EENSA_9list_av_2IT2_T3_E4typeEEEMSF_FSC_SG_ESJ_SK_")
}

#[doc(alias = "__ZN5boost9function1IvbEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
// 0x45d228 — __ZN5boost9function1IvbEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// was: __ZN5boost9function1IvbEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
pub fn stub_45d228() -> ! {
    todo!("0x45d228 __ZN5boost9function1IvbEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function1<void,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")]
// 0x45d310 — __ZN5boost9function1IvbE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEEvT_
// was: void boost::function1<void,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)
pub fn stub_45d310() -> ! {
    todo!("0x45d310 __ZN5boost9function1IvbE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x45d408 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_45d408() -> ! {
    todo!("0x45d408 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,bool>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x45d428 — __ZNK5boost6detail8function13basic_vtable1IvbE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable1<void,bool>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_45d428() -> ! {
    todo!("0x45d428 __ZNK5boost6detail8function13basic_vtable1IvbE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::function<void ()(bool)>,1,void ()(bool)>::callable<rbx::signals::signal<void ()(bool)>*>(boost::function<void ()(bool)> const&,rbx::signals::signal<void ()(bool)>*)")]
// 0x45d710 — __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_EC2IPS4_EERKS8_T_
// was: rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::function<void ()(bool)>,1,void ()(bool)>::callable<rbx::signals::signal<void ()(bool)>*>(boost::function<void ()(bool)> const&,rbx::signals::signal<void ()(bool)>*)
pub fn stub_45d710() -> ! {
    todo!("0x45d710 __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_EC2IPS4_EERKS8_T_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::function<void ()(bool)>,1,void ()(bool)>::call(bool)")]
// 0x45d810 — __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_E4callEb
// was: rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::function<void ()(bool)>,1,void ()(bool)>::call(bool)
pub fn stub_45d810() -> ! {
    todo!("0x45d810 __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_E4callEb")
}

#[doc(alias = "non_virtual_thunk_to rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::function<void ()(bool)>,1,void ()(bool)>::call(bool)")]
// 0x45d818 — __ZThn4_N3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_E4callEb
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::function<void ()(bool)>,1,void ()(bool)>::call(bool)
pub fn stub_45d818() -> ! {
    todo!("0x45d818 __ZThn4_N3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_E4callEb")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::DataModel,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::DataModel::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// 0x45dd2c — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9DataModelEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<0,RBX::DataModel,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::DataModel::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_45dd2c() -> ! {
    todo!("0x45dd2c __ZNK3RBX10Reflection13EventDescImplILi0ENS_9DataModelEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::function<void ()(void)>,0,void ()(void)>::~callable()")]
// 0x45dfb8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost8functionIS3_EELi0ES3_ED1Ev
// was: rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::function<void ()(void)>,0,void ()(void)>::~callable()
pub fn stub_45dfb8() -> ! {
    todo!("0x45dfb8 __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost8functionIS3_EELi0ES3_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::function<void ()(void)>,0,void ()(void)>::~callable()")]
// 0x45e0c8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost8functionIS3_EELi0ES3_ED0Ev
// was: rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::function<void ()(void)>,0,void ()(void)>::~callable()
pub fn stub_45e0c8() -> ! {
    todo!("0x45e0c8 __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost8functionIS3_EELi0ES3_ED0Ev")
}

#[doc(alias = "RBX::DataModel::GearType RBX::Reflection::ArgHelper::getArg<RBX::DataModel::GearType,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::DataModel::GearType> const&,boost::disable_if<boost::is_same<RBX::DataModel::GearType,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// 0x45e4ec — __ZN3RBX10Reflection9ArgHelper6getArgINS_9DataModel8GearTypeELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// was: RBX::DataModel::GearType RBX::Reflection::ArgHelper::getArg<RBX::DataModel::GearType,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::DataModel::GearType> const&,boost::disable_if<boost::is_same<RBX::DataModel::GearType,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
pub fn stub_45e4ec() -> ! {
    todo!("0x45e4ec __ZN3RBX10Reflection9ArgHelper6getArgINS_9DataModel8GearTypeELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")
}

#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,RBX::DataModel::GearType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::DataModel::GearType &,boost::enable_if<boost::is_enum<RBX::DataModel::GearType>,void>::type *)")]
// 0x45e67c — __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_9DataModel8GearTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// was: bool RBX::Reflection::ArgHelper::try_enum<1,RBX::DataModel::GearType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::DataModel::GearType &,boost::enable_if<boost::is_enum<RBX::DataModel::GearType>,void>::type *)
pub fn stub_45e67c() -> ! {
    todo!("0x45e67c __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_9DataModel8GearTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE")
}

#[doc(alias = "RBX::DataModel::GearGenreSetting RBX::Reflection::ArgHelper::getArg<RBX::DataModel::GearGenreSetting,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::DataModel::GearGenreSetting> const&,boost::disable_if<boost::is_same<RBX::DataModel::GearGenreSetting,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// 0x4618d8 — __ZN3RBX10Reflection9ArgHelper6getArgINS_9DataModel16GearGenreSettingELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// was: RBX::DataModel::GearGenreSetting RBX::Reflection::ArgHelper::getArg<RBX::DataModel::GearGenreSetting,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::DataModel::GearGenreSetting> const&,boost::disable_if<boost::is_same<RBX::DataModel::GearGenreSetting,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
pub fn stub_4618d8() -> ! {
    todo!("0x4618d8 __ZN3RBX10Reflection9ArgHelper6getArgINS_9DataModel16GearGenreSettingELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")
}

#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,RBX::DataModel::GearGenreSetting>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::DataModel::GearGenreSetting &,boost::enable_if<boost::is_enum<RBX::DataModel::GearGenreSetting>,void>::type *)")]
// 0x461a68 — __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_9DataModel16GearGenreSettingEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// was: bool RBX::Reflection::ArgHelper::try_enum<1,RBX::DataModel::GearGenreSetting>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::DataModel::GearGenreSetting &,boost::enable_if<boost::is_enum<RBX::DataModel::GearGenreSetting>,void>::type *)
pub fn stub_461a68() -> ! {
    todo!("0x461a68 __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_9DataModel16GearGenreSettingEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE")
}

#[doc(alias = "RBX::DataModel::Genre RBX::Reflection::ArgHelper::getArg<RBX::DataModel::Genre,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::DataModel::Genre> const&,boost::disable_if<boost::is_same<RBX::DataModel::Genre,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// 0x461d6c — __ZN3RBX10Reflection9ArgHelper6getArgINS_9DataModel5GenreELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// was: RBX::DataModel::Genre RBX::Reflection::ArgHelper::getArg<RBX::DataModel::Genre,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::DataModel::Genre> const&,boost::disable_if<boost::is_same<RBX::DataModel::Genre,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
pub fn stub_461d6c() -> ! {
    todo!("0x461d6c __ZN3RBX10Reflection9ArgHelper6getArgINS_9DataModel5GenreELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")
}
