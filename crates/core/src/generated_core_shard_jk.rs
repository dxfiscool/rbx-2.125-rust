//! core shard jk — 150 stubs EA-sorted, 0x35ffc..0x5f3e4 (EA-sorted asc next 150 not yet in crates/core/src, rbx_core::SharedPtr not boost).
//! Source: ida/export.json (85545 funcs) EA-sorted asc not yet in crates/core/src via grep -r stub_0x crates/core/src --include=*.rs — next 150 uncovered (73831 remaining before -> 73681 after, 0x35ffc..0x5f3e4).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "___copy_helper_block_19")]
// 0x35ffc — ___copy_helper_block_19
pub fn stub_0x35ffc() -> ! {
    todo!("0x35ffc ___copy_helper_block_19")
}

#[doc(alias = "___destroy_helper_block_20")]
// 0x36020 — ___destroy_helper_block_20
pub fn stub_0x36020() -> ! {
    todo!("0x36020 ___destroy_helper_block_20")
}

#[doc(alias = "getUserAgentString(void)")]
// 0x3603c — __Z18getUserAgentStringv
// type: id __fastcall()
pub fn stub_0x3603c() -> ! {
    todo!("0x3603c getUserAgentString(void)")
}

#[doc(alias = "global constructor keyed to_a_9")]
// 0x36e80 — __GLOBAL__I_a_9
pub fn stub_0x36e80() -> ! {
    todo!("0x36e80 global constructor keyed to_a_9")
}

#[doc(alias = "macBundlePath(void)")]
// 0x375b4 — __Z13macBundlePathv
// type: _DWORD __fastcall()
pub fn stub_0x375b4() -> ! {
    todo!("0x375b4 macBundlePath(void)")
}

#[doc(alias = "RobloxView::onPlaceIDChanged(RBX::Reflection::PropertyDescriptor const*)")]
// 0x380a0 — __ZN10RobloxView16onPlaceIDChangedEPKN3RBX10Reflection18PropertyDescriptorE
// type: _DWORD __fastcall(RobloxView *__hidden this, const PropertyDescriptor *)
pub fn stub_0x380a0() -> ! {
    todo!("0x380a0 RobloxView::onPlaceIDChanged(RBX::Reflection::PropertyDescriptor const*)")
}

#[doc(alias = "RobloxView::bindWorkspace(rbx_core::SharedPtr<RBX::ViewBase>,rbx_core::SharedPtr<RBX::DataModel>,rbx_core::SharedPtr<RBX::OverlayDataModel>)")]
// 0x380a4 — __ZN10RobloxView13bindWorkspaceEN5boost10shared_ptrIN3RBX8ViewBaseEEENS1_INS2_9DataModelEEENS1_INS2_16OverlayDataModelEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, int, int, int)
pub fn stub_0x380a4() -> ! {
    todo!("0x380a4 RobloxView::bindWorkspace(rbx_core::SharedPtr<RBX::ViewBase>,rbx_core::SharedPtr<RBX::DataModel>,rbx_core::SharedPtr<RBX::OverlayDataModel>)")
}

#[doc(alias = "RobloxView::restartDataModel(void)")]
// 0x386d0 — __ZN10RobloxView16restartDataModelEv
// type: _DWORD __fastcall(RobloxView *__hidden this)
pub fn stub_0x386d0() -> ! {
    todo!("0x386d0 RobloxView::restartDataModel(void)")
}

#[doc(alias = "____ZN10RobloxView18doRestartDataModelEv_block_invoke")]
// 0x38770 — ____ZN10RobloxView18doRestartDataModelEv_block_invoke
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x38770() -> ! {
    todo!("0x38770 ____ZN10RobloxView18doRestartDataModelEv_block_invoke")
}

#[doc(alias = "RobloxView::setupNewDataModel(void)")]
// 0x38cd0 — __ZN10RobloxView17setupNewDataModelEv
// type: _DWORD __fastcall(RobloxView *__hidden this)
pub fn stub_0x38cd0() -> ! {
    todo!("0x38cd0 RobloxView::setupNewDataModel(void)")
}

#[doc(alias = "initLogManager(void)")]
// 0x39920 — __ZL14initLogManagerv
// type: _DWORD __fastcall()
pub fn stub_0x39920() -> ! {
    todo!("0x39920 initLogManager(void)")
}

#[doc(alias = "QuitEventListener::~QuitEventListener()")]
// 0x3a1b8 — __ZN17QuitEventListenerD1Ev
// type: void __fastcall(QuitEventListener *__hidden this)
pub fn stub_0x3a1b8() -> ! {
    todo!("0x3a1b8 QuitEventListener::~QuitEventListener()")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>> const&)")]
// 0x3a278 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE7connectIN5boost3_bi6bind_tIvNSA_4_mfi3mf1Iv10RobloxViewS6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int(void)
pub fn stub_0x3a278() -> ! {
    todo!("0x3a278 rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>> const&)")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel>& rbx_core::SharedPtr<RBX::DataModel>::operator=<RBX::OverlayDataModel>(rbx_core::SharedPtr<RBX::OverlayDataModel> &&)")]
// 0x3a2ec — __ZN5boost10shared_ptrIN3RBX9DataModelEEaSINS1_16OverlayDataModelEEERS3_ONS0_IT_EE
pub fn stub_0x3a2ec() -> ! {
    todo!("0x3a2ec rbx_core::SharedPtr<RBX::DataModel>& rbx_core::SharedPtr<RBX::DataModel>::operator=<RBX::OverlayDataModel>(rbx_core::SharedPtr<RBX::OverlayDataModel> &&)")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7CreatorD1Ev")]
// 0x3a790 — __ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7CreatorD1Ev
pub fn stub_0x3a790() -> ! {
    todo!("0x3a790 __ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7CreatorD1Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Camera> RBX::Creatable<RBX::Instance>::create<RBX::Camera>(void)")]
// 0x3a798 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_6CameraEEEN5boost10shared_ptrIT_EEv
pub fn stub_0x3a798() -> ! {
    todo!("0x3a798 rbx_core::SharedPtr<RBX::Camera> RBX::Creatable<RBX::Instance>::create<RBX::Camera>(void)")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Camera,RBX::Camera>(rbx_core::SharedPtr<RBX::Camera> const*,RBX::Camera *)const")]
// 0x3a930 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_6CameraES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_0x3a930() -> ! {
    todo!("0x3a930 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Camera,RBX::Camera>(rbx_core::SharedPtr<RBX::Camera> const*,RBX::Camera *)const")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x3aa10 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6CameraENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_0x3aa10() -> ! {
    todo!("0x3aa10 boost::detail::sp_counted_impl_pd<RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x3aa18 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6CameraENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_0x3aa18() -> ! {
    todo!("0x3aa18 boost::detail::sp_counted_impl_pd<RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7CreatorC2Ev")]
// 0x3aaa0 — __ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7CreatorC2Ev
// type: int __fastcall(_DWORD)
pub fn stub_0x3aaa0() -> ! {
    todo!("0x3aaa0 __ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7CreatorC2Ev")
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_11sRunServiceEEEERKS0_v")]
// 0x3add8 — __ZN3RBX4Name7declareILZNS_11sRunServiceEEEERKS0_v
// type: int(void)
pub fn stub_0x3add8() -> ! {
    todo!("0x3add8 __ZN3RBX4Name7declareILZNS_11sRunServiceEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sRunServiceEEEERKS0_v")]
// 0x3ae20 — __ZN3RBX4Name9doDeclareILZNS_11sRunServiceEEEERKS0_v
pub fn stub_0x3ae20() -> ! {
    todo!("0x3ae20 __ZN3RBX4Name9doDeclareILZNS_11sRunServiceEEEERKS0_v")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::RunService>::shared_ptr<RBX::RunService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x3afe0 — __ZN5boost10shared_ptrIN3RBX10RunServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x3afe0() -> ! {
    todo!("0x3afe0 rbx_core::SharedPtr<RBX::RunService>::shared_ptr<RBX::RunService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x3b008 — __ZN5boost6detail12shared_countC2IPN3RBX10RunServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x3b008() -> ! {
    todo!("0x3b008 boost::detail::shared_count::shared_count<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x3b108 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_0x3b108() -> ! {
    todo!("0x3b108 boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x3b110 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_0x3b110() -> ! {
    todo!("0x3b110 boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x3b130 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_0x3b130() -> ! {
    todo!("0x3b130 boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x3b148 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_0x3b148() -> ! {
    todo!("0x3b148 boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ControllerService> RBX::Creatable<RBX::Instance>::create<RBX::ControllerService>(void)")]
// 0x3b674 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_17ControllerServiceEEEN5boost10shared_ptrIT_EEv
pub fn stub_0x3b674() -> ! {
    todo!("0x3b674 rbx_core::SharedPtr<RBX::ControllerService> RBX::Creatable<RBX::Instance>::create<RBX::ControllerService>(void)")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ControllerService>(rbx_core::SharedPtr<RBX::ControllerService> const&)")]
// 0x3b724 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_17ControllerServiceEEERS3_RKNS0_IT_EE
pub fn stub_0x3b724() -> ! {
    todo!("0x3b724 rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ControllerService>(rbx_core::SharedPtr<RBX::ControllerService> const&)")
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_18sControllerServiceEEEERKS0_v")]
// 0x3b7e0 — __ZN3RBX4Name7declareILZNS_18sControllerServiceEEEERKS0_v
// type: int(void)
pub fn stub_0x3b7e0() -> ! {
    todo!("0x3b7e0 __ZN3RBX4Name7declareILZNS_18sControllerServiceEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_18sControllerServiceEEEERKS0_v")]
// 0x3b828 — __ZN3RBX4Name9doDeclareILZNS_18sControllerServiceEEEERKS0_v
pub fn stub_0x3b828() -> ! {
    todo!("0x3b828 __ZN3RBX4Name9doDeclareILZNS_18sControllerServiceEEEERKS0_v")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ControllerService>::shared_ptr<RBX::ControllerService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x3b9e8 — __ZN5boost10shared_ptrIN3RBX17ControllerServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int(void)
pub fn stub_0x3b9e8() -> ! {
    todo!("0x3b9e8 rbx_core::SharedPtr<RBX::ControllerService>::shared_ptr<RBX::ControllerService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x3ba10 — __ZN5boost6detail12shared_countC2IPN3RBX17ControllerServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x3ba10() -> ! {
    todo!("0x3ba10 boost::detail::shared_count::shared_count<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x3bb10 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_0x3bb10() -> ! {
    todo!("0x3bb10 boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x3bb18 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_0x3bb18() -> ! {
    todo!("0x3bb18 boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x3bb38 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_0x3bb38() -> ! {
    todo!("0x3bb38 boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x3bb50 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_0x3bb50() -> ! {
    todo!("0x3bb50 boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_18sControllerServiceEEE15isNullClassNameEv")]
// 0x3bb58 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_18sControllerServiceEEE15isNullClassNameEv
// type: int(void)
pub fn stub_0x3bb58() -> ! {
    todo!("0x3bb58 __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_18sControllerServiceEEE15isNullClassNameEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>::operator=(rbx_core::SharedPtr<RBX::Instance> const&)")]
// 0x3bbf8 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSERKS3_
pub fn stub_0x3bbf8() -> ! {
    todo!("0x3bbf8 rbx_core::SharedPtr<RBX::Instance>::operator=(rbx_core::SharedPtr<RBX::Instance> const&)")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::insert(rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot *)")]
// 0x3d2f4 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE6insertEPNS8_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0x3d2f4() -> ! {
    todo!("0x3d2f4 rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::insert(rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot *)")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot> const&)")]
// 0x3d508 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEEaSERKSC_
pub fn stub_0x3d508() -> ! {
    todo!("0x3d508 rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot> const&)")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::safe_static_do_get_mutex(void)")]
// 0x3d5b0 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE24safe_static_do_get_mutexEv
pub fn stub_0x3d5b0() -> ! {
    todo!("0x3d5b0 rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::safe_static_do_get_mutex(void)")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>>::~callable_slot()")]
// 0x3d6a8 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1Iv10RobloxViewS6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEED1Ev
pub fn stub_0x3d6a8() -> ! {
    todo!("0x3d6a8 rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>>::~callable_slot()")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>>::~callable_slot()")]
// 0x3d754 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1Iv10RobloxViewS6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEED0Ev
pub fn stub_0x3d754() -> ! {
    todo!("0x3d754 rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>>::~callable_slot()")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
// 0x3d808 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1Iv10RobloxViewS7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
pub fn stub_0x3d808() -> ! {
    todo!("0x3d808 rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
// 0x3d81c — __ZThn4_N3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1Iv10RobloxViewS7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
pub fn stub_0x3d81c() -> ! {
    todo!("0x3d81c non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>::operator()<RBX::Reflection::PropertyDescriptor const*>(RBX::Reflection::PropertyDescriptor const* &)")]
// 0x3d830 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1Iv10RobloxViewPKN3RBX10Reflection18PropertyDescriptorEEENS0_5list2INS0_5valueIPS4_EENS_3argILi1EEEEEEclIS9_EEvRT_
// type: int(void)
pub fn stub_0x3d830() -> ! {
    todo!("0x3d830 void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>::operator()<RBX::Reflection::PropertyDescriptor const*>(RBX::Reflection::PropertyDescriptor const* &)")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::remove(rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot *)")]
// 0x3d848 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE6removeEPNS8_4slotE
// type: int __fastcall(int, char *)
pub fn stub_0x3d848() -> ! {
    todo!("0x3d848 rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::remove(rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot *)")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot::safe_static_init_mutex(void)")]
// 0x3d938 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slot22safe_static_init_mutexEv
pub fn stub_0x3d938() -> ! {
    todo!("0x3d938 rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot::safe_static_init_mutex(void)")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot::~slot()")]
// 0x3d940 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotD0Ev
pub fn stub_0x3d940() -> ! {
    todo!("0x3d940 rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot::~slot()")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
// 0x3d9f0 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1Iv10RobloxViewS7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_ED1Ev
pub fn stub_0x3d9f0() -> ! {
    todo!("0x3d9f0 rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
// 0x3da9c — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1Iv10RobloxViewS7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_ED0Ev
pub fn stub_0x3da9c() -> ! {
    todo!("0x3da9c rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<CRenderSettingsItem,CRenderSettingsItem>(rbx_core::SharedPtr<CRenderSettingsItem> const*,CRenderSettingsItem *)const")]
// 0x3e0b0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerI19CRenderSettingsItemS6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_0x3e0b0() -> ! {
    todo!("0x3e0b0 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<CRenderSettingsItem,CRenderSettingsItem>(rbx_core::SharedPtr<CRenderSettingsItem> const*,CRenderSettingsItem *)const")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x3e190 — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED0Ev
pub fn stub_0x3e190() -> ! {
    todo!("0x3e190 boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

#[doc(alias = "boost::singleton_pool<RBX::OnDemandInstance,20u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0x3e1e8 — __ZN5boost14singleton_poolIN3RBX16OnDemandInstanceELj20ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// type: int(void)
pub fn stub_0x3e1e8() -> ! {
    todo!("0x3e1e8 boost::singleton_pool<RBX::OnDemandInstance,20u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")
}

#[doc(alias = "boost::scoped_ptr<Ogre::LogManager>::~scoped_ptr()")]
// 0x3ec30 — __ZN5boost10scoped_ptrIN4Ogre10LogManagerEED1Ev
pub fn stub_0x3ec30() -> ! {
    todo!("0x3ec30 boost::scoped_ptr<Ogre::LogManager>::~scoped_ptr()")
}

#[doc(alias = "boost::scoped_ptr<Ogre::LogManager>::~scoped_ptr()")]
// 0x3ec34 — __ZN5boost10scoped_ptrIN4Ogre10LogManagerEED2Ev
pub fn stub_0x3ec34() -> ! {
    todo!("0x3ec34 boost::scoped_ptr<Ogre::LogManager>::~scoped_ptr()")
}

#[doc(alias = "QuitEventListener::~QuitEventListener()")]
// 0x3eccc — __ZN17QuitEventListenerD0Ev
// type: void __fastcall(QuitEventListener *__hidden this)
pub fn stub_0x3eccc() -> ! {
    todo!("0x3eccc QuitEventListener::~QuitEventListener()")
}

#[doc(alias = "Ogre::WindowEventListener::windowMoved(Ogre::RenderWindow *)")]
// 0x3ecd0 — __ZN4Ogre19WindowEventListener11windowMovedEPNS_12RenderWindowE
// type: _DWORD __fastcall(Ogre::WindowEventListener *__hidden this, RenderWindow *)
pub fn stub_0x3ecd0() -> ! {
    todo!("0x3ecd0 Ogre::WindowEventListener::windowMoved(Ogre::RenderWindow *)")
}

#[doc(alias = "Ogre::WindowEventListener::windowResized(Ogre::RenderWindow *)")]
// 0x3ecd4 — __ZN4Ogre19WindowEventListener13windowResizedEPNS_12RenderWindowE
// type: _DWORD __fastcall(Ogre::WindowEventListener *__hidden this, RenderWindow *)
pub fn stub_0x3ecd4() -> ! {
    todo!("0x3ecd4 Ogre::WindowEventListener::windowResized(Ogre::RenderWindow *)")
}

#[doc(alias = "Ogre::WindowEventListener::windowClosing(Ogre::RenderWindow *)")]
// 0x3ecd8 — __ZN4Ogre19WindowEventListener13windowClosingEPNS_12RenderWindowE
// type: _DWORD __fastcall(Ogre::WindowEventListener *__hidden this, RenderWindow *)
pub fn stub_0x3ecd8() -> ! {
    todo!("0x3ecd8 Ogre::WindowEventListener::windowClosing(Ogre::RenderWindow *)")
}

#[doc(alias = "QuitEventListener::windowClosed(Ogre::RenderWindow *)")]
// 0x3ecdc — __ZN17QuitEventListener12windowClosedEPN4Ogre12RenderWindowE
// type: _DWORD __fastcall(QuitEventListener *__hidden this, RenderWindow *)
pub fn stub_0x3ecdc() -> ! {
    todo!("0x3ecdc QuitEventListener::windowClosed(Ogre::RenderWindow *)")
}

#[doc(alias = "Ogre::WindowEventListener::windowFocusChange(Ogre::RenderWindow *)")]
// 0x3ecec — __ZN4Ogre19WindowEventListener17windowFocusChangeEPNS_12RenderWindowE
// type: _DWORD __fastcall(Ogre::WindowEventListener *__hidden this, RenderWindow *)
pub fn stub_0x3ecec() -> ! {
    todo!("0x3ecec Ogre::WindowEventListener::windowFocusChange(Ogre::RenderWindow *)")
}

#[doc(alias = "RobloxView::RenderJob::RenderJob(RBX::ViewBase *,RBX::FunctionMarshaller *,rbx_core::SharedPtr<RBX::DataModel>)")]
// 0x3ecf0 — __ZN10RobloxView9RenderJobC2EPN3RBX8ViewBaseEPNS1_18FunctionMarshallerEN5boost10shared_ptrINS1_9DataModelEEE
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, RBX::TaskScheduler::Job *, int, int, int, int)
pub fn stub_0x3ecf0() -> ! {
    todo!("0x3ecf0 RobloxView::RenderJob::RenderJob(RBX::ViewBase *,RBX::FunctionMarshaller *,rbx_core::SharedPtr<RBX::DataModel>)")
}

#[doc(alias = "RobloxView::RenderJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
// 0x3f094 — __ZN10RobloxView9RenderJob16stepDataModelJobERKN3RBX13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
pub fn stub_0x3f094() -> ! {
    todo!("0x3f094 RobloxView::RenderJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")
}

#[doc(alias = "global constructor keyed to_a_10")]
// 0x4070c — __GLOBAL__I_a_10
pub fn stub_0x4070c() -> ! {
    todo!("0x4070c global constructor keyed to_a_10")
}

#[doc(alias = "___copy_helper_block__6")]
// 0x41104 — ___copy_helper_block__6
pub fn stub_0x41104() -> ! {
    todo!("0x41104 ___copy_helper_block__6")
}

#[doc(alias = "___destroy_helper_block__6")]
// 0x41128 — ___destroy_helper_block__6
pub fn stub_0x41128() -> ! {
    todo!("0x41128 ___destroy_helper_block__6")
}

#[doc(alias = "convertToFriendlyString(NSNumber *)")]
// 0x411a0 — __Z23convertToFriendlyStringP8NSNumber
// type: _DWORD __fastcall(id)
pub fn stub_0x411a0() -> ! {
    todo!("0x411a0 convertToFriendlyString(NSNumber *)")
}

#[doc(alias = "global constructor keyed to_a_11")]
// 0x41bfc — __GLOBAL__I_a_11
pub fn stub_0x41bfc() -> ! {
    todo!("0x41bfc global constructor keyed to_a_11")
}

#[doc(alias = "global constructor keyed to_a_12")]
// 0x42580 — __GLOBAL__I_a_12
pub fn stub_0x42580() -> ! {
    todo!("0x42580 global constructor keyed to_a_12")
}

#[doc(alias = "+[RobloxWebUtility sharedInstance]")]
// 0x42718 — +[RobloxWebUtility sharedInstance]
// type: id __cdecl(id, SEL)
pub fn stub_0x42718() -> ! {
    todo!("0x42718 +[RobloxWebUtility sharedInstance]")
}

#[doc(alias = "___34+[RobloxWebUtility sharedInstance]_block_invoke")]
// 0x42774 — ___34+[RobloxWebUtility sharedInstance]_block_invoke
pub fn stub_0x42774() -> ! {
    todo!("0x42774 ___34+[RobloxWebUtility sharedInstance]_block_invoke")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>> const&)")]
// 0x46c18 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE7connectIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP13CharacterMoveEENSL_ISF_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
pub fn stub_0x46c18() -> ! {
    todo!("0x46c18 rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>> const&)")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot()")]
// 0x46c8c — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP13CharacterMoveEENSL_ISF_EENSA_3argILi1EEEEEEEED1Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int)
pub fn stub_0x46c8c() -> ! {
    todo!("0x46c8c rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot()")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot()")]
// 0x46d38 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP13CharacterMoveEENSL_ISF_EENSA_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x46d38() -> ! {
    todo!("0x46d38 rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot()")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
// 0x46de8 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP13CharacterMoveEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// type: int __fastcall(int, int)
pub fn stub_0x46de8() -> ! {
    todo!("0x46de8 rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
// 0x46df8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP13CharacterMoveEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// type: int __fastcall(int, int)
pub fn stub_0x46df8() -> ! {
    todo!("0x46df8 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
// 0x46e08 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP13CharacterMoveEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_ED1Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int)
pub fn stub_0x46e08() -> ! {
    todo!("0x46e08 rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
// 0x46eb4 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP13CharacterMoveEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_ED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x46eb4() -> ! {
    todo!("0x46eb4 rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<CharacterMove *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")
}

#[doc(alias = "-[ControlComponent getUserInputServiceForGameDataModel]")]
// 0x47338 — -[ControlComponent getUserInputServiceForGameDataModel]
// type: UserInputService *__cdecl(ControlComponent *self, SEL)
pub fn stub_0x47338() -> ! {
    todo!("0x47338 -[ControlComponent getUserInputServiceForGameDataModel]")
}

#[doc(alias = "-[ControlView checkUserInputPropertyChanged:onDataModel:]")]
// 0x48774 — -[ControlView checkUserInputPropertyChanged:onDataModel:]
// type: char __cdecl(ControlView *self, SEL, const PropertyDescriptor *, shared_ptr<RBX::DataModel>)
pub fn stub_0x48774() -> ! {
    todo!("0x48774 -[ControlView checkUserInputPropertyChanged:onDataModel:]")
}

#[doc(alias = "-[ControlView userInputPropertyChangedOnDataModel:]")]
// 0x4880c — -[ControlView userInputPropertyChangedOnDataModel:]
// type: void __cdecl(ControlView *self, SEL, const PropertyDescriptor *)
pub fn stub_0x4880c() -> ! {
    todo!("0x4880c -[ControlView userInputPropertyChangedOnDataModel:]")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>*>(boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)> const&,rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>*)")]
// 0x4a04c — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost8functionIS8_EELi1ES8_EC2IPS9_EERKSD_T_
pub fn stub_0x4a04c() -> ! {
    todo!("0x4a04c rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>*>(boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)> const&,rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>*)")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
// 0x4a148 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost8functionIS8_EELi1ES8_E4callES7_
pub fn stub_0x4a148() -> ! {
    todo!("0x4a148 rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
// 0x4a150 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost8functionIS8_EELi1ES8_E4callES7_
pub fn stub_0x4a150() -> ! {
    todo!("0x4a150 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")
}

#[doc(alias = "boost::function1<void,RBX::Reflection::PropertyDescriptor const*>::operator()(RBX::Reflection::PropertyDescriptor const*)const")]
// 0x4a158 — __ZNK5boost9function1IvPKN3RBX10Reflection18PropertyDescriptorEEclES5_
// type: int(void)
pub fn stub_0x4a158() -> ! {
    todo!("0x4a158 boost::function1<void,RBX::Reflection::PropertyDescriptor const*>::operator()(RBX::Reflection::PropertyDescriptor const*)const")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::Reflection::PropertyDescriptor const*),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::Reflection::PropertyDescriptor const*),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")]
// 0x4a21c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKN3RBX10Reflection18PropertyDescriptorEENS3_5list3INS3_5valueIS6_EENSG_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
pub fn stub_0x4a21c() -> ! {
    todo!("0x4a21c boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::Reflection::PropertyDescriptor const*),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::Reflection::PropertyDescriptor const*),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::Reflection::PropertyDescriptor const*),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::Reflection::PropertyDescriptor const>::invoke(boost::detail::function::function_buffer &,RBX::Reflection::PropertyDescriptor const)")]
// 0x4a27c — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKN3RBX10Reflection18PropertyDescriptorEENS3_5list3INS3_5valueIS6_EENSG_IS7_EENS_3argILi1EEEEEEEvSC_E6invokeERNS1_15function_bufferESC_
// type: int __fastcall(int, int)
pub fn stub_0x4a27c() -> ! {
    todo!("0x4a27c boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::Reflection::PropertyDescriptor const*),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::Reflection::PropertyDescriptor const>::invoke(boost::detail::function::function_buffer &,RBX::Reflection::PropertyDescriptor const)")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::insert(rbx::signals::signal<void ()(RBX::DataModel *)>::slot *)")]
// 0x4b164 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE6insertEPNS6_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0x4b164() -> ! {
    todo!("0x4b164 rbx::signals::signal<void ()(RBX::DataModel *)>::insert(rbx::signals::signal<void ()(RBX::DataModel *)>::slot *)")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::safe_static_init_mutex(void)")]
// 0x4b4bc — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE22safe_static_init_mutexEv
pub fn stub_0x4b4bc() -> ! {
    todo!("0x4b4bc rbx::signals::signal<void ()(RBX::DataModel *)>::safe_static_init_mutex(void)")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::safe_static_do_get_mutex(void)")]
// 0x4b4c0 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE24safe_static_do_get_mutexEv
pub fn stub_0x4b4c0() -> ! {
    todo!("0x4b4c0 rbx::signals::signal<void ()(RBX::DataModel *)>::safe_static_do_get_mutex(void)")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::disconnect(void)")]
// 0x4b860 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot10disconnectEv
// type: void __fastcall(int, int, int, int)
pub fn stub_0x4b860() -> ! {
    todo!("0x4b860 rbx::signals::signal<void ()(RBX::DataModel *)>::slot::disconnect(void)")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::connected(void)const")]
// 0x4b970 — __ZNK3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot9connectedEv
// type: bool __fastcall(int)
pub fn stub_0x4b970() -> ! {
    todo!("0x4b970 rbx::signals::signal<void ()(RBX::DataModel *)>::slot::connected(void)const")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::remove(rbx::signals::signal<void ()(RBX::DataModel *)>::slot *)")]
// 0x4ba50 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE6removeEPNS6_4slotE
// type: int __fastcall(char **, char *, int, const void *)
pub fn stub_0x4ba50() -> ! {
    todo!("0x4ba50 rbx::signals::signal<void ()(RBX::DataModel *)>::remove(rbx::signals::signal<void ()(RBX::DataModel *)>::slot *)")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::safe_static_init_mutex(void)")]
// 0x4bb40 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot22safe_static_init_mutexEv
// type: int()
pub fn stub_0x4bb40() -> ! {
    todo!("0x4bb40 rbx::signals::signal<void ()(RBX::DataModel *)>::slot::safe_static_init_mutex(void)")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::safe_static_do_get_mutex(void)")]
// 0x4bb44 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot24safe_static_do_get_mutexEv
// type: void *()
pub fn stub_0x4bb44() -> ! {
    todo!("0x4bb44 rbx::signals::signal<void ()(RBX::DataModel *)>::slot::safe_static_do_get_mutex(void)")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::~slot()")]
// 0x4bde0 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotD1Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int)
pub fn stub_0x4bde0() -> ! {
    todo!("0x4bde0 rbx::signals::signal<void ()(RBX::DataModel *)>::slot::~slot()")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::~slot()")]
// 0x4be8c — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotD0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x4be8c() -> ! {
    todo!("0x4be8c rbx::signals::signal<void ()(RBX::DataModel *)>::slot::~slot()")
}

#[doc(alias = "boost::function1<void,RBX::Reflection::PropertyDescriptor const*>::clear(void)")]
// 0x4bfdc — __ZN5boost9function1IvPKN3RBX10Reflection18PropertyDescriptorEE5clearEv
// type: int __fastcall(int *)
pub fn stub_0x4bfdc() -> ! {
    todo!("0x4bfdc boost::function1<void,RBX::Reflection::PropertyDescriptor const*>::clear(void)")
}

#[doc(alias = "-[GameInputViewController init:withBundle:withGame:overlayDataModel:]")]
// 0x4c248 — -[GameInputViewController init:withBundle:withGame:overlayDataModel:]
// type: id __cdecl(GameInputViewController *self, SEL, id, id, shared_ptr<RBX::Game>, shared_ptr<RBX::OverlayDataModel>)
pub fn stub_0x4c248() -> ! {
    todo!("0x4c248 -[GameInputViewController init:withBundle:withGame:overlayDataModel:]")
}

#[doc(alias = "+[GameKeyboard sharedInstance]")]
// 0x4c6ac — +[GameKeyboard sharedInstance]
// type: id __cdecl(id, SEL)
pub fn stub_0x4c6ac() -> ! {
    todo!("0x4c6ac +[GameKeyboard sharedInstance]")
}

#[doc(alias = "___30+[GameKeyboard sharedInstance]_block_invoke")]
// 0x4c6dc — ___30+[GameKeyboard sharedInstance]_block_invoke
// type: void __cdecl(id)
pub fn stub_0x4c6dc() -> ! {
    todo!("0x4c6dc ___30+[GameKeyboard sharedInstance]_block_invoke")
}

#[doc(alias = "-[GameViewController signalGuiServiceUrlWindowClosedOnDataModel:]")]
// 0x4dbe8 — -[GameViewController signalGuiServiceUrlWindowClosedOnDataModel:]
// type: void __cdecl(GameViewController *self, SEL, DataModel *)
pub fn stub_0x4dbe8() -> ! {
    todo!("0x4dbe8 -[GameViewController signalGuiServiceUrlWindowClosedOnDataModel:]")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>> const&)")]
// 0x4f470 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE7connectIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP10JumpButtonEENSL_ISF_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
pub fn stub_0x4f470() -> ! {
    todo!("0x4f470 rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>> const&)")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot()")]
// 0x4f4e4 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP10JumpButtonEENSL_ISF_EENSA_3argILi1EEEEEEEED1Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int)
pub fn stub_0x4f4e4() -> ! {
    todo!("0x4f4e4 rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot()")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot()")]
// 0x4f590 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSB_5list3INSB_5valueIP10JumpButtonEENSL_ISF_EENSA_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x4f590() -> ! {
    todo!("0x4f590 rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::~callable_slot()")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
// 0x4f640 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP10JumpButtonEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// type: int __fastcall(int, int)
pub fn stub_0x4f640() -> ! {
    todo!("0x4f640 rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
// 0x4f650 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP10JumpButtonEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// type: int __fastcall(int, int)
pub fn stub_0x4f650() -> ! {
    todo!("0x4f650 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
// 0x4f660 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP10JumpButtonEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_ED1Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int)
pub fn stub_0x4f660() -> ! {
    todo!("0x4f660 rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
// 0x4f70c — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKvENSC_5list3INSC_5valueIP10JumpButtonEENSM_ISG_EENSB_3argILi1EEEEEEELi1ES8_ED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x4f70c() -> ! {
    todo!("0x4f70c rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,void const*),boost::_bi::list3<boost::_bi::value<JumpButton *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")
}

#[doc(alias = "+[MainViewController sharedInstance]")]
// 0x51dc4 — +[MainViewController sharedInstance]
// type: id __cdecl(id, SEL)
pub fn stub_0x51dc4() -> ! {
    todo!("0x51dc4 +[MainViewController sharedInstance]")
}

#[doc(alias = "___36+[MainViewController sharedInstance]_block_invoke")]
// 0x51e20 — ___36+[MainViewController sharedInstance]_block_invoke
// type: id __fastcall(int)
pub fn stub_0x51e20() -> ! {
    todo!("0x51e20 ___36+[MainViewController sharedInstance]_block_invoke")
}

#[doc(alias = "-[MainViewController getOgreWindow]")]
// 0x51f40 — -[MainViewController getOgreWindow]
// type: id __cdecl(MainViewController *self, SEL)
pub fn stub_0x51f40() -> ! {
    todo!("0x51f40 -[MainViewController getOgreWindow]")
}

#[doc(alias = "-[MainViewController setOgreWindow:]")]
// 0x51f50 — -[MainViewController setOgreWindow:]
// type: void __cdecl(MainViewController *self, SEL, id)
pub fn stub_0x51f50() -> ! {
    todo!("0x51f50 -[MainViewController setOgreWindow:]")
}

#[doc(alias = "-[MainViewController getOgreView]")]
// 0x51f60 — -[MainViewController getOgreView]
// type: id __cdecl(MainViewController *self, SEL)
pub fn stub_0x51f60() -> ! {
    todo!("0x51f60 -[MainViewController getOgreView]")
}

#[doc(alias = "-[MainViewController setOgreView:]")]
// 0x51f70 — -[MainViewController setOgreView:]
// type: void __cdecl(MainViewController *self, SEL, id)
pub fn stub_0x51f70() -> ! {
    todo!("0x51f70 -[MainViewController setOgreView:]")
}

#[doc(alias = "-[MainViewController getOgreViewController]")]
// 0x51fa0 — -[MainViewController getOgreViewController]
// type: id __cdecl(MainViewController *self, SEL)
pub fn stub_0x51fa0() -> ! {
    todo!("0x51fa0 -[MainViewController getOgreViewController]")
}

#[doc(alias = "-[MainViewController setOgreViewController:]")]
// 0x51fb0 — -[MainViewController setOgreViewController:]
// type: void __cdecl(MainViewController *self, SEL, id)
pub fn stub_0x51fb0() -> ! {
    todo!("0x51fb0 -[MainViewController setOgreViewController:]")
}

#[doc(alias = "+[UIWebViewCacheManager sharedInstance]")]
// 0x584e4 — +[UIWebViewCacheManager sharedInstance]
// type: id __cdecl(id, SEL)
pub fn stub_0x584e4() -> ! {
    todo!("0x584e4 +[UIWebViewCacheManager sharedInstance]")
}

#[doc(alias = "___39+[UIWebViewCacheManager sharedInstance]_block_invoke")]
// 0x58540 — ___39+[UIWebViewCacheManager sharedInstance]_block_invoke
// type: id __fastcall(int)
pub fn stub_0x58540() -> ! {
    todo!("0x58540 ___39+[UIWebViewCacheManager sharedInstance]_block_invoke")
}

#[doc(alias = "___destroy_helper_block_56")]
// 0x58580 — ___destroy_helper_block_56
// type: void __fastcall(int)
pub fn stub_0x58580() -> ! {
    todo!("0x58580 ___destroy_helper_block_56")
}

#[doc(alias = "___copy_helper_block_78")]
// 0x58844 — ___copy_helper_block_78
// type: void __fastcall(int, int)
pub fn stub_0x58844() -> ! {
    todo!("0x58844 ___copy_helper_block_78")
}

#[doc(alias = "___destroy_helper_block_79")]
// 0x58850 — ___destroy_helper_block_79
// type: void __fastcall(int)
pub fn stub_0x58850() -> ! {
    todo!("0x58850 ___destroy_helper_block_79")
}

#[doc(alias = "___copy_helper_block_83")]
// 0x589f4 — ___copy_helper_block_83
// type: void __fastcall(int, int)
pub fn stub_0x589f4() -> ! {
    todo!("0x589f4 ___copy_helper_block_83")
}

#[doc(alias = "___destroy_helper_block_84")]
// 0x58a00 — ___destroy_helper_block_84
// type: void __fastcall(int)
pub fn stub_0x58a00() -> ! {
    todo!("0x58a00 ___destroy_helper_block_84")
}

#[doc(alias = "global constructor keyed to_a_30")]
// 0x58bb0 — __GLOBAL__I_a_30
pub fn stub_0x58bb0() -> ! {
    todo!("0x58bb0 global constructor keyed to_a_30")
}

#[doc(alias = "+[LoginManager sharedInstance]")]
// 0x58f94 — +[LoginManager sharedInstance]
// type: id __cdecl(id, SEL)
pub fn stub_0x58f94() -> ! {
    todo!("0x58f94 +[LoginManager sharedInstance]")
}

#[doc(alias = "___30+[LoginManager sharedInstance]_block_invoke")]
// 0x58ff0 — ___30+[LoginManager sharedInstance]_block_invoke
// type: id __fastcall(int)
pub fn stub_0x58ff0() -> ! {
    todo!("0x58ff0 ___30+[LoginManager sharedInstance]_block_invoke")
}

#[doc(alias = "___copy_helper_block__18")]
// 0x59024 — ___copy_helper_block__18
// type: void __fastcall(int, int)
pub fn stub_0x59024() -> ! {
    todo!("0x59024 ___copy_helper_block__18")
}

#[doc(alias = "___destroy_helper_block__18")]
// 0x59030 — ___destroy_helper_block__18
// type: void __fastcall(int)
pub fn stub_0x59030() -> ! {
    todo!("0x59030 ___destroy_helper_block__18")
}

#[doc(alias = "___copy_helper_block_149")]
// 0x59aa8 — ___copy_helper_block_149
// type: void __fastcall(int, int)
pub fn stub_0x59aa8() -> ! {
    todo!("0x59aa8 ___copy_helper_block_149")
}

#[doc(alias = "___destroy_helper_block_150")]
// 0x59acc — ___destroy_helper_block_150
// type: void __fastcall(int)
pub fn stub_0x59acc() -> ! {
    todo!("0x59acc ___destroy_helper_block_150")
}

#[doc(alias = "___copy_helper_block_192")]
// 0x5a068 — ___copy_helper_block_192
// type: void __fastcall(int, const void **)
pub fn stub_0x5a068() -> ! {
    todo!("0x5a068 ___copy_helper_block_192")
}

#[doc(alias = "___destroy_helper_block_193")]
// 0x5a0b0 — ___destroy_helper_block_193
// type: void __fastcall(const void **)
pub fn stub_0x5a0b0() -> ! {
    todo!("0x5a0b0 ___destroy_helper_block_193")
}

#[doc(alias = "global constructor keyed to_a_31")]
// 0x5b3d8 — __GLOBAL__I_a_31
pub fn stub_0x5b3d8() -> ! {
    todo!("0x5b3d8 global constructor keyed to_a_31")
}

#[doc(alias = "___copy_helper_block__19")]
// 0x5c4f4 — ___copy_helper_block__19
// type: void __fastcall(int, int)
pub fn stub_0x5c4f4() -> ! {
    todo!("0x5c4f4 ___copy_helper_block__19")
}

#[doc(alias = "___destroy_helper_block__19")]
// 0x5c518 — ___destroy_helper_block__19
// type: void __fastcall(int)
pub fn stub_0x5c518() -> ! {
    todo!("0x5c518 ___destroy_helper_block__19")
}

#[doc(alias = "___copy_helper_block_104")]
// 0x5c6c8 — ___copy_helper_block_104
// type: void __fastcall(int, int)
pub fn stub_0x5c6c8() -> ! {
    todo!("0x5c6c8 ___copy_helper_block_104")
}

#[doc(alias = "___destroy_helper_block_105")]
// 0x5c6ec — ___destroy_helper_block_105
// type: void __fastcall(int)
pub fn stub_0x5c6ec() -> ! {
    todo!("0x5c6ec ___destroy_helper_block_105")
}

#[doc(alias = "___copy_helper_block_126")]
// 0x5cad4 — ___copy_helper_block_126
// type: void __fastcall(int, int)
pub fn stub_0x5cad4() -> ! {
    todo!("0x5cad4 ___copy_helper_block_126")
}

#[doc(alias = "___destroy_helper_block_127")]
// 0x5cae0 — ___destroy_helper_block_127
// type: void __fastcall(int)
pub fn stub_0x5cae0() -> ! {
    todo!("0x5cae0 ___destroy_helper_block_127")
}

#[doc(alias = "___copy_helper_block_162")]
// 0x5d1a8 — ___copy_helper_block_162
// type: void __fastcall(int, int)
pub fn stub_0x5d1a8() -> ! {
    todo!("0x5d1a8 ___copy_helper_block_162")
}

#[doc(alias = "___destroy_helper_block_163")]
// 0x5d1b4 — ___destroy_helper_block_163
// type: void __fastcall(int)
pub fn stub_0x5d1b4() -> ! {
    todo!("0x5d1b4 ___destroy_helper_block_163")
}

#[doc(alias = "___copy_helper_block__20")]
// 0x5ed84 — ___copy_helper_block__20
// type: void __fastcall(int, int)
pub fn stub_0x5ed84() -> ! {
    todo!("0x5ed84 ___copy_helper_block__20")
}

#[doc(alias = "___destroy_helper_block__20")]
// 0x5ed90 — ___destroy_helper_block__20
// type: void __fastcall(int)
pub fn stub_0x5ed90() -> ! {
    todo!("0x5ed90 ___destroy_helper_block__20")
}

#[doc(alias = "___copy_helper_block_232_0")]
// 0x5f024 — ___copy_helper_block_232_0
// type: void __fastcall(int, int)
pub fn stub_0x5f024() -> ! {
    todo!("0x5f024 ___copy_helper_block_232_0")
}

#[doc(alias = "___destroy_helper_block_233_0")]
// 0x5f030 — ___destroy_helper_block_233_0
// type: void __fastcall(int)
pub fn stub_0x5f030() -> ! {
    todo!("0x5f030 ___destroy_helper_block_233_0")
}

#[doc(alias = "___copy_helper_block_252_0")]
// 0x5f3e4 — ___copy_helper_block_252_0
// type: void __fastcall(int, int)
pub fn stub_0x5f3e4() -> ! {
    todo!("0x5f3e4 ___copy_helper_block_252_0")
}

