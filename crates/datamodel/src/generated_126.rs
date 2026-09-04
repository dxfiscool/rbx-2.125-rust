// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: demangled contains RBX::Instance|RBX::DataModel|RBX::Workspace (exact), EA-sorted
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0x38ff5c..0x3c2a08 | total filtered 10215, remaining 6534->6434 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
// 0x38ff5c — __ZN3RBX12Accoutrement14onChildRemovedEPNS_8InstanceE
#[doc(alias = "RBX::Accoutrement::onChildRemoved(RBX::Instance *)")]
pub use crate::instance::stub_0x38ff5c as stub_0x38ff5c;
// 0x390234 — __ZN3RBX8Instance15queryTypedChildINS_13CameraSubjectEEEPT_i
#[doc(alias = "RBX::CameraSubject * RBX::Instance::queryTypedChild<RBX::CameraSubject>(int)")]
pub use crate::instance::stub_0x390234 as stub_0x390234;
// 0x390270 — __ZN3RBX12PartInstance13TouchedSignal7connectIN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_12AccoutrementENS3_10shared_ptrINS_8InstanceEEEEENS4_5list2INS4_5valueIPS8_EENS3_3argILi1EEEEEEEEEN3rbx7signals10connectionET_
#[doc(alias = "rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>)")]
// was: rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>)
pub use crate::instance::stub_0x390270 as stub_0x390270;
// 0x3903f0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_12AccoutrementES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>> const&)")]
// was: rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>> const&)
pub use crate::instance::stub_0x3903f0 as stub_0x3903f0;
// 0x390654 — __ZNK3RBX12Accoutrement11askAddChildEPKNS_8InstanceE
#[doc(alias = "RBX::Accoutrement::askAddChild(RBX::Instance const*)const")]
pub use crate::instance::stub_0x390654 as stub_0x390654;
// 0x390658 — __ZNK3RBX12Accoutrement12askSetParentEPKNS_8InstanceE
#[doc(alias = "RBX::Accoutrement::askSetParent(RBX::Instance const*)const")]
pub use crate::instance::stub_0x390658 as stub_0x390658;
// 0x391798 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_3HatEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Hat> RBX::Creatable<RBX::Instance>::create<RBX::Hat>(void)")]
// was: boost::shared_ptr<RBX::Hat> RBX::Creatable<RBX::Instance>::create<RBX::Hat>(void)
pub use crate::instance::stub_0x391798 as stub_0x391798;
// 0x391848 — __ZN5boost10shared_ptrIN3RBX3HatEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Hat>::shared_ptr<RBX::Hat,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Hat>::shared_ptr<RBX::Hat,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x391848 as stub_0x391848;
// 0x3919f8 — __ZN5boost6detail12shared_countC2IPN3RBX3HatENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x3919f8 as stub_0x3919f8;
// 0x391b00 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3HatENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x391b00 as stub_0x391b00;
// 0x391b04 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3HatENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x391b04 as stub_0x391b04;
// 0x391b08 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3HatENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub use crate::instance::stub_0x391b08 as stub_0x391b08;
// 0x391b28 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3HatENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x391b28 as stub_0x391b28;
// 0x391b40 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3HatENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x391b40 as stub_0x391b40;
// 0x391ff0 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12AccoutrementEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Accoutrement> RBX::Creatable<RBX::Instance>::create<RBX::Accoutrement>(void)")]
// was: boost::shared_ptr<RBX::Accoutrement> RBX::Creatable<RBX::Instance>::create<RBX::Accoutrement>(void)
pub use crate::instance::stub_0x391ff0 as stub_0x391ff0;
// 0x3920a0 — __ZN5boost10shared_ptrIN3RBX12AccoutrementEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Accoutrement>::shared_ptr<RBX::Accoutrement,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Accoutrement>::shared_ptr<RBX::Accoutrement,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x3920a0 as stub_0x3920a0;
// 0x392250 — __ZN5boost6detail12shared_countC2IPN3RBX12AccoutrementENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x392250 as stub_0x392250;
// 0x392358 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AccoutrementENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x392358 as stub_0x392358;
// 0x39235c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AccoutrementENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x39235c as stub_0x39235c;
// 0x392360 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AccoutrementENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub use crate::instance::stub_0x392360 as stub_0x392360;
// 0x392380 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AccoutrementENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x392380 as stub_0x392380;
// 0x392398 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AccoutrementENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x392398 as stub_0x392398;
// 0x392804 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_12AccoutrementES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>::~callable_slot()
pub use crate::instance::stub_0x392804 as stub_0x392804;
// 0x392830 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_12AccoutrementES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>::~callable_slot()
pub use crate::instance::stub_0x392830 as stub_0x392830;
// 0x392904 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_12AccoutrementES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub use crate::instance::stub_0x392904 as stub_0x392904;
// 0x392920 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_12AccoutrementES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub use crate::instance::stub_0x392920 as stub_0x392920;
// 0x39293c — __ZN5boost3_bi5list2INS0_5valueIPN3RBX12AccoutrementEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::Accoutrement *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
// was: void boost::_bi::list2<boost::_bi::value<RBX::Accoutrement *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int)
pub use crate::instance::stub_0x39293c as stub_0x39293c;
// 0x392a14 — __ZNK5boost4_mfi3mf1IvN3RBX12AccoutrementENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_
#[doc(alias = "boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Accoutrement*,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Accoutrement*,boost::shared_ptr<RBX::Instance>)const
pub use crate::instance::stub_0x392a14 as stub_0x392a14;
// 0x392afc — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_12AccoutrementES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()
pub use crate::instance::stub_0x392afc as stub_0x392afc;
// 0x392b28 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_12AccoutrementES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()
pub use crate::instance::stub_0x392b28 as stub_0x392b28;
// 0x392bfc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX12AccoutrementENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub use crate::instance::stub_0x392bfc as stub_0x392bfc;
// 0x392c5c — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX12AccoutrementENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,void,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,void,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>)
pub use crate::instance::stub_0x392c5c as stub_0x392c5c;
// 0x392c78 — __ZNK3RBX8Instance25findConstFirstChildOfTypeINS_12AccoutrementEEEPKT_v
#[doc(alias = "RBX::Accoutrement const* RBX::Instance::findConstFirstChildOfType<RBX::Accoutrement>(void)const")]
pub use crate::instance::stub_0x392c78 as stub_0x392c78;
// 0x395538 — __ZNK3RBX10Reflection17RefPropDescriptorINS_11PVAdornmentENS_10PVInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub use crate::instance::stub_0x395538 as stub_0x395538;
// 0x395618 — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_11PVAdornmentENS_10PVInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub use crate::instance::stub_0x395618 as stub_0x395618;
// 0x396080 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13PartAdornmentENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub use crate::instance::stub_0x396080 as stub_0x396080;
// 0x396160 — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_13PartAdornmentENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub use crate::instance::stub_0x396160 as stub_0x396160;
// 0x396c40 — __ZNK3RBX9Animation19getKeyframeSequenceEPKNS_8InstanceE
#[doc(alias = "RBX::Animation::getKeyframeSequence(RBX::Instance const*)const")]
pub use crate::instance::stub_0x396c40 as stub_0x396c40;
// 0x396e44 — __ZN3RBX15ServiceProvider6createINS_24KeyframeSequenceProviderEEEPT_PKNS_8InstanceE
#[doc(alias = "RBX::KeyframeSequenceProvider * RBX::ServiceProvider::create<RBX::KeyframeSequenceProvider>(RBX::Instance const*)")]
pub use crate::instance::stub_0x396e44 as stub_0x396e44;
// 0x3970bc — __ZNK3RBX9Animation12askSetParentEPKNS_8InstanceE
#[doc(alias = "RBX::Animation::askSetParent(RBX::Instance const*)const")]
pub use crate::instance::stub_0x3970bc as stub_0x3970bc;
// 0x39bdb4 — __ZN3RBX19AnimationTrackState28triggerKeyframeReachedSignalERKN5boost10shared_ptrINS_8InstanceEEEdd
#[doc(alias = "RBX::AnimationTrackState::triggerKeyframeReachedSignal(rbx_core::SharedPtr<RBX::Instance> const&,double,double)")]
// was: RBX::AnimationTrackState::triggerKeyframeReachedSignal(boost::shared_ptr<RBX::Instance> const&,double,double)
pub use crate::instance::stub_0x39bdb4 as stub_0x39bdb4;
// 0x3a395c — __ZN3RBX8Animator13loadAnimationEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Animator::loadAnimation(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Animator::loadAnimation(boost::shared_ptr<RBX::Instance>)
pub use crate::instance::stub_0x3a395c as stub_0x3a395c;
// 0x3a3d44 — __ZN3RBX8AnimatorC1EPNS_8InstanceE
#[doc(alias = "RBX::Animator::Animator(RBX::Instance *)")]
pub use crate::instance::stub_0x3a3d44 as stub_0x3a3d44;
// 0x3a3d48 — __ZN3RBX8AnimatorC2EPNS_8InstanceE
#[doc(alias = "RBX::Animator::Animator(RBX::Instance *)")]
pub use crate::instance::stub_0x3a3d48 as stub_0x3a3d48;
// 0x3a46a0 — __ZN3RBX8Animator25appendAnimatableJointsRecEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Animator::appendAnimatableJointsRec(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Animator::appendAnimatableJointsRec(boost::shared_ptr<RBX::Instance>)
pub use crate::instance::stub_0x3a46a0 as stub_0x3a46a0;
// 0x3a4ea0 — __ZNK3RBX8Animator11askAddChildEPKNS_8InstanceE
#[doc(alias = "RBX::Animator::askAddChild(RBX::Instance const*)const")]
pub use crate::instance::stub_0x3a4ea0 as stub_0x3a4ea0;
// 0x3a4edc — __ZN3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Animator,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Animator,boost::shared_ptr<RBX::Instance> ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()
pub use crate::instance::stub_0x3a4edc as stub_0x3a4edc;
// 0x3a5218 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_19AnimationTrackStateEN5boost10shared_ptrIKNS_16KeyframeSequenceEEENS6_INS_8AnimatorEEEEENS6_IT_EET0_T1_
#[doc(alias = "rbx_core::SharedPtr<RBX::AnimationTrackState> RBX::Creatable<RBX::Instance>::create<RBX::AnimationTrackState,rbx_core::SharedPtr<RBX::KeyframeSequence const>,rbx_core::SharedPtr<RBX::Animator>>(rbx_core::SharedPtr<RBX::KeyframeSequence const>,rbx_core::SharedPtr<RBX::Animator>)")]
// was: boost::shared_ptr<RBX::AnimationTrackState> RBX::Creatable<RBX::Instance>::create<RBX::AnimationTrackState,boost::shared_ptr<RBX::KeyframeSequence const>,boost::shared_ptr<RBX::Animator>>(boost::shared_ptr<RBX::KeyframeSequence const>,boost::shared_ptr<RBX::Animator>)
pub use crate::instance::stub_0x3a5218 as stub_0x3a5218;
// 0x3a5380 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_14AnimationTrackEN5boost10shared_ptrINS_19AnimationTrackStateEEENS6_INS_8AnimatorEEEEENS6_IT_EET0_T1_
#[doc(alias = "rbx_core::SharedPtr<RBX::AnimationTrack> RBX::Creatable<RBX::Instance>::create<RBX::AnimationTrack,rbx_core::SharedPtr<RBX::AnimationTrackState>,rbx_core::SharedPtr<RBX::Animator>>(rbx_core::SharedPtr<RBX::AnimationTrackState>,rbx_core::SharedPtr<RBX::Animator>)")]
// was: boost::shared_ptr<RBX::AnimationTrack> RBX::Creatable<RBX::Instance>::create<RBX::AnimationTrack,boost::shared_ptr<RBX::AnimationTrackState>,boost::shared_ptr<RBX::Animator>>(boost::shared_ptr<RBX::AnimationTrackState>,boost::shared_ptr<RBX::Animator>)
pub use crate::instance::stub_0x3a5380 as stub_0x3a5380;
// 0x3a55fc — __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_8AnimatorENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPS7_EENS2_3argILi1EEEEEEEEEvRKT_
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Animator*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Animator*>,boost::arg<1>>> const&)const")]
// was: void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Animator,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Animator*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Animator,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Animator*>,boost::arg<1>>> const&)const
pub use crate::instance::stub_0x3a55fc as stub_0x3a55fc;
// 0x3a5880 — __ZNK3RBX8Animator12askSetParentEPKNS_8InstanceE
#[doc(alias = "RBX::Animator::askSetParent(RBX::Instance const*)const")]
pub use crate::instance::stub_0x3a5880 as stub_0x3a5880;
// 0x3a5cd4 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX8AnimatorEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRKSF_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::Animator *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
// was: void boost::_bi::list2<boost::_bi::value<RBX::Animator *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Animator,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Animator,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
pub use crate::instance::stub_0x3a5cd4 as stub_0x3a5cd4;
// 0x3a5dac — __ZNK5boost4_mfi3mf1IvN3RBX8AnimatorENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_
#[doc(alias = "boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Animator*,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: boost::_mfi::mf1<void,RBX::Animator,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Animator*,boost::shared_ptr<RBX::Instance>)const
pub use crate::instance::stub_0x3a5dac as stub_0x3a5dac;
// 0x3a61fc — __ZN5boost10shared_ptrIN3RBX14AnimationTrackEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::AnimationTrack>::shared_ptr<RBX::AnimationTrack,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::AnimationTrack>::shared_ptr<RBX::AnimationTrack,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x3a61fc as stub_0x3a61fc;
// 0x3a63ac — __ZN5boost6detail12shared_countC2IPN3RBX14AnimationTrackENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x3a63ac as stub_0x3a63ac;
// 0x3a64b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x3a64b4 as stub_0x3a64b4;
// 0x3a64b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x3a64b8 as stub_0x3a64b8;
// 0x3a64bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub use crate::instance::stub_0x3a64bc as stub_0x3a64bc;
// 0x3a64dc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x3a64dc as stub_0x3a64dc;
// 0x3a64f4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x3a64f4 as stub_0x3a64f4;
// 0x3a64f8 — __ZN5boost10shared_ptrIN3RBX19AnimationTrackStateEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::AnimationTrackState>::shared_ptr<RBX::AnimationTrackState,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::AnimationTrackState>::shared_ptr<RBX::AnimationTrackState,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x3a64f8 as stub_0x3a64f8;
// 0x3a66a8 — __ZN5boost6detail12shared_countC2IPN3RBX19AnimationTrackStateENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x3a66a8 as stub_0x3a66a8;
// 0x3a67b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x3a67b0 as stub_0x3a67b0;
// 0x3a67b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x3a67b4 as stub_0x3a67b4;
// 0x3a67b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub use crate::instance::stub_0x3a67b8 as stub_0x3a67b8;
// 0x3a67d8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x3a67d8 as stub_0x3a67d8;
// 0x3a67f0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x3a67f0 as stub_0x3a67f0;
// 0x3a6dc0 — __ZN3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EEC2EMS2_FS6_S6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Animator,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::Animator::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Animator,boost::shared_ptr<RBX::Instance> ()(boost::shared_ptr<RBX::Instance>),1>::BoundFuncDesc(boost::shared_ptr<RBX::Instance> (RBX::Animator::*)(boost::shared_ptr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub use crate::instance::stub_0x3a6dc0 as stub_0x3a6dc0;
// 0x3a6f58 — __ZN3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Animator,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Animator,boost::shared_ptr<RBX::Instance> ()(boost::shared_ptr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)
pub use crate::instance::stub_0x3a6f58 as stub_0x3a6f58;
// 0x3a6f88 — __ZN3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Animator,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Animator,boost::shared_ptr<RBX::Instance> ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()
pub use crate::instance::stub_0x3a6f88 as stub_0x3a6f88;
// 0x3a70a4 — __ZNK3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Animator,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Animator,boost::shared_ptr<RBX::Instance> ()(boost::shared_ptr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub use crate::instance::stub_0x3a70a4 as stub_0x3a70a4;
// 0x3a718c — __ZN3RBX10Reflection11Call1HelperINS_8AnimatorEMS2_FN5boost10shared_ptrINS_8InstanceEEES6_ES6_S6_E4callEPS2_S8_RNS0_7VariantERKS6_
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Animator,rbx_core::SharedPtr<RBX::Instance> (RBX::Animator::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::call(RBX::Animator*,rbx_core::SharedPtr<RBX::Instance> (RBX::Animator::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: RBX::Reflection::Call1Helper<RBX::Animator,boost::shared_ptr<RBX::Instance> (RBX::Animator::*)(boost::shared_ptr<RBX::Instance>),boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::call(RBX::Animator*,boost::shared_ptr<RBX::Instance> (RBX::Animator::*)(boost::shared_ptr<RBX::Instance>),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&)
pub use crate::instance::stub_0x3a718c as stub_0x3a718c;
// 0x3a8a58 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10ArcHandlesEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::ArcHandles> RBX::Creatable<RBX::Instance>::create<RBX::ArcHandles>(void)")]
// was: boost::shared_ptr<RBX::ArcHandles> RBX::Creatable<RBX::Instance>::create<RBX::ArcHandles>(void)
pub use crate::instance::stub_0x3a8a58 as stub_0x3a8a58;
// 0x3a8b0c — __ZN5boost10shared_ptrIN3RBX10ArcHandlesEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::ArcHandles>::shared_ptr<RBX::ArcHandles,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::ArcHandles>::shared_ptr<RBX::ArcHandles,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x3a8b0c as stub_0x3a8b0c;
// 0x3a8cbc — __ZN5boost6detail12shared_countC2IPN3RBX10ArcHandlesENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x3a8cbc as stub_0x3a8cbc;
// 0x3a8dc4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x3a8dc4 as stub_0x3a8dc4;
// 0x3a8dc8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x3a8dc8 as stub_0x3a8dc8;
// 0x3a8dcc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub use crate::instance::stub_0x3a8dcc as stub_0x3a8dcc;
// 0x3a8dec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x3a8dec as stub_0x3a8dec;
// 0x3a8e04 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x3a8e04 as stub_0x3a8e04;
// 0x3b16ac — __ZN3RBX9CreatableINS_8InstanceEE6createINS_8BackpackEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Backpack> RBX::Creatable<RBX::Instance>::create<RBX::Backpack>(void)")]
// was: boost::shared_ptr<RBX::Backpack> RBX::Creatable<RBX::Instance>::create<RBX::Backpack>(void)
pub use crate::instance::stub_0x3b16ac as stub_0x3b16ac;
// 0x3b175c — __ZN5boost10shared_ptrIN3RBX8BackpackEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Backpack>::shared_ptr<RBX::Backpack,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Backpack *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Backpack>::shared_ptr<RBX::Backpack,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Backpack *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x3b175c as stub_0x3b175c;
// 0x3b1824 — __ZN5boost6detail12shared_countC2IPN3RBX8BackpackENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Backpack *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Backpack *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x3b1824 as stub_0x3b1824;
// 0x3bfaf4 — __ZN3RBX12BillboardGui10setAdorneeEPNS_8InstanceE
#[doc(alias = "RBX::BillboardGui::setAdornee(RBX::Instance *)")]
pub use crate::instance::stub_0x3bfaf4 as stub_0x3bfaf4;
// 0x3bffc0 — __ZN3RBX12BillboardGui19setPlayerToHideFromEPNS_8InstanceE
#[doc(alias = "RBX::BillboardGui::setPlayerToHideFrom(RBX::Instance *)")]
pub use crate::instance::stub_0x3bffc0 as stub_0x3bffc0;
// 0x3c0434 — __ZNK3RBX12BillboardGui12askSetParentEPKNS_8InstanceE
#[doc(alias = "RBX::BillboardGui::askSetParent(RBX::Instance const*)const")]
pub use crate::instance::stub_0x3c0434 as stub_0x3c0434;
// 0x3c0f7c — __ZN3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEED1Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::~RefPropDescriptor()")]
pub use crate::instance::stub_0x3c0f7c as stub_0x3c0f7c;
// 0x3c2560 — __ZN3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::RefPropDescriptor<RBX::Instance* (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::Instance*)>(char const*,char const*,RBX::Instance* (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(RBX::Instance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub use crate::instance::stub_0x3c2560 as stub_0x3c2560;
// 0x3c2604 — __ZN3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEED0Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::~RefPropDescriptor()")]
pub use crate::instance::stub_0x3c2604 as stub_0x3c2604;
// 0x3c2634 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::isReadOnly(void)const")]
pub use crate::instance::stub_0x3c2634 as stub_0x3c2634;
// 0x3c2644 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::isWriteOnly(void)const")]
pub use crate::instance::stub_0x3c2644 as stub_0x3c2644;
// 0x3c2654 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub use crate::instance::stub_0x3c2654 as stub_0x3c2654;
// 0x3c267c — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub use crate::instance::stub_0x3c267c as stub_0x3c267c;
// 0x3c2794 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub use crate::instance::stub_0x3c2794 as stub_0x3c2794;
// 0x3c285c — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub use crate::instance::stub_0x3c285c as stub_0x3c285c;
// 0x3c2880 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub use crate::instance::stub_0x3c2880 as stub_0x3c2880;
// 0x3c2954 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub use crate::instance::stub_0x3c2954 as stub_0x3c2954;
// 0x3c2978 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11getRefValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
pub use crate::instance::stub_0x3c2978 as stub_0x3c2978;
// 0x3c298c — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE11setRefValueEPNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
pub use crate::instance::stub_0x3c298c as stub_0x3c298c;
// 0x3c2a08 — __ZNK3RBX10Reflection17RefPropDescriptorINS_12BillboardGuiENS_8InstanceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::BillboardGui,RBX::Instance>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
pub use crate::instance::stub_0x3c2a08 as stub_0x3c2a08;
