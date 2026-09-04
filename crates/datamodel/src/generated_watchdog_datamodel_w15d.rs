//! datamodel -- generated_watchdog_datamodel_w15d -- 120 stubs (watchdog w15d Instance/DataModel/Workspace)
//! Source: ida/export.json (85545 funcs) global-dedup vs /tmp/global_eas.txt; Filter RBX::Instance
//! Strict RBX::Instance 8021 total, 0 uncovered in global -> fallback to rbx_sorted 120 (strict 0) EA-sorted asc
//! Range: 0xef04..0x2a3c28
//! Each stub preserves IDA ea + mangled + demangled for rg. Uses rbx_core::SharedPtr not boost::shared_ptr.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
// 0xef04 -- __ZN3RBX9CreatableINS_8InstanceEE6createI19CRenderSettingsItemEEN5boost10shared_ptrIT_EEv
// boost::shared_ptr<CRenderSettingsItem> RBX::Creatable<RBX::Instance>::create<CRenderSettingsItem>(void)
// type: void __fastcall(int)
// was: boost::shared_ptr<CRenderSettingsItem> RBX::Creatable<RBX::Instance>::create<CRenderSettingsItem>(void)
#[doc(alias = "rbx_core::SharedPtr<CRenderSettingsItem> RBX::Creatable<RBX::Instance>::create<CRenderSettingsItem>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createI19CRenderSettingsItemEEN5boost10shared_ptrIT_EEv")]
pub use crate::instance::stub_0xef04 as stub_ef04;
// 0xefb4 -- __ZN5boost10shared_ptrI19CRenderSettingsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_
// boost::shared_ptr<CRenderSettingsItem>::shared_ptr<CRenderSettingsItem,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int *__fastcall(int *, int, int, int)
// was: boost::shared_ptr<CRenderSettingsItem>::shared_ptr<CRenderSettingsItem,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<CRenderSettingsItem>::shared_ptr<CRenderSettingsItem,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrI19CRenderSettingsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_")]
pub use crate::instance::stub_0xefb4 as stub_efb4;
// 0xf098 -- __ZN5boost6detail12shared_countC2IP19CRenderSettingsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_
// boost::detail::shared_count::shared_count<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IP19CRenderSettingsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_")]
pub use crate::instance::stub_0xf098 as stub_f098;
// 0xf198 -- __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev
// boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev")]
pub use crate::instance::stub_0xf198 as stub_f198;
// 0xf19c -- __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv
// boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv")]
pub use crate::instance::stub_0xf19c as stub_f19c;
// 0xf1bc -- __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub use crate::instance::stub_0xf1bc as stub_f1bc;
// 0xf1d4 -- __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv
// boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub use crate::instance::stub_0xf1d4 as stub_f1d4;
// 0x179f4 -- __ZN3RBX9DataModel11uploadPlaceERKSsNS_8Instance10SaveFilterEN5boost8functionIFvNS5_10shared_ptrIKNS_10Reflection5TupleEEEEEENS6_IFvSsEEE
// RBX::DataModel::uploadPlace(std::string const&,RBX::Instance::SaveFilter,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)
// type: void __fastcall(int)
// was: RBX::DataModel::uploadPlace(std::string const&,RBX::Instance::SaveFilter,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)
#[doc(alias = "RBX::DataModel::uploadPlace(std::string const&,RBX::Instance::SaveFilter,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")]
#[doc(alias = "__ZN3RBX9DataModel11uploadPlaceERKSsNS_8Instance10SaveFilterEN5boost8functionIFvNS5_10shared_ptrIKNS_10Reflection5TupleEEEEEENS6_IFvSsEEE")]
pub use crate::instance::stub_0x179f4 as stub_179f4;
// 0x31678 -- __ZN3RBX9CreatableINS_8InstanceEE6createINS_12LoginServiceEEEN5boost10shared_ptrIT_EEv
// boost::shared_ptr<RBX::LoginService> RBX::Creatable<RBX::Instance>::create<RBX::LoginService>(void)
// was: boost::shared_ptr<RBX::LoginService> RBX::Creatable<RBX::Instance>::create<RBX::LoginService>(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::LoginService> RBX::Creatable<RBX::Instance>::create<RBX::LoginService>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_12LoginServiceEEEN5boost10shared_ptrIT_EEv")]
pub use crate::instance::stub_0x31678 as stub_31678;
// 0x31728 -- __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_12LoginServiceEEERS3_RKNS0_IT_EE
// boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::LoginService>(boost::shared_ptr<RBX::LoginService> const&)
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::LoginService>(boost::shared_ptr<RBX::LoginService> const&)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::LoginService>(rbx_core::SharedPtr<RBX::LoginService> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_12LoginServiceEEERS3_RKNS0_IT_EE")]
pub use crate::instance::stub_0x31728 as stub_31728;
// 0x319ec -- __ZN5boost10shared_ptrIN3RBX12LoginServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// boost::shared_ptr<RBX::LoginService>::shared_ptr<RBX::LoginService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int(void)
// was: boost::shared_ptr<RBX::LoginService>::shared_ptr<RBX::LoginService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::LoginService>::shared_ptr<RBX::LoginService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX12LoginServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub use crate::instance::stub_0x319ec as stub_319ec;
// 0x31aec -- __ZN5boost6detail12shared_countC2IPN3RBX12LoginServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// boost::detail::shared_count::shared_count<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX12LoginServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub use crate::instance::stub_0x31aec as stub_31aec;
// 0x31bec -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub use crate::instance::stub_0x31bec as stub_31bec;
// 0x31bf0 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub use crate::instance::stub_0x31bf0 as stub_31bf0;
// 0x31bf4 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub use crate::instance::stub_0x31bf4 as stub_31bf4;
// 0x31c14 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub use crate::instance::stub_0x31c14 as stub_31c14;
// 0x31c2c -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub use crate::instance::stub_0x31c2c as stub_31c2c;
// 0x31cd0 -- __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX8InstanceEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX8InstanceEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE")]
pub use crate::instance::stub_0x31cd0 as stub_31cd0;
// 0x31d30 -- __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX8InstanceEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
// boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::Instance>::invoke(boost::detail::function::function_buffer &,RBX::Instance)
// type: int __fastcall(int, int)
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::Instance>::invoke(boost::detail::function::function_buffer &,RBX::Instance)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::Instance>::invoke(boost::detail::function::function_buffer &,RBX::Instance)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX8InstanceEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_")]
pub use crate::instance::stub_0x31d30 as stub_31d30;
// 0x31d48 -- __ZN5boost3_bi5list3INS0_5valueIP11objc_objectEENS2_IP13objc_selectorEENS_3argILi1EEEEclIPFvS4_S6_NS_10shared_ptrIN3RBX8InstanceEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::Instance>),boost::_bi::list1<RBX::Instance&>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::Instance>) &,boost::_bi::list1<RBX::Instance&> &,int)
// type: void __fastcall(int *, void (__fastcall **)(int, int, sp_counted_base **), const shared_count **, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::Instance>),boost::_bi::list1<RBX::Instance&>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::Instance>) &,boost::_bi::list1<RBX::Instance&> &,int)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<RBX::Instance&>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,rbx_core::SharedPtr<RBX::Instance>) &,boost::_bi::list1<RBX::Instance&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueIP11objc_objectEENS2_IP13objc_selectorEENS_3argILi1EEEEclIPFvS4_S6_NS_10shared_ptrIN3RBX8InstanceEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i")]
pub use crate::instance::stub_0x31d48 as stub_31d48;
// 0x324fc -- __ZN5boost10shared_ptrIN3RBX21TaskSchedulerSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// boost::shared_ptr<RBX::TaskSchedulerSettings>::shared_ptr<RBX::TaskSchedulerSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int(void)
// was: boost::shared_ptr<RBX::TaskSchedulerSettings>::shared_ptr<RBX::TaskSchedulerSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::TaskSchedulerSettings>::shared_ptr<RBX::TaskSchedulerSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX21TaskSchedulerSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub use crate::instance::stub_0x324fc as stub_324fc;
// 0x325fc -- __ZN5boost6detail12shared_countC2IPN3RBX21TaskSchedulerSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// boost::detail::shared_count::shared_count<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX21TaskSchedulerSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub use crate::instance::stub_0x325fc as stub_325fc;
// 0x326fc -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21TaskSchedulerSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// boost::detail::sp_counted_impl_pd<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX21TaskSchedulerSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub use crate::instance::stub_0x326fc as stub_326fc;
// 0x32700 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21TaskSchedulerSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// boost::detail::sp_counted_impl_pd<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX21TaskSchedulerSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub use crate::instance::stub_0x32700 as stub_32700;
// 0x33454 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub use crate::instance::stub_0x33454 as stub_33454;
// 0x3346c -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub use crate::instance::stub_0x3346c as stub_3346c;
// 0x3a798 -- __ZN3RBX9CreatableINS_8InstanceEE6createINS_6CameraEEEN5boost10shared_ptrIT_EEv
// boost::shared_ptr<RBX::Camera> RBX::Creatable<RBX::Instance>::create<RBX::Camera>(void)
// was: boost::shared_ptr<RBX::Camera> RBX::Creatable<RBX::Instance>::create<RBX::Camera>(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::Camera> RBX::Creatable<RBX::Instance>::create<RBX::Camera>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_6CameraEEEN5boost10shared_ptrIT_EEv")]
pub use crate::instance::stub_0x3a798 as stub_3a798;
// 0x3aa10 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6CameraENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// boost::detail::sp_counted_impl_pd<RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6CameraENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub use crate::instance::stub_0x3aa10 as stub_3aa10;
// 0x3aa18 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6CameraENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// boost::detail::sp_counted_impl_pd<RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6CameraENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub use crate::instance::stub_0x3aa18 as stub_3aa18;
// 0x3afe0 -- __ZN5boost10shared_ptrIN3RBX10RunServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// boost::shared_ptr<RBX::RunService>::shared_ptr<RBX::RunService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(_DWORD, _DWORD)
// was: boost::shared_ptr<RBX::RunService>::shared_ptr<RBX::RunService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::RunService>::shared_ptr<RBX::RunService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10RunServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub use crate::instance::stub_0x3afe0 as stub_3afe0;
// 0x3b008 -- __ZN5boost6detail12shared_countC2IPN3RBX10RunServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// boost::detail::shared_count::shared_count<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX10RunServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub use crate::instance::stub_0x3b008 as stub_3b008;
// 0x3b108 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub use crate::instance::stub_0x3b108 as stub_3b108;
// 0x3b110 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub use crate::instance::stub_0x3b110 as stub_3b110;
// 0x3b130 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub use crate::instance::stub_0x3b130 as stub_3b130;
// 0x3b148 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub use crate::instance::stub_0x3b148 as stub_3b148;
// 0x3b674 -- __ZN3RBX9CreatableINS_8InstanceEE6createINS_17ControllerServiceEEEN5boost10shared_ptrIT_EEv
// boost::shared_ptr<RBX::ControllerService> RBX::Creatable<RBX::Instance>::create<RBX::ControllerService>(void)
// was: boost::shared_ptr<RBX::ControllerService> RBX::Creatable<RBX::Instance>::create<RBX::ControllerService>(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::ControllerService> RBX::Creatable<RBX::Instance>::create<RBX::ControllerService>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_17ControllerServiceEEEN5boost10shared_ptrIT_EEv")]
pub use crate::instance::stub_0x3b674 as stub_3b674;
// 0x3b724 -- __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_17ControllerServiceEEERS3_RKNS0_IT_EE
// boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::ControllerService>(boost::shared_ptr<RBX::ControllerService> const&)
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::ControllerService>(boost::shared_ptr<RBX::ControllerService> const&)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ControllerService>(rbx_core::SharedPtr<RBX::ControllerService> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_17ControllerServiceEEERS3_RKNS0_IT_EE")]
pub use crate::instance::stub_0x3b724 as stub_3b724;
// 0x3b9e8 -- __ZN5boost10shared_ptrIN3RBX17ControllerServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// boost::shared_ptr<RBX::ControllerService>::shared_ptr<RBX::ControllerService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int(void)
// was: boost::shared_ptr<RBX::ControllerService>::shared_ptr<RBX::ControllerService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::ControllerService>::shared_ptr<RBX::ControllerService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX17ControllerServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub use crate::instance::stub_0x3b9e8 as stub_3b9e8;
// 0x3ba10 -- __ZN5boost6detail12shared_countC2IPN3RBX17ControllerServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// boost::detail::shared_count::shared_count<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX17ControllerServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub use crate::instance::stub_0x3ba10 as stub_3ba10;
// 0x3bb10 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub use crate::instance::stub_0x3bb10 as stub_3bb10;
// 0x3bb18 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub use crate::instance::stub_0x3bb18 as stub_3bb18;
// 0x3bb38 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub use crate::instance::stub_0x3bb38 as stub_3bb38;
// 0x3bb50 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub use crate::instance::stub_0x3bb50 as stub_3bb50;
// 0x3bbf8 -- __ZN5boost10shared_ptrIN3RBX8InstanceEEaSERKS3_
// boost::shared_ptr<RBX::Instance>::operator=(boost::shared_ptr<RBX::Instance> const&)
// was: boost::shared_ptr<RBX::Instance>::operator=(boost::shared_ptr<RBX::Instance> const&)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>::operator=(rbx_core::SharedPtr<RBX::Instance> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8InstanceEEaSERKS3_")]
pub use crate::instance::stub_0x3bbf8 as stub_3bbf8;
// 0x3e190 -- __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED0Ev
// boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED0Ev")]
pub use crate::instance::stub_0x3e190 as stub_3e190;
// 0x258688 -- __ZN3RBX9CreatableINS_8InstanceEE6createINS_11HttpServiceEEEN5boost10shared_ptrIT_EEv
// boost::shared_ptr<RBX::HttpService> RBX::Creatable<RBX::Instance>::create<RBX::HttpService>(void)
// type: void __fastcall(int)
// was: boost::shared_ptr<RBX::HttpService> RBX::Creatable<RBX::Instance>::create<RBX::HttpService>(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::HttpService> RBX::Creatable<RBX::Instance>::create<RBX::HttpService>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_11HttpServiceEEEN5boost10shared_ptrIT_EEv")]
pub use crate::instance::stub_0x258688 as stub_258688;
// 0x258738 -- __ZN5boost10shared_ptrIN3RBX11HttpServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// boost::shared_ptr<RBX::HttpService>::shared_ptr<RBX::HttpService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int *__fastcall(int *, int, int, int)
// was: boost::shared_ptr<RBX::HttpService>::shared_ptr<RBX::HttpService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::HttpService>::shared_ptr<RBX::HttpService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX11HttpServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub use crate::instance::stub_0x258738 as stub_258738;
// 0x2588e8 -- __ZN5boost6detail12shared_countC2IPN3RBX11HttpServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// boost::detail::shared_count::shared_count<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX11HttpServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub use crate::instance::stub_0x2588e8 as stub_2588e8;
// 0x2589f0 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub use crate::instance::stub_0x2589f0 as stub_2589f0;
// 0x2589f4 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub use crate::instance::stub_0x2589f4 as stub_2589f4;
// 0x2589f8 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub use crate::instance::stub_0x2589f8 as stub_2589f8;
// 0x258a18 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub use crate::instance::stub_0x258a18 as stub_258a18;
// 0x258a30 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub use crate::instance::stub_0x258a30 as stub_258a30;
// 0x25bc38 -- __ZNK3RBX5Light12askSetParentEPKNS_8InstanceE
// RBX::Light::askSetParent(RBX::Instance const*)const
// type: _DWORD __fastcall(RBX::Light *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Light::askSetParent(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX5Light12askSetParentEPKNS_8InstanceE")]
pub use crate::instance::stub_0x25bc38 as stub_25bc38;
// 0x25bc60 -- __ZNK3RBX5Light11askAddChildEPKNS_8InstanceE
// RBX::Light::askAddChild(RBX::Instance const*)const
// type: int __fastcall(RBX::Light *this, const Instance *)
#[doc(alias = "RBX::Light::askAddChild(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX5Light11askAddChildEPKNS_8InstanceE")]
pub use crate::instance::stub_0x25bc60 as stub_25bc60;
// 0x25c4d0 -- __ZN3RBX9CreatableINS_8InstanceEE6createINS_9SpotLightEEEN5boost10shared_ptrIT_EEv
// boost::shared_ptr<RBX::SpotLight> RBX::Creatable<RBX::Instance>::create<RBX::SpotLight>(void)
// type: void __fastcall(int)
// was: boost::shared_ptr<RBX::SpotLight> RBX::Creatable<RBX::Instance>::create<RBX::SpotLight>(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::SpotLight> RBX::Creatable<RBX::Instance>::create<RBX::SpotLight>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_9SpotLightEEEN5boost10shared_ptrIT_EEv")]
pub use crate::instance::stub_0x25c4d0 as stub_25c4d0;
// 0x25c580 -- __ZN5boost10shared_ptrIN3RBX9SpotLightEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// boost::shared_ptr<RBX::SpotLight>::shared_ptr<RBX::SpotLight,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int *__fastcall(int *, int, int, int)
// was: boost::shared_ptr<RBX::SpotLight>::shared_ptr<RBX::SpotLight,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::SpotLight>::shared_ptr<RBX::SpotLight,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX9SpotLightEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub use crate::instance::stub_0x25c580 as stub_25c580;
// 0x25c730 -- __ZN5boost6detail12shared_countC2IPN3RBX9SpotLightENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// boost::detail::shared_count::shared_count<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX9SpotLightENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub use crate::instance::stub_0x25c730 as stub_25c730;
// 0x25c838 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SpotLightENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SpotLightENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub use crate::instance::stub_0x25c838 as stub_25c838;
// 0x25c83c -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SpotLightENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SpotLightENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub use crate::instance::stub_0x25c83c as stub_25c83c;
// 0x25c840 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SpotLightENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SpotLightENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub use crate::instance::stub_0x25c840 as stub_25c840;
// 0x25c860 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SpotLightENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SpotLightENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub use crate::instance::stub_0x25c860 as stub_25c860;
// 0x25c878 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SpotLightENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SpotLightENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub use crate::instance::stub_0x25c878 as stub_25c878;
// 0x25ce80 -- __ZN3RBX9CreatableINS_8InstanceEE6createINS_10PointLightEEEN5boost10shared_ptrIT_EEv
// boost::shared_ptr<RBX::PointLight> RBX::Creatable<RBX::Instance>::create<RBX::PointLight>(void)
// type: void __fastcall(int)
// was: boost::shared_ptr<RBX::PointLight> RBX::Creatable<RBX::Instance>::create<RBX::PointLight>(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::PointLight> RBX::Creatable<RBX::Instance>::create<RBX::PointLight>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_10PointLightEEEN5boost10shared_ptrIT_EEv")]
pub use crate::instance::stub_0x25ce80 as stub_25ce80;
// 0x25cf30 -- __ZN5boost10shared_ptrIN3RBX10PointLightEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// boost::shared_ptr<RBX::PointLight>::shared_ptr<RBX::PointLight,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int *__fastcall(int *, int, int, int)
// was: boost::shared_ptr<RBX::PointLight>::shared_ptr<RBX::PointLight,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::PointLight>::shared_ptr<RBX::PointLight,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10PointLightEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub use crate::instance::stub_0x25cf30 as stub_25cf30;
// 0x25d0e0 -- __ZN5boost6detail12shared_countC2IPN3RBX10PointLightENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// boost::detail::shared_count::shared_count<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX10PointLightENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub use crate::instance::stub_0x25d0e0 as stub_25d0e0;
// 0x25d1e8 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10PointLightENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10PointLightENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub use crate::instance::stub_0x25d1e8 as stub_25d1e8;
// 0x25d1ec -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10PointLightENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10PointLightENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub use crate::instance::stub_0x25d1ec as stub_25d1ec;
// 0x25d1f0 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10PointLightENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10PointLightENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub use crate::instance::stub_0x25d1f0 as stub_25d1f0;
// 0x25d210 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10PointLightENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10PointLightENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub use crate::instance::stub_0x25d210 as stub_25d210;
// 0x25d228 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10PointLightENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10PointLightENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub use crate::instance::stub_0x25d228 as stub_25d228;
// 0x26c350 -- __ZN5boost10shared_ptrIN3RBX10Reflection13DescribedBaseEEaSINS1_8InstanceEEERS4_RKNS0_IT_EE
// boost::shared_ptr<RBX::Reflection::DescribedBase>& boost::shared_ptr<RBX::Reflection::DescribedBase>::operator=<RBX::Instance>(boost::shared_ptr<RBX::Instance> const&)
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
// was: boost::shared_ptr<RBX::Reflection::DescribedBase>& boost::shared_ptr<RBX::Reflection::DescribedBase>::operator=<RBX::Instance>(boost::shared_ptr<RBX::Instance> const&)
#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::DescribedBase>& rbx_core::SharedPtr<RBX::Reflection::DescribedBase>::operator=<RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10Reflection13DescribedBaseEEaSINS1_8InstanceEEERS4_RKNS0_IT_EE")]
pub use crate::instance::stub_0x26c350 as stub_26c350;
// 0x26c38c -- __ZN3RBX3Lua15SharedPtrBridgeINS_8InstanceEE6getPtrIN5boost10shared_ptrINS_10Reflection13DescribedBaseEEEEEbP9lua_StatejRT_
// bool RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr<boost::shared_ptr<RBX::Reflection::DescribedBase>>(lua_State *,unsigned int,boost::shared_ptr<RBX::Reflection::DescribedBase> &)
// type: int __fastcall(int, int, int)
// was: bool RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr<boost::shared_ptr<RBX::Reflection::DescribedBase>>(lua_State *,unsigned int,boost::shared_ptr<RBX::Reflection::DescribedBase> &)
#[doc(alias = "bool RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr<rbx_core::SharedPtr<RBX::Reflection::DescribedBase>>(lua_State *,unsigned int,rbx_core::SharedPtr<RBX::Reflection::DescribedBase> &)")]
#[doc(alias = "__ZN3RBX3Lua15SharedPtrBridgeINS_8InstanceEE6getPtrIN5boost10shared_ptrINS_10Reflection13DescribedBaseEEEEEbP9lua_StatejRT_")]
pub use crate::instance::stub_0x26c38c as stub_26c38c;
// 0x26c830 -- __ZN3RBX3Lua15SharedPtrBridgeINS_8InstanceEE6getPtrINS_10Reflection7VariantEEEbP9lua_StatejRT_
// bool RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "bool RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
#[doc(alias = "__ZN3RBX3Lua15SharedPtrBridgeINS_8InstanceEE6getPtrINS_10Reflection7VariantEEEbP9lua_StatejRT_")]
pub use crate::instance::stub_0x26c830 as stub_26c830;
// 0x26dce4 -- __ZN3RBX3Lua14ArgumentPusherclERKN5boost10shared_ptrINS_8InstanceEEE
// RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<RBX::Instance> const&)
// type: int __fastcall(int *, int)
// was: RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<RBX::Instance> const&)
#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<RBX::Instance> const&)")]
#[doc(alias = "__ZN3RBX3Lua14ArgumentPusherclERKN5boost10shared_ptrINS_8InstanceEEE")]
pub use crate::instance::stub_0x26dce4 as stub_26dce4;
// 0x26df08 -- __ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrIKSt6vectorINS3_INS_8InstanceEEESaIS6_EEEE
// RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>)
// type: int __fastcall(_DWORD *, _DWORD *)
// was: RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>)
#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>)")]
#[doc(alias = "__ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrIKSt6vectorINS3_INS_8InstanceEEESaIS6_EEEE")]
pub use crate::instance::stub_0x26df08 as stub_26df08;
// 0x26ee14 -- __ZN3rbx8any_castIRKN5boost10shared_ptrIKSt6vectorINS2_IN3RBX8InstanceEEESaIS6_EEEENS4_7Region3EEET_RNS_13placement_anyIT0_EE
// boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> const& rbx::any_cast<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// type: char ****__fastcall(char ****)
// was: boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> const& rbx::any_cast<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
#[doc(alias = "rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> const& rbx::any_cast<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN5boost10shared_ptrIKSt6vectorINS2_IN3RBX8InstanceEEESaIS6_EEEENS4_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub use crate::instance::stub_0x26ee14 as stub_26ee14;
// 0x26ef04 -- __ZN3RBX3Lua12LuaArguments9pushArrayIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrINS_8InstanceEEESt6vectorIS8_SaIS8_EEEEEEiT_SF_P9lua_State
// int RBX::Lua::LuaArguments::pushArray<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,lua_State *)
// type: int __fastcall(char ****, char ****, int)
// was: int RBX::Lua::LuaArguments::pushArray<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,lua_State *)
#[doc(alias = "int RBX::Lua::LuaArguments::pushArray<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua12LuaArguments9pushArrayIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrINS_8InstanceEEESt6vectorIS8_SaIS8_EEEEEEiT_SF_P9lua_State")]
pub use crate::instance::stub_0x26ef04 as stub_26ef04;
// 0x26fa78 -- __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// bool RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)
// type: int __fastcall(int, int, _DWORD *)
// was: bool RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)
#[doc(alias = "bool RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_")]
pub use crate::instance::stub_0x26fa78 as stub_26fa78;
// 0x26ff94 -- __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8getValueINS3_INS_10Reflection13DescribedBaseEEEEEbP9lua_StatejRT_
// bool RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::getValue<boost::shared_ptr<RBX::Reflection::DescribedBase>>(lua_State *,unsigned int,boost::shared_ptr<RBX::Reflection::DescribedBase> &)
// type: int __fastcall(int, int, int)
// was: bool RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::getValue<boost::shared_ptr<RBX::Reflection::DescribedBase>>(lua_State *,unsigned int,boost::shared_ptr<RBX::Reflection::DescribedBase> &)
#[doc(alias = "bool RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::getValue<rbx_core::SharedPtr<RBX::Reflection::DescribedBase>>(lua_State *,unsigned int,rbx_core::SharedPtr<RBX::Reflection::DescribedBase> &)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8getValueINS3_INS_10Reflection13DescribedBaseEEEEEbP9lua_StatejRT_")]
pub use crate::instance::stub_0x26ff94 as stub_26ff94;
// 0x277af4 -- __ZN3RBX6CellID14fromParametersEbPfN5boost10shared_ptrINS_8InstanceEEE
// RBX::CellID::fromParameters(bool,float *,boost::shared_ptr<RBX::Instance>)
// type: void __fastcall(int, int, int, const shared_count *)
// was: RBX::CellID::fromParameters(bool,float *,boost::shared_ptr<RBX::Instance>)
#[doc(alias = "RBX::CellID::fromParameters(bool,float *,rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX6CellID14fromParametersEbPfN5boost10shared_ptrINS_8InstanceEEE")]
pub use crate::instance::stub_0x277af4 as stub_277af4;
// 0x27a4f8 -- __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE13registerClassEP9lua_StatePFiS8_ESA_
// RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))
// type: int __fastcall(int, int, int)
// was: RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE13registerClassEP9lua_StatePFiS8_ESA_")]
pub use crate::instance::stub_0x27a4f8 as stub_27a4f8;
// 0x27a5e0 -- __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE5on_gcEP9lua_State
// RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::on_gc(lua_State *)
// type: int __fastcall(int)
// was: RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::on_gc(lua_State *)
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_gc(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE5on_gcEP9lua_State")]
pub use crate::instance::stub_0x27a5e0 as stub_27a5e0;
// 0x27a608 -- __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE11on_tostringEP9lua_State
// RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::on_tostring(lua_State *)
// type: int __fastcall(int)
// was: RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::on_tostring(lua_State *)
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_tostring(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE11on_tostringEP9lua_State")]
pub use crate::instance::stub_0x27a608 as stub_27a608;
// 0x27c258 -- __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8on_indexERKS5_PKcP9lua_State
// RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::on_index(boost::shared_ptr<RBX::Instance> const&,char const*,lua_State *)
// type: int __fastcall(int, int, int)
// was: RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::on_index(boost::shared_ptr<RBX::Instance> const&,char const*,lua_State *)
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_index(rbx_core::SharedPtr<RBX::Instance> const&,char const*,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8on_indexERKS5_PKcP9lua_State")]
pub use crate::instance::stub_0x27c258 as stub_27c258;
// 0x27dbc8 -- __ZL22PropertyNameCorrectionRKN5boost10shared_ptrIN3RBX8InstanceEEEPKcP9lua_State
// PropertyNameCorrection(boost::shared_ptr<RBX::Instance> const&,char const*,lua_State *)
// type: const char *__fastcall(int, const char *, int)
// was: PropertyNameCorrection(boost::shared_ptr<RBX::Instance> const&,char const*,lua_State *)
#[doc(alias = "PropertyNameCorrection(rbx_core::SharedPtr<RBX::Instance> const&,char const*,lua_State *)")]
#[doc(alias = "__ZL22PropertyNameCorrectionRKN5boost10shared_ptrIN3RBX8InstanceEEEPKcP9lua_State")]
pub use crate::instance::stub_0x27dbc8 as stub_27dbc8;
// 0x27ef18 -- __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE11on_newindexERS5_PKcP9lua_State
// RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::on_newindex(boost::shared_ptr<RBX::Instance>&,char const*,lua_State *)
// type: void __fastcall(RBX::Security::Context *, const char *, int)
// was: RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::on_newindex(boost::shared_ptr<RBX::Instance>&,char const*,lua_State *)
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_newindex(rbx_core::SharedPtr<RBX::Instance>&,char const*,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE11on_newindexERS5_PKcP9lua_State")]
pub use crate::instance::stub_0x27ef18 as stub_27ef18;
// 0x280b90 -- __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE11on_tostringERKS5_P9lua_State
// RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::on_tostring(boost::shared_ptr<RBX::Instance> const&,lua_State *)
// type: int __fastcall(RBX::Instance **, int)
// was: RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::on_tostring(boost::shared_ptr<RBX::Instance> const&,lua_State *)
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_tostring(rbx_core::SharedPtr<RBX::Instance> const&,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE11on_tostringERKS5_P9lua_State")]
pub use crate::instance::stub_0x280b90 as stub_280b90;
// 0x280c4c -- __ZN3RBX3Lua15SharedPtrBridgeINS_8InstanceEE6getPtrIN5boost10shared_ptrIS2_EEEEbP9lua_StatejRT_
// bool RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr<boost::shared_ptr<RBX::Instance>>(lua_State *,unsigned int,boost::shared_ptr<RBX::Instance> &)
// type: int __fastcall(int, int, int)
// was: bool RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr<boost::shared_ptr<RBX::Instance>>(lua_State *,unsigned int,boost::shared_ptr<RBX::Instance> &)
#[doc(alias = "bool RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr<rbx_core::SharedPtr<RBX::Instance>>(lua_State *,unsigned int,rbx_core::SharedPtr<RBX::Instance> &)")]
#[doc(alias = "__ZN3RBX3Lua15SharedPtrBridgeINS_8InstanceEE6getPtrIN5boost10shared_ptrIS2_EEEEbP9lua_StatejRT_")]
pub use crate::instance::stub_0x280c4c as stub_280c4c;
// 0x281494 -- __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8getValueIS5_EEbP9lua_StatejRT_
// bool RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::getValue<boost::shared_ptr<RBX::Instance>>(lua_State *,unsigned int,boost::shared_ptr<RBX::Instance> &)
// type: int __fastcall(int, int, int)
// was: bool RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::getValue<boost::shared_ptr<RBX::Instance>>(lua_State *,unsigned int,boost::shared_ptr<RBX::Instance> &)
#[doc(alias = "bool RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::getValue<rbx_core::SharedPtr<RBX::Instance>>(lua_State *,unsigned int,rbx_core::SharedPtr<RBX::Instance> &)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8getValueIS5_EEbP9lua_StatejRT_")]
pub use crate::instance::stub_0x281494 as stub_281494;
// 0x284650 -- __ZN24YieldFunctionStateObjectC2EPKN3RBX10Reflection23YieldFunctionDescriptorEN5boost10shared_ptrINS0_8InstanceEEEP9lua_State
// YieldFunctionStateObject::YieldFunctionStateObject(RBX::Reflection::YieldFunctionDescriptor const*,boost::shared_ptr<RBX::Instance>,lua_State *)
// type: _DWORD *__fastcall(_DWORD *, int, const shared_count *, int, int, void *, int, int, int, int)
// was: YieldFunctionStateObject::YieldFunctionStateObject(RBX::Reflection::YieldFunctionDescriptor const*,boost::shared_ptr<RBX::Instance>,lua_State *)
#[doc(alias = "YieldFunctionStateObject::YieldFunctionStateObject(RBX::Reflection::YieldFunctionDescriptor const*,rbx_core::SharedPtr<RBX::Instance>,lua_State *)")]
#[doc(alias = "__ZN24YieldFunctionStateObjectC2EPKN3RBX10Reflection23YieldFunctionDescriptorEN5boost10shared_ptrINS0_8InstanceEEEP9lua_State")]
pub use crate::instance::stub_0x284650 as stub_284650;
// 0x2857a4 -- __ZN3RBX3Lua15SharedPtrBridgeINS_8InstanceEE6getPtrEP9lua_Statej
// RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr(lua_State *,unsigned int)
// type: int __fastcall(sp_counted_base **, int, const char *)
#[doc(alias = "RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr(lua_State *,unsigned int)")]
#[doc(alias = "__ZN3RBX3Lua15SharedPtrBridgeINS_8InstanceEE6getPtrEP9lua_Statej")]
pub use crate::instance::stub_0x2857a4 as stub_2857a4;
// 0x28e0c8 -- __ZN3RBX15ServiceProvider6createINS_20RuntimeScriptServiceEEEPT_PKNS_8InstanceE
// RBX::RuntimeScriptService * RBX::ServiceProvider::create<RBX::RuntimeScriptService>(RBX::Instance const*)
#[doc(alias = "RBX::RuntimeScriptService * RBX::ServiceProvider::create<RBX::RuntimeScriptService>(RBX::Instance const*)")]
#[doc(alias = "__ZN3RBX15ServiceProvider6createINS_20RuntimeScriptServiceEEEPT_PKNS_8InstanceE")]
pub use crate::instance::stub_0x28e0c8 as stub_28e0c8;
// 0x28e0e0 -- __ZN3RBX15ServiceProvider6createINS_25ScriptInformationProviderEEEPT_PKNS_8InstanceE
// RBX::ScriptInformationProvider * RBX::ServiceProvider::create<RBX::ScriptInformationProvider>(RBX::Instance const*)
#[doc(alias = "RBX::ScriptInformationProvider * RBX::ServiceProvider::create<RBX::ScriptInformationProvider>(RBX::Instance const*)")]
#[doc(alias = "__ZN3RBX15ServiceProvider6createINS_25ScriptInformationProviderEEEPT_PKNS_8InstanceE")]
pub use crate::instance::stub_0x28e0e0 as stub_28e0e0;
// 0x28e0f8 -- __ZN3RBX6Script8writeXmlERKN5boost8functionIFbPNS_8InstanceEEEENS_11CreatorRoleE
// RBX::Script::writeXml(boost::function<bool ()(RBX::Instance *)> const&,RBX::CreatorRole)
#[doc(alias = "RBX::Script::writeXml(boost::function<bool ()(RBX::Instance *)> const&,RBX::CreatorRole)")]
#[doc(alias = "__ZN3RBX6Script8writeXmlERKN5boost8functionIFbPNS_8InstanceEEEENS_11CreatorRoleE")]
pub use crate::instance::stub_0x28e0f8 as stub_28e0f8;
// 0x28e114 -- __ZNK3RBX6Script12askSetParentEPKNS_8InstanceE
// RBX::Script::askSetParent(RBX::Instance const*)const
// type: _DWORD __fastcall(RBX::Script *__hidden this, const Instance *)
#[doc(alias = "RBX::Script::askSetParent(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX6Script12askSetParentEPKNS_8InstanceE")]
pub use crate::instance::stub_0x28e114 as stub_28e114;
// 0x28e630 -- __ZN3RBX9CreatableINS_8InstanceEE6createINS_6ScriptEEEN5boost10shared_ptrIT_EEv
// boost::shared_ptr<RBX::Script> RBX::Creatable<RBX::Instance>::create<RBX::Script>(void)
// was: boost::shared_ptr<RBX::Script> RBX::Creatable<RBX::Instance>::create<RBX::Script>(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::Script> RBX::Creatable<RBX::Instance>::create<RBX::Script>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_6ScriptEEEN5boost10shared_ptrIT_EEv")]
pub use crate::instance::stub_0x28e630 as stub_28e630;
// 0x28e6e0 -- __ZN5boost10shared_ptrIN3RBX6ScriptEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// boost::shared_ptr<RBX::Script>::shared_ptr<RBX::Script,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter)
// was: boost::shared_ptr<RBX::Script>::shared_ptr<RBX::Script,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::Script>::shared_ptr<RBX::Script,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX6ScriptEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub use crate::instance::stub_0x28e6e0 as stub_28e6e0;
// 0x28e890 -- __ZN5boost6detail12shared_countC2IPN3RBX6ScriptENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// boost::detail::shared_count::shared_count<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX6ScriptENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub use crate::instance::stub_0x28e890 as stub_28e890;
// 0x28e998 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub use crate::instance::stub_0x28e998 as stub_28e998;
// 0x28e99c -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub use crate::instance::stub_0x28e99c as stub_28e99c;
// 0x28e9a0 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub use crate::instance::stub_0x28e9a0 as stub_28e9a0;
// 0x28e9c0 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub use crate::instance::stub_0x28e9c0 as stub_28e9c0;
// 0x28e9d8 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub use crate::instance::stub_0x28e9d8 as stub_28e9d8;
// 0x28f0b4 -- __ZN3RBX9CreatableINS_8InstanceEE6createINS_20RuntimeScriptServiceEEEN5boost10shared_ptrIT_EEv
// boost::shared_ptr<RBX::RuntimeScriptService> RBX::Creatable<RBX::Instance>::create<RBX::RuntimeScriptService>(void)
// was: boost::shared_ptr<RBX::RuntimeScriptService> RBX::Creatable<RBX::Instance>::create<RBX::RuntimeScriptService>(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::RuntimeScriptService> RBX::Creatable<RBX::Instance>::create<RBX::RuntimeScriptService>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_20RuntimeScriptServiceEEEN5boost10shared_ptrIT_EEv")]
pub use crate::instance::stub_0x28f0b4 as stub_28f0b4;
// 0x28f164 -- __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_20RuntimeScriptServiceEEERS3_RKNS0_IT_EE
// boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::RuntimeScriptService>(boost::shared_ptr<RBX::RuntimeScriptService> const&)
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::RuntimeScriptService>(boost::shared_ptr<RBX::RuntimeScriptService> const&)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::RuntimeScriptService>(rbx_core::SharedPtr<RBX::RuntimeScriptService> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_20RuntimeScriptServiceEEERS3_RKNS0_IT_EE")]
pub use crate::instance::stub_0x28f164 as stub_28f164;
// 0x28f62c -- __ZN5boost10shared_ptrIN3RBX20RuntimeScriptServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// boost::shared_ptr<RBX::RuntimeScriptService>::shared_ptr<RBX::RuntimeScriptService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter)
// was: boost::shared_ptr<RBX::RuntimeScriptService>::shared_ptr<RBX::RuntimeScriptService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::RuntimeScriptService>::shared_ptr<RBX::RuntimeScriptService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX20RuntimeScriptServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub use crate::instance::stub_0x28f62c as stub_28f62c;
// 0x28f7dc -- __ZN5boost6detail12shared_countC2IPN3RBX20RuntimeScriptServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// boost::detail::shared_count::shared_count<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX20RuntimeScriptServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub use crate::instance::stub_0x28f7dc as stub_28f7dc;
// 0x28f8e4 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20RuntimeScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX20RuntimeScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub use crate::instance::stub_0x28f8e4 as stub_28f8e4;
// 0x28f8e8 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20RuntimeScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX20RuntimeScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub use crate::instance::stub_0x28f8e8 as stub_28f8e8;
// 0x28f8ec -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20RuntimeScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX20RuntimeScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub use crate::instance::stub_0x28f8ec as stub_28f8ec;
// 0x28f90c -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20RuntimeScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX20RuntimeScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub use crate::instance::stub_0x28f90c as stub_28f90c;
// 0x28f924 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20RuntimeScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX20RuntimeScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub use crate::instance::stub_0x28f924 as stub_28f924;
// 0x295640 -- __ZN3RBX13ScriptContext26registerDevelopmentLibraryESsN5boost10shared_ptrINS_8InstanceEEE
// RBX::ScriptContext::registerDevelopmentLibrary(std::string,boost::shared_ptr<RBX::Instance>)
// was: RBX::ScriptContext::registerDevelopmentLibrary(std::string,boost::shared_ptr<RBX::Instance>)
#[doc(alias = "RBX::ScriptContext::registerDevelopmentLibrary(std::string,rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX13ScriptContext26registerDevelopmentLibraryESsN5boost10shared_ptrINS_8InstanceEEE")]
pub use crate::instance::stub_0x295640 as stub_295640;
// 0x295890 -- __ZN3RBX13ScriptContext13addCoreScriptEiN5boost10shared_ptrINS_8InstanceEEESs
// RBX::ScriptContext::addCoreScript(int,boost::shared_ptr<RBX::Instance>,std::string)
// was: RBX::ScriptContext::addCoreScript(int,boost::shared_ptr<RBX::Instance>,std::string)
#[doc(alias = "RBX::ScriptContext::addCoreScript(int,rbx_core::SharedPtr<RBX::Instance>,std::string)")]
#[doc(alias = "__ZN3RBX13ScriptContext13addCoreScriptEiN5boost10shared_ptrINS_8InstanceEEESs")]
pub use crate::instance::stub_0x295890 as stub_295890;
// 0x29b090 -- __ZN3RBX13ScriptContext20onCamelCaseViolationEN5boost10shared_ptrINS_8InstanceEEESsS4_
// RBX::ScriptContext::onCamelCaseViolation(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)
// was: RBX::ScriptContext::onCamelCaseViolation(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)
#[doc(alias = "RBX::ScriptContext::onCamelCaseViolation(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX13ScriptContext20onCamelCaseViolationEN5boost10shared_ptrINS_8InstanceEEESsS4_")]
pub use crate::instance::stub_0x29b090 as stub_29b090;
// 0x2a3818 -- __ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_ED1Ev
// RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::ScriptContext::*>::~EventDesc()
// was: RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::ScriptContext::*>::~EventDesc()
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_ED1Ev")]
pub use crate::instance::stub_0x2a3818 as stub_2a3818;
// 0x2a38e0 -- __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EED1Ev
// RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::~BoundFuncDesc()
// was: RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::~BoundFuncDesc()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EED1Ev")]
pub use crate::instance::stub_0x2a38e0 as stub_2a38e0;
// 0x2a3a08 -- __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EED1Ev
// RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int,boost::shared_ptr<RBX::Instance>,std::string),3>::~BoundFuncDesc()
// was: RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int,boost::shared_ptr<RBX::Instance>,std::string),3>::~BoundFuncDesc()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int,rbx_core::SharedPtr<RBX::Instance>,std::string),3>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EED1Ev")]
pub use crate::instance::stub_0x2a3a08 as stub_2a3a08;
// 0x2a3c28 -- __ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvSsSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED1Ev
// RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>)> RBX::ScriptContext::*>::~EventDesc()
// was: RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>)> RBX::ScriptContext::*>::~EventDesc()
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvSsSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED1Ev")]
pub use crate::instance::stub_0x2a3c28 as stub_2a3c28;
