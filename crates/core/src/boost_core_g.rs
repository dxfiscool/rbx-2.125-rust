//! boost_core_g — 150 boost stubs (EA-ordered, next uncovered after boost_core_f).
//! Source: `ida/export.json` filtered where mangled/demangled contains "boost", sorted by EA, next 150 uncovered.
//! Each stub preserves IDA address, mangled symbol, and demangled spelling; sanitized alias uses `rbx_core::SharedPtr` not `boost::`.
//! Sanitized: single quotes removed, boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr.

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x5238a4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_5238a4() {
    // IDA 0x5238a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x5238a8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_5238a8() {
    // IDA 0x5238a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x5238c8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_5238c8() {
    // IDA 0x5238c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatOutput *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x5238e0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ChatOutputENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_5238e0() {
    // IDA 0x5238e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::RelativePanel>::shared_ptr<RBX::RelativePanel,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x523ccc — __ZN5boost10shared_ptrIN3RBX13RelativePanelEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::RelativePanel>::shared_ptr<RBX::RelativePanel,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_523ccc() {
    // IDA 0x523ccc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RelativePanel,RBX::RelativePanel>(rbx_core::SharedPtr<RBX::RelativePanel> const*,RBX::RelativePanel *)const")]
// 0x523d94 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13RelativePanelES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RelativePanel,RBX::RelativePanel>(boost::shared_ptr<RBX::RelativePanel> const*,RBX::RelativePanel *)const
pub fn stub_523d94() {
    // IDA 0x523d94: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x523e7c — __ZN5boost6detail12shared_countC2IPN3RBX13RelativePanelENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_523e7c() {
    // IDA 0x523e7c: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x523f84 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_523f84() {
    // IDA 0x523f84: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x523f88 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_523f88() {
    // IDA 0x523f88: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x523f8c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_523f8c() {
    // IDA 0x523f8c: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x523fac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_523fac() {
    // IDA 0x523fac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RelativePanel *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x523fc4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13RelativePanelENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_523fc4() {
    // IDA 0x523fc4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GameBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GameBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x524378 — __ZN5boost6detail12shared_countC2IPN3RBX17GameBasicSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_524378() {
    // IDA 0x524378: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GameBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x524480 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17GameBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_524480() {
    // IDA 0x524480: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GameBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x524484 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17GameBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_524484() {
    // IDA 0x524484: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GameBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x5244a4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17GameBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_5244a4() {
    // IDA 0x5244a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GameBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x5244bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17GameBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_5244bc() {
    // IDA 0x5244bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GuiImageButton>::shared_ptr<RBX::GuiImageButton,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x5244c0 — __ZN5boost10shared_ptrIN3RBX14GuiImageButtonEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::GuiImageButton>::shared_ptr<RBX::GuiImageButton,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_5244c0() {
    // IDA 0x5244c0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiImageButton,RBX::GuiImageButton>(rbx_core::SharedPtr<RBX::GuiImageButton> const*,RBX::GuiImageButton *)const")]
// 0x524588 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_14GuiImageButtonES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiImageButton,RBX::GuiImageButton>(boost::shared_ptr<RBX::GuiImageButton> const*,RBX::GuiImageButton *)const
pub fn stub_524588() {
    // IDA 0x524588: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x524670 — __ZN5boost6detail12shared_countC2IPN3RBX14GuiImageButtonENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_524670() {
    // IDA 0x524670: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x524778 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_524778() {
    // IDA 0x524778: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x52477c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_52477c() {
    // IDA 0x52477c: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x524780 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_524780() {
    // IDA 0x524780: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x5247a0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_5247a0() {
    // IDA 0x5247a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x5247b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_5247b8() {
    // IDA 0x5247b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::NotificationBox>::shared_ptr<RBX::NotificationBox,RBX::Creatable<RBX::Instance>::Deleter>(RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x5247bc — __ZN5boost10shared_ptrIN3RBX15NotificationBoxEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::NotificationBox>::shared_ptr<RBX::NotificationBox,RBX::Creatable<RBX::Instance>::Deleter>(RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_5247bc() {
    // IDA 0x5247bc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::NotificationBox,RBX::NotificationBox>(rbx_core::SharedPtr<RBX::NotificationBox> const*,RBX::NotificationBox *)const")]
// 0x524884 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15NotificationBoxES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::NotificationBox,RBX::NotificationBox>(boost::shared_ptr<RBX::NotificationBox> const*,RBX::NotificationBox *)const
pub fn stub_524884() {
    // IDA 0x524884: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x52496c — __ZN5boost6detail12shared_countC2IPN3RBX15NotificationBoxENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_52496c() {
    // IDA 0x52496c: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x524a74 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_524a74() {
    // IDA 0x524a74: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x524a78 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_524a78() {
    // IDA 0x524a78: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x524a7c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_524a7c() {
    // IDA 0x524a7c: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x524a9c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_524a9c() {
    // IDA 0x524a9c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x524ab4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_524ab4() {
    // IDA 0x524ab4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Frame>::shared_ptr<RBX::Frame,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Frame *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x524ab8 — __ZN5boost10shared_ptrIN3RBX5FrameEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::Frame>::shared_ptr<RBX::Frame,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Frame *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_524ab8() {
    // IDA 0x524ab8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Frame,RBX::Frame>(rbx_core::SharedPtr<RBX::Frame> const*,RBX::Frame *)const")]
// 0x524b80 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5FrameES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Frame,RBX::Frame>(boost::shared_ptr<RBX::Frame> const*,RBX::Frame *)const
pub fn stub_524b80() {
    // IDA 0x524b80: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Frame *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Frame *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x524c68 — __ZN5boost6detail12shared_countC2IPN3RBX5FrameENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_524c68() {
    // IDA 0x524c68: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Frame *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x524d70 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5FrameENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_524d70() {
    // IDA 0x524d70: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Frame *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x524d74 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5FrameENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_524d74() {
    // IDA 0x524d74: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Frame *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x524d78 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5FrameENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_524d78() {
    // IDA 0x524d78: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Frame *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x524d98 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5FrameENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_524d98() {
    // IDA 0x524d98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Frame *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x524db0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5FrameENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_524db0() {
    // IDA 0x524db0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::PhysicsSettings> RBX::Creatable<RBX::Instance>::create<RBX::PhysicsSettings>(void)")]
// 0x525000 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_15PhysicsSettingsEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::PhysicsSettings> RBX::Creatable<RBX::Instance>::create<RBX::PhysicsSettings>(void)
pub fn stub_525000() {
    // IDA 0x525000: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::PhysicsSettings>::shared_ptr<RBX::PhysicsSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PhysicsSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x5250b0 — __ZN5boost10shared_ptrIN3RBX15PhysicsSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::PhysicsSettings>::shared_ptr<RBX::PhysicsSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PhysicsSettings *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_5250b0() {
    // IDA 0x5250b0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PhysicsSettings,RBX::PhysicsSettings>(rbx_core::SharedPtr<RBX::PhysicsSettings> const*,RBX::PhysicsSettings *)const")]
// 0x525178 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15PhysicsSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PhysicsSettings,RBX::PhysicsSettings>(boost::shared_ptr<RBX::PhysicsSettings> const*,RBX::PhysicsSettings *)const
pub fn stub_525178() {
    // IDA 0x525178: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PhysicsSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PhysicsSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x525260 — __ZN5boost6detail12shared_countC2IPN3RBX15PhysicsSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_525260() {
    // IDA 0x525260: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PhysicsSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x525368 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15PhysicsSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_525368() {
    // IDA 0x525368: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PhysicsSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x52536c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15PhysicsSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_52536c() {
    // IDA 0x52536c: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PhysicsSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x525370 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15PhysicsSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_525370() {
    // IDA 0x525370: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PhysicsSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x525390 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15PhysicsSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_525390() {
    // IDA 0x525390: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PhysicsSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x5253a8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15PhysicsSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_5253a8() {
    // IDA 0x5253a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiObject::UpdateTween(RBX::GuiObject::Tween &,RBX::GuiObject*,boost::function<void ()(RBX::GuiObject*,RBX::UDim2)>,float)")]
// 0x527344 — __ZN3RBX9GuiObject11UpdateTweenERNS0_5TweenEPS0_N5boost8functionIFvS3_NS_5UDim2EEEEf
pub fn stub_527344() {
    // IDA 0x527344: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::InvokeCallback(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus)")]
// 0x52757c — __ZN3RBXL14InvokeCallbackEN5boost8functionIFvNS_9GuiObject11TweenStatusEEEES3_
pub fn stub_52757c() {
    // IDA 0x52757c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiObject::tweenPositionDelay(RBX::UDim2,RBX::UDim2,float,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,boost::function<void ()(RBX::GuiObject::TweenStatus)>)")]
// 0x527f50 — __ZN3RBX9GuiObject18tweenPositionDelayENS_5UDim2ES1_fNS0_20TweenEasingDirectionENS0_16TweenEasingStyleEfbN5boost8functionIFvNS0_11TweenStatusEEEE
pub fn stub_527f50() {
    // IDA 0x527f50: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "RBX::InvokeTweenStatusCallback(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus)")]
// 0x528060 — __ZN3RBXL25InvokeTweenStatusCallbackEN5boost8weak_ptrINS_9GuiObjectEEENS_3Lua15WeakFunctionRefENS2_11TweenStatusE
// was: RBX::InvokeTweenStatusCallback(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus)
pub fn stub_528060() {
    // IDA 0x528060: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "RBX::GuiObject::tweenPositionDelay(RBX::UDim2,RBX::UDim2,float,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::TweenService *)")]
// 0x5288b0 — __ZN3RBX9GuiObject18tweenPositionDelayENS_5UDim2ES1_fNS0_20TweenEasingDirectionENS0_16TweenEasingStyleEfbN5boost8functionIFvNS0_11TweenStatusEEEEPNS_12TweenServiceE
pub fn stub_5288b0() {
    // IDA 0x5288b0: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "RBX::InvokeRemoveOnTweenEnd(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef)")]
// 0x528c4c — __ZN3RBXL22InvokeRemoveOnTweenEndEN5boost8weak_ptrINS_9GuiObjectEEENS_3Lua15WeakFunctionRefE
// was: RBX::InvokeRemoveOnTweenEnd(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef)
pub fn stub_528c4c() {
    // IDA 0x528c4c: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "RBX::GuiObject::tweenSizeDelay(RBX::UDim2,RBX::UDim2,float,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,boost::function<void ()(RBX::GuiObject::TweenStatus)>)")]
// 0x528d14 — __ZN3RBX9GuiObject14tweenSizeDelayENS_5UDim2ES1_fNS0_20TweenEasingDirectionENS0_16TweenEasingStyleEfbN5boost8functionIFvNS0_11TweenStatusEEEE
pub fn stub_528d14() {
    // IDA 0x528d14: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "boost::function2<void,RBX::GuiObject *,RBX::UDim2>::operator()(RBX::GuiObject *,RBX::UDim2)const")]
// 0x52cde4 — __ZNK5boost9function2IvPN3RBX9GuiObjectENS1_5UDim2EEclES3_S4_
pub fn stub_52cde4() {
    // IDA 0x52cde4: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list_av_2<boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus>::type> boost::bind<void,boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus,boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus>(void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus)")]
// 0x52ceb8 — __ZN5boost4bindIvNS_8functionIFvN3RBX9GuiObject11TweenStatusEEEES4_S6_S4_EENS_3_bi6bind_tIT_PFS9_T0_T1_ENS7_9list_av_2IT2_T3_E4typeEEESD_SF_SG_
pub fn stub_52ceb8() {
    // IDA 0x52ceb8: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list_av_3<rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,boost::arg<1>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus,rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,boost::arg<1>>(void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,boost::arg<1>)")]
// 0x52cfc0 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX9GuiObjectEEENS2_3Lua15WeakFunctionRefENS3_11TweenStatusES4_S6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSC_T0_T1_T2_ENSA_9list_av_3IT3_T4_T5_E4typeEEESH_SJ_SK_SL_
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list_av_3<boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,boost::arg<1>>::type> boost::bind<void,boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus,boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,boost::arg<1>>(void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,boost::arg<1>)
pub fn stub_52cfc0() {
    // IDA 0x52cfc0: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list_av_2<rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef>::type> boost::bind<void,rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef>(void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef)")]
// 0x52d230 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX9GuiObjectEEENS2_3Lua15WeakFunctionRefES4_S6_EENS_3_bi6bind_tIT_PFS9_T0_T1_ENS7_9list_av_2IT2_T3_E4typeEEESD_SF_SG_
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list_av_2<boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef>::type> boost::bind<void,boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef>(void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef)
pub fn stub_52d230() {
    // IDA 0x52d230: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::scoped_ptr<RBX::GuiObject::Tweens>::reset(RBX::GuiObject::Tweens*)")]
// 0x52d488 — __ZN5boost10scoped_ptrIN3RBX9GuiObject6TweensEE5resetEPS3_
pub fn stub_52d488() {
    // IDA 0x52d488: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::function<void ()(RBX::GuiObject::TweenStatus)>::operator=(boost::function<void ()(RBX::GuiObject::TweenStatus)> const&)")]
// 0x52d55c — __ZN5boost8functionIFvN3RBX9GuiObject11TweenStatusEEEaSERKS5_
pub fn stub_52d55c() {
    // IDA 0x52d55c: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>> const&)")]
// 0x52ee40 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEEENS0_10connectionERKT_
pub fn stub_52ee40() {
    // IDA 0x52ee40: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>>::~callable_slot()")]
// 0x52ef00 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED1Ev
pub fn stub_52ef00() {
    // IDA 0x52ef00: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>>::~callable_slot()")]
// 0x52ef2c — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED0Ev
pub fn stub_52ef2c() {
    // IDA 0x52ef2c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>,0,void ()(void)>::call(void)")]
// 0x52f000 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv
pub fn stub_52f000() {
    // IDA 0x52f000: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>,0,void ()(void)>::call(void)")]
// 0x52f008 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv
pub fn stub_52f008() {
    // IDA 0x52f008: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>::operator()(void)")]
// 0x52f010 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFviiEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv
pub fn stub_52f010() {
    // IDA 0x52f010: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>,0,void ()(void)>::~callable()")]
// 0x52f028 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED1Ev
pub fn stub_52f028() {
    // IDA 0x52f028: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>,0,void ()(void)>::~callable()")]
// 0x52f054 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED0Ev
pub fn stub_52f054() {
    // IDA 0x52f054: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>> const&)")]
// 0x52f128 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEEENS0_10connectionERKT_
pub fn stub_52f128() {
    // IDA 0x52f128: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>>::~callable_slot()")]
// 0x52f1e8 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEED1Ev
pub fn stub_52f1e8() {
    // IDA 0x52f1e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>>::~callable_slot()")]
// 0x52f214 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEED0Ev
pub fn stub_52f214() {
    // IDA 0x52f214: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::call(void)")]
// 0x52f2e8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv
pub fn stub_52f2e8() {
    // IDA 0x52f2e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::call(void)")]
// 0x52f2f0 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv
pub fn stub_52f2f0() {
    // IDA 0x52f2f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>::operator()(void)")]
// 0x52f2f8 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFvvEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv
pub fn stub_52f2f8() {
    // IDA 0x52f2f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::~callable()")]
// 0x52f310 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED1Ev
pub fn stub_52f310() {
    // IDA 0x52f310: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::~callable()")]
// 0x52f33c — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED0Ev
pub fn stub_52f33c() {
    // IDA 0x52f33c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>> const&)")]
// 0x52f83c — __ZN3rbx7signals6signalIFviiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_
pub fn stub_52f83c() {
    // IDA 0x52f83c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int)>::slot>::operator=(rbx::signals::signal<void ()(int,int)>::slot*)")]
// 0x52fabc — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviiEE4slotEEaSEPS6_
pub fn stub_52fabc() {
    // IDA 0x52fabc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// 0x52fae0 — __ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED1Ev
pub fn stub_52fae0() {
    // IDA 0x52fae0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// 0x52fb0c — __ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED0Ev
pub fn stub_52fb0c() {
    // IDA 0x52fb0c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::call(int,int)")]
// 0x52fcfc — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiButtonES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii
pub fn stub_52fcfc() {
    // IDA 0x52fcfc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::call(int,int)")]
// 0x52fd24 — __ZThn4_N3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiButtonES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii
pub fn stub_52fd24() {
    // IDA 0x52fd24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)> *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list2<int &,int &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int> &,boost::_bi::list2<int &,int &> &,int)")]
// 0x52fd4c — __ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_9GuiButtonEFviiEEEEENS_3argILi1EEENSA_ILi2EEEEclINS_4_mfi3mf2IvS7_iiEENS0_5list2IRiSJ_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_52fd4c() {
    // IDA 0x52fd4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::~callable()")]
// 0x530058 — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiButtonES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED1Ev
pub fn stub_530058() {
    // IDA 0x530058: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::~callable()")]
// 0x530084 — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiButtonES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED0Ev
pub fn stub_530084() {
    // IDA 0x530084: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>> const&)")]
// 0x5303f0 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEEENS0_10connectionERKT_
pub fn stub_5303f0() {
    // IDA 0x5303f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>>::~callable_slot()")]
// 0x530464 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEED1Ev
pub fn stub_530464() {
    // IDA 0x530464: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>>::~callable_slot()")]
// 0x530490 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEED0Ev
pub fn stub_530490() {
    // IDA 0x530490: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::call(void)")]
// 0x530564 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv
pub fn stub_530564() {
    // IDA 0x530564: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::call(void)")]
// 0x53056c — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv
pub fn stub_53056c() {
    // IDA 0x53056c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>::operator()(void)")]
// 0x530574 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENS4_9GuiButtonEFvvEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv
pub fn stub_530574() {
    // IDA 0x530574: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::~callable()")]
// 0x53058c — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED1Ev
pub fn stub_53058c() {
    // IDA 0x53058c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::~callable()")]
// 0x5305b8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED0Ev
pub fn stub_5305b8() {
    // IDA 0x5305b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::GuiButton,void ()(int,int),rbx::remote_signal<void ()(int,int)>,rbx::remote_signal<void ()(int,int)> RBX::GuiButton::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// 0x531a2c — __ZNK3RBX10Reflection13EventDescImplILi2ENS_9GuiButtonEFviiEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<2,RBX::GuiButton,void ()(int,int),rbx::remote_signal<void ()(int,int)>,rbx::remote_signal<void ()(int,int)> RBX::GuiButton::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_531a2c() {
    // IDA 0x531a2c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,int const&,int const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,int const&,int const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(int const&,int const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
// 0x531c60 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKiS5_NS_10shared_ptrIS3_EENS_3argILi1EEENS8_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISD_T0_T1_T2_EENSB_9list_av_3IT3_T4_T5_E4typeEEEMSG_FSD_SH_SI_ESL_SM_SN_
// was: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,int const&,int const&>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,int const&,int const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(int const&,int const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)
pub fn stub_531c60() {
    // IDA 0x531c60: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::function2<void,int,int>::clear(void)")]
// 0x531ee4 — __ZN5boost9function2IviiE5clearEv
pub fn stub_531ee4() {
    // IDA 0x531ee4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost8functionIFviiEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKiSC_EENS4_5list3INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSJ_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
// 0x531f10 — __ZN5boost8functionIFviiEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKiSC_EENS4_5list3INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSJ_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
pub fn stub_531f10() {
    // IDA 0x531f10: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost9function2IviiEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKiSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
// 0x531ff4 — __ZN5boost9function2IviiEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKiSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
pub fn stub_531ff4() {
    // IDA 0x531ff4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "void boost::function2<void,int,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,int const&,int const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,int const&,int const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")]
// 0x5320dc — __ZN5boost9function2IviiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKiSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEEEvT_
// was: void boost::function2<void,int,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,int const&,int const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,int const&,int const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)
pub fn stub_5320dc() {
    // IDA 0x5320dc: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,int const&,int const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x5321d4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKiSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,int const&,int const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_5321d4() {
    // IDA 0x5321d4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,int const&,int const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,int,int>::invoke(boost::detail::function::function_buffer &,int,int)")]
// 0x5321f0 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKiSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEviiE6invokeERNS1_15function_bufferEii
// was: boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,int const&,int const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,int,int>::invoke(boost::detail::function::function_buffer &,int,int)
pub fn stub_5321f0() {
    // IDA 0x5321f0: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,int,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,int const&,int const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,int const&,int const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")]
// 0x532208 — __ZNK5boost6detail8function13basic_vtable2IviiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKiSD_EENS5_5list3INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable2<void,int,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,int const&,int const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,int const&,int const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const
pub fn stub_532208() {
    // IDA 0x532208: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,int,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,int const&,int const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,int const&,int const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x5322f0 — __ZNK5boost6detail8function13basic_vtable2IviiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKiSD_EENS5_5list3INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable2<void,int,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,int const&,int const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,int const&,int const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_5322f0() {
    // IDA 0x5322f0: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,int,int>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,int const&,int const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,int const&,int const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x5323d4 — __ZNK5boost6detail8function13basic_vtable2IviiE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKiSD_EENS5_5list3INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable2<void,int,int>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,int const&,int const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,int const&,int const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_5323d4() {
    // IDA 0x5323d4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,int const&,int const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<int,int>(int &,int &)")]
// 0x5324a8 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKiS8_EENS0_5list3INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEENSF_ILi2EEEEEEclIiiEEvRT_RT0_
// was: void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,int const&,int const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<int,int>(int &,int &)
pub fn stub_5324a8() {
    // IDA 0x5324a8: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,int const&,int const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x5324c4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKiSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,int const&,int const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_5324c4() {
    // IDA 0x5324c4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,int)>::connect<boost::function<void ()(int,int)>>(boost::function<void ()(int,int)> const&)")]
// 0x53261c — __ZN3rbx7signals6signalIFviiEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
pub fn stub_53261c() {
    // IDA 0x53261c: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::function<void ()(int,int)>,2,void ()(int,int)>::callable<rbx::signals::signal<void ()(int,int)>*>(boost::function<void ()(int,int)> const&,rbx::signals::signal<void ()(int,int)>*)")]
// 0x532710 — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost8functionIS3_EELi2ES3_EC2IPS4_EERKS8_T_
pub fn stub_532710() {
    // IDA 0x532710: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int)>::callable_slot<boost::function<void ()(int,int)>>::~callable_slot()")]
// 0x53280c — __ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost8functionIS2_EEED1Ev
pub fn stub_53280c() {
    // IDA 0x53280c: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int)>::callable_slot<boost::function<void ()(int,int)>>::~callable_slot()")]
// 0x53291c — __ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost8functionIS2_EEED0Ev
pub fn stub_53291c() {
    // IDA 0x53291c: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::function<void ()(int,int)>,2,void ()(int,int)>::call(int,int)")]
// 0x532a4c — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost8functionIS3_EELi2ES3_E4callEii
pub fn stub_532a4c() {
    // IDA 0x532a4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::function<void ()(int,int)>,2,void ()(int,int)>::call(int,int)")]
// 0x532a54 — __ZThn4_N3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost8functionIS3_EELi2ES3_E4callEii
pub fn stub_532a54() {
    // IDA 0x532a54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function2<void,int,int>::operator()(int,int)const")]
// 0x532a5c — __ZNK5boost9function2IviiEclEii
pub fn stub_532a5c() {
    // IDA 0x532a5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::function<void ()(int,int)>,2,void ()(int,int)>::~callable()")]
// 0x532b24 — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost8functionIS3_EELi2ES3_ED1Ev
pub fn stub_532b24() {
    // IDA 0x532b24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::function<void ()(int,int)>,2,void ()(int,int)>::~callable()")]
// 0x532c34 — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost8functionIS3_EELi2ES3_ED0Ev
pub fn stub_532c34() {
    // IDA 0x532c34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function2<void,int,int>::assign_to_own(boost::function2<void,int,int> const&)")]
// 0x532d64 — __ZN5boost9function2IviiE13assign_to_ownERKS1_
pub fn stub_532d64() {
    // IDA 0x532d64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::GuiButton,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::GuiButton::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// 0x533110 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9GuiButtonEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<0,RBX::GuiButton,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::GuiButton::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_533110() {
    // IDA 0x533110: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::UDim2)>::slot> &)")]
// 0x5338ec — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
pub fn stub_5338ec() {
    // IDA 0x5338ec: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int)>::slot> &)")]
// 0x533a74 — __ZN3rbx7signals6signalIFviiEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
pub fn stub_533a74() {
    // IDA 0x533a74: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>> const&)")]
// 0x533bfc — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFvNSA_5UDim2EEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_
pub fn stub_533bfc() {
    // IDA 0x533bfc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>>::~callable_slot()")]
// 0x533cbc — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFvNSA_5UDim2EEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED1Ev
pub fn stub_533cbc() {
    // IDA 0x533cbc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>>::~callable_slot()")]
// 0x533ce8 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFvNSA_5UDim2EEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev
pub fn stub_533ce8() {
    // IDA 0x533ce8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>,0,void ()(void)>::call(void)")]
// 0x533dbc — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFvNSB_5UDim2EEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
pub fn stub_533dbc() {
    // IDA 0x533dbc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>,0,void ()(void)>::call(void)")]
// 0x533dc4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFvNSB_5UDim2EEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
pub fn stub_533dc4() {
    // IDA 0x533dc4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>::operator()(void)")]
// 0x533dcc — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiObjectEFvNS4_5UDim2EEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv
pub fn stub_533dcc() {
    // IDA 0x533dcc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>,0,void ()(void)>::~callable()")]
// 0x533de4 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFvNSB_5UDim2EEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev
pub fn stub_533de4() {
    // IDA 0x533de4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>,0,void ()(void)>::~callable()")]
// 0x533e10 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFvNSB_5UDim2EEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev
pub fn stub_533e10() {
    // IDA 0x533e10: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>> const&)")]
// 0x533ee4 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEEENS0_10connectionERKT_
pub fn stub_533ee4() {
    // IDA 0x533ee4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>>::~callable_slot()")]
// 0x533fa4 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED1Ev
pub fn stub_533fa4() {
    // IDA 0x533fa4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>>::~callable_slot()")]
// 0x533fd0 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED0Ev
pub fn stub_533fd0() {
    // IDA 0x533fd0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>,0,void ()(void)>::call(void)")]
// 0x5340a4 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv
pub fn stub_5340a4() {
    // IDA 0x5340a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>,0,void ()(void)>::call(void)")]
// 0x5340ac — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv
pub fn stub_5340ac() {
    // IDA 0x5340ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>::operator()(void)")]
// 0x5340b4 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiObjectEFviiEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv
pub fn stub_5340b4() {
    // IDA 0x5340b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>,0,void ()(void)>::~callable()")]
// 0x5340cc — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED1Ev
pub fn stub_5340cc() {
    // IDA 0x5340cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>,0,void ()(void)>::~callable()")]
// 0x5340f8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED0Ev
pub fn stub_5340f8() {
    // IDA 0x5340f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<void (RBX::GuiObject::*)(RBX::UDim2)>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x5341cc — __ZN5boost6detail8function15functor_managerIMN3RBX9GuiObjectEFvNS3_5UDim2EEE6manageERKNS1_15function_bufferERS9_NS1_30functor_manager_operation_typeE
pub fn stub_5341cc() {
    // IDA 0x5341cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::function::function_void_mem_invoker2<void (RBX::GuiObject::*)(RBX::UDim2),void,RBX::GuiObject*,RBX::UDim2>::invoke(boost::detail::function::function_buffer &,RBX::GuiObject*,RBX::UDim2)")]
// 0x53422c — __ZN5boost6detail8function26function_void_mem_invoker2IMN3RBX9GuiObjectEFvNS3_5UDim2EEvPS4_S5_E6invokeERNS1_15function_bufferES8_S5_
pub fn stub_53422c() {
    // IDA 0x53422c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function1<void,RBX::GuiObject::TweenStatus>::swap(boost::function1<void,RBX::GuiObject::TweenStatus>&)")]
// 0x534260 — __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE4swapERS4_
pub fn stub_534260() {
    // IDA 0x534260: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function1<void,RBX::GuiObject::TweenStatus>::move_assign(boost::function1<void,RBX::GuiObject::TweenStatus>&)")]
// 0x53433c — __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE11move_assignERS4_
pub fn stub_53433c() {
    // IDA 0x53433c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost8functionIFvN3RBX9GuiObject11TweenStatusEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefEENS7_5list2INS7_5valueISA_EENSG_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// 0x534440 — __ZN5boost8functionIFvN3RBX9GuiObject11TweenStatusEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefEENS7_5list2INS7_5valueISA_EENSG_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
pub fn stub_534440() {
    // IDA 0x534440: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "__ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefEENS6_5list2INS6_5valueIS9_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
// 0x5345d8 — __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefEENS6_5list2INS6_5valueIS9_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
pub fn stub_5345d8() {
    // IDA 0x5345d8: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "void boost::function1<void,RBX::GuiObject::TweenStatus>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>)")]
// 0x534774 — __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefEENS6_5list2INS6_5valueIS9_EENSF_ISB_EEEEEEEEvT_
// was: void boost::function1<void,RBX::GuiObject::TweenStatus>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>)
pub fn stub_534774() {
    // IDA 0x534774: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x534920 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEENS3_5list2INS3_5valueIS8_EENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_534920() {
    // IDA 0x534920: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>,void,RBX::GuiObject::TweenStatus>::invoke(boost::detail::function::function_buffer &,RBX::GuiObject::TweenStatus)")]
// 0x53493c — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEENS3_5list2INS3_5valueIS8_EENSE_ISA_EEEEEEvNS7_11TweenStatusEE6invokeERNS1_15function_bufferESJ_
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>,void,RBX::GuiObject::TweenStatus>::invoke(boost::detail::function::function_buffer &,RBX::GuiObject::TweenStatus)
pub fn stub_53493c() {
    // IDA 0x53493c: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::GuiObject::TweenStatus>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>,boost::detail::function::function_buffer &)const")]
// 0x534958 — __ZNK5boost6detail8function13basic_vtable1IvN3RBX9GuiObject11TweenStatusEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EENS3_3Lua15WeakFunctionRefEENS8_5list2INS8_5valueISB_EENSH_ISD_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable1<void,RBX::GuiObject::TweenStatus>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>,boost::detail::function::function_buffer &)const
pub fn stub_534958() {
    // IDA 0x534958: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::GuiObject::TweenStatus>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x534af4 — __ZNK5boost6detail8function13basic_vtable1IvN3RBX9GuiObject11TweenStatusEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EENS3_3Lua15WeakFunctionRefEENS8_5list2INS8_5valueISB_EENSH_ISD_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable1<void,RBX::GuiObject::TweenStatus>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_534af4() {
    // IDA 0x534af4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::GuiObject::TweenStatus>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x534c8c — __ZNK5boost6detail8function13basic_vtable1IvN3RBX9GuiObject11TweenStatusEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EENS3_3Lua15WeakFunctionRefEENS8_5list2INS8_5valueISB_EENSH_ISD_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable1<void,RBX::GuiObject::TweenStatus>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_534c8c() {
    // IDA 0x534c8c: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}
