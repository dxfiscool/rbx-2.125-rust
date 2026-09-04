// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: demangled contains RBX::Instance|RBX::DataModel|RBX::Workspace (exact), EA-sorted
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0x43ba50..0x44bb44 | total filtered 10215, remaining 5847 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
// 0x43ba50 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE4nextERNS2_13intrusive_ptrINSC_4slotEEE
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot> &)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot> &)
pub use crate::instance::stub_0x43ba50 as stub_43ba50;
// 0x43bbb0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::on_error(std::exception &)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::on_error(std::exception &)
pub use crate::instance::stub_0x43bbb0 as stub_43bbb0;
// 0x43bbd8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEEPKNS5_10Reflection18PropertyDescriptorEEE4slotEEaSERKSF_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot> const&)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot> const&)
pub use crate::instance::stub_0x43bbd8 as stub_43bbd8;
// 0x43bbfc — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::safe_static_init_mutex(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::safe_static_init_mutex(void)
pub use crate::instance::stub_0x43bbfc as stub_43bbfc;
// 0x43bc00 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::safe_static_do_get_mutex(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::safe_static_do_get_mutex(void)
pub use crate::instance::stub_0x43bc00 as stub_43bc00;
// 0x43bcf8 — __ZN5boost3_bi5list5INS0_5valueIPN3RBX9DataModelEEENS_3argILi1EEENS2_ISsEENS2_IdEENS2_IPdEEEclIPFvS5_NS_10shared_ptrIKNS3_13TaskScheduler3JobEEERSsdSB_ENS0_5list1IRSJ_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>,boost::_bi::value<double *>>::operator()<void (*)(RBX::DataModel *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::string &,double,double *),boost::_bi::list1<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>&>>(boost::_bi::type<void>,void (*)(RBX::DataModel *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::string &,double,double *) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>&> &,int)")]
// was: void boost::_bi::list5<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>,boost::_bi::value<double *>>::operator()<void (*)(RBX::DataModel *,boost::shared_ptr<RBX::TaskScheduler::Job const>,std::string &,double,double *),boost::_bi::list1<boost::shared_ptr<RBX::TaskScheduler::Job const>&>>(boost::_bi::type<void>,void (*)(RBX::DataModel *,boost::shared_ptr<RBX::TaskScheduler::Job const>,std::string &,double,double *) &,boost::_bi::list1<boost::shared_ptr<RBX::TaskScheduler::Job const>&> &,int)
pub use crate::instance::stub_0x43bcf8 as stub_43bcf8;
// 0x43bde0 — __ZN5boost3_bi5list5INS0_5valueIPN3RBX9DataModelEEENS_3argILi1EEENS2_ISsEENS2_IdEENS2_IPdEEEC2ES6_S8_S9_SA_SC_
#[doc(alias = "boost::_bi::list5<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>,boost::_bi::value<double *>>::list5(boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>,boost::_bi::value<double *>)")]
// was: boost::_bi::list5<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>,boost::_bi::value<double *>>::list5(boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>,boost::_bi::value<double *>)
pub use crate::instance::stub_0x43bde0 as stub_43bde0;
// 0x43bf10 — __ZN5boost3_bi8storage5INS0_5valueIPN3RBX9DataModelEEENS_3argILi1EEENS2_ISsEENS2_IdEENS2_IPdEEEC2ES6_S8_S9_SA_SC_
#[doc(alias = "boost::_bi::storage5<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>,boost::_bi::value<double *>>::storage5(boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>,boost::_bi::value<double *>)")]
// was: boost::_bi::storage5<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>,boost::_bi::value<double *>>::storage5(boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>,boost::_bi::value<double *>)
pub use crate::instance::stub_0x43bf10 as stub_43bf10;
// 0x43c044 — __ZN5boost3_bi8storage4INS0_5valueIPN3RBX9DataModelEEENS_3argILi1EEENS2_ISsEENS2_IdEEEC2ES6_S8_S9_SA_
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>>::storage4(boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>)")]
// was: boost::_bi::storage4<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>>::storage4(boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>)
pub use crate::instance::stub_0x43c044 as stub_43c044;
// 0x43c250 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX9DataModelEEENS_3argILi1EEENS2_IPSt6vectorINS3_10Reflection7VariantESaISB_EEEEEclIPFvS5_NS_10shared_ptrIKNS3_13TaskScheduler3JobEEESE_ENS0_5list1IRSM_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *>>::operator()<void (*)(RBX::DataModel *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *),boost::_bi::list1<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>&>>(boost::_bi::type<void>,void (*)(RBX::DataModel *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>&> &,int)")]
// was: void boost::_bi::list3<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *>>::operator()<void (*)(RBX::DataModel *,boost::shared_ptr<RBX::TaskScheduler::Job const>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *),boost::_bi::list1<boost::shared_ptr<RBX::TaskScheduler::Job const>&>>(boost::_bi::type<void>,void (*)(RBX::DataModel *,boost::shared_ptr<RBX::TaskScheduler::Job const>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *) &,boost::_bi::list1<boost::shared_ptr<RBX::TaskScheduler::Job const>&> &,int)
pub use crate::instance::stub_0x43c250 as stub_43c250;
// 0x440b48 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TestServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TestService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::TestService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub use crate::instance::stub_0x440b48 as stub_440b48;
// 0x440b50 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TestServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TestService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::TestService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub use crate::instance::stub_0x440b50 as stub_440b50;
// 0x440b70 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TestServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TestService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: boost::detail::sp_counted_impl_pd<RBX::TestService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub use crate::instance::stub_0x440b70 as stub_440b70;
// 0x440b88 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TestServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TestService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::TestService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub use crate::instance::stub_0x440b88 as stub_440b88;
// 0x441928 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5TeamsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Teams *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::Teams *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub use crate::instance::stub_0x441928 as stub_441928;
// 0x442184 — __ZN5boost20dynamic_pointer_castIN3RBX9GuiTargetENS1_8InstanceEEENS_10shared_ptrIT_EERKNS4_IT0_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::GuiTarget> boost::dynamic_pointer_cast<RBX::GuiTarget,RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: boost::shared_ptr<RBX::GuiTarget> boost::dynamic_pointer_cast<RBX::GuiTarget,RBX::Instance>(boost::shared_ptr<RBX::Instance> const&)
pub use crate::instance::stub_0x442184 as stub_442184;
// 0x4421cc — __ZN5boost10shared_ptrIN3RBX8InstanceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>::shared_ptr<RBX::Instance>(rbx_core::Weak<RBX::Instance> const&,boost::detail::sp_nothrow_tag)")]
// was: boost::shared_ptr<RBX::Instance>::shared_ptr<RBX::Instance>(boost::weak_ptr<RBX::Instance> const&,boost::detail::sp_nothrow_tag)
pub use crate::instance::stub_0x4421cc as stub_4421cc;
// 0x443098 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub use crate::instance::stub_0x443098 as stub_443098;
// 0x4434cc — __ZN3RBX9CreatableINS_8InstanceEE6createINS_19ServerScriptServiceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::ServerScriptService> RBX::Creatable<RBX::Instance>::create<RBX::ServerScriptService>(void)")]
// was: boost::shared_ptr<RBX::ServerScriptService> RBX::Creatable<RBX::Instance>::create<RBX::ServerScriptService>(void)
pub use crate::instance::stub_0x4434cc as stub_4434cc;
// 0x44357c — __ZN5boost10shared_ptrIN3RBX19ServerScriptServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::ServerScriptService>::shared_ptr<RBX::ServerScriptService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::ServerScriptService>::shared_ptr<RBX::ServerScriptService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x44357c as stub_44357c;
// 0x443730 — __ZN5boost6detail12shared_countC2IPN3RBX19ServerScriptServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::detail::shared_count::shared_count<RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x443730 as stub_443730;
// 0x443838 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19ServerScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub use crate::instance::stub_0x443838 as stub_443838;
// 0x44383c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19ServerScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub use crate::instance::stub_0x44383c as stub_44383c;
// 0x443840 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19ServerScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub use crate::instance::stub_0x443840 as stub_443840;
// 0x443860 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19ServerScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: boost::detail::sp_counted_impl_pd<RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub use crate::instance::stub_0x443860 as stub_443860;
// 0x443878 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19ServerScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub use crate::instance::stub_0x443878 as stub_443878;
// 0x444148 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_17ReplicatedStorageEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::ReplicatedStorage> RBX::Creatable<RBX::Instance>::create<RBX::ReplicatedStorage>(void)")]
// was: boost::shared_ptr<RBX::ReplicatedStorage> RBX::Creatable<RBX::Instance>::create<RBX::ReplicatedStorage>(void)
pub use crate::instance::stub_0x444148 as stub_444148;
// 0x4441f8 — __ZN5boost10shared_ptrIN3RBX17ReplicatedStorageEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::ReplicatedStorage>::shared_ptr<RBX::ReplicatedStorage,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::ReplicatedStorage>::shared_ptr<RBX::ReplicatedStorage,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x4441f8 as stub_4441f8;
// 0x4443ac — __ZN5boost6detail12shared_countC2IPN3RBX17ReplicatedStorageENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::detail::shared_count::shared_count<RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x4443ac as stub_4443ac;
// 0x4444b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ReplicatedStorageENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub use crate::instance::stub_0x4444b8 as stub_4444b8;
// 0x4444bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ReplicatedStorageENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: boost::detail::sp_counted_impl_pd<RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub use crate::instance::stub_0x4444bc as stub_4444bc;
// 0x4444d4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ReplicatedStorageENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub use crate::instance::stub_0x4444d4 as stub_4444d4;
// 0x444b5c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13ServerStorageEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::ServerStorage> RBX::Creatable<RBX::Instance>::create<RBX::ServerStorage>(void)")]
// was: boost::shared_ptr<RBX::ServerStorage> RBX::Creatable<RBX::Instance>::create<RBX::ServerStorage>(void)
pub use crate::instance::stub_0x444b5c as stub_444b5c;
// 0x444c0c — __ZN5boost10shared_ptrIN3RBX13ServerStorageEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::ServerStorage>::shared_ptr<RBX::ServerStorage,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::ServerStorage>::shared_ptr<RBX::ServerStorage,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x444c0c as stub_444c0c;
// 0x444dc0 — __ZN5boost6detail12shared_countC2IPN3RBX13ServerStorageENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::detail::shared_count::shared_count<RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x444dc0 as stub_444dc0;
// 0x444ec8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ServerStorageENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub use crate::instance::stub_0x444ec8 as stub_444ec8;
// 0x444ecc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ServerStorageENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub use crate::instance::stub_0x444ecc as stub_444ecc;
// 0x444ed0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ServerStorageENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub use crate::instance::stub_0x444ed0 as stub_444ed0;
// 0x444ef0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ServerStorageENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: boost::detail::sp_counted_impl_pd<RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub use crate::instance::stub_0x444ef0 as stub_444ef0;
// 0x444f08 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ServerStorageENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub use crate::instance::stub_0x444f08 as stub_444f08;
// 0x445d28 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8LightingENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Lighting *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::Lighting *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub use crate::instance::stub_0x445d28 as stub_445d28;
// 0x448434 — __ZN5boost10scoped_ptrIN3RBX9DataModel10LegacyLock14ImplementationEED2Ev
#[doc(alias = "boost::scoped_ptr<RBX::DataModel::LegacyLock::Implementation>::~scoped_ptr()")]
// was: boost::scoped_ptr<RBX::DataModel::LegacyLock::Implementation>::~scoped_ptr()
pub use crate::instance::stub_0x448434 as stub_448434;
// 0x4484dc — __ZN3RBX9DataModel10LegacyLock14ImplementationD2Ev
#[doc(alias = "RBX::DataModel::LegacyLock::Implementation::~Implementation()")]
// was: RBX::DataModel::LegacyLock::Implementation::~Implementation()
pub use crate::instance::stub_0x4484dc as stub_4484dc;
// 0x4486b8 — __ZN5boost19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE5resetEPS4_
#[doc(alias = "boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::reset(RBX::DataModel::GenericJob **)")]
// was: boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::reset(RBX::DataModel::GenericJob **)
pub use crate::instance::stub_0x4486b8 as stub_4486b8;
// 0x448914 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12AssetServiceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::AssetService> RBX::Creatable<RBX::Instance>::create<RBX::AssetService>(void)")]
// was: boost::shared_ptr<RBX::AssetService> RBX::Creatable<RBX::Instance>::create<RBX::AssetService>(void)
pub use crate::instance::stub_0x448914 as stub_448914;
// 0x4489c4 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_12AssetServiceEEERS3_RKNS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::AssetService>(rbx_core::SharedPtr<RBX::AssetService> const&)")]
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::AssetService>(boost::shared_ptr<RBX::AssetService> const&)
pub use crate::instance::stub_0x4489c4 as stub_4489c4;
// 0x448c00 — __ZN5boost10shared_ptrIN3RBX12AssetServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::AssetService>::shared_ptr<RBX::AssetService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::AssetService>::shared_ptr<RBX::AssetService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x448c00 as stub_448c00;
// 0x448db4 — __ZN5boost6detail12shared_countC2IPN3RBX12AssetServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::detail::shared_count::shared_count<RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x448db4 as stub_448db4;
// 0x448ebc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AssetServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub use crate::instance::stub_0x448ebc as stub_448ebc;
// 0x448ec0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AssetServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub use crate::instance::stub_0x448ec0 as stub_448ec0;
// 0x448ec4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AssetServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub use crate::instance::stub_0x448ec4 as stub_448ec4;
// 0x448ee4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AssetServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: boost::detail::sp_counted_impl_pd<RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub use crate::instance::stub_0x448ee4 as stub_448ee4;
// 0x448efc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AssetServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub use crate::instance::stub_0x448efc as stub_448efc;
// 0x448fa4 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13ScriptServiceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptService> RBX::Creatable<RBX::Instance>::create<RBX::ScriptService>(void)")]
// was: boost::shared_ptr<RBX::ScriptService> RBX::Creatable<RBX::Instance>::create<RBX::ScriptService>(void)
pub use crate::instance::stub_0x448fa4 as stub_448fa4;
// 0x449148 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13ScriptServiceEEERS3_RKNS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ScriptService>(rbx_core::SharedPtr<RBX::ScriptService> const&)")]
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::ScriptService>(boost::shared_ptr<RBX::ScriptService> const&)
pub use crate::instance::stub_0x449148 as stub_449148;
// 0x449b10 — __ZN5boost10shared_ptrIN3RBX13ScriptServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptService>::shared_ptr<RBX::ScriptService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::ScriptService>::shared_ptr<RBX::ScriptService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x449b10 as stub_449b10;
// 0x449cc4 — __ZN5boost6detail12shared_countC2IPN3RBX13ScriptServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::detail::shared_count::shared_count<RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x449cc4 as stub_449cc4;
// 0x449dcc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub use crate::instance::stub_0x449dcc as stub_449dcc;
// 0x449dd0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub use crate::instance::stub_0x449dd0 as stub_449dd0;
// 0x449dd4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub use crate::instance::stub_0x449dd4 as stub_449dd4;
// 0x449df4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: boost::detail::sp_counted_impl_pd<RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub use crate::instance::stub_0x449df4 as stub_449df4;
// 0x449e0c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub use crate::instance::stub_0x449e0c as stub_449e0c;
// 0x449f84 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_20ContextActionServiceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::ContextActionService> RBX::Creatable<RBX::Instance>::create<RBX::ContextActionService>(void)")]
// was: boost::shared_ptr<RBX::ContextActionService> RBX::Creatable<RBX::Instance>::create<RBX::ContextActionService>(void)
pub use crate::instance::stub_0x449f84 as stub_449f84;
// 0x44a034 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_20ContextActionServiceEEERS3_RKNS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ContextActionService>(rbx_core::SharedPtr<RBX::ContextActionService> const&)")]
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::ContextActionService>(boost::shared_ptr<RBX::ContextActionService> const&)
pub use crate::instance::stub_0x44a034 as stub_44a034;
// 0x44a270 — __ZN5boost10shared_ptrIN3RBX20ContextActionServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::ContextActionService>::shared_ptr<RBX::ContextActionService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContextActionService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::ContextActionService>::shared_ptr<RBX::ContextActionService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContextActionService *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x44a270 as stub_44a270;
// 0x44a424 — __ZN5boost6detail12shared_countC2IPN3RBX20ContextActionServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ContextActionService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContextActionService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::detail::shared_count::shared_count<RBX::ContextActionService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContextActionService *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x44a424 as stub_44a424;
// 0x44a52c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ContextActionServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContextActionService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::ContextActionService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub use crate::instance::stub_0x44a52c as stub_44a52c;
// 0x44a530 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ContextActionServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContextActionService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::ContextActionService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub use crate::instance::stub_0x44a530 as stub_44a530;
// 0x44a534 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ContextActionServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContextActionService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::ContextActionService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub use crate::instance::stub_0x44a534 as stub_44a534;
// 0x44a554 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ContextActionServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContextActionService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: boost::detail::sp_counted_impl_pd<RBX::ContextActionService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub use crate::instance::stub_0x44a554 as stub_44a554;
// 0x44a56c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ContextActionServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContextActionService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::ContextActionService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub use crate::instance::stub_0x44a56c as stub_44a56c;
// 0x44a610 — __ZN5boost6detail12shared_countC2IPN3RBX16UserInputServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::UserInputService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::UserInputService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::detail::shared_count::shared_count<RBX::UserInputService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::UserInputService *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x44a610 as stub_44a610;
// 0x44a718 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16UserInputServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::UserInputService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::UserInputService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub use crate::instance::stub_0x44a718 as stub_44a718;
// 0x44a720 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16UserInputServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::UserInputService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::UserInputService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub use crate::instance::stub_0x44a720 as stub_44a720;
// 0x44a744 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9FWServiceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::FWService> RBX::Creatable<RBX::Instance>::create<RBX::FWService>(void)")]
// was: boost::shared_ptr<RBX::FWService> RBX::Creatable<RBX::Instance>::create<RBX::FWService>(void)
pub use crate::instance::stub_0x44a744 as stub_44a744;
// 0x44a7f4 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_9FWServiceEEERS3_RKNS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::FWService>(rbx_core::SharedPtr<RBX::FWService> const&)")]
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::FWService>(boost::shared_ptr<RBX::FWService> const&)
pub use crate::instance::stub_0x44a7f4 as stub_44a7f4;
// 0x44a828 — __ZN5boost10shared_ptrIN3RBX9FWServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::FWService>::shared_ptr<RBX::FWService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FWService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::FWService>::shared_ptr<RBX::FWService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FWService *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x44a828 as stub_44a828;
// 0x44a9dc — __ZN5boost6detail12shared_countC2IPN3RBX9FWServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::FWService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FWService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::detail::shared_count::shared_count<RBX::FWService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FWService *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x44a9dc as stub_44a9dc;
// 0x44aae4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FWServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FWService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::FWService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub use crate::instance::stub_0x44aae4 as stub_44aae4;
// 0x44aae8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FWServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FWService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::FWService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub use crate::instance::stub_0x44aae8 as stub_44aae8;
// 0x44aaec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FWServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FWService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::FWService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub use crate::instance::stub_0x44aaec as stub_44aaec;
// 0x44ab0c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FWServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FWService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: boost::detail::sp_counted_impl_pd<RBX::FWService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub use crate::instance::stub_0x44ab0c as stub_44ab0c;
// 0x44ab24 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FWServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FWService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::FWService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub use crate::instance::stub_0x44ab24 as stub_44ab24;
// 0x44ac18 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub use crate::instance::stub_0x44ac18 as stub_44ac18;
// 0x44ad94 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_21PersonalServerServiceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::PersonalServerService> RBX::Creatable<RBX::Instance>::create<RBX::PersonalServerService>(void)")]
// was: boost::shared_ptr<RBX::PersonalServerService> RBX::Creatable<RBX::Instance>::create<RBX::PersonalServerService>(void)
pub use crate::instance::stub_0x44ad94 as stub_44ad94;
// 0x44ae44 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_21PersonalServerServiceEEERS3_RKNS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::PersonalServerService>(rbx_core::SharedPtr<RBX::PersonalServerService> const&)")]
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::PersonalServerService>(boost::shared_ptr<RBX::PersonalServerService> const&)
pub use crate::instance::stub_0x44ae44 as stub_44ae44;
// 0x44b080 — __ZN5boost10shared_ptrIN3RBX21PersonalServerServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::PersonalServerService>::shared_ptr<RBX::PersonalServerService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PersonalServerService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::PersonalServerService>::shared_ptr<RBX::PersonalServerService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PersonalServerService *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x44b080 as stub_44b080;
// 0x44b234 — __ZN5boost6detail12shared_countC2IPN3RBX21PersonalServerServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PersonalServerService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PersonalServerService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::detail::shared_count::shared_count<RBX::PersonalServerService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PersonalServerService *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x44b234 as stub_44b234;
// 0x44b33c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21PersonalServerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PersonalServerService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::PersonalServerService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub use crate::instance::stub_0x44b33c as stub_44b33c;
// 0x44b340 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21PersonalServerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PersonalServerService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::PersonalServerService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub use crate::instance::stub_0x44b340 as stub_44b340;
// 0x44b344 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21PersonalServerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PersonalServerService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::PersonalServerService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub use crate::instance::stub_0x44b344 as stub_44b344;
// 0x44b364 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21PersonalServerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PersonalServerService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: boost::detail::sp_counted_impl_pd<RBX::PersonalServerService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub use crate::instance::stub_0x44b364 as stub_44b364;
// 0x44b37c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21PersonalServerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PersonalServerService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::PersonalServerService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub use crate::instance::stub_0x44b37c as stub_44b37c;
// 0x44b594 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_15TeleportServiceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::TeleportService> RBX::Creatable<RBX::Instance>::create<RBX::TeleportService>(void)")]
// was: boost::shared_ptr<RBX::TeleportService> RBX::Creatable<RBX::Instance>::create<RBX::TeleportService>(void)
pub use crate::instance::stub_0x44b594 as stub_44b594;
// 0x44b644 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_15TeleportServiceEEERS3_RKNS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::TeleportService>(rbx_core::SharedPtr<RBX::TeleportService> const&)")]
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::TeleportService>(boost::shared_ptr<RBX::TeleportService> const&)
pub use crate::instance::stub_0x44b644 as stub_44b644;
// 0x44b880 — __ZN5boost10shared_ptrIN3RBX15TeleportServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::TeleportService>::shared_ptr<RBX::TeleportService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TeleportService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::TeleportService>::shared_ptr<RBX::TeleportService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TeleportService *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x44b880 as stub_44b880;
// 0x44ba34 — __ZN5boost6detail12shared_countC2IPN3RBX15TeleportServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TeleportService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TeleportService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::detail::shared_count::shared_count<RBX::TeleportService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TeleportService *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x44ba34 as stub_44ba34;
// 0x44bb3c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15TeleportServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TeleportService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::TeleportService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub use crate::instance::stub_0x44bb3c as stub_44bb3c;
// 0x44bb40 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15TeleportServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TeleportService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::TeleportService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub use crate::instance::stub_0x44bb40 as stub_44bb40;
// 0x44bb44 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15TeleportServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TeleportService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::TeleportService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub use crate::instance::stub_0x44bb44 as stub_44bb44;
