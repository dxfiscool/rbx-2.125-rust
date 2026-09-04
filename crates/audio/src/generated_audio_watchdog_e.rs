//! audio generated_audio_watchdog_e — 100 stubs EA-sorted asc gap filler not yet in audio (FMOD|Sound|Audio 2544/2544 complete, gap filler)
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 100 not in audio | rbx_core::SharedPtr not boost
//! Range 0x524778..0x529eac | existing 30112 -> 30212 distinct
//! Batch: 100 stubs | 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR mangled")

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x524778 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_524778() {
    // IDA 0x524778: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x52477c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_52477c() {
    // IDA 0x52477c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x524780 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_524780() {
    // IDA 0x524780: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x5247a0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_5247a0() {
    // IDA 0x5247a0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x5247b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiImageButton *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX14GuiImageButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_5247b8() {
    // IDA 0x5247b8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x5247bc — __ZN5boost10shared_ptrIN3RBX15NotificationBoxEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::NotificationBox>::shared_ptr<RBX::NotificationBox,RBX::Creatable<RBX::Instance>::Deleter>(RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX15NotificationBoxEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_5247bc() -> ! {
    todo!("0x5247bc __ZN5boost10shared_ptrIN3RBX15NotificationBoxEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

// 0x524884 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15NotificationBoxES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::NotificationBox,RBX::NotificationBox>(rbx_core::SharedPtr<RBX::NotificationBox> const*,RBX::NotificationBox *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15NotificationBoxES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_524884() {
    // IDA 0x524884: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x52496c — __ZN5boost6detail12shared_countC2IPN3RBX15NotificationBoxENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX15NotificationBoxENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_52496c() {
    // IDA 0x52496c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x524a74 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_524a74() {
    // IDA 0x524a74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x524a78 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_524a78() {
    // IDA 0x524a78: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x524a7c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_524a7c() {
    // IDA 0x524a7c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x524a9c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_524a9c() {
    // IDA 0x524a9c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x524ab4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NotificationBox *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NotificationBoxENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_524ab4() {
    // IDA 0x524ab4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x524ab8 — __ZN5boost10shared_ptrIN3RBX5FrameEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Frame>::shared_ptr<RBX::Frame,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Frame *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX5FrameEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_524ab8() -> ! {
    todo!("0x524ab8 __ZN5boost10shared_ptrIN3RBX5FrameEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

// 0x524b80 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5FrameES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Frame,RBX::Frame>(rbx_core::SharedPtr<RBX::Frame> const*,RBX::Frame *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5FrameES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_524b80() {
    // IDA 0x524b80: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x524c68 — __ZN5boost6detail12shared_countC2IPN3RBX5FrameENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Frame *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Frame *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX5FrameENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_524c68() {
    // IDA 0x524c68: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x524d70 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5FrameENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Frame *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5FrameENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_524d70() {
    // IDA 0x524d70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x524d74 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5FrameENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Frame *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5FrameENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_524d74() {
    // IDA 0x524d74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x524d78 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5FrameENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Frame *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5FrameENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_524d78() {
    // IDA 0x524d78: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x524d98 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5FrameENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Frame *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5FrameENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_524d98() {
    // IDA 0x524d98: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x524db0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5FrameENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Frame *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5FrameENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_524db0() {
    // IDA 0x524db0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x524db4 — __ZN3RBX14FactoryProductINS_15PhysicsSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sPhysicsSettingsEENS_8InstanceEE7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_15PhysicsSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sPhysicsSettingsEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_524db4() {
    // IDA 0x524db4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x524e50 — __ZNK3RBX14FactoryProductINS_15PhysicsSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sPhysicsSettingsEENS_8InstanceEE7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_15PhysicsSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sPhysicsSettingsEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_524e50() -> ! {
    todo!("0x524e50 __ZNK3RBX14FactoryProductINS_15PhysicsSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sPhysicsSettingsEENS_8InstanceEE7Creator12getClassNameEv")
}

// 0x524ebc — __ZNK3RBX14FactoryProductINS_15PhysicsSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sPhysicsSettingsEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_15PhysicsSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sPhysicsSettingsEENS_8InstanceEE7Creator6createEv")]
pub fn stub_524ebc() -> ! {
    todo!("0x524ebc __ZNK3RBX14FactoryProductINS_15PhysicsSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sPhysicsSettingsEENS_8InstanceEE7Creator6createEv")
}

// 0x525000 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_15PhysicsSettingsEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::PhysicsSettings> RBX::Creatable<RBX::Instance>::create<RBX::PhysicsSettings>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_15PhysicsSettingsEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_525000() -> ! {
    todo!("0x525000 __ZN3RBX9CreatableINS_8InstanceEE6createINS_15PhysicsSettingsEEEN5boost10shared_ptrIT_EEv")
}

// 0x5250b0 — __ZN5boost10shared_ptrIN3RBX15PhysicsSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::PhysicsSettings>::shared_ptr<RBX::PhysicsSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PhysicsSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX15PhysicsSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_5250b0() -> ! {
    todo!("0x5250b0 __ZN5boost10shared_ptrIN3RBX15PhysicsSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

// 0x525178 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15PhysicsSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PhysicsSettings,RBX::PhysicsSettings>(rbx_core::SharedPtr<RBX::PhysicsSettings> const*,RBX::PhysicsSettings *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15PhysicsSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_525178() {
    // IDA 0x525178: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x525260 — __ZN5boost6detail12shared_countC2IPN3RBX15PhysicsSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PhysicsSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PhysicsSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX15PhysicsSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_525260() {
    // IDA 0x525260: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x525368 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15PhysicsSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PhysicsSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX15PhysicsSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_525368() {
    // IDA 0x525368: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x52536c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15PhysicsSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PhysicsSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX15PhysicsSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_52536c() {
    // IDA 0x52536c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x525370 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15PhysicsSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PhysicsSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX15PhysicsSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_525370() {
    // IDA 0x525370: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x525390 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15PhysicsSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PhysicsSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX15PhysicsSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_525390() {
    // IDA 0x525390: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x5253a8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15PhysicsSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PhysicsSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX15PhysicsSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_5253a8() {
    // IDA 0x5253a8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x5253ac — __ZN3RBX4Name7declareILZNS_16sPhysicsSettingsEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_16sPhysicsSettingsEEEERKS0_v")]
pub fn stub_5253ac() -> ! {
    todo!("0x5253ac __ZN3RBX4Name7declareILZNS_16sPhysicsSettingsEEEERKS0_v")
}

// 0x5253f0 — __ZN3RBX4Name13callDoDeclareILZNS_16sPhysicsSettingsEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_16sPhysicsSettingsEEEEvv")]
pub fn stub_5253f0() -> ! {
    todo!("0x5253f0 __ZN3RBX4Name13callDoDeclareILZNS_16sPhysicsSettingsEEEEvv")
}

// 0x5253f4 — __ZN3RBX4Name9doDeclareILZNS_16sPhysicsSettingsEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sPhysicsSettingsEEEERKS0_v")]
pub fn stub_5253f4() -> ! {
    todo!("0x5253f4 __ZN3RBX4Name9doDeclareILZNS_16sPhysicsSettingsEEEERKS0_v")
}

// 0x5254d8 — __ZN3RBX14FactoryProductINS_15PhysicsSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sPhysicsSettingsEENS_8InstanceEE7CreatorC2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_15PhysicsSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sPhysicsSettingsEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_5254d8() -> ! {
    todo!("0x5254d8 __ZN3RBX14FactoryProductINS_15PhysicsSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sPhysicsSettingsEENS_8InstanceEE7CreatorC2Ev")
}

// 0x525700 — __ZNSt4pairIKSsN3RBX10GuiBuilder4DataEEC2ISsS3_EERKS_IT_T0_E
#[doc(alias = "std::pair<std::string const,RBX::GuiBuilder::Data>::pair<std::string,RBX::GuiBuilder::Data>(std::pair const&<std::string,RBX::GuiBuilder::Data>)")]
#[doc(alias = "__ZNSt4pairIKSsN3RBX10GuiBuilder4DataEEC2ISsS3_EERKS_IT_T0_E")]
pub fn stub_525700() -> ! {
    todo!("0x525700 __ZNSt4pairIKSsN3RBX10GuiBuilder4DataEEC2ISsS3_EERKS_IT_T0_E")
}

// 0x5257e0 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::_M_insert_unique(std::pair<std::string const,RBX::GuiBuilder::Data> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_")]
pub fn stub_5257e0() -> ! {
    todo!("0x5257e0 __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_")
}

// 0x525864 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,RBX::GuiBuilder::Data> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")]
pub fn stub_525864() -> ! {
    todo!("0x525864 __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")
}

// 0x5258b4 — __ZNSt4pairISsN3RBX10GuiBuilder4DataEEC2ERKSsRKS2_
#[doc(alias = "std::pair<std::string,RBX::GuiBuilder::Data>::pair(std::string const&,RBX::GuiBuilder::Data const&)")]
#[doc(alias = "__ZNSt4pairISsN3RBX10GuiBuilder4DataEEC2ERKSsRKS2_")]
pub fn stub_5258b4() -> ! {
    todo!("0x5258b4 __ZNSt4pairISsN3RBX10GuiBuilder4DataEEC2ERKSsRKS2_")
}

// 0x525994 — __GLOBAL__I_a_206
#[doc(alias = "global constructor keyed to_a_206")]
#[doc(alias = "__GLOBAL__I_a_206")]
pub fn stub_525994() -> ! {
    todo!("0x525994 __GLOBAL__I_a_206")
}

// 0x525cc8 — __ZN3RBX9GuiObject20tweenSizeAndPositionENS_5UDim2ES1_NS0_20TweenEasingDirectionENS0_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefE
#[doc(alias = "RBX::GuiObject::tweenSizeAndPosition(RBX::UDim2,RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef)")]
#[doc(alias = "__ZN3RBX9GuiObject20tweenSizeAndPositionENS_5UDim2ES1_NS0_20TweenEasingDirectionENS0_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefE")]
pub fn stub_525cc8() -> ! {
    todo!("0x525cc8 __ZN3RBX9GuiObject20tweenSizeAndPositionENS_5UDim2ES1_NS0_20TweenEasingDirectionENS0_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefE")
}

// 0x525e50 — __ZN3RBX9GuiObject13tweenPositionENS_5UDim2ENS0_20TweenEasingDirectionENS0_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefE
#[doc(alias = "RBX::GuiObject::tweenPosition(RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef)")]
#[doc(alias = "__ZN3RBX9GuiObject13tweenPositionENS_5UDim2ENS0_20TweenEasingDirectionENS0_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefE")]
pub fn stub_525e50() -> ! {
    todo!("0x525e50 __ZN3RBX9GuiObject13tweenPositionENS_5UDim2ENS0_20TweenEasingDirectionENS0_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefE")
}

// 0x526058 — __ZN3RBX9GuiObject9tweenSizeENS_5UDim2ENS0_20TweenEasingDirectionENS0_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefE
#[doc(alias = "RBX::GuiObject::tweenSize(RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef)")]
#[doc(alias = "__ZN3RBX9GuiObject9tweenSizeENS_5UDim2ENS0_20TweenEasingDirectionENS0_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefE")]
pub fn stub_526058() -> ! {
    todo!("0x526058 __ZN3RBX9GuiObject9tweenSizeENS_5UDim2ENS0_20TweenEasingDirectionENS0_16TweenEasingStyleEfbNS_3Lua15WeakFunctionRefE")
}

// 0x526260 — __ZN3RBX9GuiObject7setSizeENS_5UDim2E
#[doc(alias = "RBX::GuiObject::setSize(RBX::UDim2)")]
#[doc(alias = "__ZN3RBX9GuiObject7setSizeENS_5UDim2E")]
pub fn stub_526260() -> ! {
    todo!("0x526260 __ZN3RBX9GuiObject7setSizeENS_5UDim2E")
}

// 0x5262dc — __ZN3RBX9GuiObject11setPositionENS_5UDim2E
#[doc(alias = "RBX::GuiObject::setPosition(RBX::UDim2)")]
#[doc(alias = "__ZN3RBX9GuiObject11setPositionENS_5UDim2E")]
pub fn stub_5262dc() -> ! {
    todo!("0x5262dc __ZN3RBX9GuiObject11setPositionENS_5UDim2E")
}

// 0x526358 — __ZN3RBX9GuiObject18setBorderSizePixelEi
#[doc(alias = "RBX::GuiObject::setBorderSizePixel(int)")]
#[doc(alias = "__ZN3RBX9GuiObject18setBorderSizePixelEi")]
pub fn stub_526358() -> ! {
    todo!("0x526358 __ZN3RBX9GuiObject18setBorderSizePixelEi")
}

// 0x526398 — __ZN3RBX9GuiObject9setZIndexEi
#[doc(alias = "RBX::GuiObject::setZIndex(int)")]
#[doc(alias = "__ZN3RBX9GuiObject9setZIndexEi")]
pub fn stub_526398() -> ! {
    todo!("0x526398 __ZN3RBX9GuiObject9setZIndexEi")
}

// 0x5263ec — __ZN3RBX9GuiObject17setSizeConstraintENS0_14SizeConstraintE
#[doc(alias = "RBX::GuiObject::setSizeConstraint(RBX::GuiObject::SizeConstraint)")]
#[doc(alias = "__ZN3RBX9GuiObject17setSizeConstraintENS0_14SizeConstraintE")]
pub fn stub_5263ec() -> ! {
    todo!("0x5263ec __ZN3RBX9GuiObject17setSizeConstraintENS0_14SizeConstraintE")
}

// 0x526424 — __ZN3RBX9GuiObject14setBorderColorENS_10BrickColorE
#[doc(alias = "RBX::GuiObject::setBorderColor(RBX::BrickColor)")]
#[doc(alias = "__ZN3RBX9GuiObject14setBorderColorENS_10BrickColorE")]
pub fn stub_526424() -> ! {
    todo!("0x526424 __ZN3RBX9GuiObject14setBorderColorENS_10BrickColorE")
}

// 0x526444 — __ZN3RBX9GuiObject15setBorderColor3EN3G3D6Color3E
#[doc(alias = "RBX::GuiObject::setBorderColor3(G3D::Color3)")]
#[doc(alias = "__ZN3RBX9GuiObject15setBorderColor3EN3G3D6Color3E")]
pub fn stub_526444() -> ! {
    todo!("0x526444 __ZN3RBX9GuiObject15setBorderColor3EN3G3D6Color3E")
}

// 0x5264c4 — __ZN3RBX9GuiObject18setBackgroundColorENS_10BrickColorE
#[doc(alias = "RBX::GuiObject::setBackgroundColor(RBX::BrickColor)")]
#[doc(alias = "__ZN3RBX9GuiObject18setBackgroundColorENS_10BrickColorE")]
pub fn stub_5264c4() -> ! {
    todo!("0x5264c4 __ZN3RBX9GuiObject18setBackgroundColorENS_10BrickColorE")
}

// 0x5264e4 — __ZN3RBX9GuiObject19setBackgroundColor3EN3G3D6Color3E
#[doc(alias = "RBX::GuiObject::setBackgroundColor3(G3D::Color3)")]
#[doc(alias = "__ZN3RBX9GuiObject19setBackgroundColor3EN3G3D6Color3E")]
pub fn stub_5264e4() -> ! {
    todo!("0x5264e4 __ZN3RBX9GuiObject19setBackgroundColor3EN3G3D6Color3E")
}

// 0x526564 — __ZN3RBX9GuiObject25setBackgroundTransparencyEf
#[doc(alias = "RBX::GuiObject::setBackgroundTransparency(float)")]
#[doc(alias = "__ZN3RBX9GuiObject25setBackgroundTransparencyEf")]
pub fn stub_526564() -> ! {
    todo!("0x526564 __ZN3RBX9GuiObject25setBackgroundTransparencyEf")
}

// 0x526590 — __ZN3RBX9GuiObject12setDraggableEb
#[doc(alias = "RBX::GuiObject::setDraggable(bool)")]
#[doc(alias = "__ZN3RBX9GuiObject12setDraggableEb")]
pub fn stub_526590() -> ! {
    todo!("0x526590 __ZN3RBX9GuiObject12setDraggableEb")
}

// 0x5265b0 — __ZN3RBX9GuiObject11setClippingEb
#[doc(alias = "RBX::GuiObject::setClipping(bool)")]
#[doc(alias = "__ZN3RBX9GuiObject11setClippingEb")]
pub fn stub_5265b0() -> ! {
    todo!("0x5265b0 __ZN3RBX9GuiObject11setClippingEb")
}

// 0x5265d0 — __ZN3RBX9GuiObject10setVisibleEb
#[doc(alias = "RBX::GuiObject::setVisible(bool)")]
#[doc(alias = "__ZN3RBX9GuiObject10setVisibleEb")]
pub fn stub_5265d0() -> ! {
    todo!("0x5265d0 __ZN3RBX9GuiObject10setVisibleEb")
}

// 0x526608 — __ZN3RBX9GuiObject9setActiveEb
#[doc(alias = "RBX::GuiObject::setActive(bool)")]
#[doc(alias = "__ZN3RBX9GuiObject9setActiveEb")]
pub fn stub_526608() -> ! {
    todo!("0x526608 __ZN3RBX9GuiObject9setActiveEb")
}

// 0x526640 — __ZN3RBX10Reflection8EnumDescINS_9GuiObject20TweenEasingDirectionEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiObject::TweenEasingDirection>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9GuiObject20TweenEasingDirectionEEC1Ev")]
pub fn stub_526640() -> ! {
    todo!("0x526640 __ZN3RBX10Reflection8EnumDescINS_9GuiObject20TweenEasingDirectionEEC1Ev")
}

// 0x526644 — __ZN3RBX10Reflection8EnumDescINS_9GuiObject20TweenEasingDirectionEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiObject::TweenEasingDirection>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9GuiObject20TweenEasingDirectionEEC2Ev")]
pub fn stub_526644() -> ! {
    todo!("0x526644 __ZN3RBX10Reflection8EnumDescINS_9GuiObject20TweenEasingDirectionEEC2Ev")
}

// 0x526820 — __ZN3RBX10Reflection8EnumDescINS_9GuiObject16TweenEasingStyleEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiObject::TweenEasingStyle>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9GuiObject16TweenEasingStyleEEC1Ev")]
pub fn stub_526820() -> ! {
    todo!("0x526820 __ZN3RBX10Reflection8EnumDescINS_9GuiObject16TweenEasingStyleEEC1Ev")
}

// 0x526824 — __ZN3RBX10Reflection8EnumDescINS_9GuiObject16TweenEasingStyleEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiObject::TweenEasingStyle>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9GuiObject16TweenEasingStyleEEC2Ev")]
pub fn stub_526824() -> ! {
    todo!("0x526824 __ZN3RBX10Reflection8EnumDescINS_9GuiObject16TweenEasingStyleEEC2Ev")
}

// 0x526a70 — __ZN3RBX10Reflection8EnumDescINS_9GuiObject11TweenStatusEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiObject::TweenStatus>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9GuiObject11TweenStatusEEC1Ev")]
pub fn stub_526a70() -> ! {
    todo!("0x526a70 __ZN3RBX10Reflection8EnumDescINS_9GuiObject11TweenStatusEEC1Ev")
}

// 0x526a74 — __ZN3RBX10Reflection8EnumDescINS_9GuiObject11TweenStatusEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiObject::TweenStatus>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9GuiObject11TweenStatusEEC2Ev")]
pub fn stub_526a74() -> ! {
    todo!("0x526a74 __ZN3RBX10Reflection8EnumDescINS_9GuiObject11TweenStatusEEC2Ev")
}

// 0x526c38 — __ZN3RBX15StringConverterINS_9GuiObject16TweenEasingStyleEE14convertToValueERKSsRS2_
#[doc(alias = "RBX::StringConverter<RBX::GuiObject::TweenEasingStyle>::convertToValue(std::string const&,RBX::GuiObject::TweenEasingStyle&)")]
#[doc(alias = "__ZN3RBX15StringConverterINS_9GuiObject16TweenEasingStyleEE14convertToValueERKSsRS2_")]
pub fn stub_526c38() -> ! {
    todo!("0x526c38 __ZN3RBX15StringConverterINS_9GuiObject16TweenEasingStyleEE14convertToValueERKSsRS2_")
}

// 0x526c84 — __ZN3RBX15StringConverterINS_9GuiObject20TweenEasingDirectionEE14convertToValueERKSsRS2_
#[doc(alias = "RBX::StringConverter<RBX::GuiObject::TweenEasingDirection>::convertToValue(std::string const&,RBX::GuiObject::TweenEasingDirection&)")]
#[doc(alias = "__ZN3RBX15StringConverterINS_9GuiObject20TweenEasingDirectionEE14convertToValueERKSsRS2_")]
pub fn stub_526c84() -> ! {
    todo!("0x526c84 __ZN3RBX15StringConverterINS_9GuiObject20TweenEasingDirectionEE14convertToValueERKSsRS2_")
}

// 0x526cd0 — __ZN3RBX9GuiObjectC2EPKcb
#[doc(alias = "RBX::GuiObject::GuiObject(char const*,bool)")]
#[doc(alias = "__ZN3RBX9GuiObjectC2EPKcb")]
pub fn stub_526cd0() -> ! {
    todo!("0x526cd0 __ZN3RBX9GuiObjectC2EPKcb")
}

// 0x527344 — __ZN3RBX9GuiObject11UpdateTweenERNS0_5TweenEPS0_N5boost8functionIFvS3_NS_5UDim2EEEEf
#[doc(alias = "RBX::GuiObject::UpdateTween(RBX::GuiObject::Tween &,RBX::GuiObject*,boost::function<void ()(RBX::GuiObject*,RBX::UDim2)>,float)")]
#[doc(alias = "__ZN3RBX9GuiObject11UpdateTweenERNS0_5TweenEPS0_N5boost8functionIFvS3_NS_5UDim2EEEEf")]
pub fn stub_527344() -> ! {
    todo!("0x527344 __ZN3RBX9GuiObject11UpdateTweenERNS0_5TweenEPS0_N5boost8functionIFvS3_NS_5UDim2EEEEf")
}

// 0x52757c — __ZN3RBXL14InvokeCallbackEN5boost8functionIFvNS_9GuiObject11TweenStatusEEEES3_
#[doc(alias = "RBX::InvokeCallback(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus)")]
#[doc(alias = "__ZN3RBXL14InvokeCallbackEN5boost8functionIFvNS_9GuiObject11TweenStatusEEEES3_")]
pub fn stub_52757c() -> ! {
    todo!("0x52757c __ZN3RBXL14InvokeCallbackEN5boost8functionIFvNS_9GuiObject11TweenStatusEEEES3_")
}

// 0x527580 — __ZN3RBX9GuiObject16TweenInterpolateENS0_20TweenEasingDirectionENS0_16TweenEasingStyleEffRKNS_5UDim2ES5_
#[doc(alias = "RBX::GuiObject::TweenInterpolate(RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,float,RBX::UDim2 const&,RBX::UDim2 const&)")]
#[doc(alias = "__ZN3RBX9GuiObject16TweenInterpolateENS0_20TweenEasingDirectionENS0_16TweenEasingStyleEffRKNS_5UDim2ES5_")]
pub fn stub_527580() -> ! {
    todo!("0x527580 __ZN3RBX9GuiObject16TweenInterpolateENS0_20TweenEasingDirectionENS0_16TweenEasingStyleEffRKNS_5UDim2ES5_")
}

// 0x527f50 — __ZN3RBX9GuiObject18tweenPositionDelayENS_5UDim2ES1_fNS0_20TweenEasingDirectionENS0_16TweenEasingStyleEfbN5boost8functionIFvNS0_11TweenStatusEEEE
#[doc(alias = "RBX::GuiObject::tweenPositionDelay(RBX::UDim2,RBX::UDim2,float,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,boost::function<void ()(RBX::GuiObject::TweenStatus)>)")]
#[doc(alias = "__ZN3RBX9GuiObject18tweenPositionDelayENS_5UDim2ES1_fNS0_20TweenEasingDirectionENS0_16TweenEasingStyleEfbN5boost8functionIFvNS0_11TweenStatusEEEE")]
pub fn stub_527f50() -> ! {
    todo!("0x527f50 __ZN3RBX9GuiObject18tweenPositionDelayENS_5UDim2ES1_fNS0_20TweenEasingDirectionENS0_16TweenEasingStyleEfbN5boost8functionIFvNS0_11TweenStatusEEEE")
}

// 0x528060 — __ZN3RBXL25InvokeTweenStatusCallbackEN5boost8weak_ptrINS_9GuiObjectEEENS_3Lua15WeakFunctionRefENS2_11TweenStatusE
#[doc(alias = "RBX::InvokeTweenStatusCallback(rbx_core::Weak<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus)")]
#[doc(alias = "__ZN3RBXL25InvokeTweenStatusCallbackEN5boost8weak_ptrINS_9GuiObjectEEENS_3Lua15WeakFunctionRefENS2_11TweenStatusE")]
pub fn stub_528060() -> ! {
    todo!("0x528060 __ZN3RBXL25InvokeTweenStatusCallbackEN5boost8weak_ptrINS_9GuiObjectEEENS_3Lua15WeakFunctionRefENS2_11TweenStatusE")
}

// 0x528528 — __ZN3RBX9GuiObject13tweenPositionENS_5UDim2ENS0_20TweenEasingDirectionENS0_16TweenEasingStyleEfbb
#[doc(alias = "RBX::GuiObject::tweenPosition(RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,bool)")]
#[doc(alias = "__ZN3RBX9GuiObject13tweenPositionENS_5UDim2ENS0_20TweenEasingDirectionENS0_16TweenEasingStyleEfbb")]
pub fn stub_528528() -> ! {
    todo!("0x528528 __ZN3RBX9GuiObject13tweenPositionENS_5UDim2ENS0_20TweenEasingDirectionENS0_16TweenEasingStyleEfbb")
}

// 0x5288b0 — __ZN3RBX9GuiObject18tweenPositionDelayENS_5UDim2ES1_fNS0_20TweenEasingDirectionENS0_16TweenEasingStyleEfbN5boost8functionIFvNS0_11TweenStatusEEEEPNS_12TweenServiceE
#[doc(alias = "RBX::GuiObject::tweenPositionDelay(RBX::UDim2,RBX::UDim2,float,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::TweenService *)")]
#[doc(alias = "__ZN3RBX9GuiObject18tweenPositionDelayENS_5UDim2ES1_fNS0_20TweenEasingDirectionENS0_16TweenEasingStyleEfbN5boost8functionIFvNS0_11TweenStatusEEEEPNS_12TweenServiceE")]
pub fn stub_5288b0() -> ! {
    todo!("0x5288b0 __ZN3RBX9GuiObject18tweenPositionDelayENS_5UDim2ES1_fNS0_20TweenEasingDirectionENS0_16TweenEasingStyleEfbN5boost8functionIFvNS0_11TweenStatusEEEEPNS_12TweenServiceE")
}

// 0x528c4c — __ZN3RBXL22InvokeRemoveOnTweenEndEN5boost8weak_ptrINS_9GuiObjectEEENS_3Lua15WeakFunctionRefE
#[doc(alias = "RBX::InvokeRemoveOnTweenEnd(rbx_core::Weak<RBX::GuiObject>,RBX::Lua::WeakFunctionRef)")]
#[doc(alias = "__ZN3RBXL22InvokeRemoveOnTweenEndEN5boost8weak_ptrINS_9GuiObjectEEENS_3Lua15WeakFunctionRefE")]
pub fn stub_528c4c() -> ! {
    todo!("0x528c4c __ZN3RBXL22InvokeRemoveOnTweenEndEN5boost8weak_ptrINS_9GuiObjectEEENS_3Lua15WeakFunctionRefE")
}

// 0x528d14 — __ZN3RBX9GuiObject14tweenSizeDelayENS_5UDim2ES1_fNS0_20TweenEasingDirectionENS0_16TweenEasingStyleEfbN5boost8functionIFvNS0_11TweenStatusEEEE
#[doc(alias = "RBX::GuiObject::tweenSizeDelay(RBX::UDim2,RBX::UDim2,float,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,boost::function<void ()(RBX::GuiObject::TweenStatus)>)")]
#[doc(alias = "__ZN3RBX9GuiObject14tweenSizeDelayENS_5UDim2ES1_fNS0_20TweenEasingDirectionENS0_16TweenEasingStyleEfbN5boost8functionIFvNS0_11TweenStatusEEEE")]
pub fn stub_528d14() -> ! {
    todo!("0x528d14 __ZN3RBX9GuiObject14tweenSizeDelayENS_5UDim2ES1_fNS0_20TweenEasingDirectionENS0_16TweenEasingStyleEfbN5boost8functionIFvNS0_11TweenStatusEEEE")
}

// 0x5290b4 — __ZN3RBX9GuiObject9tweenStepERKd
#[doc(alias = "RBX::GuiObject::tweenStep(double const&)")]
#[doc(alias = "__ZN3RBX9GuiObject9tweenStepERKd")]
pub fn stub_5290b4() -> ! {
    todo!("0x5290b4 __ZN3RBX9GuiObject9tweenStepERKd")
}

// 0x529284 — __ZN3RBX9GuiObject18setServerGuiObjectEv
#[doc(alias = "RBX::GuiObject::setServerGuiObject(void)")]
#[doc(alias = "__ZN3RBX9GuiObject18setServerGuiObjectEv")]
pub fn stub_529284() -> ! {
    todo!("0x529284 __ZN3RBX9GuiObject18setServerGuiObjectEv")
}

// 0x529314 — __ZN3RBX9GuiObject13getWindowRectEPNS_9GuiBase2dE
#[doc(alias = "RBX::GuiObject::getWindowRect(RBX::GuiBase2d *)")]
#[doc(alias = "__ZN3RBX9GuiObject13getWindowRectEPNS_9GuiBase2dE")]
pub fn stub_529314() -> ! {
    todo!("0x529314 __ZN3RBX9GuiObject13getWindowRectEPNS_9GuiBase2dE")
}

// 0x5293fc — __ZN3RBX9GuiObject12handleResizeERKN3G3D6Rect2DEb
#[doc(alias = "RBX::GuiObject::handleResize(G3D::Rect2D const&,bool)")]
#[doc(alias = "__ZN3RBX9GuiObject12handleResizeERKN3G3D6Rect2DEb")]
pub fn stub_5293fc() -> ! {
    todo!("0x5293fc __ZN3RBX9GuiObject12handleResizeERKN3G3D6Rect2DEb")
}

// 0x529438 — __ZN3RBX9GuiObject10handleDragEN3G3D7Vector2E
#[doc(alias = "RBX::GuiObject::handleDrag(G3D::Vector2)")]
#[doc(alias = "__ZN3RBX9GuiObject10handleDragEN3G3D7Vector2E")]
pub fn stub_529438() -> ! {
    todo!("0x529438 __ZN3RBX9GuiObject10handleDragEN3G3D7Vector2E")
}

// 0x5294c8 — __ZN3RBX9GuiObject34recalculateAbsoluteSizeAndPositionERKN3G3D6Rect2DE
#[doc(alias = "RBX::GuiObject::recalculateAbsoluteSizeAndPosition(G3D::Rect2D const&)")]
#[doc(alias = "__ZN3RBX9GuiObject34recalculateAbsoluteSizeAndPositionERKN3G3D6Rect2DE")]
pub fn stub_5294c8() -> ! {
    todo!("0x5294c8 __ZN3RBX9GuiObject34recalculateAbsoluteSizeAndPositionERKN3G3D6Rect2DE")
}

// 0x529650 — __ZN3RBX9GuiObject11forceResizeEv
#[doc(alias = "RBX::GuiObject::forceResize(void)")]
#[doc(alias = "__ZN3RBX9GuiObject11forceResizeEv")]
pub fn stub_529650() -> ! {
    todo!("0x529650 __ZN3RBX9GuiObject11forceResizeEv")
}

// 0x5296a4 — __ZN3RBX9GuiObject14checkForResizeEv
#[doc(alias = "RBX::GuiObject::checkForResize(void)")]
#[doc(alias = "__ZN3RBX9GuiObject14checkForResizeEv")]
pub fn stub_5296a4() -> ! {
    todo!("0x5296a4 __ZN3RBX9GuiObject14checkForResizeEv")
}

// 0x5296f8 — __ZN3RBX9GuiObject21firstAncestorClippingEv
#[doc(alias = "RBX::GuiObject::firstAncestorClipping(void)")]
#[doc(alias = "__ZN3RBX9GuiObject21firstAncestorClippingEv")]
pub fn stub_5296f8() -> ! {
    todo!("0x5296f8 __ZN3RBX9GuiObject21firstAncestorClippingEv")
}

// 0x52973c — __ZN3RBX9GuiObject14getClippedRectEv
#[doc(alias = "RBX::GuiObject::getClippedRect(void)")]
#[doc(alias = "__ZN3RBX9GuiObject14getClippedRectEv")]
pub fn stub_52973c() -> ! {
    todo!("0x52973c __ZN3RBX9GuiObject14getClippedRectEv")
}

// 0x529798 — __ZN3RBX9GuiObject17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "RBX::GuiObject::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
#[doc(alias = "__ZN3RBX9GuiObject17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE")]
pub fn stub_529798() -> ! {
    todo!("0x529798 __ZN3RBX9GuiObject17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE")
}

// 0x5297e8 — __ZN3RBX9GuiObject17onAncestorChangedERKNS_15AncestorChangedE
#[doc(alias = "RBX::GuiObject::onAncestorChanged(RBX::AncestorChanged const&)")]
#[doc(alias = "__ZN3RBX9GuiObject17onAncestorChangedERKNS_15AncestorChangedE")]
pub fn stub_5297e8() -> ! {
    todo!("0x5297e8 __ZN3RBX9GuiObject17onAncestorChangedERKNS_15AncestorChangedE")
}

// 0x52981c — __ZNK3RBX9GuiObject12askSetParentEPKNS_8InstanceE
#[doc(alias = "RBX::GuiObject::askSetParent(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX9GuiObject12askSetParentEPKNS_8InstanceE")]
pub fn stub_52981c() -> ! {
    todo!("0x52981c __ZNK3RBX9GuiObject12askSetParentEPKNS_8InstanceE")
}

// 0x52986c — __ZNK3RBX9GuiObject25getRenderBackgroundColor4Ev
#[doc(alias = "RBX::GuiObject::getRenderBackgroundColor4(void)const")]
#[doc(alias = "__ZNK3RBX9GuiObject25getRenderBackgroundColor4Ev")]
pub fn stub_52986c() -> ! {
    todo!("0x52986c __ZNK3RBX9GuiObject25getRenderBackgroundColor4Ev")
}

// 0x5298b0 — __ZN3RBX9GuiObject14legacyRender2dEPNS_5AdornERKN3G3D6Rect2DE
#[doc(alias = "RBX::GuiObject::legacyRender2d(RBX::Adorn *,G3D::Rect2D const&)")]
#[doc(alias = "__ZN3RBX9GuiObject14legacyRender2dEPNS_5AdornERKN3G3D6Rect2DE")]
pub fn stub_5298b0() -> ! {
    todo!("0x5298b0 __ZN3RBX9GuiObject14legacyRender2dEPNS_5AdornERKN3G3D6Rect2DE")
}

// 0x5298dc — __ZN3RBX9GuiObject8render2dEPNS_5AdornE
#[doc(alias = "RBX::GuiObject::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX9GuiObject8render2dEPNS_5AdornE")]
pub fn stub_5298dc() -> ! {
    todo!("0x5298dc __ZN3RBX9GuiObject8render2dEPNS_5AdornE")
}

// 0x529948 — __ZN3RBX9GuiObject12render2dImplEPNS_5AdornERKN3G3D6Color4E
#[doc(alias = "RBX::GuiObject::render2dImpl(RBX::Adorn *,G3D::Color4 const&)")]
#[doc(alias = "__ZN3RBX9GuiObject12render2dImplEPNS_5AdornERKN3G3D6Color4E")]
pub fn stub_529948() -> ! {
    todo!("0x529948 __ZN3RBX9GuiObject12render2dImplEPNS_5AdornERKN3G3D6Color4E")
}

// 0x529960 — __ZThn96_N3RBX9GuiObject8render2dEPNS_5AdornE
#[doc(alias = "non-virtual thunk toRBX::GuiObject::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZThn96_N3RBX9GuiObject8render2dEPNS_5AdornE")]
pub fn stub_529960() {
    // IDA 0x529960: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x529968 — __ZN3RBX9GuiObject12render2dImplEPNS_5AdornERKN3G3D6Color4ERNS3_6Rect2DE
#[doc(alias = "RBX::GuiObject::render2dImpl(RBX::Adorn *,G3D::Color4 const&,G3D::Rect2D &)")]
#[doc(alias = "__ZN3RBX9GuiObject12render2dImplEPNS_5AdornERKN3G3D6Color4ERNS3_6Rect2DE")]
pub fn stub_529968() -> ! {
    todo!("0x529968 __ZN3RBX9GuiObject12render2dImplEPNS_5AdornERKN3G3D6Color4ERNS3_6Rect2DE")
}

// 0x529a50 — __ZN3RBX9GuiObject18renderSelectionBoxEPNS_5AdornE
#[doc(alias = "RBX::GuiObject::renderSelectionBox(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX9GuiObject18renderSelectionBoxEPNS_5AdornE")]
pub fn stub_529a50() -> ! {
    todo!("0x529a50 __ZN3RBX9GuiObject18renderSelectionBoxEPNS_5AdornE")
}

// 0x529b14 — __ZN3RBX9GuiObject12Scale9Rect2DERKN3G3D6Rect2DEff
#[doc(alias = "RBX::GuiObject::Scale9Rect2D(G3D::Rect2D const&,float,float)")]
#[doc(alias = "__ZN3RBX9GuiObject12Scale9Rect2DERKN3G3D6Rect2DEff")]
pub fn stub_529b14() -> ! {
    todo!("0x529b14 __ZN3RBX9GuiObject12Scale9Rect2DERKN3G3D6Rect2DEff")
}

// 0x529bd4 — __ZN3RBX9GuiObject18render2dScale9ImplEPNS_5AdornERKNS_9TextureIdERKN3G3D12Vector2int16ERKNS6_7Vector2ERNS_12GuiDrawImageERNS6_6Rect2DEPS0_
#[doc(alias = "RBX::GuiObject::render2dScale9Impl(RBX::Adorn *,RBX::TextureId const&,G3D::Vector2int16 const&,G3D::Vector2 const&,RBX::GuiDrawImage &,G3D::Rect2D &,RBX::GuiObject*)")]
#[doc(alias = "__ZN3RBX9GuiObject18render2dScale9ImplEPNS_5AdornERKNS_9TextureIdERKN3G3D12Vector2int16ERKNS6_7Vector2ERNS_12GuiDrawImageERNS6_6Rect2DEPS0_")]
pub fn stub_529bd4() -> ! {
    todo!("0x529bd4 __ZN3RBX9GuiObject18render2dScale9ImplEPNS_5AdornERKNS_9TextureIdERKN3G3D12Vector2int16ERKNS6_7Vector2ERNS_12GuiDrawImageERNS6_6Rect2DEPS0_")
}

// 0x529eac — __ZN3RBX9GuiObject16render2dTextImplEPNS_5AdornERKN3G3D6Color4ERKSsNS_11TextService4FontENS9_8FontSizeES6_S6_bbNS9_10XAlignmentENS9_10YAlignmentE
#[doc(alias = "RBX::GuiObject::render2dTextImpl(RBX::Adorn *,G3D::Color4 const&,std::string const&,RBX::TextService::Font,RBX::TextService::FontSize,G3D::Color4 const&,G3D::Color4 const&,bool,bool,RBX::TextService::XAlignment,RBX::TextService::YAlignment)")]
#[doc(alias = "__ZN3RBX9GuiObject16render2dTextImplEPNS_5AdornERKN3G3D6Color4ERKSsNS_11TextService4FontENS9_8FontSizeES6_S6_bbNS9_10XAlignmentENS9_10YAlignmentE")]
pub fn stub_529eac() -> ! {
    todo!("0x529eac __ZN3RBX9GuiObject16render2dTextImplEPNS_5AdornERKN3G3D6Color4ERKSsNS_11TextService4FontENS9_8FontSizeES6_S6_bbNS9_10XAlignmentENS9_10YAlignmentE")
}
